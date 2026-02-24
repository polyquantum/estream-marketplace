# Fleet Aggregation Specification

| Field | Value |
|-------|-------|
| **Version** | v0.9.2 |
| **Status** | Active |
| **Tier** | Cloud |
| **Lex Namespace** | `esn/fleet` |
| **Issue** | [#193](https://github.com/polyquantum/estream/issues/193) |
| **Platform Circuit** | `estream/circuits/services/fleet/fleet_aggregation.fl` |
| **Marketplace Component** | `estream-marketplace/industrial/fleet-aggregation/` |

---

## 1. Purpose

The Fleet Aggregation Framework provides a generic, reusable circuit graph for aggregating telemetry, health metrics, and anomaly signals across a fleet of geographically distributed industrial sites. It is designed to be domain-agnostic at the platform level, with domain-specific behavior injected through configuration (region mappings, health weights, AI model slots).

The framework was extracted from ThermogenZero's production fleet analytics (TZ-SPEC-007, 957-line `fleet_analytics.fl`) by factoring out domain-specific types (TEG, carbon, MPPT) and replacing them with generic `SiteKPI` structures and pluggable AI inference slots.

---

## 2. Architecture

### 2.1 Pipeline

```
Site Telemetry → Ingest → Region Mapper → ┬→ Fleet Aggregator → Regional Scorer ─→ Dashboard
                                           ├→ Performance Comparator ────────────→ Dashboard
                                           ├→ Anomaly Scorer → Anomaly Correlator → Dashboard
                                           └→ AI Model Router → Inference Slots ──→ Dashboard
```

### 2.2 Node Roles

| Node | Type | Role |
|------|------|------|
| `site_ingest` | source | Dedup, normalize, rate-limit incoming site KPIs |
| `region_mapper` | transform | Assign site_id → region_id via config table |
| `fleet_aggregator` | transform | Site → region → fleet weighted-mean rollup |
| `regional_scorer` | transform | Compute per-region health score |
| `performance_comparator` | transform | Rank sites within region and fleet |
| `anomaly_scorer` | transform | Per-site anomaly detection (reconstruction error) |
| `anomaly_correlator` | transform | Cross-site pattern detection |
| `ai_model_router` | transform | Fan-out to active AI model slots |
| `ai_inference_slot` | ai_inference | Pluggable inference (up to 16 slots) |
| `dashboard_sink` | sink | Aggregate outputs for push to dashboard |

### 2.3 Rate Limiting

The `site_ingest` node implements adaptive rate limiting based on fleet size:

| Fleet Size | Ingestion Strategy |
|------------|-------------------|
| 1–50 | Full ingestion — all KPIs processed |
| 50–200 | Downsample healthy — only process degraded/faulted sites fully |
| 200–1000 | Deviation-only — only process sites that deviate from baseline |
| Storm | 10% sampling — used during fleet-wide event storms |

---

## 3. Types

All types are defined in the platform circuit (`fleet_aggregation.fl`). Key types:

- **SiteKPI** — Per-site telemetry snapshot (16 KPI values, status, uptime, fault count)
- **RegionMapping** — Site-to-region assignment configuration
- **RegionalHealth** — Aggregated regional health with per-region scoring
- **FleetKPI** — Fleet-wide aggregate (site counts by status, health score, anomaly count)
- **SiteRanking** — Per-site rank within region and fleet (composite score, outlier flag)
- **AnomalyScore** — Per-site anomaly detection result
- **FleetCorrelation** — Cross-site correlated anomaly pattern
- **HealthWeights** — Configurable weights for health score computation
- **AIModelSlot** — AI model registration and status
- **AIInferenceResult** — Inference output from pluggable AI models

---

## 4. Streams

| Stream | Event Type | Retention | Consumers |
|--------|-----------|-----------|-----------|
| `fleet_kpi_stream` | FleetKPI | 2 years | ops_dashboard, fleet_console, ai_training, streamsight |
| `regional_health_stream` | RegionalHealth | 1 year | ops_dashboard, fleet_console, streamsight |
| `site_ranking_stream` | SiteRanking | 90 days | ops_dashboard, fleet_console |
| `fleet_correlation_stream` | FleetCorrelation | 90 days | ops_dashboard, fleet_console, alerting |
| `ai_inference_stream` | AIInferenceResult | 90 days | ai_audit, streamsight, ops_dashboard |

---

## 5. Circuit Cross-References

| Circuit | File | Purpose |
|---------|------|---------|
| `FleetAggregation` (graph) | `circuits/services/fleet/fleet_aggregation.fl` | Platform-level fleet aggregation graph |
| `compute_health_score` | `circuits/services/fleet/fleet_aggregation.fl` | Weighted health score computation |
| `detect_outlier` | `circuits/services/fleet/fleet_aggregation.fl` | Modified Z-score outlier detection |
| `correlate_anomalies` | `circuits/services/fleet/fleet_aggregation.fl` | Cross-site anomaly pattern correlation |
| `fleet_profile_small` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Small fleet preset |
| `fleet_profile_medium` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Medium fleet preset |
| `fleet_profile_large` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Large fleet preset |
| `default_weights_energy` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Energy domain health weights |
| `default_weights_manufacturing` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Manufacturing domain health weights |
| `default_weights_logistics` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Logistics domain health weights |
| `create_fleet` | `marketplace/industrial/fleet-aggregation/circuits/fleet_aggregation.fl` | Marketplace entry point |

---

## 6. Marketplace Configuration

### 6.1 Profiles

Three preconfigured profiles trade off resource usage against fleet scale:

| Profile | Max Sites | Max Regions | AI Slots | Aggregation Interval |
|---------|----------|-------------|----------|---------------------|
| Small | 50 | 4 | 2 | 60s |
| Medium | 200 | 16 | 8 | 60s |
| Large | 1000 | 64 | 16 | 30s |

### 6.2 Domain Weight Presets

Each industrial domain has different KPI priorities. Pre-tuned weight vectors are provided for energy, manufacturing, and logistics. Custom weights can be provided at configuration time.

### 6.3 AI Model Slots

The framework supports up to 16 pluggable AI model slots. Each slot has:
- `model_id` — unique model identifier
- `domain` — which KPI domain this model analyzes
- `version` — model version for audit trail
- `active` — hot-enable/disable without restart
- Rate limiting per model (configurable, default 10 Hz)
- Confidence threshold for result filtering (default 0.5)

ThermogenZero uses 6 domain-specific models (maintenance predictor, MPPT optimizer, dispatch scheduler, fault classifier, cascade detector, carbon validator). Other verticals register their own models.

---

## 7. Provenance

- **Extracted from**: ThermogenZero `fleet_analytics.fl` (957 lines, TZ-SPEC-007)
- **Generalization**: Domain-specific types replaced with generic `SiteKPI`, fixed AI models replaced with pluggable slots, hardcoded regions replaced with config-driven mapping
- **Series integrity**: Merkle chain + lattice imprint + witness attestation on all fleet KPI history
