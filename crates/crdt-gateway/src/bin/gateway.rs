use crdt_gateway::CrdtGateway;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let node_id = args.iter().position(|a| a == "--node-id").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "node-1".to_string());
    let bind_addr = args.iter().position(|a| a == "--bind").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "0.0.0.0:7777".to_string());
    let peers: Vec<String> = args.iter().position(|a| a == "--peers")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_default();

    eprintln!("Starting gateway {} on {}", node_id, bind_addr);
    
    let mut gateway = CrdtGateway::new(&node_id, &bind_addr).await?;
    
    for peer in peers {
        eprintln!("Connecting to peer {}...", peer);
        if let Err(e) = gateway.connect_peer(&peer).await {
            eprintln!("Failed to connect to {}: {}", peer, e);
        }
    }

    gateway.serve().await?;
    Ok(())
}
