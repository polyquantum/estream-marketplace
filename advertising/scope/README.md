# estream-scope

Lead partner console for eStream — partner dashboard with AI-driven projections, lead quality scoring, budget management, and ROI analytics.

## Overview

`estream-scope` provides a complete partner management and analytics layer. Partners configure budgets, cost-per-lead targets, and volume goals. The console surfaces real-time pipeline views with leads generated, qualified, converted, and associated ROI. Cortex AI powers lead quality scoring across engagement, authenticity, intent, and fit dimensions, plus revenue forecasting with market trend detection.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `scope_console` | `circuits/scope_console.fl` | Partner configuration, lead quality scoring, pipeline summary views |
| `scope_projections` | `circuits/scope_projections.fl` | AI revenue projections, market trend detection, projection comparison |

## Stratum Usage

- **KV storage** — fast lookup for partner configs, quality scores, projections by ID
- **Series** — 365d retention for partner configs and pipeline views, 90d for quality scores
- **Streams** — event streams consumed by StreamSight, Cortex, analytics, alerting

## Install

```
estream marketplace install estream-scope
```

## Cortex AI Integration

- **Lead quality scorer** — multi-dimensional scoring (engagement, authenticity, intent, fit) from user activity and engagement events
- **Revenue forecaster** — projects leads, revenue, and ROI from historical pipeline, budget, and market trends
- **Market trend detector** — identifies seasonal, competitive, regulatory, and viral market shifts

## Security

- ML-DSA-87 signatures on all mutations
- ML-KEM-1024 key encapsulation
- PoVC attestation on pipeline views and projections
- PII obfuscation/redaction via Cortex governance
- StreamSight anomaly detection on every circuit
