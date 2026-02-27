# Manifest Schema Specification (`manifest.toml`)

> Complete field-by-field schema for the `.escx` package manifest.

**Status:** Draft
**Version:** 1.0.0
**Epic:** estream-marketplace#8 (Documentation & Standards)
**Source:** `registry/package_format.fl`, `templates/estream-component.template.toml`, `specs/ESTREAM_MARKETPLACE_SPEC.md §3`

---

## 1. Overview

Every `.escx` package contains a `manifest.toml` that describes the package to the marketplace registry, CLI, and runtime. This document specifies every field, its type, constraints, and behavior.

The manifest is embedded as the first section (type `0x0001`) in the `.escx` binary format (see [ESCX_FORMAT_SPEC.md](ESCX_FORMAT_SPEC.md)). It is also used standalone as `estream-component.toml` during development.

### 1.1 Notation

| Symbol | Meaning |
|--------|---------|
| **Required** | Field must be present; publish will fail without it |
| **Optional** | Field may be omitted; default value applies |
| **Conditional** | Required when a specific condition is met |

---

## 2. `[package]` Section

Core package identity and metadata.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | **Required** | — | Unique package identifier |
| `version` | string | **Required** | — | Semantic version (`major.minor.patch`) |
| `description` | string | **Required** | — | One-line summary (max 512 bytes) |
| `license` | string | **Required** | — | SPDX license identifier (e.g., `"Apache-2.0"`) |
| `repository` | string | Optional | `""` | Source repository URL |
| `homepage` | string | Optional | `""` | Project homepage URL |
| `readme` | string | Optional | `"README.md"` | Path to README file |
| `keywords` | list\<string\> | Optional | `[]` | Discovery tags (max 10) |
| `category` | string | **Required** | — | Component category (see §2.1) |
| `implementation_type` | string | Optional | `"FastLang"` | Implementation type (see §2.2) |
| `status` | string | Optional | `"Draft"` | Development status (see §2.3) |

### 2.1 Category Values

| Value | Description |
|-------|-------------|
| `data-schema` | Data schema packs |
| `wire-adapter` | Protocol adapters (implements `WireAdapter` trait) |
| `smart-circuit` | Reusable SmartCircuit packages |
| `fpga-circuit` | FPGA bitstream components |
| `integration` | Full-stack integrations |
| `console-widget` | Console dashboard widgets |

### 2.2 Implementation Type Values

| Value | Description |
|-------|-------------|
| `FastLang` | Pure FastLang (`.fl`) |
| `Hybrid` | FastLang + Rust/RTL |
| `Pure Rust` | Rust-only implementation |
| `Pure RTL` | Verilog/VHDL only |
| `Platform` | Tooling and infrastructure |

### 2.3 Status Values

| Value | Description |
|-------|-------------|
| `Draft` | Under development |
| `Active` | Production-ready |
| `Deprecated` | Scheduled for removal (see `[lifecycle]`) |
| `Sunset` | Removed from active registry |

### 2.4 Name Conventions

- **Format:** lowercase alphanumeric with hyphens: `[a-z0-9-]+`
- **Official prefixes:** `estream-*` and `data-*` (reserved for eStream team)
- **Third-party:** `@publisher/name` (e.g., `@synergy-carbon/impact-counter`)
- **Max length:** 128 bytes

### 2.5 Version Format

Strict semantic versioning: `MAJOR.MINOR.PATCH`

| Syntax | Meaning (when used as dependency requirement) |
|--------|---------|
| `^1.2.3` | Compatible: `>=1.2.3, <2.0.0` (default) |
| `~1.2.3` | Patch-level: `>=1.2.3, <1.3.0` |
| `>=1.0.0` | Minimum version |
| `=1.2.3` | Exact version |
| `*` | Any version |

### 2.6 Example

```toml
[package]
name = "@my-org/order-validator"
version = "1.0.0"
description = "Order validation circuit with field-level checks"
license = "Apache-2.0"
repository = "https://github.com/my-org/order-validator"
homepage = "https://my-org.io/order-validator"
readme = "README.md"
keywords = ["trading", "validation", "orders"]
category = "smart-circuit"
implementation_type = "FastLang"
status = "Active"
```

---

## 3. `[publisher]` Section

Publisher identity bound to PRIME.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `id` | string | **Required** | — | Publisher PRIME identity (32-byte hex) |
| `name` | string | **Required** | — | Publisher display name (max 64 bytes) |
| `signing_key_id` | string | **Required** | — | Active ML-DSA-87 signing key identifier |
| `email` | string | Optional | `""` | Contact email |
| `url` | string | Optional | `""` | Publisher URL |

### 3.1 Signing Key Reference

The `signing_key_id` must reference an active key registered in the marketplace at `publishers/{name}.json`. The key record includes:

| Field | Description |
|-------|-------------|
| `key_id` | Unique key identifier |
| `algorithm` | Must be `ML-DSA-87` |
| `public_key_hex` | Hex-encoded 1,952-byte public key |
| `created_at` | Key creation timestamp |
| `expires_at` | Key expiration timestamp |
| `status` | `active`, `rotated`, or `revoked` |

### 3.2 Example

```toml
[publisher]
id = "a1b2c3d4e5f6..."
name = "My Organization"
signing_key_id = "my-org-signing-key-01"
email = "eng@my-org.com"
url = "https://github.com/my-org"
```

---

## 4. `[escir]` Section

ESCIR API and platform version compatibility.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `api_version` | string | **Required** | — | ESCIR API version this package targets (e.g., `"0.9.1"`) |
| `min_platform_version` | string | **Required** | — | Minimum eStream platform version required |
| `max_platform_version` | string | Optional | `"*"` | Maximum eStream platform version supported |

### 4.1 Compatibility Rules

The runtime checks at install time:

```
min_platform_version <= current_platform_version <= max_platform_version
```

If `max_platform_version` is `"*"` (default), no upper bound is enforced.

### 4.2 Stability Tiers

From `registry/package_format.fl`, `PackageStability`:

| Value | Tier | Description |
|-------|------|-------------|
| 0 | Stable | ESCIR API is frozen; no breaking changes within the major version |
| 1 | Preview | API may change in minor versions |
| 2 | Experimental | API may change at any time |

### 4.3 Example

```toml
[escir]
api_version = "0.9.1"
min_platform_version = "0.9.1"
max_platform_version = "1.0.0"
```

---

## 5. `[lex]` Section

Lex governance scope requirements.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `requirements` | list\<string\> | Optional | `[]` | Lex paths this package needs access to |

### 5.1 Lex Path Format

Lex paths follow the `esn/` hierarchical namespace:

```
esn/marketplace/licensing
esn/marketplace/registry
esn/marketplace/solutions
esn/marketplace/my-org/order-validator
```

### 5.2 Runtime Enforcement

At install time, the runtime verifies that the deployment environment provides all required lex paths. Missing lex paths cause installation to fail with error `E008 ManifestInvalid`.

### 5.3 Example

```toml
[lex]
requirements = [
    "esn/marketplace/licensing",
    "esn/marketplace/registry",
]
```

---

## 6. `[dependencies]` Section

Dependencies on other marketplace packages.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `{package_name}` | string | — | — | Version requirement for the named package |

### 6.1 Dependency Format

Each entry is a key-value pair: package name = version requirement.

```toml
[dependencies]
data-trading = "^1.0.0"
estream-wire-fix = ">=0.9.1"
"@partner/utils" = "~2.1.0"
```

### 6.2 Resolution

Dependencies are resolved using the `component_dependencies` DAG:

1. Fetch metadata for each declared dependency
2. Filter: remove yanked versions, check platform compatibility
3. Select highest matching version (semver sort)
4. `enforce acyclic` — hardware rejects circular dependencies
5. `topo_sort` — produces topological installation order
6. `version_conflict` overlay flags conflicting nodes

### 6.3 Optional Dependencies

Optional dependencies are declared with a `[dependencies.{name}]` sub-table:

```toml
[dependencies."@partner/optional-addon"]
version = "^1.0.0"
optional = true
```

### 6.4 Example

```toml
[dependencies]
data-trading = "^1.0.0"
"@my-org/core-lib" = ">=2.0.0"
```

---

## 7. `[pricing]` Section

License type and pricing for Share and Private tiers.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `license_type` | string | **Required** | — | Pricing model (see §7.1) |
| `share_price` | string | Conditional | — | Price for Share tier (required if not `free`) |
| `private_price` | string | Conditional | — | Price for Private tier (required if not `free`) |
| `free_features` | list\<string\> | Conditional | — | Free-tier feature list (required if `freemium`) |
| `premium_features` | list\<string\> | Conditional | — | Premium-tier feature list (required if `freemium`) |

### 7.1 License Type Values

| Value | Billing Unit | Description |
|-------|-------------|-------------|
| `free` | — | No charge |
| `per-seat` | User/month | Fixed per authorized user |
| `per-invocation` | Execution | Pay per circuit execution |
| `per-minute` | Compute minute | Pay for compute time |
| `metered-hourly` | Active hour | Hourly aggregated billing |
| `flat-rate` | Month | Fixed monthly/annual subscription |
| `freemium` | Feature tier | Free base + paid premium |

### 7.2 Price Format

Prices are decimal strings in US dollars:

```toml
share_price = "0.0004"    # $0.0004 per unit
private_price = "0.0006"  # $0.0006 per unit
```

The Private tier price must be >= the Share tier price.

### 7.3 Revenue Share

Revenue splits are applied automatically by the settlement circuit. See [Pricing Strategy Guide](../../docs/guides/pricing-strategy.md) §2 for split tables.

### 7.4 Examples

```toml
# Per-invocation
[pricing]
license_type = "per-invocation"
share_price = "0.0004"
private_price = "0.0006"

# Flat-rate subscription
[pricing]
license_type = "flat-rate"
share_price = "49.99"
private_price = "74.99"

# Free
[pricing]
license_type = "free"

# Freemium
[pricing]
license_type = "freemium"
share_price = "19.99"
private_price = "29.99"
free_features = ["basic-validation"]
premium_features = ["advanced-routing", "fpga-acceleration"]
```

---

## 8. `[telemetry]` Section

Defines the telemetry schema for Share tier customers. Required if `license_type` is not `free`.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `schema_hash` | string | **Required** | — | SHA3-256 of telemetry schema; `"auto"` for auto-compute |
| `metrics` | list\<string\> | **Required** | — | Named metrics collected from Share tier |
| `aggregate_window` | string | **Required** | — | Minimum aggregation period (duration, >= `"1h"`) |
| `cohort_min_size` | integer | **Required** | — | Minimum k for k-anonymity (>= 50) |
| `noise_budget` | string | **Required** | — | Differential privacy epsilon (decimal string) |

### 8.1 Metrics

Each metric is a named string that maps to an `observe metrics` field in the FastLang circuit:

```toml
metrics = [
    "invocation_count",
    "latency_p50_ms",
    "latency_p99_ms",
    "error_rate",
]
```

### 8.2 Privacy Constraints

Enforced by `validate_telemetry_schema` circuit:

| Constraint | Rule |
|------------|------|
| No PII | `contains_pii = false` — reject schemas with personally identifiable information |
| Anonymization required | `anonymization_method > 0` (k-Anonymity, Differential Privacy, or Aggregation) |
| Minimum cohort | `cohort_min_size >= 50` |
| Minimum aggregation | `aggregate_window >= "1h"` |
| Noise budget | `noise_budget` must be a positive decimal; lower = more private |

### 8.3 Example

```toml
[telemetry]
schema_hash = "auto"
metrics = [
    "invocation_count",
    "latency_p50_ms",
    "latency_p99_ms",
    "error_rate",
    "input_size_bytes",
]
aggregate_window = "1h"
cohort_min_size = 50
noise_budget = "1.0"
```

---

## 9. `[solution]` Section (Optional)

Present only for packages that are part of a solution bundle.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `tier` | string | Conditional | — | Solution tier: `"platform"`, `"solution"`, or `"customer"` |
| `branding` | table | Optional | — | Solution branding (see §9.1) |
| `lex_policy` | string | Optional | — | Path to lex policy file |
| `upgrade_policy` | table | Optional | — | Auto-upgrade rules (see §9.2) |

### 9.1 Branding Sub-Table

```toml
[solution.branding]
display_name = "Synergy Carbon Edge"
theme_hash = "auto"
logo_hash = "auto"
documentation_url = "https://docs.synergy-carbon.io/edge"
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | string | **Required** | Human-readable solution name |
| `theme_hash` | string | Optional | SHA3-256 of theme JSON; `"auto"` for auto-compute |
| `logo_hash` | string | Optional | SHA3-256 of logo asset; `"auto"` for auto-compute |
| `documentation_url` | string | Optional | Link to solution documentation |

### 9.2 Upgrade Policy Sub-Table

```toml
[solution.upgrade_policy]
auto_accept_patch = true
auto_accept_minor = true
auto_accept_major = false
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `auto_accept_patch` | bool | `true` | Auto-accept patch version upgrades |
| `auto_accept_minor` | bool | `true` | Auto-accept minor version upgrades |
| `auto_accept_major` | bool | `false` | Auto-accept major version upgrades |

### 9.3 Example

```toml
[solution]
tier = "solution"
lex_policy = "lex-policies/solution-rbac.lex"

[solution.branding]
display_name = "Synergy Carbon Edge"
theme_hash = "auto"
logo_hash = "auto"
documentation_url = "https://docs.synergy-carbon.io/edge"

[solution.upgrade_policy]
auto_accept_patch = true
auto_accept_minor = true
auto_accept_major = false
```

---

## 10. Category-Specific Sections

These sections are conditional — include only the one matching your `category`.

### 10.1 `[wire_adapter]` (category = `wire-adapter`)

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `protocol_family` | string | **Required** | — | `"financial"`, `"messaging"`, `"healthcare"`, `"industrial"`, `"general"` |
| `transports` | list\<string\> | **Required** | — | `"tcp"`, `"udp"`, `"serial"`, `"websocket"`, `"tls"`, `"quic"` |
| `bidirectional` | bool | Optional | `false` | Supports bidirectional communication |
| `request_response` | bool | Optional | `false` | Supports request-response pattern |

### 10.2 `[widget]` (category = `console-widget`)

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `widget_category` | string | **Required** | — | `"analytics"`, `"monitoring"`, `"control"`, `"visualization"` |
| `roles` | list\<string\> | **Required** | — | Required RBAC roles |
| `sizes` | list\<string\> | **Required** | — | `"small"`, `"medium"`, `"large"` |
| `data_sources` | list\<string\> | **Required** | — | `"eslite"`, `"api"`, `"lex-stream"` |

### 10.3 `[fpga]` (category = `fpga-circuit`)

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `device_family` | list\<string\> | **Required** | — | Target FPGA families (e.g., `["xcvu9p", "xcvu13p"]`) |
| `resource_estimate` | table | **Required** | — | `{ luts, brams, dsps }` resource usage |

### 10.4 Examples

```toml
# Wire adapter
[wire_adapter]
protocol_family = "financial"
transports = ["tcp", "tls"]
bidirectional = true
request_response = true

# Console widget
[widget]
widget_category = "analytics"
roles = ["operator", "admin"]
sizes = ["small", "medium", "large"]
data_sources = ["eslite"]

# FPGA circuit
[fpga]
device_family = ["xcvu9p", "xcvu13p"]
resource_estimate = { luts = 50000, brams = 100, dsps = 0 }
```

---

## 11. `[lifecycle]` Section

Component lifecycle management (v0.9.1).

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `status` | string | **Required** | `"active"` | `"active"`, `"deprecated"`, `"sunset"` |
| `sunset_date` | string | Conditional | — | ISO 8601 date (required when `status = "deprecated"`) |
| `successor` | string | Conditional | — | Replacement package (required when `status = "deprecated"`) |
| `migration_guide` | string | Optional | — | Path to migration guide |
| `breaking_change_notice_days` | integer | Optional | `90` | Minimum notice before breaking changes |
| `changelog` | string | Optional | `"CHANGELOG.md"` | Path to changelog |

### 11.1 Deprecation Rules

When `status = "deprecated"`:
- `sunset_date` must be set (ISO 8601 format)
- `successor` must reference an existing package
- StreamSight emits warnings to all active consumers
- Package is still installable during deprecation period
- At `sunset_date`, status transitions to `"sunset"` and new installations are blocked

### 11.2 Example

```toml
[lifecycle]
status = "deprecated"
sunset_date = "2027-06-01"
successor = "@my-org/order-validator-v2"
migration_guide = "MIGRATION.md"
breaking_change_notice_days = 90
changelog = "CHANGELOG.md"
```

---

## 12. `[schemas]` Section

Schema declarations.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `provides` | list\<string\> | Optional | `[]` | Schema types this package exports |
| `requires` | list\<string\> | Optional | `[]` | Schema types this package depends on |

```toml
[schemas]
provides = ["Order", "ValidationResult"]
requires = ["EStreamOrder", "EStreamFill"]
```

---

## 13. `[circuits]` Section

Circuit declarations.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `provides` | list\<string\> | Optional | `[]` | Circuit names this package exports |
| `target` | list\<string\> | Optional | `["cpu"]` | Execution targets: `"cpu"`, `"fpga"`, or both |

```toml
[circuits]
provides = ["validate_order", "route_order"]
target = ["cpu", "fpga"]
```

---

## 14. `[include]` Section

Glob patterns for files included in the published package.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `schemas` | list\<string\> | Optional | `["schemas/*.data.yaml"]` | Schema file patterns |
| `circuits` | list\<string\> | Optional | `["circuits/*.fl"]` | Circuit file patterns |
| `tests` | list\<string\> | Optional | `["tests/golden/**"]` | Test vector patterns |
| `fpga` | list\<string\> | Optional | `[]` | FPGA bitstream patterns |
| `widgets` | list\<string\> | Optional | `[]` | Widget bundle patterns |

```toml
[include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
fpga = ["fpga/*.bit"]
widgets = ["widgets/dist/**"]
```

---

## 15. `[author]` Section

Package author metadata.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | **Required** | — | Author or organization name |
| `email` | string | Optional | `""` | Contact email |
| `url` | string | Optional | `""` | Author URL |

```toml
[author]
name = "My Organization"
email = "eng@my-org.com"
url = "https://github.com/my-org"
```

---

## 16. `[marketplace]` Section

Marketplace publishing settings.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `pricing` | string | Optional | `"free"` | `"free"`, `"one-time"`, `"subscription"`, `"usage-based"`, `"enterprise"`, `"freemium"` |
| `visibility` | string | Optional | `"open"` | `"open"`, `"interface"`, `"compiled"`, `"licensed"` |

### 16.1 Visibility Levels

| Level | What's Visible |
|-------|---------------|
| `open` | Full ESCIR source visible to all |
| `interface` | Only ports, annotations, source hash |
| `compiled` | Interface + compiled artifacts (WASM/Verilog) |
| `licensed` | Full source visible only to licensees |

```toml
[marketplace]
pricing = "usage-based"
visibility = "compiled"
```

---

## 17. Complete Example

```toml
[package]
name = "@my-org/order-validator"
version = "1.0.0"
description = "Order validation circuit with field-level checks"
license = "Apache-2.0"
repository = "https://github.com/my-org/order-validator"
homepage = "https://my-org.io/order-validator"
readme = "README.md"
keywords = ["trading", "validation", "orders"]
category = "smart-circuit"
implementation_type = "FastLang"
status = "Active"

[publisher]
id = "a1b2c3d4e5f6..."
name = "My Organization"
signing_key_id = "my-org-signing-key-01"
email = "eng@my-org.com"
url = "https://github.com/my-org"

[escir]
api_version = "0.9.1"
min_platform_version = "0.9.1"
max_platform_version = "1.0.0"

[lex]
requirements = [
    "esn/marketplace/licensing",
    "esn/marketplace/registry",
]

[dependencies]
data-trading = "^1.0.0"

[pricing]
license_type = "per-invocation"
share_price = "0.0004"
private_price = "0.0006"

[telemetry]
schema_hash = "auto"
metrics = [
    "invocation_count",
    "latency_p50_ms",
    "latency_p99_ms",
    "error_rate",
]
aggregate_window = "1h"
cohort_min_size = 50
noise_budget = "1.0"

[schemas]
provides = ["Order", "ValidationResult"]
requires = ["EStreamOrder"]

[circuits]
provides = ["validate_order"]
target = ["cpu"]

[lifecycle]
status = "active"
breaking_change_notice_days = 90
changelog = "CHANGELOG.md"

[author]
name = "My Organization"
email = "eng@my-org.com"
url = "https://github.com/my-org"

[marketplace]
pricing = "usage-based"
visibility = "compiled"

[include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
```

---

## 18. Validation Rules

The CLI validates the manifest at publish time. Key rules:

| Rule | Error Code | Description |
|------|------------|-------------|
| Name format | E008 | Must match `[a-z0-9-]+` or `@[a-z0-9-]+/[a-z0-9-]+` |
| Name reserved | E009 | `estream-*` and `data-*` reserved for eStream team |
| Version format | E008 | Must be valid semver |
| Version increment | E010 | Must be higher than latest published version |
| Category valid | E008 | Must be one of the six defined categories |
| License valid | E008 | Must be valid SPDX identifier |
| Dependencies resolvable | E001/E002 | All dependencies must exist and satisfy version constraints |
| No cycles | E004 | Dependency graph must be acyclic |
| Schemas exist | E008 | All `provides` schemas must exist in package files |
| Telemetry no PII | E008 | Telemetry schema must not contain PII |
| Telemetry cohort | E008 | `cohort_min_size >= 50` |
| Telemetry window | E008 | `aggregate_window >= "1h"` |
| Price ordering | E008 | `private_price >= share_price` |
| Package size | E011 | Archive must be < 50 MB |

---

## References

- [ESCX_FORMAT_SPEC.md](ESCX_FORMAT_SPEC.md) — Binary package format containing this manifest
- [PRIVACY_GUARANTEES_SPEC.md](PRIVACY_GUARANTEES_SPEC.md) — Privacy properties of telemetry and pricing
- [ESTREAM_MARKETPLACE_SPEC.md §3](../ESTREAM_MARKETPLACE_SPEC.md) — Original manifest schema
- `templates/estream-component.template.toml` — Template for scaffolding new packages
- `registry/package_format.fl` — FastLang types for manifest fields
- [Publisher Getting Started Guide](../../docs/guides/publisher-getting-started.md) — Practical manifest authoring guide

---

*Created: February 2026*
*Status: Draft*
