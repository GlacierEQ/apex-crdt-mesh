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

### Machine–Mesh Protocol Manifest

<!-- glacier-eq-protocol:start -->
```yaml
{
  "schema": "glacier-eq.readme.machine-mesh/v1",
  "repository": {
    "id": "GlacierEQ/apex-crdt-mesh",
    "url": "https://github.com/GlacierEQ/apex-crdt-mesh",
    "readme_contract": "estate-machine-v1",
    "default_branch": "main"
  },
  "machine": {
    "repository_kind": "migration-residue",
    "public_api": "inspect-declared-entrypoints",
    "protocol_files": [],
    "entrypoints": [
      {
        "kind": "package-contract",
        "path": "Cargo.toml",
        "policy": "inspect-before-use"
      },
      {
        "kind": "capability-contract",
        "path": "GENIUS.yaml",
        "policy": "read-first"
      },
      {
        "kind": "role-contract",
        "path": "ROLE.yaml",
        "policy": "read-first"
      }
    ]
  },
  "presentation": {
    "architecture": [
      "recruiter",
      "master",
      "machine",
      "mesh"
    ],
    "authority": {
      "capability": "stone-psysoc-x",
      "repository": "GlacierEQ/AKOS",
      "manifest": "stones/psysoc-x/stone.json",
      "engine": "infinity_stones/psysoc_x.py"
    },
    "truth_invariant": "presentation-may-change-sequence-density-tone-and-style; facts-evidence-uncertainty-provenance-dignity-and-reader-agency-may-not"
  },
  "license": {
    "class": "NO_ROOT_LICENSE_DETECTED",
    "status": "ORIGINALITY_AND_PROVENANCE_REVIEW_REQUIRED",
    "controlling_path": null,
    "policy": "GlacierEQ/job-app-helix/LICENSE_POLICY.json",
    "may_relicense_automatically": false,
    "upstream_rights_must_be_preserved": false
  },
  "mesh": {
    "primary_home": null,
    "branch": "migration-residue",
    "subcategory": "unresolved-primary-home",
    "routing": [
      {
        "relation": "estate-map",
        "target": "GlacierEQ/monolith",
        "url": "https://github.com/GlacierEQ/monolith"
      }
    ],
    "boundaries": [
      "routing-does-not-transfer-source-code-evidence-deployment-or-lifecycle-authority",
      "generated-contract-is-a-source-index-not-a-runtime-or-provider-receipt",
      "implementation-and-provider-state-require-independent-evidence",
      "presentation-calibration-cannot-promote-claim-or-evidence-state",
      "license-automation-cannot-relicense-unresolved-upstream-or-third-party-rights"
    ]
  },
  "provenance": {
    "generated_by": "GlacierEQ/job-app-helix",
    "generator_contract": "estate-machine-v1",
    "classification_source": null,
    "classification_evidence_path": null,
    "classification_evidence_blob_sha": null,
    "classification_status": null,
    "contract_digest": "4c68ea0e8277d2cd575767835313068edb2ebb20fc22422e5725db12bcac8554"
  }
}
```
<!-- glacier-eq-protocol:end -->
