# estream-catalyst

AI-driven outcome optimizer for eStream — cortex feedback loops that learn from approval/denial patterns to increase conversion rates.

## Overview

`estream-catalyst` is a generic application/claims optimizer built on Stratum and Cortex. It tracks outcome patterns (approved, denied, pending, withdrawn), feeds them into a prediction model (`catalyst_outcome_predictor`), and generates actionable suggestions to improve conversion rates. The feedback loop continuously retrains on drift, adapting to shifting approval criteria and market conditions.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `catalyst_optimizer` | `circuits/catalyst_optimizer.fl` | Core optimization — outcome prediction, feedback recording, suggestion generation, model drift evaluation |

## Stratum Usage

- **KV storage** — outcome records, optimization suggestions, feedback loop state
- **Series** — 7y retention for outcomes (compliance), 365d for suggestions, forever for feedback loop metrics
- **Streams** — event streams with consumers for StreamSight, Cortex, analytics, alerting

## Install

```
estream marketplace install estream-catalyst
```

## Cortex AI Integration

- **Outcome predictor** — `catalyst_outcome_predictor` predicts approval/denial from application type, features, evidence count, and timeline; retrains on drift (threshold 0.05)
- **Drift detector** — `catalyst_drift_detector` triggers retraining when accuracy drops below 0.75
- **Suggestion engine** — AI-driven recommendations (add evidence, reword narrative, escalate review, timing optimization)

## Security

- ML-DSA-87 signatures on all mutations
- ML-KEM-1024 key encapsulation
- PoVC attestation on outcome records with Groth16 proofs
- PII obfuscation via Cortex governance (application_id obfuscated, features_hash redacted for analytics)
- StreamSight anomaly detection on every circuit

## Dependencies

- `estream-crm ^0.1.0` — application lifecycle and user profile data for prediction features
