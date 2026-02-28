# estream-portal

Embeddable widget framework — extends the `@estream/embed` architecture with partner engagement widgets, RBAC-gated interactions, and sandboxed consent flows in iframes. Generic platform integration product.

## Overview

`estream-portal` builds on the `@estream/embed` pattern (loader script, iframe sandbox, postMessage bridge, WASM runtime) to deliver secure, embeddable widgets that partners can drop into any web property. Each widget runs inside a sandboxed iframe with a signed postMessage bridge — no direct DOM access, no cookie sharing, no ambient authority.

Two new widget types extend the embed surface:

- **PartnerConnect** — partner onboarding and profile linking widget
- **EngagementDashboard** — real-time view of activity scores, compensation history, and consent status

Four new bridge events power the consent flow integration with `estream-optin`:

| Event | Direction | Description |
|-------|-----------|-------------|
| `optin:preview` | HostToWidget | Partner previews the engagement offer before user sees it |
| `optin:consent` | WidgetToHost | User grants consent — triggers `optin_activate_code` |
| `optin:reject` | WidgetToHost | User declines the engagement offer |
| `optin:settled` | HostToWidget | Settlement confirmation pushed back to the widget |

Every bridge message is ML-DSA-87 signed with origin verification, protocol version enforcement (`estream-embed-v1`), and constant-time validation to prevent timing side-channels. Widget creation is RBAC-gated — the `portal_create_widget` circuit checks user roles before instantiating any widget.

## Circuits

| Circuit | File | Description |
|---------|------|-------------|
| `portal_widget` | `circuits/portal_widget.fl` | Widget lifecycle — creation, state transitions, destruction, RBAC enforcement |
| `portal_bridge` | `circuits/portal_bridge.fl` | PostMessage bridge — signed message creation, origin validation, optin event routing |

## Stratum Usage

- **KV storage** — fast lookup for widget instances, widget configs, and bridge messages
- **Series** — 90d retention for widget instances, 365d for RBAC checks, 30d for bridge messages and validations
- **Streams** — widget events consumed by StreamSight and analytics; bridge events consumed by StreamSight and security

## Install

```
estream marketplace install estream-portal
```

## Security

- ML-DSA-87 signatures on every bridge message
- ML-KEM-1024 key encapsulation for transport security
- RBAC enforcement on widget creation (viewer, operator roles)
- Origin verification and protocol version checks on all incoming bridge messages
- Constant-time bridge validation prevents timing side-channel attacks
- PoVC attestation on RBAC checks for tamper-evident audit
- Sandboxed iframe isolation — no direct DOM access or cookie sharing
- StreamSight anomaly detection on every circuit

## Dependencies

- `estream-optin ^0.1.0` — opt-in engagement model (consent flows, engagement codes, escrow)
