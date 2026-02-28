# Opt-In Engagement Model Specification

| Field | Value |
|-------|-------|
| **Version** | v0.1.0 |
| **Status** | Draft |
| **Tier** | Cloud |
| **Lex Namespace** | `esn/marketplace/advertising/optin` |
| **Marketplace Component** | `estream-marketplace/advertising/optin/` |
| **Patent Lineage** | US20120166262A1, US20130117084A1, US20130126605A1, US20170249608A1 |

---

## 1. Patent Evolution

The opt-in engagement model descends from four patent applications that collectively define a new paradigm for partner-user engagement where the user is the active decision-maker, not a passive acceptor of terms.

### 1.1 US20120166262A1 — Opt-In Engagement System

The foundational patent. Introduces the core concepts:
- **Activity scoring**: Users build composite scores across authentication, engagement, and sharing dimensions. Higher scores signal higher-quality leads.
- **Engagement codes**: Partners create offers; users receive codes; activation constitutes explicit opt-in.
- **Compensation events**: Users receive value (priority support, early access, affiliate credit, charitable donation) in exchange for intentional data sharing.

### 1.2 US20130117084A1 — Escrow Reservation Model

Extends the engagement system with financial guarantees:
- **Pre-deposited budgets**: Partners deposit funds before creating offers, eliminating payment risk for users and the platform.
- **Per-engagement reservation**: When a code is generated, funds are reserved against the partner's escrow account. Reserved funds cannot be double-spent.
- **Expiration recovery**: Unused reservations expire back to the partner's available balance after a configurable timeout.

### 1.3 US20130126605A1 — View Record Data Rules

Defines granular data sharing controls:
- **Field-level visibility bitmask**: Each engagement specifies exactly which user data fields the partner can see.
- **Redaction enforcement**: Fields not in the visibility bitmask are cryptographically redacted — not merely hidden, but provably absent from the partner's view.
- **Transform rules**: Some fields can be shared in degraded form (metro area instead of ZIP, age range instead of birthdate, boolean presence instead of exact value).

### 1.4 US20170249608A1 — Multi-Party Settlement

Completes the economic model:
- **Deterministic distribution**: Settlement splits are computed from immutable rules (basis points per party), producing identical outputs for identical inputs.
- **Four-party split**: Every settlement distributes to platform (fee), user (compensation), affiliate (referral share), and publisher (content host share).
- **Cryptographic attestation**: Settlement records are Groth16-attested, allowing any party to independently verify correctness.

---

## 2. Key Innovation: Active Opt-In vs Passive Consent

Traditional advertising consent is passive: users click "I agree" on terms-and-conditions they never read, granting blanket data access. The opt-in model inverts this:

| Dimension | Traditional T&C | Opt-In Model |
|-----------|----------------|--------------|
| **User action** | Click "agree" once | Activate specific engagement code per partner |
| **Data scope** | Blanket access to profile | Per-engagement field-level bitmask |
| **Compensation** | None | User receives value per engagement |
| **Revocation** | Buried in settings | One-action revoke with immediate effect |
| **Consent proof** | Checkbox timestamp | ML-DSA-87 signed, DAG-chained, Groth16-attested |
| **Partner trust** | Self-reported compliance | Cryptographic enforcement — redacted fields never leak |
| **Budget model** | Pay-per-impression (speculative) | Escrow-funded (guaranteed) |

The core insight: when users actively choose to engage, conversion rates rise dramatically (10-15% vs 3-5% industry average) because every lead is intentional. Partners accept the higher per-engagement cost because the ROI is superior to shotgun advertising.

---

## 3. Activity Score Model

Activity scores quantify user quality across three orthogonal dimensions. Partners use composite scores to target offers at high-value users who are likely to complete engagements.

### 3.1 Dimensions

| Dimension | Range | Signals |
|-----------|-------|---------|
| **Authentication** | 0-100 | API verification, identity proofs, multi-factor authentication, KYC completion |
| **Engagement** | 0-100 | Form completions, questions asked, documents downloaded, surveys completed |
| **Sharing** | 0-100 | Referrals made, forum posts, community participation, content contributions |

### 3.2 Composite Score

The composite score is a weighted aggregate of all three dimensions:

```
composite = w_auth * authentication + w_eng * engagement + w_share * sharing
```

Weights are optimized by the Cortex AI `optin_score_optimizer` model, which learns from engagement completion rates. Default weights: authentication 0.40, engagement 0.35, sharing 0.25.

### 3.3 Score Events

Every user action that contributes to a score dimension is recorded as a `ScoreEvent` with PoVC attestation. Events are typed:
- `ApiVerification` — user completed API-based identity verification
- `FormCompletion` — user completed a partner intake form
- `QuestionAsked` — user asked a question in a partner context
- `DocumentDownload` — user downloaded a document or resource
- `ReferralMade` — user referred another user
- `ForumPost` — user contributed to a community forum
- `SurveyCompleted` — user completed a feedback survey

### 3.4 Score Recomputation

Scores are recomputed on each new event. The AI model provides optimized weights and flags anomalous score changes (e.g., sudden jumps from bot activity). Scores are retained for 365 days as time-series data for trend analysis.

---

## 4. Engagement Code Flow

The engagement code flow is the central interaction pattern. It ensures that data sharing never happens without explicit, specific, signed consent.

### 4.1 Lifecycle

```
Partner creates offer
    ↓
Platform generates EngagementCode (status: Generated)
    ↓ funds reserved from escrow
User sees offer in their dashboard (status: Viewed)
    ↓
User activates code — THE opt-in moment (status: Activated)
    ↓ consent signed (ML-DSA-87), ConsentProof created (DAG node)
ViewRecord generated from consent — partner sees approved fields only
    ↓
Engagement completes — settlement distributes funds
    ↓
EngagementCode reaches terminal state
```

### 4.2 Consent Activation

Code activation is the cryptographic opt-in moment:
1. User reviews the offer: what data is shared, with whom, for what compensation.
2. User signs the engagement code with their ML-DSA-87 private key.
3. A `ConsentProof` is created as an immutable DAG node, linking to the engagement code and specifying exactly which fields are consented.
4. The consent proof includes a `provenance_hop_id` linking to the Thread provenance chain for full attribution.
5. The circuit runs in constant-time mode to prevent side-channel leakage of the consent decision.

### 4.3 Revocation

Users can revoke consent at any time. Revocation:
- Sets the engagement code status to `Revoked`
- Does not delete the consent proof (immutable audit trail)
- Prevents further data access via the view record
- Does not claw back already-settled funds (settlement is final)

### 4.4 Engagement Code Status Machine

```
Generated → Viewed → Activated → (terminal)
    ↓                    ↓
  Expired             Revoked
```

---

## 5. Escrow Model

The escrow model provides budget certainty for all parties. Partners cannot create offers without sufficient funds, and users are guaranteed compensation upon completion.

### 5.1 Account Lifecycle

```
Partner deposits funds → ReservationAccount created
    ↓
Code generated → Reservation created (Reserved)
    ↓
Engagement completes → Reservation released (Released)
    OR
Engagement expires → Reservation cancelled (Expired), funds return to available
```

### 5.2 Balance Invariants

At all times:
```
deposited_amount >= reserved_amount + released_amount + expired_amount
available = deposited_amount - reserved_amount - released_amount - expired_amount
```

The `optin_reserve_against` circuit enforces `available >= amount` before creating any reservation. This prevents over-commitment and guarantees that every active engagement code has a funded backing.

### 5.3 Currency Support

Escrow accounts are denominated in a single currency. Multi-currency partners maintain separate accounts per currency. Currency conversion is out of scope for this component.

---

## 6. View Record Model

View records are the data-sharing enforcement layer. They translate consent proofs into concrete field-level access control.

### 6.1 Field Visibility

Each view record contains:
- **visible_fields**: 32-byte bitmask — which fields the partner can see
- **redacted_fields**: 32-byte bitmask — which fields are cryptographically redacted
- **transformed_fields**: list of field/transform pairs for degraded sharing

The invariant `visible_fields ⊆ consent.consented_fields` is enforced at generation time. Partners cannot see fields the user did not consent to, even if they request them.

### 6.2 Transform Types

| Transform | Description | Example |
|-----------|-------------|---------|
| `ExactValue` | Full field value shared | "San Francisco, CA 94102" |
| `MetroAreaOnly` | Geographic degradation | "San Francisco Metro" |
| `AgeRange` | Age band instead of birthdate | "25-34" |
| `BooleanOnly` | Presence/absence only | "true" (has phone number) |

### 6.3 View Enforcement

The `optin_enforce_view` circuit runs in constant-time mode and enforces:
1. Only visible fields are included in the output
2. Redacted fields are provably absent (not just zeroed — absent from the output buffer)
3. Transformed fields are degraded according to their transform type
4. Every access is logged as a `ViewAccess` record with PoVC attestation

### 6.4 Access Audit

Every partner access to a view record is logged with:
- Which fields were requested
- Which fields were actually returned
- Timestamp and IP hash
- PoVC attestation for tamper-evidence

---

## 7. Settlement Model

Multi-party settlement distributes funds from escrow according to configurable rules. Settlement is deterministic — same inputs always produce identical outputs — and Groth16-attested for independent verification.

### 7.1 Distribution Parties

| Party | Description | Typical Share |
|-------|-------------|---------------|
| **Platform** | eStream platform fee | 15-25% |
| **User** | User compensation for data sharing | 30-50% |
| **Affiliate** | Referral credit to user who referred the engaging user | 10-20% |
| **Publisher** | Content host where engagement originated | 10-20% |

### 7.2 Settlement Rules

Rules are defined as basis points (1 bps = 0.01%). The invariant `platform + user + affiliate + publisher == 10000` is enforced at rule creation time.

Example configuration:
```
platform_fee_bps     = 2000  (20%)
user_compensation_bps = 4000  (40%)
affiliate_share_bps   = 2000  (20%)
publisher_share_bps   = 2000  (20%)
                        -----
                       10000  (100%)
```

### 7.3 Settlement Execution

Settlement requires 2-of-3 witness attestation (multi-party threshold signature):
1. Validate the engagement code is in Activated state
2. Validate the reservation is in Reserved state
3. Compute per-party amounts from the reservation amount and settlement rules
4. Verify no over-distribution (sum of distributions <= reservation amount)
5. Record the settlement as an immutable DAG node with Groth16 attestation
6. Release the reservation

### 7.4 Settlement Finality

Once settled, distributions are final. Consent revocation after settlement does not trigger clawbacks. This is by design — the partner received data access and the user received compensation; reversing one without the other would create asymmetric risk.

---

## 8. Why Partners Accept This Model

| Concern | Answer |
|---------|--------|
| **Higher per-lead cost** | 10-15% conversion rate vs 3-5% — ROI is 2-3x higher per dollar spent |
| **Escrow lock-up** | Unused funds expire back to available balance; no permanent lock |
| **Limited data fields** | Every shared field is user-approved — zero compliance risk, zero GDPR/CCPA exposure |
| **Settlement complexity** | Deterministic, auditable, Groth16-attested — no billing disputes |
| **User revocation** | Revocation rate is < 5% because users opted in intentionally |

---

## 9. Circuit Cross-References

| Circuit | File | Purpose |
|---------|------|---------|
| `optin_compute_score` | `circuits/optin_profile.fl` | Aggregate score events into activity dimensions |
| `optin_record_event` | `circuits/optin_profile.fl` | Record a scoring event with PoVC attestation |
| `optin_get_score` | `circuits/optin_profile.fl` | Look up current activity score |
| `optin_generate_code` | `circuits/optin_engage.fl` | Create engagement code with escrow link |
| `optin_activate_code` | `circuits/optin_engage.fl` | User activates code — the opt-in moment |
| `optin_revoke_code` | `circuits/optin_engage.fl` | User revokes consent |
| `optin_deposit_escrow` | `circuits/optin_escrow.fl` | Partner deposits budget |
| `optin_reserve_against` | `circuits/optin_escrow.fl` | Reserve funds for engagement |
| `optin_release_on_completion` | `circuits/optin_escrow.fl` | Release reserved funds on completion |
| `optin_expire_reservation` | `circuits/optin_escrow.fl` | Expire timed-out reservation |
| `optin_generate_view` | `circuits/optin_view.fl` | Generate view record from consent proof |
| `optin_enforce_view` | `circuits/optin_view.fl` | Enforce field visibility and redaction |
| `optin_log_access` | `circuits/optin_view.fl` | Audit log every partner access |
| `optin_settle_engagement` | `circuits/optin_settle.fl` | Distribute funds per settlement rules |
| `optin_verify_settlement` | `circuits/optin_settle.fl` | Verify settlement correctness |

---

## 10. Dependencies

| Dependency | Usage |
|------------|-------|
| `estream-crm ^0.1.0` | User profiles, relationships, lead pipeline — activity scores link to CRM profiles via graph edges |
| `estream-thread ^0.1.0` | Provenance chain — consent proofs link to Thread hops for full engagement attribution |
