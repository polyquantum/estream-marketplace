# .escx Package Format Specification

> Binary format specification for compiled eStream domain packages.

**Status:** Draft
**Version:** 1.0.0
**Epic:** estream-marketplace#8 (Documentation & Standards)
**Source:** `registry/package_format.fl`

---

## 1. Overview

The `.escx` (eStream Compiled eXtensible) format is the opaque binary package format for distributing compiled domain circuits through the eStream Marketplace. Packages are not reverse-engineerable — the compiler produces position-independent ESCIR bytecode with symbol stripping.

### 1.1 Design Goals

| Goal | Description |
|------|-------------|
| **Opaque** | No reverse engineering — symbol-stripped, position-independent bytecode |
| **Attested** | PoVC witness proves authentic compilation |
| **Signed** | ML-DSA-87 signature by publisher PRIME identity |
| **Deterministic** | Reproducible archive — same source always produces same checksum |
| **Streamable** | Sections are independently addressable for partial downloads |

---

## 2. Binary Format Layout

### 2.1 Top-Level Structure

```
┌─────────────────────────────────────────────┐
│  Magic Bytes (8 bytes)                      │
│  Version Header (16 bytes)                  │
│  Section Table (variable)                   │
│  ──────────────────────────────────────────  │
│  Section 0: manifest                        │
│  Section 1: domain.escx                     │
│  Section 2: lsp.escx                        │
│  Section 3: docs                            │
│  Section 4: tests                           │
│  Section 5: LICENSE                         │
│  Section 6: attestation.q                   │
└─────────────────────────────────────────────┘
```

### 2.2 Magic Bytes

```
Offset  Size  Value              Description
0x00    4     0x45 0x53 0x43 0x58  ASCII "ESCX"
0x04    2     0x00 0x01            Format version (major.minor)
0x06    2     0x00 0x00            Reserved (must be zero)
```

The magic bytes `ESCX` (0x45534358) identify the file as an eStream compiled package. Readers must reject files that do not begin with this sequence.

### 2.3 Version Header

```
Offset  Size  Type    Description
0x08    2     u16     Format version major
0x0A    2     u16     Format version minor
0x0C    4     u32     Total file size in bytes
0x10    2     u16     Section count
0x12    2     u16     Flags (see §2.4)
0x14    4     u32     Header checksum (CRC32 of bytes 0x00–0x13)
```

### 2.4 Flags

| Bit | Name | Description |
|-----|------|-------------|
| 0 | `HAS_FPGA` | Contains FPGA bitstream section |
| 1 | `HAS_TESTS` | Contains test vector section |
| 2 | `HAS_DOCS` | Contains documentation section |
| 3 | `HAS_LSP` | Contains LSP metadata section |
| 4 | `STRIPPED` | Symbols have been stripped |
| 5–15 | Reserved | Must be zero |

### 2.5 Section Table

Immediately follows the version header. Each entry is 32 bytes:

```
Offset  Size  Type       Description
0x00    4     u32        Section type ID (see §3)
0x04    4     u32        Section offset (from file start)
0x08    4     u32        Section size (compressed)
0x0C    4     u32        Section size (uncompressed)
0x10    1     u8         Compression type (0=none, 1=zstd, 2=lz4)
0x11    1     u8         Section flags
0x12    2     u16        Reserved
0x14    12    bytes(12)  Section SHA3-256 checksum (truncated to 96 bits)
```

---

## 3. Sections

### 3.1 Section Type IDs

| ID | Name | Required | Description |
|----|------|----------|-------------|
| `0x0001` | `manifest` | Yes | Package manifest (TOML, see §4) |
| `0x0002` | `domain.escx` | Yes | Compiled opaque ESCIR bytecode |
| `0x0003` | `lsp.escx` | No | LSP metadata for IDE integration |
| `0x0004` | `docs` | No | Documentation bundle |
| `0x0005` | `tests` | No | Golden test vectors |
| `0x0006` | `LICENSE` | Yes | License text (SPDX-identified) |
| `0x0007` | `attestation.q` | Yes | PoVC compilation attestation |
| `0x0008` | `fpga` | No | FPGA bitstream bundle |
| `0x0009` | `widgets` | No | Console widget ES module bundle |

### 3.2 `manifest` (0x0001)

The manifest section contains the full `manifest.toml` in TOML format. See [MANIFEST_SCHEMA_SPEC.md](MANIFEST_SCHEMA_SPEC.md) for the schema. The manifest is always stored uncompressed for quick extraction without decompressing the entire package.

### 3.3 `domain.escx` (0x0002)

The compiled domain artifact. This section contains:

```
┌──────────────────────────────────────┐
│  ESCIR Header (32 bytes)             │
│    - ESCIR API version (12 bytes)    │
│    - Bytecode format version (4)     │
│    - Entry point count (4)           │
│    - Symbol table offset (4)         │
│    - Relocation table offset (4)     │
│    - Flags (4)                       │
│  ────────────────────────────────────│
│  Bytecode Segments                   │
│    - Position-independent ESCIR ops  │
│    - No absolute addresses           │
│    - No debug symbols                │
│  ────────────────────────────────────│
│  Entry Point Table                   │
│    - Circuit name hashes             │
│    - Bytecode offsets                │
│    - Input/output port descriptors   │
│  ────────────────────────────────────│
│  Relocation Table (stripped)         │
│    - Only contains type signatures   │
│    - No internal symbol names        │
└──────────────────────────────────────┘
```

**Opaque compilation guarantees:**

| Property | Description |
|----------|-------------|
| Symbol stripping | All internal variable names, function names, and comments removed |
| Position-independent | No absolute addresses — relocatable by the runtime |
| No source mapping | No source-to-bytecode mappings included |
| Entry points only | Only public circuit entry points are named (by hash) |
| Deterministic | Same source + same compiler version = same bytecode |

### 3.4 `lsp.escx` (0x0003)

LSP (Language Server Protocol) metadata for IDE integration:

- Circuit signatures (name, input types, output types)
- Port descriptions
- Documentation strings (from `///` comments)
- Autocompletion hints
- Resource requirements

This section enables IDE features (hover docs, autocompletion, go-to-definition) without exposing implementation details.

### 3.5 `docs` (0x0004)

Documentation bundle containing:

- `README.md` (required if section present)
- Additional markdown files
- API reference (auto-generated from LSP metadata)

Stored as a tar archive (files sorted alphabetically, timestamps zeroed).

### 3.6 `tests` (0x0005)

Golden test vectors:

- `manifest.toml` — test suite metadata
- `*.json` — individual test vector files

Format matches the test vector specification in [ESTREAM_MARKETPLACE_SPEC.md §4.3](../ESTREAM_MARKETPLACE_SPEC.md).

### 3.7 `LICENSE` (0x0006)

Plain-text license file. The SPDX identifier in the manifest must match the license text.

### 3.8 `attestation.q` (0x0007)

PoVC (Proof of Verifiable Computation) attestation. See §5.

---

## 4. Manifest Schema

The manifest section contains a TOML document conforming to the full schema defined in [MANIFEST_SCHEMA_SPEC.md](MANIFEST_SCHEMA_SPEC.md).

### 4.1 Required Manifest Fields (Summary)

| Section | Key Fields |
|---------|------------|
| `[package]` | `name`, `version`, `description`, `license`, `repository` |
| `[publisher]` | `id`, `name`, `signing_key_id` |
| `[escir]` | `api_version`, `min_platform_version` |
| `[dependencies]` | Package name = version requirement pairs |
| `[pricing]` | `license_type`, `share_price`, `private_price` |
| `[telemetry]` | `schema_hash`, `metrics`, `aggregate_window`, `cohort_min_size` |

### 4.2 Pricing Tiers in Manifest

```toml
[pricing]
license_type = "per-invocation"
share_price = "0.0004"
private_price = "0.0006"
```

The `share_price` and `private_price` correspond to the Share and Private tiers defined in `licensing/pricing_tiers.fl`.

---

## 5. Attestation Format

### 5.1 `attestation.q` Structure

The attestation section contains a `PackageAttestation` record (from `registry/package_format.fl`):

```
┌──────────────────────────────────────────────┐
│  Attestation Header                          │
│    package_id: bytes(32)                     │
│    version: bytes(20)                        │
│    attestation_type: u8                      │
│        0 = CompilerPoVC                      │
│        1 = PlatformVerification              │
│        2 = CommunityAudit                    │
│  ──────────────────────────────────────────  │
│  Compilation Proof                           │
│    compiler_version: bytes(12)               │
│    escir_api_version: bytes(12)              │
│    source_hash: bytes(32)   [SHA3-256]       │
│    compiled_hash: bytes(32) [SHA3-256]       │
│  ──────────────────────────────────────────  │
│  Witness                                     │
│    attestation_hash: bytes(32) [SHA3-256]    │
│    witness_signature: bytes(4627) [ML-DSA-87]│
│    attester_id: bytes(32)                    │
│    attested_at: u64 [Unix timestamp]         │
└──────────────────────────────────────────────┘
```

### 5.2 PoVC Witness

The PoVC (Proof of Verifiable Computation) witness proves:

| Claim | Proof |
|-------|-------|
| Authentic compiler | `attester_id` is a registered eStream compiler PRIME identity |
| Source fidelity | `source_hash` matches the SHA3-256 of the original `.fl` source |
| Compiled fidelity | `compiled_hash` matches the SHA3-256 of the `domain.escx` section |
| Determinism | Same source + same compiler = same `compiled_hash` |
| Timestamp | `attested_at` is MTP-timestamped |

### 5.3 Attestation Verification

The `verify_package_attestation` circuit checks:

1. `attestation.package_id == manifest.package_id`
2. `attestation.version == manifest.version`
3. `attestation.compiled_hash == manifest.compiled_hash`
4. ML-DSA-87 signature over `attestation_hash` is valid against `attester_id`'s public key

---

## 6. Signing Requirements

### 6.1 Publisher Signature

Every `.escx` package must be signed by the publisher's ML-DSA-87 private key (FIPS 204, post-quantum secure).

### 6.2 Signing Process

```
1. List all sections in the package (sorted by section type ID)
2. Compute SHA3-256 hash of each section's content
3. Build a Merkle tree from the section hashes
4. Sign the Merkle root with ML-DSA-87 private key
5. Embed signature in the package trailer
```

### 6.3 Package Trailer

The signature is appended after the last section:

```
Offset  Size   Type         Description
0x00    32     bytes(32)    Merkle root (SHA3-256)
0x04    4627   bytes(4627)  ML-DSA-87 signature
0x1217  64     bytes(64)    Signer key ID
0x1257  8      u64          Signed-at timestamp
0x125F  1      u8           Signature version (0x01)
```

### 6.4 ML-DSA-87 Parameters

| Parameter | Value |
|-----------|-------|
| Algorithm | ML-DSA-87 (FIPS 204) |
| Security level | NIST Level 5 (post-quantum) |
| Public key size | 1,952 bytes |
| Signature size | 4,627 bytes |
| Hash function | SHA3-256 (FIPS 202) |

### 6.5 Key Management

- Publisher keys are registered in the marketplace registry at `publishers/{name}.json`
- Keys have creation and expiration dates
- Key rotation requires signing the new key with the old key
- Revoked keys are listed in the registry; packages signed with revoked keys are flagged

---

## 7. Opaque Compilation

### 7.1 Properties

The `.escx` compiled output is designed to be opaque:

| Property | Implementation |
|----------|---------------|
| **Symbol stripping** | All internal identifiers replaced with positional indices |
| **No source maps** | No mapping from bytecode back to source lines |
| **Position-independent** | All addresses are relative — no absolute pointers |
| **Constant folding** | Compile-time expressions evaluated and inlined |
| **Dead code elimination** | Unreachable code paths removed |
| **Obfuscated control flow** | Basic block ordering randomized (deterministic seed from source hash) |

### 7.2 What IS Visible

Even in opaque packages, the following is always public:

| Visible | Source |
|---------|--------|
| Package name and version | Manifest |
| Circuit entry point names (hashed) | Entry point table |
| Input/output port types | Entry point table |
| Resource requirements | Manifest + LSP metadata |
| Estimated cost per execution | Manifest |
| Source hash | Manifest (proves source exists) |

### 7.3 What IS NOT Visible

| Not Visible | Protection |
|-------------|-----------|
| Internal variable names | Symbol stripping |
| Algorithm implementation | Bytecode obfuscation |
| Internal circuit structure | Dead code elimination + control flow obfuscation |
| Comments and documentation strings | Stripped (except `///` doc comments in LSP section) |
| Source code | Not included in `domain.escx` |

### 7.4 Visibility Tiers

The `PackageVisibility` type controls what customers can access:

| Tier | Value | What's Accessible |
|------|-------|-------------------|
| Public | 0 | Full ESCIR source visible |
| Unlisted | 1 | Not in search results, but full access via direct link |
| Private | 2 | Only accessible to specific customers via license |
| Solution-Only | 3 | Only accessible as part of a solution bundle |

---

## 8. Deterministic Archive

### 8.1 Reproducibility Rules

The published `.escx` package must be deterministic:

| Rule | Description |
|------|-------------|
| Section ordering | Sections ordered by type ID (ascending) |
| File ordering | Files within sections sorted alphabetically |
| Timestamps zeroed | All embedded timestamps set to 0 (except `attested_at`) |
| Permissions normalized | 644 for files, 755 for directories |
| Owner/group zeroed | UID/GID set to 0 |
| Compression deterministic | zstd with fixed dictionary and compression level |

### 8.2 Reproducible Builds

Given the same:
- Source `.fl` files
- Compiler version (`compiler_version` in attestation)
- ESCIR API version (`escir_api_version`)

The resulting `domain.escx` section will have the identical SHA3-256 `compiled_hash`. This enables third-party verification that a published package matches its claimed source.

---

## References

- [MANIFEST_SCHEMA_SPEC.md](MANIFEST_SCHEMA_SPEC.md) — Full manifest TOML schema
- [PRIVACY_GUARANTEES_SPEC.md](PRIVACY_GUARANTEES_SPEC.md) — Privacy guarantees for package distribution
- [ESTREAM_MARKETPLACE_SPEC.md §4](../ESTREAM_MARKETPLACE_SPEC.md) — Package format overview
- [ESTREAM_MARKETPLACE_SPEC.md §7](../ESTREAM_MARKETPLACE_SPEC.md) — ML-DSA-87 signing protocol
- `registry/package_format.fl` — FastLang type definitions for package format
- `licensing/blinded_tokens.fl` — Blinded license token protocol

---

*Created: February 2026*
*Status: Draft*
