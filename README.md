# estream-marketplace

SmartCircuit Marketplace for the eStream platform — ISO 20022 financial messaging, industrial protocol gateways, and third-party SmartCircuit packages.

## Structure

```
estream-marketplace/
├── specs/                    Marketplace specifications
│   ├── MARKETPLACE_SPEC.md
│   ├── ESTREAM_MARKETPLACE_SPEC.md
│   ├── SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md
│   ├── COMPONENT_REGISTRY_API_SPEC.md
│   ├── CONSOLE_WIDGET_MARKETPLACE_SPEC.md
│   ├── INDUSTRIAL_PROTOCOL_GATEWAY.md
│   ├── INDUSTRIAL_PROTOCOL_GATEWAY_V2.md
│   └── FPGA_COMPONENT_EXTENSION.md
├── runtime/
│   ├── iso20022/             ISO 20022 financial messaging (Rust crate)
│   └── industrial/           Industrial protocol gateway (Modbus/OPC-UA)
└── README.md
```

## Crates

### estream-iso20022

ISO 20022 financial messaging adapter for eStream. Converts between ISO 20022 XML/JSON and eStream wire format.

### estream-industrial

Industrial protocol gateway supporting:
- **Modbus TCP/RTU** — PLC and sensor integration
- **OPC-UA** — Industrial automation
- **StreamSight** — Real-time telemetry bridge
- **Lite/Standard/Premium** gateway tiers

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

## Building

```bash
cd runtime/iso20022 && cargo build
cd runtime/industrial && cargo build
```
