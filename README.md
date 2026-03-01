# AI Screen Code

上传 UI 设计图片，AI 自动生成前端代码

## 功能特性

- **AI 代码生成**: 上传 UI 设计截图，AI 自动生成 React/TypeScript 代码
- **多模型支持**: 支持通义千问、MiniMax、Kimi、GLM 等主流 AI 视觉模型
- **第三方登录**: 支持 QQ、微信、抖音一键登录
- **代码预览**: 内置 Monaco 代码编辑器，支持实时预览
- **历史记录**: 保存生成历史，随时查看和复用
- **订阅支付**: 支持信用卡/借记卡订阅服务

## 技术栈

| 组件 | 技术 |
|------|------|
| 前端框架 | React 19 + TypeScript |
| UI 样式 | Tailwind CSS 4.x |
| 后端框架 | Rust + Axum |
| 桌面应用 | Tauri 2.x |
| 状态管理 | Zustand |
| 打包工具 | Vite 7.x |
| AI 客户端 | 多模型支持（Qwen/MiniMax/Kimi/GLM） |

## 快速开始

### 前端开发

```bash
cd frontend
npm install
npm run dev
```

访问 http://localhost:5173

### 后端开发

```bash
cd backend
cargo run
```

后端运行在 http://localhost:8080

### 桌面应用

```bash
cd frontend
npm run tauri:dev
```

## 项目结构

```
ai_screen_code/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── handlers/      # API 处理器
│   │   ├── services/      # 业务服务
│   │   ├── models/       # 数据模型
│   │   └── config.rs     # 配置管理
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/               # React 前端
│   ├── src/
│   │   ├── pages/        # 页面组件
│   │   ├── components/  # UI 组件
│   │   ├── api/         # API 客户端
│   │   └── stores/      # 状态管理
│   ├── src-tauri/       # Tauri 桌面应用
│   ├── package.json
│   └── Dockerfile
├── docs/                   # 项目文档
│   ├── api.md           # API 文档
│   ├── deployment.md    # 部署指南
│   ├── development.md   # 开发指南
│   └── configuration.md # 配置说明
├── docker-compose.yml     # Docker 编排
├── deploy/               # K8s 部署配置
└── .env.example          # 环境变量模板
```

## 文档

- [API 文档](./docs/api.md) - 所有 API 端点说明
- [部署指南](./docs/deployment.md) - Docker 部署、本地开发部署
- [开发指南](./docs/development.md) - 开发环境配置、运行说明
- [配置说明](./docs/configuration.md) - 环境变量、config.yaml 说明

## 测试

```bash
# 前端单元测试
cd frontend
npm test

# 前端 E2E 测试
npm run e2e

# 后端测试
cd backend
cargo test
```

## License

MIT
