# Pricing Strategy Guide

> How to configure pricing tiers, revenue sharing, telemetry schemas, and license types for your marketplace packages.

**Audience:** Publishers configuring pricing and monetization
**Prerequisites:** Published or in-development marketplace package

---

## 1. Share vs. Private Tiers

Every paid package in the eStream Marketplace supports two pricing tiers. The tier model is implemented in `licensing/pricing_tiers.fl`.

### 1.1 How Tiers Work

| Tier | Customer Opt-in | Price | Telemetry |
|------|----------------|-------|-----------|
| **Share** | `telemetry share true` | Lower | Anonymized aggregates shared with publisher |
| **Private** | `telemetry share false` | Higher | Zero telemetry — nothing leaves the customer's lex scope |

Customers choose their tier via the lex setting `telemetry share true|false`. The choice is recorded as a `CustomerTelemetryPreference` with a blinded customer ID — the marketplace never learns which real customer made the choice.

### 1.2 Pricing the Gap

The price difference between Share and Private should reflect the genuine value of aggregate telemetry to you as a publisher:

```toml
[pricing]
license_type = "per-invocation"
share_price = "0.0004"     # $0.40 per 1,000 executions
private_price = "0.0006"   # $0.60 per 1,000 executions
```

**Guidelines:**

- Private tier should be 20–50% higher than Share tier
- The gap must be justifiable — it represents telemetry value, not a penalty
- Both tiers are legitimate and fully supported — no coercion or dark patterns
- Customers can switch tiers at any time; changes apply at the next billing period

### 1.3 Tier Changes

When a customer changes tiers, the `change_pricing_tier` circuit executes with PoVC attestation:

```
Old tier → New tier → Effective at next billing period
```

The `TierChangeRecord` is emitted to `pricing_tier_events` for audit purposes, using blinded customer IDs throughout.

---

## 2. Revenue-Share Model

### 2.1 Atomic Multi-Party Settlement

Every payment in the marketplace is settled atomically — all parties receive their share in a single transaction. The `execute_settlement` circuit in `licensing/settlement.fl` enforces the invariant:

```
publisher_bps + platform_bps + referrer_bps == 10,000 bps
```

Revenue shares are defined in basis points (1 bps = 0.01%):

### 2.2 Standard Revenue Splits

| Pricing Type | Publisher | Platform | Burn/Referrer |
|-------------|-----------|----------|---------------|
| One-Time | 85% (8,500 bps) | 10% (1,000 bps) | 5% (500 bps) |
| Subscription | 90% (9,000 bps) | 5% (500 bps) | 5% (500 bps) |
| Usage-Based | 85% (8,500 bps) | 10% (1,000 bps) | 5% (500 bps) |
| Enterprise | 80% (8,000 bps) | 15% (1,500 bps) | 5% (500 bps) |
| Freemium (premium) | 85% (8,500 bps) | 10% (1,000 bps) | 5% (500 bps) |

### 2.3 Settlement Flow

```
Customer → [Blinded Payment] → Settlement Circuit
                                    │
                          ┌─────────┼─────────┐
                          ▼         ▼         ▼
                     Publisher   Platform   Referrer
                      (85%)      (10%)      (5%)
                          │         │         │
                          ▼         ▼         ▼
                    [Witness-signed SettlementTransaction]
                    [PoVC attested, ML-DSA-87 signed]
```

Every settlement produces a `SettlementTransaction` with:
- Blinded customer ID (marketplace cannot correlate to real customer)
- Per-party payouts in microdollars
- ML-DSA-87 witness signature
- PoVC attestation

---

## 3. License Types

### 3.1 Per-Seat Licensing

Fixed price per authorized user. Best for team-based tools.

```toml
[pricing]
license_type = "per-seat"
share_price = "9.99"       # per seat per month
private_price = "14.99"
```

Use case: Console widgets, admin tools, dashboards where each team member needs access.

### 3.2 Per-Invocation Licensing

Pay per circuit execution. Best for variable workloads.

```toml
[pricing]
license_type = "per-invocation"
share_price = "0.0004"     # per execution
private_price = "0.0006"
```

Use case: Wire adapters (per-message), smart circuits (per-execution), data validators.

### 3.3 Per-Minute Licensing

Pay for compute time. Best for long-running processes.

```toml
[pricing]
license_type = "per-minute"
share_price = "0.02"       # per minute of compute
private_price = "0.03"
```

Use case: Streaming analytics, real-time monitoring, continuous aggregation circuits.

### 3.4 Metered Hourly Licensing

Hourly aggregated usage billing. The metering system in `licensing/metering.fl` records per-minute readings and aggregates them hourly:

```toml
[pricing]
license_type = "metered-hourly"
share_price = "1.20"       # per hour of active use
private_price = "1.80"
```

The `MeterReading` → `HourlyAggregate` → `BlindedBillingEvent` pipeline ensures:
- Per-minute granularity for accuracy
- Hourly aggregation for billing simplicity
- Blinded billing events for privacy

### 3.5 Flat-Rate Licensing

Fixed monthly or annual subscription regardless of usage.

```toml
[pricing]
license_type = "flat-rate"
share_price = "49.99"      # per month
private_price = "74.99"
```

Use case: Enterprise components, industrial gateways, full-stack integrations.

### 3.6 Free and Freemium

```toml
# Free
[pricing]
license_type = "free"

# Freemium — free base, paid premium features
[pricing]
license_type = "freemium"
share_price = "19.99"      # premium tier per month
private_price = "29.99"
free_features = ["basic-validation", "standard-parsing"]
premium_features = ["advanced-routing", "fpga-acceleration", "priority-support"]
```

### 3.7 License Type Comparison

| Type | Billing Unit | Predictability | Best For |
|------|-------------|---------------|----------|
| Per-Seat | User/month | High | Team tools, dashboards |
| Per-Invocation | Execution | Low | Variable workloads |
| Per-Minute | Compute minute | Medium | Streaming, long-running |
| Metered Hourly | Active hour | Medium | Continuous processes |
| Flat-Rate | Month/year | High | Enterprise, known workloads |
| Freemium | Feature tier | High | Adoption + conversion |

---

## 4. Telemetry Schema Definition

### 4.1 Defining Your Telemetry Schema

When customers choose the Share tier, they opt into anonymized telemetry. You must define exactly what is collected in the `[telemetry]` section of your manifest:

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

### 4.2 Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `schema_hash` | string | SHA3-256 of the telemetry schema; set `"auto"` for auto-computation |
| `metrics` | list | Named metrics collected from Share tier customers |
| `aggregate_window` | duration | Minimum aggregation period (must be >= `1h`) |
| `cohort_min_size` | integer | Minimum k for k-anonymity (must be >= 50) |
| `noise_budget` | string | Differential privacy epsilon budget (lower = more private) |

### 4.3 Privacy Requirements

The `validate_telemetry_schema` circuit enforces strict privacy constraints:

| Requirement | Enforcement |
|-------------|------------|
| **No PII** | `contains_pii` must be `false` — schemas containing personally identifiable information are rejected |
| **Anonymization required** | `anonymization_method` must be > 0 (k-Anonymity, Differential Privacy, or Aggregation) |
| **Minimum cohort size** | At least 50 customers in any aggregate cohort |
| **Noise budget** | Differentially private noise applied to all aggregate queries |
| **Hourly minimum** | Aggregate window must be >= 1 hour — no per-request visibility |

### 4.4 What You Receive (Share Tier)

As a publisher, Share tier telemetry gives you:

- **Aggregate invocation counts** per hour (not per customer)
- **Latency distributions** (percentiles, not individual measurements)
- **Error rates** as cohort averages
- **Usage patterns** (time-of-day, day-of-week) at cohort level

You never receive:
- Individual customer metrics
- Customer identity (blinded IDs only)
- Per-request data
- Cross-package usage correlation

---

## 5. Revenue Proofs

### 5.1 ZK Revenue Proofs

Publishers can prove their aggregate revenue for any time period without revealing individual transaction amounts. The `generate_revenue_proof` circuit produces a `ZkRevenueProof`:

```bash
estream marketplace revenue-proof --period 2026-02 --output proof.json
```

```json
{
  "proof_id": "a1b2c3...",
  "publisher_id": "my-org-id",
  "period_start": 1740000000,
  "period_end": 1742500000,
  "aggregate_revenue_micros": 1250000,
  "transaction_count": 3200,
  "proof_data": "...",
  "witness_signature": "..."
}
```

### 5.2 Verification

Anyone with the publisher's public key can verify a revenue proof:

```bash
estream marketplace revenue-proof verify proof.json --publisher my-org
```

This calls `verify_revenue_proof` which checks the ML-DSA-87 witness signature over the proof data. The proof attests:

- **Aggregate revenue** for the stated period
- **Transaction count** (not individual amounts)
- **Publisher identity** (bound to PRIME)

### 5.3 Use Cases

| Use Case | What the Proof Shows |
|----------|---------------------|
| Investor reporting | Aggregate MRR without leaking customer data |
| Creator program tier | Proof of revenue threshold for tier advancement |
| Partnership qualification | Revenue proof for partner program eligibility |
| Tax compliance | Auditable revenue attestation |

---

## 6. Solution Bundle Pricing

### 6.1 Revenue Waterfall

Solution bundles use a three-tier revenue waterfall: Platform → Solution → Publisher. The waterfall is configured in `solutions/revenue_waterfall.fl`:

```
Customer payment
    │
    ├── Platform share (e.g., 10%)
    ├── Solution share (e.g., 20%)
    └── Publisher shares (e.g., 70% split among package publishers)
```

### 6.2 Configuring the Waterfall

```toml
[solution.pricing]
model = "tiered"
platform_share_bps = 1000    # 10%
solution_share_bps = 2000    # 20%

# Publisher shares sum to remaining 7000 bps (70%)
[solution.pricing.publisher_shares]
"@my-org/core-engine" = 4000       # 40%
"@my-org/analytics" = 2000         # 20%
"@partner/connector" = 1000        # 10%
```

The `configure_waterfall` circuit enforces: `platform + solution + publishers == 10,000 bps`.

### 6.3 Waterfall Execution

Each customer payment flows through the `execute_waterfall` circuit atomically:

```
$100 customer payment:
  → Platform: $10 (1,000 bps)
  → Solution builder: $20 (2,000 bps)
  → @my-org/core-engine publisher: $40 (4,000 bps)
  → @my-org/analytics publisher: $20 (2,000 bps)
  → @partner/connector publisher: $10 (1,000 bps)
```

Every waterfall transaction is recorded with blinded customer IDs and PoVC attestation.

### 6.4 Waterfall Proofs

Solution builders can generate ZK proofs of waterfall execution to demonstrate correct revenue distribution to their package publishers:

```bash
estream marketplace waterfall-proof --solution my-solution --period 2026-02
```

---

## 7. Provider-Level Custom Pricing

### 7.1 Per-Tenant Deals

Publishers with platform approval can offer custom pricing to specific tenants. The `pricing/provider_pricing.fl` module supports:

```toml
[component.marketplace.pricing_tiers]
default_rate_micros = 1000
volume_discount_threshold = 100000
volume_discount_bps = 1500
allows_custom_deals = true
min_rate_micros = 100
revenue_share_bps = 7000
```

### 7.2 Deal Workflow

1. Publisher proposes a `TenantDeal` with custom rate and discount
2. Platform operator approves via `approve_tenant_deal` circuit
3. Deal activates between `valid_from` and `valid_until`
4. Usage is metered against the custom rate
5. Either party can revoke the deal

### 7.3 Volume Discounts

Automatic discounts when usage exceeds a threshold:

```
Usage < 100,000 units → $0.001/unit (base rate)
Usage ≥ 100,000 units → $0.00085/unit (15% discount)
```

---

## Next Steps

- [Publisher Getting Started](publisher-getting-started.md) — Set up your first package
- [Solution Builder Guide](solution-builder.md) — Bundle packages with waterfall pricing
- [Customer Guide](customer-guide.md) — How customers experience your pricing
- [Privacy Guarantees](../../specs/standards/PRIVACY_GUARANTEES_SPEC.md) — Formal privacy properties of the pricing system
- [Manifest Schema](../../specs/standards/MANIFEST_SCHEMA_SPEC.md) — Full `[pricing]` and `[telemetry]` field reference
