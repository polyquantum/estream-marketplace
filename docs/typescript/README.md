# eStream Marketplace — TypeScript Guide

## Installation

```bash
npm install @estream/marketplace
```

Or with Yarn / pnpm:

```bash
yarn add @estream/marketplace
pnpm add @estream/marketplace
```

## Consuming Components

### Browsing the Marketplace

```typescript
import { MarketplaceClient, Category } from '@estream/marketplace';

const client = new MarketplaceClient();

const components = await client.search({
  query: 'wire-adapter',
  category: Category.WireAdapter,
  limit: 20,
});

for (const comp of components) {
  console.log(`${comp.name} v${comp.version} — ${comp.description}`);
}
```

### Installing a Component

```typescript
const comp = await client.install('estream-wire-fix', '^1.0.0');
console.log(`Installed ${comp.name} at ${comp.installPath}`);
```

### Using a Component

Once installed, the component is importable as an npm package:

```typescript
import { FixDecoder, FixMessage } from 'estream-wire-fix';

const decoder = new FixDecoder();
const msg: FixMessage = decoder.decode(rawBuffer);
console.log(`Order type: ${msg.ordType}`);
```

### Checking Component Lifecycle

```typescript
import { Lifecycle } from '@estream/marketplace';

const comp = await client.get('estream-wire-fix', '^1.0.0');

switch (comp.lifecycle.status) {
  case Lifecycle.Active:
    console.log('Component is active');
    break;
  case Lifecycle.Deprecated:
    console.log(`Deprecated — migrate to ${comp.lifecycle.successor} before ${comp.lifecycle.sunset}`);
    break;
  case Lifecycle.Sunset:
    console.log('Component has been sunset and is no longer supported');
    break;
}

// Run automated migration check across your project
const report = await client.lifecycleAudit();
for (const warning of report.deprecated) {
  console.warn(`WARN: ${warning.name} is deprecated, successor: ${warning.successor}`);
}
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang typescript
cd my-circuit
```

This generates the project structure with `package.json`, `tsconfig.json`, `estream-component.toml`, source files, and tests.

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
type = "typescript"
entry = "src/index.ts"
min_estream = "0.9.1"
min_node = "18"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
npm run build
npm test
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Bump version in package.json, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
