//! Last-Writer-Wins register stamped by [`Hlc`].
//!
//! Distinct from G-Counter (monotonic sums) and OR-Set (add-wins tags).
//! Concurrent writes resolve by hybrid logical order, never by "last merge
//! call" or wall-clock luck.

use serde::{Deserialize, Serialize};

use crate::hlc::Hlc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LwwRegister<T: Clone> {
    pub value: Option<T>,
    pub stamp: Hlc,
    pub node_id: String,
}

impl<T: Clone + PartialEq> LwwRegister<T> {
    pub fn new(node_id: &str) -> Self {
        Self {
            value: None,
            stamp: Hlc::origin(node_id),
            node_id: node_id.to_string(),
        }
    }

    pub fn write(&mut self, value: T, observed_physical: u64) {
        self.stamp.tick(observed_physical);
        self.value = Some(value);
    }

    pub fn read(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Merge is a lattice join on the HLC order. Equal stamps keep `self`.
    pub fn merge(&mut self, other: &LwwRegister<T>) {
        if other.stamp > self.stamp {
            self.value = other.value.clone();
            self.stamp = other.stamp.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn replica(id: &str, value: &str, physical: u64, logical: u64) -> LwwRegister<String> {
        LwwRegister {
            value: Some(value.to_string()),
            stamp: Hlc {
                physical,
                logical,
                node_id: id.to_string(),
            },
            node_id: id.to_string(),
        }
    }

    #[test]
    fn later_write_wins_across_replicas() {
        let mut a = LwwRegister::new("A");
        a.write("first".into(), 10);
        let mut b = LwwRegister::new("B");
        b.write("second".into(), 20);
        a.merge(&b);
        assert_eq!(a.read().map(String::as_str), Some("second"));
    }

    #[test]
    fn earlier_remote_cannot_clobber() {
        let mut a = replica("A", "live", 50, 0);
        let b = replica("B", "stale", 10, 99);
        a.merge(&b);
        assert_eq!(a.read().map(String::as_str), Some("live"));
    }

    proptest! {
        #[test]
        fn merge_commutative(
            p1 in 0..200u64,
            p2 in 0..200u64,
            l1 in 0..20u64,
            l2 in 0..20u64,
        ) {
            let a = replica("A", "alpha", p1, l1);
            let b = replica("B", "beta", p2, l2);
            let mut left = a.clone();
            left.merge(&b);
            let mut right = b.clone();
            right.merge(&a);
            prop_assert_eq!(left.value, right.value);
            prop_assert_eq!(left.stamp, right.stamp);
        }

        #[test]
        fn merge_idempotent(p in 0..200u64, l in 0..20u64) {
            let a = replica("A", "once", p, l);
            let mut m = a.clone();
            m.merge(&a);
            prop_assert_eq!(m, a);
        }

        #[test]
        fn merge_associative(
            p1 in 0..80u64,
            p2 in 0..80u64,
            p3 in 0..80u64,
        ) {
            let a = replica("A", "a", p1, 0);
            let b = replica("B", "b", p2, 0);
            let c = replica("C", "c", p3, 0);
            let mut ab = a.clone();
            ab.merge(&b);
            ab.merge(&c);
            let mut bc = b.clone();
            bc.merge(&c);
            let mut left = a.clone();
            left.merge(&bc);
            prop_assert_eq!(ab.value, left.value);
            prop_assert_eq!(ab.stamp, left.stamp);
        }
    }
}
