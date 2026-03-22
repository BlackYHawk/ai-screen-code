# Implementation Plan: Homepage Image Processing Platform

**Branch**: `001-update-homepage-image-processing` | **Date**: 2026-03-21 | **Spec**: [spec.md](./spec.md)
**Input**: Homepage redesign - embedded consumer tools with direct upload, privacy-first messaging, developer entry to secondary page

## Summary

Redesign homepage as an embedded image processing platform where consumers can compress images, add watermarks, and generate ID photos directly on the homepage without navigation. Developer service (image-to-code) navigates to `/generate`. Key differentiator: "images never uploaded, processed client-side only."

## Technical Context

**Language/Version**: TypeScript 5.9, React 19.2
**Primary Dependencies**: React 19, Tailwind CSS 4.x, Lucide React, React Router DOM 7, image-wasm
**Storage**: N/A (client-side only, no persistent storage)
**Testing**: Vitest (unit/integration), Playwright (E2E)
**Target Platform**: Web browser (SPA)
**Project Type**: React SPA with client-side WASM image processing
**Performance Goals**: Homepage load <2s, image processing <3s for 10MB image
**Constraints**: Client-side only (privacy), 320px minimum responsive, WCAG 2.1 AA
**Scale/Scope**: Single homepage with 3 embedded tools + 1 navigation entry

## Constitution Check

| Gate | Status | Notes |
|------|--------|-------|
| Test Verification Gate | ✅ PASS | TDD required, 80%+ coverage |
| TDD Approach | ✅ PASS | Red-Green-Refactor cycle |
| Modularity | ✅ PASS | Components reusable |
| Code Quality | ✅ PASS | ESLint, no console.log |
| Security | ✅ PASS | Client-side only, no secrets |

## Project Structure

```text
frontend/
├── src/
│   ├── components/
│   │   ├── home/
│   │   │   ├── HeroSection.tsx
│   │   │   ├── TrustSection.tsx
│   │   │   ├── SubscriptionBanner.tsx
│   │   │   └── ToolPanel.tsx          # NEW: Embedded tool container
│   │   ├── tools/                      # NEW: Embedded tool components
│   │   │   ├── CompressTool.tsx        # NEW: Inline compression tool
│   │   │   ├── WatermarkTool.tsx       # NEW: Inline watermark tool
│   │   │   └── PortraitTool.tsx        # NEW: Inline ID photo tool
│   │   └── ui/
│   │       ├── Button.tsx
│   │       ├── Card.tsx
│   │       └── Layout.tsx
│   ├── pages/
│   │   └── HomePage.tsx                # UPDATED: Embedded tools
│   │   └── GeneratePage.tsx            # EXISTS: Developer tool page
│   ├── hooks/
│   │   └── useImageProcessor.ts         # UPDATED: WASM processor hook
│   └── utils/
│       └── imageProcessor.ts            # EXISTS: WASM utilities
└── tests/
    ├── unit/components/home/
    └── e2e/
```

**Structure Decision**: Single-page design with embedded tool UIs. No separate tool pages for consumer services. Developer flow uses `/generate` page.

## Key Changes from Previous Design

| Aspect | Old Design | New Design |
|--------|-----------|------------|
| Consumer tools | Card links → separate pages | **Embedded directly on homepage** |
| Consumer tool access | Click → navigate | **Upload → Process → Download inline** |
| Developer service | Card on homepage | Card → `/generate` page |
| Privacy messaging | Trust section only | **Prominent near every tool** |
| Navigation | All 4 services link out | Only developer service navigates |

## Implementation Notes

- image-wasm handles compress/watermark client-side
- No backend calls for image processing (privacy)
- Embedded tools use React state + image-wasm directly
- Developer card remains for image-to-code with navigation
