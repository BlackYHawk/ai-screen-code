# AI Screen Code

上传 UI 设计图片，AI 自动生成前端代码

## 功能特性

- **AI 代码生成**: 上传 UI 设计截图，AI 自动生成 React/TypeScript 代码
- **多模型支持**: 支持通义千问、MiniMax、Kimi、GLM 等 AI 模型
- **第三方登录**: 支持 QQ、微信、抖音一键登录
- **代码预览**: 内置代码编辑器，支持实时预览
- **历史记录**: 保存生成历史，随时查看和复用

## 技术栈

| 组件 | 技术 |
|------|------|
| 前端 | React 19 + TypeScript + Tailwind CSS |
| 后端 | Rust + Axum |
| 桌面 | Tauri 2.x |
| 状态管理 | Zustand |
| AI 客户端 | 多模型支持 |

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
│   │   └── models/       # 数据模型
│   └── Dockerfile         # 后端 Docker 配置
├── frontend/               # React 前端
│   ├── src/
│   │   ├── pages/        # 页面组件
│   │   ├── components/   # UI 组件
│   │   ├── api/          # API 客户端
│   │   └── stores/      # 状态管理
│   └── Dockerfile        # 前端 Docker 配置
├── docker-compose.yml     # Docker 编排
├── deploy/               # K8s 部署配置
└── .env.example          # 环境变量模板
```

## 环境变量

复制 `.env.example` 为 `.env` 并配置：

```bash
# JWT 配置
JWT_SECRET=your-secret-key

# OAuth 配置（QQ、微信、抖音）
QQ_CLIENT_ID=your_qq_app_id
WECHAT_CLIENT_ID=your_wechat_appid
DOUYIN_CLIENT_ID=your_douyin_client_key

# AI 模型 API Key
QWEN_API_KEY=your_qwen_api_key
KIMI_API_KEY=your_kimi_api_key
```

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

## 部署

详见 [DEPLOY.md](./DEPLOY.md)

## License

MIT
