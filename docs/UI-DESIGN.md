# AI Screen Code - UI/UX 设计规范

## 1. 设计原则

### 1.1 核心设计原则

1. **简洁高效** - 减少用户操作步骤，快速完成任务
2. **直观反馈** - 每个操作都有明确的视觉反馈
3. **一致性** - 统一的视觉语言和交互模式
4. **可访问性** - 考虑不同用户群体的使用体验

### 1.2 设计价值观

- **清晰优先**: 信息层次分明，重点突出
- **效率至上**: 最短路径完成任务
- **愉悦体验**: 细节打磨带来愉悦感

---

## 2. 色彩系统

### 2.1 品牌色

```css
:root {
  /* 主色 - 科技蓝 */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-200: #bfdbfe;
  --color-primary-300: #93c5fd;
  --color-primary-400: #60a5fa;
  --color-primary-500: #3b82f6;
  --color-primary-600: #2563eb;  /* 主色 */
  --color-primary-700: #1d4ed8;
  --color-primary-800: #1e40af;  /* 深蓝 */
  --color-primary-900: #1e3a8a;

  /* 强调色 - 成功绿 */
  --color-success-50: #f0fdf4;
  --color-success-500: #10b981;  /* 成功 */
  --color-success-600: #059669;

  /* 警告色 */
  --color-warning-50: #fffbeb;
  --color-warning-500: #f59e0b;  /* 警告 */
  --color-warning-600: #d97706;

  /* 错误色 */
  --color-error-50: #fef2f2;
  --color-error-500: #ef4444;  /* 错误 */
  --color-error-600: #dc2626;

  /* 背景色 */
  --color-bg-primary: #f9fafb;    /* 页面背景 */
  --color-bg-secondary: #ffffff;   /* 卡片背景 */
  --color-bg-tertiary: #f3f4f6;    /* 输入框背景 */
  --color-bg-dark: #111827;        /* 深色背景 */

  /* 文字色 */
  --color-text-primary: #111827;   /* 主要文字 */
  --color-text-secondary: #4b5563; /* 次要文字 */
  --color-text-tertiary: #9ca3af;  /* 辅助文字 */
  --color-text-inverse: #ffffff;   /* 反白文字 */

  /* 边框色 */
  --color-border: #e5e7eb;         /* 默认边框 */
  --color-border-focus: #3b82f6;   /* 聚焦边框 */
}
```

### 2.2 语义化色彩使用

| 场景 | 色彩 | CSS变量 |
|------|------|---------|
| 主按钮 | 蓝色 | `var(--color-primary-600)` |
| 成功状态 | 绿色 | `var(--color-success-500)` |
| 警告状态 | 橙色 | `var(--color-warning-500)` |
| 错误状态 | 红色 | `var(--color-error-500)` |
| 禁用状态 | 灰色 | `var(--color-text-tertiary)` |

### 2.3 模型标签色彩

```css
.model-tag-qwen { background: #e0f2fe; color: #0369a1; }    /* 浅蓝 */
.model-tag-minimax { background: #fce7f3; color: #db2777; } /* 粉色 */
.model-tag-kimi { background: #fef3c7; color: #d97706; }    /* 橙色 */
.model-tag-glm { background: #dcfce7; color: #16a34a; }     /* 绿色 */
```

---

## 3. 字体系统

### 3.1 字体族

```css
:root {
  /* 主字体 - 无衬线 */
  --font-family-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;

  /* 代码字体 */
  --font-family-mono: 'JetBrains Mono', 'Fira Code', 'Monaco', monospace;

  /* 中文字体回退 */
  --font-family-cn: 'PingFang SC', 'Microsoft YaHei', 'Hiragino Sans GB', sans-serif;
}
```

### 3.2 字号系统

```css
:root {
  /* 标题字号 */
  --text-xs: 0.75rem;    /* 12px - 标签 */
  --text-sm: 0.875rem;    /* 14px - 辅助文字 */
  --text-base: 1rem;      /* 16px - 正文 */
  --text-lg: 1.125rem;    /* 18px - 小标题 */
  --text-xl: 1.25rem;     /* 20px - 标题 */
  --text-2xl: 1.5rem;     /* 24px - 大标题 */
  --text-3xl: 1.875rem;   /* 30px - 页面标题 */
  --text-4xl: 2.25rem;    /* 36px - Hero标题 */

  /* 行高 */
  --leading-tight: 1.25;
  --leading-normal: 1.5;
  --leading-relaxed: 1.75;

  /* 字重 */
  --font-normal: 400;
  --font-medium: 500;
  --font-semibold: 600;
  --font-bold: 700;
}
```

### 3.3 排版规范

```css
/* 标题 */
h1 { font-size: var(--text-4xl); font-weight: var(--font-bold); line-height: var(--leading-tight); }
h2 { font-size: var(--text-3xl); font-weight: var(--font-bold); line-height: var(--leading-tight); }
h3 { font-size: var(--text-2xl); font-weight: var(--font-semibold); line-height: var(--leading-tight); }
h4 { font-size: var(--text-xl); font-weight: var(--font-semibold); }

/* 正文 */
p { font-size: var(--text-base); line-height: var(--leading-normal); }

/* 代码 */
code, pre { font-family: var(--font-family-mono); font-size: var(--text-sm); }
```

---

## 4. 间距系统

### 4.1 基础间距

```css
:root {
  --space-1: 0.25rem;   /* 4px */
  --space-2: 0.5rem;    /* 8px */
  --space-3: 0.75rem;   /* 12px */
  --space-4: 1rem;      /* 16px */
  --space-5: 1.25rem;   /* 20px */
  --space-6: 1.5rem;    /* 24px */
  --space-8: 2rem;      /* 32px */
  --space-10: 2.5rem;   /* 40px */
  --space-12: 3rem;     /* 48px */
  --space-16: 4rem;     /* 64px */
}
```

### 4.2 组件间距

```css
/* 页面边距 */
--page-padding-x: 1rem;      /* 移动端 */
--page-padding-x: 2rem;      /* 桌面端 */

/* 卡片内边距 */
--card-padding-sm: 1rem;
--card-padding-md: 1.5rem;
--card-padding-lg: 2rem;

/* 元素间距 */
--gap-sm: 0.5rem;
--gap-md: 1rem;
--gap-lg: 1.5rem;
--gap-xl: 2rem;
```

---

## 5. 阴影系统

### 5.1 阴影层次

```css
:root {
  /* 无阴影 */
  --shadow-none: none;

  /* 卡片阴影 */
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
  --shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);

  /* 浮层阴影 */
  --shadow-dropdown: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
  --shadow-modal: 0 25px 50px -12px rgb(0 0 0 / 0.25);

  /* 聚焦阴影 */
  --shadow-focus: 0 0 0 3px rgb(59 130 246 / 0.5);
}
```

### 5.2 阴影使用场景

| 场景 | 阴影 | 效果 |
|------|------|------|
| 按钮悬停 | `shadow-md` | 轻微浮起 |
| 卡片 | `shadow-sm` | 轻微投影 |
| 下拉菜单 | `shadow-dropdown` | 浮层效果 |
| 模态框 | `shadow-modal` | 强投影 |
| 拖拽区域 | `shadow-focus` | 聚焦高亮 |

---

## 6. 圆角系统

### 6.1 圆角大小

```css
:root {
  --radius-none: 0;
  --radius-sm: 0.25rem;    /* 4px */
  --radius-md: 0.5rem;     /* 8px */
  --radius-lg: 0.75rem;    /* 12px */
  --radius-xl: 1rem;       /* 16px */
  --radius-2xl: 1.5rem;    /* 24px */
  --radius-full: 9999px;   /* 圆形 */
}
```

### 6.2 组件圆角规范

| 组件 | 圆角 | 说明 |
|------|------|------|
| 按钮 | `radius-md` | 8px |
| 输入框 | `radius-md` | 8px |
| 卡片 | `radius-lg` | 12px |
| 头像 | `radius-full` | 圆形 |
| 标签 | `radius-sm` | 4px |
| 弹窗 | `radius-xl` | 16px |

---

## 7. 动画系统

### 7.1 过渡效果

```css
:root {
  /* 过渡时长 */
  --duration-fast: 150ms;
  --duration-normal: 200ms;
  --duration-slow: 300ms;

  /* 过渡曲线 */
  --ease-default: cubic-bezier(0.4, 0, 0.2, 1);
  --ease-in: cubic-bezier(0.4, 0, 1, 1);
  --ease-out: cubic-bezier(0, 0, 0.2, 1);
  --ease-bounce: cubic-bezier(0.68, -0.55, 0.265, 1.55);
}
```

### 7.2 常用动画

```css
/* 淡入 */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* 上浮 */
@keyframes slideUp {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

/* 缩放 */
@keyframes scaleIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

/* 旋转 */
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 脉冲 */
@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

### 7.3 动画使用场景

| 场景 | 动画 | 时长 |
|------|------|------|
| 按钮悬停 | `transform: scale(1.02)` | 150ms |
| 卡片悬停 | `transform: translateY(-2px)` + `shadow-lg` | 200ms |
| 加载中 | `spin` 旋转 | 1s 循环 |
| 进度条 | `width` 变化 | 500ms |
| Toast出现 | `slideUp` + `fadeIn` | 300ms |
| 页面切换 | `fadeIn` | 200ms |

---

## 8. 组件设计

### 8.1 按钮 (Button)

```tsx
// Props
interface ButtonProps {
  variant: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  size: 'sm' | 'md' | 'lg';
  isLoading?: boolean;
  disabled?: boolean;
  fullWidth?: boolean;
}

// 样式
.primary { background: var(--color-primary-600); color: white; }
.primary:hover { background: var(--color-primary-700); }
.primary:active { background: var(--color-primary-800); }

.outline { border: 2px solid var(--color-border); background: transparent; }
.outline:hover { background: var(--color-bg-tertiary); }
```

### 8.2 输入框 (Input)

```tsx
interface InputProps {
  type: 'text' | 'password' | 'email';
  placeholder?: string;
  error?: string;
  disabled?: boolean;
}

// 样式
.input {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: border-color 200ms, box-shadow 200ms;
}

.input:focus {
  outline: none;
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-focus);
}

.input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

### 8.3 选择器 (Select)

```tsx
interface SelectProps {
  options: { value: string; label: string; icon?: ReactNode }[];
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  searchable?: boolean;
}

// 下拉选项样式
.dropdown {
  position: absolute;
  z-index: 50;
  width: 100%;
  margin-top: 0.5rem;
  background: white;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-dropdown);
}
```

### 8.4 卡片 (Card)

```tsx
interface CardProps {
  padding?: 'none' | 'sm' | 'md' | 'lg';
  hoverable?: boolean;
  clickable?: boolean;
}

// 样式
.card {
  background: var(--color-bg-secondary);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
}

.card.hoverable:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}
```

### 8.5 上传区域 (Upload)

```tsx
interface UploadProps {
  accept?: string;        // "image/png,image/jpeg"
  maxSize?: number;        // 10MB
  multiple?: boolean;
  onUpload: (files: File[]) => void;
}

// 拖拽区域样式
.upload-zone {
  border: 2px dashed var(--color-border);
  border-radius: var(--radius-lg);
  padding: 2rem;
  text-align: center;
  transition: all 200ms;
}

.upload-zone.active {
  border-color: var(--color-primary-500);
  background: var(--color-primary-50);
}
```

### 8.6 代码编辑器 (CodeEditor)

```tsx
interface CodeEditorProps {
  code: string;
  language: string;
  readOnly?: boolean;
  showLineNumbers?: boolean;
  showToolbar?: boolean;
  height?: string;
}

// Monaco Editor 配置
{
  theme: 'vs-dark',
  minimap: { enabled: false },
  fontSize: 14,
  fontFamily: 'JetBrains Mono',
  lineNumbers: 'on',
  scrollBeyondLastLine: false,
  automaticLayout: true,
}
```

---

## 9. 响应式布局

### 9.1 断点系统

```css
:root {
  /* 移动优先断点 */
  --breakpoint-sm: 640px;   /* 小平板 */
  --breakpoint-md: 768px;   /* 平板 */
  --breakpoint-lg: 1024px;  /* 桌面 */
  --breakpoint-xl: 1280px;  /* 大桌面 */
  --breakpoint-2xl: 1536px; /* 超大桌面 */
}
```

### 9.2 布局规范

| 断点 | 屏幕宽度 | 布局 |
|------|---------|------|
| mobile | < 640px | 单列，底部导航 |
| tablet | 640-1024px | 双栏，可折叠侧边栏 |
| desktop | > 1024px | 多栏，固定侧边栏 |

### 9.3 页面最大宽度

```css
.container {
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 1rem;
}

@media (min-width: 640px) {
  .container { padding: 0 2rem; }
}

@media (min-width: 1024px) {
  .container { padding: 0 3rem; }
}
```

---

## 10. 深色模式

### 10.1 深色模式色彩

```css
.dark {
  --color-bg-primary: #111827;
  --color-bg-secondary: #1f2937;
  --color-bg-tertiary: #374151;

  --color-text-primary: #f9fafb;
  --color-text-secondary: #d1d5db;
  --color-text-tertiary: #9ca3af;

  --color-border: #374151;
}
```

### 10.2 切换逻辑

```tsx
// 使用 CSS 变量 + class 切换
// 或使用 media query 自动检测
@media (prefers-color-scheme: dark) {
  :root { /* 深色变量 */ }
}
```

---

## 11. 可访问性 (A11y)

### 11.1 键盘导航

- 所有交互元素可通过 Tab 键聚焦
- 聚焦元素有明显的视觉指示
- 支持快捷键操作

### 11.2 屏幕阅读器

- 所有图片有 alt 描述
- 表单元素有 associated label
- 动态内容有 aria-live 提示

### 11.3 对比度

- 文字与背景对比度 ≥ 4.5:1
- 大文字对比度 ≥ 3:1
- 按钮边框与背景对比度 ≥ 3:1

---

## 12. UI组件清单

### 已实现

| 组件 | 路径 | 状态 |
|------|------|------|
| Button | `components/common/Button.tsx` | ✅ |
| Input | `components/common/Input.tsx` | ✅ |
| Select | `components/common/Select.tsx` | ✅ |
| Card | `components/common/Card.tsx` | ✅ |
| Modal | `components/common/Modal.tsx` | ✅ |
| Spinner | `components/common/Spinner.tsx` | ✅ |
| PageLoader | `components/common/PageLoader.tsx` | ✅ |
| ImageUpload | `components/upload/ImageUpload.tsx` | ✅ |
| CodeEditor | `components/editor/CodeEditor.tsx` | ✅ |
| Header | `components/layout/Header.tsx` | ✅ |
| Footer | `components/layout/Footer.tsx` | ✅ |
| Layout | `components/layout/Layout.tsx` | ✅ |

### 需要补充

| 组件 | 优先级 | 说明 |
|------|--------|------|
| CodePreview | P1 | 代码预览/实时预览组件 |
| ProgressBar | P1 | 进度条组件 |
| Toast | P1 | 提示组件 (已用sonner) |
| Dropdown | P1 | 下拉菜单组件 |
| Tabs | P2 | 标签页组件 |
| Tooltip | P2 | 提示文字组件 |
| Badge | P2 | 标签组件 |
| Avatar | P2 | 头像组件 |
| Skeleton | P2 | 骨架屏组件 |

---

## 13. 设计资源

### 13.1 图标库

使用 **Lucide React** 图标库:
- 常用图标: Sparkles, Code2, Copy, Download, Upload, Settings, History, etc.
- 品牌图标: (需要设计)

### 13.2 图片资源

- Logo: SVG 格式，支持深浅色模式
- 空状态插图: SVG 格式
- 加载动画: Lottie 或 CSS 动画

### 13.3 设计工具

- Figma: UI设计稿
- Tailwind CSS: 样式实现
- CSS Variables: 主题配置

---

## 14. 页面设计

### 14.1 首页 (Home Page)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  [Logo: AI Screen Code]                    [历史] [设置] [登录/头像]  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                     AI Screen Code                                      │
│              上传UI设计图片，AI自动生成前端代码                           │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                                                                   │  │
│  │                    拖拽图片到此处                                  │  │
│  │                    或点击选择文件                                  │  │
│  │                                                                   │  │
│  │              [图标: CloudUpload]                                  │  │
│  │                                                                   │  │
│  │              支持 PNG, JPG, WebP  |  最大 10MB                    │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌──────────────────────┐  ┌──────────────────────┐                   │
│  │  选择 AI 模型        │  │  选择输出语言        │                   │
│  │  [Qwen (阿里云) ▼]  │  │  [React + TS ▼]     │                   │
│  └──────────────────────┘  └──────────────────────┘                   │
│                                                                         │
│                    [✨ 生成代码]                                        │
│                                                                         │
│  ┌──────────────────────┐  ┌──────────────────────┐                   │
│  │  🚀 多种模型         │  │  💻 多语言输出        │                   │
│  │  支持主流AI模型      │  │  React/Vue/Swift     │                   │
│  └──────────────────────┘  └──────────────────────┘                   │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│                           © 2024 AI Screen Code                        │
└─────────────────────────────────────────────────────────────────────────┘
```

### 14.2 生成页面 (Generate Page)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  [← 返回]                     生成代码                    [上传新图片]  │
├─────────────────────┬───────────────────────────────────────────────────┤
│                     │                                                   │
│  当前配置            │                                                   │
│  ┌───────────────┐  │  ┌─────────────────────────────────────────┐   │
│  │ 🤖 Qwen       │  │  │  1  │ import React from 'react';       │   │
│  │               │  │  │  2  │                                    │   │
│  │ 💻 React      │  │  │  3  │ const App = () => {               │   │
│  └───────────────┘  │  │  4  │   return (                       │   │
│                     │  │  5  │     <div className="container">   │   │
│  生成进度            │  │  6  │       ...                        │   │
│  ┌───────────────┐  │  │  ... │                                    │   │
│  │     ●●●●      │  │  │  50  │     </div>                       │   │
│  │   加载→分析    │  │  │  51  │   );                              │   │
│  │   →生成→完成   │  │  │  52  │ };                               │   │
│  │               │  │  │                                         │   │
│  │  ████████░░  │  │  └─────────────────────────────────────────┘   │
│  │     85%       │  │                                                   │
│  └───────────────┘  │  [📋 复制]  [💾 下载]  [⛶ 全屏]              │
│                     │                                                   │
│  [📋 复制代码]      │                                                   │
│  [💾 下载代码]      │                                                   │
│                     │                                                   │
└─────────────────────┴───────────────────────────────────────────────────┘
```

### 14.3 历史记录页面 (History Page)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  [Logo]                              [首页] [设置] [头像 ▼]            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  历史记录                                    [+ 新建]  [🔍 搜索...]     │
│                                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │ ┌─────────┐ │  │ ┌─────────┐ │  │ ┌─────────┐ │  │ ┌─────────┐ │   │
│  │ │         │ │  │ │         │ │  │ │         │ │  │ │         │ │   │
│  │ │  图片   │ │  │ │  图片   │ │  │ │  图片   │ │  │ │  图片   │ │   │
│  │ │         │ │  │ │         │ │  │ │         │ │  │ │         │ │   │
│  │ └─────────┘ │  │ └─────────┘ │  │ └─────────┘ │  │ └─────────┘ │   │
│  │             │  │             │  │             │  │             │   │
│  │ [Qwen] React│  │ [Kimi] Vue │  │ [GLM] Kotlin│  │ [MiniMax]   │   │
│  │             │  │             │  │             │  │  Swift      │   │
│  │ 2小时前     │  │ 昨天       │  │ 3天前       │  │ 1周前       │   │
│  │             │  │             │  │             │  │             │   │
│  │ [查看][删除] │  │ [查看][删除]│  │ [查看][删除]│  │ [查看][删除]│   │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
│                                                                         │
│                        [< 1 2 3 ... 10 >]                              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 14.4 设置页面 (Settings Page)

```
┌─────────────────────────────────────────────────────────────────────────┐
│  [Logo]                              [首页] [历史] [头像 ▼]            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  设置                                                                  │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  🔑 API Key 配置                                               │   │
│  │                                                                 │   │
│  │  配置各大AI模型的API Key，信息保存在浏览器本地，不会上传。       │   │
│  │                                                                 │   │
│  │  Qwen (阿里云)                           ✓ 已验证               │   │
│  │  ┌─────────────────────────────────────────────────────────┐   │   │
│  │  │ ••••••••••••••••••••••••••••                    [验证] │   │   │
│  │  └─────────────────────────────────────────────────────────┘   │   │
│  │  自定义 Base URL (可选):                                        │   │
│  │  ┌─────────────────────────────────────────────────────────┐   │   │
│  │  │                                                         │   │   │
│  │  └─────────────────────────────────────────────────────────┘   │   │
│  │                                                                 │   │
│  │  ─────────────────────────────────────────────────────────     │   │
│  │                                                                 │   │
│  │  MiniMax                                     ✗ 验证失败         │   │
│  │  ┌─────────────────────────────────────────────────────────┐   │   │
│  │  │                                                         │   │   │
│  │  └─────────────────────────────────────────────────────────┘   │   │
│  │  ...                                                          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  📖 使用说明                                                   │   │
│  │  • 在各AI平台申请API Key并填写到对应配置中                     │   │
│  │  • 点击"验证"按钮确认API Key是否有效                            │   │
│  │  • 如使用代理或自定义端点，可填写自定义Base URL                 │   │
│  │  • API Key仅保存在浏览器本地，不会发送给任何服务器              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```
