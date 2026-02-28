# estream-etch

Lattice-verified ad engine for eStream — cryptographically verifiable ad impressions with steganographic proofs, Merkle chains, and zero-knowledge impression counts. FPGA-acceleratable (~8000 impressions/sec per pipeline).

## Overview

`estream-etch` replaces opaque impression tracking systems (DoubleVerify, IAS, MOAT) with a lattice-signed, cryptographically verifiable approach built on Stratum. Every ad impression is ML-DSA-87 signed at the moment of serving, with a steganographic proof fragment embedded directly into the ad image. Impressions chain into a per-campaign Merkle DAG for tamper-evident ordering, and ZK proofs (Groth16) allow advertisers to verify impression counts without revealing individual viewer identities.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `etch_impression` | `circuits/etch_impression.fl` | Core impression signing, steganographic payload embedding and extraction |
| `etch_merkle` | `circuits/etch_merkle.fl` | Merkle chain for tamper-evident impression ordering per campaign |
| `etch_verify` | `circuits/etch_verify.fl` | ZK impression count verification and full campaign audit |

## Stratum Usage

- **KV storage** — fast lookup for ad creatives, impression records, chain heads, and impression proofs by ID
- **DAG storage** — `impression_chain` with CSR tier on BRAM (1M node capacity) for Merkle chain traversal
- **Series** — 365d retention for impression records and proofs, forever for chain heads
- **Streams** — impression event streams consumed by StreamSight, audit, and advertiser dashboards

## Performance

| Target | Throughput | Latency |
|--------|-----------|---------|
| WASM (CPU) | ~200 impressions/sec | ~5-8ms/impression |
| FPGA | ~8000 impressions/sec per pipeline | <120us/impression |

## Install

```
estream marketplace install estream-etch
```

## Security

- ML-DSA-87 signatures on every impression record and chain node
- ML-KEM-1024 key encapsulation for transport security
- Steganographic proofs embedded in ad images for out-of-band verification
- PoVC attestation on impression records, chain nodes, and ZK proofs
- Viewer identity blinded via SHA3-256 hash with session salt — never exposed
- ZK proofs (Groth16) for impression count verification without revealing viewer identity
- Constant-time circuits prevent timing side-channel attacks
- StreamSight anomaly detection on every circuit
