# ASIC Hard IP Licensing Extension

> **Version**: 0.1.0
> **Status**: Draft
> **Epic**: estream-marketplace#7
> **Depends On**: ESTREAM_MARKETPLACE_SPEC.md (v2.0.0), FPGA_COMPONENT_EXTENSION.md

## 1. Overview

Extends the eStream Marketplace licensing model to support ASIC hard IP — design artifacts that are fabricated permanently into silicon. Unlike FPGA bitstreams (reprogrammable), ASIC IP commits to physical fabrication and requires different visibility, pricing, escrow, and verification models.

All four marketplace licensing models (open source, licensed source, mixed composite, derivative) apply to ASIC IP. The composite model maps directly to SoC design: an SoC composes hard IP blocks from multiple vendors with mixed licensing, and the Stratum DAG + Cortex sync machinery tracks version evolution across tapeout revisions.

## 2. ASIC vs FPGA Comparison

| Dimension | FPGA | ASIC Hard IP |
|-----------|------|-------------|
| Artifact | Bitstream (reprogrammable) | GDSII/LEF/LIB (fabricated permanently) |
| Reversibility | Re-flash at any time | Cannot change after tapeout |
| Verification | Timing closure per-board | Signoff across PVT corners, DRC/LVS |
| Licensing unit | Per-device or per-site | Per-tapeout, per-wafer, per-die, royalty-per-unit |
| IP protection | Bitstream encryption | Foundry NDA + physical obfuscation + split manufacturing |
| Escrow | Not needed (reprogrammable) | May require GDSII escrow for business continuity |
| Liability | Limited (re-flash to fix) | High (mask set cost $500K-$5M+, no re-spin without new tapeout) |

## 3. ASIC Visibility Levels

Extends `SourceVisibility` with silicon-specific artifact tiers:

```
AsicVisibility {
    BehavioralOnly    = 0,  // RTL behavioral model (Verilog/VHDL), no synthesis results
    SynthNetlist      = 1,  // Gate-level netlist (post-synthesis, pre-PnR)
    TimingAbstract    = 2,  // Liberty (.lib) + LEF abstract — enough for top-level integration
    HardMacro         = 3,  // Opaque LEF/LIB only — black-box with pin geometry
    FullPhysical      = 4,  // GDSII + LEF + LIB + timing models + DRC/LVS clean deck
    EscrowedFull      = 5,  // Full GDSII in escrow, released on trigger conditions
}
```

### Visibility Hierarchy

```
BehavioralOnly ⊂ SynthNetlist ⊂ TimingAbstract ⊂ HardMacro ⊂ FullPhysical ⊂ EscrowedFull
```

Each level includes all artifacts from lower levels. A licensee with `FullPhysical` access also has `TimingAbstract`, `SynthNetlist`, and `BehavioralOnly`.

### Always Public (regardless of visibility)

- IP block name, version, publisher
- Pin/port interface (signal names, directions, bus widths)
- Target process node (e.g., TSMC 7nm, GF 12nm)
- Area estimate (um^2)
- Power estimate (mW at target frequency)
- Timing summary (setup/hold at nominal corner)
- Source hash (for integrity verification)

## 4. ASIC Pricing Models

```
AsicPricing {
    PerTapeout {
        base_fee: u64,
        includes_corners: u8,       // Number of PVT corners included
        includes_process_nodes: u8,  // Number of foundry nodes included
    },
    RoyaltyPerUnit {
        rate_basis_points: u16,      // Per-die royalty as fraction of die cost
        minimum_units: u64,          // Minimum volume before royalty kicks in
        cap_per_unit_micros: u64,    // Maximum royalty per unit (prevents runaway)
    },
    SiteLicense {
        annual_fee: u64,
        foundry_restricted: bool,    // Locked to specific foundry
        foundry_id: bytes(32),       // If restricted, which foundry
        design_team_limit: u16,      // Max concurrent designers
    },
    Subscription {
        annual_fee: u64,
        includes_updates: bool,      // New tapeout revisions included
        includes_migration: bool,    // Process node migration assistance
    },
    NRE {
        engineering_fee: u64,
        deliverable: AsicVisibility, // What level of artifact is delivered
        milestone_count: u8,         // Payment milestones
        includes_signoff: bool,      // Includes DRC/LVS/timing signoff
    },
}
```

### Revenue Waterfall for SoC Composites

An SoC tapeout triggers atomic multi-party settlement through the composite chain:

```
SoC Tapeout Fee
  → SoC Integrator share (derivative publisher)
    → CPU Core IP vendor (sub-circuit, royalty-per-unit)
    → Memory Controller IP vendor (sub-circuit, per-tapeout)
    → PHY IP vendor (sub-circuit, site license annual)
    → Platform fee
```

Each sub-circuit in the SoC composite has its own `AsicPricing` model. The `DerivativeManifest` and `WaterfallConfig` from existing circuits compose directly — the SoC integrator registers a derivative of the hard IP composite, and settlement cascades atomically.

## 5. ASIC Escrow Model

For `EscrowedFull` visibility, GDSII is held in scatter-CAS escrow with release conditions:

```
EscrowConfig {
    escrow_id: bytes(32),
    ip_block_id: PackageRef,
    gdsii_scatter_ref: bytes(32),      // scatter-CAS ref to encrypted GDSII
    encryption_key_shares: u8,          // k-of-n threshold for key release
    release_conditions: list<EscrowTrigger>,
    escrow_agent_lex: LexPath,          // Neutral third-party lex
    created_at: u64,
    expires_at: u64,
}

EscrowTrigger {
    trigger_type: u8,
    // 0 = VendorBankruptcy (registered agent files notice)
    // 1 = ContractBreach (arbitration ruling, witnessed)
    // 2 = DiscontinuedSupport (vendor stops maintaining IP for >N months)
    // 3 = AcquisitionChange (vendor acquired, new owner doesn't honor terms)
    // 4 = TimeBased (automatic release after N years)
    threshold_witnesses: u8,            // k-of-n witnesses required to trigger
    verification_lex: LexPath,          // Lex governing trigger verification
}
```

Escrow release is a governance action requiring k-of-n witness attestation in the escrow agent's lex. The GDSII encryption key is split via Shamir's Secret Sharing (or ML-KEM-1024 threshold equivalent) across the witness set.

## 6. ASIC Verification Badges

Extends the FPGA badge system with silicon-specific verification:

| Badge | Meaning | Verification |
|-------|---------|-------------|
| `DrcClean` | Passes DRC for target process | Automated DRC run, signed report |
| `LvsClean` | Layout matches schematic | Automated LVS run, signed report |
| `TimingSignedOff` | Meets timing across all PVT corners | STA results at SS/TT/FF corners |
| `PowerSignedOff` | Meets power budget at target workload | Power analysis with switching activity |
| `FormallyVerified` | RTL equivalence proven (synth vs behavioral) | Formal equivalence check report |
| `SiliconProven` | Fabricated and tested successfully | Test chip measurement data |
| `ProcessPortable` | Available on 2+ foundry nodes | Multi-node timing/area/power reports |
| `RadHard` | Radiation-hardened (space/defense) | TID/SEE test reports |

## 7. Foundry Constraints

ASIC IP licensing often includes foundry-specific restrictions:

```
FoundryConstraint {
    foundry_id: bytes(32),
    foundry_name: bytes(64),
    process_node: bytes(32),           // e.g., "TSMC N7", "GF 12LP"
    requires_nda: bool,                // Foundry NDA required before delivery
    requires_foundry_approval: bool,   // Foundry must approve IP usage
    restricted_to_foundry: bool,       // IP only works on this foundry
    pdk_version: bytes(32),            // Required PDK version
}
```

The marketplace enforces foundry constraints at install time: if the consumer's target foundry doesn't match, the resolver blocks installation and explains why.

## 8. Mapping to Marketplace Licensing Models

### Model 1: Open Source ASIC IP

- Visibility: `BehavioralOnly` (RTL source open)
- License: Apache-2.0 or CERN-OHL-S-2.0
- Example: RISC-V cores (lowRISC Ibex), open PHYs
- Fork/upstream: unrestricted, community-improved
- Cortex tracks process node migration across forks

### Model 2: Licensed Source ASIC IP

- Visibility: `FullPhysical` for licensees, `TimingAbstract` for evaluation
- License: Commercial with per-tapeout or royalty pricing
- Example: ARM Cortex-M cores, Synopsys DesignWare IP
- Internal fork with Cortex sync when vendor publishes new revision
- Org-internal modifications (custom pin muxing, power domain changes) tracked in Stratum DAG

### Model 3: Mixed Composite SoC

- SoC composes IP blocks with different visibility per block
- CPU core: licensed source (FullPhysical)
- Memory controller: compiled-only (HardMacro, opaque LEF/LIB)
- Custom accelerator: open source (BehavioralOnly)
- PHY: escrowed (EscrowedFull with business continuity trigger)
- Each block's entitlement resolved per consumer's license tokens

### Model 4: Derivative SoC

- SoC integrator wraps IP blocks, adds custom logic, republishes
- Revenue waterfall: integrator → IP vendors → platform, all atomic
- Visibility constraints inherited: cannot expose more than original allows
- New tapeout revision triggers Cortex sync for all derivative consumers

## 9. Integration with Existing Circuits

| Existing Circuit | ASIC Extension |
|-----------------|----------------|
| `composite_visibility.fl` | `register_composite_visibility` with ASIC sub-blocks using `AsicVisibility` mapped to `SourceVisibility` |
| `composite_visibility.fl` | `resolve_effective_entitlements` works unchanged — `AsicVisibility` levels map to the 4-level `SourceVisibility` |
| `revenue_waterfall.fl` | `execute_waterfall` handles SoC tapeout fees with the same atomic settlement |
| `package_fork.fl` | `create_fork` / `create_internal_override` track IP block customizations |
| `cortex_upstream_sync.fl` | `on_upstream_version_published` notifies when IP vendor publishes new revision |
| `internal_marketplace.fl` | `publish_to_org_registry` for org-internal IP blocks |
| `blinded_tokens.fl` | License tokens work for ASIC IP evaluation and purchase |
| `settlement.fl` | Per-tapeout and royalty-per-unit settle through same escrow mechanism |

## 10. Implementation Notes

The ASIC extension is primarily a **type extension** — new visibility levels, pricing models, and escrow types — layered on the existing marketplace infrastructure. No new state machines or settlement circuits are needed. The composite/derivative/entitlement circuits from `composite_visibility.fl` handle ASIC IP identically to FPGA or software circuits because they operate on the abstract `SourceVisibility` enum.

The only net-new circuit needed is the **escrow management** circuit for `EscrowedFull` visibility, which manages the scatter-CAS-held GDSII encryption key shares and the witness-attested release conditions. This composes with the existing lex boundary nesting and cross-lex bridge infrastructure.
