# eStream Marketplace SDK Documentation

Guides for publishing and consuming marketplace components across all supported languages.

## Quick Start

| Language | Install | Consume | Publish |
|----------|---------|---------|---------|
| **Rust** | `cargo add estream-marketplace` | [Guide](rust/README.md) | [Guide](rust/README.md#publishing) |
| **Python** | `pip install estream-marketplace` | [Guide](python/README.md) | [Guide](python/README.md#publishing) |
| **TypeScript** | `npm install @estream/marketplace` | [Guide](typescript/README.md) | [Guide](typescript/README.md#publishing) |
| **Go** | `go get github.com/polyquantum/estream-go/marketplace` | [Guide](go/README.md) | [Guide](go/README.md#publishing) |
| **C++** | CMake `find_package(estream)` | [Guide](cpp/README.md) | [Guide](cpp/README.md#publishing) |
| **Swift** | SPM `estream-swift` | [Guide](swift/README.md) | [Guide](swift/README.md#publishing) |
| **FastLang** | Built-in (`estream marketplace`) | [Guide](fastlang/README.md) | [Guide](fastlang/README.md#publishing) |

## Concepts

- **Component** — A marketplace package containing circuits, schemas, and metadata (`estream-component.toml`)
- **Publishing** — Uploading a component to the marketplace registry
- **Consuming** — Installing and using a marketplace component in your project
- **Lifecycle** — Components follow Draft -> Active -> Deprecated -> Sunset states

## Component Categories

| Category | Description |
|----------|-------------|
| `data-schema` | Shared data type definitions |
| `wire-adapter` | Protocol adapters (FIX, ISO 20022, Modbus, etc.) |
| `smart-circuit` | Reusable SmartCircuit logic |
| `fpga-circuit` | FPGA-targeted circuit with bitstream |
| `integration` | Multi-circuit integration bundles |
| `console-widget` | Dashboard UI components |
