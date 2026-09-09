//! Hybrid Logical Clock for this mesh.
//!
//! Not NTP. Not wall-clock LWW. Physical time is injected so tests are
//! deterministic and two nodes can race the same millisecond without
//! flipping a coin.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Hlc {
    pub physical: u64,
    pub logical: u64,
    pub node_id: String,
}

impl Hlc {
    pub fn origin(node_id: &str) -> Self {
        Self {
            physical: 0,
            logical: 0,
            node_id: node_id.to_string(),
        }
    }

    /// Advance for a local event at `observed_physical`.
    pub fn tick(&mut self, observed_physical: u64) {
        if observed_physical > self.physical {
            self.physical = observed_physical;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    /// Incorporate a remote stamp, then tick for the receive event.
    pub fn receive(&mut self, remote: &Hlc, observed_physical: u64) {
        let max_physical = self.physical.max(remote.physical).max(observed_physical);
        if max_physical == self.physical && max_physical == remote.physical {
            self.logical = self.logical.max(remote.logical).saturating_add(1);
        } else if max_physical == self.physical {
            self.logical = self.logical.saturating_add(1);
        } else if max_physical == remote.physical {
            self.logical = remote.logical.saturating_add(1);
            self.physical = max_physical;
        } else {
            self.physical = max_physical;
            self.logical = 0;
        }
    }

    pub fn happens_before(&self, other: &Hlc) -> bool {
        matches!(self.cmp(other), Ordering::Less)
    }
}

impl PartialOrd for Hlc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hlc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.physical
            .cmp(&other.physical)
            .then(self.logical.cmp(&other.logical))
            .then(self.node_id.cmp(&other.node_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_physical_wins() {
        let a = Hlc {
            physical: 10,
            logical: 9,
            node_id: "z".into(),
        };
        let b = Hlc {
            physical: 11,
            logical: 0,
            node_id: "a".into(),
        };
        assert!(a.happens_before(&b));
    }

    #[test]
    fn same_physical_logical_is_deterministic() {
        let a = Hlc {
            physical: 5,
            logical: 1,
            node_id: "A".into(),
        };
        let b = Hlc {
            physical: 5,
            logical: 1,
            node_id: "B".into(),
        };
        assert!(a.happens_before(&b));
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn receive_does_not_go_backwards() {
        let mut local = Hlc::origin("A");
        local.tick(100);
        let remote = Hlc {
            physical: 50,
            logical: 99,
            node_id: "B".into(),
        };
        local.receive(&remote, 80);
        assert!(local.physical >= 100);
    }
}
