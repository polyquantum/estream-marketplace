# Industrial Protocol Gateway - Marketplace Component

> FPGA-accelerated bridge between industrial protocols and estream.

**Status:** Draft  
**Version:** 0.1.0  
**Category:** IoT  
**Target:** Component Marketplace v0.9.0

---

## Overview

The Industrial Protocol Gateway is a marketplace component that provides bidirectional translation between industrial SCADA protocols (MODBUS, OPC-UA, DNP3) and estream streams. This enables plug-and-play integration with existing industrial infrastructure.

### Use Cases

- **Oil & Gas**: Wellpad monitoring, SCADA integration
- **Utilities**: Grid monitoring, substation automation
- **Manufacturing**: PLC integration, production monitoring
- **Energy**: Microgrid control, renewable energy systems
- **Water/Wastewater**: Treatment plant monitoring

---

## SKU Structure

Following the ISO 20022 parser pattern, the Industrial Protocol Gateway is available in multiple configurations:

### gateway-lite (Open Source)

**Target:** Budget deployments, development, single-protocol needs

| Metric | Value |
|--------|-------|
| Protocols | MODBUS TCP |
| Throughput | 1,000 registers/sec |
| Latency | < 1ms |
| LUTs | ~8,000 |
| BRAM | 4 |
| Target FPGA | iCE40 HX8K+, Nexus 20K+ |
| Price | Open Source (MIT) |

**Features:**
- MODBUS TCP master (client)
- Up to 32 slave devices
- Configurable register mapping
- Basic telemetry to estream streams

### gateway-standard (Commercial)

**Target:** Production deployments, multi-protocol environments

| Metric | Value |
|--------|-------|
| Protocols | MODBUS TCP, MODBUS RTU, OPC-UA |
| Throughput | 10,000 registers/sec |
| Latency | < 500μs |
| LUTs | ~14,000 |
| BRAM | 8 |
| Target FPGA | Nexus 40K |
| Price | Commercial (Standard SLA) |

**Features:**
- All lite features, plus:
- MODBUS RTU over RS-485
- OPC-UA client (read/write/subscribe)
- Up to 128 devices
- Alarm/event forwarding
- Historical data buffering

### gateway-premium (Commercial)

**Target:** Critical infrastructure, compliance-heavy environments

| Metric | Value |
|--------|-------|
| Protocols | MODBUS TCP/RTU, OPC-UA, DNP3 |
| Throughput | 50,000 registers/sec |
| Latency | < 100μs |
| LUTs | ~20,000 |
| BRAM | 12 |
| Target FPGA | Nexus 40K (dedicated) |
| Price | Commercial (Priority SLA) |

**Features:**
- All standard features, plus:
- DNP3 master (outstation polling)
- OPC-UA Historical Data Access (HDA)
- Redundant protocol paths
- NERC CIP compliance features
- Detailed audit logging

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     INDUSTRIAL PROTOCOL GATEWAY                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        Protocol Stack                                   │ │
│  │                                                                         │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │ │
│  │  │  MODBUS     │  │  MODBUS     │  │   OPC-UA    │  │    DNP3     │   │ │
│  │  │    TCP      │  │    RTU      │  │   Client    │  │   Master    │   │ │
│  │  │             │  │             │  │             │  │             │   │ │
│  │  │ • Master    │  │ • RS-485    │  │ • Browse    │  │ • Outstation│   │ │
│  │  │ • Slave     │  │ • Multi-    │  │ • Read      │  │   polling   │   │ │
│  │  │ • Gateway   │  │   drop      │  │ • Write     │  │ • Events    │   │ │
│  │  │             │  │             │  │ • Subscribe │  │ • Time sync │   │ │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │ │
│  │         │                │                │                │          │ │
│  │         └────────────────┴────────────────┴────────────────┘          │ │
│  │                                   │                                    │ │
│  │                                   ▼                                    │ │
│  │  ┌────────────────────────────────────────────────────────────────┐   │ │
│  │  │                    Protocol Abstraction Layer                   │   │ │
│  │  │                                                                 │   │ │
│  │  │  • Unified register model                                       │   │ │
│  │  │  • Data type conversion                                         │   │ │
│  │  │  • Polling scheduler                                            │   │ │
│  │  │  • Error handling / retry                                       │   │ │
│  │  │                                                                 │   │ │
│  │  └────────────────────────────────────────────────────────────────┘   │ │
│  │                                   │                                    │ │
│  └───────────────────────────────────┼────────────────────────────────────┘ │
│                                      │                                      │
│  ┌───────────────────────────────────┼────────────────────────────────────┐ │
│  │                                   ▼                                    │ │
│  │  ┌────────────────────────────────────────────────────────────────┐   │ │
│  │  │                    estream Integration                          │   │ │
│  │  │                                                                 │   │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │   │ │
│  │  │  │   Stream    │  │   Event     │  │   PoVC      │            │   │ │
│  │  │  │   Emitter   │  │   Router    │  │   Witness   │            │   │ │
│  │  │  │             │  │             │  │             │            │   │ │
│  │  │  │ • Telemetry │  │ • Alarms    │  │ • Data      │            │   │ │
│  │  │  │ • Metrics   │  │ • Events    │  │   attestation│           │   │ │
│  │  │  │ • Status    │  │ • Triggers  │  │ • Integrity │            │   │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘            │   │ │
│  │  │                                                                 │   │ │
│  │  └────────────────────────────────────────────────────────────────┘   │ │
│  │                                                                        │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Configuration

### Register Mapping (Lex-based)

```yaml
# config/lex/industrial/register-map.yaml
register_map:
  name: "wellpad-alpha"
  version: "1.0.0"
  
  devices:
    - id: "plc-001"
      protocol: modbus_tcp
      address: "192.168.1.100"
      port: 502
      unit_id: 1
      
      registers:
        - name: temperature
          address: 40001
          type: float32
          scale: 0.1
          unit: "°C"
          poll_interval_ms: 1000
          stream: "io.thermogen/telemetry/temperature"
          
        - name: pressure
          address: 40003
          type: uint16
          scale: 0.01
          unit: "bar"
          poll_interval_ms: 500
          stream: "io.thermogen/telemetry/pressure"
          
        - name: flow_rate
          address: 40005
          type: float32
          unit: "m³/h"
          poll_interval_ms: 2000
          stream: "io.thermogen/telemetry/flow"
          
    - id: "rtu-001"
      protocol: modbus_rtu
      serial_port: "/dev/ttyUSB0"
      baud_rate: 9600
      parity: "none"
      unit_id: 2
      
      registers:
        - name: valve_position
          address: 40001
          type: uint16
          scale: 0.1
          unit: "%"
          read_write: true
          stream: "io.thermogen/control/valve"
          
  alarms:
    - name: high_temperature
      condition: "temperature > 85.0"
      severity: warning
      stream: "io.thermogen/alarms/temperature"
      
    - name: overpressure
      condition: "pressure > 10.0"
      severity: critical
      stream: "io.thermogen/alarms/pressure"
      actions:
        - close_valve: "rtu-001.valve_position = 0"
```

### OPC-UA Configuration

```yaml
# config/lex/industrial/opcua-config.yaml
opcua:
  servers:
    - id: "scada-main"
      endpoint: "opc.tcp://192.168.1.200:4840"
      security_policy: "Basic256Sha256"
      security_mode: "SignAndEncrypt"
      
      authentication:
        type: certificate
        client_cert: "lex://secrets/opcua/client.pem"
        client_key: "lex://secrets/opcua/client.key"
        
      nodes:
        - browse_path: "Objects/Pump1/Speed"
          node_id: "ns=2;i=1001"
          type: float
          stream: "io.plant/pump1/speed"
          subscribe: true
          sampling_interval_ms: 100
          
        - browse_path: "Objects/Tank1/Level"
          node_id: "ns=2;i=1002"
          type: float
          stream: "io.plant/tank1/level"
          subscribe: true
          deadband: 0.5  # Only report changes > 0.5
```

---

## ESCIR Circuit

```yaml
# circuits/marketplace/industrial-gateway.escir.yaml
format: v0.8.0
version: 1

metadata:
  circuit_id: industrial_gateway
  name: "Industrial Protocol Gateway"
  description: "Bridge between industrial SCADA protocols and estream streams"
  category: iot
  
  marketplace:
    sku: gateway-standard
    visibility: compiled
    pricing:
      type: subscription
      monthly_es: 100
      annual_discount_pct: 20
    publisher: estream-official
    badges:
      - official
      - certified
      
  resources:
    witness_tier: 2
    compute_budget: 5000
    memory_bytes: 65536
    estimated_cost_es: 0.005

inputs:
  - name: register_config
    type: RegisterMapConfig
    description: "Register mapping configuration"
    
  - name: protocol_request
    type: ProtocolRequest
    description: "Incoming protocol request (poll result or write command)"
    
outputs:
  - name: stream_event
    type: StreamEvent
    description: "Telemetry/event to emit to estream stream"
    
  - name: protocol_response
    type: ProtocolResponse
    description: "Response to send back to protocol layer"
    
  - name: alarm_event
    type: AlarmEvent
    description: "Alarm/event for routing"

annotations:
  witness_tier: platform
  hardware_required: true
  precision_class: standard
  streamsight_emit: true
  
streamsight:
  namespace: "io.estream.gateway"
  event_types:
    - register_read
    - register_write
    - alarm_triggered
    - connection_status

compute:
  nodes:
    - id: parse_request
      type: transform
      operation: parse_protocol_request
      
    - id: map_register
      type: lookup
      operation: register_mapping_lookup
      
    - id: convert_value
      type: transform
      operation: value_type_conversion
      
    - id: check_alarms
      type: condition
      operation: alarm_condition_check
      
    - id: emit_stream
      type: output
      operation: stream_emission
      
  flows:
    - protocol_request -> parse_request
    - parse_request -> map_register
    - map_register -> convert_value
    - convert_value -> check_alarms
    - check_alarms -> emit_stream
    - check_alarms -> alarm_event  # If alarm triggered
```

---

## RTL Implementation

### Module Hierarchy

```
fpga/rtl/marketplace/industrial-gateway/
├── COMPONENT_MARKETPLACE.md          # This file
├── industrial_gateway_top.v          # Top-level integration
├── protocol/
│   ├── modbus_tcp_master.v           # MODBUS TCP (lite+)
│   ├── modbus_rtu_master.v           # MODBUS RTU (standard+)
│   ├── opcua_client.v                # OPC-UA (standard+)
│   └── dnp3_master.v                 # DNP3 (premium)
├── abstraction/
│   ├── register_model.v              # Unified register abstraction
│   ├── poll_scheduler.v              # Polling scheduler
│   └── value_converter.v             # Data type conversion
├── estream/
│   ├── stream_emitter.v              # Stream emission
│   ├── event_router.v                # Alarm/event routing
│   └── povc_witness.v                # PoVC witness generation
└── testbench/
    ├── gateway_tb.v                  # Full system testbench
    ├── modbus_slave_sim.v            # MODBUS slave simulator
    └── opcua_server_sim.v            # OPC-UA server simulator
```

### MODBUS TCP Master

```verilog
// fpga/rtl/marketplace/industrial-gateway/protocol/modbus_tcp_master.v

module modbus_tcp_master #(
    parameter MAX_DEVICES = 32,
    parameter MAX_REGISTERS_PER_REQUEST = 125
)(
    input  wire        clk,
    input  wire        rst_n,
    
    // Ethernet MAC interface
    input  wire [7:0]  eth_rx_data,
    input  wire        eth_rx_valid,
    output reg  [7:0]  eth_tx_data,
    output reg         eth_tx_valid,
    input  wire        eth_tx_ready,
    
    // Configuration interface
    input  wire [31:0] device_ip      [MAX_DEVICES-1:0],
    input  wire [15:0] device_port    [MAX_DEVICES-1:0],
    input  wire [7:0]  device_unit_id [MAX_DEVICES-1:0],
    
    // Request interface (from scheduler)
    input  wire [4:0]  req_device_idx,
    input  wire [7:0]  req_function_code,
    input  wire [15:0] req_start_addr,
    input  wire [15:0] req_quantity,
    input  wire        req_valid,
    output reg         req_ready,
    
    // Response interface
    output reg  [4:0]  resp_device_idx,
    output reg  [15:0] resp_data [MAX_REGISTERS_PER_REQUEST-1:0],
    output reg  [6:0]  resp_count,
    output reg         resp_valid,
    output reg  [7:0]  resp_error,  // 0 = success
    
    // Status
    output reg  [MAX_DEVICES-1:0] device_connected
);

    // MODBUS function codes
    localparam FC_READ_HOLDING_REGISTERS = 8'h03;
    localparam FC_READ_INPUT_REGISTERS   = 8'h04;
    localparam FC_WRITE_SINGLE_REGISTER  = 8'h06;
    localparam FC_WRITE_MULTIPLE_REGISTERS = 8'h10;
    
    // State machine
    localparam STATE_IDLE          = 4'd0;
    localparam STATE_CONNECT       = 4'd1;
    localparam STATE_SEND_REQUEST  = 4'd2;
    localparam STATE_WAIT_RESPONSE = 4'd3;
    localparam STATE_PARSE_RESPONSE = 4'd4;
    localparam STATE_ERROR         = 4'd5;
    
    reg [3:0] state;
    reg [15:0] transaction_id;
    reg [15:0] timeout_counter;
    
    // TCP connection state per device
    reg [MAX_DEVICES-1:0] tcp_connected;
    
    // MBAP header fields (MODBUS Application Protocol)
    reg [15:0] mbap_transaction_id;
    reg [15:0] mbap_protocol_id;  // Always 0 for MODBUS
    reg [15:0] mbap_length;
    reg [7:0]  mbap_unit_id;
    
    // Request building
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            state <= STATE_IDLE;
            req_ready <= 1'b1;
            transaction_id <= 16'd0;
        end else begin
            case (state)
                STATE_IDLE: begin
                    if (req_valid && req_ready) begin
                        req_ready <= 1'b0;
                        mbap_transaction_id <= transaction_id;
                        mbap_protocol_id <= 16'd0;
                        mbap_unit_id <= device_unit_id[req_device_idx];
                        
                        // Calculate length based on function code
                        case (req_function_code)
                            FC_READ_HOLDING_REGISTERS,
                            FC_READ_INPUT_REGISTERS: begin
                                mbap_length <= 16'd6;  // Unit ID + FC + Addr + Qty
                            end
                            // ... other function codes
                        endcase
                        
                        state <= tcp_connected[req_device_idx] ? 
                                 STATE_SEND_REQUEST : STATE_CONNECT;
                    end
                end
                
                STATE_SEND_REQUEST: begin
                    // Build and send MBAP header + PDU
                    // ... (TCP packet construction)
                    state <= STATE_WAIT_RESPONSE;
                    timeout_counter <= 16'd10000;  // 10ms at 1MHz
                end
                
                STATE_WAIT_RESPONSE: begin
                    if (timeout_counter == 0) begin
                        resp_error <= 8'hFF;  // Timeout
                        state <= STATE_ERROR;
                    end else begin
                        timeout_counter <= timeout_counter - 1;
                        if (eth_rx_valid) begin
                            state <= STATE_PARSE_RESPONSE;
                        end
                    end
                end
                
                STATE_PARSE_RESPONSE: begin
                    // Parse MBAP header and PDU
                    // Extract register values
                    resp_valid <= 1'b1;
                    resp_error <= 8'h00;
                    transaction_id <= transaction_id + 1;
                    state <= STATE_IDLE;
                    req_ready <= 1'b1;
                end
                
                STATE_ERROR: begin
                    resp_valid <= 1'b1;
                    state <= STATE_IDLE;
                    req_ready <= 1'b1;
                end
            endcase
        end
    end

endmodule
```

### Poll Scheduler

```verilog
// fpga/rtl/marketplace/industrial-gateway/abstraction/poll_scheduler.v

module poll_scheduler #(
    parameter MAX_REGISTERS = 256,
    parameter CLK_FREQ_HZ = 100_000_000
)(
    input  wire        clk,
    input  wire        rst_n,
    
    // Register configuration
    input  wire [4:0]  reg_device_idx  [MAX_REGISTERS-1:0],
    input  wire [15:0] reg_address     [MAX_REGISTERS-1:0],
    input  wire [31:0] reg_poll_interval_cycles [MAX_REGISTERS-1:0],
    input  wire [MAX_REGISTERS-1:0] reg_enabled,
    
    // Poll request output
    output reg  [7:0]  poll_reg_idx,
    output reg         poll_valid,
    input  wire        poll_ready
);

    // Per-register countdown timers
    reg [31:0] countdown [MAX_REGISTERS-1:0];
    
    // Round-robin arbitration
    reg [7:0] arb_idx;
    
    // Timer countdown
    integer i;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            for (i = 0; i < MAX_REGISTERS; i = i + 1) begin
                countdown[i] <= reg_poll_interval_cycles[i];
            end
        end else begin
            for (i = 0; i < MAX_REGISTERS; i = i + 1) begin
                if (reg_enabled[i]) begin
                    if (countdown[i] == 0) begin
                        // Will be reset when polled
                    end else begin
                        countdown[i] <= countdown[i] - 1;
                    end
                end
            end
            
            // Reset timer when poll acknowledged
            if (poll_valid && poll_ready) begin
                countdown[poll_reg_idx] <= reg_poll_interval_cycles[poll_reg_idx];
            end
        end
    end
    
    // Arbitration: find next register ready to poll
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            arb_idx <= 8'd0;
            poll_valid <= 1'b0;
        end else begin
            if (!poll_valid || poll_ready) begin
                // Find next register with countdown == 0
                poll_valid <= 1'b0;
                for (i = 0; i < MAX_REGISTERS; i = i + 1) begin
                    if (reg_enabled[(arb_idx + i) % MAX_REGISTERS] &&
                        countdown[(arb_idx + i) % MAX_REGISTERS] == 0 &&
                        !poll_valid) begin
                        poll_reg_idx <= (arb_idx + i) % MAX_REGISTERS;
                        poll_valid <= 1'b1;
                        arb_idx <= (arb_idx + i + 1) % MAX_REGISTERS;
                    end
                end
            end
        end
    end

endmodule
```

---

## Integration with estream

### Stream Emission

```rust
/// Industrial telemetry to estream stream conversion
pub struct IndustrialStreamEmitter {
    /// Stream client
    client: StreamClient,
    
    /// Register to stream mapping
    register_streams: HashMap<RegisterId, StreamPath>,
}

impl IndustrialStreamEmitter {
    /// Emit register value to stream
    pub async fn emit_register(
        &self,
        register_id: RegisterId,
        value: RegisterValue,
        timestamp: VdfTimestamp,
    ) -> Result<(), EmitError> {
        let stream_path = self.register_streams.get(&register_id)
            .ok_or(EmitError::NoStreamMapping)?;
        
        let event = TelemetryEvent {
            register_id,
            value: value.to_estream_value(),
            timestamp,
            source: self.gateway_id,
        };
        
        self.client.emit(stream_path, &event).await
    }
    
    /// Emit alarm to alarm stream
    pub async fn emit_alarm(
        &self,
        alarm: &AlarmEvent,
    ) -> Result<(), EmitError> {
        let alarm_stream = self.get_alarm_stream(&alarm.severity);
        
        self.client.emit(&alarm_stream, alarm).await
    }
}
```

### PoVC Witness

```rust
/// Generate PoVC witness for industrial data
pub struct IndustrialPovcWitness {
    /// Gateway identity
    pub gateway_id: GatewayId,
    
    /// Data hash (all register values)
    pub data_hash: [u8; 32],
    
    /// Protocol-level attestation
    pub protocol_attestation: ProtocolAttestation,
    
    /// Timestamp
    pub timestamp: VdfTimestamp,
    
    /// Hardware signature
    pub hardware_signature: HardwareSignature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtocolAttestation {
    /// Protocol type (MODBUS, OPC-UA, DNP3)
    pub protocol: IndustrialProtocol,
    
    /// Device addresses polled
    pub devices: Vec<DeviceAddress>,
    
    /// Transaction IDs (for audit trail)
    pub transaction_ids: Vec<u16>,
    
    /// CRC/checksum verification results
    pub integrity_checks: Vec<IntegrityCheck>,
}
```

---

## Pricing

| SKU | License | Monthly | Annual | Support |
|-----|---------|---------|--------|---------|
| **gateway-lite** | Open Source (MIT) | Free | Free | Community |
| **gateway-standard** | Commercial | 100 ES | 1,000 ES (17% off) | Standard SLA |
| **gateway-premium** | Commercial | 300 ES | 3,000 ES (17% off) | Priority SLA |

---

## References

- [MODBUS Application Protocol Specification](https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf)
- [OPC UA Specification](https://opcfoundation.org/developer-tools/specifications-unified-architecture)
- [DNP3 Specification](https://www.dnp.org/)
- [Component Marketplace Spec](./MARKETPLACE_SPEC.md)
- [ISO 20022 Parser Component](../../fpga/rtl/iso20022/COMPONENT_MARKETPLACE.md)
