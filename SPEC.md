# AI Screen Code - 技术架构优化方案

## 1. 当前状态分析

### 1.1 项目结构概览

```
ai_screen_code/
├── backend/                    # Rust + Axum 后端
│   ├── src/
│   │   ├── handlers/          # 请求处理器
│   │   │   ├── auth.rs       # 认证相关
│   │   │   ├── generate.rs   # 代码生成
│   │   │   ├── history.rs   # 历史记录
│   │   │   ├── models.rs     # 模型配置
│   │   │   ├── settings.rs   # 用户设置
│   │   │   └── subscription.rs # 订阅管理
│   │   ├── services/         # 业务逻辑层
│   │   │   ├── ai_service.rs # AI服务接口
│   │   │   ├── qwen_service.rs
│   │   │   ├── minimax_service.rs
│   │   │   ├── kimi_service.rs
│   │   │   ├── glm_service.rs
│   │   │   └── history_service.rs
│   │   ├── models/           # 数据模型
│   │   ├── middleware.rs    # 中间件
│   │   ├── state.rs         # 应用状态
│   │   ├── config.rs        # 配置管理
│   │   ├── database.rs      # 数据库
│   │   └── error.rs         # 错误处理
│   └── Cargo.toml
│
└── frontend/                  # React 19 + TypeScript 前端
    ├── src/
    │   ├── api/             # API客户端
    │   ├── components/      # 组件
    │   │   ├── common/      # 通用组件
    │   │   ├── layout/      # 布局组件
    │   │   ├── upload/      # 上传组件
    │   │   └── editor/      # 编辑器组件
    │   ├── pages/           # 页面
    │   ├── stores/          # 状态管理
    │   ├── types/           # 类型定义
    │   ├── utils/           # 工具函数
    │   ├── router.tsx       # 路由配置
    │   └── main.tsx         # 入口
    └── package.json
```

### 1.2 当前技术栈

| 层级 | 技术选型 |
|------|---------|
| 后端框架 | Rust + Axum |
| 异步运行时 | Tokio |
| HTTP客户端 | Reqwest |
| 前端框架 | React 19 + TypeScript |
| UI样式 | Tailwind CSS |
| 构建工具 | Vite |
| 桌面框架 | Tauri 2.x |
| 状态管理 | Zustand |
| 数据获取 | TanStack Query |
| HTTP客户端 | Axios |

---

## 2. 架构优化方案

### 2.1 前后端模块划分

#### 后端模块职责

```
┌─────────────────────────────────────────────────────────────────┐
│                        Backend Modules                          │
├─────────────────────────────────────────────────────────────────┤
│  api/                    # API 路由层                           │
│  ├── mod.rs              # 路由聚合                             │
│  ├── v1/                 # API v1 版本                          │
│  │   ├── mod.rs                                                  │
│  │   ├── generate.rs    # 代码生成接口                          │
│  │   ├── history.rs     # 历史记录接口                          │
│  │   ├── models.rs      # 模型管理接口                          │
│  │   ├── settings.rs    # 设置接口                              │
│  │   └── auth.rs        # 认证接口                              │
│  └── webhooks.rs        # Webhook 处理                          │
├─────────────────────────────────────────────────────────────────┤
│  core/                   # 核心业务逻辑                          │
│  ├── mod.rs                                                      │
│  ├── generate/           # 代码生成核心                          │
│  │   ├── mod.rs                                                  │
│  │   ├── prompt.rs      # Prompt 构建                           │
│  │   ├── parser.rs      # 代码解析                               │
│  │   └── validator.rs   # 输出验证                               │
│  └── history/            # 历史记录核心                          │
│      ├── mod.rs                                                  │
│      └── manager.rs                                              │
├─────────────────────────────────────────────────────────────────┤
│  services/               # 外部服务集成                          │
│  ├── mod.rs                                                      │
│  ├── ai/                 # AI 模型服务                           │
│  │   ├── mod.rs                                                  │
│  │   ├── trait.rs      # 服务接口定义                           │
│  │   ├── provider.rs   # 提供商路由                             │
│  │   ├── qwen.rs                                                │
│  │   ├── minimax.rs                                             │
│  │   ├── kimi.rs                                                │
│  │   └── glm.rs                                                │
│  ├── storage/            # 存储服务                              │
│  │   ├── mod.rs                                                  │
│  │   └── image.rs     # 图片存储                                │
│  └── cache/              # 缓存服务                              │
│      └── mod.rs                                                  │
├─────────────────────────────────────────────────────────────────┤
│  domain/                 # 领域模型                              │
│  ├── mod.rs                                                      │
│  ├── user/               # 用户领域                              │
│  │   ├── mod.rs                                                  │
│  │   ├── entity.rs   # 用户实体                                 │
│  │   ├── repository.rs # 仓储接口                               │
│  │   └── service.rs  # 领域服务                                 │
│  ├── project/            # 项目领域                              │
│  │   ├── mod.rs                                                  │
│  │   ├── entity.rs   # 项目实体                                 │
│  │   └── repository.rs                                         │
│  └── generation/         # 代码生成领域                          │
│      ├── mod.rs                                                  │
│      ├── entity.rs   # 生成记录实体                             │
│      └── repository.rs                                         │
├─────────────────────────────────────────────────────────────────┤
│  infrastructure/         # 基础设施                              │
│  ├── mod.rs                                                      │
│  ├── database/           # 数据库                                │
│  │   ├── mod.rs                                                  │
│  │   ├── connection.rs                                         │
│  │   └── migrations/                                           │
│  ├── config/             # 配置                                  │
│  │   └── mod.rs                                                  │
│  └── logging/            # 日志                                  │
│      └── mod.rs                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### 前端模块职责

```
┌─────────────────────────────────────────────────────────────────┐
│                       Frontend Modules                          │
├─────────────────────────────────────────────────────────────────┤
│  features/               # 功能模块 (按功能划分)                │
│  ├── generate/           # 代码生成功能                         │
│  │   ├── components/     # 功能组件                             │
│  │   │   ├── ImageUploader.tsx                                 │
│  │   │   ├── ModelSelector.tsx                                 │
│  │   │   ├── LanguageSelector.tsx                              │
│  │   │   ├── CodePreview.tsx                                   │
│  │   │   └── PreviewPanel.tsx                                  │
│  │   ├── hooks/          # 功能 hooks                           │
│  │   │   ├── useGenerate.ts                                    │
│  │   │   └── useStreaming.ts                                   │
│  │   └── types.ts        # 功能类型                             │
│  ├── history/            # 历史记录功能                         │
│  │   ├── components/                                              │
│  │   ├── hooks/                                                    │
│  │   └── types.ts                                                │
│  ├── settings/           # 设置功能                              │
│  │   ├── components/                                              │
│  │   ├── hooks/                                                    │
│  │   └── types.ts                                                │
│  └── auth/               # 认证功能                              │
│      ├── components/                                              │
│      ├── hooks/                                                    │
│      └── types.ts                                                │
├─────────────────────────────────────────────────────────────────┤
│  shared/                 # 共享资源                              │
│  ├── api/               # API 客户端                           │
│  │   ├── client.ts      # Axios 实例                           │
│  │   ├── endpoints.ts   # 端点定义                             │
│  │   └── types.ts       # API 类型                             │
│  ├── components/        # 共享组件                              │
│  │   ├── ui/            # 基础 UI 组件                          │
│  │   │   ├── Button/                                             │
│  │   │   ├── Input/                                              │
│  │   │   ├── Select/                                             │
│  │   │   ├── Modal/                                              │
│  │   │   └── ...                                                │
│  │   └── layout/        # 布局组件                              │
│  │       ├── Header/                                             │
│  │       ├── Footer/                                             │
│  │       └── Layout/                                             │
│  ├── hooks/             # 共享 hooks                            │
│  │   ├── useDebounce.ts                                        │
│  │   ├── useLocalStorage.ts                                     │
│  │   └── usePrevious.ts                                         │
│  ├── stores/            # 共享状态                              │
│  │   └── appStore.ts                                           │
│  ├── utils/             # 工具函数                             │
│  │   ├── format.ts                                             │
│  │   ├── validation.ts                                         │
│  │   └── helpers.ts                                            │
│  └── constants/         # 常量定义                              │
│      ├── models.ts      # 模型配置                             │
│      └── languages.ts   # 语言配置                             │
├─────────────────────────────────────────────────────────────────┤
│  app/                    # 应用入口                              │
│  ├── App.tsx                                                      │
│  ├── router.tsx         # 路由配置                              │
│  └── providers.tsx     # Context Providers                     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 数据流设计

```
┌─────────────────────────────────────────────────────────────────┐
│                        Data Flow                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐                │
│  │   UI     │────▶│  Store   │────▶│   API    │                │
│  │  Events  │     │  (Zustand)│     │  Client  │                │
│  └──────────┘     └──────────┘     └────┬─────┘                │
│       │              │                   │                      │
│       ▼              ▼                   ▼                      │
│  ┌──────────────────────────────────────────┐                │
│  │           React Query Cache              │                │
│  │    (Server State Management)              │                │
│  └──────────────────────────────────────────┘                │
│                        │                                        │
│                        ▼                                        │
│  ┌──────────────────────────────────────────┐                │
│  │              HTTP Request                │                │
│  └──────────────────┬───────────────────────┘                │
│                     │                                            │
│                     ▼                                            │
│  ┌──────────────────────────────────────────┐                │
│  │           Backend API Layer              │                │
│  │  ┌────────────┐  ┌───────────────────┐  │                │
│  │  │ Validation │  │ Rate Limiting     │  │                │
│  │  └────────────┘  └───────────────────┘  │                │
│  └──────────────────┬───────────────────────┘                │
│                     │                                            │
│                     ▼                                            │
│  ┌──────────────────────────────────────────┐                │
│  │           Business Logic Layer          │                │
│  │  ┌────────────┐  ┌───────────────────┐  │                │
│  │  │   Core     │  │   Services        │  │                │
│  │  │  Generate  │  │  (AI Providers)  │  │                │
│  │  └────────────┘  └───────────────────┘  │                │
│  └──────────────────┬───────────────────────┘                │
│                     │                                            │
│                     ▼                                            │
│  ┌──────────────────────────────────────────┐                │
│  │         Infrastructure Layer             │                │
│  │  ┌────────────┐  ┌───────────────────┐  │                │
│  │  │  Database  │  │   File Storage    │  │                │
│  │  └────────────┘  └───────────────────┘  │                │
│  └──────────────────────────────────────────┘                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### 数据流向说明

1. **用户交互层 (UI)**
   - React 组件处理用户输入
   - 调用 Zustand actions 或自定义 hooks

2. **状态管理层**
   - **客户端状态** (Zustand): 用户配置、UI状态
   - **服务端状态** (React Query): API 缓存、历史记录

3. **API 请求层**
   - Axios 客户端统一处理请求/响应
   - 自动添加认证 Token
   - 统一错误处理

4. **后端处理层**
   - 验证请求 → 业务逻辑 → 外部服务调用 → 数据持久化

5. **响应返回层**
   - 序列化响应 → 错误处理 → 返回结果

### 2.3 API 接口规范

#### RESTful API 设计

```
┌─────────────────────────────────────────────────────────────────┐
│                      API Versioning                             │
├─────────────────────────────────────────────────────────────────┤
│  Base URL: /api/v1                                              │
│                                                                  │
│  Resources:                                                     │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  POST   /generate          代码生成                      │  │
│  │  GET    /generate/status   生成状态查询                  │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │  GET    /history            历史列表                      │  │
│  │  GET    /history/:id       历史详情                      │  │
│  │  DELETE /history/:id       删除历史                      │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │  GET    /models             可用模型列表                  │  │
│  │  POST   /models/validate   验证 API Key                  │  │
│  │  GET    /models/:name      模型配置                       │  │
│  │  PUT    /models/:name      更新模型配置                  │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │  GET    /settings          用户设置                      │  │
│  │  PUT    /settings          更新设置                      │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │  POST   /auth/register     注册                          │  │
│  │  POST   /auth/login        登录                          │  │
│  │  GET    /auth/profile     用户信息                       │  │
│  │  PUT    /auth/profile     更新信息                       │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │  GET    /subscription/plans    订阅计划                  │  │
│  │  POST   /subscription/create   创建订单                  │  │
│  │  GET    /subscription/status   订阅状态                  │  │
│  │  POST   /subscription/webhook  支付回调                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

#### 统一响应格式

```typescript
// 成功响应
interface ApiSuccess<T> {
  success: true
  data: T
  meta?: {
    page?: number
    limit?: number
    total?: number
  }
}

// 错误响应
interface ApiError {
  success: false
  error: {
    code: string
    message: string
    details?: Record<string, string[]>
  }
}

// 联合类型
type ApiResponse<T> = ApiSuccess<T> | ApiError

// 示例响应
// GET /api/v1/history
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "image": "base64...",
      "code": "...",
      "language": "react",
      "model": "qwen",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 100
  }
}
```

#### 请求/响应类型定义

```typescript
// ============ Code Generation ============

// 生成请求
interface GenerateRequest {
  image: string          // Base64 编码的图片
  model: AIModel         // 'qwen' | 'minimax' | 'kimi' | 'glm'
  language: Language     // 'react' | 'vue' | 'kotlin' | 'swift'
  api_key?: string       // 可选的 API Key (优先使用)
  base_url?: string      // 可选的自定义端点
  stream?: boolean       // 是否启用流式响应
}

// 生成响应
interface GenerateResponse {
  id: string             // 生成记录 ID
  code: string           // 生成的代码
  language: Language     // 代码语言
  model: AIModel          // 使用的模型
  tokens_used?: number    // 消耗的 token 数量
  duration_ms?: number    // 生成耗时(毫秒)
}

// 流式生成事件
interface GenerateStreamEvent {
  type: 'start' | 'chunk' | 'done' | 'error'
  data?: string
  progress?: number
  error?: string
}

// ============ History ============

// 历史记录项
interface HistoryItem {
  id: string
  image: string          // Base64 或 URL
  code: string
  language: Language
  model: AIModel
  created_at: string     // ISO 8601
}

// ============ Models ============

// AI 模型信息
interface ModelInfo {
  name: string           // 模型标识
  display_name: string   // 显示名称
  provider: string       // 提供商
  supports_languages: Language[]
  max_image_size: number // 最大图片大小 (MB)
  pricing: {
    input: number        // 输入价格 (per 1K tokens)
    output: number       // 输出价格
  }
}

// 模型验证请求
interface ValidateModelRequest {
  model: AIModel
  api_key: string
  base_url?: string
}

// 模型验证响应
interface ValidateModelResponse {
  valid: boolean
  message: string
  quota_info?: {
    remaining: number
    total: number
  }
}
```

### 2.4 状态管理方案

```
┌─────────────────────────────────────────────────────────────────┐
│                    State Management Architecture                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Server State (React Query)                 │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │   │
│  │  │  /history   │  │  /models    │  │ /subscription│     │   │
│  │  │  - caching  │  │  - caching  │  │  - polling  │     │   │
│  │  │  - invalidation │ - prefetch │  │             │     │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘     │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Client State (Zustand)                     │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │
│  │  │              useAppStore                         │   │   │
│  │  │  ┌───────────────────────────────────────────┐  │   │   │
│  │  │  │ User Config                              │  │   │   │
│  │  │  │ - defaultModel                           │  │   │   │
│  │  │  │ - defaultLanguage                        │  │   │   │
│  │  │  │ - apiKeys[model]                        │  │   │   │
│  │  │  │ - customBaseUrls[model]                │  │   │   │
│  │  │  └───────────────────────────────────────────┘  │   │   │
│  │  │  ┌───────────────────────────────────────────┐  │   │   │
│  │  │  │ Generation State                          │  │   │   │
│  │  │  │ - currentFile                             │  │   │   │
│  │  │  │ - selectedModel                           │  │   │   │
│  │  │  │ - selectedLanguage                        │  │   │   │
│  │  │  │ - isGenerating                            │  │   │   │
│  │  │  │ - generateResult                          │  │   │   │
│  │  │  └───────────────────────────────────────────┘  │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### Zustand Store 结构

```typescript
// stores/appStore.ts

interface AppState {
  // ========== 用户配置 (持久化到 localStorage) ==========
  config: UserConfig
  setConfig: (config: Partial<UserConfig>) => void
  clearConfig: () => void

  // ========== 生成状态 (临时状态) ==========
  currentFile: UploadedFile | null
  selectedModel: AIModel
  selectedLanguage: Language
  isGenerating: boolean
  generationProgress: number
  generatedCode: string | null
  error: string | null

  // Actions
  setCurrentFile: (file: UploadedFile | null) => void
  setSelectedModel: (model: AIModel) => void
  setSelectedLanguage: (language: Language) => void
  startGeneration: () => void
  updateProgress: (progress: number) => void
  setGeneratedCode: (code: string) => void
  setError: (error: string) => void
  resetGeneration: () => void

  // ========== UI 状态 (临时状态) ==========
  sidebarOpen: boolean
  theme: 'light' | 'dark' | 'system'
  setSidebarOpen: (open: boolean) => void
  setTheme: (theme: 'light' | 'dark' | 'system') => void
}
```

#### React Query 配置

```typescript
// lib/queryClient.ts
import { QueryClient } from '@tanstack/react-query'

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 分钟
      gcTime: 30 * 60 * 1000,   // 30 分钟
      retry: 1,
      refetchOnWindowFocus: false,
    },
    mutations: {
      retry: 0,
    },
  },
})

// 预定义的查询键
export const queryKeys = {
  history: ['history'] as const,
  historyItem: (id: string) => ['history', id] as const,
  models: ['models'] as const,
  modelConfig: (name: string) => ['models', name] as const,
  settings: ['settings'] as const,
  subscription: ['subscription'] as const,
  subscriptionPlans: ['subscription', 'plans'] as const,
}
```

### 2.5 性能优化策略

#### 前端性能优化

```
┌─────────────────────────────────────────────────────────────────┐
│                    Frontend Performance                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. 代码分割 (Code Splitting)                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - React.lazy() 动态加载页面组件                        │   │
│  │  - 路由级别分割                                          │   │
│  │  - 组件级别分割 (Monaco Editor 等重型组件)              │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  2. 图片优化                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - 前端压缩 (compressorjs)                              │   │
│  │  - 渐进式加载                                            │   │
│  │  - 缩略图预览                                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  3. 缓存策略                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - React Query 缓存历史记录                             │   │
│  │  - Zustand 持久化用户配置                               │   │
│  │  - Service Worker 离线缓存 (PWA)                        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  4. 流式处理                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - SSE (Server-Sent Events) 接收流式代码                │   │
│  │  - 增量渲染生成的代码                                     │   │
│  │  - 取消生成请求能力                                       │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### 后端性能优化

```
┌─────────────────────────────────────────────────────────────────┐
│                     Backend Performance                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. 并发处理                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - Tokio 异步运行时处理并发请求                          │   │
│  │  - 连接池管理 (SQLx)                                     │   │
│  │  - API 请求限流 (Tower RateLimit)                       │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  2. 缓存层                                                      │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - Redis 缓存模型配置                                    │   │
│  │  - 缓存热门历史记录                                     │   │
│  │  - 缓存验证结果                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  3. 图片处理                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - Base64 编解码优化                                    │   │
│  │  - 图片尺寸验证                                          │   │
│  │  - 临时文件自动清理                                      │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  4. 资源管理                                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  - 请求超时控制                                          │   │
│  │  - 大文件上传分片                                        │   │
│  │  - 内存池优化                                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### 实施计划

| 优化项 | 优先级 | 预期效果 |
|-------|--------|---------|
| 代码分割 | P0 | 首次加载减少 40% |
| 图片压缩 | P0 | 上传速度提升 60% |
| React Query 缓存 | P1 | 重复请求减少 80% |
| SSE 流式响应 | P1 | 首字节时间减少 50% |
| 后端限流 | P1 | 防止 API 滥用 |
| Redis 缓存 | P2 | 查询性能提升 70% |

---

## 3. 架构决策记录 (ADRs)

### ADR-001: 前端模块按功能划分

**状态**: 提议

**背景**:
当前前端代码按类型组织 (components/, pages/, stores/), 导致相关代码分散。

**决策**:
采用 Feature-Sliced Design 思想的简化版，按功能模块组织代码。

**后果**:
- 优点: 高内聚、低耦合、易测试
- 缺点: 需要迁移现有代码

### ADR-002: 状态管理分离

**状态**: 提议

**背景**:
当前所有状态都在 Zustand 中管理。

**决策**:
- 客户端状态 (UI、配置): Zustand
- 服务端状态 (API 数据): React Query

**后果**:
- 优点: 职责分离、更好的缓存策略
- 缺点: 需要学习两个库

### ADR-003: API 响应格式统一

**状态**: 提议

**背景**:
当前 API 响应格式不统一。

**决策**:
采用 `{ success: boolean, data?: T, error?: Error }` 格式。

**后果**:
- 优点: 统一错误处理、易于类型推断
- 缺点: 需要修改现有响应

---

## 4. 下一步行动

### 立即执行 (1-2周)
- [ ] 重构前端目录结构 (按功能模块)
- [ ] 统一 API 响应格式
- [ ] 配置 React Query

### 短期规划 (1个月)
- [ ] 实现 SSE 流式代码生成
- [ ] 添加前端图片压缩
- [ ] 实现请求取消功能

### 中期规划 (3个月)
- [ ] 添加 Redis 缓存层
- [ ] 实现 PWA 支持
- [ ] 添加单元测试 (80% 覆盖率)

---

*文档版本: 1.0*
*最后更新: 2024-01-01*
