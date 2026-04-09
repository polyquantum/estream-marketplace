# Component Authoring Guide

> Everything you need to know about creating, structuring, and publishing eStream Marketplace components.

---

## Anatomy of a Component

Every marketplace component follows the same structure, regardless of category:

```
my-component/
├── estream-component.toml    # Required: component manifest
├── README.md                 # Required: documentation
├── CHANGELOG.md              # Recommended: version history
├── schemas/                  # Data schema definitions
│   └── *.data.yaml
├── circuits/                 # FastLang circuit definitions
│   └── *.fl
├── tests/
│   └── golden/               # Golden test vectors
│       ├── *.input.json
│       └── *.expected.json
├── fpga/                     # FPGA-specific files (fpga-circuit only)
│   └── *.v
└── widgets/                  # Console widget bundles (console-widget only)
    └── dist/
```

The `estream-component.toml` manifest is the single source of truth. It declares the component's identity, dependencies, schemas, circuits, and marketplace settings.

---

## The 6 Component Categories

### `data-schema` — Data Type Definitions

Reusable data schemas that define structured message formats for eStream streams.

**Example:** `data-trading` provides `EStreamOrder`, `EStreamFill`, `EStreamQuote`, and `EStreamMarketData` schemas for financial trading.

**Typical contents:**
```
data-trading/
├── estream-component.toml
├── README.md
└── schemas/
    ├── trading-order.data.yaml
    ├── trading-fill.data.yaml
    └── trading-quote.data.yaml
```

**Manifest excerpt:**
```toml
[component]
category = "data-schema"

[component.schemas]
provides = ["EStreamOrder", "EStreamFill", "EStreamQuote", "EStreamMarketData"]
requires = []

[component.include]
schemas = ["schemas/*.data.yaml"]
```

---

### `wire-adapter` — Protocol Adapters

Adapters that bridge external protocols (MQTT, FIX, HL7, Modbus, SWIFT) to eStream's native data format via the `WireAdapter` trait.

**Example:** `estream-wire-fix` translates FIX 4.2/4.4/5.0 messages to eStream Data schemas.

**Typical contents:**
```
estream-wire-fix/
├── estream-component.toml
├── README.md
├── schemas/
│   ├── fix-order.data.yaml
│   └── fix-execution-report.data.yaml
├── circuits/
│   └── fix_adapter.fl
└── tests/
    └── golden/
        ├── new-order-single.input.json
        └── new-order-single.expected.json
```

**Manifest excerpt:**
```toml
[component]
category = "wire-adapter"

[component.schemas]
provides = ["FixNewOrderSingle", "FixExecutionReport"]
requires = ["EStreamOrder", "EStreamFill"]

[component.include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
```

---

### `smart-circuit` — SmartCircuit Logic

FastLang circuit definitions that execute on the eStream runtime (CPU/WASM target). SmartCircuits are the proactive real-time backend — active processing units that drive queue compaction, map replication, protocol translation, and custom business logic.

**Example:** `order-matcher` implements price-time priority order matching.

**Typical contents:**
```
order-matcher/
├── estream-component.toml
├── README.md
├── circuits/
│   ├── order_matcher.fl
│   └── price_priority.fl
├── schemas/
│   └── matcher-events.data.yaml
└── tests/
    └── golden/
        ├── match-buy-sell.input.json
        └── match-buy-sell.expected.json
```

**Manifest excerpt:**
```toml
[component]
category = "smart-circuit"

[component.schemas]
provides = ["MatchEvent"]
requires = ["EStreamOrder"]

[component.circuits]
provides = ["order_matcher", "price_priority"]
target = ["cpu"]

[component.include]
circuits = ["circuits/*.fl"]
schemas = ["schemas/*.data.yaml"]
tests = ["tests/golden/**"]
```

---

### `fpga-circuit` — FPGA-Targetable Circuits

Circuits that compile through FLIR to synthesizable Verilog for hardware acceleration. These provide orders-of-magnitude speedups for latency-critical operations.

**Example:** `ntt-accelerator` provides an NTT (Number Theoretic Transform) accelerator for post-quantum cryptographic operations.

**Typical contents:**
```
ntt-accelerator/
├── estream-component.toml
├── README.md
├── circuits/
│   └── ntt_accelerator.fl
├── fpga/
│   ├── ntt_pipeline.v
│   └── constraints.xdc
└── tests/
    └── golden/
        └── ntt-transform.expected.json
```

**Manifest excerpt:**
```toml
[component]
category = "fpga-circuit"

[component.circuits]
provides = ["ntt_accelerator"]
target = ["cpu", "fpga"]

[component.include]
circuits = ["circuits/*.fl"]
fpga = ["fpga/*.v", "fpga/*.xdc"]
tests = ["tests/golden/**"]
```

---

### `integration` — Full-Stack Integration Bundles

Bundles that combine schemas, circuits, and adapters into a complete end-to-end solution for a specific domain.

**Example:** A carbon credit integration combining `data-carbon` schemas, `carbon-credit-mint` circuit, and attestation verification logic.

**Manifest excerpt:**
```toml
[component]
category = "integration"

[component.dependencies]
data-carbon = ">=0.8.0"
carbon-credit-mint = ">=0.3.0"

[component.schemas]
provides = ["IntegrationResult"]
requires = ["CarbonCredit", "CarbonAttestation"]
```

---

### `console-widget` — Dashboard Widgets

Pre-built widgets for the eStream operator console. Widgets are WASM-compiled bundles that render inside the console TUI or web dashboard.

**Example:** `impact-counter` provides real-time impact metrics visualization.

**Typical contents:**
```
impact-counter/
├── estream-component.toml
├── README.md
├── widgets/
│   └── dist/
│       ├── impact-counter.wasm
│       └── impact-counter.js
└── tests/
    └── golden/
        └── widget-render.expected.json
```

**Manifest excerpt:**
```toml
[component]
category = "console-widget"

[component.include]
widgets = ["widgets/dist/*"]
tests = ["tests/golden/**"]
```

---

## Writing the `estream-component.toml` Manifest

The manifest is a TOML file with these sections:

### `[component]` — Identity (Required)

```toml
[component]
name = "my-component"          # Unique name (lowercase, hyphens allowed)
version = "1.0.0"              # Semantic version
category = "smart-circuit"     # One of the 6 categories
description = "Short summary"  # Human-readable description
license = "Apache-2.0"         # SPDX license identifier
repository = "https://github.com/org/repo"
homepage = "https://example.com"
readme = "README.md"
keywords = ["trading", "fix"]  # Discoverable tags
```

**Required fields:** `name`, `version`, `category`, `description`

### `[component.author]` — Author Information

```toml
[component.author]
name = "Your Name"
email = "you@example.com"
url = "https://example.com"
```

### `[component.marketplace]` — Pricing and Visibility

```toml
[component.marketplace]
pricing = "free"               # free | one-time | subscription | usage-based | enterprise | freemium
visibility = "open"            # open | interface | compiled | licensed
```

See the [Pricing Guide](./pricing-guide.md) for details on each model.

### `[component.estream]` — Platform Compatibility

```toml
[component.estream]
min_version = "0.8.0"          # Minimum eStream version required
max_version = "1.0.0"          # Maximum eStream version (optional)
```

### `[component.dependencies]` — Other Components

```toml
[component.dependencies]
data-trading = ">=1.0.0"
estream-wire-fix = ">=1.0.0"
```

### `[component.schemas]` — Data Contracts

```toml
[component.schemas]
provides = ["MySchema", "MyEvent"]    # Schemas this component defines
requires = ["EStreamOrder"]           # Schemas this component depends on
```

- **`provides`**: Schema names defined by your `.data.yaml` files
- **`requires`**: Schema names from other components that yours depends on

During installation, the marketplace resolves `requires` by checking that all needed schemas are either installed or co-installed with the component.

### `[component.circuits]` — Circuit Declarations

```toml
[component.circuits]
provides = ["my_circuit", "helper_circuit"]
target = ["cpu"]               # cpu | fpga | cpu, fpga
```

### `[component.include]` — File Globs

```toml
[component.include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
fpga = ["fpga/*.v", "fpga/*.xdc"]     # fpga-circuit only
widgets = ["widgets/dist/*"]           # console-widget only
```

All paths are relative to the component root. Glob patterns follow standard POSIX glob syntax.

---

## Schema Provides/Requires — Data Contracts

The `provides`/`requires` system ensures type-safe composition:

```
┌─────────────────────┐     ┌──────────────────────┐
│  data-trading       │     │  estream-wire-fix     │
│                     │     │                       │
│  provides:          │────▶│  requires:            │
│  - EStreamOrder     │     │  - EStreamOrder       │
│  - EStreamFill      │     │  - EStreamFill        │
│  - EStreamQuote     │     │                       │
│  - EStreamMarketData│     │  provides:            │
│                     │     │  - FixNewOrderSingle   │
└─────────────────────┘     │  - FixExecutionReport  │
                            └──────────────────────┘
```

When you `estream marketplace install estream-wire-fix`, the resolver checks that `EStreamOrder` and `EStreamFill` are available — either already installed or declared as a co-dependency.

---

## Testing Your Component Before Publishing

### 1. Validate the Manifest

```bash
estream marketplace publish my-component --dry-run
```

This checks:
- All required fields are present
- Category is one of the 6 valid values
- Pricing and visibility are valid values
- Include globs resolve to actual files
- FastLang files pass syntax checking

### 2. Run Golden Tests

Place test vectors in `tests/golden/`:

```
tests/golden/
├── test-case-1.input.json
├── test-case-1.expected.json
├── test-case-2.input.json
└── test-case-2.expected.json
```

### 3. Verify Locally

After publishing locally, verify the signature:

```bash
estream marketplace verify my-component
```

### 4. Check with `info`

```bash
estream marketplace info my-component
```

This displays the full manifest details, file listing, and dependency information.

---

## Next Steps

- [Getting Started](./getting-started.md) — Quick install and publish walkthrough
- [CLI Reference](./cli-reference.md) — Full command documentation
- [Pricing Guide](./pricing-guide.md) — Pricing models and visibility levels
- [Security Model](./security-model.md) — ML-DSA-87 signing and verification
