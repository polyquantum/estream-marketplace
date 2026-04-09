# Frequently Asked Questions

> Common questions about the eStream Marketplace.

---

## General

### How is the eStream Marketplace different from npm or crates.io?

The eStream Marketplace is **domain-specific** — it distributes eStream components (data schemas, SmartCircuits, wire adapters, FPGA circuits, console widgets, and integrations), not general-purpose libraries. Key differences:

| Feature | npm / crates.io | eStream Marketplace |
|---------|----------------|-------------------|
| **Content** | JavaScript/Rust packages | SmartCircuit components (.fl, .data.yaml, .v) |
| **Signing** | GPG / Sigstore | ML-DSA-87 post-quantum signatures |
| **Execution** | Developer toolchain | Dual-target: CPU (Rust/WASM) + FPGA (Verilog) |
| **Schema system** | None built-in | provides/requires data contracts with resolution |
| **Pricing** | Free only | 6 pricing models (free through enterprise) |
| **Visibility** | Source always open | 4 levels (open, interface, compiled, licensed) |
| **Identity** | Email/OAuth | SPARK biometric attestation |

---

### What makes it post-quantum secure?

Every component is signed with **ML-DSA-87** (FIPS 204), a lattice-based signature scheme standardized by NIST in 2024. ML-DSA-87 provides NIST Level 5 security — resistant to both classical (256-bit) and quantum (128-bit) attacks.

See the [Security Model](./security-model.md) for full details.

---

### Can I publish proprietary components?

Yes. The marketplace supports 4 visibility levels:

- **`open`** — Full source code visible
- **`interface`** — Only type signatures and API surface exposed
- **`compiled`** — Only compiled WASM/Verilog binary distributed
- **`licensed`** — Access governed by enterprise license terms

```toml
[component.marketplace]
pricing = "one-time"
visibility = "compiled"
```

---

## FPGA Components

### How does FPGA component distribution work?

FPGA components (`fpga-circuit` category) contain both FastLang source (`.fl`) and synthesizable Verilog (`.v`). The FLIR compiler generates Verilog from FastLang, and the component may include pre-synthesized bitstreams.

```toml
[component.circuits]
provides = ["ntt_accelerator"]
target = ["cpu", "fpga"]

[component.include]
circuits = ["circuits/*.fl"]
fpga = ["fpga/*.v", "fpga/*.xdc"]
```

FPGA-ready components receive the **FPGA-Ready** badge. See [Badge Descriptions](../branding/badge-descriptions.md).

---

## Versioning and Compatibility

### What about versioning and breaking changes?

Components use **semantic versioning** (semver):

| Version Part | Meaning |
|-------------|---------|
| Major (1.x.x) | Breaking changes to schemas, circuits, or APIs |
| Minor (x.1.x) | New features, backward-compatible additions |
| Patch (x.x.1) | Bug fixes, documentation updates |

**Version pinning** during install:

```bash
estream marketplace install estream-wire-fix@1.0.0
```

The dependency DAG detects conflicts during installation. If a version conflict is detected, the install fails with error code `2` (`VERSION_CONFLICT`).

---

### How does eStream version compatibility work?

Components declare the eStream platform versions they support:

```toml
[component.estream]
min_version = "0.8.0"
max_version = "1.0.0"     # optional
```

---

## Publishing

### What happens when I publish a component?

1. **Validate** — Parse `estream-component.toml`, check required fields
2. **Resolve** — Expand include glob patterns, verify files exist
3. **Check** — Run `compile --check` on `.fl` files
4. **Archive** — Build deterministic `tar.gz`
5. **Sign** — Generate ML-DSA-87 signature
6. **Emit** — Write archive and `SIGNATURE.ml-dsa`

Use `--dry-run` to validate without signing:

```bash
estream marketplace publish my-component --dry-run
```

---

### Can I update a published component?

Yes — bump the version and publish again. Each version is independently signed. Previous versions remain available.

---

## Security

### How do I verify a component I've installed?

```bash
estream marketplace verify <component-name>
```

### What if a publisher's key is compromised?

Key revocation is managed through eStream's governance layer. Previously published components remain valid. The publisher re-authenticates via SPARK to derive a new key pair.

### Why no TypeScript for crypto operations?

TypeScript runs in environments with shared-memory access patterns, NPM supply chain risk, and no constant-time guarantees. All crypto is restricted to Rust/WASM.

---

## Getting Help

- [Getting Started](./getting-started.md) — 5-minute quickstart
- [Component Guide](./component-guide.md) — Full authoring reference
- [CLI Reference](./cli-reference.md) — Every command documented
- [Security Model](./security-model.md) — Post-quantum security deep dive
- [API Reference](./api-reference.md) — Stream API for programmatic access
- [Pricing Guide](./pricing-guide.md) — Pricing models and visibility levels
