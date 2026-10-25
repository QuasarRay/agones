use std::collections::{HashMap, HashSet};

use agones_core::{GameServer, GameServerState};
use prometheus::{GaugeVec, Opts, Registry};

type Key = [String; 3];

pub struct GameServerCount {
    gauge: GaugeVec,
    known: HashSet<Key>,
}

impl GameServerCount {
    pub fn register(r: &Registry) -> prometheus::Result<Self> {
        let gauge = GaugeVec::new(
            Opts::new("agones_gameservers_count", "The number of gameservers"),
            &["type", "fleet_name", "namespace"],
        )?;
        r.register(Box::new(gauge.clone()))?;
        Ok(Self { gauge, known: HashSet::new() })
    }

    pub fn record(&mut self, servers: &[GameServer]) {
        let mut counts = HashMap::<Key, u64>::new();
        for gs in servers {
            let fleet = gs.metadata.labels.get("agones.dev/fleet").map(String::as_str).unwrap_or("");
            let key = [
                gs.status.state.as_ref().map_or("", GameServerState::as_str).into(),
                fleet.into(),
                gs.metadata.namespace.clone(),
            ];
            *counts.entry(key).or_default() += 1;
        }

        let next = counts.keys().cloned().collect::<HashSet<_>>();
        for key in self.known.difference(&next) { self.set(key, 0.); }
        for (key, count) in &counts { self.set(key, *count as f64); }
        self.known = next;
    }

    fn set(&self, key: &Key, value: f64) {
        self.gauge
            .with_label_values(&[&key[0], none(&key[1]), none(&key[2])])
            .set(value);
    }
}

fn none(s: &str) -> &str { if s.is_empty() { "none" } else { s } }
