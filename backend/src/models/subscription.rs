use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use thiserror::Error;

/// 订阅相关错误
#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Invalid plan: {0}")]
    InvalidPlan(String),

    #[error("Invalid payment method: {0}")]
    InvalidPaymentMethod(String),

    #[error("Order not found: {0}")]
    OrderNotFound(String),

    #[error("Subscription not found")]
    SubscriptionNotFound,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Unauthorized")]
    Unauthorized,
}

impl Serialize for SubscriptionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// 订阅计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String, // lite/pro/max
    pub price: i32,  // 价格（分）
    pub features: Vec<String>,
}

impl SubscriptionPlan {
    /// 获取订阅计划列表（使用缓存）
    pub fn get_plans() -> &'static Vec<SubscriptionPlan> {
        static PLANS: OnceLock<Vec<SubscriptionPlan>> = OnceLock::new();
        PLANS.get_or_init(|| {
            vec![
                Self {
                    id: "lite".to_string(),
                    name: "lite".to_string(),
                    price: 1000, // 10元
                    features: vec![
                        "基础代码生成".to_string(),
                        "每日50次生成".to_string(),
                        "标准支持".to_string(),
                    ],
                },
                Self {
                    id: "pro".to_string(),
                    name: "pro".to_string(),
                    price: 3000, // 30元
                    features: vec![
                        "高级代码生成".to_string(),
                        "每日200次生成".to_string(),
                        "优先支持".to_string(),
                        "高级模板".to_string(),
                    ],
                },
                Self {
                    id: "max".to_string(),
                    name: "max".to_string(),
                    price: 5000, // 50元
                    features: vec![
                        "全部功能".to_string(),
                        "无限次数生成".to_string(),
                        "7x24支持".to_string(),
                        "专属客服".to_string(),
                        "API访问".to_string(),
                    ],
                },
            ]
        })
    }

    /// 根据 ID 获取计划
    pub fn get_by_id(id: &str) -> Option<&'static SubscriptionPlan> {
        Self::get_plans().iter().find(|p| p.id == id)
    }
}

/// 订阅状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Expired,
    Cancelled,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionStatus::Active => write!(f, "active"),
            SubscriptionStatus::Expired => write!(f, "expired"),
            SubscriptionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// 用户订阅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub user_id: String,
    pub plan: String, // lite/pro/max
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// 支付方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Wechat,
    Alipay,
    Yunshanfu,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Wechat => write!(f, "wechat"),
            PaymentMethod::Alipay => write!(f, "alipay"),
            PaymentMethod::Yunshanfu => write!(f, "yunshanfu"),
        }
    }
}

impl std::str::FromStr for PaymentMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wechat" => Ok(PaymentMethod::Wechat),
            "alipay" => Ok(PaymentMethod::Alipay),
            "yunshanfu" => Ok(PaymentMethod::Yunshanfu),
            _ => Err(format!("Unknown payment method: {}", s)),
        }
    }
}

/// 订单状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Paid => write!(f, "paid"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// 订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub plan: String,
    pub amount: i32,
    pub payment_method: PaymentMethod,
    pub status: OrderStatus,
    pub trade_no: Option<String>, // 第三方交易号
    pub created_at: DateTime<Utc>,
}

// ============ 请求模型 ============

/// 创建订单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub plan: String,
    pub payment_method: String,
}

impl CreateOrderRequest {
    /// 验证请求
    pub fn validate(&self) -> Result<(), SubscriptionError> {
        // 验证套餐
        if !["lite", "pro", "max"].contains(&self.plan.as_str()) {
            return Err(SubscriptionError::InvalidPlan(self.plan.clone()));
        }
        // 验证支付方式
        if !["alipay", "wechat", "yunshanfu"].contains(&self.payment_method.as_str()) {
            return Err(SubscriptionError::InvalidPaymentMethod(self.payment_method.clone()));
        }
        Ok(())
    }
}

/// 支付回调请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCallbackRequest {
    pub order_id: String,
    pub trade_no: String,
    pub status: String,
}

impl PaymentCallbackRequest {
    /// 验证请求
    pub fn validate(&self) -> Result<(), SubscriptionError> {
        if self.order_id.is_empty() {
            return Err(SubscriptionError::OrderNotFound("order_id is empty".to_string()));
        }
        if self.trade_no.is_empty() {
            return Err(SubscriptionError::OrderNotFound("trade_no is empty".to_string()));
        }
        if !["paid", "pending", "failed"].contains(&self.status.as_str()) {
            return Err(SubscriptionError::InvalidPaymentMethod(self.status.clone()));
        }
        Ok(())
    }
}

// ============ 响应模型 ============

/// 订阅计划响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlanResponse {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub price_display: String,
    pub features: Vec<String>,
}

impl From<&SubscriptionPlan> for SubscriptionPlanResponse {
    fn from(plan: &SubscriptionPlan) -> Self {
        Self {
            id: plan.id.clone(),
            name: plan.name.clone(),
            price: plan.price,
            price_display: format!("{:.2}", plan.price as f64 / 100.0),
            features: plan.features.clone(),
        }
    }
}

impl From<SubscriptionPlan> for SubscriptionPlanResponse {
    fn from(plan: SubscriptionPlan) -> Self {
        Self {
            id: plan.id,
            name: plan.name,
            price: plan.price,
            price_display: format!("{:.2}", plan.price as f64 / 100.0),
            features: plan.features,
        }
    }
}

/// 当前订阅状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStatusResponse {
    pub active: bool,
    pub plan: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl From<&Subscription> for SubscriptionStatusResponse {
    fn from(sub: &Subscription) -> Self {
        Self {
            active: sub.status == SubscriptionStatus::Active,
            plan: Some(sub.plan.clone()),
            status: Some(sub.status.to_string()),
            start_date: Some(sub.start_date.format("%Y-%m-%d %H:%M:%S").to_string()),
            end_date: Some(sub.end_date.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 创建订单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub plan: String, // 套餐 ID
    pub amount: i32,
    pub amount_display: String,
    pub payment_method: String,
    pub status: String, // 订单状态: pending, paid, failed
    pub qr_code: Option<String>, // 支付二维码（模拟）
    pub payment_url: Option<String>, // 支付链接
}

impl From<&Order> for CreateOrderResponse {
    fn from(order: &Order) -> Self {
        Self {
            order_id: order.id.clone(),
            plan: order.plan.clone(),
            amount: order.amount,
            amount_display: format!("{:.2}", order.amount as f64 / 100.0),
            payment_method: order.payment_method.to_string(),
            status: order.status.to_string(),
            qr_code: None,
            payment_url: None,
        }
    }
}

/// 订单历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderHistoryResponse {
    pub id: String,
    pub plan: String,
    pub amount: i32,
    pub amount_display: String,
    pub payment_method: String,
    pub status: String,
    pub trade_no: Option<String>,
    pub created_at: String,
}

impl From<&Order> for OrderHistoryResponse {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id.clone(),
            plan: order.plan.clone(),
            amount: order.amount,
            amount_display: format!("{:.2}", order.amount as f64 / 100.0),
            payment_method: order.payment_method.to_string(),
            status: order.status.to_string(),
            trade_no: order.trade_no.clone(),
            created_at: order.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

// ============ 统一 API 响应 ============

/// 统一 API 响应格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Alias for ok() - 统一响应格式
    pub fn success(data: T) -> Self {
        Self::ok(data)
    }

    pub fn err(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}
