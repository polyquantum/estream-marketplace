# estream-marketplace

SmartCircuit Marketplace for the eStream platform — domain-organized marketplace components including FIX trading, ISO 20022 financial messaging, PCI compliance, and industrial protocol gateways.

## Structure

```
estream-marketplace/
├── fintech/
│   ├── fix-trading/          FIX protocol trading gateway (FastLang)
│   ├── pci/                  PCI-DSS cardholder data governance (FastLang)
│   └── iso20022/             ISO 20022 parser circuit + test vectors (ESCIR)
├── industrial/
│   ├── components/           Modbus TCP sub-circuits (ESCIR)
│   ├── gateway/              Gateway SKUs: Lite/Standard/Premium (ESCIR)
│   └── specs/                Industrial gateway specifications
├── runtime/
│   ├── iso20022/             ISO 20022 Rust runtime crate
│   └── industrial/           Industrial gateway Rust runtime crate
├── specs/                    Cross-cutting marketplace specifications
└── templates/                Circuit templates
```

## Domains

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

## SmartCircuit Package Format

Third-party packages follow the format defined in `specs/SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md`. Install via:

```bash
estream marketplace install <package-name>
```
