# eStream Marketplace

The eStream Marketplace is an open source component exchange for discovering, publishing, installing, and composing reusable eStream components. It distributes six types of components — data schemas, SmartCircuits, wire adapters, FPGA circuits, console widgets, and full integrations — all signed with ML-DSA-87 post-quantum signatures.

## Quick Start

```bash
# Search for components
estream marketplace search "FIX adapter"

# Install a component
estream marketplace install estream-wire-fix

# Create your own component
estream marketplace scaffold smart-circuit my-circuit

# Validate before publishing
estream marketplace publish my-circuit --dry-run

# Publish with ML-DSA-87 signing
estream marketplace publish my-circuit
```

## Component Categories

| Category | Description | Example |
|----------|-------------|---------|
| `data-schema` | Reusable data type definitions | `data-trading`, `data-iot`, `data-carbon` |
| `wire-adapter` | Protocol adapters (MQTT, FIX, HL7, Modbus, SWIFT) | `estream-wire-fix`, `estream-wire-mqtt` |
| `smart-circuit` | SmartCircuit logic targeting CPU/WASM | `order-matcher`, `carbon-credit-mint` |
| `fpga-circuit` | FPGA-targetable circuits with Verilog output | `ntt-accelerator` |
| `integration` | Full-stack domain integration bundles | Carbon credit pipeline |
| `console-widget` | Dashboard widgets for the eStream console | `impact-counter` |

## Documentation

| Guide | Description |
|-------|-------------|
| [Getting Started](docs/getting-started.md) | 5-minute quickstart — install, create, and publish your first component |
| [Component Guide](docs/component-guide.md) | Deep dive into component structure, categories, and the manifest format |
| [CLI Reference](docs/cli-reference.md) | Complete reference for all 7 marketplace commands |
| [Pricing Guide](docs/pricing-guide.md) | 6 pricing models and 4 visibility levels explained |
| [Security Model](docs/security-model.md) | ML-DSA-87 post-quantum signatures and SPARK authentication |
| [API Reference](docs/api-reference.md) | Stream API topics, data types, and subscription patterns |
| [FAQ](docs/faq.md) | Common questions about the marketplace |

## Branding

| Document | Description |
|----------|-------------|
| [Brand Guidelines](branding/BRAND_GUIDELINES.md) | Color palette, typography, component card specs, badge designs |
| [Badge Descriptions](branding/badge-descriptions.md) | Verified, Official, PQ-Signed, FPGA-Ready, and Community badges |

## Architecture

```
CLI (estream marketplace)              Console (Marketplace Tab)
┌──────────────────────────┐          ┌──────────────────────────┐
│ search · install · pub   │          │ Browse · Install · Rate  │
│ verify · scaffold        │          │ Component Cards           │
└────────────┬─────────────┘          └────────────┬─────────────┘
             │                                      │
             ▼                                      ▼
┌──────────────────────────────────────────────────────────────┐
│              Component Registry (ML-DSA-87 Signed)            │
├──────────────────────────────────────────────────────────────┤
│ Data Schemas · SmartCircuits · Wire Adapters · Widgets        │
│ FPGA Circuits · Full Integrations                            │
├──────────────────────────────────────────────────────────────┤
│ Stream API: /marketplace/index, /search, /install, /publish  │
└──────────────────────────────────────────────────────────────┘
```

## Contributing

1. Fork the repository
2. Scaffold a new component: `estream marketplace scaffold <category> <name>`
3. Add your schemas, circuits, or widgets
4. Validate: `estream marketplace publish --dry-run .`
5. Submit a pull request

All contributions must pass manifest validation and ML-DSA-87 signing. See the [Component Guide](docs/component-guide.md) for authoring details and the [Security Model](docs/security-model.md) for signing requirements.

## License

Open source components: Apache 2.0
Source available components: BSL 1.1
Enterprise components: Commercial license

---

**PolyQuantum** | [eStream Platform](https://github.com/polyquantum/estream)
