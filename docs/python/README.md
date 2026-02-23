# eStream Marketplace — Python Guide

## Installation

```bash
pip install estream-marketplace
```

Or with optional async support:

```bash
pip install estream-marketplace[async]
```

## Consuming Components

### Browsing the Marketplace

```python
from estream_marketplace import MarketplaceClient, Category

async def browse():
    client = MarketplaceClient()

    components = await client.search(
        query="wire-adapter",
        category=Category.WIRE_ADAPTER,
        limit=20,
    )

    for comp in components:
        print(f"{comp.name} v{comp.version} — {comp.description}")
```

### Installing a Component

```python
comp = await client.install("estream-wire-fix", "^1.0.0")
print(f"Installed {comp.name} at {comp.install_path}")
```

### Using a Component

Once installed, the component is importable as a Python package:

```python
from estream_wire_fix import FixDecoder, FixMessage

decoder = FixDecoder()
msg: FixMessage = decoder.decode(raw_bytes)
print(f"Order type: {msg.ord_type}")
```

### Checking Component Lifecycle

```python
from estream_marketplace import Lifecycle

comp = await client.get("estream-wire-fix", "^1.0.0")

if comp.lifecycle.status == Lifecycle.ACTIVE:
    print("Component is active")
elif comp.lifecycle.status == Lifecycle.DEPRECATED:
    print(f"Deprecated — migrate to {comp.lifecycle.successor} before {comp.lifecycle.sunset}")
elif comp.lifecycle.status == Lifecycle.SUNSET:
    print("Component has been sunset and is no longer supported")

# Run automated migration check across your project
report = await client.lifecycle_audit()
for warning in report.deprecated:
    print(f"WARN: {warning.name} is deprecated, successor: {warning.successor}")
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang python
cd my-circuit
```

This generates the project structure with `pyproject.toml`, `estream-component.toml`, source files, and tests.

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
type = "python"
entry = "my_circuit/__init__.py"
min_estream = "0.9.1"
min_python = "3.10"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
python -m pytest
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Update version in pyproject.toml, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
