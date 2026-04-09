# CLI Reference

> Complete reference for all `estream marketplace` commands.

The marketplace CLI is accessible via `estream marketplace` (alias: `estream mp`). All commands operate on the local workspace and communicate with the component registry.

---

## `estream marketplace search`

Search for marketplace components by keyword and optional category filter.

### Synopsis

```
estream marketplace search <QUERY> [OPTIONS]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `QUERY` | Yes | Search query string (matches component names, categories, and descriptions) |

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--category` | `-c` | *(all)* | Filter results by component category |
| `--limit` | | `20` | Maximum number of results to return |

### Valid Categories

`data-schema`, `wire-adapter`, `smart-circuit`, `fpga-circuit`, `integration`, `console-widget`

### Examples

```bash
# Search by keyword
estream marketplace search "FIX adapter"

# Filter by category
estream marketplace search "trading" --category data-schema

# Limit results
estream marketplace search "iot" --limit 5

# Search using alias
estream mp search "carbon"
```

### Output Format

```
  NAME                           VERSION    CATEGORY           DESCRIPTION
  ------------------------------------------------------------------------------------------
  estream-wire-fix               1.0.0      wire-adapter       FIX protocol adapter for eStream
  data-trading                   1.2.0      data-schema        Financial trading data schemas

  2 result(s) found
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Search completed (results may be empty) |
| `1` | Invalid category filter or connection error |

---

## `estream marketplace install`

Install a marketplace component into the local workspace.

### Synopsis

```
estream marketplace install <COMPONENT[@VERSION]> [OPTIONS]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `COMPONENT` | Yes | Component name, optionally with `@version` suffix |

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--version` | *(latest)* | Specific version to install (alternative to `@version` syntax) |
| `--dry-run` | `false` | Preview installation without making changes |

### Version Resolution

- `estream-wire-fix` — installs the latest version
- `estream-wire-fix@1.0.0` — installs exactly version 1.0.0
- `--version 1.0.0` — equivalent to `@1.0.0`

### Examples

```bash
# Install latest version
estream marketplace install estream-wire-fix

# Install specific version
estream marketplace install estream-wire-fix@1.0.0

# Alternative version syntax
estream marketplace install estream-wire-fix --version 1.0.0

# Dry-run to preview
estream marketplace install data-trading --dry-run
```

### Output

```
  📦 Installing estream-wire-fix...

  Resolving dependencies...
    estream-wire-fix v1.0.0

  Verifying ML-DSA-87 signatures...
    estream-wire-fix v1.0.0 ✓

  ✓ Installed estream-wire-fix v1.0.0 (wire-adapter)
```

### Workspace Tracking

Installed components are recorded in `estream-workspace.toml`:

```toml
[components.estream-wire-fix]
version = "1.0.0"
category = "wire-adapter"
installed_at = "2026-02-20T12:00:00Z"
```

Component files are stored in `estream-workspace/components/<name>/`.

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Installation successful (or dry-run passed) |
| `1` | Component not found, version conflict, or verification failure |

---

## `estream marketplace publish`

Publish a component to the marketplace. Validates the manifest, resolves includes, checks FastLang files, builds a deterministic archive, and signs with ML-DSA-87.

### Synopsis

```
estream marketplace publish [PATH] [OPTIONS]
```

### Arguments

| Argument | Required | Default | Description |
|----------|----------|---------|-------------|
| `PATH` | No | `.` (current directory) | Path to the component directory |

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--dry-run` | `false` | Validate only — no signing, no archive emission |

### Validation Steps

1. Parse and validate `estream-component.toml` (required fields, valid category/pricing/visibility)
2. Resolve all include glob patterns against the component directory
3. Check FastLang `.fl` files with `compile --check`
4. Build deterministic `tar.gz` archive
5. Sign with ML-DSA-87 post-quantum signature
6. Emit `SIGNATURE.ml-dsa` alongside the archive

### Examples

```bash
# Publish from current directory
estream marketplace publish

# Publish from specific path
estream marketplace publish ./my-component

# Dry-run validation only
estream marketplace publish my-component --dry-run
```

### Generated Files

| File | Description |
|------|-------------|
| `<name>-<version>.tar.gz` | Deterministic component archive |
| `SIGNATURE.ml-dsa` | ML-DSA-87 post-quantum signature file |

### SIGNATURE.ml-dsa Format

```
algorithm: ML-DSA-87
archive: my-component-0.1.0.tar.gz
fingerprint: a1b2c3d4e5f6a7b8
timestamp: 2026-02-20T12:00:00Z
signature: <ML-DSA-87 signature bytes>
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Published successfully (or dry-run passed) |
| `1` | Manifest validation failed, include resolution error, or signing failure |

---

## `estream marketplace verify`

Verify an installed component's ML-DSA-87 post-quantum signature.

### Synopsis

```
estream marketplace verify <COMPONENT>
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `COMPONENT` | Yes | Name of the installed component to verify |

### Examples

```bash
estream marketplace verify estream-wire-fix
```

### Output

```
  🔒 Verifying component 'estream-wire-fix'...

  Result:    PASS
  Algorithm: ML-DSA-87
  Signer:    a1b2c3d4e5f6a7b8
  Timestamp: 2026-02-20T12:00:00Z
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Signature verification passed |
| `1` | Component not found, signature missing, or verification failed |

---

## `estream marketplace scaffold`

Generate a new component from a category-specific template.

### Synopsis

```
estream marketplace scaffold <CATEGORY> <NAME> [OPTIONS]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `CATEGORY` | Yes | Component category (determines template structure) |
| `NAME` | Yes | Component name |

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--path` | `./<NAME>` | Output directory for the scaffolded component |

### Generated Structure by Category

| Category | Extra Directories |
|----------|-------------------|
| `data-schema` | `schemas/`, `tests/golden/` |
| `wire-adapter` | `schemas/`, `tests/golden/` |
| `smart-circuit` | `schemas/`, `circuits/`, `tests/golden/` |
| `fpga-circuit` | `schemas/`, `circuits/`, `fpga/`, `tests/golden/` |
| `integration` | `schemas/`, `tests/golden/` |
| `console-widget` | `schemas/`, `widgets/dist/`, `tests/golden/` |

### Examples

```bash
# Scaffold a SmartCircuit
estream marketplace scaffold smart-circuit my-circuit

# Scaffold an FPGA circuit to a specific path
estream marketplace scaffold fpga-circuit ntt-accel --path ./fpga-components/ntt-accel

# Scaffold a data schema pack
estream marketplace scaffold data-schema my-schemas
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Scaffold created successfully |
| `1` | Invalid category or output directory already exists |

---

## `estream marketplace list`

List all installed components in the current workspace.

### Synopsis

```
estream marketplace list [OPTIONS]
```

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--format` | `table` | Output format: `table` or `json` |

### Examples

```bash
# Table format (default)
estream marketplace list

# JSON format for scripting
estream marketplace list --format json
```

### Table Output

```
  NAME                           VERSION    CATEGORY           INSTALLED
  --------------------------------------------------------------------------------
  data-trading                   1.2.0      data-schema        2026-02-20T12:00:00Z
  estream-wire-fix               1.0.0      wire-adapter       2026-02-20T12:01:00Z

  2 component(s) installed
```

### JSON Output

```json
[
  {
    "name": "data-trading",
    "version": "1.2.0",
    "category": "data-schema",
    "installed_at": "2026-02-20T12:00:00Z"
  }
]
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | List completed (may be empty) |

---

## `estream marketplace info`

Show detailed information for a component (installed, in registry, or known).

### Synopsis

```
estream marketplace info <COMPONENT> [OPTIONS]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `COMPONENT` | Yes | Component name |

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--version` | *(latest)* | Specific version to query |

### Examples

```bash
# Info for an installed component
estream marketplace info estream-wire-fix

# Info for a specific version
estream marketplace info estream-wire-fix --version 1.0.0
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Component information displayed |
| `1` | Component not found in any source |

---

## Global Patterns

### Alias

All commands support the `mp` alias:

```bash
estream mp search "fix"
estream mp install data-trading
estream mp publish --dry-run
```

### Workspace File

The `estream-workspace.toml` file in the project root tracks installed components.

### Component Storage

Installed component files live at `estream-workspace/components/<name>/`.

---

## See Also

- [Getting Started](./getting-started.md) — Quick walkthrough
- [Component Guide](./component-guide.md) — Authoring components
- [Security Model](./security-model.md) — Signature verification details
