# estream-marketplace

This repository contains **marketplace components** for the eStream platform — SmartCircuits, domain packages, and specifications that are distributed separately from the core platform.

**Platform version**: v0.12.0

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
- **`streams/`** — Graph-based marketplace registry model (marketplace_streams.fl).
- **`runtime/`** — Rust runtime crates that implement marketplace components as native executables.
- **`specs/`** — Cross-cutting marketplace specifications and standards.
- **`docs/guides/`** — Publisher, customer, and developer guides.

## Platform / Marketplace Boundary

See `estream/docs/PLATFORM_MARKETPLACE_BOUNDARY.md` for the canonical boundary definition.

**Platform-native** (ships with eStream): `circuits/core/`, `circuits/services/`, `circuits/hardware/`, `circuits/modules/`, `circuits/sdk/`, plus companion, detection, registry, and marketplace verticals.

**Marketplace packages** (this repo): domain-specific monetizable solutions built on the platform API.

## Conventions

- `.fl` is the canonical format for all circuits. FastLang v0.12.0 syntax:
  - Annotations use `@` prefix: `@lex`, `@precision`, `@attested`, `@observe`, `@serialize`, etc.
  - `attested` replaces `povc` for proof-of-circuit attestation
  - `@observe true` replaces `streamsight true` for observability
  - `@serialize` replaces `esz_emit` for verification artifact output
  - `@golden_test` replaces `kat_vector` for known-answer test vectors
  - Lex paths omit the `esn/` prefix (e.g., `@lex fin/pci/org/estream/trading`)
- All `estream-component.toml` manifests specify `platform = ">= 0.12.0"`
- Specs co-locate with their domain when domain-specific, or live in top-level `specs/` when cross-cutting

## Relationship to estream / estream-io

This repo was carved out per `polyquantum/estream#40`. The main repos retain:
- SmartCircuit compiler and FastLang toolchain
- Marketplace API types (generated from `.fl`)
- Platform circuits (consensus, governance, crypto, etc.)

## Developer Language Story (v0.12.0)

eStream supports **7 languages** at full parity: Rust (native), Python (PyO3), TypeScript (WASM), Go (CGo), C++ (FFI), Swift (C bridging), and FastLang (native).

### External Messaging

- Lead with **"7 supported languages"** — developers choose the language they already know
- Position FastLang as **"the shortest path to silicon"** — the easiest way to design for eStream hardware
- **ESCIR (eStream Circuit Intermediate Representation) is strictly internal** — never mention it in external-facing materials, docs, pitches, or marketing. It is an implementation detail of the compiler
- Swift (not Solidity) is the 7th language

### Internal Development

- **FastLang first**: all new circuits and features are authored in FastLang (.fl) first
- **Six-language parity**: every FastLang feature must have equivalent API surface in Rust, Python, TypeScript, Go, C++, and Swift. Do not ship a FastLang-only feature
- Implementation types: FastLang (.fl), Hybrid (FastLang + Rust/RTL), Pure Rust, Pure RTL, Platform (tooling)
- ESCIR operations power the compiler pipeline but are invisible to users

## Cross-Repo Coordination

This repo is part of the [polyquantum](https://github.com/polyquantum) organization, coordinated through the **AI Toolkit hub** at `toddrooke/ai-toolkit/`.

For cross-repo context, strategic priorities, and the master work queue:
- `toddrooke/ai-toolkit/CLAUDE-CONTEXT.md` — org map and priorities
- `toddrooke/ai-toolkit/scratch/BACKLOG.md` — master backlog
- `toddrooke/ai-toolkit/repos/polyquantum.md` — this org's status summary
