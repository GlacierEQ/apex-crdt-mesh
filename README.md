# apex-crdt-mesh

Provably convergent distributed state synchronization library in Rust.

## Architecture

This workspace consists of:
- `crdt-core`: Implements the state-based CRDTs (GCounter, PNCounter, ORSet, RGA).
- `crdt-rpc`: Common wire protocols (using JSON currently).
- `crdt-gateway`: A tokio-based P2P networking layer.

## CRDT Theory
CRDTs provide strong eventual consistency. Operations satisfy merge laws forming a join semilattice:
- Commutativity: `A merge B = B merge A`
- Associativity: `(A merge B) merge C = A merge (B merge C)`
- Idempotence: `A merge A = A`

## Future Roadmap
- Replace `serde_json` with **Cap'n Proto** for extreme zero-copy serialization performance in production.
