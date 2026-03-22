# 开发指南

本文档详细介绍 AI Screen Code 项目的开发环境配置和运行说明。

## 目录

- [环境要求](#环境要求)
- [项目结构](#项目结构)
- [本地开发环境](#本地开发环境)
- [后端开发](#后端开发)
- [前端开发](#前端开发)
- [桌面应用开发](#桌面应用开发)
- [测试](#测试)
- [代码规范](#代码规范)

---

## 环境要求

### 必需工具

| 工具 | 版本 | 说明 |
|------|------|------|
| Node.js | 18+ | 前端开发环境 |
| npm | 9+ | Node.js 包管理器 |
| Rust | 1.75+ | 后端开发环境 |
| Cargo | 1.75+ | Rust 包管理器 |
| Git | 2.0+ | 版本控制 |

### 可选工具

| 工具 | 说明 |
|------|------|
| Docker | 容器化部署 |
| VS Code | 推荐 IDE |
| RustRover | Rust IDE |
| TablePlus | SQLite 数据库管理 |

### 安装步骤

#### 1. 安装 Node.js

**macOS:**
```bash
# 使用 Homebrew
brew install node

# 或使用 nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18
```

**Linux:**
```bash
# 使用 NodeSource
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Windows:**
下载安装包：https://nodejs.org/

#### 2. 安装 Rust

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

#### 3. 安装系统依赖

**macOS:**
```bash
# 使用 Homebrew
brew install sqlite
```

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libsqlite3-dev pkg-config libssl-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install sqlite-devel openssl-devel
```

---

## 项目结构

```
ai_screen_code/
├── backend/                 # Rust 后端
│   ├── src/
│   │   ├── handlers/       # API 处理器
│   │   │   ├── auth.rs     # 认证相关
│   │   │   ├── generate.rs # 代码生成
│   │   │   ├── history.rs # 历史记录
│   │   │   ├── models.rs   # 模型配置
│   │   │   ├── settings.rs # 系统设置
│   │   │   └── subscription.rs  # 订阅支付
│   │   ├── services/       # 业务服务
│   │   │   ├── ai_service.rs      # AI 服务
│   │   │   ├── qwen_service.rs    # 通义千问
│   │   │   ├── minimax_service.rs  # MiniMax
│   │   │   ├── kimi_service.rs    # Kimi
│   │   │   ├── glm_service.rs     # GLM
│   │   │   ├── history_service.rs # 历史服务
│   │   │   └── email_service.rs   # 邮件服务
│   │   ├── models/         # 数据模型
│   │   ├── config.rs       # 配置管理
│   │   ├── database.rs     # 数据库
│   │   ├── middleware.rs   # 中间件
│   │   ├── state.rs        # 应用状态
│   │   └── main.rs         # 入口文件
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/               # React 前端
│   ├── src/
│   │   ├── api/           # API 客户端
│   │   ├── components/    # UI 组件
│   │   ├── pages/         # 页面组件
│   │   ├── stores/        # 状态管理
│   │   ├── types/         # TypeScript 类型
│   │   ├── utils/         # 工具函数
│   │   └── App.tsx        # 应用入口
│   ├── src-tauri/         # Tauri 配置
│   ├── package.json
│   └── vite.config.ts
├── docs/                   # 项目文档
├── docker-compose.yml
└── .env.example
```

---

## 本地开发环境

### 1. 克隆项目

```bash
git clone https://github.com/BlackYHawk/ai-screen-code.git
cd ai-screen-code
```

### 2. 配置后端

```bash
cd backend

# 方式一：使用环境变量
cp ../.env.example ../.env
# 编辑 ../.env 配置 API Key

# 方式二：使用 config.yaml
cp config.yaml.example config.yaml
nano config.yaml
```

### 3. 配置前端

```bash
cd frontend
cp .env.example .env
nano .env
```

---

## 后端开发

### 启动后端

```bash
cd backend
cargo run
```

后端服务默认运行在 http://localhost:8080

### 常用命令

```bash
# 开发模式（热重载）
cargo run

# 编译
cargo build

# 运行测试
cargo test

# 检查代码
cargo check

# 代码格式化
cargo fmt

# _clippy 静态分析
cargo clippy
```

### API 文档

后端启动后访问：
- 健康检查: http://localhost:8080/health
- API 端点: http://localhost:8080/api/v1/*

---

## 前端开发

### 启动前端

```bash
cd frontend
npm install
npm run dev
```

前端服务默认运行在 http://localhost:5173

### 常用命令

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建生产版本
npm run build

# 代码检查
npm run lint

# 单元测试
npm test

# E2E 测试
npm run e2e

# Tauri 开发
npm run tauri:dev

# Tauri 构建
npm run tauri:build
```

---

## 桌面应用开发

### 启动桌面应用

```bash
cd frontend
npm run tauri:dev
```

### 调试

```bash
# 查看 Tauri 日志
# macOS
~/Library/Logs/ai-screen-code/

# Linux
~/.local/share/ai-screen-code/logs/

# Windows
%APPDATA%\ai-screen-code\logs\
```

---

## 测试

### 后端测试

```bash
cd backend

# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 显示测试覆盖率
cargo tarpaulin --out Html
```

### 前端测试

```bash
cd frontend

# 运行单元测试
npm test

# 监听模式
npm run test:watch

# 生成覆盖率报告
npm run test:coverage

# E2E 测试
npm run e2e

# E2E 测试（UI 模式）
npm run e2e:ui
```

---

## 代码规范

### Rust 代码规范

1. 使用 `cargo fmt` 格式化代码
2. 使用 `cargo clippy` 检查代码问题
3. 遵循 Rust 官方代码风格
4. 编写文档注释 `///` 和模块注释 `//!`

### TypeScript 代码规范

1. 使用 ESLint 检查代码
2. 遵循项目 ESLint 配置
3. 使用 TypeScript 严格模式
4. 组件使用函数式组件和 Hooks

### Git 提交规范

```bash
# 提交类型
feat:     新功能
fix:      Bug 修复
refactor: 代码重构
docs:     文档更新
test:     测试相关
chore:    构建/工具变动

# 示例
git commit -m "feat: 添加用户注册功能"
git commit -m "fix: 修复登录验证问题"
```

---

## 常用配置

### 配置 AI 模型

在 `.env` 或 `config.yaml` 中配置至少一个 AI 模型：

```env
# 至少配置一个
QWEN_API_KEY=your_api_key
# 或
MINIMAX_API_KEY=your_api_key
# 或
KIMI_API_KEY=your_api_key
# 或
GLM_API_KEY=your_api_key
```

### 配置 OAuth 登录（可选）

```env
# QQ
QQ_CLIENT_ID=your_qq_app_id
QQ_CLIENT_SECRET=your_qq_app_secret

# 微信
WECHAT_CLIENT_ID=your_wechat_appid
WECHAT_CLIENT_SECRET=your_wechat_appsecret

# 抖音
DOUYIN_CLIENT_ID=your_douyin_client_key
DOUYIN_CLIENT_SECRET=your_douyin_client_secret
```

---

## 常见问题

### Q1: Cargo 编译失败？

```bash
# 更新 Rust
rustup update

# 清理缓存
cargo clean
cargo build
```

### Q2: 前端依赖安装失败？

```bash
# 清理 npm 缓存
npm cache clean --force

# 删除 node_modules 重新安装
rm -rf node_modules
npm install
```

### Q3: 数据库初始化失败？

```bash
# 检查 SQLite 是否安装
sqlite3 --version

# 手动创建数据库
cd backend
cargo run -- --init-db
```

### Q4: 端口被占用？

```bash
# 查找占用端口的进程
lsof -i :8080

# 杀掉进程
kill -9 <PID>
```

---

## 下一步

- 阅读 [API 文档](./api.md) 了解所有接口
- 阅读 [配置说明](./configuration.md) 了解配置详情
- 阅读 [部署指南](./deployment.md) 了解部署方式
