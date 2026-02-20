# eStream Marketplace Specification

> Canonical specification for the eStream component marketplace — discovery, packaging, signing, pricing, and developer tooling for SmartCircuit components.

**Status:** Draft
**Version:** 2.0.0
**Epic:** [polyquantum/estream#136](https://github.com/polyquantum/estream/issues/136)
**Supersedes:** MARKETPLACE_SPEC.md (v0.8.0), COMPONENT_REGISTRY_API_SPEC.md (v1.0.0), SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md (v1.0.0), CONSOLE_WIDGET_MARKETPLACE_SPEC.md (v1.0.0), FPGA_COMPONENT_EXTENSION.md

---

## 1. Overview

The eStream Marketplace is an open source component exchange — "npm for verifiable circuits" — enabling developers to discover, publish, install, and compose reusable eStream components: data schemas, SmartCircuits, protocol adapters, FPGA circuits, console widgets, and full-stack integrations.

### 1.1 Design Principles

1. **Stream API, not HTTP** — All marketplace data flows over eStream's native lattice streams (WebTransport/QUIC binary wire protocol), defined in FastLang. See §5.
2. **No trusted TypeScript** — All signatures (ML-DSA-87), verifications, Spark authentication, and RBAC execute in Rust/WASM (`estream-app-wasm`). TypeScript is UI-only.
3. **CLI full parity** — Everything the visual circuit designer can do, the CLI (`estream marketplace`) must also do. The CLI is the primary testing interface.
4. **SmartCircuit-native** — Every queue and map operation is a SmartCircuit execution on native lex streams and lex state, not a standalone library crate. Components are active processing units with witness attestation and 8D metering.

### 1.2 SmartCircuit-Native Architecture

Chronicle Software proved that open-sourcing high-performance middleware creates a thriving ecosystem. eStream follows the same open-core model but with a fundamentally different architecture: **every queue and map operation is a SmartCircuit execution on native lex streams and lex state**.

| Chronicle (OSS) | eStream Native | Execution Model |
|-----------------|---------------|-----------------|
| Chronicle Queue | **Queue Streams** — SmartCircuit-driven lex streams | `queue.append.v1` circuit → Witness → Lex Store |
| Chronicle Map | **State Maps** — SmartCircuit-driven lex state | `map.put.v1` circuit → State Root → VRF Scatter |
| Chronicle Wire | Data — eStream Data | Exists |
| Chronicle Services | SmartCircuit runtime (BSL 1.1) | Exists |
| Chronicle FIX | Wire adapters via `WireAdapter` trait | Circuit-wrapped protocol adapters |

### 1.3 Licensing Tiers

| Tier | License | Components |
|------|---------|-----------|
| **Open Source** | Apache 2.0 | Queue, Map, Wire adapters, data schemas, SDK |
| **Source Available** | BSL 1.1 | FPGA acceleration, VRF Scatter HA, production runtime |
| **Commercial** | Enterprise | Managed deployment, SLA, support, custom adapters |

---

## 2. Component Model

### 2.1 Component Categories

Every publishable component belongs to one infrastructure category:

| Category | Description | Examples |
|----------|-------------|---------|
| `data-schema` | data schema packs | `data-iot`, `data-trading`, `data-carbon` |
| `wire-adapter` | Protocol adapters (impl `WireAdapter` trait) | `estream-wire-fix`, `estream-wire-mqtt` |
| `smart-circuit` | Reusable SmartCircuit packages | `carbon-credit-mint`, `order-matcher` |
| `fpga-circuit` | FPGA bitstream components | `ntt-accelerator`, `sha3-pipeline` |
| `integration` | Full-stack integrations | `thermogen-zero-edge` |
| `console-widget` | Console dashboard widgets | `impact-counter`, `network-map` |

Components are also tagged with domain categories for discovery:

```rust
pub enum DomainCategory {
    Crypto, Identity, DeFi, Gaming, Social,
    Compliance, Analytics, Networking, Storage,
    AiMl, Iot, Utility, Template,
}
```

### 2.2 Component Record

```rust
pub struct MarketplaceComponent {
    pub id: ComponentId,
    pub name: String,
    pub version: semver::Version,
    pub publisher: Publisher,
    pub description: String,
    pub readme: String,
    pub license: License,
    pub pricing: Pricing,
    pub badges: Vec<Badge>,
    pub categories: Vec<DomainCategory>,
    pub tags: Vec<String>,
    pub dependencies: Vec<DependencyRef>,
    pub stats: ComponentStats,
    pub storage: StorageRefs,
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
```

### 2.3 Source Visibility

Components support **tiered visibility**, controlled by the creator:

```rust
pub enum SourceVisibility {
    Open,          // Full ESCIR source visible to all
    Interface,     // Only ports, annotations, source hash visible
    Compiled,      // Interface + compiled artifacts (WASM/Verilog)
    LicensedFull,  // Full source visible only to licensees
}
```

**Always public** regardless of visibility level: name, version, publisher, input/output ports, resource requirements, estimated cost per execution, source hash.

Visibility is implemented using Data Filter — the same field-level privacy primitive used throughout the platform:

```rust
pub struct FilteredComponentData {
    pub interface: ComponentInterface,       // always visible
    pub resources: ResourceRequirements,     // always visible
    pub source_hash: [u8; 32],              // always visible

    #[filter(audiences = ["public", "licensee"])]
    pub escir_source: Filtered<ESCIR>,

    #[filter(audiences = ["public", "compiled", "licensee"])]
    pub wasm_artifact: Filtered<AssetId>,

    #[filter(audiences = ["public", "compiled", "licensee"])]
    pub verilog_artifact: Filtered<AssetId>,
}
```

**Policy**: Creators can increase visibility at any time (Interface → Open) but cannot decrease it after publishing. Official eStream components are always Open + Free.

---

## 3. Component Manifest (`estream-component.toml`)

Every publishable component includes an `estream-component.toml` manifest at its root.

### 3.1 Full Schema

```toml
[component]
name = "estream-wire-fix"
version = "1.0.0"
category = "wire-adapter"
description = "FIX protocol adapter for eStream"
license = "Apache-2.0"
repository = "https://github.com/toddrooke/estream-wire-fix"
homepage = "https://estream.io/components/wire-fix"
readme = "README.md"
keywords = ["fix", "trading", "capital-markets", "protocol"]

[component.author]
name = "eStream Contributors"
email = "components@estream.io"
url = "https://github.com/toddrooke"

[component.marketplace]
pricing = "free"        # free | one-time | subscription | usage-based | enterprise | freemium
visibility = "open"     # open | interface | compiled | licensed

[component.estream]
min_version = "0.8.0"
max_version = "1.0.0"   # optional

[component.dependencies]
data-trading = "^1.0.0"

[component.schemas]
provides = ["FixNewOrderSingle", "FixExecutionReport", "FixMarketData"]
requires = ["EStreamOrder", "EStreamFill"]

[component.circuits]
provides = ["fix_parser_circuit", "fix_order_router"]
target = ["cpu", "fpga"]

# Wire adapter metadata (category = "wire-adapter" only)
[component.wire_adapter]
protocol_family = "financial"
transports = ["tcp", "tls"]
bidirectional = true
request_response = true

# Console widget metadata (category = "console-widget" only)
[component.widget]
widget_category = "analytics"
roles = ["operator", "admin"]
sizes = ["small", "medium", "large"]
data_sources = ["eslite"]

# FPGA metadata (category = "fpga-circuit" only)
[component.fpga]
device_family = ["xcvu9p", "xcvu13p"]
resource_estimate = { luts = 50000, brams = 100, dsps = 0 }

[component.include]
schemas = ["schemas/*.data.yaml"]
circuits = ["circuits/*.fl"]
tests = ["tests/golden/**"]
fpga = ["fpga/*.bit"]
```

### 3.2 Name Conventions

- **Format**: lowercase alphanumeric with hyphens (`[a-z0-9-]+`)
- **Official prefixes**: `estream-*` and `data-*` (reserved for eStream team)
- **Third-party**: `@publisher/name` format (e.g., `@synergy-carbon/impact-counter`)

### 3.3 Version Requirements

| Syntax | Meaning |
|--------|---------|
| `^1.2.3` | Compatible (default): `>=1.2.3, <2.0.0` |
| `~1.2.3` | Patch-level: `>=1.2.3, <1.3.0` |
| `>=1.0.0` | Minimum version |
| `=1.2.3` | Exact version |
| `*` | Any version |

---

## 4. Package Format

### 4.1 Standard Directory Structure

```
my-component/
├── estream-component.toml         # Manifest (required)
├── README.md                      # Documentation (required)
├── LICENSE                        # License file
├── CHANGELOG.md                   # Version history
├── schemas/
│   └── *.data.yaml                # data schema definitions
├── circuits/
│   ├── *.fl                       # FastLang circuit definitions
│   └── *.circuit.yaml             # ESCIR circuit definitions
├── src/
│   └── lib.rs                     # Rust implementation (wire adapters)
├── tests/
│   └── golden/
│       ├── manifest.toml          # Test vector manifest
│       └── *.json                 # Golden test vectors
├── fpga/
│   ├── manifest.toml              # Bitstream manifest
│   └── *.bit                      # FPGA bitstreams
├── widgets/
│   ├── widget.manifest.yaml       # Widget metadata
│   ├── src/                       # TypeScript/TSX source
│   └── dist/                      # Built ES module bundle
└── SIGNATURE.ml-dsa               # ML-DSA-87 signature (generated on publish)
```

### 4.2 Minimum Viable Package

```
my-component/
├── estream-component.toml
├── README.md
└── circuits/
    └── my-circuit.fl
```

### 4.3 Test Vector Format

Golden test vectors are JSON files defining input/output pairs:

```json
{
  "name": "new-order-buy-limit",
  "description": "Standard buy limit order",
  "circuit": "fix_parser_circuit",
  "input": {
    "fix_message": "8=FIX.4.4|35=D|49=SENDER|56=TARGET|..."
  },
  "expected_output": {
    "order": {
      "side": "buy",
      "type": "limit",
      "quantity": 1000,
      "price": 150.25
    }
  },
  "expected_witness_tier": 2
}
```

Test vector manifest (`tests/golden/manifest.toml`):

```toml
[test_suite]
name = "estream-wire-fix golden tests"
circuit = "fix_parser_circuit"
version = "1.0.0"

[[vectors]]
file = "new-order.json"
tags = ["order", "happy-path"]

[[vectors]]
file = "execution-report.json"
tags = ["fill", "happy-path"]
```

### 4.4 Deterministic Archive

The published package is a deterministic `tar.gz`:
- Files sorted alphabetically
- Timestamps zeroed
- Owner/group set to `0/0`
- Permissions normalized to `644` (files) / `755` (dirs)

This ensures the SHA3-256 checksum is reproducible.

---

## 5. Data Model — Graph-Based Registry

The marketplace is modeled as a **`graph` construct** — the same first-class FastLang primitive used for device mesh, wallet ledger, governance, and expert mesh. Publishers, components, versions, reviews, and licenses are typed nodes and edges stored in CSR format with real-time overlays, AI-powered recommendations, and a tamper-proof series.

Component dependency resolution uses a **`dag` construct** with `enforce acyclic` and `topo_sort` — hardware-enforced cycle detection and topological installation ordering.

See `streams/marketplace_streams.fl` for the full implementation.

### 5.1 Registry Graph

```fastlang
graph marketplace_registry {
    node ComponentNode
    edge PublishesEdge

    // Real-time marketplace state overlays
    overlay download_count: u64 bitmask delta_curate
    overlay active_installs: u64 bitmask delta_curate
    overlay rating_x100: u32 bitmask delta_curate
    overlay badges: u32 bitmask
    overlay revenue_es: u64 bitmask delta_curate

    // Security monitoring overlays
    overlay vulnerability_count: u16 curate delta_curate
    overlay signature_valid: bool curate delta_curate

    storage csr {
        hot @bram,
        warm @ddr,
        cold @nvme,
    }

    ai_feed marketplace_recommendation

    observe marketplace_registry: [download_count, rating_x100, vulnerability_count, signature_valid] threshold: {
        anomaly_score 0.85
        baseline_window 300
    }
}

series registry_series: marketplace_registry
    merkle_chain true
    lattice_imprint true
    witness_attest true
```

The `ai_feed marketplace_recommendation` drives trending predictions and personalized "for you" suggestions. The `observe` clause with `anomaly_score 0.85` triggers StreamSight escalation when download or security patterns deviate from baseline.

### 5.2 Dependency DAG

```fastlang
dag component_dependencies {
    node ComponentNode
    edge DependsOnEdge

    overlay version_conflict: bool curate delta_curate
    overlay install_order: u32 curate

    storage csr {
        hot @bram,
        warm @ddr,
        cold @nvme,
    }
}
```

This replaces the custom 9-step version resolution algorithm (§6.4) with native `dag` operations:
- **`enforce acyclic`** — hardware rejects circular dependencies at the CSR pipeline level
- **`topo_sort`** — returns topological installation order
- **`version_conflict` overlay** — curated to flag only conflicting nodes

### 5.3 Overlays vs. Separate Streams

Instead of separate streams for downloads, ratings, and security, the graph's overlays provide per-cycle, per-component state:

| Overlay | Type | Curate | Purpose |
|---------|------|--------|---------|
| `download_count` | `u64` | `delta_curate` | Trending, popularity ranking |
| `active_installs` | `u64` | `delta_curate` | Active user tracking |
| `rating_x100` | `u32` | `delta_curate` | Quality ranking |
| `badges` | `u32` | — | Trust indicators (bitmask) |
| `revenue_es` | `u64` | `delta_curate` | Creator program tier tracking |
| `vulnerability_count` | `u16` | `curate` + `delta_curate` | Security alerting |
| `signature_valid` | `bool` | `curate` + `delta_curate` | Integrity monitoring |

### 5.4 Series — Tamper-Proof Audit Trail

The `series` on the registry graph means every mutation (publish, yank, review, license grant) is automatically:
- **MTP-timestamped** — ordering from physics
- **Merkle-chained** — position in causal DAG is mathematically determined
- **Lattice-imprinted** — Proof of Circuit proves which hardware processed it
- **Witness-attested** — PoVC verification at epoch boundaries

No separate audit log is needed. The graph series IS the audit trail.

### 5.5 Topic Map (Lattice Projections)

Lattice topics are **projections** of graph state, not the primary data model:

| Topic | Type | Projected From |
|-------|------|---------------|
| `/marketplace/index` | state | `marketplace_registry` graph + overlays |
| `/marketplace/search` | event | `marketplace_search_query` circuit |
| `/marketplace/reviews` | event | `ReviewNode` graph entries |
| `/marketplace/install/{requestId}` | event | `marketplace_install_component` circuit |
| `/marketplace/publish/{requestId}` | event | `marketplace_publish_component` circuit |
| `/marketplace/licenses` | state | `LicenseNode` graph entries |
| `/marketplace/publishers` | state | `PublisherNode` graph entries |

### 5.6 Browser SDK Integration

The `@estream/sdk-browser` provides typed React hooks for marketplace topics:

```typescript
import {
  useMarketplaceIndex,
  useMarketplaceComponent,
  useMarketplaceSearch,
  useMarketplaceInstall,
  useMarketplaceLicenses,
} from '@estream/sdk-browser';
```

These use the same `useSubscription<T>(topic)` / `useEmit(topic)` pattern as topology, circuits, and governance hooks. The hooks subscribe to lattice topic projections of graph state — the TypeScript layer sees typed data, not the underlying graph structure.

### 5.7 Trust Boundary

```
TypeScript (UI-only, untrusted)      Rust/WASM (trusted)
├── React components                  ├── ML-DSA-87 verify_signature
├── Zustand store                     ├── ML-DSA-87 sign_message
├── ReactFlow canvas                  ├── SHA3-256 hash
└── useSubscription/useEmit           ├── RBAC verify_token
    └── WebTransport/QUIC ────────────└── Spark auth verify
         └── Edge Node (lattice streams)
```

TypeScript never touches signing keys, session tokens, or signature verification.

---

## 6. Registry Protocol

### 6.1 GitHub-Backed Registry

The registry is a GitHub repository (`estream-io/registry`):

```
estream-io/registry/
├── README.md
├── config.json
├── index/
│   ├── data-schema/
│   │   └── data-trading/
│   │       ├── metadata.json
│   │       └── 1.0.0/
│   │           ├── estream-component.toml
│   │           ├── SIGNATURE.ml-dsa
│   │           └── checksum.sha3
│   ├── wire-adapter/
│   ├── smart-circuit/
│   ├── fpga-circuit/
│   ├── integration/
│   └── console-widget/
└── publishers/
    ├── estream.json
    └── synergy-carbon.json
```

### 6.2 `metadata.json`

```json
{
  "name": "estream-wire-mqtt",
  "category": "wire-adapter",
  "publisher": "estream",
  "description": "MQTT 5.0 protocol adapter for eStream",
  "versions": [
    {
      "version": "1.0.0",
      "published_at": "2026-02-10T12:00:00Z",
      "checksum_sha3": "a1b2c3...",
      "signature_pubkey_id": "estream-signing-key-01",
      "yanked": false,
      "estream_min_version": "0.8.0",
      "archive_url": "https://github.com/estream-io/registry/releases/download/estream-wire-mqtt-1.0.0/package.tar.gz"
    }
  ],
  "latest": "1.0.0",
  "keywords": ["mqtt", "iot", "messaging"],
  "badges": ["official", "tested"],
  "stats": { "downloads": 0, "stars": 0 }
}
```

### 6.3 `publishers/{name}.json`

```json
{
  "name": "estream",
  "display_name": "eStream Contributors",
  "url": "https://github.com/polyquantum",
  "verified": true,
  "signing_keys": [
    {
      "key_id": "estream-signing-key-01",
      "algorithm": "ML-DSA-87",
      "public_key_hex": "abcd1234...",
      "created_at": "2026-01-01T00:00:00Z",
      "expires_at": "2028-01-01T00:00:00Z",
      "status": "active"
    }
  ]
}
```

### 6.4 Version Resolution

Dependency resolution operates on the `component_dependencies` DAG (§5.2). The `enforce acyclic` constraint rejects circular dependencies at the hardware level, and `topo_sort` produces the installation order:

```
1. Fetch index/{category}/{name}/metadata.json
2. Parse version requirement (default: latest non-yanked)
3. Filter: remove yanked, check estream_min_version compatibility
4. Select highest matching version (semver sort)
5. Build dependency DAG (add edges to component_dependencies)
6. enforce acyclic — hardware rejects circular dependencies
7. topo_sort — produces topological installation order
8. version_conflict overlay — curate flags conflicting nodes
9. Download package archives in topological order
10. Verify ML-DSA-87 signatures and SHA3-256 checksums
11. Install to workspace
```

Steps 5-8 are native `dag` operations, not application code.

### 6.5 Local Cache

```
$HOME/.estream/cache/
├── registry/
│   └── index-snapshot.json
├── packages/
│   └── estream-wire-fix/1.0.0/
│       ├── package.tar.gz
│       └── estream-component.toml
└── config.toml
```

Cache TTL defaults to 24 hours. `estream marketplace install --force` bypasses cache.

### 6.6 Installation Targets

| Category | Install Path |
|----------|-------------|
| `data-schema` | `schemas/` |
| `wire-adapter` | `adapters/` |
| `smart-circuit` | `circuits/` |
| `fpga-circuit` | `fpga/` |
| `integration` | `integrations/` |
| `console-widget` | `widgets/` |

### 6.7 Workspace Integration

After installation, components are tracked in `estream-workspace.toml`:

```toml
[dependencies]
estream-wire-fix = "1.0.0"
data-trading = "1.2.0"

[dependencies.estream-wire-fix]
version = "1.0.0"
checksum = "a1b2c3d4..."
installed_at = "2026-02-10T12:00:00Z"
```

### 6.8 Multiple Registries

Resolution order: local workspace → default registry → additional registries (in declaration order). Scoped names (`@publisher/name`) resolve against the registry where the publisher is registered.

### 6.9 Offline Support

```bash
estream marketplace export estream-wire-fix --output ./offline-bundle/
estream marketplace install --from ./offline-bundle/
```

---

## 7. ML-DSA-87 Signing

All published components are signed with ML-DSA-87 (FIPS 204) for post-quantum integrity. The signing model uses a Merkle tree of individual file hashes for tamper detection.

### 7.1 Signing Process

```
1. List all files in the package (sorted alphabetically)
2. Compute SHA3-256 hash of each file
3. Build a Merkle tree from the file hashes
4. Sign the Merkle root with ML-DSA-87 private key
5. Write SIGNATURE.ml-dsa with root, per-file hashes, and signature
```

### 7.2 Signing Workflow

```
Publisher                              Registry
   │                                      │
   │  1. estream marketplace publish      │
   │     ├─ Build deterministic archive   │
   │     ├─ Compute Merkle root           │
   │     ├─ Sign with ML-DSA-87           │
   │     └─ Create SIGNATURE.ml-dsa       │
   │                                      │
   │  2. Submit PR to registry repo       │
   │     ├─ index/{cat}/{name}/{ver}/     │
   │     ├─ estream-component.toml        │
   │     ├─ SIGNATURE.ml-dsa              │
   │     └─ checksum.sha3                 │
   │                                      │
   │                  ◀──── 3. CI verifies │
   │                        ├─ Signature valid?
   │                        ├─ Publisher key in publishers/?
   │                        ├─ Version increment correct?
   │                        ├─ Manifest validates?
   │                        └─ Archive matches checksum?
   │                                      │
   │                  ◀──── 4. Merge PR    │
   │                        (auto or review)
   │                                      │
   │  5. Upload archive to GitHub Release │
```

### 7.3 `SIGNATURE.ml-dsa` Format

```json
{
  "algorithm": "ML-DSA-87",
  "key_id": "estream-signing-key-01",
  "signed_at": "2026-02-10T12:00:00Z",
  "merkle_root": "a1b2c3d4...",
  "file_hashes": {
    "estream-component.toml": "f1e2d3...",
    "circuits/fix-parser.fl": "b4c5d6...",
    "schemas/order.data.yaml": "e7f8a9..."
  },
  "signature_hex": "0123456789abcdef..."
}
```

### 7.4 Verification

```rust
pub fn verify_package(
    signature_file: &SignatureFile,
    package_dir: &Path,
    publisher_keys: &PublisherKeys,
) -> Result<VerificationResult, VerifyError> {
    // 1. Recompute file hashes
    for (path, expected_hash) in &signature_file.file_hashes {
        let content = std::fs::read(package_dir.join(path))?;
        let actual = estream_kernel::crypto::sha3_256(&content);
        if hex::encode(&actual) != *expected_hash {
            return Ok(VerificationResult::TamperedFile(path.clone()));
        }
    }

    // 2. Recompute Merkle root
    let computed_root = compute_merkle_root(&signature_file.file_hashes);
    if computed_root != signature_file.merkle_root {
        return Ok(VerificationResult::MerkleRootMismatch);
    }

    // 3. Verify ML-DSA-87 signature over the Merkle root
    let pubkey = publisher_keys
        .get(&signature_file.key_id)
        .ok_or(VerifyError::UnknownKey(signature_file.key_id.clone()))?;

    let valid = estream_kernel::crypto::verify_mldsa87(
        pubkey.as_bytes(),
        computed_root.as_bytes(),
        &hex::decode(&signature_file.signature_hex)?,
    );

    if valid {
        Ok(VerificationResult::Valid {
            key_id: signature_file.key_id.clone(),
            signed_at: signature_file.signed_at.clone(),
        })
    } else {
        Ok(VerificationResult::InvalidSignature)
    }
}
```

### 7.5 WASM Trust Boundary

The same Rust crypto code runs in two targets:
- **CLI**: Native Rust via `estream_kernel::crypto`
- **Browser**: WASM via `estream-app-wasm` (`verify_signature`, `sign_message`, `hash_sha3_256`)

TypeScript never handles signing keys or performs verification directly.

---

## 8. Pricing and Licensing

### 8.1 Pricing Models

```rust
pub enum Pricing {
    Free,
    OneTime { price_es: u64 },
    Subscription { monthly_es: u64, annual_discount_pct: u8 },
    UsageBased { per_execution_es: Fixed64, free_tier_executions: u64 },
    Enterprise,
    Freemium { free_features: Vec<String>, premium_price_es: u64, premium_features: Vec<String> },
}
```

### 8.2 Revenue Split

| Pricing Type | Creator | Platform | Burn |
|-------------|---------|----------|------|
| OneTime | 85% | 10% | 5% |
| Subscription | 90% | 5% | 5% |
| UsageBased | 85% | 10% | 5% |
| Enterprise | 80% | 15% | 5% |
| Freemium | 85% | 10% | 5% |

### 8.3 License Types

```rust
pub enum LicenseType {
    Perpetual,
    Subscription { period: SubscriptionPeriod },
    UsageBased { max_executions: u64 },
    Trial { duration_days: u8 },
}
```

### 8.4 Real-Time Metering

Payments happen in real-time, in parallel with execution. Metering runs as a parallel circuit with zero overhead, settled atomically at execution end:

```
[Input] → [Component A] → [Component B] → [Output]
               │                  │
               ▼                  ▼
    Metering Circuit (parallel, zero overhead)
    Component A: 0.002 ES  |  Component B: 0.001 ES
    ──────────────────────────────────────────────
    Single Atomic Settlement at execution end
```

### 8.5 Cost Estimation

```rust
pub fn estimate_cost(circuit: &ESCIR) -> CostEstimate {
    let mut total = Fixed64::ZERO;
    for component in circuit.components() {
        total += component.estimated_cost_per_exec();
    }
    total += circuit.witness_tier().cost();
    total += platform_fee(total);
    CostEstimate { min: total * 0.9, max: total * 1.1, expected: total }
}
```

---

## 9. Quality and Trust

### 9.1 Badges

```rust
pub enum Badge {
    Verified,                                    // Identity verified
    Tested { test_count: u32, coverage_pct: u8 }, // Automated tests pass
    Audited { auditor: String, report_url: String, date: DateTime<Utc> },
    Certified { level: CertificationLevel, expires: DateTime<Utc> },
    Official,                                    // Built by eStream team
    CommunityChoice { year: u16 },
    HighPerformance { benchmark_score: u32 },
    PostQuantum,
}

pub enum CertificationLevel { Bronze, Silver, Gold }
```

| Badge | Requirements | Cost |
|-------|-------------|------|
| Verified | KYC/identity verification | Free |
| Tested | CI/CD with >80% coverage | Free |
| Audited | Third-party audit report | Audit cost |
| Certified | eStream team review + ongoing | 500 ES/year |
| Official | Built by eStream team | N/A |

### 9.2 FPGA Badges

```rust
pub enum FpgaBadge {
    TimingVerified { fmax_mhz: u32, slack_ns: f32, tool: String },
    FormallyVerified { tool: String, properties_checked: u32 },
    ConstantTime,
    CrossPlatform { targets: Vec<FpgaTarget> },
    Simulated { tool: String, coverage_pct: u8 },
    LowPower { watts: f32 },
}
```

### 9.3 Reviews and Ratings

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
}
```

Rating score uses weighted calculation: 40% rating, 30% popularity, 20% freshness, 10% quality badges.

### 9.4 Moderation

Automated checks: malware scanning, license compliance, dependency vulnerabilities, code quality. Manual review triggers: first publish, significant update, reported content, high-risk category.

### 9.5 Dispute Resolution

Dispute reasons: copyright claim, security vulnerability, false advertising, license violation. Status flow: Open → UnderReview → AwaitingResponse → Resolved/Escalated.

---

## 10. Creator Program

### 10.1 Creator Tiers

| Tier | Threshold | Benefits |
|------|----------|---------|
| **Starter** | Default | Basic analytics, community support |
| **Growing** | 100+ ES/mo or 1K+ downloads | Detailed analytics, email support, featured in category |
| **Established** | 1K+ ES/mo or 10K+ downloads | Advanced analytics, priority support, homepage featuring |
| **Partner** | 10K+ ES/mo + invitation | Full analytics, dedicated account manager, co-marketing, roadmap input |

---

## 11. Console Widgets

### 11.1 Widget Metadata

Widgets include a `widget.manifest.yaml` alongside the component manifest:

```yaml
widget:
  name: impact-counter
  display_name: "Impact Counter"
  icon: activity
  default_size: medium
  min_size: small
  max_size: large
  config_schema:
    type: object
    properties:
      metric:
        type: string
        enum: [carbon_offset, energy_saved, revenue]
        default: carbon_offset
  data_sources:
    - type: eslite
      table: metrics
      fields: [value, timestamp]
```

### 11.2 Bundle Format

- **Format**: ES Module (`.mjs`), React component default export
- **Max size**: 500KB (gzipped)
- **Externalized peers**: `react`, `react-dom`, `@estream/sdk-browser`
- **CSP compliant**: No inline scripts, no eval

### 11.3 Trust Levels

| Level | Access | Isolation | Publisher Requirement |
|-------|--------|-----------|---------------------|
| **Trusted** | Full Console API | Direct React render | Official eStream |
| **Verified** | Scoped data via WidgetGateway | Scoped provider | Governance-reviewed |
| **Community** | postMessage bridge only | Sandboxed iframe | Any publisher |

### 11.4 Security Model

- All widgets verified with ML-DSA-87 via WASM RBAC module
- Community widgets run in sandboxed iframes with CSP: `default-src 'self'; script-src 'self' blob:; style-src 'self' 'unsafe-inline'`
- Widget publish requires governance circuit: `estream.marketplace.widget.publish.v1`

---

## 12. FPGA Components

### 12.1 FPGA Resource Requirements

```rust
pub struct FpgaResources {
    pub luts: u32,
    pub bram_kb: u32,
    pub dsp_slices: u32,
    pub flip_flops: u32,
    pub target_fmax_mhz: u32,
    pub io_pins: u32,
    pub estimated_power_watts: f32,
}

pub enum FpgaTarget {
    Nexus, Artix, Kintex, Virtex,
    Cyclone, Arria, Stratix, Generic,
}
```

### 12.2 FPGA Pricing

```rust
pub enum FpgaPricing {
    PerBitstream { price_es: u64 },
    PerMessage { price_es: Fixed64, includes_per_month: u64 },
    PerDevice { monthly_es: u64 },
    SiteLicense { annual_es: u64 },
}
```

### 12.3 SKU Tiers

FPGA components commonly ship in multiple tiers with different resource/performance tradeoffs:

| Tier | Typical Profile |
|------|----------------|
| **Lite** | Minimal resources, single protocol, no hardware acceleration |
| **Standard** | Multi-protocol, StreamSight telemetry, moderate FPGA acceleration |
| **Premium** | Full protocol suite, FPGA-only acceleration, enterprise SLA, formal verification |

### 12.4 Bitstream Packaging

FPGA bitstreams are included in the `fpga/` directory with a `manifest.toml`:

```toml
[[bitstreams]]
target = "xcvu9p"
file = "fix-parser-xcvu9p.bit"
synthesis_tool = "Vivado 2024.2"
resource_usage = { luts = 45000, brams = 90, dsps = 0, ffs = 38000 }
timing = { fmax_mhz = 300, wns_ns = 0.5 }
```

Bitstreams are licensed under BSL 1.1 (source-available, time-delayed open-source).

---

## 13. Circuit Designer Integration

The visual circuit designer provides an "app store" marketplace experience. See [Phase 4: Circuit Designer UX](https://github.com/polyquantum/estream/issues/133) for implementation details.

### 13.1 Marketplace Panel

The Sidebar gains a tab toggle: **Palette** | **Marketplace**. When Marketplace is active:

```
┌──────────────────────────────────────┐
│  [Palette]  [Marketplace]            │
├──────────────────────────────────────┤
│  ┌────────────────────────────────┐  │
│  │ 🔍 Search components...       │  │
│  └────────────────────────────────┘  │
│                                      │
│  Featured                            │
│  ┌──────────────────────────────┐   │
│  │ estream-wire-fix    v1.0.0  │   │
│  │ eStream      Official Free  │   │
│  │ FIX protocol wire adapter   │   │
│  │ ★★★★☆  1.2K installs       │   │
│  │              [Install]      │   │
│  └──────────────────────────────┘   │
│                                      │
│  Categories                          │
│  ▸ Fintech (3)                      │
│  ▸ Industrial (1)                   │
│  ▸ Crypto (0)                       │
│  ▸ IoT (0)                          │
└──────────────────────────────────────┘
```

### 13.2 Component Detail Modal

```
┌──────────────────────────────────────────────────────────────┐
│  estream-wire-fix v1.0.0                            [Close] │
│  by eStream Contributors (verified)                         │
│  ────────────────────────────────────────────────────────── │
│  [Official] [Tested] [PostQuantum]         Free | Apache-2.0│
│  ★★★★☆ 4.2 (47 reviews)  |  1,247 installs                │
│  Source: Open  |  Targets: CPU, FPGA                        │
│                                                              │
│  Ports:                                                      │
│   IN:  fix_raw (bytes)       OUT: orders (Order)            │
│   IN:  session_cfg (Config)  OUT: executions (Fill)          │
│                                                              │
│  Resources: T2 witness, 2K compute, 8KB mem                 │
│  Est. cost: 0.003 ES/exec                                   │
│                                                              │
│  Dependencies: data-trading ^1.0.0 (auto-installed)          │
│                                                              │
│  [Install]  [View Source]  [View Docs]  [Add to Circuit]    │
└──────────────────────────────────────────────────────────────┘
```

### 13.3 Install Flow

1. User clicks [Install]
2. Browser emits to `/marketplace/install` via `useEmit`
3. WASM: `verify_signature()` for ML-DSA-87 package signature
4. WASM: `hash_sha3_256()` for checksum verification
5. Progress shown via `/marketplace/install/{requestId}` subscription
6. Installed component appears in Palette tab under "Marketplace" section
7. Component is draggable onto canvas like any native node

### 13.4 Publish-from-Designer

1. Requires Spark authentication (indicator in toolbar)
2. [Publish] opens dialog with auto-detected manifest
3. Visibility selector (4 tiers), pricing model selector
4. WASM: `sign_message()` signs package hash with ML-DSA-87
5. Emits to `/marketplace/publish` stream

### 13.5 Branding

The circuit designer follows the [eStream Brand Guidelines](../../business/brand/estream/BRAND_GUIDELINES.md):
- Primary: eStream Blue `#2F5CD5`, Slate 900 `#0F172A`, Slate 800 `#1E293B`
- Typography: Inter (headings/body), JetBrains Mono (code/export)
- Logo: `estream-logo-white.svg` in toolbar

---

## 14. CLI Commands

All commands live under `estream marketplace` (aliased as `estream mp`).

### 14.1 `estream marketplace search`

```
USAGE:
    estream marketplace search [OPTIONS] <QUERY>

OPTIONS:
    -c, --category <CATEGORY>    Filter by category
    -t, --tag <TAG>              Filter by keyword tag
        --verified               Only verified publishers
        --official               Only official eStream components
        --limit <N>              Maximum results (default: 20)
    -o, --output <FORMAT>        Output format: table (default), json
```

```
$ estream marketplace search "FIX adapter"

  NAME                  VERSION  CATEGORY       PUBLISHER   BADGES
  estream-wire-fix      1.0.0    wire-adapter   estream     ✓ Official, Tested
  fix-order-router      0.3.0    smart-circuit  acme-fin    Verified

  2 results found
```

### 14.2 `estream marketplace install`

```
USAGE:
    estream marketplace install [OPTIONS] <COMPONENT>[@VERSION]

OPTIONS:
        --dry-run    Show what would be installed
        --force      Bypass cache
        --no-verify  Skip signature verification (NOT recommended)
        --save       Add to estream-workspace.toml
```

```
$ estream marketplace install estream-wire-fix@^1.0.0

  Resolving dependencies...
    estream-wire-fix v1.0.0
    └── data-trading v1.2.0

  Verifying ML-DSA-87 signatures...
    estream-wire-fix v1.0.0 ✓ (key: estream-signing-key-01)
    data-trading v1.2.0 ✓ (key: estream-signing-key-01)

  Installed 2 components (6 files) in 3.2s
```

### 14.3 `estream marketplace publish`

```
USAGE:
    estream marketplace publish [OPTIONS] [PATH]

OPTIONS:
        --dry-run        Validate without publishing
        --key <FILE>     ML-DSA-87 private key file
        --skip-tests     Skip test vectors
    -y, --yes            Skip confirmation
```

### 14.4 `estream marketplace verify`

Verify ML-DSA-87 signature of an installed component.

### 14.5 `estream marketplace scaffold`

```
$ estream marketplace scaffold wire-adapter estream-wire-amqp

  Created estream-wire-amqp/
  ├── estream-component.toml
  ├── README.md
  ├── schemas/
  ├── circuits/
  │   └── amqp-adapter.circuit.yaml
  ├── tests/golden/
  └── src/lib.rs
```

### 14.6 `estream marketplace list`

List installed components from `estream-workspace.toml`.

### 14.7 `estream marketplace info`

Show detailed information for a specific component.

---

## 15. Error Codes

| Code | Name | Description |
|------|------|-------------|
| `E001` | `ComponentNotFound` | No component with the given name |
| `E002` | `VersionNotFound` | No matching version |
| `E003` | `VersionConflict` | Incompatible version requirements |
| `E004` | `CircularDependency` | Dependency cycle detected |
| `E005` | `SignatureInvalid` | ML-DSA-87 verification failed |
| `E006` | `ChecksumMismatch` | SHA3-256 checksum mismatch |
| `E007` | `UnknownPublisher` | Publisher key not in registry |
| `E008` | `ManifestInvalid` | Manifest validation failed |
| `E009` | `NameReserved` | Uses reserved `estream-*` or `data-*` prefix |
| `E010` | `VersionNotIncremented` | Version not higher than latest |
| `E011` | `PackageTooLarge` | Archive exceeds 50 MB |
| `E012` | `NetworkError` | Cannot reach registry |
| `E013` | `CacheCorrupted` | Local cache corrupted |
| `E014` | `SchemaConflict` | Two components provide same schema |
| `E015` | `PermissionDenied` | Insufficient publish permissions |

---

## 16. Configuration

### 16.1 Global Config

```toml
# $HOME/.estream/config.toml

[registry]
default = "estream-io/registry"

[[registry.sources]]
name = "enterprise"
url = "my-org/estream-registry-internal"
token_env = "ESTREAM_ENTERPRISE_TOKEN"

[cache]
ttl_hours = 24
max_size_mb = 500

[signing]
default_key = "$HOME/.estream/keys/signing-key.pem"
```

---

## 17. Publishing Flow

### 17.1 Pre-publish Validation

1. Manifest completeness
2. Name format (`[a-z0-9-]+` or `@[a-z0-9-]+/[a-z0-9-]+`)
3. Version increment (higher than any published version)
4. Schema validation (all `provides` schemas exist)
5. Dependency resolution (all `requires` available)
6. Circuit validation (all circuits parse as valid ESCIR)
7. Test vectors pass (if `tests/golden/` exists)
8. File size limit (< 50 MB)

### 17.2 CI Verification (GitHub Actions)

1. Download package archive from PR
2. Verify SHA3-256 checksum
3. Verify ML-DSA-87 signature against publisher key
4. Validate manifest schema
5. Check version increment
6. Run test vectors
7. Post results as PR comment
8. Auto-merge for verified publishers

---

## 18. Submission Process

### 18.1 End-to-End Flow

```
1.  Build circuit (visual designer or FastLang)
2.  estream marketplace scaffold <category> <name>
3.  Edit estream-component.toml, add circuits, tests, docs
4.  Spark authenticate (CLI challenge or visual auth)
5.  estream marketplace publish (or [Publish] in designer)
    a. Validate manifest
    b. Run test vectors
    c. Build deterministic tar.gz
    d. Compute Merkle root of file hashes (SHA3-256)
    e. Sign Merkle root with ML-DSA-87
    f. Emit to /marketplace/publish stream
6.  Edge node creates PR against estream-io/registry
7.  CI: signature, checksum, version, tests → all pass
8.  Auto-merge (verified publisher) or manual review
9.  Component appears on /marketplace/index stream
10. Available to all designer/CLI users
```

### 18.2 First-Party vs Third-Party

**First-party** (eStream team): Direct commit to `estream-marketplace` repo, publish via CI, auto-verified.

**Third-party** (app developers): Fork registry, submit PR, CI verification, review for first publish, auto-merge after established.

---

## 19. Domain Schema Packs

### 19.1 `data-iot`

IoT telemetry schemas: SensorReading, DeviceState, CommandEnvelope, AlertEvent, Geolocation.

### 19.2 `data-trading`

Capital markets schemas: EStreamOrder, EStreamFill, EStreamQuote, EStreamMarketData.

### 19.3 `data-carbon`

Carbon/ESG schemas: CarbonCredit, EmissionReport, CarbonOffset, MethodologyAttestation.

### 19.4 Schema Pack Manifest

```toml
[component]
name = "data-iot"
version = "1.0.0"
category = "data-schema"
description = "IoT telemetry schemas for sensor networks"

[component.schemas]
provides = ["SensorReading", "DeviceState", "CommandEnvelope", "AlertEvent"]
requires = []

[component.include]
schemas = ["schemas/*.data.yaml"]
```

---

## 20. Wire Adapters

Future marketplace wire adapters:

| Protocol | Package Name | Family | Status |
|----------|-------------|--------|--------|
| FIX 4.2/4.4 | `estream-wire-fix` | Financial | Available |
| ISO 20022 | `estream-wire-iso20022` | Financial | Available |
| Modbus TCP | `estream-wire-modbus` | Industrial | Available |
| MQTT 5.0 | `estream-wire-mqtt` | Messaging | Planned |
| HL7 FHIR | `estream-wire-hl7` | Healthcare | Planned |
| SWIFT MX | `estream-wire-swift` | Financial | Planned |
| OPC-UA | `estream-wire-opcua` | Industrial | Planned |

---

## 21. Performance Targets

### CPU Targets

| Component | Metric | Target |
|-----------|--------|--------|
| Queue append | Throughput | > 1M msg/sec |
| Map put/get | Latency (p99) | < 1μs |
| Wire adapter (FIX) | Parse throughput | > 500K msg/sec |
| Data serialize | Throughput | > 2M msg/sec |

### FPGA Targets

| Component | Metric | Target |
|-----------|--------|--------|
| Queue append | Throughput | > 100M msg/sec |
| Map put/get | Latency | < 100ns |
| Wire adapter (FIX) | Parse throughput | > 50M msg/sec |
| Data serialize | Pipeline rate | 1 msg/clock cycle |

---

## References

- [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) — `WireAdapter` trait
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema dependencies
- [COMPONENT_SYSTEM_SPEC.md](../protocol/COMPONENT_SYSTEM_SPEC.md) — Core component model
- [HARDWARE_TIER_SPEC.md](../specs/HARDWARE_TIER_SPEC.md) — FPGA hardware tiers
- [Brand Guidelines](../../business/brand/estream/BRAND_GUIDELINES.md) — Visual identity
- [marketplace_streams.fl](../streams/marketplace_streams.fl) — Stream API data declarations

---

*Created: February 2026*
*Status: Draft*
*Epic: [polyquantum/estream#136](https://github.com/polyquantum/estream/issues/136)*
