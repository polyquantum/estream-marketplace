# Badge System Documentation

> Trust signals and capability indicators for eStream Marketplace components.

---

## Overview

Badges are visual indicators displayed on component cards and detail views. They communicate trust, provenance, and capabilities at a glance. Each badge has specific criteria that must be met before it is assigned.

---

## Badge Types

### Verified

**Appearance:** Green pill (`#10B981`) with white text
**Label:** `VERIFIED`

**Meaning:** The component publisher's identity has been confirmed through SPARK biometric attestation. This badge indicates that a real, verified individual or organization stands behind the component.

**Criteria:**
- Publisher has completed SPARK biometric enrollment
- SPARK identity attestation is current (not expired or revoked)
- Publisher's ML-DSA-87 public key is registered in the component registry
- Identity has been cross-verified against the publisher's claimed organization (if applicable)

**What it does NOT mean:**
- It does not certify the component is free of bugs
- It does not guarantee the component meets any particular quality standard
- It does not imply endorsement by PolyQuantum

---

### Official

**Appearance:** Blue pill (`#1E40AF`) with white text
**Label:** `OFFICIAL`

**Meaning:** The component is published and maintained by the PolyQuantum team. Official components are part of the core eStream ecosystem and receive direct engineering support.

**Criteria:**
- Published by a PolyQuantum team member with an `@polyquantum` SPARK identity
- Maintained in the `polyquantum/estream` monorepo or an official PolyQuantum repository
- Subject to PolyQuantum's internal code review and testing standards

**Examples of official components:**
- `estream-wire-fix` — FIX protocol adapter
- `estream-wire-mqtt` — MQTT protocol adapter
- `data-trading` — Financial trading data schemas
- `data-iot` — IoT sensor data schemas
- `data-carbon` — Carbon credit data schemas

---

### PQ-Signed

**Appearance:** Violet pill (`#7C3AED`) with white text
**Label:** `PQ-SIGNED`

**Meaning:** The component's archive has been signed with an ML-DSA-87 post-quantum digital signature. This ensures the component has not been tampered with and that its provenance can be cryptographically verified — even against future quantum computing attacks.

**Criteria:**
- A valid `SIGNATURE.ml-dsa` file is present in the component package
- The signature uses the ML-DSA-87 algorithm (FIPS 204, NIST Level 5)
- The signature verifies against the archive contents
- The signing key is registered in the component registry

**Verification:**
```bash
estream marketplace verify <component-name>
```

**Technical details:**
- Algorithm: ML-DSA-87 (lattice-based, NIST Level 5)
- Signature size: 4,627 bytes
- Security: 256-bit classical, 128-bit quantum

---

### FPGA-Ready

**Appearance:** Amber pill (`#F59E0B`) with dark text (`#0F172A`)
**Label:** `FPGA-READY`

**Meaning:** The component includes synthesizable Verilog output and is designed to run on FPGA hardware. FPGA-ready components provide orders-of-magnitude performance improvements over CPU-only execution for latency-critical operations.

**Criteria:**
- Component's `target` field includes `"fpga"` in `[component.circuits]`
- The `[component.include]` section includes `fpga` glob patterns (e.g., `fpga/*.v`)
- Verilog files are present and included in the published archive
- Component category is `fpga-circuit` (primary) or `smart-circuit` with dual targets

**Manifest example:**
```toml
[component.circuits]
provides = ["ntt_accelerator"]
target = ["cpu", "fpga"]

[component.include]
circuits = ["circuits/*.fl"]
fpga = ["fpga/*.v", "fpga/*.xdc"]
```

---

### Community

**Appearance:** Outlined pill (transparent background, `#475569` border) with gray text (`#94A3B8`)
**Label:** `COMMUNITY`

**Meaning:** The component is a third-party contribution from the eStream community. Community components are not published or maintained by PolyQuantum, but they have been published through the standard marketplace pipeline with ML-DSA-87 signing.

**Criteria:**
- Publisher is not a PolyQuantum team member
- Component was published through the standard `estream marketplace publish` pipeline
- The component does not have the **Official** badge

**Note:** Community and Official badges are mutually exclusive. A component is either Official (published by PolyQuantum) or Community (published by a third party).

---

## Badge Display Rules

### Ordering

Badges are always displayed in this fixed order:

1. Verified
2. Official
3. PQ-Signed
4. FPGA-Ready
5. Community

### Combinations

Common badge combinations:

| Combination | Typical Scenario |
|-------------|-----------------|
| Verified + Official + PQ-Signed | Core eStream component from PolyQuantum |
| Verified + Official + PQ-Signed + FPGA-Ready | Core FPGA-accelerated component |
| Verified + PQ-Signed + Community | Third-party component with verified publisher |
| PQ-Signed + Community | Third-party component with unverified publisher |
| Verified + PQ-Signed + FPGA-Ready + Community | Third-party FPGA component from a verified publisher |

### Mutual Exclusivity

- **Official** and **Community** are mutually exclusive
- All other badges can be combined freely

### Minimum Badges

All properly published components have at least **PQ-Signed** (because `estream marketplace publish` always signs with ML-DSA-87).

---

## Badge Rendering Specifications

See [Brand Guidelines](./BRAND_GUIDELINES.md) for visual specifications including:
- Colors and typography
- Pill dimensions and border radius
- Hover states and accessibility requirements

---

## See Also

- [Brand Guidelines](./BRAND_GUIDELINES.md) — Visual specifications
- [Security Model](../docs/security-model.md) — ML-DSA-87 and SPARK authentication
- [Component Guide](../docs/component-guide.md) — How to earn the FPGA-Ready badge
