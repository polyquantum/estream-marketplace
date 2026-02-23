# eStream Marketplace — FastLang Guide

FastLang is the native language of the eStream platform. Marketplace integration is built directly into the `estream` CLI — no additional SDK installation is needed.

## Installation

FastLang marketplace support is built into the eStream CLI:

```bash
estream --version   # requires >= 0.9.1
```

## Consuming Components

### Browsing the Marketplace

```bash
estream marketplace search wire-adapter --category wire-adapter --limit 20
```

Or from within a `.fl` file using compile-time resolution:

```fl
@marketplace_search("wire-adapter", category: "wire-adapter")
```

### Installing a Component

```bash
estream marketplace install estream-wire-fix ^1.0.0
```

This adds the dependency to your project's `estream.toml` and fetches the component into the local circuit cache.

### Using a Component

Import directly in your `.fl` files:

```fl
import estream-wire-fix

stream order_pipeline {
    input raw_bytes: bytes

    stage decode {
        let msg = estream-wire-fix::decode(raw_bytes)
        emit msg
    }

    stage validate {
        require msg.ord_type != nil
        emit msg
    }
}
```

### Checking Component Lifecycle

```bash
estream marketplace lifecycle check
```

Output:

```
estream-wire-fix v1.0.0  DEPRECATED
  successor: estream-wire-fix@^2.0.0
  sunset:    2027-06-01
  migration: run `estream marketplace migrate estream-wire-fix`

estream-data-iso20022 v3.1.0  ACTIVE
  no action required
```

Run an automated migration:

```bash
estream marketplace migrate estream-wire-fix
```

This updates imports, rewrites API calls to the successor's interface, and bumps `estream.toml`.

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold wire-adapter my-adapter
cd my-adapter
```

This generates:

```
my-adapter/
├── estream-component.toml
├── estream.toml
├── src/
│   └── adapter.fl
└── tests/
    └── adapter_test.fl
```

### Define the Manifest

`estream-component.toml`:

```toml
[component]
name = "my-adapter"
version = "0.1.0"
description = "Custom wire adapter for proprietary protocol"
category = "wire-adapter"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]

[component.implementation]
type = "fastlang"
entry = "src/adapter.fl"
min_estream = "0.9.1"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
estream build
estream test
estream marketplace validate .
```

The `validate` command checks manifest completeness, circuit compilation, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Update version in estream-component.toml, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-adapter@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
