# Component Registry API Specification

> GitHub-backed component registry for discovering, installing, publishing, and verifying eStream components.

**Status:** Draft  
**Version:** 1.0.0  
**Issue:** [#525](https://github.com/polyquantum/estream-io/issues/525)  
**Parent Epic:** [#524](https://github.com/polyquantum/estream-io/issues/524)  
**Dependencies:** Component System (#309), ML-DSA-87 Signing (`estream-kernel::pq::sign`)  
**Extends:** [COMPONENT_SYSTEM_SPEC.md](../protocol/COMPONENT_SYSTEM_SPEC.md)

---

## 1. Overview

The Component Registry API provides a standard mechanism for discovering, installing, publishing, and verifying eStream components. It is backed by a GitHub repository to minimize infrastructure costs while leveraging existing developer workflows (pull requests for publishing, GitHub Actions for CI/CD verification).

### 1.1 Why GitHub-Backed

- Zero infrastructure cost to start
- Developers already have GitHub accounts
- PR-based publishing creates a natural review flow
- GitHub Actions for automated verification
- Can migrate to a dedicated registry server later if needed

### 1.2 Component Categories

| Category | Description | Examples |
|----------|-------------|---------|
| `esf-schema` | ESF schema packs | `esf-iot`, `esf-trading`, `esf-carbon` |
| `wire-adapter` | Protocol adapters (impl `WireAdapter` trait) | `estream-wire-fix`, `estream-wire-mqtt` |
| `smart-circuit` | Reusable SmartCircuit packages | `carbon-credit-mint`, `order-matcher` |
| `fpga-circuit` | FPGA bitstream components | `ntt-accelerator`, `sha3-pipeline` |
| `integration` | Full-stack integrations | `thermogen-zero-edge` |
| `console-widget` | Console dashboard widgets | `impact-counter`, `network-map` |

---

## 2. Component Manifest (`estream-component.toml`)

Every publishable component includes an `estream-component.toml` manifest at its root.

### 2.1 Full Schema

```toml
[component]
# Required fields
name = "estream-wire-fix"                    # Unique component name (lowercase, hyphens)
version = "1.0.0"                            # Semantic version (major.minor.patch)
category = "wire-adapter"                    # One of the component categories
description = "FIX protocol adapter for eStream"

# Optional metadata
license = "Apache-2.0"                       # SPDX license identifier
repository = "https://github.com/toddrooke/estream-wire-fix"
homepage = "https://estream.io/components/wire-fix"
readme = "README.md"                         # Relative path to readme
keywords = ["fix", "trading", "capital-markets", "protocol"]

# Publisher identity
[component.author]
name = "eStream Contributors"
email = "components@estream.io"
url = "https://github.com/toddrooke"

# Marketplace settings (optional — defaults to free/open)
[component.marketplace]
pricing = "free"                             # free | one-time | subscription | usage-based
visibility = "open"                          # open | interface | compiled | licensed
                                             # (see MARKETPLACE_SPEC.md for visibility model)

# eStream version compatibility
[component.estream]
min_version = "0.8.0"                        # Minimum estream-kernel version
max_version = "1.0.0"                        # Maximum estream-kernel version (optional)

# Dependencies on other marketplace components
[component.dependencies]
esf-trading = "^1.0.0"                       # Semver version requirement
estream-kernel = ">=0.8.0"                   # Core dependency (always implicit)

# ESF schemas provided and required (see ESF_SCHEMA_COMPOSITION_SPEC.md)
[component.schemas]
provides = ["FixNewOrderSingle", "FixExecutionReport", "FixMarketData"]
requires = ["EStreamOrder", "EStreamFill"]

# Circuit definitions included in this component
[component.circuits]
provides = ["fix_parser_circuit", "fix_order_router"]
target = ["cpu", "fpga"]                     # Execution targets

# Wire adapter metadata (category = "wire-adapter" only)
[component.wire_adapter]
protocol_family = "financial"                # messaging | financial | healthcare | industrial | general
transports = ["tcp", "tls"]                  # tcp | udp | serial | websocket | tls | quic
bidirectional = true
request_response = true

# Console widget metadata (category = "console-widget" only)
[component.widget]
widget_category = "analytics"                # analytics | monitoring | control | visualization
roles = ["operator", "admin"]                # Required RBAC roles
sizes = ["small", "medium", "large"]         # Supported widget sizes
data_sources = ["eslite"]                    # eslite | api | lex-stream

# FPGA metadata (category = "fpga-circuit" only)
[component.fpga]
device_family = ["xcvu9p", "xcvu13p"]        # Target FPGA families
resource_estimate = { luts = 50000, brams = 100, dsps = 0 }

# Files to include in the published package (glob patterns)
[component.include]
schemas = ["schemas/*.esf.yaml"]
circuits = ["circuits/*.circuit.yaml"]
tests = ["tests/golden/**"]
fpga = ["fpga/*.bit"]                        # Optional FPGA bitstreams
```

### 2.2 Name Conventions

- **Format:** lowercase alphanumeric with hyphens (`[a-z0-9-]+`)
- **Prefixes:** Official eStream components use `estream-` or `esf-` prefixes
- **Scoped names:** Third-party publishers use `@publisher/name` format (e.g., `@synergy-carbon/impact-counter`)
- **Reserved:** `estream-*` and `esf-*` prefixes are reserved for official components

### 2.3 Version Requirements

Version requirements follow Cargo/npm semver conventions:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `^1.2.3` | Compatible (default) | `>=1.2.3, <2.0.0` |
| `~1.2.3` | Patch-level changes | `>=1.2.3, <1.3.0` |
| `>=1.0.0` | Minimum version | `>=1.0.0` |
| `=1.2.3` | Exact version | `=1.2.3` |
| `>=1.0.0, <2.0.0` | Range | `>=1.0.0, <2.0.0` |
| `*` | Any version | `>=0.0.0` |

---

## 3. GitHub-Backed Registry Protocol

### 3.1 Registry Repository Layout

The registry is a GitHub repository (`estream-io/registry`) with this structure:

```
estream-io/registry/
├── README.md
├── index/
│   ├── esf-schema/
│   │   ├── esf-iot/
│   │   │   ├── metadata.json          # Component metadata (all versions)
│   │   │   └── 1.0.0/
│   │   │       ├── estream-component.toml
│   │   │       ├── SIGNATURE.ml-dsa   # ML-DSA-87 signature
│   │   │       └── checksum.sha3      # SHA3-256 of package archive
│   │   └── esf-trading/
│   │       └── ...
│   ├── wire-adapter/
│   │   ├── estream-wire-mqtt/
│   │   │   └── ...
│   │   └── estream-wire-fix/
│   │       └── ...
│   ├── smart-circuit/
│   │   └── ...
│   ├── fpga-circuit/
│   │   └── ...
│   ├── integration/
│   │   └── ...
│   └── console-widget/
│       └── ...
├── publishers/
│   ├── estream.json                   # Publisher identity + public key
│   └── synergy-carbon.json
└── config.json                        # Registry configuration
```

### 3.2 `metadata.json` Format

Each component has a `metadata.json` with version history:

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
  "stats": {
    "downloads": 0,
    "stars": 0
  }
}
```

### 3.3 `publishers/{name}.json` Format

Publisher identity with ML-DSA-87 public keys:

```json
{
  "name": "estream",
  "display_name": "eStream Contributors",
  "url": "https://github.com/toddrooke",
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

### 3.4 Version Resolution Algorithm

When resolving `estream marketplace install <component>[@version]`:

```
1. Fetch index/{category}/{name}/metadata.json
2. Parse version requirement (default: latest non-yanked)
3. Filter versions: remove yanked, check estream_min_version compatibility
4. Select highest matching version (semver sort)
5. Resolve transitive dependencies:
   a. Parse estream-component.toml for the selected version
   b. For each dependency, recursively resolve (depth-first)
   c. Detect version conflicts (two deps require incompatible versions)
   d. Detect circular dependencies
6. Download package archives in topological order
7. Verify signatures and checksums
8. Install to workspace
```

### 3.5 Local Cache

Resolved components are cached locally to avoid repeated downloads:

```
$HOME/.estream/cache/
├── registry/
│   └── index-snapshot.json            # Cached registry index
├── packages/
│   ├── estream-wire-mqtt/
│   │   └── 1.0.0/
│   │       ├── package.tar.gz
│   │       └── estream-component.toml
│   └── esf-trading/
│       └── 1.0.0/
│           └── ...
└── config.toml                        # Cache settings (TTL, max size)
```

Cache TTL defaults to 24 hours. `estream marketplace install --force` bypasses cache.

---

## 4. ML-DSA-87 Component Signing

All published components are signed with ML-DSA-87 (FIPS 204) for post-quantum integrity.

### 4.1 Signing Workflow

```
Publisher                              Registry
   │                                      │
   │  1. estream marketplace publish      │
   │     ├─ Build package archive         │
   │     ├─ Compute SHA3-256 checksum     │
   │     ├─ Sign checksum with ML-DSA-87  │
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
   │                                      │
```

### 4.2 `SIGNATURE.ml-dsa` Format

```json
{
  "algorithm": "ML-DSA-87",
  "key_id": "estream-signing-key-01",
  "signed_at": "2026-02-10T12:00:00Z",
  "checksum_algorithm": "SHA3-256",
  "checksum": "a1b2c3d4e5f6...",
  "signature_hex": "0123456789abcdef..."
}
```

The signature covers the SHA3-256 checksum of the package archive (not individual files). The checksum is computed over the deterministic tar.gz of the package contents.

### 4.3 Verification Pipeline

```rust
/// Verify a component package signature.
pub fn verify_component_signature(
    signature_file: &SignatureFile,
    archive_bytes: &[u8],
    publisher_keys: &PublisherKeys,
) -> Result<VerificationResult, VerifyError> {
    // 1. Compute SHA3-256 of archive
    let computed_checksum = estream_kernel::crypto::sha3_256(archive_bytes);

    // 2. Verify checksum matches
    if hex::encode(&computed_checksum) != signature_file.checksum {
        return Ok(VerificationResult::ChecksumMismatch);
    }

    // 3. Look up publisher's public key
    let pubkey = publisher_keys
        .get(&signature_file.key_id)
        .ok_or(VerifyError::UnknownKey(signature_file.key_id.clone()))?;

    // 4. Verify ML-DSA-87 signature over the checksum
    let signature_bytes = hex::decode(&signature_file.signature_hex)?;
    let valid = estream_kernel::crypto::verify_mldsa87(
        pubkey.as_bytes(),
        &computed_checksum,
        &signature_bytes,
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

pub enum VerificationResult {
    Valid { key_id: String, signed_at: String },
    InvalidSignature,
    ChecksumMismatch,
    ExpiredKey,
    RevokedKey,
}
```

---

## 5. CLI Commands

All commands live under `estream marketplace` (aliased as `estream mp`).

### 5.1 `estream marketplace search`

Search for components by keyword, category, or tag.

```
USAGE:
    estream marketplace search [OPTIONS] <QUERY>

ARGS:
    <QUERY>    Search query (matches name, description, keywords)

OPTIONS:
    -c, --category <CATEGORY>    Filter by category (esf-schema, wire-adapter, ...)
    -t, --tag <TAG>              Filter by keyword tag
        --verified               Only show verified publishers
        --official               Only show official eStream components
        --limit <N>              Maximum results (default: 20)
    -o, --output <FORMAT>        Output format: table (default), json
```

**Example Output:**

```
$ estream marketplace search "FIX adapter"

  NAME                  VERSION  CATEGORY       PUBLISHER   BADGES
  estream-wire-fix      1.0.0    wire-adapter   estream     ✓ Official, Tested
  fix-order-router      0.3.0    smart-circuit  acme-fin    Verified

  2 results found
```

### 5.2 `estream marketplace install`

Install a component and its dependencies into the workspace.

```
USAGE:
    estream marketplace install [OPTIONS] <COMPONENT>[@VERSION]

ARGS:
    <COMPONENT>    Component name (e.g., estream-wire-fix, esf-trading)
    @VERSION       Optional version requirement (default: latest)

OPTIONS:
        --dry-run            Show what would be installed without installing
        --force              Bypass cache, re-download everything
        --no-verify          Skip signature verification (NOT recommended)
        --save               Add to workspace estream-component.toml dependencies
    -o, --output <FORMAT>    Output format: text (default), json
```

**Example Output:**

```
$ estream marketplace install estream-wire-fix@^1.0.0

  Resolving dependencies...
    estream-wire-fix v1.0.0
    └── esf-trading v1.2.0
        ├── EStreamOrder (schema)
        └── EStreamFill (schema)

  Downloading...
    estream-wire-fix v1.0.0 ............ done (142 KB)
    esf-trading v1.2.0 ................. done (23 KB)

  Verifying ML-DSA-87 signatures...
    estream-wire-fix v1.0.0 ✓ (key: estream-signing-key-01)
    esf-trading v1.2.0 ✓ (key: estream-signing-key-01)

  Installing...
    schemas/fix-new-order-single.esf.yaml
    schemas/fix-execution-report.esf.yaml
    schemas/estream-order.esf.yaml
    schemas/estream-fill.esf.yaml
    circuits/fix-parser.circuit.yaml
    circuits/fix-order-router.circuit.yaml

  Installed 2 components (6 files) in 3.2s
```

### 5.3 `estream marketplace publish`

Publish a component to the registry.

```
USAGE:
    estream marketplace publish [OPTIONS] [PATH]

ARGS:
    [PATH]    Path to component directory (default: current directory)

OPTIONS:
        --dry-run            Validate and show what would be published
        --key <KEY_FILE>     Path to ML-DSA-87 private key file
        --registry <URL>     Registry repository URL (default: estream-io/registry)
        --skip-tests         Skip running test vectors before publishing
    -y, --yes                Skip confirmation prompt
```

**Publish Flow:**

```
$ estream marketplace publish

  Validating estream-component.toml...
    ✓ Name: estream-wire-fix
    ✓ Version: 1.0.0
    ✓ Category: wire-adapter
    ✓ Schemas: provides 3, requires 2
    ✓ Circuits: 2 definitions found

  Running test vectors...
    tests/golden/new-order.json ✓
    tests/golden/execution-report.json ✓
    2/2 tests passed

  Building package archive...
    Package size: 142 KB (7 files)

  Signing with ML-DSA-87...
    Key: estream-signing-key-01
    Checksum (SHA3-256): a1b2c3d4...
    Signature: 4627 bytes

  Publishing to estream-io/registry...
    Created PR #47: "Publish estream-wire-fix v1.0.0"

  Done! PR will be reviewed and merged by registry maintainers.
```

### 5.4 `estream marketplace verify`

Verify the ML-DSA-87 signature of an installed component.

```
USAGE:
    estream marketplace verify [OPTIONS] <COMPONENT>

ARGS:
    <COMPONENT>    Component name to verify

OPTIONS:
        --all                Verify all installed components
    -o, --output <FORMAT>    Output format: text (default), json
```

**Example Output:**

```
$ estream marketplace verify estream-wire-fix

  estream-wire-fix v1.0.0
    Signature:  ✓ Valid (ML-DSA-87)
    Key ID:     estream-signing-key-01
    Signed at:  2026-02-10T12:00:00Z
    Publisher:  estream (verified)
    Checksum:   ✓ SHA3-256 matches
```

### 5.5 `estream marketplace scaffold`

Generate a template for a new component of a given category.

```
USAGE:
    estream marketplace scaffold [OPTIONS] <CATEGORY> <NAME>

ARGS:
    <CATEGORY>    Component category (esf-schema, wire-adapter, smart-circuit, ...)
    <NAME>        Component name

OPTIONS:
        --author <NAME>      Author name
        --license <SPDX>     License (default: Apache-2.0)
    -o, --output <DIR>       Output directory (default: ./<NAME>)
```

**Example:**

```
$ estream marketplace scaffold wire-adapter estream-wire-amqp

  Created estream-wire-amqp/
  ├── estream-component.toml
  ├── README.md
  ├── schemas/
  │   └── .gitkeep
  ├── circuits/
  │   └── amqp-adapter.circuit.yaml
  ├── tests/
  │   └── golden/
  │       └── .gitkeep
  └── src/
      └── lib.rs (WireAdapter implementation skeleton)
```

---

## 6. `GitHubRegistry` Implementation

### 6.1 Trait Integration

The `GitHubRegistry` implements the existing `Registry` trait from `crates/estream-escir/src/composition/registry.rs`:

```rust
/// GitHub-backed remote component registry.
pub struct GitHubRegistry {
    /// GitHub repository (e.g., "estream-io/registry").
    repo: String,

    /// GitHub API token (optional, for higher rate limits).
    token: Option<String>,

    /// Local cache directory.
    cache_dir: PathBuf,

    /// Cached index (loaded lazily).
    index: RwLock<Option<RegistryIndex>>,

    /// HTTP client.
    client: reqwest::Client,
}

impl Registry for GitHubRegistry {
    fn save(&mut self, _component: Component) -> Result<(), RegistryError> {
        // Remote save not supported via trait — use `publish` CLI command instead.
        Err(RegistryError::IoError(
            "use `estream marketplace publish` for remote publishing".into()
        ))
    }

    fn load(&self, name: &str, version_req: &VersionReq) -> Option<&Component> {
        // Load from local cache after install.
        // Falls back to fetching from GitHub if not cached.
        self.load_cached_or_fetch(name, version_req)
    }

    fn list(&self) -> Vec<ComponentSummary> {
        self.fetch_index()
            .map(|idx| idx.list_all())
            .unwrap_or_default()
    }

    fn search(&self, query: &str) -> Vec<ComponentSummary> {
        self.fetch_index()
            .map(|idx| idx.search(query))
            .unwrap_or_default()
    }
}
```

### 6.2 Registry Index

The index is a lightweight JSON file that can be fetched in a single request:

```rust
/// In-memory representation of the registry index.
pub struct RegistryIndex {
    pub components: HashMap<String, ComponentIndexEntry>,
    pub fetched_at: SystemTime,
}

pub struct ComponentIndexEntry {
    pub name: String,
    pub category: String,
    pub publisher: String,
    pub description: String,
    pub latest_version: String,
    pub versions: Vec<VersionEntry>,
    pub keywords: Vec<String>,
    pub badges: Vec<String>,
}

pub struct VersionEntry {
    pub version: String,
    pub published_at: String,
    pub yanked: bool,
    pub archive_url: String,
    pub checksum_sha3: String,
}
```

### 6.3 Workspace Integration

After installation, components are tracked in the workspace's `estream-workspace.toml`:

```toml
# estream-workspace.toml (auto-managed)
[dependencies]
estream-wire-fix = "1.0.0"
esf-trading = "1.2.0"

[dependencies.estream-wire-fix]
version = "1.0.0"
checksum = "a1b2c3d4..."
installed_at = "2026-02-10T12:00:00Z"
```

### 6.4 Installation Targets

Components install into workspace-relative paths based on category:

| Category | Install Path |
|----------|-------------|
| `esf-schema` | `schemas/` |
| `wire-adapter` | `adapters/` |
| `smart-circuit` | `circuits/` |
| `fpga-circuit` | `fpga/` |
| `integration` | `integrations/` |
| `console-widget` | `widgets/` |

---

## 7. Publishing Flow (Detailed)

### 7.1 Pre-publish Validation

Before signing and submitting, the CLI validates:

1. **Manifest completeness** — All required fields present in `estream-component.toml`
2. **Name format** — Matches `[a-z0-9-]+` or `@[a-z0-9-]+/[a-z0-9-]+`
3. **Version increment** — Must be higher than any published version
4. **Schema validation** — All declared `provides` schemas exist in `schemas/`
5. **Dependency resolution** — All `requires` schemas are available from declared dependencies
6. **Circuit validation** — All declared circuits parse as valid ESCIR
7. **Test vectors** — If `tests/golden/` exists, all test vectors must pass
8. **File size limits** — Package archive must be < 50 MB (configurable)

### 7.2 Package Archive Format

The archive is a deterministic `tar.gz` created from the component directory. Deterministic means:
- Files sorted alphabetically
- Timestamps zeroed
- Owner/group set to `0/0`
- Permissions normalized to `644` (files) / `755` (dirs)

This ensures the SHA3-256 checksum is reproducible.

### 7.3 GitHub PR Creation

The CLI creates a PR against the registry repository containing:
- `index/{category}/{name}/{version}/estream-component.toml`
- `index/{category}/{name}/{version}/SIGNATURE.ml-dsa`
- `index/{category}/{name}/{version}/checksum.sha3`
- Updated `index/{category}/{name}/metadata.json`

The archive itself is uploaded as a GitHub Release asset (not stored in the repo).

### 7.4 CI Verification (GitHub Actions)

The registry repository includes a GitHub Actions workflow that:

1. Downloads the package archive from the PR
2. Verifies the SHA3-256 checksum matches
3. Verifies the ML-DSA-87 signature against the publisher's registered key
4. Validates the `estream-component.toml` schema
5. Checks version increment against existing metadata
6. Runs any included test vectors
7. Posts verification results as a PR comment
8. Auto-merges if all checks pass (for verified publishers)

---

## 8. Error Codes

| Code | Name | Description |
|------|------|-------------|
| `E001` | `ComponentNotFound` | No component with the given name exists |
| `E002` | `VersionNotFound` | No version matching the requirement exists |
| `E003` | `VersionConflict` | Two dependencies require incompatible versions |
| `E004` | `CircularDependency` | Dependency graph contains a cycle |
| `E005` | `SignatureInvalid` | ML-DSA-87 signature verification failed |
| `E006` | `ChecksumMismatch` | SHA3-256 checksum does not match archive |
| `E007` | `UnknownPublisher` | Publisher key not found in registry |
| `E008` | `ManifestInvalid` | `estream-component.toml` fails validation |
| `E009` | `NameReserved` | Component name uses a reserved prefix |
| `E010` | `VersionNotIncremented` | Published version is not higher than latest |
| `E011` | `PackageTooLarge` | Package archive exceeds size limit |
| `E012` | `NetworkError` | Failed to reach registry (GitHub API) |
| `E013` | `CacheCorrupted` | Local cache is corrupted, run `--force` |
| `E014` | `SchemaConflict` | Two installed components provide the same schema |
| `E015` | `PermissionDenied` | Insufficient permissions for publish |

---

## 9. Configuration

### 9.1 Global Configuration

```toml
# $HOME/.estream/config.toml

[registry]
# Default registry repository
default = "estream-io/registry"

# Additional registries (for private/enterprise components)
[[registry.sources]]
name = "enterprise"
url = "my-org/estream-registry-internal"
token_env = "ESTREAM_ENTERPRISE_TOKEN"

[cache]
# Cache directory (default: $HOME/.estream/cache)
dir = "/home/user/.estream/cache"

# Cache TTL in hours (default: 24)
ttl_hours = 24

# Maximum cache size in MB (default: 500)
max_size_mb = 500

[signing]
# Default signing key file
default_key = "/home/user/.estream/keys/signing-key.pem"
```

### 9.2 Multiple Registries

Components can be resolved from multiple registries. Resolution order:
1. Local workspace (already installed)
2. Default registry
3. Additional registries (in declaration order)

First match wins. Scoped names (`@publisher/name`) are resolved against the registry where the publisher is registered.

---

## 10. Offline Support

For air-gapped or CI environments:

```bash
# Export a component and all dependencies to a directory
estream marketplace export estream-wire-fix --output ./offline-bundle/

# Import from offline bundle
estream marketplace install --from ./offline-bundle/
```

The offline bundle contains all archives, signatures, and metadata needed for installation without network access.

---

## 11. Integration with Existing Component System

The `GitHubRegistry` extends the existing `LocalRegistry` pattern:

```
┌────────────────────────────────────────────────────────────┐
│                    Workspace                                 │
│                                                              │
│  ┌──────────────┐    ┌──────────────┐   ┌───────────────┐  │
│  │ LocalRegistry │    │GitHubRegistry│   │  Component    │  │
│  │ (in-memory)   │◀──▶│ (remote)     │──▶│  Resolution   │  │
│  └──────┬───────┘    └──────┬───────┘   └──────┬────────┘  │
│         │                   │                   │           │
│         ▼                   ▼                   ▼           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                 Registry trait                         │   │
│  │  save() | load() | list() | search()                  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└────────────────────────────────────────────────────────────┘
```

Both `LocalRegistry` and `GitHubRegistry` implement `Registry`. The `ComponentResolver` queries both, preferring local for speed and falling back to remote for discovery.

---

## References

- [COMPONENT_SYSTEM_SPEC.md](../protocol/COMPONENT_SYSTEM_SPEC.md) — Component model and versioning
- [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md) — Pricing, visibility, creator program
- [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) — Wire adapter components
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema dependency resolution
- [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) — Package format

---

*Created: 2026-02-11*  
*Status: Draft*  
*Issue: #525*
