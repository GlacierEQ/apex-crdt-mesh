use crdt_core::{GCounter, PNCounter, CrdtError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrdtMessage {
    SyncGCounter {
        node_id: String,
        counter: GCounter,
    },
    SyncPNCounter {
        node_id: String,
        counter: PNCounter,
    },
    SyncORSet {
        node_id: String,
        set_name: String,
        set_json: String,
    },
    SyncRGA {
        node_id: String,
        array_name: String,
        rga_json: String,
    },
    Ping {
        node_id: String,
        timestamp_ms: u64,
    },
    Pong {
        node_id: String,
        timestamp_ms: u64,
    },
}

pub fn encode(msg: &CrdtMessage) -> Result<Vec<u8>, CrdtError> {
    serde_json::to_vec(msg).map_err(CrdtError::Serialization)
}

pub fn decode(data: &[u8]) -> Result<CrdtMessage, CrdtError> {
    serde_json::from_slice(data).map_err(CrdtError::Serialization)
}
