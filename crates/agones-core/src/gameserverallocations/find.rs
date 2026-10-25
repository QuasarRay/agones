use rand::{seq::SliceRandom, Rng};
use std::{error::Error, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllocationSelectionError {
    UnsupportedScheduling(String),
    NoGameServer,
}

impl fmt::Display for AllocationSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScheduling(strategy) => {
                write!(
                    f,
                    "scheduling strategy of '{strategy}' is not supported"
                )
            }
            Self::NoGameServer => {
                write!(f, "could not find an allocatable GameServer")
            }
        }
    }
}

impl Error for AllocationSelectionError {}

pub fn find_game_server_for_allocation<'a, R>(
    allocation: &GameServerAllocation,
    game_servers: &'a [GameServer],
    rng: &mut R,
) -> Result<(&'a GameServer, usize), AllocationSelectionError>
where
    R: Rng + ?Sized,
{
    // One slot per selector.
    //
    // Each selector records the index of the FIRST GameServer that matched
    // that selector according to the selected candidate iteration order.
    let mut selector_matches = vec![None; allocation.spec.selectors.len()];

    // Preserve the original GameServer slice. Distributed scheduling
    // randomises indices rather than moving the GameServers themselves.
    let mut candidate_indices =
        (0..game_servers.len()).collect::<Vec<_>>();

    match allocation.spec.scheduling.as_ref() {
        Some(SchedulingStrategy::Packed) => {
            // Already in the required order.
        }

        Some(SchedulingStrategy::Distributed)
            if allocation.spec.priorities.is_empty() =>
        {
            candidate_indices.shuffle(rng);
        }

        Some(SchedulingStrategy::Distributed) => {
            // When priorities are present, upstream Agones deliberately
            // does NOT shuffle. The caller has already sorted the list
            // according to the configured priorities.
        }

        Some(SchedulingStrategy::Unknown(strategy)) => {
            return Err(
                AllocationSelectionError::UnsupportedScheduling(
                    strategy.clone(),
                ),
            );
        }

        None => {
            return Err(
                AllocationSelectionError::UnsupportedScheduling(
                    String::new(),
                ),
            );
        }
    }

    for index in candidate_indices {
        let game_server = &game_servers[index];

        // Allocations can only select GameServers from the same namespace.
        if game_server.metadata.namespace
            != allocation.metadata.namespace
        {
            continue;
        }

        for (selector_index, selector) in
            allocation.spec.selectors.iter().enumerate()
        {
            // Once a selector has found its first match, later GameServers
            // cannot replace it.
            if selector_matches[selector_index].is_none()
                && selector.matches(game_server)
            {
                selector_matches[selector_index] = Some(index);
            }
        }
    }

    // Selector order is itself a priority order.
    //
    // Therefore selector 0 wins over selector 1 even if selector 1 found
    // an earlier GameServer in the candidate traversal.
    for index in selector_matches.into_iter().flatten() {
        return Ok((&game_servers[index], index));
    }

    Err(AllocationSelectionError::NoGameServer)
}