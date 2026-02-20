# Industrial Protocol Gateway v2 - Layered Architecture

> FPGA-accelerated bridge between industrial protocols and estream with modular, composable design.

**Status:** Draft  
**Version:** 2.0.0  
**Category:** IoT / Industrial  
**Issue:** #424  
**Target:** Component Marketplace v1.0.0

---

## Executive Summary

This specification defines a **layered, composable architecture** for industrial protocol integration:

1. **Transport Layer** - Generic TCP/UDP/Serial clients
2. **Protocol Layer** - MODBUS, OPC-UA, DNP3 as ESF schemas
3. **StreamSight Layer** - Unified telemetry and observability
4. **ESCIR Circuits** - Open-source behavior definitions
5. **Composite Components** - Marketplace SKUs

**Key Design Decisions:**
- **ESCIR is Open Source** - Behavior definitions are transparent and auditable
- **RTL is Optional** - Commercial FPGA acceleration available as upgrade
- **Software Reference** - Rust implementation for development/testing
- **Composable** - Mix and match protocols via ESCIR composition

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Transport Layer](#2-transport-layer)
3. [Protocol Layer - ESF Schemas](#3-protocol-layer---esf-schemas)
4. [StreamSight Integration](#4-streamsight-integration)
5. [ESCIR Circuit Definitions](#5-escir-circuit-definitions)
6. [Composite Components](#6-composite-components)
7. [Implementation Targets](#7-implementation-targets)
8. [Configuration](#8-configuration)
9. [Security Considerations](#9-security-considerations)
10. [Testing Strategy](#10-testing-strategy)
11. [Marketplace SKUs](#11-marketplace-skus)

---

## 1. Architecture Overview

### 1.1 Layered Stack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    INDUSTRIAL PROTOCOL GATEWAY v2                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      ESCIR Circuit Layer (Open Source)                  │ │
│  │                                                                         │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │                    Composite Components                           │  │ │
│  │  │                                                                   │  │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │ │
│  │  │  │ gateway-    │  │ gateway-    │  │ gateway-    │              │  │ │
│  │  │  │ lite.escir  │  │ standard    │  │ premium     │              │  │ │
│  │  │  │             │  │ .escir      │  │ .escir      │              │  │ │
│  │  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │  │ │
│  │  └─────────┼────────────────┼────────────────┼──────────────────────┘  │ │
│  │            │                │                │                         │ │
│  │  ┌─────────┴────────────────┴────────────────┴──────────────────────┐  │ │
│  │  │                    Protocol Circuits                              │  │ │
│  │  │                                                                   │  │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │ │
│  │  │  │ modbus_tcp  │  │ modbus_rtu  │  │ opcua_      │              │  │ │
│  │  │  │ .escir      │  │ .escir      │  │ client.escir│              │  │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘              │  │ │
│  │  │                                                                   │  │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │ │
│  │  │  │ poll_       │  │ alarm_      │  │ stream_     │              │  │ │
│  │  │  │ scheduler   │  │ evaluator   │  │ emitter     │              │  │ │
│  │  │  │ .escir      │  │ .escir      │  │ .escir      │              │  │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘              │  │ │
│  │  └───────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                         │ │
│  │  ┌───────────────────────────────────────────────────────────────────┐  │ │
│  │  │                    Transport Circuits                              │  │ │
│  │  │                                                                    │  │ │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐               │  │ │
│  │  │  │ tcp_client  │  │ serial_uart │  │ udp_client  │               │  │ │
│  │  │  │ .escir      │  │ .escir      │  │ .escir      │               │  │ │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘               │  │ │
│  │  └───────────────────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      ESF Schema Layer (Open Source)                     │ │
│  │                                                                         │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │ │
│  │  │ modbus.esf  │  │ opcua.esf   │  │ dnp3.esf    │  │ industrial- │   │ │
│  │  │             │  │             │  │             │  │ telemetry   │   │ │
│  │  │ • MBAP      │  │ • UA Binary │  │ • Link      │  │ .esf        │   │ │
│  │  │ • PDU       │  │ • Services  │  │ • Transport │  │             │   │ │
│  │  │ • Functions │  │ • Types     │  │ • App       │  │ • Events    │   │ │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      StreamSight Integration                            │ │
│  │                                                                         │ │
│  │  Topics:                                                                │ │
│  │  • lex://estream/sys/industrial/{gateway_id}/connection                │ │
│  │  • lex://estream/sys/industrial/{gateway_id}/protocol/{protocol}       │ │
│  │  • lex://estream/sys/industrial/{gateway_id}/device/{device_id}        │ │
│  │  • lex://estream/sys/industrial/{gateway_id}/alarm                     │ │
│  │  • lex://estream/sys/industrial/{gateway_id}/health                    │ │
│  │                                                                         │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      Implementation Layer                               │ │
│  │                                                                         │ │
│  │  ┌─────────────────────────┐    ┌─────────────────────────┐           │ │
│  │  │   Software (Rust)       │    │   Hardware (Verilog)     │           │ │
│  │  │   Open Source           │    │   Commercial License     │           │ │
│  │  │                         │    │                          │           │ │
│  │  │   • Development         │    │   • Production           │           │ │
│  │  │   • Testing             │    │   • Low latency          │           │ │
│  │  │   • Simulation          │    │   • High throughput      │           │ │
│  │  └─────────────────────────┘    └─────────────────────────┘           │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Design Principles

| Principle | Implementation |
|-----------|----------------|
| **Transparency** | ESCIR circuits are open source, auditable |
| **Composability** | Mix protocol circuits via ESCIR composition |
| **Portability** | Same ESCIR runs on software or FPGA |
| **Observability** | Full StreamSight integration at every layer |
| **Upgradability** | Swap software for FPGA without changing config |

---

## 2. Transport Layer

### 2.1 TCP Client Circuit

```yaml
# circuits/transport/tcp_client.escir.yaml
escir: "0.8.0"
name: tcp_client
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: transport_tcp_client
  name: "Generic TCP Client"
  description: "Reusable TCP client with connection management and telemetry"
  category: transport
  
  marketplace:
    visibility: source  # Open source
    publisher: estream-official
    tags: ["tcp", "transport", "network"]

types:
  TcpConfig:
    kind: struct
    fields:
      - name: remote_ip
        type: "bytes(4)"     # IPv4
      - name: remote_port
        type: u16
      - name: connect_timeout_ms
        type: u32
      - name: read_timeout_ms
        type: u32
      - name: keepalive_interval_ms
        type: u32
      - name: max_retries
        type: u8

  TcpState:
    kind: enum
    variants:
      - Disconnected
      - Connecting
      - Connected
      - Error

  TcpRequest:
    kind: struct
    fields:
      - name: connection_id
        type: u32
      - name: payload
        type: "bytes(1460)"   # MTU - headers
      - name: payload_len
        type: u16

  TcpResponse:
    kind: struct
    fields:
      - name: connection_id
        type: u32
      - name: payload
        type: "bytes(1460)"
      - name: payload_len
        type: u16
      - name: latency_us
        type: u32

  TcpError:
    kind: struct
    fields:
      - name: connection_id
        type: u32
      - name: error_code
        type: u8
      - name: error_message
        type: "string(64)"

inputs:
  - name: config
    type: TcpConfig
    description: "Connection configuration"
  - name: send_request
    type: TcpRequest
    description: "Data to send"
  - name: connect_trigger
    type: bool
    description: "Trigger connection"
  - name: disconnect_trigger
    type: bool
    description: "Trigger disconnection"

outputs:
  - name: state
    type: TcpState
    description: "Current connection state"
  - name: response
    type: TcpResponse
    description: "Received data"
  - name: error
    type: TcpError
    description: "Error events"

annotations:
  witness_tier: "infrastructure"
  hardware_accelerated: true
  streamsight_emit: true

streamsight:
  namespace: "io.estream.transport.tcp"
  emit:
    - event: "connection_state_change"
      topic: "lex://estream/sys/industrial/{gateway_id}/connection"
      fields: ["connection_id", "old_state", "new_state", "remote_ip"]
    - event: "request_sent"
      topic: "lex://estream/sys/industrial/{gateway_id}/connection"
      fields: ["connection_id", "payload_len", "timestamp"]
    - event: "response_received"
      topic: "lex://estream/sys/industrial/{gateway_id}/connection"
      fields: ["connection_id", "payload_len", "latency_us"]
    - event: "error"
      topic: "lex://estream/sys/industrial/{gateway_id}/connection"
      fields: ["connection_id", "error_code", "error_message"]

compute:
  nodes:
    - id: connection_manager
      type: state_machine
      states: [Disconnected, Connecting, Connected, Error]
      
    - id: send_buffer
      type: fifo
      depth: 4
      
    - id: receive_buffer
      type: fifo
      depth: 4
      
    - id: timeout_monitor
      type: timer
      
    - id: retry_controller
      type: counter

  flows:
    - connect_trigger -> connection_manager
    - send_request -> send_buffer -> connection_manager
    - connection_manager -> receive_buffer -> response
    - timeout_monitor -> connection_manager -> error
```

### 2.2 Serial UART Circuit

```yaml
# circuits/transport/serial_uart.escir.yaml
escir: "0.8.0"
name: serial_uart
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: transport_serial_uart
  name: "Serial UART"
  description: "RS-232/RS-485 serial interface with configurable parameters"
  category: transport

types:
  SerialConfig:
    kind: struct
    fields:
      - name: baud_rate
        type: u32
      - name: data_bits
        type: u8         # 7 or 8
      - name: parity
        type: u8         # 0=none, 1=odd, 2=even
      - name: stop_bits
        type: u8         # 1 or 2
      - name: flow_control
        type: u8         # 0=none, 1=hw, 2=sw
      - name: rs485_mode
        type: bool
      - name: inter_frame_gap_us
        type: u32

  SerialFrame:
    kind: struct
    fields:
      - name: data
        type: "bytes(256)"
      - name: length
        type: u8
      - name: timestamp
        type: u64

inputs:
  - name: config
    type: SerialConfig
  - name: tx_frame
    type: SerialFrame

outputs:
  - name: rx_frame
    type: SerialFrame
  - name: tx_complete
    type: bool
  - name: rx_error
    type: u8

annotations:
  witness_tier: "infrastructure"
  hardware_accelerated: true
  streamsight_emit: true

streamsight:
  namespace: "io.estream.transport.serial"
  emit:
    - event: "frame_sent"
      fields: ["length", "timestamp"]
    - event: "frame_received"
      fields: ["length", "timestamp"]
    - event: "error"
      fields: ["error_code"]
```

---

## 3. Protocol Layer - ESF Schemas

### 3.1 MODBUS Protocol Schema

```yaml
# schemas/modbus.esf.yaml
esf: "1.0"
name: modbus
version: "1.0.0"
license: "Apache-2.0"
description: "MODBUS Application Protocol message formats"

# ============================================================================
# MODBUS TCP (MBAP Header + PDU)
# ============================================================================

messages:
  MbapHeader:
    description: "MODBUS Application Protocol header (TCP)"
    fields:
      - name: transaction_id
        type: u16
        description: "Transaction identifier (echoed in response)"
      - name: protocol_id
        type: u16
        value: 0
        description: "Protocol identifier (always 0 for MODBUS)"
      - name: length
        type: u16
        description: "Number of following bytes (Unit ID + PDU)"
      - name: unit_id
        type: u8
        description: "Unit identifier (slave address)"

  # Read Holding Registers (FC 03)
  ReadHoldingRegistersRequest:
    description: "Read Holding Registers request (Function Code 0x03)"
    fields:
      - name: function_code
        type: u8
        value: 0x03
      - name: starting_address
        type: u16
        description: "Starting register address (0-based)"
      - name: quantity
        type: u16
        description: "Number of registers to read (1-125)"

  ReadHoldingRegistersResponse:
    description: "Read Holding Registers response"
    fields:
      - name: function_code
        type: u8
        value: 0x03
      - name: byte_count
        type: u8
        description: "Number of data bytes (2 * quantity)"
      - name: register_values
        type: "[u16; 125]"
        description: "Register values"

  # Read Input Registers (FC 04)
  ReadInputRegistersRequest:
    description: "Read Input Registers request (Function Code 0x04)"
    fields:
      - name: function_code
        type: u8
        value: 0x04
      - name: starting_address
        type: u16
      - name: quantity
        type: u16

  # Write Single Register (FC 06)
  WriteSingleRegisterRequest:
    description: "Write Single Register request (Function Code 0x06)"
    fields:
      - name: function_code
        type: u8
        value: 0x06
      - name: register_address
        type: u16
      - name: register_value
        type: u16

  # Write Multiple Registers (FC 16)
  WriteMultipleRegistersRequest:
    description: "Write Multiple Registers request (Function Code 0x10)"
    fields:
      - name: function_code
        type: u8
        value: 0x10
      - name: starting_address
        type: u16
      - name: quantity
        type: u16
      - name: byte_count
        type: u8
      - name: register_values
        type: "[u16; 123]"

  # Exception Response
  ExceptionResponse:
    description: "MODBUS exception response"
    fields:
      - name: function_code
        type: u8
        description: "Original function code + 0x80"
      - name: exception_code
        type: u8
        description: "Exception code (1-6)"

# ============================================================================
# MODBUS RTU (Address + PDU + CRC)
# ============================================================================

  RtuFrame:
    description: "MODBUS RTU frame format"
    fields:
      - name: slave_address
        type: u8
        description: "Slave address (1-247)"
      - name: pdu
        type: "bytes(253)"
        description: "Protocol Data Unit"
      - name: pdu_length
        type: u8
      - name: crc16
        type: u16
        description: "CRC-16 (polynomial 0xA001)"

# ============================================================================
# Common Types
# ============================================================================

  RegisterMapping:
    description: "Register to stream mapping configuration"
    fields:
      - name: device_id
        type: "string(32)"
      - name: register_address
        type: u16
      - name: register_type
        type: u8   # 0=holding, 1=input, 2=coil, 3=discrete
      - name: data_type
        type: u8   # 0=u16, 1=i16, 2=u32, 3=i32, 4=f32
      - name: scale_factor
        type: f32
      - name: offset
        type: f32
      - name: unit
        type: "string(16)"
      - name: poll_interval_ms
        type: u32
      - name: stream_path
        type: "string(128)"

# ============================================================================
# Exception Codes
# ============================================================================

enums:
  ExceptionCode:
    values:
      - name: IllegalFunction
        value: 1
      - name: IllegalDataAddress
        value: 2
      - name: IllegalDataValue
        value: 3
      - name: SlaveDeviceFailure
        value: 4
      - name: Acknowledge
        value: 5
      - name: SlaveDeviceBusy
        value: 6

  FunctionCode:
    values:
      - name: ReadCoils
        value: 0x01
      - name: ReadDiscreteInputs
        value: 0x02
      - name: ReadHoldingRegisters
        value: 0x03
      - name: ReadInputRegisters
        value: 0x04
      - name: WriteSingleCoil
        value: 0x05
      - name: WriteSingleRegister
        value: 0x06
      - name: WriteMultipleCoils
        value: 0x0F
      - name: WriteMultipleRegisters
        value: 0x10
```

### 3.2 Industrial Telemetry Schema

```yaml
# schemas/industrial-telemetry.esf.yaml
esf: "1.0"
name: industrial_telemetry
version: "1.0.0"
license: "Apache-2.0"
description: "Telemetry events for industrial protocol gateway"

messages:
  ConnectionEvent:
    description: "Transport connection state change"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: connection_id
        type: u32
      - name: protocol
        type: u8      # 0=modbus_tcp, 1=modbus_rtu, 2=opcua
      - name: remote_address
        type: "string(64)"
      - name: old_state
        type: u8
      - name: new_state
        type: u8
      - name: timestamp
        type: u64
      - name: error_code
        type: u8
        optional: true

  ProtocolRequest:
    description: "Protocol request telemetry"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: device_id
        type: "string(32)"
      - name: protocol
        type: u8
      - name: function_code
        type: u8
      - name: address
        type: u16
      - name: quantity
        type: u16
      - name: timestamp
        type: u64
      - name: transaction_id
        type: u16

  ProtocolResponse:
    description: "Protocol response telemetry"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: device_id
        type: "string(32)"
      - name: transaction_id
        type: u16
      - name: success
        type: bool
      - name: latency_us
        type: u32
      - name: byte_count
        type: u16
      - name: error_code
        type: u8
        optional: true
      - name: timestamp
        type: u64

  RegisterValue:
    description: "Register value telemetry"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: device_id
        type: "string(32)"
      - name: register_name
        type: "string(64)"
      - name: raw_value
        type: u32
      - name: scaled_value
        type: f64
      - name: unit
        type: "string(16)"
      - name: quality
        type: u8      # 0=good, 1=uncertain, 2=bad
      - name: timestamp
        type: u64

  AlarmEvent:
    description: "Alarm/event telemetry"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: alarm_id
        type: "string(64)"
      - name: severity
        type: u8      # 0=info, 1=warning, 2=critical
      - name: condition
        type: "string(128)"
      - name: current_value
        type: f64
      - name: threshold_value
        type: f64
      - name: acknowledged
        type: bool
      - name: timestamp
        type: u64

  GatewayHealth:
    description: "Gateway health metrics"
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: uptime_seconds
        type: u64
      - name: devices_connected
        type: u16
      - name: devices_total
        type: u16
      - name: requests_total
        type: u64
      - name: requests_failed
        type: u64
      - name: avg_latency_us
        type: u32
      - name: memory_used_bytes
        type: u32
      - name: cpu_percent
        type: u8
      - name: timestamp
        type: u64
```

---

## 4. StreamSight Integration

### 4.1 Topic Hierarchy

```
lex://estream/sys/industrial/
├── {gateway_id}/
│   ├── connection/              # Transport layer events
│   │   ├── state_change
│   │   ├── error
│   │   └── metrics
│   ├── protocol/
│   │   ├── modbus/
│   │   │   ├── request
│   │   │   ├── response
│   │   │   └── exception
│   │   ├── opcua/
│   │   │   ├── browse
│   │   │   ├── read
│   │   │   ├── write
│   │   │   └── subscription
│   │   └── dnp3/
│   │       ├── poll
│   │       ├── event
│   │       └── time_sync
│   ├── device/
│   │   └── {device_id}/
│   │       ├── telemetry        # Register values
│   │       ├── status           # Device status
│   │       └── alarm            # Device alarms
│   ├── alarm/                   # Aggregated alarms
│   │   ├── active
│   │   ├── history
│   │   └── acknowledge
│   └── health/                  # Gateway health
│       ├── metrics
│       └── diagnostics
```

### 4.2 StreamSight Bridge Circuit

```yaml
# circuits/industrial/industrial_streamsight_bridge.escir.yaml
escir: "0.8.0"
name: industrial_streamsight_bridge
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: industrial_streamsight_bridge
  name: "Industrial StreamSight Bridge"
  description: "Unified telemetry bridge for all industrial protocols"
  category: observability

types:
  TelemetryEvent:
    kind: union
    variants:
      - ConnectionEvent
      - ProtocolRequest
      - ProtocolResponse
      - RegisterValue
      - AlarmEvent
      - GatewayHealth

  StreamSightConfig:
    kind: struct
    fields:
      - name: gateway_id
        type: "bytes(32)"
      - name: emit_interval_ms
        type: u32
      - name: buffer_size
        type: u16
      - name: compression_enabled
        type: bool

inputs:
  - name: config
    type: StreamSightConfig
  - name: event
    type: TelemetryEvent
  - name: flush_trigger
    type: bool

outputs:
  - name: lex_stream_out
    type: LexStreamPacket
  - name: buffer_full
    type: bool
  - name: events_emitted
    type: u64

annotations:
  witness_tier: "platform"
  hardware_accelerated: true

compute:
  nodes:
    - id: event_buffer
      type: fifo
      depth: 256
      
    - id: event_serializer
      type: transform
      operation: esf_serialize
      
    - id: topic_router
      type: lookup
      operation: route_to_topic
      
    - id: batch_aggregator
      type: accumulator
      batch_size: 32
      timeout_ms: 100
      
    - id: lex_emitter
      type: output
      protocol: lex_stream

  flows:
    - event -> event_buffer
    - event_buffer -> event_serializer
    - event_serializer -> topic_router
    - topic_router -> batch_aggregator
    - batch_aggregator -> lex_emitter -> lex_stream_out
```

---

## 5. ESCIR Circuit Definitions

### 5.1 MODBUS TCP Client Circuit

```yaml
# circuits/industrial/modbus_tcp_client.escir.yaml
escir: "0.8.0"
name: modbus_tcp_client
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: industrial_modbus_tcp_client
  name: "MODBUS TCP Client"
  description: "MODBUS TCP master implementation using generic TCP transport"
  category: industrial
  
  marketplace:
    visibility: source
    publisher: estream-official
    tags: ["modbus", "scada", "industrial", "plc"]
    
  dependencies:
    - circuit: tcp_client
      version: "^1.0.0"

types:
  ModbusDevice:
    kind: struct
    fields:
      - name: device_id
        type: "string(32)"
      - name: ip_address
        type: "bytes(4)"
      - name: port
        type: u16
      - name: unit_id
        type: u8
      - name: timeout_ms
        type: u32

  ModbusRequest:
    kind: struct
    fields:
      - name: device_idx
        type: u8
      - name: function_code
        type: u8
      - name: start_address
        type: u16
      - name: quantity
        type: u16
      - name: write_data
        type: "[u16; 123]"
        optional: true

  ModbusResponse:
    kind: struct
    fields:
      - name: device_idx
        type: u8
      - name: transaction_id
        type: u16
      - name: success
        type: bool
      - name: data
        type: "[u16; 125]"
      - name: data_count
        type: u8
      - name: exception_code
        type: u8
        optional: true
      - name: latency_us
        type: u32

inputs:
  - name: devices
    type: "[ModbusDevice; 32]"
  - name: device_count
    type: u8
  - name: request
    type: ModbusRequest
  - name: request_valid
    type: bool

outputs:
  - name: response
    type: ModbusResponse
  - name: response_valid
    type: bool
  - name: device_status
    type: "[u8; 32]"   # 0=disconnected, 1=connected, 2=error

annotations:
  witness_tier: "platform"
  precision_class: "standard"
  hardware_accelerated: true
  streamsight_emit: true

streamsight:
  namespace: "io.estream.industrial.modbus"
  emit:
    - event: "request"
      topic: "lex://estream/sys/industrial/{gateway_id}/protocol/modbus/request"
      fields: ["device_idx", "function_code", "start_address", "quantity"]
    - event: "response"
      topic: "lex://estream/sys/industrial/{gateway_id}/protocol/modbus/response"
      fields: ["device_idx", "transaction_id", "success", "latency_us", "exception_code"]
    - event: "device_status"
      topic: "lex://estream/sys/industrial/{gateway_id}/device/{device_id}/status"
      fields: ["device_id", "status", "error_code"]

compute:
  nodes:
    - id: mbap_builder
      type: transform
      operation: build_mbap_header
      description: "Build MODBUS Application Protocol header"
      
    - id: pdu_builder
      type: transform
      operation: build_pdu
      description: "Build Protocol Data Unit"
      
    - id: transaction_tracker
      type: state
      description: "Track in-flight transactions"
      
    - id: response_parser
      type: transform
      operation: parse_modbus_response
      
    - id: exception_handler
      type: condition
      operation: check_exception

  flows:
    - request -> mbap_builder
    - mbap_builder -> pdu_builder
    - pdu_builder -> tcp_client.send_request
    - tcp_client.response -> response_parser
    - response_parser -> exception_handler
    - exception_handler -> response

# Sub-circuit instantiation
subcircuits:
  - name: tcp_client
    circuit: tcp_client
    version: "^1.0.0"
```

### 5.2 Poll Scheduler Circuit

```yaml
# circuits/industrial/poll_scheduler.escir.yaml
escir: "0.8.0"
name: poll_scheduler
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: industrial_poll_scheduler
  name: "Industrial Poll Scheduler"
  description: "Configurable polling scheduler for register reads"
  category: industrial

types:
  PollConfig:
    kind: struct
    fields:
      - name: register_idx
        type: u16
      - name: device_idx
        type: u8
      - name: function_code
        type: u8
      - name: start_address
        type: u16
      - name: quantity
        type: u16
      - name: interval_ms
        type: u32
      - name: enabled
        type: bool
      - name: priority
        type: u8

  PollTrigger:
    kind: struct
    fields:
      - name: register_idx
        type: u16
      - name: device_idx
        type: u8
      - name: function_code
        type: u8
      - name: start_address
        type: u16
      - name: quantity
        type: u16
      - name: scheduled_time
        type: u64

inputs:
  - name: poll_configs
    type: "[PollConfig; 256]"
  - name: poll_count
    type: u16
  - name: current_time_ms
    type: u64
  - name: poll_complete
    type: u16    # Index of completed poll

outputs:
  - name: poll_trigger
    type: PollTrigger
  - name: poll_valid
    type: bool

annotations:
  witness_tier: "infrastructure"
  hardware_accelerated: true

compute:
  nodes:
    - id: countdown_timers
      type: timer_array
      count: 256
      
    - id: priority_arbiter
      type: arbiter
      algorithm: priority_round_robin
      
    - id: next_poll_selector
      type: selector

  flows:
    - poll_configs -> countdown_timers
    - current_time_ms -> countdown_timers
    - countdown_timers -> priority_arbiter
    - priority_arbiter -> next_poll_selector
    - next_poll_selector -> poll_trigger
```

### 5.3 Stream Emitter Circuit

```yaml
# circuits/industrial/stream_emitter.escir.yaml
escir: "0.8.0"
name: stream_emitter
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: industrial_stream_emitter
  name: "Industrial Stream Emitter"
  description: "Convert register values to estream events"
  category: industrial

types:
  RegisterValueEvent:
    kind: struct
    fields:
      - name: device_id
        type: "string(32)"
      - name: register_name
        type: "string(64)"
      - name: raw_value
        type: u32
      - name: scaled_value
        type: f64
      - name: unit
        type: "string(16)"
      - name: stream_path
        type: "string(128)"
      - name: timestamp
        type: u64

  AlarmTrigger:
    kind: struct
    fields:
      - name: alarm_id
        type: "string(64)"
      - name: severity
        type: u8
      - name: condition_met
        type: bool
      - name: current_value
        type: f64
      - name: threshold
        type: f64

inputs:
  - name: register_mapping
    type: "[RegisterMapping; 256]"
  - name: raw_register_data
    type: "[u16; 256]"
  - name: data_valid
    type: "[bool; 256]"
  - name: timestamp
    type: u64

outputs:
  - name: stream_event
    type: RegisterValueEvent
  - name: stream_valid
    type: bool
  - name: alarm_trigger
    type: AlarmTrigger
  - name: alarm_valid
    type: bool

annotations:
  witness_tier: "platform"
  hardware_accelerated: true
  streamsight_emit: true

compute:
  nodes:
    - id: value_converter
      type: transform
      operation: scale_and_convert
      
    - id: change_detector
      type: comparator
      operation: detect_significant_change
      
    - id: alarm_evaluator
      type: condition
      operation: evaluate_alarm_conditions
      
    - id: event_formatter
      type: transform
      operation: format_stream_event

  flows:
    - raw_register_data -> value_converter
    - value_converter -> change_detector
    - change_detector -> alarm_evaluator
    - change_detector -> event_formatter
    - alarm_evaluator -> alarm_trigger
    - event_formatter -> stream_event
```

---

## 6. Composite Components

### 6.1 Gateway Lite (Open Source)

```yaml
# circuits/marketplace/industrial-gateway-lite.escir.yaml
escir: "0.8.0"
name: industrial_gateway_lite
version: "1.0.0"
license: "Apache-2.0"

metadata:
  circuit_id: marketplace_industrial_gateway_lite
  name: "Industrial Gateway Lite"
  description: "MODBUS TCP gateway with StreamSight integration"
  category: industrial
  
  marketplace:
    sku: "gateway-lite"
    visibility: source
    pricing:
      type: free
    publisher: estream-official
    badges:
      - official
      - open-source

  resources:
    witness_tier: 2
    compute_budget: 10000
    memory_bytes: 131072
    estimated_luts: 8000
    estimated_bram: 4

# Compose from sub-circuits
composition:
  - name: tcp_client
    circuit: tcp_client
    version: "^1.0.0"
    
  - name: modbus_client
    circuit: modbus_tcp_client
    version: "^1.0.0"
    bind:
      - tcp_client: tcp_client
      
  - name: poll_scheduler
    circuit: poll_scheduler
    version: "^1.0.0"
    
  - name: stream_emitter
    circuit: stream_emitter
    version: "^1.0.0"
    
  - name: streamsight_bridge
    circuit: industrial_streamsight_bridge
    version: "^1.0.0"

# Wiring between composed circuits
wiring:
  # Poll scheduler triggers MODBUS requests
  - from: poll_scheduler.poll_trigger
    to: modbus_client.request
    
  # MODBUS responses to stream emitter
  - from: modbus_client.response
    to: stream_emitter.raw_register_data
    
  # All telemetry to StreamSight bridge
  - from: modbus_client.telemetry
    to: streamsight_bridge.event
    
  - from: stream_emitter.stream_event
    to: streamsight_bridge.event
    
  - from: tcp_client.connection_event
    to: streamsight_bridge.event

# External interface (exposed to users)
inputs:
  - name: config
    type: GatewayLiteConfig
    map_to: 
      - tcp_client.config
      - modbus_client.devices
      - poll_scheduler.poll_configs
      - stream_emitter.register_mapping

outputs:
  - name: lex_stream
    type: LexStreamPacket
    map_from: streamsight_bridge.lex_stream_out
    
  - name: device_status
    type: "[u8; 32]"
    map_from: modbus_client.device_status
```

### 6.2 Gateway Standard (Commercial)

```yaml
# circuits/marketplace/industrial-gateway-standard.escir.yaml
escir: "0.8.0"
name: industrial_gateway_standard
version: "1.0.0"
license: "Commercial"

metadata:
  circuit_id: marketplace_industrial_gateway_standard
  name: "Industrial Gateway Standard"
  description: "MODBUS TCP/RTU + OPC-UA gateway"
  category: industrial
  
  marketplace:
    sku: "gateway-standard"
    visibility: compiled    # Compiled only, not source
    pricing:
      type: subscription
      monthly_es: 100
      annual_es: 1000
    publisher: estream-official
    badges:
      - official
      - certified

  resources:
    witness_tier: 2
    compute_budget: 25000
    memory_bytes: 262144
    estimated_luts: 14000
    estimated_bram: 8

composition:
  # All lite components
  - include: industrial_gateway_lite
  
  # Additional protocols
  - name: serial_uart
    circuit: serial_uart
    version: "^1.0.0"
    
  - name: modbus_rtu_client
    circuit: modbus_rtu_client
    version: "^1.0.0"
    bind:
      - serial_uart: serial_uart
      
  - name: opcua_client
    circuit: opcua_client
    version: "^1.0.0"
    
  # Additional features
  - name: alarm_manager
    circuit: alarm_manager
    version: "^1.0.0"
    
  - name: history_buffer
    circuit: history_buffer
    version: "^1.0.0"
    config:
      buffer_size: 10000
```

### 6.3 Gateway Premium (Commercial)

```yaml
# circuits/marketplace/industrial-gateway-premium.escir.yaml
escir: "0.8.0"
name: industrial_gateway_premium
version: "1.0.0"
license: "Commercial"

metadata:
  circuit_id: marketplace_industrial_gateway_premium
  name: "Industrial Gateway Premium"
  description: "Full industrial protocol suite with compliance features"
  category: industrial
  
  marketplace:
    sku: "gateway-premium"
    visibility: compiled
    pricing:
      type: subscription
      monthly_es: 300
      annual_es: 3000
    publisher: estream-official
    badges:
      - official
      - certified
      - nerc-cip

  resources:
    witness_tier: 3
    compute_budget: 50000
    memory_bytes: 524288
    estimated_luts: 20000
    estimated_bram: 12

composition:
  # All standard components
  - include: industrial_gateway_standard
  
  # Additional protocols
  - name: dnp3_client
    circuit: dnp3_client
    version: "^1.0.0"
    
  - name: opcua_hda_client
    circuit: opcua_hda_client
    version: "^1.0.0"
    
  # Redundancy and compliance
  - name: redundancy_manager
    circuit: redundancy_manager
    version: "^1.0.0"
    
  - name: audit_logger
    circuit: audit_logger
    version: "^1.0.0"
    config:
      retention_days: 90
      tamper_evident: true
```

---

## 7. Implementation Targets

### 7.1 Software Reference (Rust - Open Source)

```rust
// crates/estream-industrial/src/lib.rs
//
// Software reference implementation of industrial gateway circuits.
// Open source under Apache-2.0.

pub mod transport;
pub mod modbus;
pub mod poll_scheduler;
pub mod stream_emitter;
pub mod streamsight;

// Re-exports
pub use transport::TcpClient;
pub use modbus::ModbusTcpClient;
pub use poll_scheduler::PollScheduler;
pub use stream_emitter::StreamEmitter;
```

### 7.2 Hardware Acceleration (Verilog - Commercial)

```
fpga/rtl/industrial/           # Commercial RTL (optional upgrade)
├── tcp_client.v               # Generic TCP client
├── modbus_tcp_codec.v         # MODBUS TCP protocol
├── modbus_rtu_codec.v         # MODBUS RTU protocol
├── poll_scheduler.v           # Hardware poll scheduler
├── stream_emitter.v           # Register to stream conversion
└── industrial_streamsight_bridge.v
```

### 7.3 Feature Matrix

| Feature | Software | Hardware |
|---------|----------|----------|
| MODBUS TCP | ✅ | ✅ (Commercial) |
| MODBUS RTU | ✅ | ✅ (Commercial) |
| OPC-UA | ✅ | ✅ (Commercial) |
| Latency | ~1ms | <100μs |
| Throughput | 1K reg/s | 50K reg/s |
| Power | ~10W (CPU) | ~2W (FPGA) |
| Deployment | Container | Bitstream |

---

## 8. Configuration

### 8.1 Gateway Configuration Schema

```yaml
# Example configuration for gateway-lite
gateway:
  id: "wellpad-alpha-gw"
  version: "1.0.0"
  
  streamsight:
    enabled: true
    emit_interval_ms: 100
    topics:
      base: "lex://io.thermogen/industrial"
      
  devices:
    - id: "plc-001"
      protocol: modbus_tcp
      address: "192.168.1.100"
      port: 502
      unit_id: 1
      timeout_ms: 1000
      
  registers:
    - device: "plc-001"
      name: "temperature"
      address: 40001
      type: float32
      scale: 0.1
      unit: "°C"
      poll_ms: 1000
      stream: "io.thermogen/telemetry/temperature"
      
    - device: "plc-001"
      name: "pressure"
      address: 40003
      type: uint16
      scale: 0.01
      unit: "bar"
      poll_ms: 500
      stream: "io.thermogen/telemetry/pressure"
      
  alarms:
    - id: "high-temp"
      condition: "temperature > 85"
      severity: warning
      stream: "io.thermogen/alarms"
```

---

## 9. Security Considerations

| Concern | Mitigation |
|---------|------------|
| **Network isolation** | Gateway should be on isolated SCADA network |
| **Authentication** | Support OPC-UA security policies |
| **Audit logging** | All operations logged via StreamSight |
| **Input validation** | Strict register address bounds checking |
| **DoS protection** | Rate limiting on poll scheduler |
| **PoVC attestation** | Hardware witness for data integrity |

---

## 10. Testing Strategy

### 10.1 ESCIR Tests

```yaml
# tests/industrial/modbus_tcp_test.escir.yaml
test:
  name: "MODBUS TCP Read Holding Registers"
  circuit: modbus_tcp_client
  
  setup:
    devices:
      - device_id: "test-plc"
        ip: "127.0.0.1"
        port: 5020
        unit_id: 1
        
  cases:
    - name: "Read single register"
      input:
        request:
          device_idx: 0
          function_code: 0x03
          start_address: 0
          quantity: 1
      expect:
        response:
          success: true
          data_count: 1
          
    - name: "Read multiple registers"
      input:
        request:
          device_idx: 0
          function_code: 0x03
          start_address: 0
          quantity: 10
      expect:
        response:
          success: true
          data_count: 10
          
    - name: "Handle timeout"
      setup:
        mock:
          delay_ms: 5000  # Exceed timeout
      expect:
        response:
          success: false
          exception_code: timeout
```

### 10.2 Software Simulator

```rust
// tests/modbus_slave_sim.rs
//
// MODBUS TCP slave simulator for testing

pub struct ModbusSlaveSimulator {
    holding_registers: [u16; 65536],
    input_registers: [u16; 65536],
    // ...
}
```

---

## 11. Marketplace SKUs

| SKU | License | ESCIR | Software | Hardware | Monthly | Support |
|-----|---------|-------|----------|----------|---------|---------|
| **gateway-lite** | Apache-2.0 | ✅ Source | ✅ Source | ❌ | Free | Community |
| **gateway-standard** | Commercial | ❌ Compiled | ✅ Binary | ✅ Bitstream | 100 ES | Standard |
| **gateway-premium** | Commercial | ❌ Compiled | ✅ Binary | ✅ Bitstream | 300 ES | Priority |

---

## Appendix A: Related Specifications

- [Component Marketplace Spec](./MARKETPLACE_SPEC.md)
- [ESCIR Language Spec](../protocol/ESCIR_LANGUAGE_SPEC.md)
- [ESF Schema Spec](../protocol/ESF_SCHEMA_SPEC.md)
- [StreamSight Spec](../protocol/STREAMSIGHT_SPEC.md)
- [ISO 20022 Parser Component](../../fpga/rtl/iso20022/COMPONENT_MARKETPLACE.md)

---

## Appendix B: References

- [MODBUS Application Protocol Specification V1.1b3](https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf)
- [MODBUS over TCP/IP Implementation Guide](https://modbus.org/docs/Modbus_Messaging_Implementation_Guide_V1_0b.pdf)
- [OPC UA Specification Part 1-14](https://opcfoundation.org/developer-tools/specifications-unified-architecture)
- [IEEE 1815 (DNP3)](https://www.dnp.org/)
