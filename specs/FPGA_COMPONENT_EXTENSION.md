# FPGA Component Extension for Marketplace

> Extension to Component Marketplace Specification v0.8.0 for hardware (FPGA) components.

**Status:** Draft  
**Dependencies:** MARKETPLACE_SPEC.md, HARDWARE_TIER_SPEC.md

## Overview

FPGA components differ from ESCIR (circuit) components in several ways:
- Resource requirements measured in LUTs/BRAM/DSP, not compute/memory/witness
- Performance measured in throughput/latency, not execution time
- Artifacts are Verilog/VHDL, not WASM
- Deployment targets specific FPGA families

This extension defines the additional metadata and marketplace behavior for FPGA components.

---

## Component Type

```rust
pub enum ComponentType {
    /// Software circuit (ESCIR → WASM)
    Circuit,
    
    /// Hardware accelerator (Verilog/VHDL → Bitstream)
    Fpga,
    
    /// Hybrid (ESCIR with FPGA acceleration option)
    Hybrid,
}
```

---

## FPGA Resource Requirements

```rust
/// FPGA-specific resource requirements (extends ResourceRequirements)
pub struct FpgaResources {
    /// Logic cells / LUTs
    pub luts: u32,
    
    /// Block RAM (18Kb blocks)
    pub bram: u16,
    
    /// DSP slices
    pub dsp: u16,
    
    /// Flip-flops (if different from LUTs)
    pub ffs: Option<u32>,
    
    /// Target clock frequency
    pub target_fmax_mhz: u16,
    
    /// Minimum FPGA family
    pub target_fpga: FpgaTarget,
    
    /// IO requirements
    pub io_pins: Option<u16>,
    
    /// Estimated power (mW)
    pub power_mw: Option<u16>,
}

pub enum FpgaTarget {
    /// Lattice Nexus family
    Nexus { min_luts_k: u8 },  // e.g., Nexus { min_luts_k: 20 } = Nexus 20K+
    
    /// Xilinx families
    Artix,
    Kintex,
    Virtex,
    
    /// Intel/Altera families
    Cyclone,
    Arria,
    Stratix,
    
    /// Any FPGA meeting resource requirements
    Generic,
}
```

---

## FPGA Performance Metrics

```rust
/// FPGA-specific performance metrics (extends ComponentStats)
pub struct FpgaPerformance {
    /// Sustained throughput (MB/s)
    pub throughput_mbps: u32,
    
    /// Peak throughput (MB/s) 
    pub peak_throughput_mbps: Option<u32>,
    
    /// Pipeline latency (clock cycles)
    pub latency_cycles: u16,
    
    /// Bytes processed per clock cycle
    pub bytes_per_cycle: f32,
    
    /// Whether timing is constant (security property)
    pub constant_time: bool,
    
    /// Benchmark conditions
    pub benchmark: BenchmarkConditions,
}

pub struct BenchmarkConditions {
    pub clock_mhz: u16,
    pub input_pattern: String,  // e.g., "random", "structured", "worst-case"
    pub test_size_bytes: u32,
}
```

---

## FPGA Component Variants

FPGA components often have multiple variants with different performance/area tradeoffs.

```rust
/// FPGA component with multiple SKU variants
pub struct FpgaComponent {
    /// Base component info
    pub base: MarketplaceComponent,
    
    /// Available variants
    pub variants: Vec<FpgaVariant>,
}

pub struct FpgaVariant {
    /// Variant identifier (e.g., "lite", "standard", "premium")
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Variant-specific description
    pub description: String,
    
    /// Variant-specific pricing
    pub pricing: Pricing,
    
    /// Variant-specific visibility
    pub visibility: SourceVisibility,
    
    /// Resource requirements for this variant
    pub resources: FpgaResources,
    
    /// Performance metrics for this variant
    pub performance: FpgaPerformance,
    
    /// Configuration parameters
    pub configuration: HashMap<String, ConfigValue>,
    
    /// Files included in this variant
    pub files: Vec<String>,
    
    /// Badges earned by this variant
    pub badges: Vec<Badge>,
}

pub enum ConfigValue {
    Int(i64),
    Bool(bool),
    String(String),
}
```

---

## Variant Selection UI

When a user adds an FPGA component, they select a variant:

```
┌─────────────────────────────────────────────────────────────────────┐
│  ISO 20022 Parser                              [Official] [Certified] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Select Variant:                                                     │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  ● Lite                                              [Free]    │ │
│  │    1 Gbps • 14K LUTs • Nexus 20K+                             │ │
│  │    Best for: Development, pilots, multi-function FPGAs        │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  ○ Standard                                    [0.0001 ES/exec]│ │
│  │    4 Gbps • 18K LUTs • Nexus 40K                              │ │
│  │    Best for: Production, regional networks                     │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  ○ Premium                                      [1000 ES/month]│ │
│  │    8 Gbps • 22K LUTs • Dedicated Nexus 40K                    │ │
│  │    Best for: Central banks, RTGS, clearing houses             │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  Your FPGA: Nexus 40K (39,000 LUTs available)                       │
│  ✓ All variants compatible                                          │
│                                                                      │
│  [Cancel]                                    [Add to Design]         │
└─────────────────────────────────────────────────────────────────────┘
```

---

## FPGA Compatibility Checking

The marketplace should validate FPGA compatibility:

```rust
impl FpgaComponent {
    /// Check if variant fits on target FPGA
    pub fn check_compatibility(
        &self, 
        variant: &FpgaVariant,
        target: &DeploymentTarget,
    ) -> CompatibilityResult {
        let available = target.available_resources();
        let required = &variant.resources;
        
        let mut issues = Vec::new();
        
        if required.luts > available.luts {
            issues.push(CompatibilityIssue::InsufficientLuts {
                required: required.luts,
                available: available.luts,
            });
        }
        
        if required.bram > available.bram {
            issues.push(CompatibilityIssue::InsufficientBram {
                required: required.bram,
                available: available.bram,
            });
        }
        
        if required.target_fmax_mhz > target.max_fmax_mhz {
            issues.push(CompatibilityIssue::TimingRisk {
                target_mhz: required.target_fmax_mhz,
                achievable_mhz: target.max_fmax_mhz,
            });
        }
        
        if issues.is_empty() {
            CompatibilityResult::Compatible {
                lut_utilization: required.luts as f32 / available.luts as f32,
                bram_utilization: required.bram as f32 / available.bram as f32,
            }
        } else {
            CompatibilityResult::Incompatible { issues }
        }
    }
    
    /// Recommend best variant for target
    pub fn recommend_variant(&self, target: &DeploymentTarget) -> Option<&FpgaVariant> {
        self.variants
            .iter()
            .filter(|v| self.check_compatibility(v, target).is_compatible())
            .max_by_key(|v| v.performance.throughput_mbps)
    }
}
```

---

## FPGA-Specific Badges

Additional badges for FPGA components:

```rust
pub enum FpgaBadge {
    /// Meets timing on target FPGA
    TimingVerified { 
        fpga: FpgaTarget, 
        achieved_mhz: u16 
    },
    
    /// Formal verification passed
    FormallyVerified,
    
    /// Constant-time implementation (side-channel resistant)
    ConstantTime,
    
    /// Multiple FPGA families supported
    CrossPlatform { 
        platforms: Vec<FpgaTarget> 
    },
    
    /// Has simulation testbench
    Simulated {
        tool: String,  // "verilator", "iverilog", "questa"
        coverage_pct: u8,
    },
    
    /// Power optimized
    LowPower {
        mw_at_target_freq: u16,
    },
}
```

---

## Pricing Considerations

FPGA components may have different pricing models:

```rust
pub enum FpgaPricing {
    /// Per-bitstream generation (one-time per deployment)
    PerBitstream { price_es: u64 },
    
    /// Per-message throughput
    PerMessage { 
        per_million_messages_es: u64 
    },
    
    /// Per-device license
    PerDevice { 
        price_es: u64,
        device_fingerprint_required: bool,
    },
    
    /// Site license (unlimited devices at location)
    SiteLicense {
        monthly_es: u64,
    },
}
```

---

## Integration with Hardware Tiers

FPGA components should specify compatibility with hardware tiers:

```rust
pub struct HardwareTierCompatibility {
    /// Minimum tier required
    pub min_tier: HardwareTier,
    
    /// Tier-specific configurations
    pub tier_configs: HashMap<HardwareTier, TierConfig>,
}

pub struct TierConfig {
    /// Variant recommended for this tier
    pub recommended_variant: String,
    
    /// Expected performance on this tier
    pub expected_performance: FpgaPerformance,
    
    /// Resource utilization on this tier
    pub utilization_pct: u8,
}
```

---

## Publishing FPGA Components

Extended publish flow for FPGA components:

```rust
pub struct FpgaPublishRequest {
    /// Base publish request
    pub base: PublishRequest,
    
    /// Component type marker
    pub component_type: ComponentType::Fpga,
    
    /// Variants to publish
    pub variants: Vec<FpgaVariantPublish>,
    
    /// Synthesis reports (optional, for verification)
    pub synthesis_reports: Option<Vec<SynthesisReport>>,
    
    /// Simulation results
    pub simulation_results: Option<SimulationResults>,
}

pub struct FpgaVariantPublish {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pricing: Pricing,
    pub visibility: SourceVisibility,
    pub resources: FpgaResources,
    pub performance: FpgaPerformance,
    pub configuration: HashMap<String, ConfigValue>,
    pub verilog_files: Vec<VerilogFile>,
}

pub struct SynthesisReport {
    pub tool: String,           // "radiant", "vivado", "quartus"
    pub target_fpga: String,
    pub achieved_fmax_mhz: u16,
    pub resource_utilization: FpgaResources,
    pub report_hash: [u8; 32],  // For verification
}
```

---

## Example Component Manifest

See `fpga/rtl/iso20022/component.yaml` for a complete example.

---

## References

- [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md)
- [HARDWARE_TIER_SPEC.md](../hardware/HARDWARE_TIER_SPEC.md)
- [fpga/rtl/iso20022/component.yaml](../../fpga/rtl/iso20022/component.yaml)
