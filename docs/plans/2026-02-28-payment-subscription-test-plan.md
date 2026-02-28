# 支付订阅功能测试计划

## 1. 当前状态概述

### 1.1 后端 (Rust/Axum)
- **现有测试**: 19个测试通过 (subscription_model_test.rs: 13个, subscription_handler_test.rs: 6个)
- **测试类型**: 主要是单元测试，覆盖模型序列化/反序列化、枚举转换
- **缺失**: 集成测试、handler 单元测试、数据库交互测试

### 1.2 前端 (React/TypeScript)
- **现有测试**: 无
- **需覆盖**: 组件测试、API Mock 测试

---

## 2. 测试计划

### 2.1 后端测试

#### 2.1.1 单元测试 (已有，需补充)
| 测试项 | 状态 | 优先级 |
|--------|------|--------|
| SubscriptionPlan 模型 | ✅ 通过 | - |
| PaymentMethod 枚举 | ✅ 通过 | - |
| OrderStatus 枚举 | ✅ 通过 | - |
| CreateOrderRequest 验证 | ✅ 通过 | - |
| PaymentCallbackRequest 验证 | ✅ 通过 | - |
| 响应模型转换 | ✅ 通过 | - |

#### 2.1.2 需新增的单元测试
| 测试项 | 描述 | 优先级 |
|--------|------|--------|
| CreateOrderRequest::validate() | 测试有效/无效请求 | 高 |
| PaymentCallbackRequest::validate() | 测试各种状态验证 | 高 |
| 订阅状态转换逻辑 | Active -> Expired -> Cancelled | 中 |
| Order 到 CreateOrderResponse 转换 | 确认字段映射正确 | 中 |

#### 2.1.3 集成测试 (需新增)
| 测试项 | 描述 | 优先级 |
|--------|------|--------|
| create_order_handler | 成功创建订单 | 高 |
| create_order_handler | 无效套餐 | 高 |
| create_order_handler | 无效支付方式 | 高 |
| get_subscription_status_handler | 有订阅状态 | 中 |
| get_subscription_status_handler | 无订阅状态 | 中 |
| payment_callback_handler | 支付成功回调 | 高 |
| payment_callback_handler | 订单不存在 | 中 |
| payment_callback_handler | 重复回调幂等性 | 高 |

### 2.2 前端测试

#### 2.2.1 组件测试
| 测试项 | 描述 | 优先级 |
|--------|------|--------|
| SubscribePage 渲染 | 正确显示三个套餐 | 高 |
| SubscribePage 交互 | 选择套餐后更新状态 | 高 |
| SubscribePage 交互 | 选择支付方式 | 中 |
| SubscribePage 错误 | 加载失败显示错误 | 中 |
| PaymentPage 渲染 | 显示订单详情 | 高 |
| PaymentPage 轮询 | 订单状态轮询 | 中 |
| PaymentPage 模拟支付 | DEV模式模拟支付 | 低 |

#### 2.2.2 API Mock 测试
| 测试项 | 描述 | 优先级 |
|--------|------|--------|
| getPlans | 返回套餐列表 | 高 |
| createOrder | 创建订单 | 高 |
| getOrderStatus | 获取订单状态 | 中 |
| getSubscriptionStatus | 获取订阅状态 | 中 |
| paymentCallback | 支付回调 | 高 |

---

## 3. 优化建议

### 3.1 后端优化
1. **幂等性检查**: payment_callback_handler 需检查订单是否已处理
2. **用户认证**: 移除硬编码的 "demo_user"
3. **错误处理**: 统一错误响应格式
4. **日志增强**: 增加关键操作的日志

### 3.2 前端优化
1. **加载状态**: 添加骨架屏
2. **错误边界**: 添加 React Error Boundary
3. **重试机制**: API 请求失败自动重试

---

## 4. 测试执行顺序

1. 先运行现有后端测试 ✅
2. 补充后端单元测试
3. 添加后端集成测试
4. 添加前端组件测试
5. 验证全部测试通过
6. 生成测试报告

---

## 5. 验收标准

- [ ] 后端测试覆盖率达到 80%+
- [ ] 所有后端测试通过
- [ ] 前端核心组件有测试覆盖
- [ ] 已识别并修复关键问题
- [ ] 测试报告完整
