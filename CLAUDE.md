# estream-marketplace

This repository contains **marketplace components** for the eStream platform — SmartCircuits, domain packages, and specifications that are distributed separately from the core platform.

**Platform version**: v0.22.0

## Directory Layout

- **`fintech/`** — Financial technology components organized by sub-domain (FIX trading, PCI, ISO 20022, cross-border). Published as `polyquantum.fintech`.
- **`industrial/`** — Industrial protocol gateway components: fleet aggregation, design explorer. Published as `polyquantum.industrial`.
- **`advertising/`** — Ad tech components: Etch (impressions), Catalyst (optimization), CRM, Optin, Portal, Relay, Sage, Scope, Thread, Tide. Published as `polyquantum.advertising`.
- **`payments/`** — ISO 20022, HTLC, offline payments, settlement DAG. Published as `polyquantum.payments`.
- **`radio/`** — 5G/6G spectrum management, tactical mesh, beamforming. Published as `polyquantum.radio`.
- **`secure-video/`** — SVD + content security pipelines. Published as `polyquantum.secure_video`.
- **`bridge/`** — Cross-chain, cross-lex bridge circuits. Published as `polyquantum.bridge`.
- **`widgets/`** — UI interaction components for eStream console. Published as `polyquantum.widgets`.
- **`registry/`** — Domain package registry circuits: package format, dependency resolution, mirrors/cache.
- **`licensing/`** — ZK licensing circuits: blinded tokens, metering, atomic settlement, pricing tiers.
- **`solutions/`** — Solution bundle circuits: manifest, lex boundary nesting, revenue waterfall, customer onboarding.
- **`console/`** — Console circuits: publisher/customer/admin dashboards, developer tooling.
- **`pricing/`** — Provider-level custom pricing circuits.
- **`streams/`** — Graph-based marketplace registry model and Platform Graph inventory (marketplace_streams.fl, marketplace_inventory.fl).
- **`specs/`** — Cross-cutting marketplace specifications and standards.
- **`docs/guides/`** — Publisher, customer, and developer guides.

## Platform / Marketplace Boundary

See `estream/docs/PLATFORM_MARKETPLACE_BOUNDARY.md` for the canonical boundary definition.

**Platform-native** (ships with eStream): `circuits/core/`, `circuits/services/`, `circuits/hardware/`, `circuits/modules/`, `circuits/sdk/`, plus companion, detection, registry, and marketplace verticals.

**Marketplace packages** (this repo): domain-specific monetizable solutions built on the platform API.

## Conventions

- `.fl` is the canonical format for all circuits. FastLang v0.22.0 syntax:
  - **Zero hand-written Rust**: All application logic is in FL. Only `extern "rust"` for I/O boundaries
  - **Stratum / Cortex / AI triad**: Data model (store kv/graph/series) + Rule engine (cortex) + Intelligence (AI)
  - Annotations use `@` prefix: `@status production`, `@precision`, `@attested true`, etc.
  - `observe metrics: [metric_list]` for explicit observability instrumentation
  - `schema` blocks with `sign { algorithm mldsa87 }` and `attest { proof_system groth16 }` for signed data
  - `state_machine` with `persistence wal`, `ai_anomaly_detection true`, and formal verification (`verify deadlock_free`, `verify reachable_all`, `verify terminal_convergence`)
  - `store kv` / `store graph` / `store series` for declarative persistent storage
  - `cortex` blocks for AI governance (`expose`, `infer on_write`, `on_anomaly alert`)
  - `cloud` circuits for multi-provider deployment with tier progression
  - Lex paths omit the `esn/` prefix (e.g., `lex fin/pci/org/estream/trading`)
  - Platform crypto is exclusively PRIME: SHA-3 (KECCAK), ML-DSA-87, ML-KEM-1024
- All `estream-component.toml` manifests specify `platform_minimum = "0.22.0"`
- FLIR (internal IR) replaces all former ESCIR references
- Specs co-locate with their domain when domain-specific, or live in top-level `specs/` when cross-cutting

## Relationship to estream / estream-io

This repo was carved out per `polyquantum/estream#40`. The main repos retain:
- SmartCircuit compiler and FastLang toolchain
- Marketplace API types (generated from `.fl`)
- Platform circuits (consensus, governance, crypto, etc.)

## Developer Language Story (v0.22.0)

eStream supports **7 languages** at full parity: Rust (native), Python (PyO3), TypeScript (WASM), Go (CGo), C++ (FFI), Swift (C bridging), and FastLang (native).

### External Messaging

- Lead with **"7 supported languages"** — developers choose the language they already know
- Position FastLang as **"the shortest path to silicon"** — the easiest way to design for eStream hardware
- **FLIR (FastLang Intermediate Representation) is strictly internal** — never mention it in external-facing materials, docs, pitches, or marketing. It is an implementation detail of the compiler
- Swift (not Solidity) is the 7th language

### Internal Development

- **FastLang first**: all new circuits and features are authored in FastLang (.fl) first
- **Six-language parity**: every FastLang feature must have equivalent API surface in Rust, Python, TypeScript, Go, C++, and Swift. Do not ship a FastLang-only feature
- Implementation types: FastLang (.fl), Hybrid (FastLang + Rust/RTL), Pure Rust, Pure RTL, Platform (tooling)
- FLIR operations power the compiler pipeline but are invisible to users

## Cross-Repo Coordination

This repo is part of the [polyquantum](https://github.com/polyquantum) organization, coordinated through the **AI Toolkit hub** at `toddrooke/ai-toolkit/`.

For cross-repo context, strategic priorities, and the master work queue:
- `toddrooke/ai-toolkit/CLAUDE-CONTEXT.md` — org map and priorities
- `toddrooke/ai-toolkit/scratch/BACKLOG.md` — master backlog
- `toddrooke/ai-toolkit/repos/polyquantum.md` — this org's status summary
