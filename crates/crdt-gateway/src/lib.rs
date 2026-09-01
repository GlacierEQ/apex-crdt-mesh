use crdt_core::{GCounter, PNCounter, CrdtError};
use crdt_rpc::{encode, decode, CrdtMessage};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CRDT error: {0}")]
    Crdt(#[from] CrdtError),
    #[error("Framing error")]
    FramingError,
}

#[derive(Default)]
pub struct GatewayState {
    pub gcounters: HashMap<String, GCounter>,
    pub pncounters: HashMap<String, PNCounter>,
}

pub struct CrdtGateway {
    pub node_id: String,
    pub bind_addr: String,
    pub peers: Arc<RwLock<HashMap<String, Arc<RwLock<TcpStream>>>>>,
    pub state: Arc<RwLock<GatewayState>>,
}

impl CrdtGateway {
    pub async fn new(node_id: &str, bind_addr: &str) -> Result<Self, GatewayError> {
        Ok(Self {
            node_id: node_id.to_string(),
            bind_addr: bind_addr.to_string(),
            peers: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(RwLock::new(GatewayState::default())),
        })
    }

    pub async fn connect_peer(&mut self, peer_addr: &str) -> Result<(), GatewayError> {
        let stream = TcpStream::connect(peer_addr).await?;
        self.peers.write().await.insert(peer_addr.to_string(), Arc::new(RwLock::new(stream)));
        Ok(())
    }

    pub async fn broadcast_sync(&self) -> Result<(), GatewayError> {
        let state = self.state.read().await;
        
        let mut messages = Vec::new();
        for (_, counter) in &state.gcounters {
            messages.push(CrdtMessage::SyncGCounter {
                node_id: self.node_id.clone(),
                counter: counter.clone(),
            });
        }
        for (_, counter) in &state.pncounters {
            messages.push(CrdtMessage::SyncPNCounter {
                node_id: self.node_id.clone(),
                counter: counter.clone(),
            });
        }
        
        let peers = self.peers.read().await;
        for (_, peer) in peers.iter() {
            let mut stream = peer.write().await;
            for msg in &messages {
                let data = encode(msg)?;
                let len = (data.len() as u32).to_be_bytes();
                stream.write_all(&len).await?;
                stream.write_all(&data).await?;
            }
        }
        
        Ok(())
    }

    pub async fn serve(&self) -> Result<(), GatewayError> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        
        loop {
            let (mut socket, _) = listener.accept().await?;
            let state_ref = self.state.clone();
            
            tokio::spawn(async move {
                let mut len_buf = [0u8; 4];
                loop {
                    if let Err(_) = socket.read_exact(&mut len_buf).await {
                        break;
                    }
                    let len = u32::from_be_bytes(len_buf) as usize;
                    let mut data_buf = vec![0u8; len];
                    if let Err(_) = socket.read_exact(&mut data_buf).await {
                        break;
                    }
                    
                    if let Ok(msg) = decode(&data_buf) {
                        let mut state = state_ref.write().await;
                        match msg {
                            CrdtMessage::SyncGCounter { node_id: _, counter } => {
                                // In a real app we'd need an ID for the counter too. Here just hardcode "default"
                                let c = state.gcounters.entry("default".to_string()).or_insert_with(|| GCounter::new("remote"));
                                c.merge(&counter);
                            }
                            CrdtMessage::SyncPNCounter { node_id: _, counter } => {
                                let c = state.pncounters.entry("default".to_string()).or_insert_with(|| PNCounter::new("remote"));
                                c.merge(&counter);
                            }
                            _ => {}
                        }
                    }
                }
            });
        }
    }
}
