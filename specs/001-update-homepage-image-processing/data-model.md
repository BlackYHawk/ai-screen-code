# Data Model: Embedded Homepage Image Processing

## Component Structure

### HomePage (Page Component)

**Purpose**: Main landing page with embedded consumer tools and developer entry

**Children**:
- `HeroSection` - with privacy messaging
- `ToolPanel` - tabbed container for 3 consumer tools
- `DeveloperCard` - entry point to /generate
- `TrustSection` - privacy reinforcement

---

### ToolPanel (Component)

**Purpose**: Tabbed container for embedded consumer image tools

**Props**: None (manages internal tab state)

**State**:
```typescript
type ToolType = 'compress' | 'watermark' | 'portrait'
type ToolState = 'idle' | 'uploading' | 'processing' | 'done' | 'error'

interface ToolPanelState {
  activeTab: ToolType
  toolState: ToolState
  uploadedFile: File | null
  processedUrl: string | null
  error: string | null
}
```

**Children**:
- `CompressTool`
- `WatermarkTool`
- `PortraitTool`

---

### CompressTool (Component)

**Purpose**: Inline image compression tool

**Props**: None (self-contained)

**State**:
```typescript
interface CompressState {
  file: File | null
  preview: string | null
  quality: number // 0-100, default 80
  processing: boolean
  resultUrl: string | null
}
```

**Controls**:
- Quality slider (0-100%)
- Upload zone (drag & drop or click)
- Download button (enabled after processing)

---

### WatermarkTool (Component)

**Purpose**: Inline image watermarking tool

**Props**: None (self-contained)

**State**:
```typescript
interface WatermarkState {
  file: File | null
  preview: string | null
  watermarkText: string
  position: 'center' | 'tiled' | 'corner'
  opacity: number // 0-100
  processing: boolean
  resultUrl: string | null
}
```

---

### PortraitTool (Component)

**Purpose**: Inline ID photo generation tool

**Props**: None (self-contained)

**State**:
```typescript
interface PortraitState {
  file: File | null
  preview: string | null
  format: '1寸' | '2寸' | '签证'
  bgColor: 'white' | 'blue' | 'red'
  processing: boolean
  resultUrl: string | null
}
```

---

### DeveloperCard (Component)

**Purpose**: Entry point to developer tool (image-to-code)

**Props**: None

**Visual**:
- Dark gradient background (gray-900 to gray-800)
- Tech badges: React, TypeScript, Tailwind
- CTA button: "开始使用" → navigates to `/generate`

---

### HeroSection (Component)

**Purpose**: Display main value proposition with privacy emphasis

**Visual Elements**:
- `<h1>`: "专业图片处理平台"
- `<p>`: "图片不上传，保护隐私"

---

## Routing Structure

| Path | Page | Purpose |
|------|------|---------|
| `/` | HomePage | Embedded tools + developer entry |
| `/generate` | GeneratePage | Full image-to-code tool (developer) |

**Note**: Consumer tool pages (`/tools`, `/tools?tab=*`) are deprecated - tools are now inline on homepage.

---

## Key Entities

| Entity | Type | Description |
|--------|------|-------------|
| ToolPanel | Component | Tab container for 3 embedded tools |
| CompressTool | Component | Inline compression with quality slider |
| WatermarkTool | Component | Inline watermark with text/position controls |
| PortraitTool | Component | Inline ID photo with format selector |
| DeveloperCard | Component | Navigation entry to /generate |
| HeroSection | Component | Hero with privacy messaging |
| TrustSection | Component | Privacy trust signals |

---

## Validation Rules

### File Upload
- Max file size: 10MB
- Accepted formats: JPEG, PNG, WebP
- Display error for invalid files

### Image Processing
- Processing must be client-side (WASM)
- No server upload for consumer tools
- Result available immediately after processing

### Accessibility
- All controls keyboard accessible
- Focus indicators visible
- Screen reader announces state changes
