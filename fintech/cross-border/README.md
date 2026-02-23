# eStream Cross-Border Payment Protocol

mBridge-compatible cross-border payment component for the eStream Marketplace.

## Overview

Provides atomic FX swaps, multi-CBDC settlement, and compliance-aware routing for cross-border payments. Uses ISO 20022 (pacs.008) as the underlying message format via the `estream-wire-iso20022` marketplace component.

## Installation

```sh
estream marketplace install estream-cross-border
```

## Dependencies

- `estream-wire-iso20022` (marketplace component) — ISO 20022 message types
- `estream-kernel` (platform) — Core runtime
- `estream-compliance` (platform) — Sanctions screening and compliance checks
- `estream-zk-proofs` (platform) — Zero-knowledge proof primitives

## Usage

```rust
use estream_cross_border::{CrossBorderIntent, SettlementEngine};

let intent = CrossBorderIntent::new()
    .from_currency("USD")
    .to_currency("EUR")
    .amount(50_000_00) // cents
    .route_via_mbridge();

let proof = engine.settle(intent).await?;
```

## Lifecycle

| Field | Value |
|-------|-------|
| Status | Active |
| Breaking Change Notice | 90 days |
| Platform Version | >= 0.9.1 |
