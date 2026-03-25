# Poly Messenger — eStream Marketplace Component

**Publisher:** Poly Labs (`polylabs`)
**Version:** 0.12.0
**Category:** Smart Circuit
**Wire Protocol:** 0xB0–0xB5 (UDP)

## Overview

Post-quantum encrypted messaging with blind relay routing, double-ratchet forward secrecy, enterprise RBAC governance, incognito mode, and agent-to-agent/human-to-agent wire protocol integration.

## Circuits (8)

| Circuit | Description |
|---------|-------------|
| `polymsg_encrypt` | ML-KEM-1024 session establishment, AES-256-GCM encrypt/decrypt, key rotation |
| `polymsg_ratchet` | Double-ratchet with symmetric chain advance, out-of-order tolerance, skipped key tracking |
| `polymsg_relay` | Blind relay mesh routing: path selection, onion layering, cover traffic, constant-size 4096B packets |
| `polymsg_classify` | ESLM content classification with human-in-the-loop feedback |
| `polymsg_rbac` | Enterprise role-based access: admin, moderator, member, guest, auditor permissions |
| `polymsg_incognito` | Incognito mode: ephemeral keys, session isolation, metadata scrubbing |
| `polymsg_metering` | Per-invocation usage metering across 8 resource dimensions |
| `polymsg_platform_health` | Relay mesh health monitoring, circuit health probes, platform-level anomaly detection |

## Graphs (2) + DAG (1)

| Structure | Type | Description |
|-----------|------|-------------|
| `polymsg_contact_graph` | `graph` | Contact network: human contacts, agent contacts, groups, trust levels |
| `polymsg_relay_graph` | `graph` | Relay mesh: relay nodes, health scores, geographic zones, route weights |
| `polymsg_conversation_dag` | `dag` | Message threads: message ordering, lifecycle states, thread overlays |

## Wire Protocol

Poly Messenger extends the eStream wire protocol with six packet types in the 0xB0–0xB5 range:

| Code | Name | Direction |
|------|------|-----------|
| 0xB0 | `MsgAgentMessage` | Agent → Agent |
| 0xB1 | `MsgAgentMessageAck` | Agent → Agent (ACK) |
| 0xB2 | `MsgAgentRegistryUpdate` | Agent → Node (presence) |
| 0xB3 | `MsgRelayForward` | Node → Node (onion relay) |
| 0xB4 | `MsgHumanMessage` | Human → Agent |
| 0xB5 | `MsgStructuredMessage` | Any → Any (work assignments, sprint notifications) |

## Dependencies

- `polykit-identity` ≥ 0.2.0 (SPARK identity, ML-DSA-87, ML-KEM-1024)
- `polykit-metering` ≥ 0.2.0 (8-dimension resource metering)
- `polykit-telemetry` ≥ 0.2.0 (StreamSight telemetry pipeline)
- `polykit-rate-limiter` ≥ 0.2.0 (Per-tier rate limiting)

## Source

- **Repository:** [polylabs-dev/polymessenger](https://github.com/polylabs-dev/polymessenger)
- **Rust crates:** `poly-core` (runtime), `poly-sdk` (client SDK)
