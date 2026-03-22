# Quickstart: Homepage Redesign

## Development Setup

```bash
cd frontend
npm install
npm run dev
```

Server runs at http://localhost:5173

## Testing

```bash
# Unit tests
npm test

# E2E tests
npm run e2e
```

## Key Files

| File | Change |
|------|--------|
| `src/pages/HomePage.tsx` | Full redesign |
| `src/pages/ImageToolsPage.tsx` | Unchanged |
| `src/pages/GeneratePage.tsx` | Unchanged |

## Service Navigation

| Service | Route | Tab |
|---------|-------|-----|
| 图片压缩 | `/tools` | compress |
| 图片水印 | `/tools?tab=watermark` | watermark |
| 生成证件照 | `/tools?tab=portrait` | portrait |
| 图片转前端代码 | `/generate` | - |

## Implementation Order

1. Create `ServiceCard` component
2. Create `HeroSection` component
3. Create `ServiceGrid` layout
4. Create `SubscriptionBanner` (optional)
5. Assemble in `HomePage.tsx`
6. Add Playwright E2E tests for navigation
7. Run full test suite
8. Verify no console.log statements (constitution rule)
