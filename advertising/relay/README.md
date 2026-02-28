# estream-relay

Affiliate and referral platform for eStream — user-to-user referral programs with attribution tracking, compensation chains, and viral growth analytics.

## Overview

`estream-relay` replaces traditional affiliate/referral platforms (Impact, PartnerStack, ReferralCandy) with a graph-native approach built on Stratum. Referral links, multi-level referral trees, and compensation records are first-class graph nodes and edges — not rows in a relational database. Cortex AI powers viral coefficient analysis and detects viral growth thresholds in real time.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `relay_affiliate` | `circuits/relay_affiliate.fl` | Referral links, attribution tracking, multi-level compensation, viral metrics |

## Stratum Usage

- **Graph storage** — `referral_graph` with CSR tier on BRAM (500K node capacity) for referral chain traversal
- **KV storage** — fast lookup for referral links, compensation records by ID
- **Series** — 365d retention for referral links and viral metrics, 7y for compensation records
- **Streams** — event streams consumed by StreamSight, audit, Cortex, analytics

## Install

```
estream marketplace install estream-relay
```

## Cortex AI Integration

- **Viral coefficient analysis** — graph-level AI feed tracking referral depth and viral coefficient
- **Viral detector** — inference-on-threshold consumer triggered when viral coefficient exceeds 1.5
- **Compensation visibility** — obfuscated referrer/referee IDs with exposed amounts for audit

## Security

- ML-DSA-87 signatures on all mutations and graph edges
- ML-KEM-1024 key encapsulation
- PoVC attestation on compensation records and viral metrics
- PII obfuscation via Cortex governance (referrer/referee IDs)
- StreamSight anomaly detection on every circuit
