# ISO 20022 Parser - Component Marketplace SKUs

## Overview

The ISO 20022 FPGA Parser is available in multiple configurations to match different deployment requirements. Each SKU offers a different performance/area tradeoff.

## Available SKUs

### iso20022-parser-lite
**For: Budget deployments, multi-function FPGAs, development**

| Metric | Value |
|--------|-------|
| Throughput | 125 MB/s (1 Gbps) |
| Latency | 4 cycles |
| LUTs | ~14,000 |
| BRAM | 13 |
| Target FPGA | Nexus 20K+ |

**Use Cases:**
- CBDC pilot programs
- Development and testing environments
- Edge devices with multiple functions
- Cost-sensitive deployments

**Configuration:**
```verilog
xml_tokenizer_configurable #(.PARALLEL_WIDTH(1)) u_parser (...);
tree_walker_fsm u_walker (...);            // Sequential
field_extractor u_extractor (...);         // Sequential
```

---

### iso20022-parser-standard
**For: Production deployments, balanced performance/area**

| Metric | Value |
|--------|-------|
| Throughput | 500 MB/s (4 Gbps) |
| Latency | 5 cycles |
| LUTs | ~18,000 |
| BRAM | 15 |
| Target FPGA | Nexus 40K |

**Use Cases:**
- Regional payment networks
- Commercial bank infrastructure
- Medium-volume RTGS participants

**Configuration:**
```verilog
xml_tokenizer_configurable #(.PARALLEL_WIDTH(4)) u_parser (...);
tree_walker_parallel u_walker (...);       // 4-byte parallel hash
field_extractor u_extractor (...);         // Sequential (often not bottleneck)
```

---

### iso20022-parser-premium
**For: High-volume infrastructure, maximum throughput**

| Metric | Value |
|--------|-------|
| Throughput | 1,000 MB/s (8 Gbps) |
| Latency | 6 cycles |
| LUTs | ~22,000 |
| BRAM | 17 |
| Target FPGA | Nexus 40K (dedicated) or larger |

**Use Cases:**
- Central bank RTGS systems
- Tier-1 clearing houses
- High-frequency settlement systems
- Dedicated parser appliances

**Configuration:**
```verilog
xml_tokenizer_configurable #(.PARALLEL_WIDTH(8)) u_parser (...);
tree_walker_parallel u_walker (...);       // 4-byte parallel hash
field_extractor_parallel u_extractor (...);// 8-byte parallel extraction
```

---

## FPGA Resource Planning

### Nexus 40K Resource Budget

```
Total LUTs available: ~39,000

┌────────────────────────────────────────────────────────────────┐
│                     FPGA Resource Allocation                    │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌────────────────┐  │
│  │   ISO 20022     │  │   ISO 20022     │  │   ISO 20022    │  │
│  │   Parser-Lite   │  │   Parser-Std    │  │   Parser-Prem  │  │
│  │                 │  │                 │  │                │  │
│  │   14,000 LUTs   │  │   18,000 LUTs   │  │   22,000 LUTs  │  │
│  │   (36%)         │  │   (46%)         │  │   (56%)        │  │
│  └─────────────────┘  └─────────────────┘  └────────────────┘  │
│                                                                │
│  Available for other functions:                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌────────────────┐  │
│  │   25,000 LUTs   │  │   21,000 LUTs   │  │   17,000 LUTs  │  │
│  │   (64%)         │  │   (54%)         │  │   (44%)        │  │
│  │                 │  │                 │  │                │  │
│  │ Scatter: ✓      │  │ Scatter: ✓      │  │ Scatter: ✓     │  │
│  │ Governance: ✓   │  │ Governance: ✓   │  │ Governance: ⚠  │  │
│  │ StreamSight: ✓  │  │ StreamSight: ✓  │  │ StreamSight: ✓ │  │
│  │ Custom: 13K     │  │ Custom: 9K      │  │ Custom: 5K     │  │
│  └─────────────────┘  └─────────────────┘  └────────────────┘  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

### Typical Component Sizes (for reference)

| Component | LUTs | BRAM |
|-----------|------|------|
| Scatter Router | ~5,000 | 4 |
| Governance Chain | ~4,000 | 2 |
| StreamSight Bridge | ~2,000 | 1 |
| PoVC Attestation | ~3,000 | 2 |
| AXI Interconnect | ~1,500 | 0 |
| HTU (Timestamp) | ~500 | 0 |

---

## Decision Matrix

| Requirement | Lite | Standard | Premium |
|-------------|------|----------|---------|
| Budget-constrained | ✓✓✓ | ✓✓ | ✓ |
| Multi-function FPGA | ✓✓✓ | ✓✓ | ✓ |
| < 1M msg/day | ✓✓✓ | ✓✓ | ✓ |
| 1-10M msg/day | ✓ | ✓✓✓ | ✓✓ |
| > 10M msg/day | - | ✓✓ | ✓✓✓ |
| Latency-critical | ✓✓✓ | ✓✓ | ✓ |
| Throughput-critical | ✓ | ✓✓ | ✓✓✓ |
| RTGS/Central Bank | - | ✓✓ | ✓✓✓ |

---

## Upgrade Path

Deployments can start with **Lite** and upgrade to **Standard** or **Premium** as volume grows:

```
Development → Lite (pilot) → Standard (production) → Premium (scale)
```

The interface is identical across all SKUs, so upgrading requires only:
1. Change `PARALLEL_WIDTH` parameter
2. Re-synthesize bitstream
3. Deploy new bitstream (no software changes)

---

## Pricing Tiers (Suggested)

| SKU | License | Support |
|-----|---------|---------|
| Lite | Open Source (MIT) | Community |
| Standard | Commercial | Standard SLA |
| Premium | Commercial | Priority SLA |

---

## Files Included

```
fpga/rtl/iso20022/
├── xml_tokenizer.v              # Lite tokenizer
├── xml_tokenizer_parallel.v     # Premium tokenizer
├── xml_tokenizer_configurable.v # Configurable wrapper
├── tree_walker_fsm.v            # Lite walker
├── tree_walker_parallel.v       # Standard/Premium walker
├── field_extractor.v            # Lite/Standard extractor
├── field_extractor_parallel.v   # Premium extractor
├── iso20022_parser_top.v        # Top-level integration
└── ... (other modules)
```
