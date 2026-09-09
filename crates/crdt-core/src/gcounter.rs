use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GCounter {
    pub counts: HashMap<String, u64>,
    pub node_id: String,
}

impl GCounter {
    pub fn new(node_id: &str) -> Self {
        Self {
            counts: HashMap::new(),
            node_id: node_id.to_string(),
        }
    }

    pub fn increment(&mut self, delta: u64) {
        let count = self.counts.entry(self.node_id.clone()).or_insert(0);
        *count += delta;
    }

    pub fn value(&self) -> u64 {
        self.counts.values().sum()
    }

    pub fn merge(&mut self, other: &GCounter) {
        for (node_id, &other_count) in &other.counts {
            let count = self.counts.entry(node_id.clone()).or_insert(0);
            *count = std::cmp::max(*count, other_count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn merge_is_commutative(delta1 in 0..1000u64, delta2 in 0..1000u64) {
            let mut c1 = GCounter::new("A");
            c1.increment(delta1);
            let mut c2 = GCounter::new("B");
            c2.increment(delta2);

            let mut c3 = c1.clone();
            c3.merge(&c2);

            let mut c4 = c2.clone();
            c4.merge(&c1);

            prop_assert_eq!(c3.value(), c4.value());
            prop_assert_eq!(c3.counts, c4.counts);
        }
        
        #[test]
        fn merge_is_idempotent(delta in 0..1000u64) {
            let mut c1 = GCounter::new("A");
            c1.increment(delta);
            
            let c2 = c1.clone();
            c1.merge(&c2);
            
            prop_assert_eq!(c1.value(), c2.value());
            prop_assert_eq!(c1.counts, c2.counts);
        }

        #[test]
        fn merge_is_associative(d1 in 0..200u64, d2 in 0..200u64, d3 in 0..200u64) {
            let mut a = GCounter::new("A");
            a.increment(d1);
            let mut b = GCounter::new("B");
            b.increment(d2);
            let mut c = GCounter::new("C");
            c.increment(d3);

            let mut ab = a.clone();
            ab.merge(&b);
            ab.merge(&c);

            let mut bc = b.clone();
            bc.merge(&c);
            let mut left = a.clone();
            left.merge(&bc);

            prop_assert_eq!(ab.value(), left.value());
            prop_assert_eq!(ab.counts, left.counts);
        }
    }
}
