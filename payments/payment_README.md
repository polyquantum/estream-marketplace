# Payment Circuits

ESCIR circuit definitions for offline payment protocol.

**Issue**: [#351](https://github.com/toddrooke/estream-io/issues/351)  
**Spec**: [SPARK_OFFLINE_PAYMENT_SPEC.md](../../specs/protocol/SPARK_OFFLINE_PAYMENT_SPEC.md)  
**Schema**: [offline-payment.data.yaml](../../schemas/offline-payment.data.yaml)

## Circuits

| Circuit | Purpose | Size | Runtime |
|---------|---------|------|---------|
| [offline-balance-proof](./offline-balance-proof/) | ZK proof of sufficient balance | ~192 bytes | ~200ms |
| [offline-range-proof](./offline-range-proof/) | ZK proof of valid amount range | ~700 bytes | ~50ms |
| [offline-payment](./offline-payment/) | Complete payment verification | - | ~500ms |
| [offline-settlement](./offline-settlement/) | Batch settlement to chain | - | ~2s |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      OFFLINE PAYMENT CIRCUIT FLOW                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SENDER (Proof Generation)                                                  │
│  ┌─────────────────────┐    ┌─────────────────────┐                        │
│  │ offline-balance-    │    │ offline-range-      │                        │
│  │ proof               │    │ proof               │                        │
│  │                     │    │                     │                        │
│  │ Inputs:             │    │ Inputs:             │                        │
│  │ • commitment        │    │ • amount            │                        │
│  │ • amount            │    │ • min/max           │                        │
│  │ • balance (private) │    │                     │                        │
│  │                     │    │ Output:             │                        │
│  │ Output:             │    │ • range_proof       │                        │
│  │ • balance_proof     │    │   (~700 bytes)      │                        │
│  │   (~192 bytes)      │    │                     │                        │
│  └──────────┬──────────┘    └──────────┬──────────┘                        │
│             │                          │                                    │
│             └────────────┬─────────────┘                                    │
│                          │                                                  │
│                          ▼                                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    OfflinePaymentTransaction                         │   │
│  │  • tx_id, sender, recipient, amount, asset                          │   │
│  │  • balance_proof, range_proof                                       │   │
│  │  • sender_signature (ML-DSA-87)                                     │   │
│  │  • sender_povc (64-bit witness)                                     │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                    ═══════════════════════════                              │
│                    ║   SPARK VISUAL TRANSFER  ║                             │
│                    ═══════════════════════════                              │
│                                 │                                           │
│                                 ▼                                           │
│  RECIPIENT (Verification)                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       offline-payment                                │   │
│  │                                                                      │   │
│  │  Verifies:                                                           │   │
│  │  1. ML-DSA-87 signature ────────────────────────────────── (fast)   │   │
│  │  2. Nonce freshness (replay protection) ───────────────── (fast)    │   │
│  │  3. Timestamp validity ────────────────────────────────── (fast)    │   │
│  │  4. Party consistency ─────────────────────────────────── (fast)    │   │
│  │  5. Balance proof (Groth16 verify) ───────────────────── (medium)   │   │
│  │  6. Range proof (Bulletproofs verify) ────────────────── (medium)   │   │
│  │  7. Liveness score ────────────────────────────────────── (fast)    │   │
│  │  8. PoVC witness ──────────────────────────────────────── (fast)    │   │
│  │                                                                      │   │
│  │  Output: transaction_valid (bool)                                    │   │
│  │  PoVC: 64-bit witness proving verification was correct               │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                                 ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    OfflinePaymentReceipt                             │   │
│  │  • tx_hash, proofs_verified, liveness_verified                      │   │
│  │  • recipient_signature (ML-DSA-87)                                  │   │
│  │  • recipient_povc (64-bit witness)                                  │   │
│  └──────────────────────────────┬──────────────────────────────────────┘   │
│                                 │                                           │
│                    ═══════════════════════════                              │
│                    ║     DEVICE STORAGE       ║                             │
│                    ═══════════════════════════                              │
│                                 │                                           │
│                                 ▼                                           │
│  SETTLEMENT (When Online)                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      offline-settlement                              │   │
│  │                                                                      │   │
│  │  Batch Processing:                                                   │   │
│  │  1. Validate all receipts                                           │   │
│  │  2. Build Merkle tree of receipts                                   │   │
│  │  3. Check for double spends (nonce uniqueness)                      │   │
│  │  4. Verify on-chain balances sufficient                             │   │
│  │  5. Generate settlement transaction                                  │   │
│  │                                                                      │   │
│  │  Max batch: 256 receipts                                            │   │
│  │  Settlement time: ~2s (WASM), ~500ms (native)                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## StreamSight Integration

All circuits emit telemetry to StreamSight:

| Stream URI | Content |
|------------|---------|
| `lex://estream/sys/payment/offline/telemetry` | Operation spans |
| `lex://estream/sys/payment/offline/errors` | Error events |
| `lex://estream/sys/payment/offline/metrics` | Aggregated metrics |
| `lex://estream/sys/alerts` | Security alerts |

## Error Codes

| Code | Name | Circuit | Description |
|------|------|---------|-------------|
| E4001 | INSUFFICIENT_BALANCE | balance-proof | Balance proof failed |
| E4002 | INVALID_RANGE | range-proof | Range proof failed |
| E4003 | INVALID_SIGNATURE | offline-payment | ML-DSA-87 signature invalid |
| E4004 | LIVENESS_FAILED | offline-payment | Spark liveness below 80% |
| E4005 | NONCE_REUSE | offline-payment | Replay attack detected |
| E4006 | EXPIRED | offline-payment | Transaction expired |
| E4010 | POVC_INVALID | offline-payment | PoVC witness invalid |
| E4015 | SETTLEMENT_FAILED | settlement | Batch settlement failed |

## PoVC Witness Format

Each circuit produces a 64-bit PoVC witness:

```
┌────────────┬────────────┬────────────────────────────────────────┐
│ Circuit ID │   Nonce    │              Checksum                  │
│  (8 bits)  │ (16 bits)  │              (40 bits)                 │
└────────────┴────────────┴────────────────────────────────────────┘
```

Circuit IDs:
- `0x10` - Balance proof
- `0x11` - Range proof
- `0x12` - Payment verification
- `0x13` - Settlement

## Cryptographic Primitives

| Primitive | Algorithm | Security Level |
|-----------|-----------|----------------|
| Signatures | ML-DSA-87 | NIST Level 5 (PQ) |
| Balance Proof | Groth16 (BLS12-381) | ~128 bits |
| Range Proof | Bulletproofs (Ristretto255) | ~128 bits |
| Hashing | SHA3-256 | 256 bits |
| Commitment | Pedersen | Information-theoretic |

## Resource Estimates

| Circuit | WASM Memory | WASM Time | Native Time |
|---------|-------------|-----------|-------------|
| balance-proof | 4 MB | 200ms | 50ms |
| range-proof | 1 MB | 50ms | 10ms |
| offline-payment | 8 MB | 500ms | 100ms |
| settlement | 16 MB | 2000ms | 500ms |
