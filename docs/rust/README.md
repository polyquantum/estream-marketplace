# eStream Marketplace — Rust Guide

## Installation

```bash
cargo add estream-marketplace
```

Or add to `Cargo.toml`:

```toml
[dependencies]
estream-marketplace = "0.9"
```

## Consuming Components

### Browsing the Marketplace

```rust
use estream_marketplace::{MarketplaceClient, Category};

#[tokio::main]
async fn main() -> estream_marketplace::Result<()> {
    let client = MarketplaceClient::new()?;

    let components = client.search("wire-adapter")
        .category(Category::WireAdapter)
        .limit(20)
        .send()
        .await?;

    for comp in &components {
        println!("{} v{} — {}", comp.name(), comp.version(), comp.description());
    }
    Ok(())
}
```

### Installing a Component

```rust
let comp = client.install("estream-wire-fix", "^1.0.0").await?;
println!("Installed {} at {}", comp.name(), comp.install_path().display());
```

### Using a Component

Once installed, the component's crate is available as a dependency:

```rust
use estream_wire_fix::{FixDecoder, FixMessage};

let decoder = FixDecoder::new();
let msg: FixMessage = decoder.decode(raw_bytes)?;
println!("Order type: {:?}", msg.ord_type());
```

### Checking Component Lifecycle

```rust
use estream_marketplace::Lifecycle;

let comp = client.get("estream-wire-fix", "^1.0.0").await?;

match comp.lifecycle().status() {
    Lifecycle::Active => println!("Component is active"),
    Lifecycle::Deprecated { successor, sunset } => {
        println!("Deprecated — migrate to {} before {}", successor, sunset);
    }
    Lifecycle::Sunset => println!("Component has been sunset and is no longer supported"),
    _ => {}
}

// Run automated migration check across your project
let report = client.lifecycle_audit().await?;
for warning in report.deprecated() {
    eprintln!("WARN: {} is deprecated, successor: {}", warning.name(), warning.successor());
}
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang rust
cd my-circuit
```

This generates the project structure with `Cargo.toml`, `estream-component.toml`, source files, and tests.

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
type = "rust"
entry = "src/lib.rs"
min_estream = "0.9.1"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
cargo build --release
cargo test
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Bump version
cargo set-version 1.1.0

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
