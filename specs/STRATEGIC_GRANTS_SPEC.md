# Strategic License Grants Specification

> Publisher-declared, consumer-specific zero-fee full-source licensing for the eStream Marketplace.

**Status:** Draft
**Version:** 1.0.0
**Epic:** estream-marketplace#8

---

## 1. Overview

Strategic License Grants enable marketplace publishers to declare zero-fee, full-source access for specific consumer lex patterns. This allows corporate families (e.g., Poly Labs granting access to eStream) and strategic partners (e.g., Poly Labs granting access to Paragon) to consume marketplace packages without incurring fees, while external consumers pay normal marketplace pricing.

### 1.1 Design Principles

1. **Publisher-Declared** -- Only the package publisher can create, renew, or revoke grants
2. **Lex-Scoped** -- Grants target lex path patterns (globs), not individual consumers
3. **Zero-Linkage Preserving** -- Grants do not create cross-product correlation; blinded tokens auto-issued via grants are indistinguishable from purchased tokens
4. **Audit-Complete** -- Every grant action (create, match, token issue, renew, revoke) produces an ML-DSA-87 signed audit record
5. **Settlement-Compatible** -- Zero-fee transactions flow through the existing settlement pipeline at `gross_amount_micros = 0`

### 1.2 Grant Types

| Type | Code | Description | Example |
|------|------|-------------|---------|
| PlatformGrant | 0 | Same corporate family | Poly Labs → eStream (PolyQuantum) |
| StrategicPartner | 1 | Cross-organization strategic relationship | Poly Labs → Paragon |
| EcosystemGrant | 2 | Open ecosystem participants, community contributors | Publisher → approved community members |

---

## 2. Data Model

### 2.1 StrategicLicenseGrant

| Field | Type | Description |
|-------|------|-------------|
| `grant_id` | bytes(32) | SHA3-256 of (publisher + package_pattern + grantee_lex_pattern) |
| `publisher_spark_id` | bytes(32) | SPARK identity of the granting publisher |
| `package_pattern` | bytes(128) | Glob pattern matching package IDs (e.g., `polylabs/*`) |
| `grantee_lex_pattern` | LexPath | Glob pattern matching consumer lex paths (e.g., `lex://paragon/*`) |
| `granted_visibility` | SourceVisibility | Typically `3` (LicensedFull) |
| `pricing_override_micros` | u64 | `0` for no-fee grants |
| `grant_type` | GrantType | 0=PlatformGrant, 1=StrategicPartner, 2=EcosystemGrant |
| `include_source` | bool | Full FL source access |
| `include_fork_rights` | bool | Permission to fork and customize internally |
| `renewable` | bool | Auto-renew on expiry |
| `created_at` | u64 | Nanosecond timestamp |
| `expires_at` | u64 | Nanosecond timestamp |
| `witness_chain_hash` | bytes(32) | ML-DSA-87 witness attestation chain |
| `is_active` | bool | False after revocation |

### 2.2 GrantedEntitlement

Returned by `check_grant_entitlement` when a consumer/package pair matches an active grant.

| Field | Type | Description |
|-------|------|-------------|
| `grant_id` | GrantId | Which grant matched |
| `consumer_lex` | LexPath | The consumer's lex path |
| `package_id` | PackageRef | The package being accessed |
| `effective_visibility` | SourceVisibility | Granted visibility level |
| `has_source_access` | bool | Full source flag |
| `has_fork_rights` | bool | Fork permission flag |
| `pricing_micros` | u64 | Effective price (0 for free grants) |
| `grant_type` | GrantType | Type of grant that matched |

### 2.3 Lex Pattern Matching

Grant lex patterns use glob matching with the following semantics:

- `lex://polyquantum/*` matches all paths under `lex://polyquantum/` including nested paths
- `lex://paragon/*` matches `lex://paragon/foundation`, `lex://paragon/fo/family1`, etc.
- `lex://paragon/fo/*` matches only family office instances (narrower scope)

Package patterns follow the same glob semantics:

- `polylabs/*` matches all Poly Labs packages
- `polylabs/polyportal` matches only PolyPortal
- `polylabs/poly{docs,sign,audit}` matches specific products

---

## 3. Circuit Inventory

| Circuit | Precision | RBAC | Description |
|---------|-----------|------|-------------|
| `create_strategic_grant` | A | publisher, org_admin | Create a new grant with ML-DSA-87 witness attestation |
| `check_grant_entitlement` | A | (none) | Check if consumer lex + package matches an active grant |
| `auto_issue_granted_token` | A | (attested) | Auto-issue blinded license token at zero cost when grant matches |
| `renew_strategic_grant` | B | publisher, org_admin | Extend grant duration with fresh witness attestation |
| `revoke_strategic_grant` | B | publisher, org_admin, marketplace_admin | Revoke grant; existing tokens remain valid until expiry |
| `list_grants_for_package` | C | publisher, org_admin, marketplace_admin | Query active grants for a package |

---

## 4. Integration Points

### 4.1 Entitlement Resolution (composite_visibility.fl)

The `compute_sub_circuit_entitlement` circuit checks grants **before** token lookup for `LicensedFull` (visibility=3) sub-circuits:

```
Open (0)          → source access for everyone (reason 0)
LicensedFull (3)  → strategic grant check (reason 5) ← NEW
                  → license token check (reason 1)
                  → bundled with parent (reason 2)
                  → fallback: interface only (reason 4)
Compiled (2)      → compiled access for everyone (reason 3)
Interface (1)     → interface only (reason 4)
```

### 4.2 Resolution Priority (internal_marketplace.fl)

The `resolve_package_lex_priority` circuit now has a 5-step priority chain:

1. Internal Override (resolved_from=0)
2. Org Registry (resolved_from=1)
3. Cross-Lex Bridge (resolved_from=2)
4. **Strategic Grant (resolved_from=3)** -- NEW
5. Public Marketplace (resolved_from=4)

### 4.3 Settlement (settlement.fl)

No changes required. The existing `execute_settlement` circuit handles `gross_amount_micros = 0` correctly, producing zero payouts across publisher/platform/referrer with full ML-DSA-87 signed audit trail.

### 4.4 Blinded Tokens (blinded_tokens.fl)

No changes to the circuit itself. `auto_issue_granted_token` in `strategic_grants.fl` composes over the existing `issue_blinded_token` to handle grant-based issuance. Auto-issued tokens are indistinguishable from purchased tokens.

---

## 5. Manifest Declaration

Publishers declare grants in their `estream-component.toml`:

```toml
[[grants.strategic]]
grantee_lex = "lex://polyquantum/*"
visibility = "licensed"
pricing = "free"
type = "platform"
include_source = true
include_fork_rights = true

[[grants.strategic]]
grantee_lex = "lex://paragon/*"
visibility = "licensed"
pricing = "free"
type = "strategic_partner"
include_source = true
include_fork_rights = true
```

At publish time, each `[[grants.strategic]]` entry is materialized as a `StrategicLicenseGrant` in the grant registry.

---

## 6. Poly Labs Grant Configuration

### 6.1 eStream Grant (PlatformGrant)

All Poly Labs products grant eStream (PolyQuantum) zero-fee full-source access with fork rights:

- **Package pattern**: `polylabs/*`
- **Grantee lex**: `lex://polyquantum/*`
- **Visibility**: LicensedFull (3)
- **Pricing**: 0 micros
- **Fork rights**: true (enables InternalOverrideManifest from package_fork.fl)
- **Auto-renewable**: true

### 6.2 Paragon Grant (StrategicPartner)

Poly Labs products consumed by Paragon get strategic partner grants:

- **Package pattern**: `polylabs/*`
- **Grantee lex**: `lex://paragon/*` (includes `lex://paragon/fo/*` for family office instances)
- **Visibility**: LicensedFull (3)
- **Pricing**: 0 micros
- **Fork rights**: true
- **Auto-renewable**: true

### 6.3 Paragon Consumption Matrix

| Paragon Module | Poly Products | Bridge Circuit |
|---|---|---|
| Foundation | PolyDocs, PolySign, PolyAudit, PolyComply, PolyFiles, PolyMessenger, PolyOAuth | doc_bridge, compliance_bridge, rbac_profiles |
| Govern | PolyPortal | portal_bridge |
| Counsel | PolyDocs, PolySign | doc_bridge |
| Ledger | PolyAudit | compliance_bridge |
| Invest | PolyFiles | doc_bridge |
| Sentinel | PolyVPN, PolyMonitor | security_bridge |

---

## 7. Zero-Linkage Preservation

Strategic grants do not violate Poly Labs' zero-linkage privacy architecture:

1. Grants are publisher-declared, stored in the marketplace grant registry (not in product telemetry)
2. Blinded tokens auto-issued via grants are indistinguishable from purchased tokens
3. Settlement at $0 records no payment correlation
4. Each product's lex isolation remains intact -- the grant is per-product, not cross-product
5. Paragon family office instances inherit grants via lex hierarchy, not via cross-product linking

---

## 8. Streams

| Stream | Retention | Consumers |
|--------|-----------|-----------|
| `strategic_grants` | Forever | cli, console, licensing, publisher_dashboard, audit |
| `grant_audit` | 7 years | console, audit, compliance |
| `auto_issued_tokens` | 365 days | licensing, billing, audit |
