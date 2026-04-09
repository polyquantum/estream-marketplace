# Stream API Reference

> Programmatic access to the eStream Marketplace via native Stream API topics.

---

## Overview

All marketplace operations flow over eStream's native Stream API. Each operation publishes or subscribes to typed stream topics, carrying FastLang-defined data structures. The stream-based architecture provides built-in ordering, persistence, witness attestation, and observability through Observe.

Data types are defined in `circuits/marketplace/marketplace_types.fl` and `circuits/marketplace/marketplace_streams.fl`.

---

## Stream Topics

### `/marketplace/index`

**Stream:** `marketplace_index`
**Event Type:** `ComponentNode`
**Retention:** 365 days
**Consumers:** Observe, Console, CLI

The component index stream. Every published component emits a `ComponentNode` event here.

**ComponentNode Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `node_id` | `bytes(32)` | Unique component identifier |
| `component_name` | `bytes(64)` | Component name (padded) |
| `component_name_len` | `u8` | Actual length of component name |
| `category_code` | `u8` | 0=data-schema, 1=wire-adapter, 2=smart-circuit, 3=fpga-circuit, 4=integration, 5=console-widget |
| `publisher_id` | `bytes(32)` | Publisher's SPARK identity fingerprint |
| `version_major` | `u16` | Semantic version major |
| `version_minor` | `u16` | Semantic version minor |
| `version_patch` | `u16` | Semantic version patch |
| `created_at` | `u64` | Unix epoch timestamp |

---

### `/marketplace/search`

**Stream:** `marketplace_search`
**Event Type:** `SearchResultEvent`
**Retention:** 7 days
**Consumers:** Observe, Console, CLI

**SearchResultEvent Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_hash` | `bytes(32)` | Hash of the original search query |
| `component_id` | `bytes(32)` | Matching component's node ID |
| `relevance_score` | `u32` | Relevance ranking (higher = more relevant) |

**SearchQuery Request Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `search_text` | `bytes(128)` | Query string (padded) |
| `search_text_len` | `u8` | Actual length of search text |
| `filter_category` | `u8` | Category filter (0xFF = no filter) |
| `filter_min_rating` | `u8` | Minimum rating filter (0-100) |
| `filter_license` | `u8` | License type filter |
| `page_offset` | `u32` | Pagination offset |
| `page_limit` | `u16` | Results per page |

**SearchResult Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_hash` | `bytes(32)` | Correlates to the original query |
| `result_count` | `u32` | Number of results in this page |
| `total_matches` | `u64` | Total matching components |
| `page_offset` | `u32` | Current page offset |
| `has_more` | `bool` | Whether more pages are available |

---

### `/marketplace/install/{requestId}`

**Stream:** `marketplace_install`
**Event Type:** `InstallEvent`
**Retention:** 90 days
**Consumers:** Observe, Console, CLI, Audit

**InstallEvent Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Unique installation request identifier |
| `component_id` | `bytes(32)` | Component being installed |
| `install_status` | `u8` | 0=pending, 1=downloading, 2=verifying, 3=installing, 4=complete, 5=failed |
| `progress_pct` | `u8` | Progress percentage (0-100) |

**InstallRequest Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Unique request identifier |
| `component_name` | `bytes(64)` | Component to install (padded) |
| `component_name_len` | `u8` | Actual length of component name |
| `version_constraint` | `bytes(32)` | Version constraint string (padded) |
| `version_constraint_len` | `u8` | Actual length of version constraint |
| `target_project_hash` | `bytes(32)` | Target workspace identifier |

**InstallProgress Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Correlates to the installation request |
| `download_pct` | `u8` | Download progress (0-100) |
| `verify_status` | `u8` | 0=pending, 1=verifying, 2=passed, 3=failed |
| `install_status` | `u8` | 0=pending, 1=extracting, 2=linking, 3=complete, 4=failed |
| `dependency_total` | `u16` | Total dependencies to resolve |
| `dependencies_resolved` | `u16` | Dependencies resolved so far |
| `error_code` | `u16` | Error code (0 = no error) |

---

### `/marketplace/publish/{requestId}`

**Stream:** `marketplace_publish`
**Event Type:** `PublishEvent`
**Retention:** 365 days
**Consumers:** Observe, Console, CLI, Governance, Audit

**PublishEvent Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Unique publish request identifier |
| `component_id` | `bytes(32)` | Component being published |
| `publish_status` | `u8` | 0=pending, 1=validating, 2=signing, 3=uploading, 4=complete, 5=failed |
| `validation_pct` | `u8` | Validation progress (0-100) |

**PublishRequest Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Unique request identifier |
| `manifest_hash` | `bytes(32)` | SHA3-256 hash of the manifest |
| `tarball_hash` | `bytes(32)` | SHA3-256 hash of the archive |
| `signature_hash` | `bytes(32)` | SHA3-256 hash of the signature |
| `publisher_key_id` | `bytes(32)` | Publisher's ML-DSA-87 public key fingerprint |

**PublishProgress Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `request_id` | `bytes(16)` | Correlates to the publish request |
| `validation_pct` | `u8` | Validation progress (0-100) |
| `review_status` | `u8` | 0=pending, 1=reviewing, 2=approved, 3=rejected |
| `publish_status` | `u8` | 0=pending, 1=uploading, 2=indexing, 3=complete, 4=failed |
| `error_code` | `u16` | Error code (0 = no error) |

---

### `/marketplace/licenses`

**Stream:** `marketplace_licenses`
**Event Type:** `LicenseEvent`
**Retention:** 365 days
**Consumers:** Observe, Console, Billing

**LicenseEvent Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `license_id` | `bytes(32)` | Unique license identifier |
| `component_id` | `bytes(32)` | Licensed component |
| `license_type` | `u8` | 0=free, 1=one-time, 2=subscription, 3=usage-based, 4=enterprise, 5=freemium |
| `granted_at` | `u64` | Unix epoch grant timestamp |
| `expiry_timestamp` | `u64` | Unix epoch expiry (0 = perpetual) |

---

## Graph Structures

### Registry Graph — `marketplace_registry`

```
graph marketplace_registry {
    node ComponentNode
    edge PublishesEdge
    overlay download_count: u64 bitmask delta_curate
    overlay active_installs: u64 bitmask delta_curate
    overlay rating_x100: u32 bitmask delta_curate
    overlay badges: u32 bitmask
    overlay revenue_es: u64 bitmask delta_curate
    overlay vulnerability_count: u16 curate delta_curate
    overlay signature_valid: bool curate delta_curate
    storage csr { hot @bram, warm @ddr, cold @nvme }
}
```

**Overlays:**

| Overlay | Type | Description |
|---------|------|-------------|
| `download_count` | `u64` | Total downloads (delta-curated) |
| `active_installs` | `u64` | Current active installations |
| `rating_x100` | `u32` | Average rating x100 (450 = 4.50 stars) |
| `badges` | `u32` | Bitmask of assigned badges |
| `revenue_es` | `u64` | Total revenue in eStream units |
| `vulnerability_count` | `u16` | Known vulnerabilities |
| `signature_valid` | `bool` | Whether the latest signature is valid |

### Dependency DAG — `component_dependencies`

```
dag component_dependencies {
    node ComponentNode
    edge DependsOnEdge
    overlay version_conflict: bool curate delta_curate
    overlay install_order: u32 curate
    storage csr { hot @bram, warm @ddr, cold @nvme }
}
```

---

## Circuits

### `marketplace_search_query`

```
circuit marketplace_search_query(
    registry_id: bytes(32),
    search_req: SearchQuery
) -> u32
```

| Property | Value |
|----------|-------|
| Lex path | `esn/marketplace/search` |
| Precision | B |
| Observed metrics | `search_text_len`, `filter_category`, `result_count` |

### `marketplace_verify_signature`

```
circuit marketplace_verify_signature(
    pkg_hash: bytes(32),
    sig_hash: bytes(32),
    pubkey_id: bytes(32)
) -> bool
```

| Property | Value |
|----------|-------|
| Lex path | `esn/marketplace/verify` |
| Precision | C |
| PoVC | Enabled |
| Observed metrics | `pkg_hash`, `pubkey_id`, `sig_valid` |

---

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| `0` | `OK` | No error |
| `1` | `NOT_FOUND` | Component not found in registry |
| `2` | `VERSION_CONFLICT` | Requested version conflicts with existing dependencies |
| `3` | `SIGNATURE_INVALID` | ML-DSA-87 signature verification failed |
| `4` | `MANIFEST_INVALID` | Component manifest validation failed |
| `5` | `DEPENDENCY_CYCLE` | Circular dependency detected in DAG |
| `6` | `SCHEMA_MISSING` | Required schema not available |
| `7` | `PERMISSION_DENIED` | Publisher not authorized (SPARK auth failure) |
| `8` | `QUOTA_EXCEEDED` | Storage or rate limit exceeded |
| `9` | `NETWORK_ERROR` | Registry communication failure |
| `10` | `ARCHIVE_CORRUPT` | Archive integrity check failed |
| `11` | `LICENSE_EXPIRED` | Component license has expired |
| `12` | `GOVERNANCE_REJECTED` | Governance review rejected the component |

---

## Data Type Encoding Conventions

| Convention | Example | Description |
|-----------|---------|-------------|
| Ratings | `rating_x100 = 450` | Multiply by 100 (4.50 stars) |
| Amounts | Stored in smallest unit | Integer-only financial values |
| Dates | Unix epoch `u64` | Seconds since 1970-01-01T00:00:00Z |
| Strings | `bytes(N)` + `_len: u8` | Fixed-size buffer with length field |
| Booleans | `bool` or `u8` | Native bool or 0/1 |

---

## See Also

- [CLI Reference](./cli-reference.md) — Command-line equivalents for all API operations
- [Security Model](./security-model.md) — ML-DSA-87 signature details
- [Component Guide](./component-guide.md) — Schema provides/requires
