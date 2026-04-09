# Pricing and Licensing Guide

> How to price your components and control source visibility on the eStream Marketplace.

---

## Overview

The eStream Marketplace supports 6 pricing models and 4 visibility levels, giving component authors full control over monetization and intellectual property protection.

---

## The 6 Pricing Models

### `free`

No cost. The component is available to all users at no charge.

**Best for:** Open source components, community contributions, reference implementations, data schemas.

```toml
[component.marketplace]
pricing = "free"
visibility = "open"
```

**Example:** `data-iot` — IoT sensor data schemas (Apache 2.0, open source).

---

### `one-time`

Single purchase. The user pays once and receives perpetual access to the purchased version.

**Best for:** Standalone circuits, specialized adapters, utility components with a clear deliverable.

```toml
[component.marketplace]
pricing = "one-time"
visibility = "compiled"
```

---

### `subscription`

Recurring payment (monthly or annual). Access is valid while the subscription is active. Updates are included.

**Best for:** Components that require ongoing maintenance, continuous updates, or access to evolving services.

```toml
[component.marketplace]
pricing = "subscription"
visibility = "interface"
```

---

### `usage-based`

Pay-per-use pricing metered via eStream's 8D resource metering. Charges are based on actual circuit executions, data volume, or other metered dimensions.

**Best for:** High-volume processing circuits, per-transaction services, metered APIs.

```toml
[component.marketplace]
pricing = "usage-based"
visibility = "compiled"
```

---

### `enterprise`

Custom pricing negotiated directly between the publisher and the enterprise customer. Typically includes SLA guarantees, dedicated support, and custom deployment options.

**Best for:** Mission-critical components, compliance-sensitive integrations, components requiring dedicated support channels.

```toml
[component.marketplace]
pricing = "enterprise"
visibility = "licensed"
```

---

### `freemium`

Free tier with limited functionality or capacity. Paid upgrade unlocks full capabilities.

**Best for:** Components where users benefit from trying before buying, or where a free tier drives adoption of premium features.

```toml
[component.marketplace]
pricing = "freemium"
visibility = "interface"
```

---

## The 4 Visibility Levels

Visibility controls how much of the component source code is accessible to consumers.

| Level | Source Code | Interface | Compiled Binary | Description |
|-------|------------|-----------|----------------|-------------|
| `open` | Full access | Full access | Full access | Complete source code visible |
| `interface` | Hidden | Visible | Full access | Only type signatures and API surface exposed |
| `compiled` | Hidden | Hidden | Full access | Only compiled binary (WASM/Verilog) distributed |
| `licensed` | Negotiable | Negotiable | Full access | Access governed by enterprise license terms |

---

## Setting Pricing in Your Manifest

Add the `[component.marketplace]` section to your `estream-component.toml`:

```toml
[component.marketplace]
pricing = "free"          # free | one-time | subscription | usage-based | enterprise | freemium
visibility = "open"       # open | interface | compiled | licensed
```

Both fields are optional and default to `pricing = "free"` and `visibility = "open"` when omitted.

### Validation

The CLI validates pricing and visibility values during `estream marketplace publish`:

```bash
estream marketplace publish .
# Error: Invalid pricing 'pay-what-you-want'. Must be one of: free, one-time, subscription, usage-based, enterprise, freemium
```

---

## Licensing Tiers

The eStream platform operates a three-tier licensing model:

| Tier | License | What's Included |
|------|---------|-----------------|
| **Open Source** | Apache 2.0 | Queue, Map, Wire adapters, Data schemas, SDK, CLI |
| **Source Available** | BSL 1.1 | FPGA acceleration, VRF Scatter HA, production runtime |
| **Commercial** | Enterprise | Managed deployment, SLA, support, custom adapters |

---

## Pricing Model Comparison

| Model | Payment | Access Duration | Updates | Best For |
|-------|---------|----------------|---------|----------|
| Free | None | Perpetual | All versions | OSS, community |
| One-Time | Single | Perpetual (purchased version) | Purchased version only | Standalone tools |
| Subscription | Recurring | While active | All versions | Maintained services |
| Usage-Based | Per-use | While metered | All versions | High-volume processing |
| Enterprise | Negotiated | Per agreement | Per agreement | Mission-critical |
| Freemium | None / Upgrade | Perpetual (free tier) | All versions | Try-before-buy |

---

## See Also

- [Component Guide](./component-guide.md) — Full manifest reference
- [Security Model](./security-model.md) — How signing protects paid components
- [FAQ](./faq.md) — Common pricing questions
