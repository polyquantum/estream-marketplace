# ISO 20022 Parser Circuit

> Marketplace reference implementation: FPGA + ESCIR + Rust

## Overview

The ISO 20022 Parser is the first complete marketplace component, demonstrating how a single parsing specification can target multiple execution environments:

| Target | Description | Throughput | Use Case |
|--------|-------------|------------|----------|
| **FPGA** | Hardware-accelerated RTL | 125 MB/s - 1 GB/s | Production payments |
| **ESCIR→Rust** | Compiled from circuit spec | 50-200 MB/s | Fallback/verification |
| **Native Rust** | Hand-optimized crate | 150-280 MB/s | Development/testing |

## Files

```
circuits/iso20022/
├── README.md                    # This file
├── circuit.v080.escir.yaml      # ESCIR circuit definition
└── test_vectors/                # Shared test data
    ├── pacs008_minimal.xml
    ├── pacs008_full.xml
    ├── pacs002_status.xml
    └── pacs008_json.json

fpga/rtl/iso20022/
├── component.yaml               # Marketplace manifest
├── COMPONENT_MARKETPLACE.md     # SKU documentation
├── PERFORMANCE_OPTIMIZATION.md  # Optimization notes
├── xml_tokenizer.v              # Sequential tokenizer
├── xml_tokenizer_parallel.v     # 8-byte parallel tokenizer
├── xml_tokenizer_configurable.v # Configurable wrapper
├── json_tokenizer.v             # JSON tokenizer
├── tokenizer_mux.v              # Format multiplexer
├── tree_walker_fsm.v            # Path tracking
├── tree_walker_parallel.v       # Parallel hash computation
├── field_extractor.v            # Field extraction
├── field_extractor_parallel.v   # Parallel extraction
├── schema_rom.v                 # Field lookup table
├── data_output.v                 # Data formatting
├── iso20022_parser_top.v        # Top-level module
├── iso20022_streamsight_bridge.v # Telemetry interface
└── xml_template_engine.v        # XML generation

crates/estream-iso20022/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── messages/
│   │   ├── pacs008.rs
│   │   └── pacs002.rs
│   ├── schema.rs
│   ├── data.rs
│   └── types.rs
└── benches/
    └── parser_benchmark.rs
```

## ESCIR Circuit

The ESCIR circuit (`circuit.v080.escir.yaml`) defines the parser as a dataflow graph:

```
┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│ Byte Stream  │──▶│  Tokenizer   │──▶│ Tree Walker  │──▶│   Field      │
│  Ingress     │   │  (XML/JSON)  │   │    FSM       │   │  Extractor   │
└──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘
                         │                                      │
                         │                                      ▼
                   ┌─────┴─────┐                          ┌──────────────┐
                   │ Tokenizer │                          │  Data Output  │
                   │   Mux     │                          └──────────────┘
                   └───────────┘                                │
                                                                ▼
                                                          ┌──────────────┐
                                                          │PoVC Witness  │
                                                          └──────────────┘
```

### Compile to Rust

```bash
estream codegen --circuit circuit.v080.escir.yaml --target rust --output parser.rs
```

### Compile to Verilog

```bash
estream codegen --circuit circuit.v080.escir.yaml --target verilog --output parser.v
```

## Supported Messages

| Message | Description | Fields |
|---------|-------------|--------|
| `pacs.008` | FIToFICustomerCreditTransfer | 25 |
| `pacs.002` | FIToFIPaymentStatusReport | 15 |
| `camt.053` | BankToCustomerStatement | 30 |
| `camt.052` | BankToCustomerAccountReport | 20 |

## Usage

### FPGA (via estream-fpga-bridge)

```rust
use estream_fpga_bridge::Iso20022Parser;

let parser = Iso20022Parser::new(FpgaDevice::open()?)?;
let data = parser.parse_xml(&xml_bytes)?;
```

### Native Rust

```rust
use estream_iso20022::{Pacs008, parse_xml};

let msg: Pacs008 = parse_xml(&xml_bytes)?;
let data = msg.to_data()?;
```

### ESCIR→Rust (Generated)

```rust
// Generated from circuit.v080.escir.yaml
use iso20022_parser::Iso20022ParserCircuit;

let mut circuit = Iso20022ParserCircuit::new();
let data = circuit.execute(&input)?;
```

## Testing

### Cross-Target Verification

All three targets must produce identical Data output for the same input:

```bash
# Run cross-target tests
cargo test -p estream-iso20022 --features cross-target-verify
```

### Benchmark

```bash
# Rust benchmarks
cargo bench -p estream-iso20022

# FPGA benchmarks (requires hardware)
cd fpga/sim/iso20022
make test_performance
```

## Marketplace Component

This component is published to the eStream Component Marketplace with:

- **3 FPGA variants**: Lite (14K LUTs), Standard (18K LUTs), Premium (22K LUTs)
- **ESCIR source**: Open visibility for verification
- **Rust crate**: `estream-iso20022` on crates.io

See `component.yaml` for full manifest and `COMPONENT_MARKETPLACE.md` for SKU details.

## References

- [ISO20022_FPGA_PARSER_SPEC.md](../../specs/protocol/ISO20022_FPGA_PARSER_SPEC.md)
- [MARKETPLACE_SPEC.md](../../specs/marketplace/MARKETPLACE_SPEC.md)
- [FPGA_COMPONENT_EXTENSION.md](../../specs/marketplace/FPGA_COMPONENT_EXTENSION.md)
- [CODEGEN_PIPELINE_SPEC.md](../../specs/compiler/CODEGEN_PIPELINE_SPEC.md)
