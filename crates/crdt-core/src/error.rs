use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CrdtError {
    #[error("Entry not found: {0}")]
    EntryNotFound(Uuid),
    #[error("Merge conflict: {0}")]
    MergeConflict(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
