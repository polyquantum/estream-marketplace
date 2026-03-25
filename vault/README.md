# Poly Pass — eStream Marketplace Component

**Publisher:** Poly Labs (`polylabs`)
**Version:** 0.12.0
**Category:** Smart Circuit
**Wire Protocol:** 0xC0-0xD8 (UDP)

## Overview

Post-quantum encrypted password manager with SPARK biometric authentication, per-credential AES-256-GCM encryption, breach monitoring, TOTP/HOTP generation, enterprise RBAC with role inheritance, and team/family vault sharing.

## Circuits (10)

| Circuit | Description |
|---------|-------------|
| `polypass_encrypt` | ML-KEM-1024 vault key derivation, per-credential AES-256-GCM encrypt/decrypt, key rotation with full re-wrap |
| `polypass_rbac` | Enterprise RBAC: 5-tier role hierarchy (Viewer→Owner), bitmask permissions, org hierarchy, vault-scoped ACLs |
| `polypass_share` | Credential sharing: key re-wrapping, share lifecycle (pending→accepted→revoked), family/team vaults |
| `polypass_autofill` | URL domain matching, credential lookup, autofill frequency tracking |
| `polypass_audit` | Password strength scoring, age tracking, usage analytics |
| `polypass_breach` | Breach database checking with k-anonymity, batch scanning, real-time monitoring subscriptions |
| `polypass_totp` | TOTP/HOTP generation and verification (HMAC-SHA1/256/512) |
| `polypass_import` | Multi-format import: 1Password, Bitwarden, LastPass, Dashlane, KeePass, Chrome, Firefox CSV |
| `polypass_metering` | Per-seat and per-invocation metering across 8 resource dimensions |
| `polypass_platform_health` | Vault integrity checks, circuit health probes, anomaly detection |

## Graphs (2)

| Structure | Type | Description |
|-----------|------|-------------|
| `polypass_vault_graph` | `graph` | Vault registry: CredentialNode, FolderNode, TagNode with breach/strength/age/autofill overlays |
| `polypass_share_graph` | `graph` | Share network: SharedVaultNode, ShareInvite edges, permission masks |

## RBAC Permission Model

| Bit | Permission | Viewer | User | Manager | Admin | Owner |
|-----|-----------|--------|------|---------|-------|-------|
| 0x01 | READ | x | x | x | x | x |
| 0x02 | DECRYPT | | x | x | x | x |
| 0x04 | WRITE | | x | x | x | x |
| 0x08 | SHARE | | | x | x | x |
| 0x10 | DELETE | | | | x | x |
| 0x20 | ADMIN | | | x | x | x |
| 0x40 | AUDIT | | | | x | x |
| 0x80 | EXPORT | | | | | x |

## Dependencies

- `polykit-identity` >= 0.2.0 (SPARK identity, ML-DSA-87)
- `polykit-metering` >= 0.2.0 (8-dimension resource metering)
- `polykit-telemetry` >= 0.2.0 (StreamSight telemetry pipeline)
- `polykit-rate-limiter` >= 0.2.0 (Per-tier rate limiting)

## Source

- **Repository:** [polylabs-dev/polypass](https://github.com/polylabs-dev/polypass)
- **Rust crate:** `poly-pass-core`
