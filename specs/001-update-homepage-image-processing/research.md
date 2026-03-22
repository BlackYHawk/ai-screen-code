# Research: Homepage Redesign - Embedded Image Processing

## Key Change from Previous Design

**Previous**: Homepage had 4 service cards that navigated to separate pages
**New**: Homepage has 3 embedded consumer tools (upload/process/download inline) + 1 developer entry card

---

## Technology Stack

**Chosen**: Keep current dependencies as-is
- React 19.2
- Tailwind CSS 4.x
- Vite 7.x
- TypeScript 5.9
- image-wasm (existing WASM library)

**Rationale**: No dependency changes needed. image-wasm already supports compress, watermark, portrait features.

---

## Image Processing (WASM)

### Existing `image-wasm` Library - Full Feature Set

| Feature | Implementation | Status |
|---------|---------------|--------|
| Compression | JPEG, PNG, WebP with quality levels | ✅ Implemented |
| Watermarking | Text watermarks, tiled pattern, rotation, color | ✅ Implemented |
| Portrait Photos | Background replacement + standard ID photo sizes | ✅ Implemented |

**Frontend integration files:**
- `frontend/src/types/image-wasm.ts` - TypeScript types
- `frontend/src/utils/imageProcessor.ts` - React integration utilities
- `frontend/public/wasm/image_wasm_bg.wasm` - WASM binary

---

## Homepage Layout Strategy

### Decision: Tabbed Tool Interface

**Layout:**
```
┌─────────────────────────────────────────────────────┐
│                    HERO SECTION                     │
│           专业图片处理平台                            │
│           图片不上传，保护隐私                        │
│              [立即开始]                             │
├─────────────────────────────────────────────────────┤
│  [压缩] [水印] [证件照] | [图生代码] (external)    │
├─────────────────────────────────────────────────────┤
│                                                     │
│   ┌───────────────────────────────────────────┐    │
│   │         ACTIVE TOOL PANEL                 │    │
│   │   Upload Zone | Controls | Download          │    │
│   └───────────────────────────────────────────┘    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Privacy Messaging Placement

**Decision**: "图片不上传，保护隐私" appears in:
1. Hero subtitle (immediately visible)
2. Tool panel header (contextually when uploading)
3. Trust section (reassurance)

### Developer Entry Design

**Decision**: Developer card uses distinct dark theme + tech badges, acts as navigation entry

| Aspect | Consumer Tools | Developer Entry |
|--------|---------------|----------------|
| Location | Tab within homepage | Separate card/section |
| Interaction | Upload → Process inline | Click → Navigate to /generate |
| Styling | Light, accessible | Dark theme, tech badges |
| CTA | Upload/Process | "开始使用" |

---

## Accessibility (WCAG 2.1 AA)

| Requirement | Implementation |
|-------------|----------------|
| 4.5:1 contrast | Tailwind `text-slate-600`+ for body, `text-slate-500`+ for 18px+ |
| Focus visible | `focus-visible:ring-2` always |
| Keyboard nav | Tab through upload, controls, download |
| Labels | `htmlFor`/`id` pairing or `aria-label` |
| Touch targets | Minimum 44x44px |
| Skip link | First focusable element |

---

## Files to Create/Modify

### New Components

| File | Purpose |
|------|---------|
| `frontend/src/components/home/ToolPanel.tsx` | Tab container for 3 consumer tools |
| `frontend/src/components/tools/CompressTool.tsx` | Embedded compression tool |
| `frontend/src/components/tools/WatermarkTool.tsx` | Embedded watermark tool |
| `frontend/src/components/tools/PortraitTool.tsx` | Embedded ID photo tool |
| `frontend/src/components/home/DeveloperCard.tsx` | Developer entry with nav |

### Modified Files

| File | Change |
|------|--------|
| `frontend/src/pages/HomePage.tsx` | Replace ServiceGrid with ToolPanel + DeveloperCard |
| `frontend/src/components/home/HeroSection.tsx` | Update with privacy messaging |
| `frontend/src/components/home/TrustSection.tsx` | Emphasize privacy |

### Deprecated (Can Discard)

| File | Note |
|------|------|
| `frontend/src/pages/ImageToolsPage.tsx` | Consumer tools now embedded on homepage |
| Consumer tool pages | No longer needed - inline on homepage |

---

## Implementation Notes

- Consumer tools (compress/watermark/portrait) are React components rendered inline
- Each tool manages its own state: `idle` → `uploading` → `processing` → `done`
- Download triggers browser save dialog, no server round-trip
- Developer "图生代码" remains a card that navigates to `/generate`
