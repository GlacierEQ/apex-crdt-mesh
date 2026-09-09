use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ORSet<T: Clone + Eq + Hash> {
    pub elements: HashMap<T, HashSet<Uuid>>,
    pub removed: HashSet<Uuid>,
    pub node_id: String,
}

impl<T: Clone + Eq + Hash> Default for ORSet<T> {
    fn default() -> Self {
        Self {
            elements: HashMap::new(),
            removed: HashSet::new(),
            node_id: String::new(),
        }
    }
}

impl<T: Clone + Eq + Hash> ORSet<T> {
    pub fn new(node_id: &str) -> Self {
        Self {
            elements: HashMap::new(),
            removed: HashSet::new(),
            node_id: node_id.to_string(),
        }
    }

    pub fn add(&mut self, element: T) -> Uuid {
        let tag = Uuid::new_v4();
        self.elements.entry(element).or_insert_with(HashSet::new).insert(tag);
        tag
    }

    pub fn remove(&mut self, element: &T) {
        if let Some(tags) = self.elements.get(element) {
            self.removed.extend(tags.iter().copied());
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        if let Some(tags) = self.elements.get(element) {
            tags.iter().any(|tag| !self.removed.contains(tag))
        } else {
            false
        }
    }

    pub fn elements(&self) -> HashSet<&T> {
        self.elements
            .iter()
            .filter_map(|(elem, tags)| {
                if tags.iter().any(|tag| !self.removed.contains(tag)) {
                    Some(elem)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn merge(&mut self, other: &ORSet<T>) {
        self.removed.extend(&other.removed);
        for (elem, tags) in &other.elements {
            let self_tags = self.elements.entry(elem.clone()).or_insert_with(HashSet::new);
            self_tags.extend(tags);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove_contains() {
        let mut set = ORSet::new("A");
        set.add("apple".to_string());
        assert!(set.contains(&"apple".to_string()));
        
        set.remove(&"apple".to_string());
        assert!(!set.contains(&"apple".to_string()));
    }

    #[test]
    fn concurrent_add_remove() {
        let mut set_a = ORSet::new("A");
        let mut set_b = ORSet::new("B");
        
        let elem = "banana".to_string();
        
        set_a.add(elem.clone());
        set_b.add(elem.clone());
        set_b.remove(&elem);
        
        set_a.merge(&set_b);
        assert!(set_a.contains(&elem));
    }

    #[test]
    fn merge_is_commutative_and_add_wins() {
        let mut a = ORSet::new("A");
        let mut b = ORSet::new("B");
        a.add("x".to_string());
        b.add("y".to_string());
        b.remove(&"y".to_string());
        let mut left = a.clone();
        left.merge(&b);
        let mut right = b.clone();
        right.merge(&a);
        assert_eq!(left.contains(&"x".to_string()), right.contains(&"x".to_string()));
        assert_eq!(left.contains(&"y".to_string()), right.contains(&"y".to_string()));
        assert!(left.contains(&"x".to_string()));
        assert!(!left.contains(&"y".to_string()));
    }
}
