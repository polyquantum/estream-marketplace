# eStream Marketplace — Swift Guide

## Installation

Add to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/polyquantum/estream-swift", from: "0.9.1"),
],
targets: [
    .target(
        name: "MyTarget",
        dependencies: [
            .product(name: "EStreamMarketplace", package: "estream-swift"),
        ]
    ),
]
```

## Consuming Components

### Browsing the Marketplace

```swift
import EStreamMarketplace

let client = MarketplaceClient()

let components = try await client.search(
    query: "wire-adapter",
    category: .wireAdapter,
    limit: 20
)

for comp in components {
    print("\(comp.name) v\(comp.version) — \(comp.description)")
}
```

### Installing a Component

```swift
let comp = try await client.install("estream-wire-fix", version: "^1.0.0")
print("Installed \(comp.name) at \(comp.installPath)")
```

### Using a Component

Once installed, the component is available as a Swift module:

```swift
import EStreamWireFix

let decoder = FixDecoder()
let msg = try decoder.decode(rawData)
print("Order type: \(msg.ordType)")
```

### Checking Component Lifecycle

```swift
let comp = try await client.get("estream-wire-fix", version: "^1.0.0")

switch comp.lifecycle.status {
case .active:
    print("Component is active")
case .deprecated(let successor, let sunset):
    print("Deprecated — migrate to \(successor) before \(sunset)")
case .sunset:
    print("Component has been sunset and is no longer supported")
}

// Run automated migration check across your project
let report = try await client.lifecycleAudit()
for warning in report.deprecated {
    print("WARN: \(warning.name) is deprecated, successor: \(warning.successor)")
}
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang swift
cd my-circuit
```

This generates the project structure with `Package.swift`, `estream-component.toml`, source files, and tests.

### Define the Manifest

`estream-component.toml`:

```toml
[component]
name = "my-circuit"
version = "0.1.0"
description = "Custom SmartCircuit for order validation"
category = "smart-circuit"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]

[component.implementation]
type = "swift"
entry = "Sources/MyCircuit/Circuit.swift"
min_estream = "0.9.1"
min_swift = "5.9"
platforms = ["macOS", "iOS", "linux"]

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
swift build
swift test
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Update version in estream-component.toml and Package.swift, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
