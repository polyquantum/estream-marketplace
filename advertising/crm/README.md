# estream-crm

Graph-native CRM and lifecycle management engine for eStream.

## Overview

`estream-crm` replaces traditional CRM platforms (HubSpot, Salesforce) with a graph-native approach built on Stratum. User profiles, claims, leads, and partner relationships are first-class graph nodes and edges — not rows in a relational database. Cortex AI governance provides automated lifecycle classification, claim outcome prediction, and lead quality scoring.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `crm_graph` | `circuits/crm_graph.fl` | Core data model — user profiles, relationships, graph traversal |
| `crm_lifecycle` | `circuits/crm_lifecycle.fl` | Generic user lifecycle state machine (Anonymous → Active → Churned) |
| `crm_claim` | `circuits/crm_claim.fl` | Claim filing, state transitions, evidence tracking with AI outcome prediction |
| `crm_lead` | `circuits/crm_lead.fl` | Lead pipeline — creation, qualification, partner matching, conversion |

## Stratum Usage

- **Graph storage** — `crm_graph` with CSR tier on BRAM (100K node capacity)
- **KV storage** — fast lookup for profiles, claims, leads by ID
- **Series** — 365d retention for profiles/leads, 7y for claims and audit trails
- **Streams** — event streams with 7y retention, consumed by StreamSight, audit, Cortex

## Install

```
estream marketplace install estream-crm
```

## Cortex AI Integration

- **Lifecycle classifier** — infers lifecycle stage transitions from engagement patterns
- **Claim outcome predictor** — predicts approval/denial with confidence scores, retrains on drift
- **Lead quality scorer** — scores lead quality and suggests partner matches

## Security

- ML-DSA-87 signatures on all mutations
- ML-KEM-1024 key encapsulation
- PoVC attestation on claims and audit records
- PII obfuscation/redaction via Cortex governance
- StreamSight anomaly detection on every circuit
