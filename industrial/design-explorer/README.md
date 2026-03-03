# eStream Design Explorer

Multi-physics parametric design space exploration for industrial system design on the eStream platform.

## Overview

The Design Explorer component provides a domain-agnostic NSGA-II multi-objective optimization framework that domain-specific publishers can extend with their own parameter groups, objectives, constraints, and physics evaluation pipelines. It leverages the eStream platform's FEA, CFD, thermal-hydraulic, combustion, piping, pressure vessel, and HAZOP engines as pluggable evaluation stages.

The framework is analogous to EDA place-and-route for chip design: the optimizer explores hundreds of thousands of design candidates, evaluates each through a multi-physics pipeline, and produces a Pareto frontier of non-dominated designs with full BOM, CAD, and engineering reports.

## Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│  Domain Extension (publisher-defined)                              │
│  • Parameter groups (what gets optimized)                          │
│  • Objectives (what's scored)                                      │
│  • Constraints (what's required)                                   │
│  • Physics pipeline stages (how candidates are evaluated)          │
├────────────────────────────────────────────────────────────────────┤
│  Design Explorer Core (this component)                             │
│  • NSGA-II population management                                   │
│  • Pareto sort + crowding distance                                 │
│  • Tournament selection, SBX crossover, polynomial mutation        │
│  • Convergence detection + campaign orchestration                  │
│  • Output generation (Pareto SVG, BOM CSV, report)                 │
├────────────────────────────────────────────────────────────────────┤
│  eStream Platform (built-in)                                       │
│  • FEA, CFD, combustion, TH network, piping, vessel, HAZOP, P&ID  │
│  • Graph + overlay + AI feeds + series                             │
│  • FPGA acceleration for compute-bound stages                      │
│  • PoVC, StreamSight, Cortex privacy                               │
└────────────────────────────────────────────────────────────────────┘
```

## Features

- **Domain-agnostic optimizer** — NSGA-II with configurable population size (200–1,000), generations (100–500), and up to 500,000 total evaluations
- **Pluggable parameter groups** — define any number of typed parameters with ranges, enums, or discrete sets
- **Pluggable objectives** — define minimization/maximization targets with configurable weights
- **Hard + soft constraints** — hard constraints eliminate infeasible designs; soft constraints penalize with configurable severity
- **Physics pipeline registration** — chain any sequence of platform physics engines (FEA, CFD, TH, combustion, piping stress, vessel code check, HAZOP) as evaluation stages
- **FPGA-accelerated stages** — compute-bound physics stages (material lookup, I-V curves, flow solvers) can target FPGA kernels for 100-2,700x speedup
- **AI surrogate pre-screening** — Cortex AI feed learns from evaluated candidates to pre-screen obviously infeasible designs, reducing unnecessary full-physics evaluations by 40-60%
- **Pareto frontier export** — SVG visualization, BOM CSV, STEP CAD references, and PDF engineering reports for each non-dominated design
- **StreamSight instrumentation** — convergence tracking, generation-level statistics, constraint violation heatmaps, and objective correlation analysis
- **Campaign persistence** — campaigns are merkle-chained in series storage for auditability and resumability

## Domain Extension Points

### Parameter Groups

```fastlang
use estream_marketplace::design_explorer::dse_define_param_group

let teg_module = dse_define_param_group(campaign_id, DseParamGroup {
    name: "teg_module",
    params: [
        DseParam { name: "material", kind: ENUM, values: [BITE, PBTE, TETRA, SKUT] },
        DseParam { name: "face_mm", kind: CONTINUOUS, min: 20, max: 62 },
        DseParam { name: "leg_height_um", kind: CONTINUOUS, min: 500, max: 3000 },
        DseParam { name: "pairs_count", kind: INTEGER, min: 31, max: 241 },
    ],
})
```

### Objectives

```fastlang
use estream_marketplace::design_explorer::dse_define_objective

let cost_obj = dse_define_objective(campaign_id, DseObjective {
    name: "cost_per_kw",
    direction: MINIMIZE,
    unit: "USD/kW",
})
let efficiency_obj = dse_define_objective(campaign_id, DseObjective {
    name: "thermal_efficiency",
    direction: MAXIMIZE,
    unit: "percent",
})
```

### Constraints

```fastlang
use estream_marketplace::design_explorer::dse_define_constraint

let atex_zone = dse_define_constraint(campaign_id, DseConstraint {
    name: "atex_zone_1_clearance",
    kind: HARD,
    check_stage: 12,    // evaluated at pipeline stage 12
    description: "ATEX Zone 1 minimum clearance from TEG arrays",
})
```

### Physics Pipeline Stages

```fastlang
use estream_marketplace::design_explorer::dse_register_physics_stage

dse_register_physics_stage(campaign_id, DsePhysicsStage {
    order: 1,
    name: "material_properties",
    engine: "lookup",           // built-in lookup table
    target: FPGA,               // FPGA-accelerated
})
dse_register_physics_stage(campaign_id, DsePhysicsStage {
    order: 5,
    name: "thermal_hydraulic",
    engine: "th_network",       // platform TH solver
    target: CPU,
})
```

## Domain Presets

Pre-configured domain extensions for common industrial verticals (each available as a separate marketplace component or included as presets):

| Preset | Parameter Groups | Objectives | Constraints | Physics Stages |
|--------|-----------------|------------|-------------|----------------|
| **TEG Power System** | 17 groups (90 params) | 10 | 10 | 14 stages |
| **Industrial Boiler** | 8 groups (~60 params) | 6 | 8 | 7 stages |
| **Chemical Reactor** | 10 groups (~80 params) | 8 | 12 | 9 stages |
| **HVAC System** | 6 groups (~40 params) | 5 | 6 | 5 stages |
| **Marine Propulsion** | 9 groups (~70 params) | 7 | 10 | 8 stages |

> Note: The TEG Power System preset is published separately by ThermogenZero as `tz-design-explorer-teg`. Other presets are contributed by their respective domain publishers.

## Profiles

| Profile | Population | Generations | Max Evaluations | Target |
|---------|-----------|-------------|-----------------|--------|
| Quick   | 200       | 100         | 20,000          | CPU    |
| Standard | 500      | 300         | 150,000         | CPU    |
| Deep    | 1,000     | 500         | 500,000         | CPU + FPGA |

## Circuits

| Circuit | Description |
|---------|-------------|
| `dse_create_campaign` | Initialize a new design exploration campaign with config |
| `dse_define_param_group` | Register a parameter group with typed parameters |
| `dse_define_objective` | Register an optimization objective |
| `dse_define_constraint` | Register a hard or soft constraint |
| `dse_register_physics_stage` | Add a physics evaluation stage to the pipeline |
| `dse_run_generation` | Execute one NSGA-II generation (evaluate → sort → select → breed) |
| `dse_evaluate_candidate` | Run a single candidate through the full physics pipeline |
| `dse_pareto_sort` | Non-dominated sort with crowding distance |
| `dse_crowding_distance` | Compute crowding distance for a population front |
| `dse_tournament_select` | Binary tournament selection for parent selection |
| `dse_sbx_crossover` | Simulated Binary Crossover |
| `dse_polynomial_mutation` | Polynomial mutation with adaptive step size |
| `dse_check_convergence` | Detect Pareto frontier stability / improvement stall |
| `dse_export_pareto` | Generate Pareto frontier SVG visualization |
| `dse_export_bom` | Generate BOM CSV for a selected design |
| `dse_export_report` | Generate PDF engineering report for a selected design |

## Stratum Graph Model

The design explorer uses the platform graph with:

- **Nodes**: `CandidateNode` (design vector), `GenerationNode` (population snapshot), `ParetoFrontNode` (non-dominated set), `PhysicsResultNode` (per-stage evaluation results)
- **Edges**: `DominatesEdge` (Pareto dominance), `CrossoverEdge` (parent → child), `MutationEdge` (mutation lineage), `EvalEdge` (candidate → physics result)
- **Overlays**: `fitness`, `constraint_violation`, `crowding_distance`, `convergence_metric`, `stage_time`, `memory_usage`
- **AI Feeds**: `surrogate_prescreener` (pre-filter infeasible candidates), `design_advisor` (suggest promising regions), `convergence_predictor` (estimate generations to convergence)
- **Series**: `dse_series` with merkle chain, lex imprint, witness attestation for full campaign audit trail

## Cortex AI Integration

- **Surrogate pre-screening**: After ~1,000 evaluated candidates, Cortex trains a fast surrogate model that predicts constraint violations and objective scores. Candidates predicted infeasible are skipped, reducing full-physics evaluations by 40-60%.
- **Design advisory**: Cortex analyzes the current Pareto frontier and suggests parameter combinations in under-explored regions, improving frontier diversity.
- **Convergence prediction**: Cortex estimates how many additional generations are needed for the frontier to stabilize, enabling early termination for time-constrained campaigns.

## Spec

See [DESIGN_EXPLORER_SPEC](specs/DESIGN_EXPLORER_SPEC.md) for the full specification.

## Install

```bash
estream marketplace install estream-design-explorer
```

## Security

- All campaign results are merkle-chained for tamper-evident audit trails
- PoVC attestation on evaluation circuits ensures physics computations are provably correct
- Cortex privacy controls (redact/obfuscate/expose) protect proprietary parameter ranges and material properties
- ML-DSA-87 signatures on exported reports and BOMs

## License

Apache-2.0
