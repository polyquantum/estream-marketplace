# Customer Guide

> How to discover, install, license, and manage eStream Marketplace packages.

**Audience:** Developers and teams consuming marketplace packages
**Prerequisites:** eStream CLI v0.9.1+, authenticated session

---

## 1. Searching the Marketplace

### 1.1 Text Search

```bash
estream marketplace search "FIX adapter"
```

```
  NAME                  VERSION  CATEGORY       PUBLISHER   BADGES
  estream-wire-fix      1.0.0    wire-adapter   estream     ✓ Official, Tested
  fix-order-router      0.3.0    smart-circuit  acme-fin    Verified

  2 results found
```

### 1.2 Filtered Search

| Flag | Description | Example |
|------|-------------|---------|
| `-c, --category` | Filter by component category | `--category wire-adapter` |
| `-t, --tag` | Filter by keyword tag | `--tag trading` |
| `--verified` | Only verified publishers | `--verified` |
| `--official` | Only official eStream components | `--official` |
| `--limit <N>` | Maximum results (default: 20) | `--limit 50` |
| `-o, --output` | Output format: `table` or `json` | `--output json` |

```bash
estream marketplace search "carbon" --category smart-circuit --verified --output json
```

### 1.3 Browsing Categories

```bash
estream marketplace search "" --category data-schema
estream marketplace search "" --category wire-adapter
estream marketplace search "" --category smart-circuit
estream marketplace search "" --category fpga-circuit
estream marketplace search "" --category integration
estream marketplace search "" --category console-widget
```

### 1.4 Component Details

```bash
estream marketplace info estream-wire-fix
```

```
  estream-wire-fix v1.0.0
  by eStream Contributors (verified)
  ──────────────────────────────────────────
  Category:     wire-adapter
  License:      Apache-2.0
  Pricing:      Free
  Visibility:   Open
  Badges:       Official, Tested, PostQuantum
  Rating:       ★★★★☆ 4.2 (47 reviews)
  Installs:     1,247
  Targets:      CPU, FPGA

  Ports:
    IN:  fix_raw (bytes)         OUT: orders (Order)
    IN:  session_cfg (Config)    OUT: executions (Fill)

  Resources:    T2 witness, 2K compute, 8KB mem
  Est. cost:    0.003 ES/exec

  Dependencies: data-trading ^1.0.0

  Homepage:     https://estream.io/components/wire-fix
  Repository:   https://github.com/toddrooke/estream-wire-fix
```

### 1.5 Visual Circuit Designer

The eStream Console circuit designer includes a Marketplace panel. Toggle from **Palette** to **Marketplace** in the sidebar to browse, search, and install components visually. Installed components appear in the Palette and can be dragged onto the canvas.

---

## 2. Installing Packages

### 2.1 Basic Installation

```bash
estream marketplace install estream-wire-fix
```

This installs the latest non-yanked version and all transitive dependencies:

```
  Resolving dependencies...
    estream-wire-fix v1.0.0
    └── data-trading v1.2.0

  Verifying ML-DSA-87 signatures...
    estream-wire-fix v1.0.0 ✓ (key: estream-signing-key-01)
    data-trading v1.2.0 ✓ (key: estream-signing-key-01)

  Installed 2 components (6 files) in 3.2s
```

### 2.2 Version Pinning

```bash
estream marketplace install estream-wire-fix@^1.0.0   # Compatible range
estream marketplace install estream-wire-fix@~1.2.3   # Patch-level range
estream marketplace install estream-wire-fix@=1.0.0   # Exact version
```

| Syntax | Meaning |
|--------|---------|
| `^1.2.3` | Compatible: `>=1.2.3, <2.0.0` |
| `~1.2.3` | Patch-level: `>=1.2.3, <1.3.0` |
| `>=1.0.0` | Minimum version |
| `=1.2.3` | Exact version |
| `*` | Any version |

### 2.3 Installation Options

```bash
estream marketplace install estream-wire-fix --dry-run    # Preview without installing
estream marketplace install estream-wire-fix --force      # Bypass cache
estream marketplace install estream-wire-fix --save       # Add to estream-workspace.toml
estream marketplace install estream-wire-fix --no-verify  # Skip signature check (NOT recommended)
```

### 2.4 Installation Targets

Components install to category-specific directories:

| Category | Install Path |
|----------|-------------|
| `data-schema` | `schemas/` |
| `wire-adapter` | `adapters/` |
| `smart-circuit` | `circuits/` |
| `fpga-circuit` | `fpga/` |
| `integration` | `integrations/` |
| `console-widget` | `widgets/` |

### 2.5 Workspace Tracking

After installation with `--save`, components are tracked in `estream-workspace.toml`:

```toml
[dependencies]
estream-wire-fix = "1.0.0"
data-trading = "1.2.0"

[dependencies.estream-wire-fix]
version = "1.0.0"
checksum = "a1b2c3d4..."
installed_at = "2026-02-10T12:00:00Z"
```

### 2.6 Offline Installation

For air-gapped environments:

```bash
# Export on a connected machine
estream marketplace export estream-wire-fix --output ./offline-bundle/

# Import on the target machine
estream marketplace install --from ./offline-bundle/
```

---

## 3. Managing Licenses

### 3.1 License Types

| Type | Description | Use Case |
|------|-------------|----------|
| **Perpetual** | One-time purchase, unlimited use | Buy once, use forever |
| **Subscription** | Time-limited access (monthly/annual) | Ongoing access with updates |
| **Usage-Based** | Pay per execution or compute time | Variable workloads |
| **Trial** | Free time-limited access | Evaluation period |

### 3.2 Activation

When you install a paid component, a blinded license token is issued:

```bash
estream marketplace install @acme/premium-adapter
# Activating license...
# License token issued: tok_abc123 (blinded)
# Expires: 2027-02-27
```

Your license is a **blinded token** — the marketplace operator never learns which customer bought which package. See [Privacy Guarantees](../../specs/standards/PRIVACY_GUARANTEES_SPEC.md) for the cryptographic protocol.

### 3.3 Viewing Active Licenses

```bash
estream marketplace licenses
```

```
  PACKAGE                    LICENSE     STATUS   EXPIRES
  @acme/premium-adapter      Subscription Active   2027-02-27
  @acme/analytics-widget     UsageBased   Active   —
  estream-wire-fix           Free         Active   —
```

### 3.4 License Portability

Licenses are bound to your PRIME identity, not to a specific machine. You can use licensed components on any device where you authenticate with Spark:

```bash
# On a new machine:
estream auth login
estream marketplace install @acme/premium-adapter
# License token verified via ZK ownership proof — no re-purchase needed
```

### 3.5 License Renewal

Subscription licenses auto-renew by default. To manage renewal:

```bash
estream marketplace license renew @acme/premium-adapter     # Manual renewal
estream marketplace license cancel @acme/premium-adapter    # Cancel auto-renewal
```

Renewal uses the `renew_blinded_token` circuit — the renewal signature is generated without revealing your identity to the marketplace.

### 3.6 License Ownership Proofs

You can prove license ownership without revealing your identity using a ZK proof:

```bash
estream marketplace license prove @acme/premium-adapter
# ZK ownership proof generated: proof_xyz789
```

This uses the `prove_token_ownership` circuit from `licensing/blinded_tokens.fl`.

---

## 4. Telemetry Preferences

### 4.1 Share vs. Private Tiers

Every paid package offers two pricing tiers:

| Tier | Telemetry | Price | What's Shared |
|------|-----------|-------|---------------|
| **Share** | Opted-in | Lower price | Anonymized, k-anonymous, differentially private aggregates |
| **Private** | Zero telemetry | Higher price | Nothing |

Both tiers are legitimate — there is no coercion. The price difference reflects the value publishers derive from aggregate usage analytics.

### 4.2 Setting Your Preference

```bash
estream marketplace telemetry set @acme/premium-adapter --share
estream marketplace telemetry set @acme/premium-adapter --private
```

You can change tiers at any time. Changes take effect at the next billing period.

### 4.3 What "Share" Means

When you opt into the Share tier:

- **Collected:** anonymized usage metrics (invocation counts, latency distributions, error rates)
- **Anonymization:** k-anonymous (minimum cohort size enforced) + differentially private (noise budget per query)
- **Aggregation window:** hourly minimum — no per-request visibility
- **No PII:** telemetry schemas cannot contain personally identifiable information
- **Auditable:** the telemetry schema hash is published in the manifest, so you can verify exactly what is collected

### 4.4 Viewing Tier Savings

```bash
estream marketplace telemetry savings @acme/premium-adapter
# Share tier: $8.50/month
# Private tier: $12.00/month
# Savings: $3.50/month (29%)
```

---

## 5. Managing Installed Packages

### 5.1 Listing Installed Packages

```bash
estream marketplace list
```

```
  PACKAGE                    VERSION  CATEGORY       STATUS
  estream-wire-fix           1.0.0    wire-adapter   Active
  data-trading               1.2.0    data-schema    Active
  @acme/premium-adapter      2.1.0    smart-circuit  Active
  @old/legacy-component      1.0.0    smart-circuit  Deprecated (sunset: 2027-06-01)
```

### 5.2 Checking for Updates

```bash
estream marketplace outdated
```

```
  PACKAGE              CURRENT  LATEST  TYPE
  data-trading         1.2.0    1.3.0   Minor
  @acme/premium-adapter 2.1.0  2.2.0   Minor
```

### 5.3 Upgrading Packages

```bash
estream marketplace upgrade data-trading            # Upgrade single package
estream marketplace upgrade --all                   # Upgrade all packages
estream marketplace upgrade --shadow @acme/pkg@3.0  # Shadow test major upgrade
```

Shadow testing runs old and new versions side-by-side against production traffic before committing to the upgrade.

### 5.4 Dependency Graph

```bash
estream marketplace deps
```

```
  estream-wire-fix v1.0.0
  └── data-trading v1.2.0

  @acme/premium-adapter v2.1.0
  ├── data-trading v1.2.0 (shared)
  └── @acme/core-lib v1.0.0
```

Dependencies are resolved using the `component_dependencies` DAG with hardware-enforced `enforce acyclic` cycle detection and `topo_sort` for installation ordering.

### 5.5 Verifying Installed Packages

```bash
estream marketplace verify estream-wire-fix
```

```
  Verifying estream-wire-fix v1.0.0...
    ML-DSA-87 signature:    ✓ (key: estream-signing-key-01)
    SHA3-256 checksum:      ✓
    Merkle root:            ✓
    PoVC attestation:       ✓ (compiler: estream-compiler-0.9.1)
    Signed at:              2026-02-10T12:00:00Z
```

### 5.6 Uninstalling

```bash
estream marketplace uninstall @acme/legacy-component
```

---

## 6. Billing and Usage Tracking

### 6.1 Usage Dashboard

```bash
estream marketplace usage
```

```
  PACKAGE                     THIS MONTH    COST        LICENSE
  @acme/premium-adapter       12,450 exec   $4.98       Usage-Based (Share)
  @acme/analytics-widget      —             $8.50/mo    Subscription (Share)
  estream-wire-fix            89,200 exec   Free        Free
```

### 6.2 Metering Model

Usage-based packages are metered per-package in isolated lex scopes:

- **Per-minute readings** — recorded by the `record_meter_reading` circuit
- **Hourly blind aggregation** — aggregated by `aggregate_hourly`, no per-request correlation
- **Blinded billing events** — the `emit_billing_event` circuit produces witness-attested billing records

Each package meters independently. No cross-package correlation is possible — customer lex scope isolation is hardware-enforced.

### 6.3 Billing Events

All billing flows through blinded customer IDs:

```bash
estream marketplace billing history
```

```
  DATE         PACKAGE                  UNITS      AMOUNT
  2026-02-26   @acme/premium-adapter    1,205 exec $0.48
  2026-02-25   @acme/premium-adapter    1,340 exec $0.54
  ...
```

### 6.4 Settlement

Revenue settlement is atomic and multi-party:

```
Customer payment → [Settlement Circuit] → Publisher (85%) + Platform (10%) + Referrer (5%)
```

Every settlement produces a witness-signed `SettlementTransaction` with PoVC attestation. You can verify any settlement:

```bash
estream marketplace billing verify <settlement-id>
```

### 6.5 Cost Estimation

Before installing a usage-based component, estimate costs:

```bash
estream marketplace cost-estimate @acme/premium-adapter --monthly-executions 50000
```

```
  @acme/premium-adapter cost estimate:
    Rate:           $0.0004/exec (Share tier)
    Monthly est.:   $20.00
    Witness cost:   $0.50 (T2, included)
    Platform fee:   $2.05
    Total est.:     $22.55
```

---

## 7. Security

### 7.1 Signature Verification

Every installed package has its ML-DSA-87 (FIPS 204) signature verified automatically. The verification:

1. Recomputes SHA3-256 hashes of all package files
2. Rebuilds the Merkle tree
3. Verifies the ML-DSA-87 signature over the Merkle root against the publisher's registered public key

This happens in Rust/WASM — TypeScript never touches signing keys or performs verification.

### 7.2 Trust Indicators

| Badge | Meaning |
|-------|---------|
| **Official** | Built by the eStream team |
| **Verified** | Publisher identity verified (KYC) |
| **Tested** | CI/CD with >80% test coverage |
| **Audited** | Third-party security audit completed |
| **Certified** | eStream team review + ongoing compliance |
| **PostQuantum** | Uses PQ-safe cryptographic primitives |

### 7.3 Reporting Issues

```bash
estream marketplace report @acme/suspicious-component --reason "security-vulnerability"
```

---

## Next Steps

- [Pricing Strategy Guide](pricing-strategy.md) — Understand share/private tiers and license types
- [Solution Builder Guide](solution-builder.md) — Work with multi-package solutions
- [Privacy Guarantees](../../specs/standards/PRIVACY_GUARANTEES_SPEC.md) — Formal privacy properties
- [Marketplace Spec §14](../../specs/ESTREAM_MARKETPLACE_SPEC.md) — Full CLI reference
