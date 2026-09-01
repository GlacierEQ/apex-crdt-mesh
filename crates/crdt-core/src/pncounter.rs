use serde::{Deserialize, Serialize};
use crate::gcounter::GCounter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PNCounter {
    pub positive: GCounter,
    pub negative: GCounter,
    pub node_id: String,
}

impl PNCounter {
    pub fn new(node_id: &str) -> Self {
        Self {
            positive: GCounter::new(node_id),
            negative: GCounter::new(node_id),
            node_id: node_id.to_string(),
        }
    }

    pub fn increment(&mut self, delta: u64) {
        self.positive.increment(delta);
    }

    pub fn decrement(&mut self, delta: u64) {
        self.negative.increment(delta);
    }

    pub fn value(&self) -> i64 {
        (self.positive.value() as i64) - (self.negative.value() as i64)
    }

    pub fn merge(&mut self, other: &PNCounter) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn merge_is_commutative(inc1 in 0..100u64, dec1 in 0..100u64, inc2 in 0..100u64, dec2 in 0..100u64) {
            let mut c1 = PNCounter::new("A");
            c1.increment(inc1);
            c1.decrement(dec1);
            
            let mut c2 = PNCounter::new("B");
            c2.increment(inc2);
            c2.decrement(dec2);

            let mut c3 = c1.clone();
            c3.merge(&c2);

            let mut c4 = c2.clone();
            c4.merge(&c1);

            prop_assert_eq!(c3.value(), c4.value());
            prop_assert_eq!(c3.positive.counts, c4.positive.counts);
            prop_assert_eq!(c3.negative.counts, c4.negative.counts);
        }
    }
}
