# API 文档

本文档详细介绍 AI Screen Code 后端 API 的所有端点。

## Base URL

```
开发环境: http://localhost:8080
生产环境: https://your-domain.com
```

## 通用说明

### 认证方式

除公开端点外，其他端点需要在请求头中携带 JWT Token：

```
Authorization: Bearer <your-jwt-token>
```

### 响应格式

所有响应都遵循统一格式：

```typescript
// 成功响应
{
  "success": true,
  "data": { ... }
}

// 失败响应
{
  "success": false,
  "error": "错误信息"
}
```

---

## 认证接口

### 注册

**POST** `/api/v1/auth/register`

注册新用户账号。

**请求体:**

```typescript
{
  "email": "user@example.com",      // 邮箱
  "password": "password123",        // 密码
  "username": "username"            // 用户名
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "username",
      "avatar": null,
      "created_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

---

### 登录

**POST** `/api/v1/auth/login`

用户登录。

**请求体:**

```typescript
{
  "email": "user@example.com",
  "password": "password123"
}
```

**响应:** 同注册接口

---

### 发送验证码

**POST** `/api/v1/auth/code/send`

发送邮箱验证码（用于注册/找回密码）。

**请求体:**

```typescript
{
  "email": "user@example.com",
  "type": "register" | "reset_password"  // 验证码类型
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "message": "验证码已发送"
  }
}
```

---

### 验证验证码

**POST** `/api/v1/auth/code/verify`

验证邮箱验证码。

**请求体:**

```typescript
{
  "email": "user@example.com",
  "code": "123456"
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "valid": true
  }
}
```

---

### 重置密码

**POST** `/api/v1/auth/password/reset`

通过验证码重置密码。

**请求体:**

```typescript
{
  "email": "user@example.com",
  "code": "123456",
  "new_password": "newpassword123"
}
```

---

### 第三方登录

**POST** `/api/v1/auth/third-party/login`

第三方平台登录。

**请求体:**

```typescript
{
  "provider": "qq" | "wechat" | "douyin",
  "code": "authorization_code_from_provider"
}
```

---

### 绑定第三方账号

**POST** `/api/v1/auth/third-party/bind`

将第三方账号绑定到已有账户。

**请求体:**

```typescript
{
  "provider": "qq" | "wechat" | "douyin",
  "code": "authorization_code_from_provider"
}
```

---

### 获取 OAuth URL

**GET** `/api/v1/auth/third-party/url/:provider`

获取第三方登录授权页面 URL。

**参数:**

- `provider`: `qq` | `wechat` | `douyin`

**响应:**

```typescript
{
  "success": true,
  "data": {
    "url": "https://open.qq.com/..."
  }
}
```

---

### 获取用户资料

**GET** `/api/v1/auth/profile`

获取当前登录用户信息（需认证）。

**响应:**

```typescript
{
  "success": true,
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "username",
    "avatar": "https://...",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 更新用户资料

**PUT** `/api/v1/auth/profile`

更新当前用户信息（需认证）。

**请求体:**

```typescript
{
  "username": "new_username",   // 可选
  "avatar": "https://..."      // 可选，头像 URL
}
```

---

### 银行卡管理

**GET** `/api/v1/auth/cards`

获取已绑定的银行卡列表（需认证）。

**POST** `/api/v1/auth/cards`

绑定银行卡（需认证）。

**请求体:**

```typescript
{
  "card_number": "6222021234567890",
  "card_holder": "张三",
  "bank_name": "中国工商银行"
}
```

**DELETE** `/api/v1/auth/cards/:id`

解绑银行卡（需认证）。

---

## 代码生成接口

### 生成代码

**POST** `/api/v1/generate`

上传 UI 设计图片，生成前端代码。

**请求体:**

```typescript
{
  "image": "base64_encoded_image",  // Base64 编码的图片
  "language": "tsx" | "jsx" | "vue", // 目标语言
  "model": "qwen" | "minimax" | "kimi" | "glm"  // AI 模型
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "id": "history_uuid",
    "code": "生成的代码...",
    "language": "tsx",
    "model": "qwen",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 流式生成代码

**POST** `/api/v1/generate/stream`

流式返回生成的代码（Server-Sent Events）。

**请求体:** 同 `/api/v1/generate`

**响应:** SSE 流格式

```
data: 代码片段1
data: 代码片段2
...
data: [DONE]
```

---

## 模型配置接口

### 获取可用模型

**GET** `/api/v1/models`

获取所有可用的 AI 模型列表。

**响应:**

```typescript
{
  "success": true,
  "data": [
    {
      "name": "qwen",
      "display_name": "通义千问",
      "default_model": "qwen-vl-max",
      "enabled": true,
      "configured": true
    },
    {
      "name": "minimax",
      "display_name": "MiniMax",
      "default_model": "MiniMax-VL01",
      "enabled": true,
      "configured": false
    }
  ]
}
```

---

### 验证模型 API Key

**POST** `/api/v1/models/validate`

验证指定的模型 API Key 是否有效。

**请求体:**

```typescript
{
  "model": "qwen" | "minimax" | "kimi" | "glm",
  "api_key": "your_api_key"
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "valid": true,
    "model": "qwen-vl-max"
  }
}
```

---

### 获取模型配置

**GET** `/api/v1/models/:model`

获取指定模型的配置信息。

**参数:**

- `model`: 模型名称 (qwen/minimax/kimi/glm)

**响应:**

```typescript
{
  "success": true,
  "data": {
    "api_key": "***",  // 隐藏部分
    "base_url": "https://...",
    "default_model": "qwen-vl-max",
    "enabled": true
  }
}
```

---

### 更新模型配置

**POST** `/api/v1/models/:model`

更新指定模型的配置。

**请求体:**

```typescript
{
  "api_key": "new_api_key",       // 可选
  "base_url": "https://...",      // 可选
  "default_model": "model_name",  // 可选
  "enabled": true                 // 可选
}
```

---

## 历史记录接口

### 获取历史列表

**GET** `/api/v1/history`

获取当前用户的历史记录列表。

**查询参数:**

- `page`: 页码 (默认 1)
- `limit`: 每页数量 (默认 20)

**响应:**

```typescript
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "image_url": "https://...",
      "code": "...",
      "language": "tsx",
      "model": "qwen",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 获取历史详情

**GET** `/api/v1/history/:id`

获取单条历史记录的详情。

---

### 删除历史记录

**DELETE** `/api/v1/history/:id`

删除指定的历史记录。

---

## 设置接口

### 获取设置

**GET** `/api/v1/settings`

获取系统设置信息。

**响应:**

```typescript
{
  "success": true,
  "data": {
    "default_model": "qwen",
    "default_language": "tsx",
    "theme": "light" | "dark"
  }
}
```

---

### 更新设置

**POST** `/api/v1/settings`

更新系统设置。

**请求体:**

```typescript
{
  "default_model": "qwen",
  "default_language": "tsx",
  "theme": "dark"
}
```

---

## 订阅支付接口

### 获取订阅计划

**GET** `/api/v1/subscriptions/plans`

获取所有可用的订阅计划。

**响应:**

```typescript
{
  "success": true,
  "data": [
    {
      "id": "plan_1",
      "name": "免费版",
      "price": 0,
      "features": ["每天 5 次生成", "基础模型"],
      "duration_days": null
    },
    {
      "id": "plan_2",
      "name": "专业版",
      "price": 29.9,
      "features": ["无限次数生成", "所有模型", "优先处理"],
      "duration_days": 30
    }
  ]
}
```

---

### 创建订单

**POST** `/api/v1/subscriptions/create`

创建订阅订单。

**请求体:**

```typescript
{
  "plan_id": "plan_2",
  "payment_method": "card",
  "card_id": "card_uuid"  // 可选，使用已绑定的卡
}
```

**响应:**

```typescript
{
  "success": true,
  "data": {
    "order_id": "order_uuid",
    "amount": 29.9,
    "status": "pending",
    "payment_url": "https://payment..."
  }
}
```

---

### 获取订阅状态

**GET** `/api/v1/subscriptions/status`

获取当前用户的订阅状态。

**响应:**

```typescript
{
  "success": true,
  "data": {
    "plan": "pro",
    "status": "active",
    "expires_at": "2024-02-01T00:00:00Z",
    "remaining_generations": null  // null 表示无限制
  }
}
```

---

### 支付回调

**POST** `/api/v1/subscriptions/webhook`

支付平台回调接口。

---

### 获取订单历史

**GET** `/api/v1/subscriptions/orders`

获取用户的订单历史列表。

---

### 获取订单状态

**GET** `/api/v1/subscriptions/orders/:order_id`

获取指定订单的详细信息。

---

## 通用接口

### 健康检查

**GET** `/health`

检查服务健康状态。

**响应:** `OK`

---

### 根路由

**GET** `/`

返回服务信息。

**响应:** `AI Screen Code Backend API`

---

## 错误码

| 错误码 | 说明 |
|--------|------|
| 400 | 请求参数错误 |
| 401 | 未授权（未登录或 Token 无效） |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 429 | 请求过于频繁 |
| 500 | 服务器内部错误 |
