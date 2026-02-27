# Solution Builder Guide

> How to compose multi-package solutions with branding, lex policies, customer onboarding, and revenue waterfalls.

**Audience:** Solution builders packaging marketplace components for vertical markets
**Prerequisites:** Familiarity with [Publisher Getting Started](publisher-getting-started.md) and [Pricing Strategy](pricing-strategy.md)

---

## 1. Solution Architecture

### 1.1 Three-Tier Model

Solutions use a Platform → Solution → Customer tier hierarchy:

```
┌──────────────────────────────────────────────────┐
│  Platform Tier (Tier 0)                          │
│  eStream platform lex, RBAC, metering            │
│  ┌──────────────────────────────────────────┐    │
│  │  Solution Tier (Tier 1)                  │    │
│  │  Your bundled packages + lex policies    │    │
│  │  ┌──────────────────────────────────┐    │    │
│  │  │  Customer Tier (Tier 2)          │    │    │
│  │  │  Customer-specific lex boundary  │    │    │
│  │  │  Isolated package activation     │    │    │
│  │  └──────────────────────────────────┘    │    │
│  │  ┌──────────────────────────────────┐    │    │
│  │  │  Customer Tier (Tier 2)          │    │    │
│  │  │  Another customer's boundary     │    │    │
│  │  └──────────────────────────────────┘    │    │
│  └──────────────────────────────────────────┘    │
└──────────────────────────────────────────────────┘
```

Each tier is a **lex boundary** — a hardware-attested isolation scope defined in `solutions/lex_boundary.fl`. Visibility flows strictly downward: parents can observe children, but children cannot observe parents or siblings.

### 1.2 What a Solution Contains

| Component | Description |
|-----------|-------------|
| **Bundled Packages** | Curated set of marketplace packages |
| **Lex Policies** | Access control rules for the solution scope |
| **Branding** | Display name, theme, logo, documentation URL |
| **Pricing Model** | Per-seat, per-usage, flat, or tiered |
| **Revenue Waterfall** | Atomic settlement split across tiers |
| **Onboarding Flow** | Customer provisioning and package activation |

### 1.3 Solution Manifest

Every solution is described by a `SolutionManifest` (from `solutions/solution_manifest.fl`):

| Field | Type | Description |
|-------|------|-------------|
| `solution_id` | bytes(32) | SHA3-256 of name + version + publisher |
| `name` | bytes(128) | Solution display name |
| `version` | bytes(32) | Semantic version |
| `publisher` | bytes(64) | Publisher PRIME identity |
| `packages_count` | u16 | Number of bundled packages |
| `lex_policy_hash` | bytes(32) | SHA3-256 of lex policy document |
| `branding_hash` | bytes(32) | SHA3-256 of branding assets |
| `pricing_model` | u8 | 0=PerSeat, 1=PerUsage, 2=Flat, 3=Tiered |
| `tier` | u8 | 0=Platform, 1=Solution, 2=Customer |

---

## 2. Creating a Solution Bundle

### 2.1 Initialize

```bash
estream marketplace solution create \
  --name "Synergy Carbon Edge" \
  --description "Complete carbon credit lifecycle management" \
  --pricing-model tiered
```

This creates a solution manifest and registers it on the `solution_registry` stream.

### 2.2 Solution Directory Structure

```
synergy-carbon-edge/
├── solution.toml                  # Solution manifest
├── branding/
│   ├── logo.svg
│   ├── theme.json
│   └── docs-url.txt
├── lex-policies/
│   ├── solution-rbac.lex
│   └── customer-default.lex
├── packages/
│   ├── @synergy/carbon-mint.toml     # Package reference + version pin
│   ├── @synergy/emission-report.toml
│   └── @partner/impact-counter.toml
└── onboarding/
    └── provision-template.toml
```

### 2.3 Solution Manifest (`solution.toml`)

```toml
[solution]
name = "Synergy Carbon Edge"
version = "1.0.0"
publisher = "synergy-carbon"
description = "Complete carbon credit lifecycle management for industrial operators"
pricing_model = "tiered"

[solution.branding]
display_name = "Synergy Carbon Edge"
theme_hash = "auto"
logo_hash = "auto"
documentation_url = "https://docs.synergy-carbon.io/edge"

[solution.lex_policy]
path = "lex-policies/solution-rbac.lex"

[[solution.packages]]
name = "@synergy/carbon-mint"
version = "^2.0.0"
required = true

[[solution.packages]]
name = "@synergy/emission-report"
version = "^1.5.0"
required = true

[[solution.packages]]
name = "@partner/impact-counter"
version = "^1.0.0"
required = false

[solution.pricing]
model = "tiered"
platform_share_bps = 1000
solution_share_bps = 2000

[solution.pricing.publisher_shares]
"@synergy/carbon-mint" = 4000
"@synergy/emission-report" = 2000
"@partner/impact-counter" = 1000
```

---

## 3. Adding Packages

### 3.1 Adding a Package

```bash
estream marketplace solution add-package \
  --solution "synergy-carbon-edge" \
  --package "@synergy/carbon-mint" \
  --version "^2.0.0" \
  --required
```

This calls the `add_package_to_solution` circuit with `rbac [solution_admin]` and PoVC attestation.

### 3.2 Required vs. Optional Packages

| Flag | Behavior |
|------|----------|
| `--required` | Must be installed for every customer; included in base price |
| (default) | Optional add-on; customer can choose to activate |

### 3.3 Removing a Package

```bash
estream marketplace solution remove-package \
  --solution "synergy-carbon-edge" \
  --package "@partner/impact-counter"
```

### 3.4 Validating the Bundle

```bash
estream marketplace solution validate "synergy-carbon-edge"
```

The `validate_solution_bundle` circuit checks:
- All required packages resolve in the registry
- No dependency conflicts between bundled packages
- Lex policy references valid lex paths
- Pricing shares sum to 10,000 bps
- Package count > 0

---

## 4. Branding

### 4.1 Theme Configuration

```json
{
  "primary_color": "#1B5E20",
  "secondary_color": "#4CAF50",
  "background": "#FAFAFA",
  "font_family": "Inter",
  "code_font": "JetBrains Mono"
}
```

### 4.2 Branding Assets

| Asset | Format | Max Size | Purpose |
|-------|--------|----------|---------|
| Logo | SVG or PNG | 50 KB | Solution identity in marketplace |
| Theme | JSON | 5 KB | Color scheme for customer console |
| Documentation URL | URL | — | Link to solution docs |

Branding is hashed and included in the `SolutionManifest.branding_hash` for integrity verification.

---

## 5. Lex Policies

### 5.1 Solution-Level RBAC

Define access control for the solution scope:

```
# lex-policies/solution-rbac.lex
lex esn/marketplace/solutions/synergy-carbon-edge

rbac solution_admin {
    allow create_solution
    allow add_package_to_solution
    allow remove_package_from_solution
    allow provision_customer
    allow configure_waterfall
    allow request_upgrade
}

rbac solution_developer {
    allow validate_solution_bundle
    allow get_solution_info
}

rbac customer_admin {
    allow accept_upgrade
    allow rollback_upgrade
    allow set_telemetry_preference
}
```

### 5.2 Customer-Default Policy

Template policy applied to each new customer lex boundary:

```
# lex-policies/customer-default.lex
lex esn/marketplace/solutions/synergy-carbon-edge/customer/{customer_id}

rbac customer_user {
    allow read_package_data
    allow invoke_circuit
}

rbac customer_admin {
    allow manage_telemetry
    allow view_billing
    allow accept_upgrade
}
```

### 5.3 Lex Boundary Nesting

Each customer gets their own lex boundary nested under the solution boundary. The `create_lex_boundary` and `nest_boundary` circuits enforce:

- **Tier descent:** child tier must be > parent tier (Platform < Solution < Customer)
- **PRIME attestation:** each boundary is hardware-attested via `attest_boundary`
- **Visibility isolation:** `enforce_visibility` ensures parents can observe children but not the reverse

```
Platform lex (tier 0)
  └── Solution lex (tier 1) — synergy-carbon-edge
        ├── Customer lex (tier 2) — acme-corp
        ├── Customer lex (tier 2) — globex-inc
        └── Customer lex (tier 2) — initech-co
```

---

## 6. Customer Onboarding and Provisioning

### 6.1 Provisioning a Customer

```bash
estream marketplace solution provision \
  --solution "synergy-carbon-edge" \
  --customer-lex "acme-corp"
```

The `provision_customer` circuit:

1. Creates a new `CustomerProvision` record (status: Pending)
2. Creates a customer lex boundary nested under the solution boundary
3. Attests the boundary with PRIME
4. Returns a `ProvisionId` for tracking

### 6.2 Activating Packages

After provisioning, activate the bundled packages for the customer:

```bash
estream marketplace solution activate \
  --provision <provision-id> \
  --packages all
```

The `activate_packages` circuit transitions the provision status from Pending (0) to Active (1).

### 6.3 Provisioning Flow

```
Solution Admin
    │
    ├── 1. provision_customer(solution_id, customer_lex_id)
    │       → CustomerProvision { status: Pending }
    │
    ├── 2. create_lex_boundary(solution_boundary, tier=2)
    │       → Customer lex boundary created
    │
    ├── 3. attest_boundary(customer_boundary, prime_key)
    │       → PRIME attestation recorded
    │
    ├── 4. activate_packages(provision, package_count)
    │       → CustomerProvision { status: Active }
    │
    └── 5. Customer can now use all required packages
```

### 6.4 Deactivating a Customer

```bash
estream marketplace solution deactivate --provision <provision-id>
```

This calls `deactivate_customer` which transitions the provision to Deactivated (2). The customer's lex boundary can optionally be revoked.

---

## 7. Upgrade Orchestration

### 7.1 Requesting an Upgrade

When you publish a new solution version with updated packages:

```bash
estream marketplace solution upgrade \
  --solution "synergy-carbon-edge" \
  --from "1.0.0" \
  --to "2.0.0" \
  --customer "acme-corp" \
  --auto-accept false
```

The `request_upgrade` circuit creates an `UpgradeRequest` and emits it to the `upgrade_events` stream.

### 7.2 Upgrade Lifecycle

```
UpgradeRequest
    │
    ├── auto_accept = true  → Automatically applied
    │
    └── auto_accept = false → Customer admin reviews
            │
            ├── accept_upgrade → UpgradeStatus { status: Accepted → Applied }
            │
            └── rollback_upgrade → UpgradeStatus { status: RolledBack }
```

| Status | Value | Description |
|--------|-------|-------------|
| Pending | 0 | Upgrade requested, awaiting customer response |
| Accepted | 1 | Customer accepted the upgrade |
| Applied | 2 | Upgrade successfully applied |
| Rejected | 3 | Customer rejected the upgrade |
| RolledBack | 4 | Upgrade was applied but rolled back |

### 7.3 Auto-Accept Policy

For non-breaking upgrades (patch and minor versions), solutions can configure auto-accept:

```toml
[solution.upgrade_policy]
auto_accept_patch = true
auto_accept_minor = true
auto_accept_major = false
```

### 7.4 Rollback

If an upgrade causes issues, the customer admin can trigger a rollback:

```bash
estream marketplace solution rollback --request <request-id>
```

The `rollback_upgrade` circuit reverts the customer's packages to their pre-upgrade versions and records the rollback with PoVC attestation.

---

## 8. Revenue Waterfall Configuration

### 8.1 Waterfall Model

Revenue flows atomically through the three-tier hierarchy:

```
Customer Payment ($100)
    │
    ├── Platform   (10%, 1,000 bps)  →  $10
    ├── Solution   (20%, 2,000 bps)  →  $20
    └── Publishers (70%, 7,000 bps)  →  $70
          ├── @synergy/carbon-mint    (4,000 bps)  →  $40
          ├── @synergy/emission-report (2,000 bps) →  $20
          └── @partner/impact-counter  (1,000 bps) →  $10
```

### 8.2 Configuring the Waterfall

```bash
estream marketplace solution waterfall configure \
  --solution "synergy-carbon-edge" \
  --platform-share 1000 \
  --solution-share 2000 \
  --publisher-shares "@synergy/carbon-mint:4000,@synergy/emission-report:2000,@partner/impact-counter:1000"
```

The `configure_waterfall` circuit validates the invariant: all shares must sum to 10,000 bps.

### 8.3 Waterfall Execution

Each customer payment triggers `execute_waterfall`:

- All payouts are computed atomically
- Publisher payouts are hashed (not individually visible to the platform)
- Blinded customer ID ensures the platform cannot correlate payments to real customers
- PoVC attestation proves correct execution

### 8.4 Waterfall Proofs

Generate a ZK proof that waterfall execution was correct for a given period:

```bash
estream marketplace solution waterfall-proof \
  --solution "synergy-carbon-edge" \
  --period-start 2026-02-01 \
  --period-end 2026-02-28
```

Partners can verify the proof to confirm they received their correct share.

---

## 9. RBAC Roles

### 9.1 Solution Roles

| Role | Scope | Permissions |
|------|-------|-------------|
| `solution_admin` | Solution-wide | Create solutions, add/remove packages, provision customers, configure waterfalls, request upgrades |
| `solution_developer` | Solution-wide | Validate bundles, view solution info, read-only access |
| `customer_admin` | Per-customer | Accept/reject upgrades, manage telemetry, view billing |
| `marketplace_admin` | Platform-wide | Override any solution operation, resolve disputes |
| `platform_admin` | Platform-wide | Create platform-level lex boundaries, manage solution admins |

### 9.2 Role Assignment

Roles are assigned at the lex boundary level:

```bash
estream marketplace solution role assign \
  --solution "synergy-carbon-edge" \
  --user <prime-id> \
  --role solution_developer
```

### 9.3 Role Inheritance

Roles follow the lex hierarchy:
- `platform_admin` implicitly has `solution_admin` for all solutions
- `solution_admin` implicitly has `customer_admin` for all customers in that solution
- `customer_admin` has no visibility into other customers or the solution tier

---

## 10. Publishing the Solution

### 10.1 Validation

```bash
estream marketplace solution validate "synergy-carbon-edge"
```

### 10.2 Publish

```bash
estream marketplace solution publish "synergy-carbon-edge"
```

The `publish_solution` circuit registers the solution on `solution_registry` and emits a `SolutionManifest` event to `solution_events`.

### 10.3 Marketplace Discovery

Published solutions appear in the marketplace alongside individual packages. Customers can:

```bash
estream marketplace search "carbon" --type solution
estream marketplace solution info "synergy-carbon-edge"
```

---

## Next Steps

- [Publisher Getting Started](publisher-getting-started.md) — Package creation fundamentals
- [Pricing Strategy](pricing-strategy.md) — Per-package pricing and telemetry configuration
- [Customer Guide](customer-guide.md) — How your customers interact with solutions
- [Privacy Guarantees](../../specs/standards/PRIVACY_GUARANTEES_SPEC.md) — Lex boundary isolation guarantees
- [Marketplace Spec §5](../../specs/ESTREAM_MARKETPLACE_SPEC.md) — Graph-based registry model
