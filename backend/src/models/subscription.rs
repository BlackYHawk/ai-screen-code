use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 订阅计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String, // lite/pro/max
    pub price: i32,  // 价格（分）
    pub features: Vec<String>,
}

impl SubscriptionPlan {
    pub fn get_plans() -> Vec<Self> {
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

/// 支付回调请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCallbackRequest {
    pub order_id: String,
    pub trade_no: String,
    pub status: String,
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
    pub amount: i32,
    pub amount_display: String,
    pub payment_method: String,
    pub qr_code: Option<String>, // 支付二维码（模拟）
    pub payment_url: Option<String>, // 支付链接
}

impl From<&Order> for CreateOrderResponse {
    fn from(order: &Order) -> Self {
        Self {
            order_id: order.id.clone(),
            amount: order.amount,
            amount_display: format!("{:.2}", order.amount as f64 / 100.0),
            payment_method: order.payment_method.to_string(),
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
