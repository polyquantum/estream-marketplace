# Component Marketplace Specification v0.8.0

> Component registry, payments, creator program, and source visibility model.

**Status:** Draft (target v0.8.0)
**Dependencies:** Component System (#309), Witness Framework (#314), Token Contracts (#326)

## Overview

The Component Marketplace is the "npm for verifiable circuits" - a platform for discovering, sharing, and monetizing ESCIR components.

---

## Source Visibility Philosophy

A key design decision: **what do we show for each component?**

### Options Considered

| Approach | Pros | Cons |
|----------|------|------|
| **Full ESCIR Source** | Transparency, auditability, learning | IP exposure, complexity for consumers |
| **Interface Only** | Clean API, protects IP | Black box, trust required |
| **Tiered Visibility** | Flexibility, creator control | Complexity |

### Recommended: Tiered Visibility

Components should support **multiple visibility levels**, controlled by the creator:

```rust
pub enum SourceVisibility {
    /// Full ESCIR source visible to all
    Open,
    
    /// Only interface (ports, annotations) visible
    /// Source hidden, but hash available for verification
    Interface,
    
    /// Interface + compiled artifacts (WASM/Verilog)
    /// Source hidden, artifacts inspectable
    Compiled,
    
    /// Full source visible only to licensees
    LicensedFull,
}
```

### What's Always Visible

Regardless of visibility level, these are **always public**:

```yaml
# Component Interface Card
name: payment-validator
version: 1.2.0
publisher: acme-circuits (verified)

# Ports (always visible - this IS the API)
inputs:
  - name: transaction
    type: Transaction
    description: "Incoming payment to validate"
  - name: rules
    type: RuleSet
    description: "Validation rules"

outputs:
  - name: result
    type: ValidationResult
    description: "Pass/fail with reasons"
  - name: audit_log
    type: AuditEntry
    description: "Compliance record"

# Resource Requirements (always visible - affects cost)
resources:
  witness_tier: 2
  compute_budget: 1000
  memory_bytes: 4096
  storage_bytes: 0
  estimated_cost_es: 0.003

# Hash for verification (always visible)
source_hash: sha256:abc123...
```

### What's Conditionally Visible

```yaml
# ESCIR Source (visibility: Open or LicensedFull)
nodes:
  - id: validate_amount
    type: transform
    # ... full implementation details

flows:
  - transaction -> validate_amount -> ...

# Compiled Artifacts (visibility: Compiled or Open)
artifacts:
  wasm_url: es://assets/...      # Native eStream asset storage
  verilog_url: es://assets/...   # Native eStream asset storage
```

### Rationale

1. **Interface is the API** - Consumers need to know inputs/outputs/resources to integrate. This is like a function signature.

2. **Resource requirements affect cost** - Users need to know `witness_tier`, memory, compute to estimate costs.

3. **Source hash enables verification** - Even if source is hidden, the hash proves the deployed code matches what was audited.

4. **IP protection is valid** - Some creators want to monetize implementations, not just charge for usage.

5. **Open source drives adoption** - Official eStream components should be fully open to serve as examples.

### Designer Integration

In the Circuit Designer, components appear as:

```
┌─────────────────────────────────────────────┐
│ payment-validator v1.2.0      [Certified]   │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────┐           ┌─────────────┐     │
│  │transaction│─────────▶│   result    │     │
│  └─────────┘           └─────────────┘     │
│  ┌─────────┐           ┌─────────────┐     │
│  │  rules   │─────────▶│  audit_log  │     │
│  └─────────┘           └─────────────┘     │
│                                             │
│  Resources: T2 witness, 1K compute, 4KB mem │
│  Cost: ~0.003 ES/exec                       │
│                                             │
│  [View Source] [View Docs] [Add to Circuit] │
│       ↑                                     │
│   Only if visibility allows                 │
└─────────────────────────────────────────────┘
```

### Creator Visibility Configuration

Creators control visibility during component publishing:

**CLI Configuration:**

```bash
# Publish with full source visibility
estream component publish --visibility open

# Publish with interface-only visibility (default)
estream component publish --visibility interface

# Publish with compiled artifacts visible
estream component publish --visibility compiled

# Publish with source visible only to licensees
estream component publish --visibility licensed
```

**Circuit Designer UI:**

When publishing from the visual designer, creators see:

```
┌──────────────────────────────────────────────────────────────┐
│  Publish Component                                            │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  Name: payment-validator                                      │
│  Version: 1.2.0                                               │
│                                                               │
│  ┌─ Source Visibility ────────────────────────────────────┐  │
│  │                                                         │  │
│  │  ○ Open                                                │  │
│  │    Full ESCIR source visible to everyone               │  │
│  │    Best for: Open source, reference implementations    │  │
│  │                                                         │  │
│  │  ● Interface Only                     [Recommended]     │  │
│  │    Ports and resource requirements visible             │  │
│  │    Source hidden, hash available for verification      │  │
│  │    Best for: Commercial components                     │  │
│  │                                                         │  │
│  │  ○ Compiled                                            │  │
│  │    Interface + WASM/Verilog artifacts visible          │  │
│  │    Source hidden, compiled code inspectable            │  │
│  │    Best for: Transparency with IP protection           │  │
│  │                                                         │  │
│  │  ○ Licensed Full                                       │  │
│  │    Full source visible only after license purchase     │  │
│  │    Best for: Premium SDKs, educational content         │  │
│  │                                                         │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                               │
│  ┌─ What's Always Public ─────────────────────────────────┐  │
│  │ ✓ Component name, version, publisher                   │  │
│  │ ✓ Input/output ports and types                         │  │
│  │ ✓ Resource requirements (witness tier, compute, memory)│  │
│  │ ✓ Estimated cost per execution                         │  │
│  │ ✓ Source hash (for verification)                       │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                               │
│  [Cancel]                            [Publish to Marketplace] │
└──────────────────────────────────────────────────────────────┘
```

**Implementation: ESF Filter**

Visibility is implemented using ESF Filter - the same field-level privacy primitive used throughout the platform. No separate visibility system needed.

```rust
/// Component published with ESF Filter for visibility control
pub struct PublishedComponent {
    /// Component identity (always public)
    pub id: ComponentId,
    pub name: String,
    pub version: SemVer,
    pub publisher: PublisherId,
    
    /// Component data with Filter applied
    pub data: FilteredComponentData,
}

/// ESF Filter applied to component fields
pub struct FilteredComponentData {
    /// Always visible (no filter)
    pub interface: ComponentInterface,
    pub resources: ResourceRequirements,
    pub source_hash: [u8; 32],
    
    /// Filtered fields - visibility controlled by audiences
    #[filter(audiences = ["public", "licensee"])]
    pub escir_source: Filtered<ESCIR>,
    
    #[filter(audiences = ["public", "compiled", "licensee"])]
    pub wasm_artifact: Filtered<AssetId>,
    
    #[filter(audiences = ["public", "compiled", "licensee"])]
    pub verilog_artifact: Filtered<AssetId>,
}

/// Visibility presets map to Filter audience grants
impl SourceVisibility {
    pub fn to_filter_grants(&self) -> Vec<AudienceGrant> {
        match self {
            // Open: everyone gets all audiences
            SourceVisibility::Open => vec![
                AudienceGrant::public("public"),
            ],
            
            // Interface: no one gets source/artifacts
            SourceVisibility::Interface => vec![],
            
            // Compiled: everyone gets compiled audience
            SourceVisibility::Compiled => vec![
                AudienceGrant::public("compiled"),
            ],
            
            // LicensedFull: licensees get full access
            SourceVisibility::LicensedFull => vec![
                AudienceGrant::conditional("licensee", |addr| {
                    has_license(addr)
                }),
            ],
        }
    }
}
```

**How It Works:**

```
┌────────────────────────────────────────────────────────────┐
│  Component: payment-validator v1.2.0                        │
│  Visibility: Compiled (creator setting)                     │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  Always Visible (unfiltered):                               │
│  ├─ name, version, publisher                                │
│  ├─ input/output ports                                      │
│  ├─ resource requirements                                   │
│  └─ source_hash                                             │
│                                                             │
│  Filtered by Audience:                                      │
│  ├─ escir_source    [audiences: public, licensee]          │
│  │   └─ Viewer has: compiled → ✗ HIDDEN                    │
│  ├─ wasm_artifact   [audiences: public, compiled, licensee]│
│  │   └─ Viewer has: compiled → ✓ VISIBLE                   │
│  └─ verilog_artifact [audiences: public, compiled, licensee]│
│      └─ Viewer has: compiled → ✓ VISIBLE                   │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

**Benefits of Using ESF Filter:**

1. **Single primitive** - No separate visibility system to maintain
2. **Familiar model** - Same as privacy masking for payloads
3. **Composable** - Can combine with other filters
4. **FPGA-accelerated** - Visibility checks use existing Filter hardware
5. **Audience flexibility** - Creators can define custom audiences beyond presets

**Visibility Change Policy:**

- Creators can **increase** visibility at any time (e.g., Interface → Open)
- Creators **cannot decrease** visibility after publishing (protects users who chose based on visibility)
- Version updates can have different visibility than previous versions

### Official Component Policy

eStream-published components should be:
- **Visibility: Open** - Full source available
- **Pricing: Free** - Drive adoption
- **Badges: Official, Certified** - Trust signal

This creates a reference implementation ecosystem while allowing third-party creators to build proprietary components on top.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  MARKETPLACE ARCHITECTURE                                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐           │
│  │  Frontend   │   │  API        │   │  Registry   │           │
│  │  (React)    │ ◀▶│  (Rust)     │ ◀▶│  (ESLite)   │           │
│  └─────────────┘   └─────────────┘   └─────────────┘           │
│         │                │                  │                   │
│         │                │                  │                   │
│         ▼                ▼                  ▼                   │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐           │
│  │  CDN        │   │  Payment    │   │  Storage    │           │
│  │  (Assets)   │   │  (Real-time)│   │  (ESLite)   │           │
│  └─────────────┘   └─────────────┘   └─────────────┘           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Registry

### Component Record

```rust
pub struct MarketplaceComponent {
    /// Unique component ID
    pub id: ComponentId,
    
    /// Component name
    pub name: String,
    
    /// Version (semver)
    pub version: semver::Version,
    
    /// Publisher
    pub publisher: Publisher,
    
    /// Description
    pub description: String,
    
    /// Long-form documentation
    pub readme: String,
    
    /// License
    pub license: License,
    
    /// Pricing model
    pub pricing: Pricing,
    
    /// Quality badges
    pub badges: Vec<Badge>,
    
    /// Categories/tags
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
    
    /// Dependencies
    pub dependencies: Vec<DependencyRef>,
    
    /// Statistics
    pub stats: ComponentStats,
    
    /// Storage references
    pub storage: StorageRefs,
    
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Publisher {
    pub id: PublisherId,
    pub name: String,
    pub verified: bool,
    pub avatar_url: Option<String>,
    pub reputation_score: f32,
}

pub struct ComponentStats {
    pub downloads: u64,
    pub active_installs: u64,
    pub stars: u64,
    pub reviews_count: u64,
    pub average_rating: f32,
    pub revenue_total: u64,
}

pub struct StorageRefs {
    /// ESCIR source (eStream native asset)
    pub escir_asset: AssetId,
    
    /// Compiled WASM (if applicable)
    pub wasm_tx: Option<String>,
    
    /// Compiled Verilog (if applicable)
    pub verilog_tx: Option<String>,
    
    /// Documentation assets
    pub docs_tx: Option<String>,
}
```

### Categories

```rust
pub enum Category {
    // Core
    Crypto,
    Identity,
    
    // Applications
    DeFi,
    Gaming,
    Social,
    
    // Enterprise
    Compliance,
    Analytics,
    
    // Infrastructure
    Networking,
    Storage,
    
    // Emerging
    AiMl,
    Iot,
    
    // Other
    Utility,
    Template,
}
```

---

## Pricing Models

### Pricing Types

```rust
pub enum Pricing {
    /// Completely free
    Free,
    
    /// One-time purchase
    OneTime { price_es: u64 },
    
    /// Monthly subscription
    Subscription { 
        monthly_es: u64,
        annual_discount_pct: u8,
    },
    
    /// Pay per use
    UsageBased {
        per_execution_es: Fixed64,
        free_tier_executions: u64,
    },
    
    /// Contact for pricing
    Enterprise,
    
    /// Freemium (basic free, premium paid)
    Freemium {
        free_features: Vec<String>,
        premium_price_es: u64,
        premium_features: Vec<String>,
    },
}

impl Pricing {
    pub fn display_price(&self) -> String {
        match self {
            Pricing::Free => "Free".to_string(),
            Pricing::OneTime { price_es } => format!("{} ES", price_es),
            Pricing::Subscription { monthly_es, .. } => format!("{} ES/mo", monthly_es),
            Pricing::UsageBased { per_execution_es, .. } => {
                format!("{} ES/exec", per_execution_es)
            }
            Pricing::Enterprise => "Contact".to_string(),
            Pricing::Freemium { premium_price_es, .. } => {
                format!("Free / {} ES", premium_price_es)
            }
        }
    }
}
```

### Revenue Split

```rust
pub struct RevenueSplit {
    /// Creator share (%)
    pub creator_pct: u8,
    
    /// Platform share (%)
    pub platform_pct: u8,
    
    /// Burn share (%)
    pub burn_pct: u8,
}

impl RevenueSplit {
    pub fn for_pricing(pricing: &Pricing) -> Self {
        match pricing {
            Pricing::Free => Self { creator_pct: 0, platform_pct: 0, burn_pct: 0 },
            Pricing::OneTime { .. } => Self { creator_pct: 85, platform_pct: 10, burn_pct: 5 },
            Pricing::Subscription { .. } => Self { creator_pct: 90, platform_pct: 5, burn_pct: 5 },
            Pricing::UsageBased { .. } => Self { creator_pct: 85, platform_pct: 10, burn_pct: 5 },
            Pricing::Enterprise => Self { creator_pct: 80, platform_pct: 15, burn_pct: 5 },
            Pricing::Freemium { .. } => Self { creator_pct: 85, platform_pct: 10, burn_pct: 5 },
        }
    }
    
    pub fn calculate(&self, revenue: u64) -> RevenueDistribution {
        RevenueDistribution {
            creator: revenue * self.creator_pct as u64 / 100,
            platform: revenue * self.platform_pct as u64 / 100,
            burn: revenue * self.burn_pct as u64 / 100,
        }
    }
}
```

---

## Quality Badges

### Badge Types

```rust
pub enum Badge {
    /// Identity verified
    Verified,
    
    /// Automated tests pass
    Tested { 
        test_count: u32, 
        coverage_pct: u8 
    },
    
    /// Third-party security audit
    Audited { 
        auditor: String, 
        report_url: String,
        date: DateTime<Utc>,
    },
    
    /// eStream team certified
    Certified {
        certification_level: CertificationLevel,
        expires: DateTime<Utc>,
    },
    
    /// Official eStream component
    Official,
    
    /// Community favorite
    CommunityChoice { year: u16 },
    
    /// High performance
    HighPerformance { 
        benchmark_score: u32 
    },
    
    /// Post-quantum certified
    PostQuantum,
}

#[derive(Clone, Copy)]
pub enum CertificationLevel {
    Bronze,   // Basic review
    Silver,   // Thorough review
    Gold,     // Full audit + ongoing monitoring
}
```

### Earning Badges

| Badge | Requirements | Cost |
|-------|--------------|------|
| **Verified** | KYC/identity verification | Free |
| **Tested** | CI/CD with >80% coverage | Free |
| **Audited** | Third-party audit report | Audit cost |
| **Certified** | eStream team review + ongoing | 500 ES/year |
| **Official** | Built by eStream team | N/A |

---

## Reviews and Ratings

### Review Structure

```rust
pub struct Review {
    pub id: ReviewId,
    pub component_id: ComponentId,
    pub reviewer: Publisher,
    pub rating: u8,  // 1-5
    pub title: String,
    pub body: String,
    pub helpful_count: u32,
    pub verified_purchase: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ReviewStats {
    pub total_reviews: u32,
    pub average_rating: f32,
    pub rating_distribution: [u32; 5],  // 1-5 star counts
}
```

### Rating Calculation

```rust
impl ComponentStats {
    pub fn calculate_score(&self) -> f32 {
        // Weighted score considering multiple factors
        let rating_weight = 0.4;
        let popularity_weight = 0.3;
        let freshness_weight = 0.2;
        let quality_weight = 0.1;
        
        let rating_score = self.average_rating / 5.0;
        let popularity_score = (self.downloads as f32).log10() / 6.0;  // Max ~1M
        let freshness_score = 1.0;  // Based on update recency
        let quality_score = self.badge_score();
        
        rating_weight * rating_score
            + popularity_weight * popularity_score
            + freshness_weight * freshness_score
            + quality_weight * quality_score
    }
}
```

---

## Discovery

### Search API

```rust
pub struct SearchQuery {
    /// Text query
    pub query: Option<String>,
    
    /// Category filter
    pub categories: Vec<Category>,
    
    /// Tag filter
    pub tags: Vec<String>,
    
    /// Pricing filter
    pub pricing_type: Option<PricingType>,
    
    /// Badge filter
    pub required_badges: Vec<BadgeType>,
    
    /// Sort order
    pub sort: SortOrder,
    
    /// Pagination
    pub offset: u32,
    pub limit: u32,
}

pub enum SortOrder {
    Relevance,
    Downloads,
    Rating,
    Recent,
    Trending,
}

pub struct SearchResult {
    pub components: Vec<ComponentSummary>,
    pub total_count: u64,
    pub facets: SearchFacets,
}

pub struct SearchFacets {
    pub categories: HashMap<Category, u32>,
    pub pricing_types: HashMap<PricingType, u32>,
    pub badges: HashMap<BadgeType, u32>,
}
```

### Recommendation Engine

```rust
pub struct Recommendations {
    /// Similar to components you've used
    pub similar: Vec<ComponentSummary>,
    
    /// Frequently used together
    pub also_used: Vec<ComponentSummary>,
    
    /// Popular in your categories
    pub popular: Vec<ComponentSummary>,
    
    /// New and trending
    pub trending: Vec<ComponentSummary>,
}

impl RecommendationEngine {
    pub fn for_user(&self, user: &UserId) -> Recommendations {
        let user_history = self.get_user_history(user);
        let user_categories = self.extract_categories(&user_history);
        
        Recommendations {
            similar: self.find_similar(&user_history),
            also_used: self.find_co_used(&user_history),
            popular: self.popular_in_categories(&user_categories),
            trending: self.trending_overall(),
        }
    }
}
```

---

## Real-Time Payments

### Design Principle

Payments happen **in real-time, in parallel with execution**, not as separate transactions. This follows the same pattern as 8D resource metering - we don't touch transactions multiple times.

**Reference:** [US20190250941A1 - FPGA Platform as a Service](https://patents.google.com/patent/US20190250941A1)
> "In parallel with processing the first digital bit stream of data through the plurality of circuits, generating a usage value indicative of execution of at least one of the plurality of circuits consuming the first digital bit stream"

### Real-Time Metering Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  REAL-TIME PAYMENT (Single-Touch)                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Circuit Execution                                          ││
│  │  ─────────────────────────────────────────────────────────  ││
│  │                                                              ││
│  │  [Input] ──▶ [Component A] ──▶ [Component B] ──▶ [Output]   ││
│  │                   │                  │                       ││
│  │                   ▼                  ▼                       ││
│  │  ┌──────────────────────────────────────────────────────┐   ││
│  │  │  Metering Circuit (parallel, zero overhead)          │   ││
│  │  │  ─────────────────────────────────────────────────── │   ││
│  │  │  Component A: 0.002 ES  │  Component B: 0.001 ES     │   ││
│  │  │  Witness:     0.001 ES  │  Platform:   0.0004 ES     │   ││
│  │  │  ─────────────────────────────────────────────────── │   ││
│  │  │  Total:       0.0044 ES │  Split: 85/10/5            │   ││
│  │  └──────────────────────────────────────────────────────┘   ││
│  │                              │                               ││
│  └──────────────────────────────┼───────────────────────────────┘│
│                                 ▼                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Single Atomic Settlement (end of execution)                ││
│  │  ───────────────────────────────────────────────────────────││
│  │  Caller balance:  -0.0044 ES                                ││
│  │  Creator A:       +0.0017 ES (85% of 0.002)                 ││
│  │  Creator B:       +0.00085 ES (85% of 0.001)                ││
│  │  Witness:         +0.001 ES                                 ││
│  │  Platform:        +0.00044 ES (10%)                         ││
│  │  Burn:            +0.00022 ES (5%)                          ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Key Properties

1. **Zero Overhead** - Metering runs in parallel with execution (FPGA circuit)
2. **Single Touch** - One atomic settlement per execution, not per component
3. **Real-Time** - No batch processing, no delayed billing
4. **Transparent** - Caller sees exact cost before execution
5. **Composable** - Nested components aggregate naturally

### Metering Integration

```rust
/// Metering happens during execution, not after
pub struct ExecutionMetering {
    /// Component usage counters (updated in parallel)
    component_usage: HashMap<ComponentId, UsageCount>,
    
    /// Resource dimensions (8D)
    resources: ResourceMetrics,
    
    /// Accumulated cost
    total_cost_es: Fixed64,
    
    /// Revenue split pending
    pending_splits: Vec<RevenueSplit>,
}

/// At execution end, single atomic settlement
impl ExecutionMetering {
    pub fn settle(&self) -> SettlementResult {
        // One transaction, all parties
        self.ledger.atomic_transfer(vec![
            Transfer::debit(self.caller, self.total_cost_es),
            Transfer::credit_each(&self.creator_shares),
            Transfer::credit(self.witness, self.witness_fee),
            Transfer::credit(PLATFORM, self.platform_fee),
            Transfer::burn(self.burn_amount),
        ])
    }
}
```

### Cost Estimation (Pre-Execution)

```rust
/// Estimate before execution (from component metadata)
pub fn estimate_cost(circuit: &ESCIR) -> CostEstimate {
    let mut total = Fixed64::ZERO;
    
    for component in circuit.components() {
        total += component.estimated_cost_per_exec();
    }
    
    total += circuit.witness_tier().cost();
    total += platform_fee(total);
    
    CostEstimate {
        min: total * 0.9,  // Conservative
        max: total * 1.1,  // With variance
        expected: total,
    }
}
```

### License Management

```rust
pub struct License {
    pub id: LicenseId,
    pub component_id: ComponentId,
    pub licensee: Address,
    pub license_type: LicenseType,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub usage_limit: Option<u64>,
    pub usage_count: u64,
}

pub enum LicenseType {
    /// Perpetual access
    Perpetual,
    
    /// Time-limited
    Subscription { 
        period: SubscriptionPeriod 
    },
    
    /// Usage-limited
    UsageBased { 
        max_executions: u64 
    },
    
    /// Trial
    Trial { 
        duration_days: u8 
    },
}

impl License {
    pub fn is_valid(&self) -> bool {
        if let Some(expires) = self.expires_at {
            if Utc::now() > expires {
                return false;
            }
        }
        
        if let Some(limit) = self.usage_limit {
            if self.usage_count >= limit {
                return false;
            }
        }
        
        true
    }
}
```

---

## Creator Program

### Creator Tiers

```rust
pub enum CreatorTier {
    /// Just starting
    Starter,
    
    /// Some traction
    Growing {
        monthly_revenue: u64,
        total_downloads: u64,
    },
    
    /// Significant presence
    Established {
        monthly_revenue: u64,
        verified: bool,
    },
    
    /// Top creators
    Partner {
        monthly_revenue: u64,
        dedicated_support: bool,
    },
}

impl CreatorTier {
    pub fn from_stats(stats: &CreatorStats) -> Self {
        if stats.monthly_revenue >= 10_000 && stats.is_partner {
            CreatorTier::Partner {
                monthly_revenue: stats.monthly_revenue,
                dedicated_support: true,
            }
        } else if stats.monthly_revenue >= 1_000 || stats.total_downloads >= 10_000 {
            CreatorTier::Established {
                monthly_revenue: stats.monthly_revenue,
                verified: stats.is_verified,
            }
        } else if stats.monthly_revenue >= 100 || stats.total_downloads >= 1_000 {
            CreatorTier::Growing {
                monthly_revenue: stats.monthly_revenue,
                total_downloads: stats.total_downloads,
            }
        } else {
            CreatorTier::Starter
        }
    }
    
    pub fn benefits(&self) -> Vec<&'static str> {
        match self {
            CreatorTier::Starter => vec![
                "Basic analytics",
                "Community support",
            ],
            CreatorTier::Growing { .. } => vec![
                "Detailed analytics",
                "Email support",
                "Featured in category",
            ],
            CreatorTier::Established { .. } => vec![
                "Advanced analytics",
                "Priority support",
                "Homepage featuring",
                "Early access to features",
            ],
            CreatorTier::Partner { .. } => vec![
                "Full analytics suite",
                "Dedicated account manager",
                "Co-marketing opportunities",
                "Revenue share negotiation",
                "Roadmap input",
            ],
        }
    }
}
```

### Creator Dashboard

```
┌─────────────────────────────────────────────────────────────────┐
│  CREATOR DASHBOARD                                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Revenue This Month        Downloads This Week    Rating        │
│  ┌───────────────────┐    ┌─────────────────┐   ┌───────────┐  │
│  │   4,523 ES        │    │     2,341       │   │  ★★★★☆    │  │
│  │   +12% vs last    │    │   +23% vs last  │   │  4.2/5.0  │  │
│  └───────────────────┘    └─────────────────┘   └───────────┘  │
│                                                                  │
│  REVENUE BREAKDOWN                                               │
│  ─────────────────────────────────────────────────────────────  │
│                                                                  │
│  ████████████████████████████████░░░░░░░░  Payment Validator    │
│  2,341 ES (52%)                                                 │
│                                                                  │
│  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  Token Bridge         │
│  1,234 ES (27%)                                                 │
│                                                                  │
│  ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  KYC Attestor         │
│  948 ES (21%)                                                   │
│                                                                  │
│  PENDING PAYOUTS                                                 │
│  ─────────────────────────────────────────────────────────────  │
│  4,523 ES available for withdrawal                              │
│  [Withdraw to Wallet]                                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Publishing Flow

### Publish API

```rust
pub struct PublishRequest {
    /// Component metadata
    pub name: String,
    pub version: semver::Version,
    pub description: String,
    pub readme: String,
    pub license: License,
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
    
    /// Pricing
    pub pricing: Pricing,
    
    /// Source code
    pub escir_source: Vec<u8>,
    
    /// Optional compiled artifacts
    pub wasm_binary: Option<Vec<u8>>,
    pub verilog_source: Option<String>,
    
    /// Dependencies
    pub dependencies: Vec<DependencyRef>,
}

pub struct PublishResult {
    pub component_id: ComponentId,
    pub version: semver::Version,
    pub storage_tx: String,
    pub publish_fee: u64,
}

impl MarketplaceService {
    pub async fn publish(&self, req: PublishRequest, publisher: &Publisher) -> Result<PublishResult, PublishError> {
        // 1. Validate ESCIR
        self.validator.validate_escir(&req.escir_source)?;
        
        // 2. Check version increment
        self.check_version(&req.name, &req.version, publisher)?;
        
        // 3. Upload to permanent storage
        let storage_tx = self.storage.upload(&req.escir_source).await?;
        
        // 4. Charge publish fee
        let fee = self.calculate_publish_fee(&req);
        self.payment.charge(publisher, fee)?;
        
        // 5. Create registry entry
        let component = self.registry.create(req, publisher, &storage_tx)?;
        
        // 6. Index for search
        self.search.index(&component).await?;
        
        Ok(PublishResult {
            component_id: component.id,
            version: component.version,
            storage_tx,
            publish_fee: fee,
        })
    }
}
```

### Version Management

```rust
impl MarketplaceService {
    pub fn check_version(
        &self, 
        name: &str, 
        new_version: &semver::Version,
        publisher: &Publisher,
    ) -> Result<(), VersionError> {
        // Get existing versions
        let existing = self.registry.get_versions(name)?;
        
        // Check ownership
        if let Some(latest) = existing.last() {
            if latest.publisher.id != publisher.id {
                return Err(VersionError::NotOwner);
            }
            
            // Ensure version increment
            if new_version <= &latest.version {
                return Err(VersionError::MustIncrement {
                    current: latest.version.clone(),
                    proposed: new_version.clone(),
                });
            }
        }
        
        Ok(())
    }
}
```

---

## API Endpoints

### REST API

```
GET  /v1/components                      # List/search components
GET  /v1/components/:id                  # Get component details
GET  /v1/components/:id/versions         # List versions
GET  /v1/components/:id/reviews          # List reviews

POST /v1/components                      # Publish component
PUT  /v1/components/:id                  # Update metadata

POST /v1/components/:id/purchase         # Purchase/subscribe
GET  /v1/licenses                        # List user licenses
POST /v1/licenses/:id/validate           # Validate license

GET  /v1/creators/dashboard              # Creator dashboard
GET  /v1/creators/payouts                # Payout history
POST /v1/creators/withdraw               # Withdraw earnings

GET  /v1/discover/featured               # Featured components
GET  /v1/discover/trending               # Trending components
GET  /v1/discover/recommendations        # Personalized recommendations
```

### GraphQL API

```graphql
type Query {
  component(id: ID!): Component
  components(filter: ComponentFilter, pagination: Pagination): ComponentConnection
  search(query: String!, filter: SearchFilter): SearchResult
  recommendations: Recommendations
  creatorDashboard: CreatorDashboard
}

type Mutation {
  publishComponent(input: PublishInput!): PublishResult!
  purchaseComponent(componentId: ID!, paymentMethod: PaymentMethod!): License!
  submitReview(componentId: ID!, review: ReviewInput!): Review!
  withdrawEarnings(amount: BigInt!): Withdrawal!
}

type Component {
  id: ID!
  name: String!
  version: String!
  publisher: Publisher!
  description: String!
  pricing: Pricing!
  stats: ComponentStats!
  badges: [Badge!]!
  reviews(first: Int): ReviewConnection!
}
```

---

## Moderation

### Content Policy

```rust
pub struct ModerationPolicy {
    /// Automated checks
    pub automated_checks: Vec<AutomatedCheck>,
    
    /// Manual review triggers
    pub review_triggers: Vec<ReviewTrigger>,
    
    /// Prohibited content
    pub prohibited: Vec<ContentCategory>,
}

pub enum AutomatedCheck {
    MalwareScanning,
    LicenseCompliance,
    DependencyVulnerabilities,
    CodeQuality,
}

pub enum ReviewTrigger {
    FirstPublish,
    SignificantUpdate,
    ReportedContent,
    HighRiskCategory,
}

pub enum ContentCategory {
    Malware,
    Phishing,
    IllegalContent,
    CopyrightViolation,
    Spam,
}
```

### Dispute Resolution

```rust
pub struct Dispute {
    pub id: DisputeId,
    pub component_id: ComponentId,
    pub reporter: Address,
    pub reason: DisputeReason,
    pub evidence: Vec<String>,
    pub status: DisputeStatus,
    pub resolution: Option<Resolution>,
}

pub enum DisputeReason {
    CopyrightClaim,
    SecurityVulnerability,
    FalseAdvertising,
    LicenseViolation,
    Other(String),
}

pub enum DisputeStatus {
    Open,
    UnderReview,
    AwaitingResponse,
    Resolved,
    Escalated,
}
```

---

## References

- [COMPONENT_SYSTEM_SPEC.md](../protocol/COMPONENT_SYSTEM_SPEC.md)
- [TOKENOMICS_SPEC.md](../economics/TOKENOMICS_SPEC.md)
- [STRATEGIC_ROADMAP.md](../../docs/STRATEGIC_ROADMAP.md)
