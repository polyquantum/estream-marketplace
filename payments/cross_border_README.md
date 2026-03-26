# Cross-Border Payment Circuits

Zero-knowledge proof circuits for the Cross-Border Payment Protocol (mBridge compatible).

**Issue**: [#348](https://github.com/polyquantum/estream-marketplace/issues/348)  
**Spec**: [CROSS_BORDER_MBRIDGE_SPEC.md](../../specs/protocol/CROSS_BORDER_MBRIDGE_SPEC.md)

## Circuits

### 1. Balance Proof (`balance-proof/`)

Proves sufficient balance for cross-border transfer without revealing actual balance.

| Property | Value |
|----------|-------|
| **Type** | Groth16 |
| **Curve** | BN254 |
| **Constraints** | ~2,048 |
| **Proving Time** | ~500ms |
| **Verification Time** | ~5ms |

**Public Inputs:**
- `balance_commitment` - Pedersen commitment to balance
- `minimum_required` - Source amount + fees
- `currency_hash` - Hash of source currency
- `wallet_id_hash` - Binding to wallet

**Private Inputs:**
- `balance` - Actual wallet balance
- `blinding_factor` - Commitment randomness

### 2. Atomic Swap Verification (`atomic-swap/`)

Verifies atomic swap execution integrity including FX conversion.

| Property | Value |
|----------|-------|
| **Type** | Groth16 |
| **Curve** | BN254 |
| **Constraints** | ~8,192 |
| **Proving Time** | ~1,500ms |
| **Verification Time** | ~10ms |

**Public Inputs:**
- `swap_id` - Unique swap identifier
- `source_amount` - Amount debited
- `destination_amount` - Amount credited
- `fx_rate_fp18` - Locked FX rate
- `hashlock` - HTLC hashlock
- Balance commitments (before/after for both parties)

**Private Inputs:**
- `preimage` - Hashlock preimage
- Actual balances before/after
- Commitment blinding factors

## Usage

```rust
use estream_cross_border::proofs::{
    generate_balance_proof,
    verify_balance_proof,
    generate_swap_proof,
    verify_swap_proof,
};

// Generate balance proof
let balance_proof = generate_balance_proof(
    balance,
    minimum_required,
    currency,
    wallet_id,
    blinding_factor,
)?;

// Verify balance proof
let valid = verify_balance_proof(
    &balance_proof,
    balance_commitment,
    minimum_required,
    currency_hash,
)?;

// Generate swap verification proof
let swap_proof = generate_swap_proof(
    &swap,
    preimage,
    source_balances,
    destination_balances,
)?;
```

## StreamSight Integration

All circuits emit telemetry to StreamSight:

| Metric | Topic |
|--------|-------|
| Balance proof telemetry | `lex://estream/sys/crossborder/proof/balance/telemetry` |
| Swap proof telemetry | `lex://estream/sys/crossborder/proof/swap/telemetry` |

## Dependencies

- `estream-zk-proofs` - Core ZK proof library
- `ark-groth16` - Groth16 implementation
- `ark-bn254` - BN254 curve
- `blake3` - BLAKE3 hashing
