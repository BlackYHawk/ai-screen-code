# AI Screen Code 技术架构文档

## 1. 系统架构概述

### 1.1 整体架构
```
┌─────────────────────────────────────────────────────────────────┐
│                        客户端层                                  │
│  ┌─────────────────┐              ┌─────────────────┐          │
│  │   Web 前端      │              │   Tauri 桌面    │          │
│  │  (React + Vite) │              │     应用         │          │
│  └────────┬────────┘              └────────┬────────┘          │
└───────────┼──────────────────────────────┼────────────────────┘
            │                              │
            ▼                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        API 网关层 (Axum)                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Auth | Generate | History | Subscription | Settings     │  │
│  └──────────────────────────────────────────────────────────┘  │
└───────────┬──────────────────────────────┼────────────────────┘
            │                              │
            ▼                              ▼
┌─────────────────────────┐    ┌─────────────────────────────────┐
│    AI 模型服务层        │    │        数据存储层               │
│  ┌─────────────────┐   │    │  ┌─────────┐  ┌─────────────┐  │
│  │ Qwen            │   │    │  │ SQLite  │  │ 本地文件    │  │
│  │ MiniMax         │   │    │  │ (用户/  │  │ (配置/图片) │  │
│  │ Kimi            │   │    │  │ 订单)   │  │             │  │
│  │ GLM             │   │    │  └─────────┘  └─────────────┘  │
│  └─────────────────┘   │    └─────────────────────────────────┘
└─────────────────────────┘
```

### 1.2 技术栈

| 层级 | 技术 | 版本 |
|------|------|------|
| 后端框架 | Axum | 0.7 |
| 异步运行时 | Tokio | 1.x |
| HTTP客户端 | Reqwest | 0.12 |
| 数据库 | SQLite (rusqlite) | 0.31 |
| 前端框架 | React | 19.x |
| 构建工具 | Vite | 7.x |
| 桌面框架 | Tauri | 2.x |
| UI库 | Tailwind CSS | 3.x |

## 2. 后端架构

### 2.1 模块划分

```
backend/src/
├── main.rs              # 入口 & 路由配置
├── lib.rs               # 库导出
├── config.rs            # 配置管理
├── state.rs             # 应用状态
├── error.rs             # 错误定义
├── database.rs          # 数据库层
├── middleware.rs        # 中间件
│
├── handlers/            # API处理器
│   ├── mod.rs
│   ├── auth.rs          # 认证
│   ├── generate.rs      # 代码生成
│   ├── history.rs       # 历史记录
│   ├── settings.rs      # 设置
│   ├── models.rs        # 模型配置
│   └── subscription.rs  # 订阅支付
│
├── services/            # 业务逻辑
│   ├── mod.rs
│   ├── ai_service.rs    # AI服务接口
│   ├── qwen_service.rs  # 阿里云通义
│   ├── minimax_service.rs
│   ├── kimi_service.rs  # 月之暗面
│   ├── glm_service.rs   # 智谱AI
│   └── history_service.rs
│
└── models/              # 数据模型
    ├── mod.rs
    ├── user.rs
    ├── request.rs
    ├── response.rs
    ├── history.rs
    └── subscription.rs
```

### 2.2 API 设计

#### 公开API (无需认证)
```
GET  /health                      - 健康检查
POST /api/v1/auth/register        - 用户注册
POST /api/v1/auth/login          - 用户登录
POST /api/v1/generate            - 代码生成
POST /api/v1/generate/stream     - 流式代码生成
GET  /api/v1/models              - 获取模型列表
POST /api/v1/models/validate     - 验证API Key
GET  /api/v1/history             - 获取历史记录
GET  /api/v1/history/:id         - 获取单条记录
GET  /api/v1/settings            - 获取设置
POST /api/v1/settings            - 更新设置
GET  /api/v1/subscriptions/plans - 获取订阅计划
POST /api/v1/subscriptions/create - 创建订单
GET  /api/v1/subscriptions/status - 获取订阅状态
POST /api/v1/subscriptions/webhook - 支付回调
```

#### 私有API (需要认证)
```
GET  /api/v1/auth/profile        - 获取用户资料
PUT  /api/v1/auth/profile        - 更新用户资料
GET  /api/v1/auth/cards          - 获取银行卡列表
POST /api/v1/auth/cards          - 绑定银行卡
DELETE /api/v1/auth/cards/:id    - 删除银行卡
```

### 2.3 数据流

```
用户请求
    │
    ▼
Axum Router
    │
    ▼
Middleware (CORS, Auth)
    │
    ▼
Handler (参数验证)
    │
    ▼
Service (业务逻辑)
    │
    ├─► AI Service (调用外部API)
    │
    └─► Database (持久化)
    │
    ▼
Response
```

## 3. 前端架构

### 3.1 目录结构

```
frontend/src/
├── main.tsx              # 入口
├── App.tsx               # 根组件
├── router.tsx            # 路由配置
│
├── api/                  # API调用
│   ├── client.ts
│   └── auth.ts
│
├── components/           # 组件
│   ├── common/           # 通用组件
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   ├── Input.tsx
│   │   ├── Select.tsx
│   │   ├── Modal.tsx
│   │   └── Spinner.tsx
│   │
│   ├── layout/           # 布局组件
│   │   ├── Header.tsx
│   │   ├── Footer.tsx
│   │   └── Layout.tsx
│   │
│   ├── upload/           # 上传组件
│   │   └── ImageUpload.tsx
│   │
│   └── editor/          # 编辑器组件
│       ├── CodeEditor.tsx
│       └── CodePreview.tsx
│
├── pages/                # 页面
│   ├── HomePage.tsx
│   ├── GeneratePage.tsx
│   ├── PreviewPage.tsx
│   ├── HistoryPage.tsx
│   ├── SettingsPage.tsx
│   ├── SubscribePage.tsx
│   └── PaymentPage.tsx
│
├── stores/               # 状态管理
│   └── useAppStore.ts
│
├── types/                # 类型定义
│   └── api.ts
│
└── utils/               # 工具函数
    └── runtime.ts
```

### 3.2 状态管理

使用 Zustand 进行状态管理:

```typescript
interface AppState {
  // 用户状态
  user: User | null
  isAuthenticated: boolean

  // 生成状态
  currentImage: string | null
  selectedModel: string
  selectedLanguage: string
  generatedCode: string | null
  isGenerating: boolean

  // 订阅状态
  subscription: SubscriptionStatus | null
}
```

## 4. 安全架构

### 4.1 认证与授权

- JWT Token 认证
- Token 存储在 localStorage
- 每次请求携带 Authorization header
- 密码使用 bcrypt 加密

### 4.2 API 安全

- CORS 配置限制来源
- 请求参数验证 (validator)
- API Key 本地存储，不上传服务器
- 敏感信息日志脱敏

### 4.3 数据安全

- 用户密码 bcrypt 哈希存储
- SQLite 数据库本地存储
- 配置文件敏感信息环境变量

## 5. 部署架构

### 5.1 开发模式

```
前端: npm run dev     → http://localhost:5173
后端: cargo run      → http://localhost:8080
Tauri: npm run tauri → 桌面应用
```

### 5.2 生产模式

```
前端: npm run build   → dist/
后端: cargo build --release
Tauri: npm run tauri build
```

## 6. 性能优化

### 6.1 后端

- 连接池复用 (SQLite 锁优化)
- 流式响应 (SSE)
- 异步 I/O (Tokio)
- 图片压缩处理

### 6.2 前端

- Code Splitting
- 懒加载路由
- 图片压缩上传
- 缓存策略

## 7. 扩展性设计

### 7.1 AI 模型扩展

通过 trait 接口轻松添加新模型:

```rust
#[async_trait]
pub trait AiService: Send + Sync {
    async fn generate_code(...) -> AppResult<String>;
    async fn validate_api_key(...) -> AppResult<bool>;
}
```

### 7.2 支付渠道扩展

订阅模块支持多支付渠道:

```rust
enum PaymentMethod {
    Alipay,
    Wechat,
    Yunshanfu,
}
```
