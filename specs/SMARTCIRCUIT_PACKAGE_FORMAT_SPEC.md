# SmartCircuit Package Format Specification

> Standard packaging format for distributing reusable SmartCircuits with their schemas, test vectors, and FPGA bitstreams.

**Status:** Draft  
**Version:** 1.0.0  
**Issue:** [#527](https://github.com/polyquantum/estream-io/issues/527)  
**Parent Epic:** [#524](https://github.com/polyquantum/estream-io/issues/524)  
**Dependencies:** Component Registry API (#525), ESF Schema Composition (#526)  
**Extends:** [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md)

---

## 1. Overview

SmartCircuits are currently defined inline in workspace specs and circuit YAML files. There is no standard format for packaging a SmartCircuit with its schemas, test vectors, and optional FPGA bitstreams for distribution through the marketplace.

This spec defines:
- Package directory structure and required files
- Manifest extensions for circuit-specific metadata
- ML-DSA-87 signing of package contents (tamper-proof integrity)
- Test vector format for reproducible validation
- FPGA bitstream inclusion and verification
- Installation flow and version management

### 1.1 Design Principles

1. **Self-contained** — A package includes everything needed to use the circuit: schemas, ESCIR definitions, tests, documentation
2. **Verifiable** — Every package is ML-DSA-87 signed with a Merkle root of all contents
3. **Deterministic** — Package archives are reproducibly built for checksum verification
4. **Governance-aware** — Circuits that require governance approval declare their category
5. **Multi-target** — A single package can include CPU (WASM), FPGA, and native implementations

---

## 2. Package Directory Structure

### 2.1 Standard Layout

```
estream-wire-fix/
├── estream-component.toml           # Component manifest (required)
├── README.md                        # Documentation (recommended)
├── LICENSE                          # License file (required for publishing)
│
├── schemas/                         # ESF schema definitions
│   ├── fix-new-order.esf.yaml
│   ├── fix-execution-report.esf.yaml
│   └── fix-market-data.esf.yaml
│
├── circuits/                        # ESCIR circuit definitions (required)
│   ├── fix-parser.circuit.yaml      # Main circuit
│   └── fix-order-router.circuit.yaml
│
├── src/                             # Rust source (optional, for reference impl)
│   └── lib.rs
│
├── tests/                           # Test vectors and specs
│   ├── golden/                      # Golden reference test vectors
│   │   ├── new-order-basic.json
│   │   ├── new-order-complex.json
│   │   ├── execution-report.json
│   │   └── manifest.toml            # Test vector manifest
│   └── journeys/                    # Journey test definitions (optional)
│       └── fix-parser-journey.toml
│
├── fpga/                            # FPGA bitstreams (optional, licensed)
│   ├── fix-parser-xcvu9p.bit
│   ├── fix-parser-xcvu13p.bit
│   └── manifest.toml                # FPGA target manifest
│
├── docs/                            # Additional documentation (optional)
│   └── integration-guide.md
│
└── SIGNATURE.ml-dsa                 # ML-DSA-87 package signature (required)
```

### 2.2 Minimum Viable Package

The minimum required files for a valid package:

```
my-circuit/
├── estream-component.toml
├── circuits/
│   └── my-circuit.circuit.yaml
└── SIGNATURE.ml-dsa
```

---

## 3. Manifest Extensions

The `estream-component.toml` for a SmartCircuit package extends the base manifest (defined in [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md)) with circuit-specific fields.

### 3.1 Circuit Metadata

```toml
[component]
name = "estream-wire-fix"
version = "1.0.0"
category = "smart-circuit"              # or "wire-adapter" for adapter packages
description = "FIX protocol parser and order router"
license = "Apache-2.0"
keywords = ["fix", "trading", "capital-markets"]

[component.author]
name = "eStream Contributors"
url = "https://github.com/toddrooke"

# Circuit-specific metadata
[component.circuit]
# Primary circuit entry point
entry = "circuits/fix-parser.circuit.yaml"

# All circuits in this package
circuits = [
    "circuits/fix-parser.circuit.yaml",
    "circuits/fix-order-router.circuit.yaml",
]

# Execution targets
targets = ["cpu", "fpga"]                # cpu | fpga | wasm | native

# Witness tier requirement
witness_tier = 2                         # 0-4 (see witness spec)

# Resource budget estimates (8D metering)
[component.circuit.resources]
compute_budget = 5000                    # Compute units per execution
memory_bytes = 16384                     # Peak memory usage
storage_bytes = 0                        # Persistent storage per execution
estimated_cost_es = "0.005"              # Estimated ES cost per execution

# Governance requirements
[component.circuit.governance]
category = "Generic"                     # Generic | InfraOps | MpcThresholdDecrypt |
                                         # AiModelDeployment | AiCriticalExecution
required_approvals = 1                   # Number of governance approvals to deploy
auto_approve = true                      # Whether the circuit can be auto-approved
                                         # (only for Generic category)

# FPGA metadata (if targets includes "fpga")
[component.circuit.fpga]
device_families = ["xcvu9p", "xcvu13p"]
resource_estimate = { luts = 50000, brams = 100, dsps = 0 }
clock_mhz = 250
pipeline_stages = 8

# Dependencies
[component.dependencies]
esf-trading = "^1.0.0"

[component.schemas]
provides = ["FixNewOrderSingle", "FixExecutionReport", "FixMarketData"]
requires = ["EStreamOrder", "EStreamFill"]
```

---

## 4. Circuit YAML Format

Circuit files follow the ESCIR specification. Each circuit YAML must be a valid ESCIR document:

```yaml
# circuits/fix-parser.circuit.yaml
escir_version: "0.8.0"
name: fix_parser
version: "1.0.0"

description: "Parses FIX 4.2/4.4/5.0 messages into ESF format"

types:
  - name: FixRawMessage
    fields:
      - { name: tag_value_pairs, type: "bytes" }
      - { name: fix_version, type: "u8" }
      - { name: msg_type, type: "string" }

inputs:
  - name: raw_fix
    type: FixRawMessage
    description: "Raw FIX message bytes"

outputs:
  - name: parsed_esf
    type: EsfMessage
    description: "Parsed and translated ESF message"
  - name: parse_error
    type: FixParseError
    description: "Parse errors for malformed messages"

annotations:
  witness_tier: 2
  streamsight_emit: true
  hardware_target: [cpu, fpga]
  marketplace:
    category: wire-adapter
    publisher: estream

compute:
  - id: tag_splitter
    type: transform
    description: "Split FIX tag=value pairs"

  - id: field_validator
    type: transform
    description: "Validate required fields per msg_type"

  - id: esf_translator
    type: transform
    description: "Map FIX fields to ESF schema"

flows:
  - raw_fix -> tag_splitter -> field_validator -> esf_translator -> parsed_esf
  - field_validator.error -> parse_error

invariants:
  - "output_count == input_count"
  - "no_data_loss: every input produces either parsed_esf or parse_error"
```

---

## 5. Test Vector Format

### 5.1 Golden Test Vectors

Test vectors are JSON files with input/output pairs. They enable reproducible validation of circuit behavior.

```json
{
  "test_vector": {
    "name": "new-order-basic",
    "description": "Basic FIX NewOrderSingle with required fields only",
    "circuit": "fix-parser",
    "version": "1.0.0"
  },
  "input": {
    "raw_fix": {
      "tag_value_pairs": "8=FIX.4.4|9=120|35=D|49=SENDER|56=TARGET|11=ORD001|55=AAPL|54=1|38=100|40=2|44=150.50|10=123|",
      "fix_version": 4,
      "msg_type": "D"
    }
  },
  "expected_output": {
    "parsed_esf": {
      "message_type": "FixNewOrderSingle",
      "fields": {
        "cl_ord_id": "ORD001",
        "symbol": "AAPL",
        "side": "Buy",
        "quantity": 100,
        "order_type": "Limit",
        "price": 150.50
      }
    }
  },
  "metadata": {
    "tags": ["basic", "new-order", "fix-4.4"],
    "created_at": "2026-02-10T12:00:00Z"
  }
}
```

### 5.2 Test Vector Manifest

```toml
# tests/golden/manifest.toml

[test_suite]
name = "fix-parser-golden"
circuit = "fix-parser"
version = "1.0.0"

[[tests]]
file = "new-order-basic.json"
tags = ["basic", "new-order"]
expected = "pass"

[[tests]]
file = "new-order-complex.json"
tags = ["complex", "new-order", "optional-fields"]
expected = "pass"

[[tests]]
file = "execution-report.json"
tags = ["execution-report"]
expected = "pass"

[[tests]]
file = "malformed-message.json"
tags = ["error-handling", "negative"]
expected = "error"
expected_error = "parse_error"
```

### 5.3 Running Test Vectors

```bash
# Run all test vectors in a package
estream marketplace test ./estream-wire-fix

# Run specific test tags
estream marketplace test ./estream-wire-fix --tags basic

# Run tests with verbose output
estream marketplace test ./estream-wire-fix --verbose
```

**Example Output:**

```
$ estream marketplace test ./estream-wire-fix

  Running test suite: fix-parser-golden (4 tests)

  ✓ new-order-basic .............. PASS (2ms)
  ✓ new-order-complex ........... PASS (3ms)
  ✓ execution-report ............ PASS (2ms)
  ✓ malformed-message ........... PASS (error expected, got parse_error)

  4/4 tests passed in 9ms
```

### 5.4 Journey Test Integration

Packages can include journey test definitions compatible with the `crates/estream-test/` framework:

```toml
# tests/journeys/fix-parser-journey.toml

[journey]
name = "fix_parser_lifecycle"
category = "WireAdapter"
tags = ["Integration", "Wire"]

[[steps]]
name = "parse_basic_new_order"
action = "ExecuteCircuit"
input_file = "../golden/new-order-basic.json"
expected = "pass"

[[steps]]
name = "parse_complex_order"
action = "ExecuteCircuit"
input_file = "../golden/new-order-complex.json"
expected = "pass"

[[steps]]
name = "handle_malformed_input"
action = "ExecuteCircuit"
input_file = "../golden/malformed-message.json"
expected = "error"
```

---

## 6. ML-DSA-87 Package Signing

### 6.1 Signing Process

Package signing uses a Merkle tree of all file contents:

```
1. Enumerate all files in the package (sorted alphabetically)
2. Compute SHA3-256 hash of each file
3. Build Merkle tree of file hashes
4. Sign the Merkle root with ML-DSA-87
5. Write SIGNATURE.ml-dsa with:
   - Merkle root
   - ML-DSA-87 signature
   - File manifest (path → hash)
   - Key ID and timestamp
```

### 6.2 `SIGNATURE.ml-dsa` Format

```json
{
  "format_version": 1,
  "algorithm": "ML-DSA-87",
  "key_id": "estream-signing-key-01",
  "signed_at": "2026-02-10T12:00:00Z",

  "merkle_root": "sha3-256:a1b2c3d4e5f6...",

  "files": [
    {
      "path": "estream-component.toml",
      "hash": "sha3-256:1234abcd...",
      "size": 1024
    },
    {
      "path": "circuits/fix-parser.circuit.yaml",
      "hash": "sha3-256:5678efgh...",
      "size": 2048
    },
    {
      "path": "schemas/fix-new-order.esf.yaml",
      "hash": "sha3-256:9012ijkl...",
      "size": 512
    }
  ],

  "signature_hex": "0123456789abcdef..."
}
```

### 6.3 Verification

```rust
/// Verify a SmartCircuit package signature.
pub fn verify_package(
    package_dir: &Path,
    publisher_keys: &PublisherKeys,
) -> Result<PackageVerification, VerifyError> {
    // 1. Read SIGNATURE.ml-dsa
    let sig_file = read_signature(package_dir)?;

    // 2. Verify each file hash
    let mut file_hashes = Vec::new();
    for entry in &sig_file.files {
        let file_path = package_dir.join(&entry.path);
        let content = std::fs::read(&file_path)?;
        let hash = estream_kernel::crypto::sha3_256(&content);
        let hash_hex = format!("sha3-256:{}", hex::encode(&hash));

        if hash_hex != entry.hash {
            return Ok(PackageVerification::FileModified {
                path: entry.path.clone(),
                expected: entry.hash.clone(),
                actual: hash_hex,
            });
        }
        file_hashes.push(hash);
    }

    // 3. Recompute Merkle root
    let computed_root = compute_merkle_root(&file_hashes);
    let root_hex = format!("sha3-256:{}", hex::encode(&computed_root));
    if root_hex != sig_file.merkle_root {
        return Ok(PackageVerification::MerkleRootMismatch);
    }

    // 4. Verify ML-DSA-87 signature over Merkle root
    let pubkey = publisher_keys
        .get(&sig_file.key_id)
        .ok_or(VerifyError::UnknownKey(sig_file.key_id.clone()))?;

    let signature_bytes = hex::decode(&sig_file.signature_hex)?;
    let valid = estream_kernel::crypto::verify_mldsa87(
        pubkey.as_bytes(),
        &computed_root,
        &signature_bytes,
    );

    if valid {
        Ok(PackageVerification::Valid {
            key_id: sig_file.key_id,
            signed_at: sig_file.signed_at,
            file_count: sig_file.files.len(),
        })
    } else {
        Ok(PackageVerification::InvalidSignature)
    }
}

/// Compute Merkle root from file hashes.
fn compute_merkle_root(hashes: &[[u8; 32]]) -> [u8; 32] {
    if hashes.is_empty() {
        return [0u8; 32];
    }
    if hashes.len() == 1 {
        return hashes[0];
    }

    let mut level: Vec<[u8; 32]> = hashes.to_vec();
    while level.len() > 1 {
        let mut next = Vec::new();
        for chunk in level.chunks(2) {
            let mut combined = Vec::with_capacity(64);
            combined.extend_from_slice(&chunk[0]);
            if chunk.len() > 1 {
                combined.extend_from_slice(&chunk[1]);
            } else {
                combined.extend_from_slice(&chunk[0]); // Duplicate last if odd
            }
            next.push(estream_kernel::crypto::sha3_256(&combined));
        }
        level = next;
    }
    level[0]
}
```

### 6.4 Tamper Detection

If any file is modified after signing:
- Individual file hash check fails → identifies the exact modified file
- Merkle root mismatch → fast detection without checking every file
- Signature verification failure → detects signature forgery

---

## 7. FPGA Bitstream Inclusion

### 7.1 Bitstream Manifest

```toml
# fpga/manifest.toml

[fpga]
# FPGA implementation version (must match circuit version)
circuit_version = "1.0.0"

[[fpga.targets]]
device_family = "xcvu9p"
bitstream = "fix-parser-xcvu9p.bit"
checksum = "sha3-256:abcd1234..."
clock_mhz = 250
resource_usage = { luts = 48532, brams = 96, dsps = 0 }
verified = true
verified_by = "estream-fpga-team"
verified_at = "2026-02-08T10:00:00Z"

[[fpga.targets]]
device_family = "xcvu13p"
bitstream = "fix-parser-xcvu13p.bit"
checksum = "sha3-256:efgh5678..."
clock_mhz = 300
resource_usage = { luts = 48532, brams = 96, dsps = 0 }
verified = true
verified_by = "estream-fpga-team"
verified_at = "2026-02-08T10:00:00Z"
```

### 7.2 Licensing

FPGA bitstreams may have a different license than the circuit definition:

```toml
[component]
license = "Apache-2.0"                  # Circuit definition license

[component.circuit.fpga]
license = "BSL-1.1"                     # FPGA bitstream license (may differ)
```

This follows the Chronicle model: open-source behavior definitions (ESCIR) with licensed hardware acceleration (FPGA bitstreams).

### 7.3 Bitstream Verification

FPGA bitstreams are included in the Merkle tree signing. Additionally, the FPGA manifest includes per-target checksums for independent verification:

```bash
$ estream marketplace verify estream-wire-fix --fpga

  Package signature: ✓ Valid
  
  FPGA bitstreams:
    fix-parser-xcvu9p.bit .... ✓ SHA3-256 match (48,532 LUTs, 250 MHz)
    fix-parser-xcvu13p.bit ... ✓ SHA3-256 match (48,532 LUTs, 300 MHz)
    
  Verified by: estream-fpga-team (2026-02-08)
```

---

## 8. Installation Flow

### 8.1 Install Process

```
$ estream marketplace install estream-wire-fix

  1. Resolve dependencies (see COMPONENT_REGISTRY_API_SPEC.md)
     └── esf-trading v1.2.0

  2. Download package archives
     ├── estream-wire-fix v1.0.0 (142 KB)
     └── esf-trading v1.2.0 (23 KB)

  3. Verify ML-DSA-87 signatures
     ├── estream-wire-fix: ✓ Valid (Merkle root + file hashes)
     └── esf-trading: ✓ Valid

  4. Resolve ESF schemas (see ESF_SCHEMA_COMPOSITION_SPEC.md)
     ├── provides: FixNewOrderSingle, FixExecutionReport, FixMarketData
     └── requires: EStreamOrder ✓, EStreamFill ✓

  5. Install files to workspace
     ├── circuits/fix-parser.circuit.yaml
     ├── circuits/fix-order-router.circuit.yaml
     ├── schemas/fix-new-order.esf.yaml
     ├── schemas/fix-execution-report.esf.yaml
     ├── schemas/fix-market-data.esf.yaml
     └── schemas/estream-order.esf.yaml (from esf-trading)

  6. Run test vectors
     └── 4/4 tests passed

  7. Update workspace tracking
     └── estream-workspace.toml updated
```

### 8.2 Installation Targets

| File Type | Install Location | Overwrite Policy |
|-----------|-----------------|------------------|
| Circuit YAML | `circuits/` | Fail if exists (use `--force`) |
| ESF schemas | `schemas/` | Fail if exists (use `--force`) |
| FPGA bitstreams | `fpga/` | Always install (versioned by filename) |
| Test vectors | `tests/vendor/{name}/` | Always overwrite |
| Documentation | `docs/vendor/{name}/` | Always overwrite |

### 8.3 Version Pinning

Installed versions are pinned in `estream-workspace.toml`:

```toml
[dependencies]
estream-wire-fix = "=1.0.0"             # Pinned after install
esf-trading = "=1.2.0"

[lock]
# Lock file for reproducible installs
estream-wire-fix = { version = "1.0.0", checksum = "sha3-256:a1b2c3..." }
esf-trading = { version = "1.2.0", checksum = "sha3-256:d4e5f6..." }
```

### 8.4 Upgrade Flow

```bash
# Check for updates
estream marketplace outdated

# Upgrade a specific component
estream marketplace upgrade estream-wire-fix

# Upgrade all components
estream marketplace upgrade --all
```

Upgrades follow these rules:
- **Patch upgrades** (1.0.0 → 1.0.1): Auto-applied, backward compatible
- **Minor upgrades** (1.0.0 → 1.1.0): Applied with confirmation, new optional features
- **Major upgrades** (1.0.0 → 2.0.0): Require `--force`, may include breaking changes

---

## 9. Governance Integration

### 9.1 Circuit Categories and Approval

Circuits declare their governance category in the manifest. The category determines the approval workflow:

| Category | Auto-Approve | Required Approvals | Review |
|----------|-------------|-------------------|--------|
| `Generic` | Yes | 0 | Automated only |
| `InfraOps` | No | 1 | Ops team review |
| `MpcThresholdDecrypt` | No | 2 | Security team review |
| `AiModelDeployment` | No | 2 | AI safety review |
| `AiCriticalExecution` | No | 3 | Full governance vote |

### 9.2 Deployment Approval Flow

When a marketplace circuit is deployed to a production lex:

```
1. Circuit installed from marketplace (already ML-DSA-87 verified)
2. Deployment request created:
   - circuit_id, lex_id, deployer
   - CircuitCategory from manifest
3. Governance check:
   a. If auto_approve: deployed immediately
   b. If not: approval request sent to governance members
   c. Required approvals collected (ML-DSA-87 signed)
4. Deployment executed with witness
```

This integrates with the existing `CircuitApproval` system in `crates/estream-governance/src/circuit_approval.rs`.

---

## 10. Scaffolding

The `estream marketplace scaffold smart-circuit <name>` command generates a complete package template:

```bash
$ estream marketplace scaffold smart-circuit my-validator

  Created my-validator/
  ├── estream-component.toml          # Pre-filled manifest
  ├── README.md                       # Template readme
  ├── LICENSE                         # Apache-2.0
  ├── circuits/
  │   └── my-validator.circuit.yaml   # ESCIR skeleton
  ├── schemas/
  │   └── .gitkeep
  ├── tests/
  │   └── golden/
  │       ├── manifest.toml           # Empty test manifest
  │       └── example-test.json       # Example test vector
  └── fpga/
      └── .gitkeep

  Next steps:
    1. Edit circuits/my-validator.circuit.yaml
    2. Add ESF schemas to schemas/
    3. Write test vectors in tests/golden/
    4. Run: estream marketplace test ./my-validator
    5. Publish: estream marketplace publish ./my-validator
```

---

## References

- [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) — Registry, manifest, CLI
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema dependencies
- [ESCIR_LANGUAGE_SPEC.md](../protocol/ESCIR_LANGUAGE_SPEC.md) — Circuit YAML format
- [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md) — Pricing, visibility, creator program
- [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) — Wire adapter packages
- [COMPONENT_SYSTEM_SPEC.md](../protocol/COMPONENT_SYSTEM_SPEC.md) — Component model

---

*Created: 2026-02-11*  
*Status: Draft*  
*Issue: #527*
