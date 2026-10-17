use std::collections::HashMap;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicI64, Ordering},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FleetAllocationKey {
    namespace: String,
    fleet_name: String,
}

impl FleetAllocationKey {
    fn new(namespace: &str, fleet_name: &str) -> Self {
        Self {
            namespace: namespace.to_owned(),
            fleet_name: fleet_name.to_owned(),
        }
    }
}

/// Rust equivalent of upstream fleets.allocTracker.
///
/// The map itself is synchronized independently from each counter. Existing
/// entries are incremented/decremented atomically without taking the write
/// lock, matching the Go implementation's map-of-*atomic.Int64 shape.
#[derive(Debug, Default)]
pub struct FleetAllocationTracker {
    counts: RwLock<HashMap<FleetAllocationKey, Arc<AtomicI64>>>,
}

impl FleetAllocationTracker {
    pub fn get(&self, namespace: &str, fleet_name: &str) -> i64 {
        let key = FleetAllocationKey::new(namespace, fleet_name);
        let counts = self
            .counts
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        counts
            .get(&key)
            .map(|count| count.load(Ordering::SeqCst))
            .unwrap_or(0)
    }

    pub fn increment(
        &self,
        namespace: &str,
        fleet_name: &str,
        value: i64,
    ) {
        let key = FleetAllocationKey::new(namespace, fleet_name);

        {
            let counts = self
                .counts
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(count) = counts.get(&key) {
                count.fetch_add(value, Ordering::SeqCst);
                return;
            }
        }

        let mut counts = self
            .counts
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        counts
            .entry(key)
            .or_insert_with(|| Arc::new(AtomicI64::new(0)))
            .fetch_add(value, Ordering::SeqCst);
    }

    /// Mirrors upstream dec: decrementing a missing Fleet does not create an
    /// entry, preventing confusing negative counters for unknown Fleets.
    pub fn decrement(
        &self,
        namespace: &str,
        fleet_name: &str,
        value: i64,
    ) {
        let key = FleetAllocationKey::new(namespace, fleet_name);
        let counts = self
            .counts
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(count) = counts.get(&key) {
            count.fetch_sub(value, Ordering::SeqCst);
        }
    }

    pub fn remove(&self, namespace: &str, fleet_name: &str) {
        self.counts
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(&FleetAllocationKey::new(namespace, fleet_name));
    }

    /// Snapshot used by Fleet shutdown flushing.
    ///
    /// Upstream copies the map of atomic pointers under lock and loads values
    /// afterward. Clone the Arc counters here to preserve the same property:
    /// increment/decrement can proceed without holding the map lock while the
    /// snapshot is read. Zero values are skipped exactly like allocTracker.forEach.
    pub fn nonzero_snapshot(&self) -> Vec<(String, String, i64)> {
        let entries: Vec<_> = self
            .counts
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .map(|(key, count)| (key.clone(), Arc::clone(count)))
            .collect();

        let mut snapshot: Vec<_> = entries
            .into_iter()
            .filter_map(|(key, count)| {
                let value = count.load(Ordering::SeqCst);
                (value != 0).then_some((
                    key.namespace,
                    key.fleet_name,
                    value,
                ))
            })
            .collect();

        // Upstream map traversal order is intentionally unspecified. Sorting is
        // observably equivalent while making tests and flush plans deterministic.
        snapshot.sort_by(|left, right| {
            (&left.0, &left.1).cmp(&(&right.0, &right.1))
        });
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[test]
    fn missing_fleet_reads_zero_and_decrement_does_not_create() {
        let tracker = FleetAllocationTracker::default();
        assert_eq!(tracker.get("default", "fleet"), 0);

        tracker.decrement("default", "fleet", 3);
        assert_eq!(tracker.get("default", "fleet"), 0);
        assert!(tracker.nonzero_snapshot().is_empty());
    }

    #[test]
    fn increment_decrement_remove_match_upstream_tracker() {
        let tracker = FleetAllocationTracker::default();

        tracker.increment("default", "fleet", 5);
        tracker.increment("default", "fleet", 2);
        assert_eq!(tracker.get("default", "fleet"), 7);

        tracker.decrement("default", "fleet", 3);
        assert_eq!(tracker.get("default", "fleet"), 4);
        assert_eq!(
            tracker.nonzero_snapshot(),
            vec![("default".into(), "fleet".into(), 4)]
        );

        tracker.remove("default", "fleet");
        assert_eq!(tracker.get("default", "fleet"), 0);
    }

    #[test]
    fn snapshot_skips_zero_and_is_namespace_scoped() {
        let tracker = FleetAllocationTracker::default();
        tracker.increment("a", "fleet", 3);
        tracker.increment("b", "fleet", 4);
        tracker.increment("c", "zero", 1);
        tracker.decrement("c", "zero", 1);

        assert_eq!(
            tracker.nonzero_snapshot(),
            vec![
                ("a".into(), "fleet".into(), 3),
                ("b".into(), "fleet".into(), 4),
            ]
        );
    }

    #[test]
    fn concurrent_increment_uses_atomic_entry_updates() {
        let tracker = Arc::new(FleetAllocationTracker::default());
        let mut threads = Vec::new();

        for _ in 0..8 {
            let tracker = Arc::clone(&tracker);
            threads.push(thread::spawn(move || {
                for _ in 0..1_000 {
                    tracker.increment("default", "fleet", 1);
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(tracker.get("default", "fleet"), 8_000);
    }
}
