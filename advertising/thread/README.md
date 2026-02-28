# estream-thread

Provenance chain — full verifiable click path from ad impression through landing page to conversion, with per-session lattice chains and zero-knowledge path proofs. FPGA-acceleratable.

## Overview

`estream-thread` replaces opaque attribution tracking (Google Analytics, AppsFlyer, Branch) with a lattice-signed, cryptographically verifiable provenance chain built on Stratum. Every interaction hop — ad view, click, landing page view, form engagement, conversion — is ML-DSA-87 signed and appended to a per-session DAG chain. ZK proofs (Groth16) allow advertisers to verify attribution paths without revealing the intermediate pages a user visited.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `thread_chain` | `circuits/thread_chain.fl` | Per-session provenance chain — record hops, retrieve chains, verify integrity |
| `thread_proof` | `circuits/thread_proof.fl` | ZK path proof generation and verification for attribution properties |

## Stratum Usage

- **KV storage** — fast lookup for session chains and path proofs by ID
- **DAG storage** — `provenance_chain` with CSR tier on BRAM (5M node capacity) for append-only per-session chains
- **Series** — 365d retention for provenance hops, session chains, and path proofs
- **Streams** — hop event streams consumed by StreamSight, audit, and analytics

## Install

```
estream marketplace install estream-thread
```

## Security

- ML-DSA-87 signatures on every provenance hop and chain node
- ML-KEM-1024 key encapsulation for transport security
- PoVC attestation on provenance hops and ZK path proofs
- Session and page identifiers blinded via SHA3-256 hashing — never exposed raw
- ZK proofs (Groth16) for path property verification without revealing intermediate steps
- Constant-time circuits prevent timing side-channel attacks
- StreamSight anomaly detection on every circuit

## Dependencies

- `estream-etch ^0.1.0` — lattice-verified ad impression primitives
