# estream-marketplace

This repository contains **marketplace components** for the eStream platform — SmartCircuits, domain packages, and specifications that are distributed separately from the core platform.

## Directory Layout

- **`fintech/`** — Financial technology components organized by sub-domain (FIX trading, PCI, ISO 20022). FastLang `.fl` files are the canonical source; `.escir.yaml` files exist where no `.fl` equivalent has been written yet.
- **`industrial/`** — Industrial protocol gateway components. `components/` has individual ESCIR sub-circuits; `gateway/` has composite SKU circuits (Lite/Standard/Premium); `specs/` has the gateway specifications.
- **`registry/`** — Domain package registry circuits: package format, dependency resolution, mirrors/cache (Epic 4, #4).
- **`licensing/`** — ZK licensing circuits: blinded tokens, metering, atomic settlement, pricing tiers (Epic 5, #5).
- **`solutions/`** — Solution bundle circuits: manifest, lex boundary nesting, revenue waterfall, customer onboarding (Epic 6, #6).
- **`console/`** — Console circuits: publisher/customer/admin dashboards, developer tooling (Epic 7, #7).
- **`pricing/`** — Provider-level custom pricing circuits.
- **`streams/`** — Graph-based marketplace registry model (marketplace_streams.fl).
- **`runtime/`** — Rust runtime crates (`estream-iso20022`, `estream-industrial`) that implement the marketplace components as native executables.
- **`specs/`** — Cross-cutting marketplace specifications and standards.
- **`docs/guides/`** — Publisher, customer, and developer guides.
- **`templates/`** — ESCIR circuit templates for marketplace patterns.

## Conventions

- `.fl` is the path forward for all new components. New marketplace circuits should be written in FastLang.
- `.escir.yaml` circuits remain where no `.fl` replacement exists (e.g., ISO 20022 parser, industrial sub-circuits).
- Specs co-locate with their domain when domain-specific (e.g., `industrial/specs/`), or live in the top-level `specs/` directory when cross-cutting.

## Relationship to estream / estream-io

This repo was carved out per `polyquantum/estream#40`. The main repos retain:
- SmartCircuit compiler and FastLang toolchain
- Marketplace API types (generated from `.fl`)
- Platform circuits (consensus, governance, crypto, etc.)

## Developer Language Story (v0.9.1)

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
