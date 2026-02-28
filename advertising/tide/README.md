# estream-tide

Organic growth engine for eStream — SEO optimization, traffic analysis, content strategy, and AI-driven growth feedback loops.

## Overview

`estream-tide` is a generic organic growth platform built on Stratum and Cortex. It ingests traffic metrics, detects growth signals (spikes, drops, ranking changes, competitor moves, seasonal trends), ranks content by SEO performance, and forecasts future traffic. The `tide_content_ranker` model retrains on drift to adapt to evolving search algorithms and audience behavior.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `tide_growth` | `circuits/tide_growth.fl` | Growth engine — traffic analysis, content ranking, growth forecasting, anomaly detection |

## Stratum Usage

- **KV storage** — growth signals, content performance records
- **Graph storage** — content-to-keyword relationship graph (CSR on BRAM, 100K capacity)
- **Series** — 365d retention for traffic and content metrics, 90d for growth signals
- **Streams** — event streams with consumers for StreamSight, Cortex, analytics, alerting

## Install

```
estream marketplace install estream-tide
```

## Cortex AI Integration

- **Growth analyzer** — `tide_growth_analyzer` runs daily scheduled inference on traffic metrics to detect patterns and growth signals
- **Content ranker** — `tide_content_ranker` predicts search ranking and improvement suggestions from engagement metrics; retrains on drift
- **Anomaly detector** — StreamSight inline anomaly detection on traffic patterns for real-time alerting

## Security

- ML-DSA-87 signatures on all mutations
- ML-KEM-1024 key encapsulation
- PoVC attestation on traffic metrics and growth signals
- PII obfuscation via Cortex governance (content_id obfuscated, keyword data redacted for analytics)
- StreamSight anomaly detection on every circuit

## Dependencies

- `estream-sage ^0.1.0` — content generation and SEO foundation for growth optimization
