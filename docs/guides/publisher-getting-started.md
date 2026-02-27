# Publisher Getting Started Guide

> End-to-end walkthrough for publishing your first domain package to the eStream Marketplace.

**Audience:** Component publishers (third-party and first-party)
**Prerequisites:** eStream CLI v0.9.1+, PRIME identity, FastLang familiarity

---

## 1. Prerequisites

### 1.1 Install the eStream CLI

```bash
curl -fsSL https://install.estream.io | sh
estream --version
# estream 0.9.1
```

Verify marketplace subcommand availability:

```bash
estream marketplace --help
```

### 1.2 Authenticate with PRIME Identity

Every publisher needs a PRIME identity — the post-quantum biometric identity system that binds your ML-DSA-87 signing key to your person.

```bash
estream auth login
# Opens Spark biometric authentication
# Generates ML-DSA-87 keypair if first time
```

Your signing key is stored at `$HOME/.estream/keys/signing-key.pem`. The public key is registered with the marketplace under your publisher profile.

### 1.3 Register as a Publisher

```bash
estream marketplace publisher register \
  --name "my-org" \
  --display-name "My Organization" \
  --url "https://github.com/my-org"
```

This creates a `publishers/my-org.json` entry in the registry. First-time publishers are verified through the governance circuit `estream.marketplace.publisher.register.v1`.

### 1.4 Development Environment

Ensure your workspace has:

| Tool | Version | Purpose |
|------|---------|---------|
| eStream CLI | >= 0.9.1 | Compilation, testing, publishing |
| FastLang | Bundled with CLI | Circuit authoring |
| Rust toolchain | >= 1.75 | Runtime crates (if hybrid) |

---

## 2. Creating Your First Domain Package

### 2.1 Scaffold a New Package

Use the CLI to generate the project structure:

```bash
estream marketplace scaffold smart-circuit @my-org/order-validator
```

This creates:

```
order-validator/
├── estream-component.toml        # Package manifest
├── README.md                     # Documentation (required)
├── CHANGELOG.md                  # Version history
├── circuits/
│   └── order-validator.fl        # Your FastLang circuit
├── tests/
│   └── golden/
│       └── manifest.toml         # Test vector manifest
└── schemas/
```

### 2.2 Write Your FastLang Circuit

Create your domain logic in `circuits/order-validator.fl`:

```fastlang
/// govern lex esn/marketplace/my-org/order-validator

type Order = struct {
    order_id: bytes(16),
    symbol: bytes(8),
    side: u8,
    quantity: u64,
    price: u64,
    timestamp: u64,
}

type ValidationResult = struct {
    order_id: bytes(16),
    is_valid: bool,
    rejection_reason: bytes(64),
    validated_at: u64,
}

/// test golden
circuit validate_order(order: Order) -> ValidationResult
    lex esn/marketplace/my-org/order-validator
    precision B
    observe metrics: [orders_validated, validation_failures, symbol]
{
    let valid_qty = order.quantity > 0
    let valid_price = order.price > 0
    let is_valid = valid_qty
    ValidationResult {
        order_id: order.order_id,
        is_valid: is_valid,
        rejection_reason: bytes(64),
        validated_at: current_time(),
    }
}
```

Key annotations:

| Annotation | Purpose |
|------------|---------|
| `/// govern lex` | Declares the lex governance scope |
| `/// test golden` | Marks the circuit for golden test vector validation |
| `precision B` | Sets witness tier (A=highest, C=lowest) |
| `observe metrics` | Declares StreamSight telemetry fields |
| `povc true` | Enables Proof of Verifiable Computation attestation |

---

## 3. Writing the Manifest

The `estream-component.toml` manifest describes your package to the marketplace. See the full schema in [MANIFEST_SCHEMA_SPEC.md](../../specs/standards/MANIFEST_SCHEMA_SPEC.md).

### 3.1 Minimal Manifest

```toml
[component]
name = "@my-org/order-validator"
version = "1.0.0"
category = "smart-circuit"
implementation_type = "FastLang"
status = "Draft"
description = "Order validation circuit with field-level checks"
license = "Apache-2.0"
repository = "https://github.com/my-org/order-validator"
readme = "README.md"
keywords = ["trading", "validation", "orders"]

[component.author]
name = "My Organization"
email = "eng@my-org.com"

[component.marketplace]
pricing = "usage-based"
visibility = "compiled"

[component.estream]
min_version = "0.9.1"

[component.dependencies]
data-trading = "^1.0.0"

[component.schemas]
provides = ["Order", "ValidationResult"]
requires = ["EStreamOrder"]

[component.circuits]
provides = ["validate_order"]
target = ["cpu"]

[component.lifecycle]
status = "active"
breaking_change_notice_days = 90
changelog = "CHANGELOG.md"

[component.include]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
schemas = ["schemas/*.data.yaml"]
```

### 3.2 Required Fields

| Field | Description |
|-------|-------------|
| `name` | Unique identifier. Third-party: `@publisher/name`. Official: `estream-*` or `data-*` |
| `version` | Semantic version (`major.minor.patch`) |
| `category` | One of: `data-schema`, `wire-adapter`, `smart-circuit`, `fpga-circuit`, `integration`, `console-widget` |
| `description` | One-line summary |
| `license` | SPDX license identifier |

### 3.3 Naming Conventions

- **Format:** lowercase alphanumeric with hyphens (`[a-z0-9-]+`)
- **Official prefixes:** `estream-*` and `data-*` are reserved for the eStream team
- **Third-party:** `@publisher/name` (e.g., `@synergy-carbon/impact-counter`)

---

## 4. Local Development and Testing

### 4.1 Development Server

Run the local marketplace dev server for rapid iteration:

```bash
estream marketplace dev
```

This starts a local registry mirror, watches your `.fl` files for changes, and recompiles on save. The dev server exposes the same lattice streams as production, so you can test with the browser SDK or CLI.

### 4.2 Running Tests

```bash
estream marketplace test
```

This runs:

1. **Manifest validation** — checks all required fields and schema correctness
2. **Circuit compilation** — compiles `.fl` files to ESCIR
3. **Golden test vectors** — runs all `/// test golden` circuits against `tests/golden/` vectors
4. **Dependency resolution** — verifies all declared dependencies are available

### 4.3 Writing Golden Test Vectors

Create test vectors in `tests/golden/`:

```json
{
  "name": "valid-buy-order",
  "description": "Standard buy order passes validation",
  "circuit": "validate_order",
  "input": {
    "order": {
      "order_id": "0123456789abcdef",
      "symbol": "AAPL",
      "side": 0,
      "quantity": 1000,
      "price": 15025,
      "timestamp": 1740000000
    }
  },
  "expected_output": {
    "is_valid": true
  },
  "expected_witness_tier": 2
}
```

Register vectors in `tests/golden/manifest.toml`:

```toml
[test_suite]
name = "order-validator golden tests"
circuit = "validate_order"
version = "1.0.0"

[[vectors]]
file = "valid-buy-order.json"
tags = ["happy-path", "buy"]

[[vectors]]
file = "reject-zero-qty.json"
tags = ["rejection", "edge-case"]
```

### 4.4 Linting and Validation

```bash
estream marketplace lint
# Checks: manifest schema, naming conventions, license file, README existence
```

---

## 5. Compilation and Attestation

### 5.1 Compile to .escx Package

The `.escx` format is the opaque compiled package format. See [ESCX_FORMAT_SPEC.md](../../specs/standards/ESCX_FORMAT_SPEC.md) for the binary layout.

```bash
estream domain compile
```

This produces:

1. **ESCIR bytecode** — position-independent, symbol-stripped, not reverse-engineerable
2. **LSP metadata** — for IDE integration (autocompletion, hover docs)
3. **PoVC attestation** — proof that the compiler faithfully compiled your source

```
Compiling @my-org/order-validator v1.0.0...
  ✓ FastLang parse                      12ms
  ✓ ESCIR codegen                       45ms
  ✓ Symbol stripping                     3ms
  ✓ PoVC witness generation             89ms
  ✓ Package attestation (ML-DSA-87)     15ms

  Output: target/order-validator-1.0.0.escx (24 KB)
```

### 5.2 Attestation

Every compiled package includes a PoVC (Proof of Verifiable Computation) witness that proves:

- The package was compiled by an authentic eStream compiler
- The source hash matches the compiled artifact hash
- The compiler version and ESCIR API version are recorded

The attestation is signed with ML-DSA-87 by the compiler's PRIME identity. See `registry/package_format.fl` for the `PackageAttestation` type.

### 5.3 Signing

The package is signed with your publisher ML-DSA-87 private key:

```bash
estream domain compile --key $HOME/.estream/keys/signing-key.pem
```

The signing process builds a Merkle tree of SHA3-256 file hashes and signs the root. See [ESTREAM_MARKETPLACE_SPEC.md §7](../../specs/ESTREAM_MARKETPLACE_SPEC.md) for the full signing protocol.

---

## 6. Publishing to the Marketplace

### 6.1 Pre-Publish Checklist

```bash
estream marketplace publish --dry-run
```

The dry-run validates:

| Check | Description |
|-------|-------------|
| Manifest completeness | All required fields present |
| Name format | Matches `@publisher/name` or `[a-z0-9-]+` |
| Version increment | Higher than any published version |
| Schema validation | All `provides` schemas exist in package |
| Dependency resolution | All `requires` available in registry |
| Circuit validation | All `.fl` files parse and compile |
| Test vectors | All golden tests pass |
| File size | Archive < 50 MB |

### 6.2 Publish

```bash
estream marketplace publish
```

```
Publishing @my-org/order-validator v1.0.0...

  Phase 1: Validating manifest...          ✓
  Phase 2: Running test vectors...         ✓ (4/4 passed)
  Phase 3: Building deterministic archive...✓
  Phase 4: Computing Merkle root...        ✓
  Phase 5: Signing with ML-DSA-87...       ✓
  Phase 6: Submitting to registry...       ✓

  PR created: https://github.com/estream-io/registry/pull/42
  Status: Awaiting CI verification
```

### 6.3 CI Verification

After submission, the registry CI pipeline:

1. Downloads and extracts your package archive
2. Verifies SHA3-256 checksum
3. Verifies ML-DSA-87 signature against your registered publisher key
4. Validates manifest schema
5. Checks version increment
6. Runs golden test vectors
7. Posts results as a PR comment

**First-time publishers** require manual review. **Established publishers** with a track record get auto-merged.

### 6.4 Post-Publish

After merge, your component appears on the `/marketplace/index` lattice stream and is discoverable via:

```bash
estream marketplace search "order-validator"
```

---

## 7. Version Management and Deprecation

### 7.1 Semantic Versioning

All packages follow strict semver:

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Breaking API change | Major | 1.0.0 → 2.0.0 |
| New feature (backward-compatible) | Minor | 1.0.0 → 1.1.0 |
| Bug fix | Patch | 1.0.0 → 1.0.1 |

### 7.2 Publishing Updates

Update the version in `estream-component.toml` and re-publish:

```bash
# Update version in manifest
estream marketplace publish
```

The registry enforces monotonic version increments — you cannot publish a version lower than or equal to the latest.

### 7.3 Deprecating a Version

When a new major version is ready:

```toml
[component.lifecycle]
status = "deprecated"
sunset_date = "2027-06-01"
successor = "@my-org/order-validator-v2"
migration_guide = "MIGRATION.md"
breaking_change_notice_days = 90
```

The deprecation protocol:

1. Set `status = "deprecated"` with `sunset_date` and `successor`
2. Marketplace emits StreamSight warnings to all active consumers at `lex://esn/marketplace/deprecation`
3. Provide a `MIGRATION.md` documenting breaking changes
4. During deprecation: package is still installable and receives security patches
5. At sunset date: new installations blocked, existing deployments unaffected

### 7.4 Yanking a Version

To immediately remove a version due to a critical issue:

```bash
estream marketplace yank @my-org/order-validator@1.0.0
```

Yanked versions are excluded from dependency resolution but remain available for projects that already depend on them.

### 7.5 Shadow Testing

When publishing a major version upgrade, consumers can run old and new versions side-by-side:

```bash
estream marketplace upgrade --shadow @my-org/order-validator@^2.0.0
```

This uses the `shadow_test` lifecycle circuit to validate the new version against existing production traffic before switching over.

---

## Next Steps

- [Pricing Strategy Guide](pricing-strategy.md) — Configure share/private tiers and revenue models
- [Solution Builder Guide](solution-builder.md) — Bundle packages into complete solutions
- [ESCX Format Specification](../../specs/standards/ESCX_FORMAT_SPEC.md) — Binary package format details
- [Manifest Schema Specification](../../specs/standards/MANIFEST_SCHEMA_SPEC.md) — Full manifest field reference
- [Privacy Guarantees](../../specs/standards/PRIVACY_GUARANTEES_SPEC.md) — What the marketplace cannot learn about your customers
