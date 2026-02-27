# estream-marketplace

SmartCircuit Marketplace for the eStream platform — domain-organized marketplace components including FIX trading, ISO 20022 financial messaging, PCI compliance, industrial protocol gateways, and the full domain package registry, ZK licensing, and solution bundle infrastructure.

## Structure

```
estream-marketplace/
├── registry/                 Domain Package Registry (Epic 4)
│   ├── package_format.fl         .escx format, manifest, attestation, signing
│   ├── dependency_resolution.fl  DAG topo-sort, version constraints, conflict detection
│   └── package_mirror.fl         Local cache, scatter-distributed mirrors
├── licensing/                ZK Licensing & Atomic Settlement (Epic 5)
│   ├── blinded_tokens.fl         Chaum-style blinded license tokens (PQ-safe)
│   ├── metering.fl               Per-minute tracking, hourly blind aggregation
│   ├── settlement.fl             Atomic multi-party settlement, ZK revenue proofs
│   └── pricing_tiers.fl          Share vs. private tiers, telemetry preferences
├── solutions/                Solution Bundles — White-Label Distribution (Epic 6)
│   ├── solution_manifest.fl      Bundle definition, package composition
│   ├── lex_boundary.fl           Hierarchical lex nesting (Platform→Solution→Customer)
│   ├── revenue_waterfall.fl      Atomic revenue splits through tier chain
│   └── customer_onboarding.fl    Provisioning, upgrades, rollback
├── console/                  Console & Developer Experience (Epic 7)
│   ├── publisher_console.fl      Publisher dashboard, analytics, deprecation
│   ├── customer_console.fl       Installed packages, licenses, billing
│   ├── admin_console.fl          Marketplace health, compliance, ESCIR matrix
│   └── dev_tooling.fl            Dev server, testing, linting, doc generation
├── fintech/
│   ├── fix-trading/          FIX protocol trading gateway (FastLang)
│   ├── pci/                  PCI-DSS cardholder data governance (FastLang)
│   └── iso20022/             ISO 20022 parser circuit + test vectors (ESCIR)
├── industrial/
│   ├── components/           Modbus TCP sub-circuits (ESCIR)
│   ├── gateway/              Gateway SKUs: Lite/Standard/Premium (ESCIR)
│   └── specs/                Industrial gateway specifications
├── pricing/                  Provider-level custom pricing (FastLang)
├── streams/                  Graph-based registry model (FastLang)
├── runtime/
│   ├── iso20022/             ISO 20022 Rust runtime crate
│   └── industrial/           Industrial gateway Rust runtime crate
├── specs/
│   ├── ESTREAM_MARKETPLACE_SPEC.md   Canonical v2.0.0 spec
│   ├── standards/
│   │   ├── ESCX_FORMAT_SPEC.md       .escx binary format specification
│   │   ├── MANIFEST_SCHEMA_SPEC.md   manifest.toml full schema
│   │   └── PRIVACY_GUARANTEES_SPEC.md Privacy & ZK proof specs
│   └── archive/              Superseded specs
├── docs/
│   ├── guides/
│   │   ├── publisher-getting-started.md
│   │   ├── customer-guide.md
│   │   ├── pricing-strategy.md
│   │   └── solution-builder.md
│   └── (language SDK docs)
└── templates/                Circuit templates
```

## Marketplace Infrastructure (Epics 4–8)

### Registry & Distribution (Epic 4)

Domain package (.escx) registry with PoVC attestation, ML-DSA-87 signing, DAG-based dependency resolution, and scatter-distributed mirrors. 16 circuits across package format, dependency resolution, and caching.

### ZK Licensing & Atomic Settlement (Epic 5)

Privacy-preserving licensing with Chaum-style blinded tokens (PQ-safe via ML-KEM + PRIME). Per-minute metering with hourly blind aggregation. Atomic multi-party settlement (publisher + platform + referrer in one transaction). ZK revenue proofs. Share vs. private pricing tiers. 20 circuits.

### Solution Bundles (Epic 6)

White-label distribution with three-tier model: Platform → Solution → Customer. Each tier has its own hardware-attested lex boundary. Revenue flows up atomically through the waterfall. 23 circuits covering bundles, lex nesting, waterfall settlement, and customer lifecycle.

### Console & Developer Experience (Epic 7)

Publisher, customer, and admin consoles plus developer tooling (dev server, test runner, linter, doc generator). 24 circuits. CLI: `estream marketplace {dev, test, lint, docs, solution, update}`.

### Documentation & Standards (Epic 8)

Publisher and customer guides, pricing strategy, solution builder guide. Formal specifications: .escx binary format, manifest.toml schema, privacy guarantees (what the marketplace operator cannot learn), ZK proof specs.

## Domain Components

### Fintech

**FIX Trading** — Three-component decomposition for FIX 4.2/4.4/5.0 protocol integration:
- `trading_schemas.fl` — Order, Fill, Quote, MarketData data types with validation circuits
- `fix_wire_adapter.fl` — FIX message parsing/serialization, session management
- `fix_trading_gateway.fl` — Composite gateway wiring all components together

**PCI** — PCI-DSS cardholder data governance with field-level tokenization and sub-lex fan_out.

**ISO 20022** — Full ESCIR circuit for ISO 20022 XML/JSON parsing with PoVC witness generation. Includes pacs.008 and pacs.002 test vectors.

### Industrial

**Components** — Individual ESCIR sub-circuits: Modbus TCP client, poll scheduler, stream emitter, StreamSight bridge.

**Gateway SKUs** — Composite gateway circuits at three tiers:
- **Lite** (free/Apache-2.0) — 10 devices, 256 registers
- **Standard** ($100/mo) — 50 devices, serial/RTU, FPGA acceleration
- **Premium** ($300/mo) — 200 devices, DNP3, OPC-UA, hot standby, PoVC attestation

## Runtime Crates

### estream-iso20022

ISO 20022 financial messaging adapter. Converts between ISO 20022 XML/JSON and eStream wire format.

### estream-industrial

Industrial protocol gateway supporting Modbus TCP/RTU, OPC-UA, and StreamSight telemetry.

```bash
cd runtime/iso20022 && cargo build
cd runtime/industrial && cargo build
```

## Relationship to Main Repo

This repo was separated from `polyquantum/estream` (Wave 3, Issue #40). The main repo retains:
- SmartCircuit compiler support (`compiler/`)
- Marketplace API types (generated from `.fl`)
- Testing fixtures (`testing/`)

## Package Format

Domain packages use the `.escx` format defined in `specs/standards/ESCX_FORMAT_SPEC.md`. Component packages follow `specs/ESTREAM_MARKETPLACE_SPEC.md`.

```bash
estream marketplace install <package-name>
estream marketplace search "thermal analysis"
estream marketplace publish .
estream marketplace solution create my-solution
estream marketplace dev .
estream marketplace test .
estream marketplace lint .
```
