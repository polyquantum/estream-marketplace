# Privacy Guarantees Specification

> Formal specification of marketplace privacy guarantees — what each party can and cannot learn.

**Status:** Draft
**Version:** 1.0.0
**Epic:** estream-marketplace#8 (Documentation & Standards)
**Source:** `licensing/blinded_tokens.fl`, `licensing/metering.fl`, `licensing/pricing_tiers.fl`, `licensing/settlement.fl`

---

## 1. Overview

The eStream Marketplace is designed around a fundamental privacy constraint: **the marketplace operator cannot correlate purchaser identities, license ownership, or cross-package usage**. This document formally specifies what each party can and cannot learn, the cryptographic protocols that enforce these guarantees, and the threat model.

### 1.1 Privacy Principles

| Principle | Description |
|-----------|-------------|
| **Purchase unlinkability** | The operator cannot learn which real customer purchased which package |
| **License ownership opacity** | License tokens are blinded — ownership cannot be correlated to identity |
| **Cross-package isolation** | Usage of package A reveals nothing about usage of package B |
| **Telemetry minimization** | Share tier collects only k-anonymous, differentially private aggregates |
| **Billing isolation** | Blinded customer IDs prevent transaction correlation |

---

## 2. What the Marketplace Operator CANNOT Learn

### 2.1 Identity Privacy

| Protected Information | Mechanism |
|----------------------|-----------|
| **Purchaser real identity** | Blinded customer IDs (`BlindedCustomerId = bytes(32)`) — derived from PRIME identity via one-way blinding |
| **Publisher-customer relationship** | Settlement uses blinded IDs; operator sees revenue totals but not who paid whom |
| **Customer package portfolio** | Each package uses an independent blinded ID; no cross-package ID linkage |
| **Customer organization** | Lex boundary isolation — customer lex scope is invisible to the platform tier |

### 2.2 License Ownership

| Protected Information | Mechanism |
|----------------------|-----------|
| **Token-to-customer mapping** | `BlindedToken.blinded_customer_id` is a one-way derivation; operator cannot reverse |
| **License activation count per customer** | Tokens are independently issued; no correlation possible |
| **Multi-package license correlation** | Each package issues tokens with independent blinded IDs |
| **License transfer history** | Token portability uses ZK ownership proofs; no identity revealed during transfer |

### 2.3 Usage Correlation

| Protected Information | Mechanism |
|----------------------|-----------|
| **Cross-package usage patterns** | Metering is per-package in isolated lex scopes (`MeterConfig.isolation_scope`) |
| **Individual request data** | Hourly minimum aggregation window; no per-request visibility |
| **Customer-specific usage** | Usage data keyed by blinded customer ID |
| **Time-of-use correlation** | Meter readings have 1-hour minimum aggregation; no sub-hour timing correlation |

---

## 3. Blinded Token Protocol

### 3.1 Protocol Overview

The marketplace uses a Chaum-style blinded token protocol adapted for post-quantum safety. The protocol is implemented in `licensing/blinded_tokens.fl`.

### 3.2 Token Issuance

```
Customer                     Marketplace Issuer
    │                              │
    │  1. Generate BlindingFactor  │
    │     bf = random(64 bytes)    │
    │                              │
    │  2. Compute blinded ID       │
    │     blinded_id = HKDF(       │
    │       PRIME_id, bf,          │
    │       "estream-license-v1")  │
    │                              │
    │  3. Request token            │
    │     ──────────────────────►  │
    │     { blinded_id, pkg_id,    │
    │       bf_hash }              │
    │                              │
    │                 4. Issue token│
    │  ◄──────────────────────────│
    │  { token_id, blinded_id,     │
    │    token_signature }         │
    │                              │
    │  5. Store token + bf locally │
```

### 3.3 Token Data Structure

From `licensing/blinded_tokens.fl`:

| Field | Type | Description |
|-------|------|-------------|
| `token_id` | bytes(32) | Unique token identifier |
| `blinded_customer_id` | bytes(32) | One-way blinded derivation of PRIME ID |
| `package_id` | bytes(32) | Package this token grants access to |
| `blinding_factor_hash` | bytes(32) | SHA3-256 of the blinding factor (stored by customer) |
| `issued_at` | u64 | Issuance timestamp |
| `expires_at` | u64 | Expiration timestamp |
| `token_signature` | bytes(4627) | ML-DSA-87 signature by issuer |
| `is_active` | bool | Active/revoked status |

### 3.4 Token Verification

The `verify_blinded_token` circuit verifies a token without learning the customer's identity:

1. Compute verification hash from token fields
2. Verify ML-DSA-87 signature against the issuer's public key
3. Check `is_active` flag and `expires_at`

The verifier learns only that a valid token exists for a given package — not who owns it.

### 3.5 Ownership Proofs

The `prove_token_ownership` circuit enables ZK proof of license ownership:

```
Customer                    Verifier
    │                          │
    │  1. Provide:             │
    │     - BlindedToken       │
    │     - BlindingFactor     │
    │     - OwnershipProof     │
    │     ──────────────────►  │
    │                          │
    │     2. Verify:           │
    │     - SHA3(bf) == token. │
    │       blinding_factor_hash
    │     - token.is_active    │
    │                          │
    │  ◄──────────────────────│
    │     { is_valid: true }   │
```

The verifier learns that the customer owns the token, but the blinding factor never leaves the customer's device in the clear — it is only verified via hash comparison.

### 3.6 Token Renewal

The `renew_blinded_token` circuit extends token expiration without revealing identity:

1. Customer presents the existing token
2. New `expires_at` is set
3. Renewal is signed with ML-DSA-87
4. No re-identification occurs — the blinded customer ID remains unchanged

### 3.7 PQ Safety

| Primitive | PQ-Safe Algorithm | NIST Level |
|-----------|-------------------|------------|
| Token signature | ML-DSA-87 (FIPS 204) | Level 5 |
| Blinding factor derivation | HKDF-SHA3-256 | N/A (symmetric) |
| Token hash | SHA3-256 (FIPS 202) | N/A (hash) |

---

## 4. Metering Isolation Guarantees

### 4.1 Per-Package Isolation

Every package meters independently in its own lex scope. From `licensing/metering.fl`:

```
Package A lex scope                Package B lex scope
┌──────────────────┐              ┌──────────────────┐
│ MeterConfig A    │              │ MeterConfig B    │
│ isolation_scope: │              │ isolation_scope: │
│  "pkg-a-lex-..."│              │  "pkg-b-lex-..."│
│                  │              │                  │
│ MeterReading A₁  │              │ MeterReading B₁  │
│ MeterReading A₂  │              │ MeterReading B₂  │
│       ▼          │              │       ▼          │
│ HourlyAggregate  │              │ HourlyAggregate  │
│       ▼          │              │       ▼          │
│ BlindedBilling   │              │ BlindedBilling   │
└──────────────────┘              └──────────────────┘
      ╳ No cross-scope access ╳
```

### 4.2 Isolation Verification

The `verify_metering_isolation` circuit formally verifies that two packages' metering data cannot be correlated:

```fastlang
circuit verify_metering_isolation(package_a: PackageId, package_b: PackageId,
    readings_a: list<MeterReading>, readings_b: list<MeterReading>) -> bool
```

This circuit asserts `package_a != package_b` and verifies that readings are scoped to their respective packages.

### 4.3 Metering Data Flow

| Stage | Data | Retention | Correlation Risk |
|-------|------|-----------|-----------------|
| `MeterReading` | Per-minute, per-package | 1 hour TTL | None — destroyed after aggregation |
| `HourlyAggregate` | Hourly, per-package, per-blinded-customer | 90 days | Blinded customer ID only |
| `BlindedBillingEvent` | Hourly, per-package, per-blinded-customer | 365 days | Witness-signed, blinded |

### 4.4 No Cross-Package Aggregation

The metering system enforces that:

- Each `MeterConfig` is bound to a single `PackageId`
- `MeterReading`s can only be aggregated within their `isolation_scope`
- `HourlyAggregate`s are keyed by `(package_id, blinded_customer_id)` — no composite keys across packages
- The platform cannot construct a query that joins metering data across packages

---

## 5. Telemetry Privacy

### 5.1 Share Tier Guarantees

When a customer opts into the Share tier, the following privacy guarantees apply:

| Guarantee | Specification |
|-----------|--------------|
| **k-Anonymity** | Every aggregate cohort contains at least `cohort_min_size` (>= 50) customers |
| **Differential privacy** | Laplace noise with epsilon budget `noise_budget` applied to all aggregate queries |
| **No PII** | `TelemetrySchema.contains_pii` must be `false`; schemas with PII are rejected by `validate_telemetry_schema` |
| **Hourly minimum aggregation** | `aggregate_window >= 1h` — no per-request or per-minute visibility |
| **Schema transparency** | `schema_hash` published in manifest — customers can audit exactly what is collected |

### 5.2 Anonymization Methods

From `licensing/pricing_tiers.fl`:

| Value | Method | Description |
|-------|--------|-------------|
| 0 | None | Not permitted for Share tier |
| 1 | k-Anonymity | Suppresses cohorts smaller than `cohort_min_size` |
| 2 | Differential Privacy | Adds calibrated Laplace noise per `noise_budget` |
| 3 | Aggregation | Only aggregate statistics (sum, mean, percentiles) |

All Share tier schemas must use anonymization method >= 1.

### 5.3 What Publishers Receive

| Data | Format | Granularity |
|------|--------|-------------|
| Invocation counts | Aggregate sum | Per hour, per cohort |
| Latency distributions | Percentiles (p50, p95, p99) | Per hour, per cohort |
| Error rates | Aggregate ratio | Per hour, per cohort |
| Custom metrics | As defined in `[telemetry].metrics` | Per `aggregate_window`, per cohort |

### 5.4 What Publishers Do NOT Receive

| Data | Reason |
|------|--------|
| Individual customer metrics | k-Anonymity + aggregation |
| Customer identity (even blinded) | Telemetry data is decoupled from billing identity |
| Per-request telemetry | Hourly minimum aggregation |
| Cross-package usage | Metering isolation (§4) |
| Time-correlated usage patterns | Noise injection + hourly aggregation |

---

## 6. Billing Isolation

### 6.1 Blinded Customer IDs

All billing data uses `BlindedCustomerId = bytes(32)`:

```
Real PRIME Identity
        │
        ▼  (one-way HKDF derivation, per-package context)
BlindedCustomerId_Package_A
BlindedCustomerId_Package_B
BlindedCustomerId_Package_C
```

Each package gets an independently derived blinded customer ID. The platform cannot link:
- Package A's blinded ID to Package B's blinded ID
- Any blinded ID back to the real PRIME identity

### 6.2 No Transaction Correlation

Settlement transactions use blinded customer IDs:

| Field | Type | Visible to Operator |
|-------|------|-------------------|
| `settlement_id` | bytes(32) | Yes (unique per transaction) |
| `package_id` | bytes(32) | Yes |
| `publisher_id` | bytes(32) | Yes |
| `blinded_customer_id` | bytes(32) | Yes (but cannot deanonymize) |
| `gross_amount_micros` | u64 | Yes (for revenue accounting) |
| Real customer identity | — | **No** |
| Cross-package correlation | — | **No** |

### 6.3 Billing Event Pipeline

```
MeterReading (per-minute, 1h TTL)
    │
    ▼ aggregate_hourly
HourlyAggregate (per-package, per-blinded-customer, 90d retention)
    │
    ▼ emit_billing_event
BlindedBillingEvent (witness-signed, 365d retention)
    │
    ▼ execute_settlement
SettlementTransaction (atomic, multi-party, PoVC attested)
```

At every stage:
- Customer identity is blinded
- Data is scoped to a single package
- Witness signatures provide non-repudiation without identity revelation

---

## 7. ZK Proof Specifications

### 7.1 Revenue Proofs

The `generate_revenue_proof` circuit produces a `ZkRevenueProof`:

| Field | Type | Purpose |
|-------|------|---------|
| `proof_id` | bytes(32) | Unique proof identifier |
| `publisher_id` | bytes(32) | Publisher PRIME identity |
| `period_start` | u64 | Revenue period start |
| `period_end` | u64 | Revenue period end |
| `aggregate_revenue_micros` | u64 | Total revenue for period |
| `transaction_count` | u64 | Number of transactions |
| `proof_data` | bytes(256) | ZK proof payload |
| `witness_signature` | bytes(4627) | ML-DSA-87 attestation |

**What the proof reveals:** aggregate revenue and transaction count for a specific publisher and time period.

**What the proof does NOT reveal:** individual transaction amounts, customer identities (blinded or otherwise), per-package breakdown (unless publisher chooses to include it).

### 7.2 License Ownership Proofs

The `prove_token_ownership` circuit enables proof of license ownership:

**What the proof reveals:** that the prover possesses a valid, active license token for a specific package.

**What the proof does NOT reveal:** the prover's PRIME identity, other packages they have licenses for, or their usage history.

### 7.3 Waterfall Proofs

The `generate_waterfall_proof` circuit proves correct revenue waterfall execution:

**What the proof reveals:** total revenue for a solution over a time period, and that the waterfall split was correctly executed.

**What the proof does NOT reveal:** individual customer payments, per-customer breakdown, or customer identities.

### 7.4 Settlement Verification

The `verify_revenue_proof` circuit allows third-party verification:

```
Verifier receives:  ZkRevenueProof + publisher's public key
Verifier checks:    ML-DSA-87 signature over proof_data
Verifier learns:    "Publisher X earned $Y from Z transactions in period P"
Verifier does NOT learn: any customer data
```

---

## 8. Threat Model

### 8.1 Parties

| Party | Description |
|-------|-------------|
| **Customer** | Purchases and uses marketplace packages |
| **Publisher** | Creates and sells marketplace packages |
| **Marketplace Operator** | Runs the registry, settlement, and metering infrastructure |
| **External Observer** | Network adversary, government agency, or other third party |

### 8.2 What Each Party Can Learn

#### Customer

| Can Learn | Cannot Learn |
|-----------|-------------|
| Their own purchased packages | Other customers' packages or purchases |
| Their own usage metrics | Other customers' usage |
| Their own billing history | Other customers' billing |
| Publisher identity for packages they use | Publisher revenue from other customers |
| Their own blinding factors | Other customers' blinding factors |

#### Publisher

| Can Learn | Cannot Learn |
|-----------|-------------|
| Aggregate revenue (their own packages) | Individual customer identities |
| Aggregate telemetry (Share tier only) | Per-customer usage breakdown |
| Transaction count | Which customer made which purchase |
| Cohort-level usage patterns | Individual usage patterns |
| License issuance count | License owner identities |

#### Marketplace Operator

| Can Learn | Cannot Learn |
|-----------|-------------|
| Package registry (public) | Customer-to-package purchase mapping |
| Publisher profiles (public) | License token ownership |
| Aggregate platform revenue | Cross-package customer correlation |
| Settlement totals per publisher | Individual customer billing |
| Blinded customer IDs (per-package) | Real customer identities |
| Package download counts | Per-customer download history |

#### External Observer

| Can Learn | Cannot Learn |
|-----------|-------------|
| Published package metadata (public) | Any customer data |
| Publisher profiles (public) | Any transaction data |
| Network traffic patterns (encrypted) | Transaction contents (ML-KEM-1024 encrypted) |

### 8.3 Attack Scenarios

| Attack | Mitigation |
|--------|-----------|
| **Operator deanonymizes customers** | Blinded IDs are one-way HKDF derivations; no reverse mapping exists |
| **Cross-package correlation via timing** | Hourly aggregation + noise injection prevents timing correlation |
| **Publisher colludes with operator** | Publisher sees only aggregates; operator sees only blinded IDs; neither can reconstruct full picture |
| **Quantum computer breaks signatures** | ML-DSA-87 is NIST Level 5 PQ-safe; blinding uses symmetric HKDF |
| **Subpoena for customer data** | Operator has no customer identity data; blinded IDs cannot be reversed |
| **Metering side-channel** | Per-package lex isolation; metering circuits verify isolation invariant |
| **Token theft** | Tokens are bound to PRIME biometric identity; stolen token requires biometric to activate |

### 8.4 Trust Assumptions

| Assumption | Justification |
|------------|--------------|
| HKDF is one-way | SHA3-256 preimage resistance |
| ML-DSA-87 is unforgeable | NIST FIPS 204, Level 5 |
| Lex boundaries are enforced | Hardware-attested via PRIME |
| Metering isolation holds | Verified by `verify_metering_isolation` circuit |
| Differential privacy provides formal guarantees | Calibrated Laplace mechanism with bounded epsilon |

---

## References

- `licensing/blinded_tokens.fl` — Blinded token protocol implementation
- `licensing/metering.fl` — Per-package metering circuits
- `licensing/pricing_tiers.fl` — Share/Private tier management
- `licensing/settlement.fl` — Atomic multi-party settlement
- `solutions/lex_boundary.fl` — Lex boundary nesting and isolation
- [ESCX_FORMAT_SPEC.md](ESCX_FORMAT_SPEC.md) — Package format (attestation section)
- [MANIFEST_SCHEMA_SPEC.md](MANIFEST_SCHEMA_SPEC.md) — Telemetry schema definition
- [ESTREAM_MARKETPLACE_SPEC.md §8](../ESTREAM_MARKETPLACE_SPEC.md) — Pricing and licensing overview

---

*Created: February 2026*
*Status: Draft*
