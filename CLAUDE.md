# estream-marketplace

This repository contains **marketplace components** for the eStream platform — SmartCircuits and specifications that are distributed separately from the core platform.

## Directory Layout

- **`fintech/`** — Financial technology components organized by sub-domain (FIX trading, PCI, ISO 20022). FastLang `.fl` files are the canonical source; `.escir.yaml` files exist where no `.fl` equivalent has been written yet.
- **`industrial/`** — Industrial protocol gateway components. `components/` has individual ESCIR sub-circuits; `gateway/` has composite SKU circuits (Lite/Standard/Premium); `specs/` has the gateway specifications.
- **`runtime/`** — Rust runtime crates (`estream-iso20022`, `estream-industrial`) that implement the marketplace components as native executables.
- **`specs/`** — Cross-cutting marketplace specifications (registry API, package format, FPGA extension, console widgets).
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
