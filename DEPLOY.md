# 部署指南

本文档详细介绍如何将 AI Screen Code 部署到阿里云 ECS。

## 目录

- [快速部署](#快速部署)
- [手动部署](#手动部署)
- [配置说明](#配置说明)
- [常见问题](#常见问题)

---

## 快速部署

### 方式一：Docker Compose（推荐）

```bash
# 1. 克隆项目
git clone https://github.com/BlackYHawk/ai-screen-code.git
cd ai-screen-code

# 2. 配置环境变量
cp .env.example .env
nano .env  # 编辑配置

# 3. 启动服务
docker compose up -d

# 4. 访问
# 前端: http://你的服务器IP
# 后端: http://你的服务器IP:8080
```

### 方式二：一键部署脚本

```bash
# 在 ECS 上执行
curl -sL https://raw.githubusercontent.com/BlackYHawk/ai-screen-code/main/deploy/install.sh | bash
```

---

## 手动部署

### 前置要求

- Docker 20.10+
- Docker Compose 2.0+
- 阿里云 ECS 实例（建议 2核4G 以上）

### 步骤 1: 安装 Docker

```bash
# 安装 Docker
curl -fsSL https://get.docker.com | sh

# 启动 Docker
systemctl start docker
systemctl enable docker

# 添加当前用户到 docker 组
sudo usermod -aG docker $USER
```

### 步骤 2: 克隆项目

```bash
cd /opt
git clone https://github.com/BlackYHawk/ai-screen-code.git
cd ai-screen-code
```

### 步骤 3: 配置环境变量

```bash
cp .env.example .env
nano .env
```

填写以下必要配置：

```env
# JWT 密钥（必须修改）
JWT_SECRET=生成一个随机字符串

# 服务器地址
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# OAuth 配置（可选，配置后才能使用第三方登录）
# QQ OAuth - https://connect.qq.com/
QQ_CLIENT_ID=your_qq_app_id
QQ_CLIENT_SECRET=your_qq_app_secret
QQ_REDIRECT_URI=https://your-domain.com/auth/callback/qq

# 微信 OAuth - https://open.weixin.qq.com/
WECHAT_CLIENT_ID=your_wechat_appid
WECHAT_CLIENT_SECRET=your_wechat_appsecret
WECHAT_REDIRECT_URI=https://your-domain.com/auth/callback/wechat

# 抖音 OAuth - https://open.douyin.com/
DOUYIN_CLIENT_ID=your_douyin_client_key
DOUYIN_CLIENT_SECRET=your_douyin_client_secret
DOUYIN_REDIRECT_URI=https://your-domain.com/auth/callback/douyin

# AI 模型 API（至少配置一个）
QWEN_API_KEY=your_qwen_api_key
# 或
KIMI_API_KEY=your_kimi_api_key
# 或
MINIMAX_API_KEY=your_minimax_api_key
# 或
GLM_API_KEY=your_glm_api_key
```

### 步骤 4: 启动服务

```bash
# 构建并启动所有服务
docker compose up -d

# 查看日志
docker compose logs -f

# 查看服务状态
docker compose ps
```

### 步骤 5: 验证部署

```bash
# 检查后端健康状态
curl http://localhost:8080/health

# 检查前端
curl http://localhost
```

---

## 配置说明

### 端口说明

| 端口 | 服务 | 说明 |
|------|------|------|
| 80 | Nginx | 前端静态页面 |
| 8080 | Rust API | 后端服务 |

### 目录结构

```
/opt/ai-screen-code/
├── backend/           # 后端代码
│   └── Dockerfile
├── frontend/          # 前端代码
│   ├── Dockerfile
│   └── nginx.conf
├── docker-compose.yml # 编排配置
├── .env              # 环境变量（敏感）
├── .env.example      # 环境变量模板
└── data/            # 数据目录（可选）
    └── logs/
```

### Nginx 配置

前端 nginx.conf 已配置：
- API 反向代理到后端
- 静态资源缓存
- SPA 路由支持
- 安全头配置

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
2. 修改 `frontend/nginx.conf` 中的 `server_name`
3. 配置 SSL 证书（推荐使用 Let's Encrypt）

---

## 生产环境建议

1. **配置 SSL/HTTPS**
2. **配置域名**
3. **设置防火墙规则**
4. **配置日志收集**
5. **定期备份数据**
6. **监控服务状态**

---

## 技术支持

如有部署问题，请提交 Issue：https://github.com/BlackYHawk/ai-screen-code/issues
