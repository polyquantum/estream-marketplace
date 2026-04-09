# Security Model

> Post-quantum security architecture for the eStream Marketplace.

---

## Overview

Every component published to the eStream Marketplace is cryptographically signed with **ML-DSA-87** (FIPS 204), a post-quantum digital signature algorithm based on lattice problems. This ensures that components remain tamper-proof even against future quantum computing attacks.

---

## ML-DSA-87 Post-Quantum Signatures

### Why ML-DSA-87?

ML-DSA-87 (formerly CRYSTALS-Dilithium) is a NIST-standardized post-quantum signature scheme:

| Property | Value |
|----------|-------|
| Standard | FIPS 204 (2024) |
| Security Level | NIST Level 5 (256-bit classical, 128-bit quantum) |
| Public Key Size | 2,592 bytes |
| Signature Size | 4,627 bytes |
| Signing Speed | ~150 μs |
| Verification Speed | ~50 μs |
| Assumption | Module Lattice (MLWE/MSIS) |

### Platform Crypto Boundary

The eStream platform crypto surface is **exclusively PRIME** — SHA-3 (KECCAK), lattice-based (ML-DSA, ML-KEM), and PRIME-derived primitives. No non-PRIME hash functions (blake3, SHA-256, etc.) are used in platform code.

---

## Component Verification Flow

Every component goes through a verification pipeline during publish and install:

```
                    PUBLISH                                    INSTALL
                    
  ┌─────────────┐    ┌──────────────┐    ┌────────────┐    ┌─────────────┐
  │  Validate    │───▶│  Build       │───▶│  Sign      │───▶│  Registry   │
  │  Manifest    │    │  Archive     │    │  ML-DSA-87 │    │  Store      │
  └─────────────┘    └──────────────┘    └────────────┘    └──────┬──────┘
                                                                   │
                                          ┌─────────────────┐     │
                                          │  User installs  │◀────┘
                                          └────────┬────────┘
                                                   │
                                          ┌────────▼────────┐
                                          │  Download       │
                                          │  Archive + Sig  │
                                          └────────┬────────┘
                                                   │
                                          ┌────────▼────────┐
                                          │  Verify         │
                                          │  ML-DSA-87 Sig  │
                                          └────────┬────────┘
                                                   │
                                      ┌────────────┴────────────┐
                                      │                         │
                                ┌─────▼─────┐           ┌──────▼──────┐
                                │  PASS     │           │  FAIL       │
                                │  Install  │           │  Reject     │
                                └───────────┘           └─────────────┘
```

### Publish Steps

1. **Manifest validation**: All required fields present, valid category/pricing/visibility
2. **Include resolution**: All glob patterns resolve to real files
3. **FastLang checking**: `.fl` files pass `compile --check`
4. **Deterministic archive**: Build reproducible `tar.gz` (sorted entries, fixed timestamps)
5. **ML-DSA-87 signing**: Sign the archive hash with the publisher's private key
6. **Signature emission**: Write `SIGNATURE.ml-dsa` with algorithm, fingerprint, and timestamp

### Install Steps

1. **Download**: Fetch the archive and `SIGNATURE.ml-dsa` from the registry
2. **Signature parse**: Extract algorithm, fingerprint, and timestamp
3. **ML-DSA-87 verify**: Verify the signature against the archive contents and publisher's public key
4. **Pass/Fail**: Only install if verification passes

---

## SPARK Authentication for Publishers

Publishers authenticate via **SPARK** (Secure Personal Authentication and Recognition using Keystreams), the eStream platform's biometric identity system.

### Publisher Identity Flow

1. Publisher authenticates via SPARK biometrics
2. SPARK derives a deterministic ML-DSA-87 key pair using HKDF with a marketplace-specific context
3. The derived key pair signs all published components
4. The public key is registered in the component registry for verification

SPARK uses per-product HKDF contexts (`q-marketplace-v1`), ensuring the marketplace signing key is cryptographically isolated from all other eStream keys.

---

## No Trusted TypeScript Principle

A core architectural constraint: **no security-critical operation ever executes in TypeScript**. All cryptographic operations — signing, verification, key derivation, hash computation — are performed in Rust (native) or WASM (browser/edge).

This eliminates an entire class of supply chain attacks where malicious NPM packages could intercept or forge signatures.

---

## Supply Chain Security

### Deterministic Builds

Component archives are built deterministically:
- File entries are sorted alphabetically
- Timestamps are normalized
- Compression uses a fixed algorithm and level
- The same source always produces the same archive hash

### Signature File Format

The `SIGNATURE.ml-dsa` file accompanies every published archive:

```
algorithm: ML-DSA-87
archive: my-component-1.0.0.tar.gz
fingerprint: a1b2c3d4e5f6a7b8
timestamp: 2026-02-20T12:00:00Z
signature: <ML-DSA-87 signature bytes>
```

| Field | Description |
|-------|-------------|
| `algorithm` | Always `ML-DSA-87` |
| `archive` | Filename of the signed archive |
| `fingerprint` | First 16 bytes of the archive hash (hex) |
| `timestamp` | RFC 3339 signing timestamp |
| `signature` | ML-DSA-87 signature bytes |

---

## Post-Quantum Threat Model

### What ML-DSA-87 Protects Against

| Threat | Protection |
|--------|-----------|
| **Tampering** | Any modification to the archive invalidates the signature |
| **Impersonation** | Only the publisher's SPARK-derived key can produce valid signatures |
| **Quantum attack** | Lattice-based scheme resists Shor's algorithm and Grover's algorithm |
| **Replay** | Timestamp + archive hash prevent signature reuse across versions |
| **Downgrade** | Algorithm field is part of the signed data |

### What It Does Not Protect Against

| Threat | Mitigation |
|--------|-----------|
| **Malicious publisher** | Community review, badge system (Official, Verified), governance oversight |
| **Compromised SPARK biometrics** | Key rotation, revocation via governance, multi-factor attestation |
| **Logic bugs in components** | Golden test vectors, `publish --dry-run` validation, community auditing |

---

## See Also

- [Component Guide](./component-guide.md) — Manifest structure and include patterns
- [CLI Reference](./cli-reference.md) — `publish` and `verify` command details
- [Badge Descriptions](../branding/badge-descriptions.md) — Verified, PQ-Signed, and other badges
- [FAQ](./faq.md) — Security-related questions
