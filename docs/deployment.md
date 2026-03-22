# 部署指南

本文档详细介绍 AI Screen Code 的部署方式，包括 Docker Compose 部署、本地开发和生产环境部署。

## 目录

- [前置要求](#前置要求)
- [Docker Compose 部署（推荐）](#docker-compose-部署推荐)
- [手动部署](#手动部署)
- [本地开发部署](#本地开发部署)
- [生产环境配置](#生产环境配置)
- [常见问题](#常见问题)

---

## 前置要求

| 工具 | 版本要求 |
|------|----------|
| Docker | 20.10+ |
| Docker Compose | 2.0+ |
| Node.js | 18+ (开发环境) |
| Rust | 1.75+ (开发环境) |
| PostgreSQL | 14+ (可选，生产环境推荐) |

**服务器配置建议：**
- CPU: 2 核心以上
- 内存: 4GB 以上
- 磁盘: 20GB 以上

---

## Docker Compose 部署（推荐）

### 步骤 1: 克隆项目

```bash
git clone https://github.com/BlackYHawk/ai-screen-code.git
cd ai-screen-code
```

### 步骤 2: 配置环境变量

```bash
cp .env.example .env
nano .env
```

编辑 `.env` 文件，配置必要的环境变量：

```env
# JWT 密钥（必须修改，请使用随机字符串）
JWT_SECRET=your-super-secret-jwt-key-change-in-production
JWT_EXPIRATION_DAYS=7

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# AI 模型 API Key（至少配置一个）
QWEN_API_KEY=your_qwen_api_key
MINIMAX_API_KEY=your_minimax_api_key
KIMI_API_KEY=your_kimi_api_key
GLM_API_KEY=your_glm_api_key

# OAuth 配置（可选）
QQ_CLIENT_ID=your_qq_app_id
QQ_CLIENT_SECRET=your_qq_app_secret
QQ_REDIRECT_URI=https://your-domain.com/auth/callback/qq

WECHAT_CLIENT_ID=your_wechat_appid
WECHAT_CLIENT_SECRET=your_wechat_appsecret
WECHAT_REDIRECT_URI=https://your-domain.com/auth/callback/wechat

DOUYIN_CLIENT_ID=your_douyin_client_key
DOUYIN_CLIENT_SECRET=your_douyin_client_secret
DOUYIN_REDIRECT_URI=https://your-domain.com/auth/callback/douyin

# 邮件配置（可选，用于发送验证码）
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password
SMTP_FROM=noreply@your-domain.com

# 前端配置
VITE_API_BASE_URL=https://your-domain.com/api
```

### 步骤 3: 启动服务

```bash
# 构建并启动所有服务
docker compose up -d

# 查看日志
docker compose logs -f

# 查看服务状态
docker compose ps
```

### 步骤 4: 验证部署

```bash
# 检查后端健康状态
curl http://localhost:8080/health

# 检查前端
curl http://localhost
```

服务启动后：
- 前端: http://你的服务器IP
- 后端 API: http://你的服务器IP:8080

---

## 手动部署

如果需要手动部署而不是使用 Docker，可以按照以下步骤：

### 后端部署

#### 1. 安装 Rust 环境

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装 SQLite 开发库
# macOS
brew install sqlite

# Ubuntu/Debian
sudo apt-get install libsqlite3-dev
```

#### 2. 编译后端

```bash
cd backend

# 复制配置
cp config.yaml.example config.yaml
nano config.yaml  # 编辑配置

# 编译
cargo build --release
```

#### 3. 运行后端

```bash
# 开发模式
cargo run

# 生产模式
cargo run --release
```

### 前端部署

#### 1. 安装 Node.js 依赖

```bash
cd frontend
npm install
```

#### 2. 构建前端

```bash
# 开发构建
npm run build

# 生产构建（需配置 VITE_API_BASE_URL）
VITE_API_BASE_URL=http://localhost:8080/api npm run build
```

#### 3. 使用 Nginx 部署

创建 Nginx 配置文件：

```nginx
server {
    listen 80;
    server_name your-domain.com;

    root /var/www/ai-screen-code/dist;
    index index.html;

    # SPA 路由支持
    location / {
        try_files $uri $uri/ /index.html;
    }

    # API 反向代理
    location /api/ {
        proxy_pass http://localhost:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

---

## 本地开发部署

### 后端开发

```bash
cd backend

# 方式一：使用 .env 文件
cp .env.example .env
# 编辑 .env 配置你的 API Key
cargo run

# 方式二：使用 config.yaml
cp config.yaml.example config.yaml
nano config.yaml
cargo run
```

后端服务启动在 http://localhost:8080

### 前端开发

```bash
cd frontend
npm install
npm run dev
```

前端服务启动在 http://localhost:5173

### 桌面应用开发

```bash
cd frontend
npm run tauri:dev
```

这将启动 Tauri 桌面应用。

---

## 生产环境配置

### 安全建议

1. **修改 JWT 密钥**
   ```env
   JWT_SECRET=生成一个强随机字符串
   ```

2. **配置 HTTPS**
   - 使用 Let's Encrypt 免费证书
   - 或购买商业 SSL 证书

3. **配置防火墙**
   ```bash
   # 开放必要端口
   sudo firewall-cmd --permanent --add-port=80/tcp
   sudo firewall-cmd --permanent --add-port=443/tcp
   sudo firewall-cmd --reload
   ```

4. **配置日志收集**
   ```bash
   # 查看 Docker 日志
   docker compose logs -f --tail=100

   # 持久化日志
   docker compose logs > /var/log/ai-screen-code.log 2>&1 &
   ```

### 性能优化

1. **使用 PostgreSQL 替代 SQLite**
   修改 `docker-compose.yml` 添加 PostgreSQL 服务

2. **配置 Redis 缓存**
   用于缓存会话和热点数据

3. **配置 CDN**
   静态资源使用 CDN 加速

### 监控配置

1. **使用 Prometheus + Grafana**
   监控服务指标

2. **配置告警**
   - 磁盘空间告警
   - 内存使用告警
   - 服务不可用告警

---

## 常见问题

### Q1: 第三方登录提示异常？

**原因**: OAuth 凭证未配置

**解决**: 在 `.env` 中配置 `QQ_CLIENT_ID`、`WECHAT_CLIENT_ID` 等

### Q2: 前端无法连接后端？

**解决**:
1. 检查后端是否启动: `docker compose ps`
2. 检查防火墙: `firewall-cmd --list-all`
3. 开放端口: `firewall-cmd --add-port=8080/tcp`

### Q3: 构建失败？

**解决**:
1. 确保 Docker 内存充足（至少 4GB）
2. 清理缓存: `docker system prune -a`

### Q4: 如何更新部署？

```bash
# 拉取最新代码
git pull origin main

# 重新构建
docker compose build

# 重启服务
docker compose up -d
```

### Q5: 如何配置域名？

1. 购买域名并配置 DNS 解析
2. 修改 Nginx 配置中的 `server_name`
3. 配置 SSL 证书

### Q6: AI 模型 API Key 从哪里获取？

| 模型 | 获取地址 |
|------|----------|
| 通义千问 (Qwen) | https://dashscope.console.aliyun.com/ |
| MiniMax | https://platform.minimaxi.com/ |
| Kimi (Moonshot) | https://platform.moonshot.cn/ |
| GLM (Zhipu) | https://open.bigmodel.cn/ |

---

## 技术支持

如有部署问题，请提交 Issue：https://github.com/BlackYHawk/ai-screen-code/issues
