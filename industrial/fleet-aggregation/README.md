# eStream Fleet Aggregation

Multi-site fleet aggregation component for industrial IoT deployments on the eStream platform.

## Overview

The Fleet Aggregation component provides a complete site-to-region-to-fleet rollup pipeline with health scoring, cross-site performance comparison, anomaly correlation, and pluggable AI model hooks. It is extracted from production ThermogenZero fleet analytics and generalized for any multi-site industrial deployment.

## Features

- **Site ingestion** with deduplication, normalization, and storm-mode rate limiting
- **Region mapping** with configurable site-to-region assignment and hot-reload
- **Fleet KPI rollup** with weighted-mean aggregation and configurable intervals
- **Regional health scoring** with tunable weights per domain
- **Cross-site comparison** using modified Z-score outlier detection
- **Anomaly correlation** across sites with pattern detection and root-cause inference
- **Pluggable AI model slots** (up to 16) with fan-out routing and confidence thresholds
- **StreamSight instrumentation** at every node for operational visibility

## Profiles

| Profile | Sites | Regions | AI Slots | Ingestion Mode |
|---------|-------|---------|----------|----------------|
| Small   | 1–50  | 4       | 2        | Full ingestion |
| Medium  | 50–200 | 16     | 8        | Downsample healthy |
| Large   | 200–1000 | 64   | 16       | Deviation-only |

## Quick Start

```fastlang
use estream_marketplace::fleet_aggregation::create_fleet
use estream_marketplace::fleet_aggregation::fleet_profile_medium
use estream_marketplace::fleet_aggregation::default_weights_energy

let profile = fleet_profile_medium()
let config = FleetConfig {
    profile: profile,
    region_mappings: [...],
    health_weights: default_weights_energy(),
    ai_models: [...],
}
let fleet = create_fleet(config)
```

## Domain Weight Presets

Pre-tuned health scoring weights for common industrial verticals:

- **Energy** — thermoelectric, solar, wind. Emphasizes uptime and thermal KPIs.
- **Manufacturing** — SCADA, PLC, production line. Emphasizes throughput and fault rate.
- **Logistics** — fleet tracking, asset monitoring. Balanced across all KPIs.

## Platform Circuit

This marketplace component packages the platform-level circuit at `estream/circuits/services/fleet/fleet_aggregation.fl` (`esn.fleet`) with marketplace-specific configuration profiles, domain weight presets, and documentation.

## Spec

See [ESTREAM-FLEET-001](specs/FLEET_AGGREGATION_SPEC.md) for the full specification.

## License

Apache-2.0
