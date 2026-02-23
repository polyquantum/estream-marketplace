# FIX Trading Marketplace Component Specification

> SmartCircuit-native FIX protocol adapter expressed as FastLang marketplace components with ESCIR compilation, tiered SKUs, and composite gateway packaging.

**Status:** Draft  
**Version:** 1.0.0  
**Issue:** [#524](https://github.com/polyquantum/estream-io/issues/524) (Phase 4)  
**Parent Spec:** [ESTREAM_MARKETPLACE_SPEC.md](./ESTREAM_MARKETPLACE_SPEC.md)

---

## Table of Contents

1. [Overview](#1-overview)
2. [Architecture](#2-architecture)
3. [Component Decomposition](#3-component-decomposition)
4. [Trading Data Schema Pack](#4-trading-data-schema-pack)
5. [FIX Wire Adapter](#5-fix-wire-adapter)
6. [FIX Trading Gateway Composite](#6-fix-trading-gateway-composite)
7. [Marketplace SKUs](#7-marketplace-skus)
8. [FastLang Circuit Design](#8-fastlang-circuit-design)
9. [ESCIR Circuit Definitions](#9-escir-circuit-definitions)
10. [Configuration](#10-configuration)
11. [Performance Targets](#11-performance-targets)
12. [Testing Strategy](#12-testing-strategy)
13. [Implementation Roadmap](#13-implementation-roadmap)
14. [Appendix A: Recommended FastLang Language Extensions](#appendix-a-recommended-fastlang-language-extensions)

---

## 1. Overview

The FIX Trading Marketplace Component provides FIX protocol (4.2/4.4/5.0) integration as a set of composable marketplace components, following the same layered architecture pattern established by the [Industrial Protocol Gateway v2](./INDUSTRIAL_PROTOCOL_GATEWAY_v0.9.1.md) and the [ISO 20022 FPGA Parser](../protocol/ISO20022_FPGA_PARSER_SPEC.md).

### 1.1 Design Principles

- **Not a monolithic crate** — decomposed into schema pack, wire adapter, and composite gateway
- **FastLang-first** — all protocol logic expressed as FastLang circuits, compiled via ESCIR
- **Dual-target** — same circuits run on CPU (Rust/WASM) or FPGA (synthesized Verilog)
- **PoVC-attested** — every FIX message translation carries witness attestation
- **Lex-governed** — session management, compliance, and audit under lex governance
- **Tiered SKUs** — Lite/Standard/Premium following the industrial gateway model

### 1.2 Relationship to Existing Components

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Existing Marketplace Components                       │
├──────────────────────┬──────────────────────┬───────────────────────────┤
│  ISO 20022           │  Industrial Gateway  │  FIX Trading (this spec) │
│  ──────────────────  │  ──────────────────  │  ─────────────────────── │
│  ESCIR circuit       │  ESCIR circuits (3)  │  FastLang circuits       │
│  Rust crate          │  Rust crate          │  ESCIR + Rust crate      │
│  FPGA RTL (19 files) │  FPGA RTL (optional) │  FPGA RTL (Premium SKU)  │
│  Spec + benchmarks   │  3-tier SKUs         │  3-tier SKUs             │
│  No FastLang yet     │  FastLang (Modbus)   │  FastLang-first design   │
└──────────────────────┴──────────────────────┴───────────────────────────┘
```

### 1.3 Upstream Dependencies

| Spec | Issue | Dependency |
|------|-------|------------|
| [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) | #528 | `WireAdapter` trait, lifecycle, data conversion |
| [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) | #526 | Schema provides/requires resolution |
| [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) | #525 | Manifest format, CLI install |
| [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) | #527 | Package archive, ML-DSA-87 signing |
| [ESTREAM_MARKETPLACE_SPEC.md](./ESTREAM_MARKETPLACE_SPEC.md) | #524 | Marketplace architecture, schema packs |

---

## 2. Architecture

### 2.1 Layered Stack

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      FIX TRADING GATEWAY                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                Composite Layer (fix-trading-gateway)               │  │
│  │  ┌──────────┐  ┌───────────────┐  ┌───────────┐  ┌───────────┐  │  │
│  │  │ Compliance│  │ Order Router  │  │ Settlement│  │ Audit     │  │  │
│  │  │ Pipeline  │  │               │  │ Circuit   │  │ Export    │  │  │
│  │  └─────┬─────┘  └──────┬────────┘  └─────┬─────┘  └─────┬─────┘  │  │
│  └────────┼───────────────┼──────────────────┼──────────────┼────────┘  │
│           │               │                  │              │           │
│  ┌────────┼───────────────┼──────────────────┼──────────────┼────────┐  │
│  │        │    Wire Adapter Layer (estream-wire-fix)        │        │  │
│  │  ┌─────┴─────┐  ┌─────┴────────┐  ┌──────┴──────┐               │  │
│  │  │ FIX Parser │  │ Session Mgr  │  │ Data        │               │  │
│  │  │ (tag=val)  │  │ (Logon/HB)   │  │ Translator  │               │  │
│  │  └─────┬─────┘  └──────────────┘  └──────┬──────┘               │  │
│  └────────┼──────────────────────────────────┼──────────────────────┘  │
│           │                                  │                         │
│  ┌────────┼──────────────────────────────────┼──────────────────────┐  │
│  │        │    Schema Layer (data-trading)     │                      │  │
│  │  ┌─────┴──────────┐  ┌───────────────────┴──┐  ┌─────────────┐  │  │
│  │  │ EStreamOrder   │  │ EStreamFill           │  │ EStreamQuote│  │  │
│  │  │ EStreamMktData │  │ FixNewOrderSingle     │  │ FixExecRpt  │  │  │
│  │  └────────────────┘  └──────────────────────┘  └─────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │               eStream Platform (lex streams, PoVC, ESCIR)        │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Data Flow

```
FIX Counterparty          eStream Platform                    Downstream Consumer
      │                         │                                      │
      │  FIX 4.4 (TCP/TLS)     │                                      │
      │────────────────────────▶│                                      │
      │                    fix_parser                                   │
      │                    ├─ parse tag=value                           │
      │                    ├─ validate checksum                        │
      │                    ├─ extract MsgType                          │
      │                    ▼                                           │
      │               fix_session_manager                              │
      │               ├─ Logon / Heartbeat / Logout                   │
      │               ├─ SeqNum tracking                              │
      │               ├─ Gap detection + ResendRequest                │
      │               ▼                                               │
      │          fix_to_data_translator                                │
      │          ├─ NewOrderSingle (D) → EStreamOrder                 │
      │          ├─ MarketDataSnapshot (W) → EStreamMarketData        │
      │          ▼                                                    │
      │     ┌──────────┐                                              │
      │     │  Data    │  Protocol-agnostic pivot                     │
      │     │ Message  │                                              │
      │     └────┬─────┘                                              │
      │          │                                                    │
      │    ┌─────┼───────────────────┐                                │
      │    │     │  Composite Layer  │                                │
      │    │     ▼                   │                                │
      │    │ compliance_check        │                                │
      │    │     ▼                   │                                │
      │    │ order_router            │                                │
      │    │     ▼                   │                                │
      │    │ PoVC witness            │                                │
      │    └─────┬───────────────────┘                                │
      │          │                                                    │
      │          ├───────────────────────────────────────────────────▶│
      │          │  data events on lex lattice paths                   │
      │◀─────────┤                                                    │
      │  data_to_fix_translator                                        │
      │  ├─ EStreamFill → ExecutionReport (8)                         │
      │  └─ EStreamOrder ack → OrderAck                               │
```

---

## 3. Component Decomposition

Three installable marketplace components compose the full FIX trading solution:

| Component | Category | License | Dependencies |
|-----------|----------|---------|-------------|
| `data-trading` | `data-schema` | Apache 2.0 | None |
| `estream-wire-fix` | `wire-adapter` | Apache 2.0 | `data-trading` |
| `fix-trading-gateway` | `integration` | Apache 2.0 | `estream-wire-fix`, `data-trading` |

### 3.1 Install Experience

```bash
# Install just the schema pack
estream marketplace install data-trading

# Install the wire adapter (auto-resolves data-trading dependency)
estream marketplace install estream-wire-fix

# Install the full composite gateway (auto-resolves all dependencies)
estream marketplace install fix-trading-gateway
```

### 3.2 Component Manifests

**data-trading:**

```toml
[component]
name = "data-trading"
version = "1.0.0"
category = "data-schema"
description = "Trading domain schemas: Order, Fill, Quote, MarketData"
license = "Apache-2.0"
keywords = ["trading", "fix", "order", "execution", "market-data"]

[component.author]
name = "eStream Contributors"

[component.schemas]
provides = ["EStreamOrder", "EStreamFill", "EStreamQuote", "EStreamMarketData"]
requires = []

[component.include]
schemas = ["schemas/*.data.yaml"]
fastlang = ["circuits/*.fl"]
```

**estream-wire-fix:**

```toml
[component]
name = "estream-wire-fix"
version = "1.0.0"
category = "wire-adapter"
description = "FIX 4.2/4.4/5.0 protocol adapter with SmartCircuit-native parsing"
license = "Apache-2.0"
keywords = ["fix", "trading", "financial", "wire-adapter", "protocol"]

[component.author]
name = "eStream Contributors"

[component.schemas]
provides = ["FixNewOrderSingle", "FixExecutionReport", "FixMarketData", "FixOrderCancel", "FixOrderCancelReject"]
requires = ["EStreamOrder", "EStreamFill", "EStreamMarketData"]

[component.dependencies]
data-trading = "^1.0.0"

[component.include]
fastlang = ["circuits/*.fl"]
escir = ["circuits/*.escir.yaml"]
crate = "crates/estream-wire-fix/"

[component.wire_adapter]
protocol_family = "Financial"
transports = ["tcp", "tls"]
bidirectional = true
```

**fix-trading-gateway:**

```toml
[component]
name = "fix-trading-gateway"
version = "1.0.0"
category = "integration"
description = "Complete FIX trading gateway: wire adapter + schemas + compliance + settlement"
license = "Apache-2.0"
keywords = ["fix", "gateway", "trading", "composite", "compliance"]

[component.author]
name = "eStream Contributors"

[component.dependencies]
data-trading = "^1.0.0"
estream-wire-fix = "^1.0.0"

[component.include]
fastlang = ["circuits/*.fl"]
escir = ["circuits/*.escir.yaml"]
```

---

## 4. Trading Data Schema Pack

### 4.1 Schemas

The `data-trading` schema pack provides four core data schemas. Full YAML definitions are in [ESTREAM_MARKETPLACE_SPEC.md §6.2](./ESTREAM_MARKETPLACE_SPEC.md#62-data-trading--trading-schema-pack).

| Schema | Lattice Path | Description |
|--------|-------------|-------------|
| `EStreamOrder` | `/trading/orders/{order_id}` | Order submission (Buy/Sell, Limit/Market/Stop) |
| `EStreamFill` | `/trading/fills/{fill_id}` | Trade execution / fill report |
| `EStreamQuote` | `/trading/quotes/{instrument}` | Bid/ask quote |
| `EStreamMarketData` | `/trading/market-data/{instrument}` | Market data snapshot (OHLCV) |

### 4.2 FastLang Validation Circuits

The schema pack includes FastLang circuits for schema validation, risk checks, and order normalization. See [`examples/fintech/fix-trading/trading_schemas.fl`](../../crates/estream-fastlang/examples/fintech/fix-trading/trading_schemas.fl).

---

## 5. FIX Wire Adapter

### 5.1 Protocol Support

| FIX Version | MsgTypes Supported | SKU |
|-------------|-------------------|-----|
| FIX 4.2 | D, 8, F, 9, W, A, 0, 5 | Lite, Standard, Premium |
| FIX 4.4 | All 4.2 + AE, AP, AQ | Standard, Premium |
| FIX 5.0 (FIXT 1.1) | All 4.4 + transport-independent | Premium |

### 5.2 Message Mappings

| FIX Message | MsgType Tag | Direction | Data Schema |
|-------------|-------------|-----------|-----------|
| NewOrderSingle | D | Ingress → Data | `FixNewOrderSingle` → `EStreamOrder` |
| ExecutionReport | 8 | Data → Egress | `EStreamFill` → `FixExecutionReport` |
| MarketDataSnapshotFullRefresh | W | Ingress → Data | `FixMarketData` → `EStreamMarketData` |
| OrderCancelRequest | F | Ingress → Data | `FixOrderCancel` |
| OrderCancelReject | 9 | Data → Egress | `FixOrderCancelReject` |
| Logon | A | Session | (session management) |
| Heartbeat | 0 | Session | (session management) |
| Logout | 5 | Session | (session management) |
| ResendRequest | 2 | Session | (gap fill recovery) |
| SequenceReset | 4 | Session | (sequence reset) |

### 5.3 FIX Tag Parsing

FIX messages use a `tag=value\x01` delimited format. The parser circuit operates as a streaming state machine:

```
Raw TCP bytes → Tag extraction → Value extraction → Checksum validation → MsgType dispatch
```

Key FIX tags parsed:

| Tag | Field | Type |
|-----|-------|------|
| 8 | BeginString | string (FIX.4.2, FIX.4.4, FIXT.1.1) |
| 9 | BodyLength | u32 |
| 35 | MsgType | char (D, 8, F, W, A, 0, 5, ...) |
| 49 | SenderCompID | string(64) |
| 56 | TargetCompID | string(64) |
| 34 | MsgSeqNum | u64 |
| 52 | SendingTime | timestamp |
| 10 | CheckSum | u8[3] |
| 11 | ClOrdID | string(64) |
| 55 | Symbol | string(32) |
| 54 | Side | char (1=Buy, 2=Sell) |
| 40 | OrdType | char (1=Market, 2=Limit, 3=Stop) |
| 38 | OrderQty | u64 |
| 44 | Price | fixed-point u64 (8 decimals) |
| 59 | TimeInForce | char (0=Day, 1=GTC, 3=IOC, 4=FOK) |

### 5.4 Session Management

FIX session state is maintained as a state machine under lex governance:

```
States: Disconnected → Connecting → LogonSent → Active → LogoutSent → Disconnected
                                                   │
                                                   ├── Heartbeat (periodic)
                                                   ├── TestRequest / TestResponse
                                                   └── ResendRequest / SequenceReset (gap recovery)
```

Session state tracked per connection:

| Field | Type | Description |
|-------|------|-------------|
| `sender_comp_id` | string(64) | Our CompID |
| `target_comp_id` | string(64) | Counterparty CompID |
| `next_send_seq` | u64 | Next outbound sequence number |
| `next_recv_seq` | u64 | Next expected inbound sequence number |
| `heartbeat_interval` | u32 | Heartbeat interval in seconds |
| `last_sent_time` | timestamp | Last message sent |
| `last_recv_time` | timestamp | Last message received |
| `session_state` | enum | Current FSM state |

### 5.5 FastLang Circuits

See [`examples/fintech/fix-trading/fix_wire_adapter.fl`](../../crates/estream-fastlang/examples/fintech/fix-trading/fix_wire_adapter.fl) for the full FastLang circuit definitions:

- `fix_parse` — Tag=value parser, checksum validation, MsgType extraction
- `fix_session` — Session state machine (Logon/Heartbeat/Logout/SeqNum tracking)
- `fix_to_data` — Ingress translation: FIX messages → data schemas
- `data_to_fix` — Egress translation: data schemas → FIX messages

---

## 6. FIX Trading Gateway Composite

The composite component bundles the wire adapter, schema pack, compliance pipeline, and settlement circuit into a single installable gateway.

### 6.1 Composite Wiring

See [`examples/fintech/fix-trading/fix_trading_gateway.fl`](../../crates/estream-fastlang/examples/fintech/fix-trading/fix_trading_gateway.fl) for the full FastLang composition.

```
fix_parse ──▶ fix_session ──▶ fix_to_data ──▶ compliance_check ──▶ order_route
                                                                       │
                                                                       ▼
data_to_fix ◀── settlement ◀── fill_stream ◀── lex stream (trading.fills)
```

### 6.2 Included Circuits

| Circuit | Source | Purpose |
|---------|--------|---------|
| `fix_parse` | `estream-wire-fix` | FIX tag=value parsing |
| `fix_session` | `estream-wire-fix` | Session management FSM |
| `fix_to_data` | `estream-wire-fix` | Ingress translation |
| `data_to_fix` | `estream-wire-fix` | Egress translation |
| `order_validate` | `data-trading` | Order field validation, risk checks |
| `fill_validate` | `data-trading` | Fill correctness validation |
| `compliance_check` | `fix-trading-gateway` | Pre-trade compliance (OFAC, position limits) |
| `order_route` | `fix-trading-gateway` | Route validated orders to lex trading stream |
| `settlement_bridge` | `fix-trading-gateway` | Match fills, trigger settlement |
| `audit_export` | `fix-trading-gateway` | Regulatory audit trail export |

---

## 7. Marketplace SKUs

Following the [Industrial Protocol Gateway](./INDUSTRIAL_PROTOCOL_GATEWAY_v0.9.1.md) tiered model:

### 7.1 SKU Comparison

| Feature | Lite | Standard | Premium |
|---------|------|----------|---------|
| **Price** | Free | Commercial | Commercial |
| **License** | Apache 2.0 | BSL 1.1 | Enterprise |
| **FIX versions** | 4.2 | 4.2, 4.4 | 4.2, 4.4, 5.0 (FIXT 1.1) |
| **Max sessions** | 10 | 100 | Unlimited |
| **Max msg/sec** | 10K | 100K | 1M+ |
| **Session persistence** | In-memory | Lex state | Lex state + FPGA |
| **Gap recovery** | Basic (ResendRequest) | Full (+ SequenceReset) | Full + FPGA-accelerated |
| **FPGA acceleration** | No | No | Yes (ESCIR → Verilog) |
| **Compliance circuits** | No | Basic (OFAC) | Full (OFAC + position limits + regulatory reporting) |
| **StreamSight telemetry** | Inline (`observe` verbs) | Full (+ `monitor` alerts) | Full + adaptive levels + custom dashboards |
| **Support** | Community | Email | Dedicated |

### 7.2 ESCIR Circuit Files

| SKU | ESCIR File | Location |
|-----|-----------|----------|
| Lite | `fix-trading-lite.escir.yaml` | `circuits/marketplace/` |
| Standard | `fix-trading-standard.escir.yaml` | `circuits/marketplace/` |
| Premium | `fix-trading-premium.escir.yaml` | `circuits/marketplace/` |

---

## 8. FastLang Circuit Design

### 8.1 Data Declarations

All types use the `data` construct (#652) with verbs for wire encoding, StreamSight hooks, privacy governance, signing envelopes, and PoVC attestation. StreamSight is integrated inline via `observe` verbs on data types and `streamsight true` / `observe metrics:` annotations on circuits — not as a standalone composed component.

```
data FixParsedMessage : circuit v1 {
    msg_type: bytes(2),
    sender_comp_id: bytes(64),
    target_comp_id: bytes(64),
    msg_seq_num: u64,
    sending_time: u64,
    body_length: u32,
    checksum: u32,
    field_count: u32,
    raw: bytes(8192),
    raw_length: u32,
}
    encode @ bytes(8340) { ... }
    observe metrics [msg_type, msg_seq_num, body_length] level adaptive
    observe events [sender_comp_id, target_comp_id, msg_type]

data FixSessionState : circuit v1 {
    sender_comp_id: bytes(64),
    target_comp_id: bytes(64),
    next_send_seq: u64,
    next_recv_seq: u64,
    heartbeat_interval: u32,
    last_sent: u64,
    last_recv: u64,
    state: u8,
    test_req_id: bytes(32),
}
    observe metrics [next_send_seq, next_recv_seq, state] level adaptive
    govern { SESSION -> encrypted(trading_admin), METADATA -> public }

data EStreamOrder : app v1 {
    order_id: bytes(32),
    client_order_id: bytes(64),
    instrument: bytes(32),
    side: u8,
    order_type: u8,
    quantity: u64,
    price: u64,
    time_in_force: u8,
    submitted_at: u64,
    submitter: bytes(32),
}
    encode @ bytes(209) { ... }
    observe metrics [quantity, price, side, order_type] level adaptive
    govern { PII -> encrypted(auditor), IDENTIFIER -> public, MARKET_DATA -> public }
    sign { algorithm mldsa87, key_field submitter, detached true }
    attest { povc true, anchor_field order_id, proof_system groth16 }

data EStreamFill : app v1 {
    fill_id: bytes(32),
    order_id: bytes(32),
    instrument: bytes(32),
    side: u8,
    quantity: u64,
    price: u64,
    fee: u64,
    executed_at: u64,
    venue: bytes(32),
}
    encode @ bytes(201) { ... }
    observe metrics [quantity, price, fee] level adaptive
    observe events [fill_id, order_id, instrument]
    attest { povc true, anchor_field fill_id, proof_system groth16 }
```

### 8.2 Circuit Summary

| Circuit | Inputs | Output | Annotations |
|---------|--------|--------|-------------|
| `fix_parse` | `FixRawMessage` | `FixParsedField` | precision C, constant_time, critical_path |
| `fix_session` | `FixParsedField, FixSessionState` | `FixSessionState` | precision B, witness threshold(2,3), lex governed |
| `fix_to_data` | `FixParsedField` | `EStreamOrder / EStreamFill` | precision C, observe metrics |
| `data_to_fix` | `EStreamFill` | `FixRawMessage` | precision C, observe metrics |
| `order_validate` | `EStreamOrder` | `bool` | precision A, constant_time, sanitize pii |
| `fill_validate` | `EStreamFill` | `bool` | precision A |
| `compliance_check` | `EStreamOrder` | `ComplianceResult` | precision A, constant_time, lex compliance |
| `order_route` | `EStreamOrder, ComplianceResult` | `bytes(32)` | precision B, witness |
| `settlement_bridge` | `EStreamFill` | `SettlementResult` | precision A, witness threshold(2,3) |

---

## 9. ESCIR Circuit Definitions

### 9.1 Lite SKU Structure

```yaml
escir: "0.8.0"
name: fix_trading_lite
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: marketplace_fix_trading_lite
  name: "FIX Trading Gateway Lite"
  description: |
    Entry-level FIX protocol gateway with FIX 4.2 support.
    Ideal for development, testing, and low-volume trading integration.
  category: marketplace
  marketplace:
    sku: "fix-trading-lite"
    visibility: source
    publisher: estream-official
    pricing:
      model: free
      price: 0
    license: "Apache-2.0"
    tags: ["fix", "trading", "gateway", "free", "open-source"]

limits:
  max_sessions: 10
  max_messages_per_second: 10000
  fix_versions: ["4.2"]

composition:
  - id: fix_parser
    circuit: wire/fix/fix_parse
    instances: 10
  - id: session_manager
    circuit: wire/fix/fix_session
    instances: 10
  - id: ingress_translator
    circuit: wire/fix/fix_to_data
    instances: 10
  - id: egress_translator
    circuit: wire/fix/data_to_fix
    instances: 10
  - id: streamsight
    circuit: platform/streamsight_emitter
    instances: 1

wiring:
  - from: fix_parser[n].parsed_out
    to: session_manager[n].message_in
  - from: session_manager[n].app_message_out
    to: ingress_translator[n].fix_in
  - from: ingress_translator[*].data_out
    to: streamsight.event_in
  - from: egress_translator[*].fix_out
    to: session_manager[n].send_in

inputs:
  - name: raw_fix
    type: FixRawMessage
    target: fix_parser[n].raw_in
  - name: data_egress
    type: EStreamFill
    target: egress_translator[n].data_in

outputs:
  - name: data_orders
    type: EStreamOrder
    source: ingress_translator[*].data_out
  - name: fix_responses
    type: FixRawMessage
    source: egress_translator[*].fix_out
  - name: metrics
    type: StreamSightMetrics
    source: streamsight.metrics_out

annotations:
  witness_tier: 1
  marketplace_sku: "fix-trading-lite"
  open_source: true
  hardware_accelerated: false
  streamsight_emit: true
```

---

## 10. Configuration

### 10.1 Gateway Configuration

```yaml
fix_trading_gateway:
  sku: lite  # lite | standard | premium
  
  sessions:
    - id: "BROKER_A"
      sender_comp_id: "ESTREAM01"
      target_comp_id: "BROKERA"
      fix_version: "FIX.4.2"
      host: "fix.broker-a.com"
      port: 9876
      tls:
        enabled: true
        cert_path: "/etc/estream/certs/fix-client.pem"
      heartbeat_interval_sec: 30
      reconnect:
        enabled: true
        max_attempts: 10
        backoff_ms: [1000, 2000, 4000, 8000]
  
  trading:
    lattice: estream.trading
    order_path: /trading/orders
    fill_path: /trading/fills
    quote_path: /trading/quotes
    market_data_path: /trading/market-data
  
  compliance:
    enabled: false  # Lite SKU: no compliance circuits
  
  streamsight:
    enabled: true
    metrics: [messages_parsed, sessions_active, orders_ingested, fills_emitted, parse_errors]
```

---

## 11. Performance Targets

### 11.1 CPU Target (Rust)

| Metric | Lite | Standard | Premium |
|--------|------|----------|---------|
| FIX parse latency | < 500 ns | < 500 ns | < 500 ns |
| Data translation | < 200 ns | < 200 ns | < 200 ns |
| End-to-end (parse → lex) | < 2 μs | < 2 μs | < 2 μs |
| Message throughput | 10K msg/s | 100K msg/s | 500K msg/s |
| Session establishment | < 10 ms | < 10 ms | < 10 ms |

### 11.2 FPGA Target (Premium SKU)

| Metric | Target | vs CPU |
|--------|--------|--------|
| FIX parse latency | < 40 ns | 12× faster |
| Data translation | < 20 ns | 10× faster |
| End-to-end (parse → lex) | < 100 ns | 20× faster |
| Message throughput | 5M msg/s | 10× CPU |
| Tail latency (p99) | Deterministic | No jitter |

---

## 12. Testing Strategy

### 12.1 Test Layers

| Layer | Test Type | Location |
|-------|-----------|----------|
| FastLang circuits | Golden tests | `crates/estream-fastlang/tests/golden/marketplace/` |
| FIX parser | Unit tests, fuzz | `crates/estream-wire-fix/src/parser/` |
| Session management | FSM property tests | `crates/estream-wire-fix/src/session/` |
| Data translation | Round-trip tests | `crates/estream-wire-fix/src/translator/` |
| Composite gateway | Journey tests | `crates/estream-test/` (`JourneyCategory::Marketplace`) |
| ESCIR circuits | Differential golden | ESCIR test framework |

### 12.2 FIX Compliance Test Vectors

Standard FIX test messages for each MsgType:

| Test Vector | MsgType | Purpose |
|-------------|---------|---------|
| `fix42_new_order_single.fix` | D | NewOrderSingle with all required tags |
| `fix42_execution_report.fix` | 8 | ExecutionReport (fill) |
| `fix42_market_data_snapshot.fix` | W | MarketDataSnapshotFullRefresh |
| `fix42_order_cancel.fix` | F | OrderCancelRequest |
| `fix42_logon.fix` | A | Logon with HeartBtInt |
| `fix42_heartbeat.fix` | 0 | Heartbeat |
| `fix42_malformed_checksum.fix` | - | Invalid checksum (error case) |
| `fix42_sequence_gap.fix` | - | Sequence gap requiring ResendRequest |

---

## 13. Implementation Roadmap

```
Phase 1: FastLang Circuits + ESCIR (2 weeks)
    ├── FastLang: fix_wire_adapter.fl, trading_schemas.fl, fix_trading_gateway.fl
    ├── ESCIR: fix-trading-lite.escir.yaml
    └── Golden tests for all circuits

Phase 2: Rust Crate — estream-wire-fix (3 weeks)
    ├── FIX tag=value parser (no_std compatible)
    ├── Session management FSM
    ├── WireAdapter trait implementation
    ├── Data translation (ingress + egress)
    └── Unit tests + fuzz tests

Phase 3: Composite Gateway + Standard SKU (2 weeks)
    ├── fix-trading-gateway composite wiring
    ├── Compliance circuits (OFAC, position limits)
    ├── fix-trading-standard.escir.yaml
    └── Journey tests in estream-test

Phase 4: Premium SKU + FPGA (4 weeks)
    ├── FPGA RTL: FIX parser pipeline
    ├── FPGA RTL: session state CAM
    ├── fix-trading-premium.escir.yaml
    ├── Simulation + benchmarks
    └── Marketplace publish (all 3 SKUs)
```

---

## Appendix A: Recommended FastLang Language Extensions

The FIX trading component can be fully expressed using existing FastLang constructs (`circuit`, `pipeline`, `platform`-style composition, `state_machine`, `stream`). However, three new constructs would make wire adapter marketplace components first-class citizens of the language. These are proposed as future language enhancements and are **not required** to implement this spec.

### A.1 `adapter` Declaration

A first-class construct for wire protocol translation that formalizes ingress/egress translation, session lifecycle, and protocol-to-data mapping:

```
adapter fix_adapter v1 {
    protocol "FIX" versions ["4.2", "4.4", "5.0"]
    transport tcp, tls
    session_management true

    ingress NewOrderSingle(raw: FixRawMessage) -> EStreamOrder {
        let parsed = fix_parse(raw)
        fix_to_data(parsed)
    }

    egress ExecutionReport(fill: EStreamFill) -> FixRawMessage {
        data_to_fix(fill)
    }

    session logon(credentials: bytes(256)) -> FixSessionState {
        fix_session_logon(credentials)
    }

    session heartbeat(state: FixSessionState) -> FixSessionState {
        fix_session_heartbeat(state)
    }
}
```

**Compiler benefits:**
- Auto-generate `WireAdapter` trait implementations from the declaration
- Validate bidirectional mappings at compile time (every ingress type has a corresponding data schema)
- Generate ESCIR with proper session state management annotations
- Type-check that ingress/egress functions return the declared data types

### A.2 `component` Declaration

A marketplace packaging construct that declares provides/requires schemas, SKU tiers, and composition boundaries — replacing the `estream-component.toml` manifest with a FastLang-native declaration:

```
component estream_wire_fix v1 {
    category wire_adapter
    license "Apache-2.0"
    
    provides [FixNewOrderSingle, FixExecutionReport, FixMarketData]
    requires [EStreamOrder, EStreamFill]

    tier lite {
        max_sessions 10
        versions ["4.2"]
        fpga false
    }
    
    tier standard {
        max_sessions 100
        versions ["4.2", "4.4"]
        fpga false
    }
    
    tier premium {
        max_sessions unlimited
        versions ["4.2", "4.4", "5.0"]
        fpga true
    }
}
```

**Compiler benefits:**
- Generate `estream-component.toml` manifests from FastLang source
- Validate schema provides/requires at compile time against actual circuit signatures
- Enforce tier limits in ESCIR generation (e.g., lite SKU cannot reference premium-only circuits)

### A.3 `composite` Declaration

A multi-component assembly construct for bundling marketplace components into installable solutions. This extends the existing `platform` construct (which is core-platform-scoped) to marketplace bundles:

```
composite fix_trading_gateway v1 {
    install data_trading v1
    install estream_wire_fix v1 tier standard
    
    connect fix_parse.data_out -> compliance_check.order_in
    connect compliance_check.cleared_out -> order_route.order_in
    connect order_route.routed_out -> lex_stream estream.trading.order
    connect lex_stream estream.trading.fill -> data_to_fix.fill_in
}
```

**Compiler benefits:**
- Validate that all `install` dependencies are satisfiable
- Type-check `connect` wiring across component boundaries
- Generate composite ESCIR from sub-component ESCIR definitions

### A.4 Recommendation

These three constructs should be evaluated as FastLang language extensions in a future ESCIR SDK version. For this spec, the same semantics are expressed using:

- **`adapter`** → multiple `circuit` declarations + `state_machine` for session FSM
- **`component`** → `estream-component.toml` manifest file
- **`composite`** → `platform`-style `connect` wiring in a top-level `.fl` file

---

## References

- [ESTREAM_MARKETPLACE_SPEC.md](./ESTREAM_MARKETPLACE_SPEC.md) — Parent marketplace specification
- [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) — WireAdapter trait (#528)
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema composition (#526)
- [INDUSTRIAL_PROTOCOL_GATEWAY_v0.9.1.md](./INDUSTRIAL_PROTOCOL_GATEWAY_v0.9.1.md) — Industrial gateway pattern reference
- [ISO20022_FPGA_PARSER_SPEC.md](../protocol/ISO20022_FPGA_PARSER_SPEC.md) — ISO 20022 component reference
- [FIX Protocol Specification](https://www.fixtrading.org/standards/) — FIX Trading Community standards

---

*Created: 2026-02-18*  
*Status: Draft*  
*Issue: #524 (Phase 4)*
