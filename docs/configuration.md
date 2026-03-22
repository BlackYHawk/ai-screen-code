# 配置说明

本文档详细介绍 AI Screen Code 的所有配置选项，包括环境变量和 config.yaml 配置文件。

## 目录

- [配置文件结构](#配置文件结构)
- [环境变量](#环境变量)
- [config.yaml 配置](#configyaml-配置)
- [AI 模型配置](#ai-模型配置)
- [OAuth 配置](#oauth-配置)
- [邮件配置](#邮件配置)
- [配置优先级](#配置优先级)

---

## 配置文件结构

项目使用两种配置方式：

1. **环境变量** (`.env`) - 敏感信息和运行时配置
2. **config.yaml** - 应用程序配置文件

```
ai_screen_code/
├── .env                    # 环境变量（敏感）
├── config.yaml             # 应用配置
└── .env.example           # 环境变量模板
```

---

## 环境变量

### 必需配置

| 变量名 | 说明 | 示例 |
|--------|------|------|
| `JWT_SECRET` | JWT 密钥（生产环境必须修改） | `your-secret-key` |

### 服务器配置

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `SERVER_HOST` | `0.0.0.0` | 服务器监听地址 |
| `SERVER_PORT` | `8080` | 服务器监听端口 |

### JWT 配置

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `JWT_SECRET` | - | JWT 签名密钥 |
| `JWT_EXPIRATION_DAYS` | `7` | Token 过期天数 |

### AI 模型 API Key

| 变量名 | 说明 |
|--------|------|
| `QWEN_API_KEY` | 通义千问 API Key |
| `MINIMAX_API_KEY` | MiniMax API Key |
| `KIMI_API_KEY` | Kimi (Moonshot) API Key |
| `GLM_API_KEY` | GLM (Zhipu) API Key |

#### 可选的模型覆盖配置

| 变量名 | 说明 |
|--------|------|
| `QWEN_BASE_URL` | 通义千问 API 地址 |
| `QWEN_MODEL` | 通义千问模型名称 |
| `MINIMAX_BASE_URL` | MiniMax API 地址 |
| `MINIMAX_MODEL` | MiniMax 模型名称 |
| `KIMI_BASE_URL` | Kimi API 地址 |
| `KIMI_MODEL` | Kimi 模型名称 |
| `GLM_BASE_URL` | GLM API 地址 |
| `GLM_MODEL` | GLM 模型名称 |

### OAuth 配置

#### QQ OAuth

| 变量名 | 说明 |
|--------|------|
| `QQ_CLIENT_ID` | QQ 应用 ID |
| `QQ_CLIENT_SECRET` | QQ 应用密钥 |
| `QQ_REDIRECT_URI` | 回调地址 |

#### 微信 OAuth

| 变量名 | 说明 |
|--------|------|
| `WECHAT_CLIENT_ID` | 微信应用 ID |
| `WECHAT_CLIENT_SECRET` | 微信应用密钥 |
| `WECHAT_REDIRECT_URI` | 回调地址 |

#### 抖音 OAuth

| 变量名 | 说明 |
|--------|------|
| `DOUYIN_CLIENT_ID` | 抖音应用 Key |
| `DOUYIN_CLIENT_SECRET` | 抖音应用密钥 |
| `DOUYIN_REDIRECT_URI` | 回调地址 |

### 邮件配置

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `SMTP_HOST` | - | SMTP 服务器地址 |
| `SMTP_PORT` | `587` | SMTP 服务器端口 |
| `SMTP_USERNAME` | - | SMTP 用户名 |
| `SMTP_PASSWORD` | - | SMTP 密码 |
| `SMTP_FROM` | - | 发件人邮箱 |

### 数据库配置

| 变量名 | 说明 |
|--------|------|
| `DATABASE_URL` | PostgreSQL 连接字符串（可选，使用 SQLite 则不需要） |

### 前端配置

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `VITE_API_BASE_URL` | `/api` | 前端 API 地址 |

---

## config.yaml 配置

项目根目录下的 `config.yaml` 包含应用程序的主要配置。

### 配置示例

```yaml
server:
  host: "0.0.0.0"
  port: 8080

models:
  qwen:
    api_key: "${QWEN_API_KEY}"
    base_url: "https://dashscope.aliyuncs.com/api/v1"
    default_model: "qwen-vl-max"
    enabled: true

  minimax:
    api_key: "${MINIMAX_API_KEY}"
    base_url: "https://api.minimax.chat/v1"
    default_model: "MiniMax-VL01"
    enabled: true

  kimi:
    api_key: "${KIMI_API_KEY}"
    base_url: "https://api.moonshot.cn/v1"
    default_model: "moonshot-v1-vision-preview"
    enabled: true

  glm:
    api_key: "${GLM_API_KEY}"
    base_url: "https://open.bigmodel.cn/api/paas/v4"
    default_model: "glm-4v"
    enabled: true

logging:
  level: "info"
  format: "json"

cors:
  allowed_origins:
    - "http://localhost:5173"
    - "http://localhost:3000"
    - "https://your-domain.com"
  allowed_methods:
    - "GET"
    - "POST"
    - "PUT"
    - "DELETE"
    - "OPTIONS"
  allowed_headers:
    - "Content-Type"
    - "Authorization"
  allow_credentials: true
```

### 配置说明

#### server

服务器配置。

```yaml
server:
  host: "0.0.0.0"  # 监听地址
  port: 8080       # 监听端口
```

#### models

AI 模型配置。

```yaml
models:
  qwen:
    api_key: "your-api-key"           # API Key（支持 ${ENV_VAR} 语法）
    base_url: "https://..."           # API 基础地址
    default_model: "qwen-vl-max"     # 默认模型
    enabled: true                    # 是否启用
```

**支持的环境变量替换：**
- `${VAR_NAME}` - 使用环境变量值
- `${VAR_NAME:-default}` - 使用环境变量值，如未设置则使用默认值

#### logging

日志配置。

```yaml
logging:
  level: "info"     # 日志级别: trace, debug, info, warn, error
  format: "json"    # 格式: json, plain
```

#### cors

跨域配置。

```yaml
cors:
  allowed_origins:     # 允许的来源
    - "http://localhost:5173"
  allowed_methods:     # 允许的方法
    - "GET"
    - "POST"
  allowed_headers:     # 允许的头部
    - "Content-Type"
  allow_credentials:   # 是否允许凭证
    true
```

---

## AI 模型配置

### 通义千问 (Qwen)

```yaml
models:
  qwen:
    api_key: "${QWEN_API_KEY}"
    base_url: "https://dashscope.aliyuncs.com/api/v1"
    default_model: "qwen-vl-max"
    enabled: true
```

**获取地址:** https://dashscope.console.aliyun.com/

**支持的模型:**
- `qwen-vl-max` (推荐)
- `qwen-vl-plus`

---

### MiniMax

```yaml
models:
  minimax:
    api_key: "${MINIMAX_API_KEY}"
    base_url: "https://api.minimax.chat/v1"
    default_model: "MiniMax-VL01"
    enabled: true
```

**获取地址:** https://platform.minimaxi.com/

**支持的模型:**
- `MiniMax-VL01` (推荐)
- `MiniMax-Text-01`

---

### Kimi (Moonshot)

```yaml
models:
  kimi:
    api_key: "${KIMI_API_KEY}"
    base_url: "https://api.moonshot.cn/v1"
    default_model: "moonshot-v1-vision-preview"
    enabled: true
```

**获取地址:** https://platform.moonshot.cn/

**支持的模型:**
- `moonshot-v1-vision-preview` (推荐)
- `moonshot-v1-8k-vision-preview`

---

### GLM (Zhipu)

```yaml
models:
  glm:
    api_key: "${GLM_API_KEY}"
    base_url: "https://open.bigmodel.cn/api/paas/v4"
    default_model: "glm-4v"
    enabled: true
```

**获取地址:** https://open.bigmodel.cn/

**支持的模型:**
- `glm-4v` (推荐)
- `glm-4v-plus`

---

## OAuth 配置

### QQ OAuth

1. 前往 https://connect.qq.com/ 创建应用
2. 获取 `APP ID` 和 `APP Key`
3. 配置回调地址

```env
QQ_CLIENT_ID=your_app_id
QQ_CLIENT_SECRET=your_app_key
QQ_REDIRECT_URI=https://your-domain.com/auth/callback/qq
```

### 微信 OAuth

1. 前往 https://open.weixin.qq.com/ 创建应用
2. 获取 `AppID` 和 `AppSecret`
3. 配置授权回调域

```env
WECHAT_CLIENT_ID=your_appid
WECHAT_CLIENT_SECRET=your_appsecret
WECHAT_REDIRECT_URI=https://your-domain.com/auth/callback/wechat
```

### 抖音 OAuth

1. 前往 https://open.douyin.com/ 创建应用
2. 获取 `Client Key` 和 `Client Secret`
3. 配置回调地址

```env
DOUYIN_CLIENT_ID=your_client_key
DOUYIN_CLIENT_SECRET=your_client_secret
DOUYIN_REDIRECT_URI=https://your-domain.com/auth/callback/douyin
```

---

## 邮件配置

用于发送验证码等邮件通知。

```env
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=your-username
SMTP_PASSWORD=your-password
SMTP_FROM=noreply@your-domain.com
```

支持的邮件服务商：
- Gmail (smtp.gmail.com)
- QQ 邮箱 (smtp.qq.com)
- 阿里云邮箱 (smtp.aliyun.com)
- 企业邮件服务

---

## 配置优先级

配置加载顺序（后者覆盖前者）：

1. `config.yaml` 默认值
2. `config.yaml` 中的环境变量 `${VAR}` 替换
3. 系统环境变量覆盖

**示例：**

```yaml
# config.yaml
models:
  qwen:
    api_key: "${QWEN_API_KEY:-fallback-key}"
    base_url: "https://dashscope.aliyuncs.com/api/v1"
```

- 如果 `QWEN_API_KEY` 环境变量已设置，使用环境变量值
- 如果未设置，使用 `config.yaml` 中的默认值 `fallback-key`
- `base_url` 始终使用 `config.yaml` 中的值

---

## 安全建议

1. **敏感信息不要提交到 Git**
   - `.env` 文件已添加到 `.gitignore`
   - 使用 `.env.example` 作为模板

2. **生产环境必须修改的配置**
   - `JWT_SECRET`: 使用强随机字符串
   - 所有 API Key

3. **定期轮换**
   - 定期更换 API Key
   - 定期更换 JWT Secret

---

## 常见问题

### Q1: 如何查看当前配置？

启动后端后，日志会输出配置加载信息：

```
Loaded .env from /path/to/.env
Configuration loaded successfully
```

### Q2: 配置文件中的环境变量不生效？

确保：
1. 环境变量已正确设置
2. 使用正确的语法 `${VAR_NAME}` 或 `${VAR_NAME:-default}`
3. `.env` 文件在项目根目录

### Q3: 如何禁用某个 AI 模型？

在 `config.yaml` 中设置：

```yaml
models:
  kimi:
    enabled: false
```

或在环境变量中设置对应覆盖。
