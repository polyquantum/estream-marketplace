# eStream Marketplace — Brand Guidelines

> Visual identity specifications for the eStream Marketplace. All marketplace UIs, documentation, and third-party integrations should follow these guidelines for consistency with the eStream brand.

---

## Color Palette

### Primary Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **eStream Blue** | `#1E40AF` | `30, 64, 175` | Primary brand color, buttons, links, headers |
| **Circuit Green** | `#10B981` | `16, 185, 129` | Success states, verified badges, positive actions |
| **Slate Background** | `#0F172A` | `15, 23, 42` | Page background, dark surfaces |
| **Slate Surface** | `#1E293B` | `30, 41, 59` | Cards, panels, elevated surfaces |

### Extended Palette

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Blue 400** | `#60A5FA` | `96, 165, 250` | Hover states, secondary text on dark |
| **Blue 600** | `#2563EB` | `37, 99, 235` | Active/pressed states |
| **Green 500** | `#22C55E` | `34, 197, 94` | Success indicators, install complete |
| **Red 500** | `#EF4444` | `239, 68, 68` | Error states, verification failures |
| **Amber 500** | `#F59E0B` | `245, 158, 11` | Warning states, deprecation notices |
| **Slate 400** | `#94A3B8` | `148, 163, 184` | Secondary text, descriptions |
| **Slate 600** | `#475569` | `71, 85, 105` | Borders, dividers |
| **White** | `#F8FAFC` | `248, 250, 252` | Primary text on dark backgrounds |

### CSS Variables

```css
:root {
  /* Primary */
  --es-blue: #1E40AF;
  --es-green: #10B981;
  --es-bg: #0F172A;
  --es-surface: #1E293B;

  /* Extended */
  --es-blue-400: #60A5FA;
  --es-blue-600: #2563EB;
  --es-green-500: #22C55E;
  --es-red-500: #EF4444;
  --es-amber-500: #F59E0B;
  --es-slate-400: #94A3B8;
  --es-slate-600: #475569;
  --es-text: #F8FAFC;
}
```

---

## Typography

### Font Families

| Role | Font | Weight | Usage |
|------|------|--------|-------|
| **Headings** | Inter | Bold (700) | Page titles, section headers, card titles |
| **Body** | Inter | Regular (400) | Descriptions, paragraphs, labels |
| **Code** | JetBrains Mono | Regular (400) | Code blocks, CLI output, manifest examples |
| **Monospace UI** | JetBrains Mono | Medium (500) | Component names, version numbers in UI |

### Font Loading

Load via `@fontsource` (recommended) or CDN:

```css
/* @fontsource (npm) */
@import '@fontsource/inter/400.css';
@import '@fontsource/inter/700.css';
@import '@fontsource/jetbrains-mono/400.css';
@import '@fontsource/jetbrains-mono/500.css';
```

```html
<!-- CDN fallback -->
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
```

### Type Scale

| Element | Size | Weight | Line Height | Letter Spacing |
|---------|------|--------|-------------|----------------|
| H1 | 32px / 2rem | 700 | 1.2 | -0.02em |
| H2 | 24px / 1.5rem | 700 | 1.3 | -0.01em |
| H3 | 20px / 1.25rem | 700 | 1.4 | 0 |
| Body | 16px / 1rem | 400 | 1.6 | 0 |
| Small | 14px / 0.875rem | 400 | 1.5 | 0 |
| Caption | 12px / 0.75rem | 400 | 1.4 | 0.01em |
| Code | 14px / 0.875rem | 400 | 1.6 | 0 |

---

## Component Card Design

Component cards are the primary UI element for browsing and search results.

### Specifications

```
┌────────────────────────────────────────────────────────────┐
│                                                            │
│  padding: 16px                                             │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  [Category Icon]  Component Name        v1.0.0      │  │
│  │                                                     │  │
│  │  Short description of the component that may        │  │
│  │  wrap to two lines maximum.                         │  │
│  │                                                     │  │
│  │  [verified] [pq-signed] [fpga-ready]                │  │
│  │                                                     │  │
│  │  ★★★★☆  4.5  ·  1,234 installs  ·  free            │  │
│  │                                                     │  │
│  │  Publisher Name                          [Install]   │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

| Property | Value |
|----------|-------|
| **Background** | `var(--es-surface)` / `#1E293B` |
| **Border** | 1px solid `var(--es-slate-600)` / `#475569` |
| **Border Radius** | 8px |
| **Padding** | 16px |
| **Shadow** | `0 1px 3px rgba(0, 0, 0, 0.3)` |
| **Hover Shadow** | `0 4px 12px rgba(0, 0, 0, 0.4)` |
| **Hover Border** | 1px solid `var(--es-blue-400)` / `#60A5FA` |
| **Transition** | `all 150ms ease-in-out` |
| **Max Width** | 100% (responsive grid) |
| **Min Height** | 160px |

### Card Typography

| Element | Font | Size | Color |
|---------|------|------|-------|
| Component name | Inter Bold | 16px | `var(--es-text)` |
| Version | JetBrains Mono | 12px | `var(--es-slate-400)` |
| Description | Inter Regular | 14px | `var(--es-slate-400)` |
| Publisher | Inter Regular | 12px | `var(--es-blue-400)` |
| Stats | Inter Regular | 12px | `var(--es-slate-400)` |

---

## Badge Designs

Badges appear on component cards and detail views to convey trust signals and capabilities.

### Badge Specifications

| Property | Value |
|----------|-------|
| **Shape** | Pill (rounded rectangle) |
| **Height** | 22px |
| **Padding** | 4px 8px |
| **Border Radius** | 11px (full pill) |
| **Font** | Inter Bold |
| **Font Size** | 11px |
| **Letter Spacing** | 0.03em |
| **Text Transform** | Uppercase |

### Badge Colors

| Badge | Background | Text | Border |
|-------|-----------|------|--------|
| **Verified** | `#10B981` (Green) | `#FFFFFF` | none |
| **Official** | `#1E40AF` (Blue) | `#FFFFFF` | none |
| **PQ-Signed** | `#7C3AED` (Violet) | `#FFFFFF` | none |
| **FPGA-Ready** | `#F59E0B` (Amber) | `#0F172A` | none |
| **Community** | transparent | `#94A3B8` | 1px solid `#475569` |

### Badge Display Order

Always display badges in this order: Verified, Official, PQ-Signed, FPGA-Ready, Community.

---

## Category Icons and Colors

Each component category has a distinct icon and accent color for quick visual identification.

| Category | Icon | Accent Color | Hex |
|----------|------|-------------|-----|
| `data-schema` | Database / Table | Blue | `#3B82F6` |
| `wire-adapter` | Plug / Connection | Teal | `#14B8A6` |
| `smart-circuit` | CPU / Chip | Violet | `#8B5CF6` |
| `fpga-circuit` | Circuit Board | Amber | `#F59E0B` |
| `integration` | Puzzle Piece | Emerald | `#10B981` |
| `console-widget` | Layout / Dashboard | Rose | `#F43F5E` |

Use the category accent color for:
- Category label text on component cards
- Category filter pills in search
- Category icon backgrounds

---

## "Powered by eStream" Badge

For third-party integrations and components that want to signal eStream compatibility.

### Specifications

```
┌──────────────────────────────────┐
│  ⚡ Powered by eStream           │
└──────────────────────────────────┘
```

| Property | Value |
|----------|-------|
| **Background** | `#0F172A` (Slate Background) |
| **Text** | `#F8FAFC` (White) |
| **Border** | 1px solid `#475569` |
| **Border Radius** | 6px |
| **Padding** | 6px 12px |
| **Font** | Inter Bold 12px |
| **Icon** | Lightning bolt, `#60A5FA` |

### Usage Rules

- Place in footer or sidebar of third-party applications
- Do not alter colors, proportions, or typography
- Maintain minimum 8px clear space on all sides
- Do not place on backgrounds lighter than `#334155`

### Variants

| Variant | Background | Use Case |
|---------|-----------|----------|
| **Dark** (default) | `#0F172A` | Dark UIs and documentation |
| **Light** | `#F8FAFC` with `#1E40AF` text | Light-themed contexts |
| **Minimal** | Transparent with `#1E40AF` text | Inline references |

---

## Layout Grid

### Desktop (>= 1024px)

- Component cards: 3-column grid, 24px gap
- Sidebar: 280px fixed width
- Content area: fluid

### Tablet (768px — 1023px)

- Component cards: 2-column grid, 16px gap
- Sidebar: collapsed to top navigation
- Content area: fluid

### Mobile (< 768px)

- Component cards: 1-column stack, 12px gap
- Sidebar: bottom sheet or hamburger menu
- Content area: full width with 16px padding

---

## Accessibility

- All text meets WCAG 2.1 AA contrast ratios against their backgrounds
- Badge text on colored backgrounds meets minimum 4.5:1 contrast
- Interactive elements have visible focus indicators (`outline: 2px solid var(--es-blue-400)`)
- Color is never the sole means of conveying information (badges include text labels)

---

## Related

- [Badge Descriptions](./badge-descriptions.md) — Full badge system documentation
- [Security Model](../docs/security-model.md) — What badges signify about component security
