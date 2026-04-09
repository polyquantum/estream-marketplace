# Getting Started with eStream Marketplace

> Install your first component in under 5 minutes.

---

## Prerequisites

| Requirement | Minimum Version | Install |
|-------------|----------------|---------|
| **Rust** | 1.75+ | [rustup.rs](https://rustup.rs) |
| **eStream CLI** | 0.8.0+ | `cargo install estream-cli` |

Verify your installation:

```bash
estream --version
# estream 0.8.3
```

---

## 1. Install Your First Component

The marketplace ships with official components ready to install. Let's start with a wire adapter:

```bash
estream marketplace install estream-wire-fix
```

Output:

```
  📦 Installing estream-wire-fix...

  Resolving dependencies...
    estream-wire-fix v1.0.0

  Verifying ML-DSA-87 signatures...
    estream-wire-fix v1.0.0 ✓

  ✓ Installed estream-wire-fix v1.0.0 (wire-adapter)
```

Pin a specific version with `@version` syntax:

```bash
estream marketplace install estream-wire-fix@1.0.0
```

Preview what would be installed without making changes:

```bash
estream marketplace install estream-wire-fix --dry-run
```

---

## 2. Search for Components

Browse available components by keyword or category:

```bash
estream marketplace search "FIX adapter"
```

Filter by category:

```bash
estream marketplace search "trading" --category data-schema
```

The 6 component categories are:

| Category | Description |
|----------|-------------|
| `data-schema` | Reusable data type definitions (`.data.yaml`) |
| `wire-adapter` | Protocol adapters (MQTT, FIX, HL7, Modbus, SWIFT) |
| `smart-circuit` | SmartCircuit logic (`.fl` files targeting CPU/WASM) |
| `fpga-circuit` | FPGA-targetable circuits with Verilog output |
| `integration` | Full-stack integration bundles |
| `console-widget` | Dashboard widgets for the eStream console |

---

## 3. Create Your First Component

Scaffold a new SmartCircuit component:

```bash
estream marketplace scaffold smart-circuit my-circuit
```

This generates:

```
my-circuit/
├── estream-component.toml    # Component manifest
├── README.md                 # Documentation
├── circuits/                 # FastLang circuit definitions
├── schemas/                  # Data schema files
└── tests/
    └── golden/               # Golden test vectors
```

Edit `estream-component.toml` with your details:

```toml
[component]
name = "my-circuit"
version = "0.1.0"
category = "smart-circuit"
description = "My first eStream SmartCircuit"
license = "Apache-2.0"
keywords = ["example"]

[component.author]
name = "Your Name"
email = "you@example.com"

[component.marketplace]
pricing = "free"
visibility = "open"

[component.estream]
min_version = "0.8.0"

[component.schemas]
provides = []
requires = []

[component.circuits]
provides = ["my_circuit"]
target = ["cpu"]

[component.include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
```

---

## 4. Validate and Publish

Run a dry-run to validate your component before publishing:

```bash
estream marketplace publish my-circuit --dry-run
```

Output:

```
  📤 Publish dry-run for my-circuit

  ✓ Manifest valid
    Name:     my-circuit
    Version:  0.1.0
    Category: smart-circuit
  ✓ Include globs resolve to 2 file(s)
    circuits/my_circuit.fl
    schemas/my_schema.data.yaml

  Checking 1 FastLang file(s)...
    ✓ circuits/my_circuit.fl

  ℹ Dry-run: skipping signature and archive emission

  Package Summary
  --------------------------------------------------
  Name:        my-circuit
  Version:     0.1.0
  Category:    smart-circuit
  Files:       2
  Author:      Your Name

  ✓ Publish dry-run passed
```

When ready, publish for real:

```bash
estream marketplace publish my-circuit
```

The CLI will:
1. Validate the `estream-component.toml` manifest
2. Resolve and verify all include globs
3. Check FastLang files with `compile --check`
4. Build a deterministic `tar.gz` archive
5. Sign with ML-DSA-87 (post-quantum signature)
6. Emit the package and `SIGNATURE.ml-dsa`

---

## 5. Verify and Inspect

Verify an installed component's signature:

```bash
estream marketplace verify estream-wire-fix
```

View detailed information:

```bash
estream marketplace info estream-wire-fix
```

List all installed components:

```bash
estream marketplace list
```

---

## End-to-End Walkthrough (5 Minutes)

```bash
# 1. Search for IoT data schemas
estream marketplace search "iot" --category data-schema

# 2. Install the IoT schema pack
estream marketplace install data-iot

# 3. Scaffold a new circuit that uses IoT schemas
estream marketplace scaffold smart-circuit iot-processor

# 4. Edit your circuit (add schemas/circuits as needed)
cd iot-processor
# ... edit estream-component.toml, add your .fl files ...

# 5. Validate
estream marketplace publish . --dry-run

# 6. Publish
estream marketplace publish .

# 7. Verify your published component
estream marketplace verify iot-processor
```

---

## Next Steps

- [Component Authoring Guide](./component-guide.md) — Deep dive into component structure
- [CLI Reference](./cli-reference.md) — Full command documentation
- [Pricing Guide](./pricing-guide.md) — Set up pricing for your components
- [Security Model](./security-model.md) — How post-quantum signing works
- [API Reference](./api-reference.md) — Stream API for programmatic access
- [FAQ](./faq.md) — Common questions answered
