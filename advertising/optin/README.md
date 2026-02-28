# estream-optin

Patent-backed opt-in engagement model — activity scores, compensation events, engagement codes, escrow accounts, view records, and multi-party settlement for privacy-preserving partner engagement. FPGA-acceleratable.

## Overview

`estream-optin` implements a fundamentally different engagement model descended from patents US20120166262A1, US20130117084A1, US20130126605A1, and US20170249608A1. Unlike passive terms-and-conditions consent, this system requires users to **actively opt in** with clear, specific terms — what data is shared, with whom, for what compensation. Every consent decision is ML-DSA-87 signed and recorded as an immutable DAG node, producing a cryptographically provable consent chain.

Partners pre-deposit budgets into escrow accounts, providing budget certainty. When a user activates an engagement code, funds are reserved from escrow and released only upon verified completion. Multi-party settlement distributes funds deterministically across platform, user, affiliate, and publisher shares with Groth16-attested correctness.

The result: partners get high-quality intentional leads (10-15% conversion vs 3-5% industry average), users control their data and receive compensation, and every step is auditable without exposing personal information.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `optin_profile` | `circuits/optin_profile.fl` | User activity scores — authentication, engagement, and sharing dimensions with AI-optimized weighting |
| `optin_engage` | `circuits/optin_engage.fl` | Engagement codes and consent — offer generation, user activation (the opt-in moment), lattice-signed consent proofs |
| `optin_escrow` | `circuits/optin_escrow.fl` | Partner budget escrow — deposit, reserve, release on completion, expire on timeout |
| `optin_view` | `circuits/optin_view.fl` | View records and data rules — per-engagement field visibility bitmask, redaction enforcement, access audit |
| `optin_settle` | `circuits/optin_settle.fl` | Multi-party settlement — deterministic fund distribution from escrow per configurable rules |

## Patent Lineage

| Patent | Title | Key Innovation |
|--------|-------|----------------|
| US20120166262A1 | System for opt-in engagement | Activity scoring, engagement codes, compensation events |
| US20130117084A1 | Escrow reservation model | Pre-deposited partner budgets, per-engagement reservation |
| US20130126605A1 | View record data rules | Field-level visibility bitmask, user-approved data sharing |
| US20170249608A1 | Multi-party settlement | Deterministic fund distribution with cryptographic proofs |

## Stratum Usage

- **KV storage** — fast lookup for activity scores, engagement codes, escrow accounts, reservations, view records, settlement records, and rules
- **Graph storage** — activity scores linked to CRM user profiles; engagement codes linked to escrow reservations
- **DAG storage** — immutable consent chain (ConsentProof nodes); settlement chain (SettlementRecord nodes)
- **Series** — 365d retention for activity events and scores; 7y retention for engagement codes, escrow, view records, and settlements
- **Streams** — event streams consumed by StreamSight, audit, cortex, analytics, and alerting

## Install

```
estream marketplace install estream-optin
```

## Cortex AI Integration

- **Activity score optimizer** — learns optimal weights for authentication, engagement, and sharing dimensions; retrains on score distribution drift
- **Score prediction** — predicts user activity trajectory and flags anomalous score changes

## Security

- ML-DSA-87 signatures on every engagement code, consent proof, and settlement record
- ML-KEM-1024 key encapsulation for transport security
- PoVC attestation on score events, engagement codes, consent proofs, view access logs, and settlements
- Groth16 ZK proofs on consent proofs and settlement records
- Constant-time circuits for consent activation and settlement prevent timing side-channel attacks
- Per-engagement field visibility bitmask — redacted fields never leak, even under adversarial access
- Escrow invariants enforce sufficient balance before every reservation
- Settlement invariants enforce determinism and prevent over-distribution
- StreamSight anomaly detection on every circuit

## Dependencies

- `estream-crm ^0.1.0` — graph-native CRM engine (user profiles, relationships, lead pipeline)
- `estream-thread ^0.1.0` — provenance chain for engagement attribution
