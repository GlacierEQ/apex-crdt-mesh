use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::CrdtError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RGAEntry<T> {
    pub id: Uuid,
    pub after: Option<Uuid>,
    pub value: T,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RGA<T: Clone> {
    pub entries: Vec<RGAEntry<T>>,
    pub node_id: String,
}

impl<T: Clone + PartialEq> RGA<T> {
    pub fn new(node_id: &str) -> Self {
        Self {
            entries: Vec::new(),
            node_id: node_id.to_string(),
        }
    }

    pub fn insert(&mut self, after: Option<Uuid>, value: T) -> Uuid {
        let id = Uuid::new_v4();
        let entry = RGAEntry {
            id,
            after,
            value,
            deleted: false,
        };
        self.entries.push(entry);
        id
    }

    pub fn delete(&mut self, id: Uuid) -> Result<(), CrdtError> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.deleted = true;
            Ok(())
        } else {
            Err(CrdtError::EntryNotFound(id))
        }
    }
    
    pub fn to_vec(&self) -> Vec<&T> {
        let mut adj: std::collections::HashMap<Option<Uuid>, Vec<&RGAEntry<T>>> = std::collections::HashMap::new();
        for e in &self.entries {
            adj.entry(e.after).or_insert_with(Vec::new).push(e);
        }
        for children in adj.values_mut() {
            children.sort_by(|a, b| b.id.cmp(&a.id));
        }
        let mut result = Vec::new();
        let mut stack = Vec::new();
        if let Some(children) = adj.get(&None) {
            for child in children.iter().rev() {
                stack.push(*child);
            }
        }
        while let Some(curr) = stack.pop() {
            if !curr.deleted {
                result.push(&curr.value);
            }
            if let Some(children) = adj.get(&Some(curr.id)) {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
        result
    }

    pub fn merge(&mut self, other: &RGA<T>) -> Result<(), CrdtError> {
        let mut known: std::collections::HashSet<Uuid> = self.entries.iter().map(|e| e.id).collect();
        let mut tombstoned: std::collections::HashSet<Uuid> = self.entries.iter().filter(|e| e.deleted).map(|e| e.id).collect();
        for other_entry in &other.entries {
            if !known.contains(&other_entry.id) {
                self.entries.push(other_entry.clone());
                known.insert(other_entry.id);
            }
            if other_entry.deleted {
                tombstoned.insert(other_entry.id);
            }
        }
        for entry in &mut self.entries {
            if tombstoned.contains(&entry.id) {
                entry.deleted = true;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rga_insert_delete_merge() {
        let mut rga1 = RGA::new("A");
        let id1 = rga1.insert(None, "hello".to_string());
        
        let mut rga2 = RGA::new("B");
        rga2.merge(&rga1).unwrap();
        let _id2 = rga2.insert(Some(id1), "world".to_string());
        
        rga1.merge(&rga2).unwrap();
        let result = rga1.to_vec();
        assert_eq!(result, vec![&"hello".to_string(), &"world".to_string()]);
        
        rga1.delete(id1).unwrap();
        let result2 = rga1.to_vec();
        assert_eq!(result2, vec![&"world".to_string()]);
    }
}
