# eStream Marketplace — Open Source Component Exchange Specification

> Comprehensive specification for the eStream component marketplace, modeled after Chronicle Software's open-core strategy.

**Status:** Draft  
**Version:** 1.0.0  
**Issue:** [#524](https://github.com/polyquantum/estream-io/issues/524)  
**Epic:** [EPIC_ESTREAM_MARKETPLACE.md](../../.github/epics/EPIC_ESTREAM_MARKETPLACE.md)

---

## 1. Overview

The eStream Marketplace is an open source component exchange that enables developers to discover, publish, install, and compose reusable eStream components — ESF schemas, SmartCircuits, protocol adapters, FPGA circuits, console widgets, and pre-built integrations.

### 1.1 Beyond Chronicle — SmartCircuit-Native Architecture

Chronicle Software proved that open-sourcing high-performance middleware (Queue, Map, Wire) creates a thriving ecosystem and enterprise adoption funnel. eStream follows the same open-core model but with a fundamentally different architecture: **every queue and map operation is a SmartCircuit execution on native lex streams and lex state** — not a standalone library crate.

Chronicle Queue and Chronicle Map are Java libraries that ultimately hit the JVM ceiling: GC pauses, OS scheduling jitter, heap management, and no path to hardware acceleration. eStream transcends this by making queue and map operations SmartCircuit executions that:

1. **Run natively on lex streams and lex state** — persistence, ordering, replication, and witness attestation come from the platform, not from library code
2. **Are dual-target compiled via ESCIR** — the same circuit definition runs on CPU (Rust/WASM) or FPGA (synthesized Verilog)
3. **Achieve orders-of-magnitude speedup on FPGA** — queue append becomes a hardware pipeline (ESF serialize → PRIME signer → lex store write → ack) with deterministic nanosecond latency and zero OS overhead

| Chronicle (OSS) | eStream Native | Execution Model |
|-----------------|---------------|-----------------|
| Chronicle Queue | **Queue Streams** — SmartCircuit-driven lex streams | `queue.append.v1` circuit → Witness → Lex Store |
| Chronicle Map | **State Maps** — SmartCircuit-driven lex state | `map.put.v1` circuit → State Root → VRF Scatter |
| Chronicle Wire | ESF — eStream Format | Exists |
| Chronicle Services | SmartCircuit runtime (BSL 1.1) | Exists |
| Chronicle FIX | Wire adapters via `WireAdapter` trait | Circuit-wrapped protocol adapters |

**SmartCircuits are the proactive real-time backend** — not passive marketplace components waiting to be composed, but active processing units that drive queue compaction, map replication, TTL garbage collection, and protocol translation. They execute on triggers (events, timers, thresholds) with full witness attestation and 8D metering.

### 1.2 Licensing Tiers

| Tier | License | Components |
|------|---------|-----------|
| **Open Source** | Apache 2.0 | Queue, Map, Wire adapters, ESF schemas, SDK |
| **Source Available** | BSL 1.1 | FPGA acceleration, VRF Scatter HA, production runtime |
| **Commercial** | Enterprise | Managed deployment, SLA, support, custom adapters |

### 1.3 Upstream Specifications

This spec builds on five upstream requirement specifications:

| Spec | Issue | Description |
|------|-------|-------------|
| [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) | #528 | `WireAdapter` trait, lifecycle, ESF conversion |
| [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) | #525 | Manifest, registry, CLI, ML-DSA-87 signing |
| [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) | #526 | Schema dependency resolution |
| [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) | #527 | Package format, test vectors, FPGA bitstreams |
| [CONSOLE_WIDGET_MARKETPLACE_SPEC.md](./CONSOLE_WIDGET_MARKETPLACE_SPEC.md) | #533 | Console widget marketplace extension |

And three pre-existing specifications:

| Spec | Description |
|------|-------------|
| [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md) | Pricing, visibility, creator program |
| [FPGA_COMPONENT_EXTENSION.md](./FPGA_COMPONENT_EXTENSION.md) | FPGA component metadata |
| [INDUSTRIAL_PROTOCOL_GATEWAY_V2.md](./INDUSTRIAL_PROTOCOL_GATEWAY_V2.md) | Layered adapter architecture |

---

## 2. Architecture

### 2.1 System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           eStream Marketplace                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CLI (estream marketplace)            Console (WidgetPicker)                │
│  ┌──────────────────────────┐        ┌──────────────────────────┐          │
│  │ search · install · pub   │        │ Browse · Install · Rate  │          │
│  │ verify · scaffold        │        │ Marketplace Tab           │          │
│  └────────────┬─────────────┘        └────────────┬─────────────┘          │
│               │                                    │                        │
│               ▼                                    ▼                        │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │              Component Registry (GitHub-backed)              │           │
│  └─────────────────────────────┬───────────────────────────────┘           │
│                                │                                            │
│  ┌─────────────────────────────┼───────────────────────────────┐           │
│  │              Component Types│                                │           │
│  │  ESF Schemas · SmartCircuits · Wire Adapters · Widgets       │           │
│  │  FPGA Circuits · Full Integrations                           │           │
│  └──────────────────────────────────────────────────────────────┘           │
│                                                                             │
│  ╔══════════════════════════════════════════════════════════════╗           │
│  ║          SmartCircuit Runtime (Proactive Real-Time Backend)  ║           │
│  ║                                                              ║           │
│  ║  ┌────────────────────────┐  ┌────────────────────────┐     ║           │
│  ║  │   Queue Stream Circuits│  │   State Map Circuits   │     ║           │
│  ║  │   queue.append.v1      │  │   map.put.v1           │     ║           │
│  ║  │   queue.compact.v1     │  │   map.cas.v1           │     ║           │
│  ║  │   queue.replicate.v1   │  │   map.gc.v1            │     ║           │
│  ║  └───────────┬────────────┘  └───────────┬────────────┘     ║           │
│  ║              │                            │                  ║           │
│  ║              ▼                            ▼                  ║           │
│  ║  ┌──────────────────────────────────────────────────────┐   ║           │
│  ║  │           Lex Streams + Lex State                     │   ║           │
│  ║  │  Witness Attestation · 8D Metering · MTP Ordering     │   ║           │
│  ║  │  VRF Scatter Replication · PQ Signing (PRIME Signer)  │   ║           │
│  ║  └──────────────────────────────────────────────────────┘   ║           │
│  ║                                                              ║           │
│  ║  ┌─────────────────┐  ┌──────────────────────────────┐      ║           │
│  ║  │  CPU Target      │  │  FPGA Target                 │      ║           │
│  ║  │  Rust / WASM     │  │  ESCIR → Verilog → Bitstream │      ║           │
│  ║  │  (competitive)   │  │  (orders of magnitude faster) │      ║           │
│  ║  └─────────────────┘  └──────────────────────────────┘      ║           │
│  ╚══════════════════════════════════════════════════════════════╝           │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────┐           │
│  │  Wire Adapters (WireAdapter trait)                            │           │
│  │  MQTT · FIX · HL7 · Modbus · SWIFT · OPC-UA                  │           │
│  └──────────────────────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 End-to-End Data Flow — SmartCircuit Execution Path

```
External Protocol                  eStream Platform                      Consumer
      │                                  │                                  │
      │  MQTT/FIX/HL7/Modbus            │                                  │
      │─────────────────────────▶       │                                  │
      │                         WireAdapter                                │
      │                         ├─ ingest()                                │
      │                         ├─ translate_ingress()                     │
      │                         ▼                                          │
      │                    ┌─────────┐                                     │
      │                    │  ESF    │  Protocol-agnostic pivot            │
      │                    │ Message │                                     │
      │                    └────┬────┘                                     │
      │                         │                                          │
      │           ┌─────────────┼──────────────┐                           │
      │           │             │              │                           │
      │           ▼             ▼              ▼                           │
      │    ┌─────────────┐ ┌─────────┐ ┌────────────┐                    │
      │    │queue.append │ │map.put  │ │ User       │                    │
      │    │.v1 Circuit  │ │.v1 Circ │ │ Circuits   │                    │
      │    └──────┬──────┘ └────┬────┘ └─────┬──────┘                    │
      │           │             │             │                            │
      │           ▼             ▼             ▼                            │
      │    ┌────────────────────────────────────────┐                     │
      │    │       Witness Attestation (PoVC)        │                     │
      │    │       8D Resource Metering               │                     │
      │    └──────────────────┬─────────────────────┘                     │
      │                       │                                            │
      │           ┌───────────┼───────────┐                                │
      │           ▼           ▼           ▼                                │
      │    ┌───────────┐ ┌─────────┐ ┌──────────┐                        │
      │    │ Lex Stream│ │Lex State│ │ Lex      │                        │
      │    │ (persist) │ │ (root)  │ │ (govern) │                        │
      │    └─────┬─────┘ └────┬────┘ └────┬─────┘                        │
      │          │             │           │                               │
      │          └──────────┬──┘           │                               │
      │                     │              │                               │
      │  ┌──────────────────┼──────────────┼───────────────────────┐      │
      │  │  EXECUTION TARGET│              │                       │      │
      │  │  ┌───────────────┴──┐  ┌────────┴────────────────┐     │      │
      │  │  │ CPU (Rust/WASM)  │  │ FPGA (ESCIR→Verilog)    │     │      │
      │  │  │ ~100ns append    │  │ ~10ns append             │     │      │
      │  │  │ ~400μs signed    │  │ ~400ns signed (PRIME)    │     │      │
      │  │  └──────────────────┘  └─────────────────────────┘     │      │
      │  └─────────────────────────────────────────────────────────┘      │
      │                     │                                              │
      │                     ├─────────────────────────────────────────────▶│
      │                     │  ESF events on lattice paths                 │
      │◀────────────────────┤                                              │
      │  translate_egress() │  WireAdapter.emit()                          │
```

---

## 3. Phase 1: Marketplace Infrastructure

Phase 1 is fully specified by the upstream specs. Implementation creates:

| Deliverable | Spec | Crate/Location |
|-------------|------|----------------|
| `estream-component.toml` manifest | [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) §2 | Parsed in `estream-cli` |
| `GitHubRegistry` | [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) §6 | `crates/estream-escir/src/composition/github.rs` |
| CLI commands | [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) §5 | `crates/estream-cli/src/commands/marketplace.rs` |
| ML-DSA-87 signing | [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) §4 | `crates/estream-kernel/src/pq/sign.rs` (existing) |
| ESF schema resolution | [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) | `crates/estream-escir/src/esf.rs` (extend) |
| Package format | [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) | `crates/estream-cli/src/commands/marketplace.rs` |

### 3.1 Implementation Checklist

- [ ] Parse `estream-component.toml` manifest (all fields per §2 of Registry spec)
- [ ] `GitHubRegistry` implementing existing `Registry` trait from `composition/registry.rs`
- [ ] `estream marketplace search` command
- [ ] `estream marketplace install` command with dependency resolution
- [ ] `estream marketplace publish` command with ML-DSA-87 signing
- [ ] `estream marketplace verify` command
- [ ] `estream marketplace scaffold` command
- [ ] ESF schema provides/requires resolution during install
- [ ] Package archive creation (deterministic tar.gz)
- [ ] `SIGNATURE.ml-dsa` generation and verification
- [ ] Local cache at `$HOME/.estream/cache/`
- [ ] `estream-workspace.toml` tracking

---

## 4. Platform Stream Primitives (Moved)

> **This content has been moved to the canonical [Stream Architecture Specification v0.8.1](../architecture/STREAM_ARCHITECTURE_SPEC.md).**
>
> The Stream Architecture Spec v0.8.1 defines:
> - **Typed stream patterns**: event, state, signal, curated, log, media, transaction, mpc_session (§2)
> - **Growing context pipelines**: Multi-stage widening ESF schemas (§3)
> - **ESCIR pattern annotations**: stream_pattern, topology, governance, sla (§4)
> - **Stream operators**: filter, transform, aggregate, throttle, join, materialize, pipeline (§13)
> - **Audience filters**: Crypto-enforced field visibility with governance auto-generation (§14)
> - **Adaptive observation**: StreamSight dynamic telemetry L0-L3 (§15)
> - **Governance lifecycle**: Genesis, lex activation, protocol upgrade, emergency (§16)
> - **Transport & addressing**: IPv6, SRv6, geo-avoidance, ESP tunnels (§17)
> - **DNS & naming**: gTLD hybrid DNS+lex resolution (§18)
> - **High availability**: SLA-driven HA clusters with witness braiding (§19)
> - **Multi-party computation**: Garbled circuits, PSI, threshold decryption (§20)
> - **Deployment pipeline**: Growing context release flow with PoVC (§21)
> - **Circuit references**: Lex-path-based URI scheme (§22)
> - **Codegen contract**: Rust and Verilog generation per annotation (§5)
> - **Typed governance primitives**: SOC2, PCI-DSS, HIPAA, GDPR (§6)
> - **Lex hierarchy & typed consensus**: N-level with governance cascade (§7)
> - **Declarative SLAs**: Latency, durability, geo, throughput targets (§8)
> - **Multi-language SDK design**: Rust, Swift, Kotlin, Go, TypeScript, Python (§9)
>
> Queue Streams (`estream-queue`) and State Maps (`estream-map`) are now expressed as
> **event streams** and **state streams** respectively, implemented as SmartCircuit-driven
> typed stream patterns on lex FIFO and KV storage primitives.
>
> See the [Stream Architecture Implementation Plan](../architecture/STREAM_ARCHITECTURE_IMPLEMENTATION_PLAN.md)
> for the phased build roadmap.

---

## ~~5. Phase 2: State Maps~~ (Moved — see §4 above)

---

## 5. `WireAdapter` Trait + `estream-wire-mqtt`

Fully specified in [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md).

### 5.1 Implementation Checklist

- [ ] `WireAdapter` trait in `crates/estream-kernel/src/wire.rs`
- [ ] `WireAdapterFactory` and `AdapterRegistry`
- [ ] `AdapterError`, `HealthStatus`, `AdapterDescriptor`, `AdapterSchemas` types
- [ ] `AdapterEvent` for StreamSight integration
- [ ] `EsfMessage` pivot format
- [ ] `estream-wire-mqtt` crate: MQTT 5.0 reference implementation
  - [ ] `MqttConfig` with broker, auth, TLS, subscriptions
  - [ ] `MqttAdapter` implementing `WireAdapter`
  - [ ] Topic-to-schema mapping for ESF translation
  - [ ] Reconnect with exponential backoff
  - [ ] StreamSight event emission
- [ ] Compliance test suite for `WireAdapter` implementations
- [ ] Journey tests in `crates/estream-test/`

---

## 6. Phase 3: Domain Schema Packs

Three ESF schema packs covering the most common integration domains. Each is published as an `esf-schema` component to the marketplace.

### 6.1 `esf-iot` — IoT Schema Pack

**Component:** `esf-iot`  
**Schemas:** Telemetry, Command, Alert

```yaml
# schemas/iot-telemetry.esf.yaml
version: "1.0"
namespace: estream.iot

schemas:
  IoTTelemetry:
    description: "Real-time device telemetry reading"
    lattice: estream.iot.telemetry
    path: /devices/{device_id}/telemetry
    fields:
      device_id:
        type: string(64)
        description: "Unique device identifier"
      timestamp:
        type: timestamp
        description: "Reading timestamp (MTP)"
      metric_name:
        type: string(128)
        description: "Metric identifier"
      value:
        type: f64
        description: "Metric value"
      unit:
        type: string(32)
        description: "Unit of measurement"
        optional: true
      quality:
        type: u8
        description: "Data quality indicator (0-100)"
        optional: true
      location:
        type: object
        fields:
          latitude: { type: f64 }
          longitude: { type: f64 }
          altitude_m: { type: f64, optional: true }
        optional: true
      metadata:
        type: "map(string, string)"
        description: "Key-value metadata"
        optional: true
    retention:
      duration_days: 90

  IoTCommand:
    description: "Command sent to a device"
    lattice: estream.iot.command
    path: /devices/{device_id}/commands
    fields:
      command_id:
        type: bytes32
        description: "Unique command identifier"
      device_id:
        type: string(64)
      command_type:
        type: string(64)
        description: "Command type identifier"
      payload:
        type: bytes
        description: "Command payload"
      issued_at:
        type: timestamp
      issued_by:
        type: bytes32
        description: "Issuer public key fingerprint"
        privacy: encrypted
      expires_at:
        type: timestamp
        optional: true
      signature:
        type: pq_signature
        description: "ML-DSA-87 command signature"
    retention:
      duration_days: 30

  IoTAlert:
    description: "Device alert or alarm"
    lattice: estream.iot.alert
    path: /devices/{device_id}/alerts
    fields:
      alert_id:
        type: bytes32
      device_id:
        type: string(64)
      severity:
        type: AlertSeverity
      message:
        type: string
      triggered_at:
        type: timestamp
      acknowledged_at:
        type: timestamp
        optional: true
      resolved_at:
        type: timestamp
        optional: true
      trigger_value:
        type: f64
        optional: true
      threshold:
        type: f64
        optional: true
    retention:
      duration_days: 365

enums:
  AlertSeverity:
    values:
      - Info
      - Warning
      - Critical
      - Emergency
```

### 6.2 `esf-trading` — Trading Schema Pack

**Component:** `esf-trading`  
**Schemas:** EStreamOrder, EStreamFill, EStreamQuote, EStreamMarketData

```yaml
# schemas/trading-order.esf.yaml
version: "1.0"
namespace: estream.trading

schemas:
  EStreamOrder:
    description: "Order submission"
    lattice: estream.trading.order
    path: /trading/orders/{order_id}
    fields:
      order_id:
        type: bytes32
      client_order_id:
        type: string(64)
      instrument:
        type: string(32)
        description: "Instrument symbol"
      side:
        type: OrderSide
      order_type:
        type: OrderType
      quantity:
        type: u64
        description: "Quantity in base units"
      price:
        type: u64
        description: "Price in quote units (fixed-point, 8 decimals)"
        optional: true
      time_in_force:
        type: TimeInForce
      submitted_at:
        type: timestamp
      submitter:
        type: bytes32
        privacy: encrypted
    retention:
      duration_days: 365

  EStreamFill:
    description: "Trade execution / fill"
    lattice: estream.trading.fill
    path: /trading/fills/{fill_id}
    fields:
      fill_id:
        type: bytes32
      order_id:
        type: bytes32
      instrument:
        type: string(32)
      side:
        type: OrderSide
      quantity:
        type: u64
      price:
        type: u64
      fee:
        type: u64
        description: "Transaction fee in quote units"
      executed_at:
        type: timestamp
      venue:
        type: string(32)
        optional: true
    retention:
      duration_days: 365

  EStreamQuote:
    description: "Bid/ask quote"
    lattice: estream.trading.quote
    path: /trading/quotes/{instrument}
    fields:
      instrument:
        type: string(32)
      bid_price:
        type: u64
      bid_quantity:
        type: u64
      ask_price:
        type: u64
      ask_quantity:
        type: u64
      timestamp:
        type: timestamp
      source:
        type: string(32)
    retention:
      duration_hours: 24

  EStreamMarketData:
    description: "Market data snapshot (OHLCV)"
    lattice: estream.trading.marketdata
    path: /trading/market-data/{instrument}
    fields:
      instrument:
        type: string(32)
      interval:
        type: MarketDataInterval
      open:
        type: u64
      high:
        type: u64
      low:
        type: u64
      close:
        type: u64
      volume:
        type: u64
      timestamp:
        type: timestamp
    retention:
      duration_days: 365

enums:
  OrderSide:
    values: [Buy, Sell]
  OrderType:
    values: [Market, Limit, Stop, StopLimit]
  TimeInForce:
    values: [GTC, IOC, FOK, DAY]
  MarketDataInterval:
    values: [Tick, Sec1, Min1, Min5, Min15, Hour1, Day1]
```

### 6.3 `esf-carbon` — Carbon Markets Schema Pack

**Component:** `esf-carbon`  
**Schemas:** CarbonCredit, CarbonAttestation, CarbonMint

```yaml
# schemas/carbon-credit.esf.yaml
version: "1.0"
namespace: estream.carbon

schemas:
  CarbonCredit:
    description: "Verified carbon credit"
    lattice: estream.carbon.credit
    path: /carbon/credits/{credit_id}
    fields:
      credit_id:
        type: bytes32
      project_id:
        type: string(64)
        description: "Carbon offset project identifier"
      vintage_year:
        type: u16
        description: "Year the offset was generated"
      credit_type:
        type: CreditType
      tonnes_co2e:
        type: u64
        description: "Tonnes CO2 equivalent (fixed-point, 6 decimals)"
      registry:
        type: string(32)
        description: "Originating registry (Verra, Gold Standard, etc.)"
      serial_number:
        type: string(128)
        description: "Registry serial number"
      status:
        type: CreditStatus
      owner:
        type: bytes32
        privacy:
          default_level: encrypted
          audiences:
            - audience: owner
              level: public
            - audience: auditor
              level: public
      issued_at:
        type: timestamp
      retired_at:
        type: timestamp
        optional: true
    retention:
      duration_days: 3650

  CarbonAttestation:
    description: "Third-party verification attestation"
    lattice: estream.carbon.attestation
    path: /carbon/attestations/{attestation_id}
    fields:
      attestation_id:
        type: bytes32
      credit_id:
        type: bytes32
      verifier:
        type: bytes32
        description: "Verifier public key"
      methodology:
        type: string(64)
        description: "Verification methodology"
      verified_tonnes:
        type: u64
      confidence_pct:
        type: u8
        description: "Confidence level (0-100)"
      attested_at:
        type: timestamp
      signature:
        type: pq_signature
        description: "ML-DSA-87 attestation signature"
      evidence_hash:
        type: bytes32
        description: "SHA3-256 hash of supporting evidence"
    retention:
      duration_days: 3650

  CarbonMint:
    description: "Minting event for new carbon credits"
    lattice: estream.carbon.mint
    path: /carbon/mints/{mint_id}
    fields:
      mint_id:
        type: bytes32
      project_id:
        type: string(64)
      credit_ids:
        type: "array(bytes32)"
        description: "Credits minted in this batch"
      total_tonnes:
        type: u64
      minted_at:
        type: timestamp
      minter:
        type: bytes32
        privacy: encrypted
      attestation_ids:
        type: "array(bytes32)"
        description: "Supporting attestations"
      governance_approval:
        type: bytes32
        description: "Governance approval reference"
        optional: true
    retention:
      duration_days: 3650

enums:
  CreditType:
    values:
      - Avoidance
      - Removal
      - Sequestration
      - Reduction
  CreditStatus:
    values:
      - Active
      - Retired
      - Cancelled
      - Pending
      - Suspended
```

### 6.4 Schema Pack Manifest

Each pack uses an `estream-component.toml` of category `esf-schema`:

```toml
# esf-iot/estream-component.toml
[component]
name = "esf-iot"
version = "1.0.0"
category = "esf-schema"
description = "IoT domain schemas: Telemetry, Command, Alert"
license = "Apache-2.0"
keywords = ["iot", "telemetry", "devices", "edge"]

[component.author]
name = "eStream Contributors"

[component.schemas]
provides = ["IoTTelemetry", "IoTCommand", "IoTAlert"]
requires = []

[component.include]
schemas = ["schemas/*.esf.yaml"]
```

---

## 7. Phase 4: Enterprise Wire Adapters

Each adapter implements the `WireAdapter` trait (from [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md)) and is published as a `wire-adapter` component.

### 7.1 `estream-wire-fix` — FIX Protocol Adapter

**Protocol:** FIX 4.2 / 4.4 / 5.0  
**Market:** Capital markets, trading  
**Crate:** `crates/estream-wire-fix/`  
**License:** Apache 2.0

| Property | Value |
|----------|-------|
| Protocol family | `Financial` |
| Transports | TCP, TLS |
| Bidirectional | Yes |
| Request/Response | Yes (NewOrderSingle → ExecutionReport) |
| ESF schemas required | `esf-trading` (EStreamOrder, EStreamFill) |
| ESF schemas provided | FixNewOrderSingle, FixExecutionReport, FixMarketData |

**Key message mappings:**

| FIX Message | MsgType | ESF Schema |
|-------------|---------|-----------|
| NewOrderSingle | D | `FixNewOrderSingle` → `EStreamOrder` |
| ExecutionReport | 8 | `FixExecutionReport` → `EStreamFill` |
| MarketDataSnapshotFullRefresh | W | `FixMarketData` → `EStreamMarketData` |
| OrderCancelRequest | F | `FixOrderCancel` |
| OrderCancelReject | 9 | `FixOrderCancelReject` |

### 7.2 `estream-wire-hl7` — HL7 FHIR Adapter

**Protocol:** HL7 FHIR R4  
**Market:** Healthcare  
**Crate:** `crates/estream-wire-hl7/`  
**License:** Apache 2.0

| Property | Value |
|----------|-------|
| Protocol family | `Healthcare` |
| Transports | TCP (MLLP), TLS, HTTP (FHIR REST) |
| Bidirectional | Yes |
| Request/Response | Yes (FHIR REST) |
| ESF schemas provided | FhirPatient, FhirObservation, FhirEncounter, FhirConsent |

**Privacy requirements:** Healthcare data requires field-level privacy with HIPAA-compliant audience controls. All patient-identifying fields must use `encrypted` default with explicit `auditor` and `provider` audience grants.

### 7.3 `estream-wire-modbus` — Modbus Adapter

**Protocol:** Modbus TCP / RTU  
**Market:** Industrial IoT, SCADA  
**Crate:** `crates/estream-wire-modbus/`  
**License:** Apache 2.0

Upgrades the existing `ModbusTcpClient` in `crates/estream-industrial/` to implement the `WireAdapter` trait. See [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) §13.1 for migration path.

| Property | Value |
|----------|-------|
| Protocol family | `Industrial` |
| Transports | TCP, Serial (RTU) |
| Bidirectional | Yes |
| Request/Response | Yes (read/write registers) |
| ESF schemas provided | ModbusReadResponse, ModbusWriteResponse, ModbusEvent |

### 7.4 `estream-wire-swift` — ISO 20022 / SWIFT Adapter

**Protocol:** ISO 20022 (pacs.008, pacs.002, camt.053, camt.052)  
**Market:** Banking, payments  
**Crate:** `crates/estream-wire-swift/`  
**License:** Apache 2.0

Upgrades the existing `estream-iso20022` crate to implement the `WireAdapter` trait. See [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) §13.2 for migration path.

| Property | Value |
|----------|-------|
| Protocol family | `Financial` |
| Transports | TCP, TLS |
| Bidirectional | Yes |
| Request/Response | Yes |
| ESF schemas provided | Pacs008CreditTransfer, Pacs002StatusReport, Camt053Statement |

**Privacy requirements:** Financial data requires field-level privacy with PCI-DSS and regulatory audience controls.

---

## 8. Implementation Roadmap

### 8.1 Phase Ordering and Dependencies

```
Phase 1: Marketplace Infrastructure (3 weeks)
    │
    ├──▶ Phase 2a: WireAdapter trait + estream-wire-mqtt (2 weeks)
    │
    ├──▶ Phase 2b: Queue Stream Circuits + SDK (4 weeks)
    │       ├── ESCIR circuits: queue-append, queue-compact, queue-replicate
    │       ├── CPU ComputeCircuit implementations
    │       ├── estream-queue SDK crate (thin circuit invocation layer)
    │       └── FPGA synthesis + PRIME signer integration
    │
    ├──▶ Phase 2c: State Map Circuits + SDK (4 weeks)
    │       ├── ESCIR circuits: map-put, map-cas, map-gc, map-sync
    │       ├── CPU ComputeCircuit implementations
    │       ├── estream-map SDK crate (direct reads + circuit mutations)
    │       └── FPGA synthesis + ML-KEM hardware integration
    │
    ├──▶ Phase 3: Domain Schema Packs (2 weeks, parallel with Phase 2)
    │       ├── esf-iot
    │       ├── esf-trading
    │       └── esf-carbon
    │
    └──▶ Phase 4: Enterprise Wire Adapters (6 weeks, after Phase 2a + Phase 3)
            ├── estream-wire-fix (requires esf-trading)
            ├── estream-wire-hl7
            ├── estream-wire-modbus (upgrade)
            └── estream-wire-swift (upgrade)
```

### 8.2 Crate + Circuit Creation Summary

| Deliverable | Type | Phase | Dependencies |
|-------------|------|-------|-------------|
| `circuits/queue/append/` | ESCIR circuit | 2b | Lex streams, PRIME signer |
| `circuits/queue/compact/` | ESCIR circuit | 2b | Lex streams |
| `circuits/queue/replicate/` | ESCIR circuit | 2b | VRF Scatter |
| `crates/estream-queue/` | SDK crate | 2b | `estream-kernel` (ComputeCircuit, LexStream) |
| `circuits/map/put/` | ESCIR circuit | 2c | Lex state, ML-KEM |
| `circuits/map/cas/` | ESCIR circuit | 2c | Lex state |
| `circuits/map/gc/` | ESCIR circuit | 2c | Lex state |
| `circuits/map/sync/` | ESCIR circuit | 2c | VRF Scatter |
| `crates/estream-map/` | SDK crate | 2c | `estream-kernel` (ComputeCircuit, LexState) |
| `crates/estream-wire-mqtt/` | Adapter crate | 2a | `estream-kernel` (WireAdapter trait) |
| `crates/estream-wire-fix/` | Adapter crate | 4 | WireAdapter, `esf-trading` |
| `crates/estream-wire-hl7/` | Adapter crate | 4 | WireAdapter |
| `crates/estream-wire-modbus/` | Adapter crate (upgrade) | 4 | WireAdapter, `estream-industrial` |
| `crates/estream-wire-swift/` | Adapter crate (upgrade) | 4 | WireAdapter, `estream-iso20022` |

### 8.3 Test Strategy

Each component includes:
1. **Unit tests** — Within the crate (`#[cfg(test)]`)
2. **Golden test vectors** — In `tests/golden/` per [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) §5
3. **Journey tests** — Registered in `crates/estream-test/` under `JourneyCategory::Marketplace`
4. **Integration tests** — Cross-crate tests verifying end-to-end data flow

---

## 9. Success Criteria

### 9.1 Ecosystem & Experience

| Criterion | Metric | Target |
|-----------|--------|--------|
| Ecosystem adoption | Community-contributed components | 5+ within 6 months |
| Developer experience | `estream marketplace install` time | < 30 seconds |
| Security | Published component signature coverage | 100% ML-DSA-87 signed |
| Interoperability | Wire adapters integrated without lock-in | 4+ protocol adapters |
| Enterprise funnel | OSS → BSL → managed conversion rate | Measurable pipeline |

### 9.2 Performance — CPU Target (SmartCircuit on Rust)

| Metric | Target | Chronicle Comparison |
|--------|--------|---------------------|
| Queue append (unsigned) | < 100 ns | 10× faster than Chronicle Queue |
| Queue append (ML-DSA-87 signed) | < 400 μs | N/A (Chronicle has no PQ signing) |
| Queue subscribe latency | < 50 ns | Comparable |
| Map get (direct read) | < 50 ns | Comparable to Chronicle Map off-heap |
| Map put (circuit + witness) | < 200 ns | 2× faster (no JVM overhead) |
| Map CAS (circuit) | < 400 ns | Comparable |
| Queue throughput (unsigned) | 100M msg/sec | 5× Chronicle's GC-limited ceiling |
| Map throughput (puts) | 50M ops/sec | 5× Chronicle Map |

### 9.3 Performance — FPGA Target (SmartCircuit on Hardware)

| Metric | Target | vs Chronicle | vs CPU Target |
|--------|--------|-------------|---------------|
| Queue append (unsigned) | < 40 ns | **50× faster** | 2.5× faster |
| Queue append (signed, 4× PRIME) | < 100 μs | N/A | 4× faster |
| Map get (CAM hit) | < 5 ns | **10× faster** | 10× faster |
| Map put (pipeline) | < 60 ns | **~2× faster** | 3× faster |
| Queue throughput (unsigned) | 200M msg/sec | **10× Chronicle** | 2× CPU |
| Map throughput (puts) | 200M ops/sec | **20× Chronicle** | 4× CPU |
| Tail latency (p99) | Deterministic | **No GC spikes** | No jitter |
| Latency variance | 0 ns (constant-time) | ∞× improvement | Orders of magnitude |

---

## References

- [EPIC_ESTREAM_MARKETPLACE.md](../../.github/epics/EPIC_ESTREAM_MARKETPLACE.md) — Epic overview
- [WIRE_ADAPTER_TRAIT_SPEC.md](../protocol/WIRE_ADAPTER_TRAIT_SPEC.md) — WireAdapter trait (#528)
- [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) — Registry API (#525)
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema composition (#526)
- [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) — Package format (#527)
- [CONSOLE_WIDGET_MARKETPLACE_SPEC.md](./CONSOLE_WIDGET_MARKETPLACE_SPEC.md) — Widget marketplace (#533)
- [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md) — Pricing, visibility, creator program
- [FPGA_COMPONENT_EXTENSION.md](./FPGA_COMPONENT_EXTENSION.md) — FPGA component metadata
- [INDUSTRIAL_PROTOCOL_GATEWAY_V2.md](./INDUSTRIAL_PROTOCOL_GATEWAY_V2.md) — Layered adapter architecture
- [Chronicle Queue](https://github.com/OpenHFT/Chronicle-Queue) — Open source model reference
- [Chronicle Map](https://github.com/OpenHFT/Chronicle-Map) — Distributed state model

---

*Created: 2026-02-11*  
*Updated: 2026-02-12 — Cross-references updated to Stream Architecture Spec v0.8.1*  
*Status: Draft*  
*Issue: #524*
