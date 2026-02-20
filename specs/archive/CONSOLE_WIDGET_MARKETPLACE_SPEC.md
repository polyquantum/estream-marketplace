# Console Widget Marketplace Specification

> Extending the eStream marketplace to support console widgets as first-class distributable components.

**Status:** Draft  
**Version:** 1.0.0  
**Issue:** [#533](https://github.com/polyquantum/estream-io/issues/533)  
**Parent Epic:** [#524](https://github.com/polyquantum/estream-io/issues/524)  
**Dependencies:** Component Registry API (#525), SmartCircuit Package Format (#527), Console Widget System (#501)  
**Extends:** [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md), [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md)

---

## 1. Overview

Console widgets are first-class platform elements established in #501. The current system supports 14 built-in widgets registered at build time in `apps/console/src/widgets/index.ts`. This spec extends the marketplace to enable third-party widget publishing, runtime discovery, installation, and lifecycle management.

### 1.1 Current Widget System

The existing widget infrastructure (from `packages/sdk-browser/src/widgets/`):

| Component | Purpose |
|-----------|---------|
| `Widget` interface | Widget definition (id, title, category, component, roles, data sources) |
| `WidgetRegistry` | In-memory `Map<string, Widget>` with `registerWidget()` / `getWidgets()` |
| `WidgetPicker` | Modal UI for browsing and adding widgets to a dashboard |
| `WidgetFrame` | Chrome wrapper with header, resize, collapse |
| `WidgetGrid` | Dashboard grid layout manager |
| RBAC (`rbac.ts`) | WASM-backed role verification and access control |

### 1.2 What's Missing

- No path for third-party widget publishing
- No runtime widget discovery from marketplace
- No dynamic widget loading (all widgets bundled at build time)
- No ML-DSA-87 bundle verification
- No sandbox isolation for untrusted widgets
- No version management or dependency resolution for widgets

---

## 2. Widget as Component Type

### 2.1 Category Registration

Extend the marketplace component categories (from [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md)):

```toml
# estream-component.toml
[component]
name = "@synergy-carbon/impact-counter"
version = "1.0.0"
category = "console-widget"                  # NEW component type
description = "Real-time carbon impact visualization widget"
license = "Apache-2.0"
keywords = ["carbon", "sustainability", "dashboard", "visualization"]
```

### 2.2 Widget Metadata in Manifest

```toml
[component.widget]
# Widget interface declaration
widget_category = "economics"                # infrastructure | observability | governance |
                                             # developer | economics | registry
# RBAC requirements
required_roles = ["operator"]                # viewer | operator | developer | registrar |
                                             # governance | admin

# Supported sizes
default_size = { cols = 4, rows = 3 }
min_size = { cols = 2, rows = 2 }
max_size = { cols = 8, rows = 6 }

# Data source type
data_source = "eslite"                       # subscription | polling | static | eslite

# Lex stream subscriptions (if data_source = "subscription" or "eslite")
lex_streams = [
    "lex://estream/carbon/{lex_id}/impact",
    "lex://estream/carbon/{lex_id}/credits",
]

# ESLite tables required (if data_source = "eslite")
eslite_tables = ["carbon_credits", "carbon_attestations"]

# Console version compatibility
console_min_version = "0.8.0"

# Theme support
supports_light_mode = true
custom_accent = "#22C55E"                    # Optional accent color

# Optional backing ESCIR circuit
backing_circuit = "circuits/impact-aggregator.circuit.yaml"

# Bundle format
[component.widget.bundle]
entry = "dist/index.js"                      # ES module entry point
format = "esm"                               # esm (required)
framework = "react"                          # react (required for now)
```

---

## 3. Widget Package Format

### 3.1 Directory Structure

```
@synergy-carbon/impact-counter/
├── estream-component.toml               # Component manifest
├── widget.manifest.yaml                 # Widget-specific manifest (detailed)
├── README.md
├── LICENSE
│
├── src/                                 # Source code (TypeScript/TSX)
│   ├── ImpactCounterWidget.tsx          # Main widget component
│   ├── hooks/
│   │   └── useImpactData.ts
│   └── index.ts                         # Export entry
│
├── dist/                                # Built bundle (for publishing)
│   ├── index.js                         # Lazy-loadable ES module
│   └── index.js.map                     # Source map (optional)
│
├── assets/                              # Widget assets
│   ├── icon.svg                         # Widget icon (24x24 SVG)
│   ├── thumbnail.png                    # Preview thumbnail (400x300)
│   └── screenshot.png                   # Full screenshot (optional)
│
├── circuits/                            # Optional backing ESCIR circuits
│   └── impact-aggregator.circuit.yaml
│
├── schemas/                             # Optional ESF schemas
│   └── carbon-impact.esf.yaml
│
├── tests/                               # Widget tests
│   ├── e2e/                             # Playwright E2E specs
│   │   └── impact-counter.spec.ts
│   └── unit/                            # Unit tests
│       └── ImpactCounterWidget.test.tsx
│
└── SIGNATURE.ml-dsa                     # ML-DSA-87 package signature
```

### 3.2 `widget.manifest.yaml`

Detailed widget metadata beyond what `estream-component.toml` covers:

```yaml
# widget.manifest.yaml
widget:
  id: "@synergy-carbon/impact-counter"
  version: "1.0.0"
  title: "Carbon Impact Counter"
  
  # Visual identity
  icon: "assets/icon.svg"
  thumbnail: "assets/thumbnail.png"
  screenshots:
    - "assets/screenshot.png"
  
  # Interface declaration
  category: "economics"
  data_source: "eslite"
  required_roles: ["operator"]
  
  default_size:
    cols: 4
    rows: 3
  min_size:
    cols: 2
    rows: 2
  max_size:
    cols: 8
    rows: 6
  
  # Data requirements
  lex_streams:
    - pattern: "lex://estream/carbon/{lex_id}/impact"
      description: "Real-time carbon impact metrics"
      required: true
    - pattern: "lex://estream/carbon/{lex_id}/credits"
      description: "Carbon credit balances"
      required: false
  
  eslite_tables:
    - name: "carbon_credits"
      description: "Carbon credit records"
      access: "read"
    - name: "carbon_attestations"
      description: "Carbon attestation records"
      access: "read"
  
  # Configuration schema (rendered in widget settings)
  config_schema:
    properties:
      refresh_interval_ms:
        type: integer
        default: 5000
        description: "Data refresh interval in milliseconds"
      display_unit:
        type: string
        enum: ["tonnes", "kg", "lbs"]
        default: "tonnes"
        description: "Unit for carbon display"
  
  # Runtime constraints
  bundle:
    entry: "dist/index.js"
    format: "esm"
    framework: "react"
    max_bundle_size_kb: 500
  
  # Dependencies
  peer_dependencies:
    react: "^18.0.0"
    "@estream/sdk-browser": "^0.8.0"
```

---

## 4. Widget Bundle Build Pipeline

### 4.1 Build Process

Widgets are built from TypeScript/TSX source into a lazy-loadable ES module:

```bash
# Build the widget bundle
estream widget build

# Build flow:
# 1. TypeScript compilation (tsconfig.json)
# 2. Bundle with tree-shaking (esbuild / vite)
# 3. Externalize peer dependencies (react, @estream/sdk-browser)
# 4. Output: dist/index.js (ES module)
# 5. Generate source map (optional)
# 6. Validate bundle size
```

### 4.2 Bundle Requirements

| Requirement | Value | Rationale |
|-------------|-------|-----------|
| Format | ES Module | Dynamic `import()` for lazy loading |
| Externals | `react`, `react-dom`, `@estream/sdk-browser` | Shared from host console |
| Max size | 500 KB (default, configurable) | Fast loading |
| No `eval` | Required | CSP compliance |
| No inline scripts | Required | CSP compliance |
| Default export | `React.ComponentType<WidgetProps>` | Standard widget interface |

### 4.3 Bundle Contract

Every widget bundle must default-export a React component matching `WidgetProps`:

```typescript
// Widget bundle entry point (dist/index.js)
import type { WidgetProps } from '@estream/sdk-browser';

const ImpactCounterWidget: React.FC<WidgetProps> = ({ widget, theme, collapsed, config }) => {
  // Widget implementation
  return <div>...</div>;
};

export default ImpactCounterWidget;
```

---

## 5. Runtime Widget Installation

### 5.1 Registry Extension

Extend the existing `WidgetRegistry` (`packages/sdk-browser/src/widgets/registry.ts`) with marketplace operations:

```typescript
// New functions added to registry.ts

/**
 * Install a widget from the marketplace.
 * Fetches the bundle, verifies ML-DSA-87 signature, registers.
 */
export async function installWidget(
  marketplaceId: string,
  options?: InstallOptions,
): Promise<InstallResult> {
  // 1. Fetch widget manifest from marketplace registry
  const manifest = await fetchWidgetManifest(marketplaceId);
  
  // 2. Download bundle
  const bundle = await fetchWidgetBundle(manifest.bundle.url);
  
  // 3. Verify ML-DSA-87 signature
  const verification = await verifyWidgetSignature(bundle, manifest);
  if (!verification.valid) {
    throw new WidgetSecurityError('ML-DSA-87 signature verification failed');
  }
  
  // 4. Determine trust level
  const trustLevel = determineTrustLevel(manifest.publisher, verification);
  
  // 5. Register widget with appropriate isolation
  const widget = await loadWidgetBundle(bundle, manifest, trustLevel);
  registerWidget(widget);
  
  // 6. Persist installation record
  await persistInstallation(marketplaceId, manifest.version, verification);
  
  return { widgetId: widget.id, version: manifest.version, trustLevel };
}

/**
 * Uninstall a marketplace widget.
 * Removes from registry, cleans up ESLite subscriptions.
 */
export async function uninstallWidget(widgetId: string): Promise<void> {
  // 1. Find widget in registry
  const widget = getWidget(widgetId);
  if (!widget) throw new Error(`Widget not found: ${widgetId}`);
  
  // 2. Clean up ESLite subscriptions
  if (widget.lexStreams) {
    await cleanupLexSubscriptions(widgetId, widget.lexStreams);
  }
  
  // 3. Remove from registry
  unregisterWidget(widgetId);
  
  // 4. Remove persisted installation record
  await removeInstallation(widgetId);
  
  // 5. Revoke blob URLs / clean up loaded modules
  await cleanupWidgetResources(widgetId);
}

/**
 * Update a marketplace widget to a new version.
 * Hot-swaps the bundle without page reload.
 */
export async function updateWidget(widgetId: string): Promise<UpdateResult> {
  // 1. Check for newer version in marketplace
  const current = getInstalledVersion(widgetId);
  const available = await checkForUpdate(widgetId);
  
  if (!available || available.version === current) {
    return { updated: false, reason: 'already up to date' };
  }
  
  // 2. Install new version
  const newWidget = await installWidget(widgetId, { version: available.version });
  
  // 3. Hot-swap: unregister old, register new (preserves layout position)
  return { updated: true, from: current, to: available.version };
}
```

### 5.2 Installation Types

```typescript
interface InstallOptions {
  /** Specific version to install (default: latest) */
  version?: string;
  /** Skip signature verification (NOT recommended) */
  skipVerify?: boolean;
  /** Force reinstall even if already installed */
  force?: boolean;
}

interface InstallResult {
  widgetId: string;
  version: string;
  trustLevel: TrustLevel;
}

interface UpdateResult {
  updated: boolean;
  from?: string;
  to?: string;
  reason?: string;
}
```

---

## 6. Security Model

### 6.1 Trust Levels

```typescript
export type TrustLevel = 'trusted' | 'verified' | 'community';
```

| Trust Level | Publisher | Execution | API Surface |
|-------------|----------|-----------|-------------|
| **Trusted** | eStream organization | Direct in console context | Full console API, all data sources |
| **Verified** | Governance-reviewed, ML-DSA-87 signed | Within declared RBAC scope | Scoped to declared lex streams and ESLite tables |
| **Community** | Unreviewed | Sandboxed iframe | Limited API via `postMessage`, no direct DOM access |

### 6.2 Trust Level Determination

```typescript
function determineTrustLevel(
  publisher: PublisherInfo,
  verification: SignatureVerification,
): TrustLevel {
  // Official eStream widgets are always trusted
  if (publisher.name === 'estream' && verification.valid) {
    return 'trusted';
  }
  
  // Governance-approved widgets are verified
  if (verification.valid && verification.governanceApproved) {
    return 'verified';
  }
  
  // Everything else is community
  return 'community';
}
```

### 6.3 Trusted Widget Execution

Trusted widgets (from eStream org) run directly in the console React tree:

```typescript
// Direct integration — same as built-in widgets
const TrustedWidget = React.lazy(() => import(/* webpackIgnore: true */ bundleUrl));

<WidgetFrame widget={widgetDef}>
  <React.Suspense fallback={<WidgetSkeleton />}>
    <TrustedWidget widget={widgetDef} theme={theme} config={config} />
  </React.Suspense>
</WidgetFrame>
```

### 6.4 Verified Widget Execution

Verified widgets run in the console React tree but with scoped data access:

```typescript
// Scoped data gateway (enforces declared lex_streams and eslite_tables)
const scopedGateway = createScopedGateway(widgetDef.lexStreams, widgetDef.esliteTables);

<WidgetFrame widget={widgetDef}>
  <WidgetGatewayProvider gateway={scopedGateway}>
    <React.Suspense fallback={<WidgetSkeleton />}>
      <VerifiedWidget widget={widgetDef} theme={theme} config={config} />
    </React.Suspense>
  </WidgetGatewayProvider>
</WidgetFrame>
```

The `WidgetGatewayProvider` (from `gateway.ts`) restricts data access to only the declared streams and tables. Any attempt to access undeclared data sources throws a `WidgetSecurityError`.

### 6.5 Community Widget Sandbox

Community widgets run in a sandboxed iframe with `postMessage` communication:

```typescript
// Sandboxed iframe execution
<WidgetFrame widget={widgetDef}>
  <WidgetSandbox
    bundleUrl={bundleUrl}
    widgetId={widgetDef.id}
    config={config}
    allowedStreams={widgetDef.lexStreams || []}
    csp={COMMUNITY_CSP}
  />
</WidgetFrame>
```

**Content Security Policy (Community):**

```
Content-Security-Policy:
  default-src 'none';
  script-src blob:;
  style-src 'unsafe-inline';
  img-src data: blob:;
  connect-src 'none';
  frame-src 'none';
```

**`WidgetSandbox` Component:**

```typescript
interface WidgetSandboxProps {
  bundleUrl: string;
  widgetId: string;
  config: Record<string, unknown>;
  allowedStreams: string[];
  csp: string;
}

const WidgetSandbox: React.FC<WidgetSandboxProps> = ({
  bundleUrl, widgetId, config, allowedStreams, csp,
}) => {
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    // Set up postMessage bridge for data access
    const handler = (event: MessageEvent) => {
      if (event.source !== iframeRef.current?.contentWindow) return;

      switch (event.data.type) {
        case 'widget:data:request':
          // Validate stream is in allowedStreams
          if (!allowedStreams.includes(event.data.stream)) {
            event.source.postMessage({
              type: 'widget:data:error',
              error: 'Access denied: stream not declared',
            }, '*');
            return;
          }
          // Forward data request to gateway
          fetchStreamData(event.data.stream).then(data => {
            event.source!.postMessage({
              type: 'widget:data:response',
              requestId: event.data.requestId,
              data,
            }, '*');
          });
          break;
      }
    };

    window.addEventListener('message', handler);
    return () => window.removeEventListener('message', handler);
  }, [allowedStreams]);

  return (
    <iframe
      ref={iframeRef}
      sandbox="allow-scripts"
      src={createSandboxHtml(bundleUrl, widgetId, config)}
      style={{ width: '100%', height: '100%', border: 'none' }}
    />
  );
};
```

### 6.6 ML-DSA-87 Bundle Verification

Widget bundles are verified using the same ML-DSA-87 pipeline as SmartCircuit packages (see [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md)), with verification performed in the RBAC WASM module:

```typescript
async function verifyWidgetSignature(
  bundle: ArrayBuffer,
  manifest: WidgetManifest,
): Promise<SignatureVerification> {
  // Use the existing RBAC WASM module for ML-DSA-87 verification
  const rbacModule = await getRbacModule();
  
  // Compute SHA3-256 of bundle
  const checksum = rbacModule.sha3_256(new Uint8Array(bundle));
  
  // Verify ML-DSA-87 signature
  const valid = rbacModule.verify_mldsa87(
    manifest.signature.publicKey,
    checksum,
    manifest.signature.signatureBytes,
  );
  
  // Check governance approval status
  const governanceApproved = manifest.signature.governanceApprovals >= 
    manifest.governance?.requiredApprovals ?? 0;
  
  return { valid, governanceApproved, keyId: manifest.signature.keyId };
}
```

---

## 7. WidgetPicker Marketplace Tab

### 7.1 Extended WidgetPicker

Add a "Marketplace" tab to the existing `WidgetPicker` component:

```typescript
export interface WidgetPickerProps {
  isOpen: boolean;
  onClose: () => void;
  onAddWidget: (widgetId: string) => void;
  userRoles: WidgetRole[];
}

// Existing tabs: filter by WidgetCategory
// New tab: "Marketplace" — browse and install from marketplace
```

### 7.2 Marketplace Tab UI

```
┌──────────────────────────────────────────────────────────────────────┐
│  Add Widget                                                    [✕]   │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  [All] [Infrastructure] [Observability] [Economics] [Marketplace]    │
│                                                      ~~~~~~~~~~~~    │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │  🔍 Search marketplace widgets...                              │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌─────────────────────────┐  ┌─────────────────────────┐           │
│  │  [thumbnail]             │  │  [thumbnail]             │           │
│  │  Carbon Impact Counter   │  │  Trading Dashboard      │           │
│  │  @synergy-carbon         │  │  @acme-trading          │           │
│  │  ★★★★☆ (4.2) · Free     │  │  ★★★★★ (4.8) · 5 ES/mo │           │
│  │  ✓ Verified              │  │  ✓ Verified              │           │
│  │  [Install]               │  │  [Install]               │           │
│  └─────────────────────────┘  └─────────────────────────┘           │
│                                                                      │
│  ┌─────────────────────────┐  ┌─────────────────────────┐           │
│  │  [thumbnail]             │  │  [thumbnail]             │           │
│  │  IoT Monitor             │  │  Custom Chart Builder   │           │
│  │  @iot-works              │  │  community/chart-pro     │           │
│  │  ★★★☆☆ (3.5) · Free     │  │  ★★★★☆ (4.0) · Free     │           │
│  │  Community               │  │  Community (sandboxed)   │           │
│  │  [Install]               │  │  [Install]               │           │
│  └─────────────────────────┘  └─────────────────────────┘           │
│                                                                      │
│  ─── Installed ──────────────────────────────────────────────────── │
│                                                                      │
│  ┌─────────────────────────┐                                        │
│  │  Carbon Impact Counter   │                                        │
│  │  v1.0.0 · Installed      │                                        │
│  │  [Add to Dashboard] [⋮]  │                                        │
│  └─────────────────────────┘                                        │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 7.3 Implementation

```typescript
const MarketplaceTab: React.FC<{ userRoles: WidgetRole[] }> = ({ userRoles }) => {
  const [searchQuery, setSearchQuery] = useState('');
  const [marketplaceWidgets, setMarketplaceWidgets] = useState<MarketplaceWidgetEntry[]>([]);
  const [installedWidgets, setInstalledWidgets] = useState<InstalledWidgetEntry[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    // Fetch available widgets from marketplace
    fetchMarketplaceWidgets(searchQuery).then(setMarketplaceWidgets);
    // Fetch locally installed marketplace widgets
    getInstalledMarketplaceWidgets().then(setInstalledWidgets);
  }, [searchQuery]);

  const handleInstall = async (entry: MarketplaceWidgetEntry) => {
    setLoading(true);
    try {
      const result = await installWidget(entry.id);
      // Refresh installed list
      const updated = await getInstalledMarketplaceWidgets();
      setInstalledWidgets(updated);
    } catch (error) {
      showError(`Failed to install widget: ${error.message}`);
    }
    setLoading(false);
  };

  return (
    <div>
      <SearchBar value={searchQuery} onChange={setSearchQuery} />
      
      <Section title="Available">
        <WidgetCardGrid>
          {marketplaceWidgets.map(entry => (
            <MarketplaceWidgetCard
              key={entry.id}
              entry={entry}
              installed={installedWidgets.some(w => w.id === entry.id)}
              canAccess={hasRequiredRoles(userRoles, entry.requiredRoles)}
              onInstall={() => handleInstall(entry)}
            />
          ))}
        </WidgetCardGrid>
      </Section>
      
      <Section title="Installed">
        <WidgetCardGrid>
          {installedWidgets.map(entry => (
            <InstalledWidgetCard
              key={entry.id}
              entry={entry}
              onAdd={() => onAddWidget(entry.id)}
              onUninstall={() => uninstallWidget(entry.id)}
              onUpdate={() => updateWidget(entry.id)}
            />
          ))}
        </WidgetCardGrid>
      </Section>
    </div>
  );
};
```

---

## 8. Governance Circuit

### 8.1 Widget Publish Governance

Define `estream.marketplace.widget.publish.v1` — a governance circuit that validates widget publications:

```yaml
# circuits/governance/widget-publish.circuit.yaml
escir_version: "0.8.0"
name: estream.marketplace.widget.publish.v1
version: "1.0.0"

description: "Governance circuit for marketplace widget publishing"

types:
  - name: WidgetPublishRequest
    fields:
      - { name: widget_id, type: string }
      - { name: version, type: string }
      - { name: publisher_id, type: "[u8; 32]" }
      - { name: manifest_hash, type: "[u8; 32]" }
      - { name: bundle_hash, type: "[u8; 32]" }
      - { name: bundle_size_bytes, type: u64 }
      - { name: required_roles, type: "[string]" }
      - { name: requested_trust_level, type: string }

  - name: WidgetPublishDecision
    fields:
      - { name: approved, type: bool }
      - { name: trust_level, type: string }
      - { name: rejection_reasons, type: "[string]" }
      - { name: approver_signatures, type: "[bytes]" }

inputs:
  - name: request
    type: WidgetPublishRequest
  - name: publisher_identity
    type: PublisherIdentity

outputs:
  - name: decision
    type: WidgetPublishDecision

annotations:
  witness_tier: 2
  governance_category: Generic

compute:
  - id: identity_verifier
    type: transform
    description: "Verify ML-DSA-87 publisher identity"

  - id: manifest_validator
    type: transform
    description: "Validate widget manifest completeness and correctness"

  - id: bundle_integrity
    type: transform
    description: "Verify bundle hash and size limits"

  - id: role_validator
    type: transform
    description: "Validate declared RBAC roles are within allowed set"

  - id: version_checker
    type: transform
    description: "Enforce semver increment over previous version"

  - id: trust_evaluator
    type: stateful
    description: "Determine trust level based on publisher history"

  - id: approval_aggregator
    type: stateful
    description: "Collect and threshold governance approvals"

flows:
  - request -> identity_verifier -> manifest_validator -> bundle_integrity
  - bundle_integrity -> role_validator -> version_checker -> trust_evaluator
  - trust_evaluator -> approval_aggregator -> decision

invariants:
  - "bundle_size <= 5MB"
  - "required_roles subset of [viewer, operator, developer, registrar, governance, admin]"
  - "version > latest_published_version"
```

### 8.2 Validation Rules

| Check | Rule | Rejection Reason |
|-------|------|------------------|
| Publisher identity | ML-DSA-87 signed, key in registry | `invalid_publisher_identity` |
| Manifest completeness | All required fields present | `incomplete_manifest` |
| Bundle hash | SHA3-256 matches declared hash | `bundle_integrity_failed` |
| Bundle size | ≤ 5 MB | `bundle_too_large` |
| RBAC roles | All declared roles in valid set | `invalid_role_declaration` |
| Version increment | Strictly greater than latest | `version_not_incremented` |
| CSP compliance | No `eval`, no inline scripts | `csp_violation` |
| Peer dependencies | `react`, `@estream/sdk-browser` declared | `missing_peer_dependency` |

---

## 9. CLI Support

### 9.1 Widget Scaffolding

```bash
$ estream init --template widget @my-org/my-widget

  Created @my-org/my-widget/
  ├── estream-component.toml
  ├── widget.manifest.yaml
  ├── README.md
  ├── LICENSE
  ├── src/
  │   ├── MyWidget.tsx
  │   ├── hooks/
  │   │   └── useWidgetData.ts
  │   └── index.ts
  ├── assets/
  │   ├── icon.svg
  │   └── thumbnail.png
  ├── tests/
  │   ├── e2e/
  │   │   └── my-widget.spec.ts
  │   └── unit/
  │       └── MyWidget.test.tsx
  └── tsconfig.json

  Next steps:
    1. Edit src/MyWidget.tsx
    2. Run: estream widget dev (hot-reload preview)
    3. Build: estream widget build
    4. Test: estream widget test
    5. Publish: estream widget publish
```

### 9.2 Widget Development Server

```bash
$ estream widget dev

  Widget dev server started at http://localhost:3001
  
  Preview:  http://localhost:3001/preview
  Console:  Connected to local console at http://localhost:3000
  
  Watching for changes...
    src/MyWidget.tsx → rebuilt in 120ms
```

### 9.3 Widget Build

```bash
$ estream widget build

  Compiling TypeScript...
    ✓ 3 files compiled (0 errors)

  Bundling ES module...
    ✓ dist/index.js (42 KB, gzipped: 12 KB)

  Validating bundle...
    ✓ Default export: React.ComponentType<WidgetProps>
    ✓ No eval() usage
    ✓ No inline scripts
    ✓ Peer dependencies externalized
    ✓ Bundle size: 42 KB (limit: 500 KB)

  Build complete.
```

### 9.4 Widget Publish

```bash
$ estream widget publish

  Validating...
    ✓ estream-component.toml valid
    ✓ widget.manifest.yaml valid
    ✓ Bundle exists: dist/index.js (42 KB)
    ✓ Icon exists: assets/icon.svg
    ✓ Thumbnail exists: assets/thumbnail.png

  Signing with ML-DSA-87...
    ✓ Merkle root: sha3-256:abc123...
    ✓ Signature: 4627 bytes

  Publishing to estream-io/registry...
    ✓ Created PR #52: "Publish @my-org/my-widget v1.0.0"

  Done!
```

### 9.5 Widget Install (CLI)

```bash
$ estream widget install @synergy-carbon/impact-counter

  Downloading...
    @synergy-carbon/impact-counter v1.0.0 (42 KB)

  Verifying ML-DSA-87 signature...
    ✓ Valid (key: synergy-carbon-key-01)

  Installing to widgets/
    widgets/@synergy-carbon/impact-counter/

  Installed. Widget available in Console WidgetPicker.
```

---

## 10. Pricing and Licensing

Widget pricing follows the same models as circuit components (see [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md)):

| Model | Description | Metering |
|-------|-------------|----------|
| **Free** | Open source, no cost | None |
| **Subscription** | Monthly ES fee | Per-month |
| **Usage-based** | Per-load or per-interaction | Tracked via StreamSight |
| **Freemium** | Free base, paid premium features | Feature-gated |
| **Enterprise** | Custom licensing | Negotiated |

Source visibility applies to widget TypeScript/TSX source:
- **Open**: Full TSX source visible
- **Interface**: Only `widget.manifest.yaml` and compiled bundle visible
- **Compiled**: Bundle + source map visible, TSX hidden
- **Licensed Full**: TSX visible only to licensees

---

## 11. Widget Analytics (StreamSight)

Marketplace widgets emit analytics events to StreamSight:

```
lex://estream/sys/console/widgets/{widget_id}/loaded
lex://estream/sys/console/widgets/{widget_id}/interaction
lex://estream/sys/console/widgets/{widget_id}/error
lex://estream/sys/console/widgets/{widget_id}/uninstalled
```

These events power:
- Widget usage dashboards for creators
- Error monitoring and alerting
- "Frequently used together" recommendations
- Trending widget rankings

---

## 12. Implementation Phases

### Phase 1: Foundation
- [ ] Extend marketplace component types with `console-widget`
- [ ] Define `widget.manifest.yaml` schema
- [ ] Add `installWidget` / `uninstallWidget` to registry
- [ ] Add `WidgetCategory: 'marketplace'` filtering

### Phase 2: CLI and Publishing
- [ ] `estream init --template widget` scaffolding
- [ ] `estream widget build` (TSX → ES module)
- [ ] `estream widget publish` command
- [ ] `estream.marketplace.widget.publish.v1` governance circuit

### Phase 3: Discovery and Installation
- [ ] Marketplace tab in WidgetPicker
- [ ] Runtime widget loading (dynamic `import()`)
- [ ] ML-DSA-87 bundle verification (via RBAC WASM)
- [ ] Sandbox iframe isolation for community widgets

### Phase 4: Ecosystem
- [ ] Synergy Carbon Impact Widget (#431) as first marketplace widget
- [ ] Widget analytics via StreamSight
- [ ] Widget rating and review system
- [ ] "Frequently used together" recommendations

---

## References

- [COMPONENT_REGISTRY_API_SPEC.md](./COMPONENT_REGISTRY_API_SPEC.md) — Registry, manifest, CLI
- [SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md](./SMARTCIRCUIT_PACKAGE_FORMAT_SPEC.md) — Package signing
- [MARKETPLACE_SPEC.md](./MARKETPLACE_SPEC.md) — Pricing, visibility, creator program
- [ESF_SCHEMA_COMPOSITION_SPEC.md](../protocol/ESF_SCHEMA_COMPOSITION_SPEC.md) — Schema dependencies
- Console Widget System (#501) — Widget types, registry, WidgetPicker

---

*Created: 2026-02-11*  
*Status: Draft*  
*Issue: #533*
