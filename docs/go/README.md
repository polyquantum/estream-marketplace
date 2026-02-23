# eStream Marketplace — Go Guide

## Installation

```bash
go get github.com/polyquantum/estream-go/marketplace
```

## Consuming Components

### Browsing the Marketplace

```go
package main

import (
	"context"
	"fmt"

	"github.com/polyquantum/estream-go/marketplace"
)

func main() {
	ctx := context.Background()
	client := marketplace.NewClient()

	components, err := client.Search(ctx, &marketplace.SearchOpts{
		Query:    "wire-adapter",
		Category: marketplace.CategoryWireAdapter,
		Limit:    20,
	})
	if err != nil {
		panic(err)
	}

	for _, comp := range components {
		fmt.Printf("%s v%s — %s\n", comp.Name(), comp.Version(), comp.Description())
	}
}
```

### Installing a Component

```go
comp, err := client.Install(ctx, "estream-wire-fix", "^1.0.0")
if err != nil {
	return fmt.Errorf("install failed: %w", err)
}
fmt.Printf("Installed %s at %s\n", comp.Name(), comp.InstallPath())
```

### Using a Component

Once installed, the component is available as a Go module:

```go
import "github.com/polyquantum/estream-wire-fix"

decoder := fix.NewDecoder()
msg, err := decoder.Decode(rawBytes)
if err != nil {
	return err
}
fmt.Printf("Order type: %s\n", msg.OrdType())
```

### Checking Component Lifecycle

```go
comp, err := client.Get(ctx, "estream-wire-fix", "^1.0.0")
if err != nil {
	return err
}

switch comp.Lifecycle().Status() {
case marketplace.LifecycleActive:
	fmt.Println("Component is active")
case marketplace.LifecycleDeprecated:
	fmt.Printf("Deprecated — migrate to %s before %s\n",
		comp.Lifecycle().Successor(), comp.Lifecycle().Sunset())
case marketplace.LifecycleSunset:
	fmt.Println("Component has been sunset and is no longer supported")
}

// Run automated migration check across your project
report, err := client.LifecycleAudit(ctx)
if err != nil {
	return err
}
for _, warning := range report.Deprecated() {
	fmt.Printf("WARN: %s is deprecated, successor: %s\n", warning.Name(), warning.Successor())
}
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang go
cd my-circuit
```

This generates the project structure with `go.mod`, `estream-component.toml`, source files, and tests.

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
type = "go"
entry = "circuit.go"
min_estream = "0.9.1"
min_go = "1.21"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
go build ./...
go test ./...
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Tag the new version in go.mod, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
