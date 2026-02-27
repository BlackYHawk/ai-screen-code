use crate::models::{
    CreateOrderRequest, CreateOrderResponse, Order, OrderHistoryResponse, OrderStatus,
    PaymentCallbackRequest, PaymentMethod, Subscription, SubscriptionPlan,
    SubscriptionPlanResponse, SubscriptionStatus, SubscriptionStatusResponse,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use uuid::Uuid;

/// 获取订阅计划列表
pub async fn get_plans_handler() -> Json<Vec<SubscriptionPlanResponse>> {
    let plans = SubscriptionPlan::get_plans();
    Json(plans.into_iter().map(|p| p.into()).collect())
}

/// 创建订单
pub async fn create_order_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, String> {
    // 验证套餐
    let plans = SubscriptionPlan::get_plans();
    let plan = plans
        .iter()
        .find(|p| p.name == req.plan)
        .ok_or_else(|| "Invalid plan".to_string())?;

    // 验证支付方式
    let payment_method: PaymentMethod = req.payment_method.parse().map_err(|e: String| e)?;

    // 生成订单ID
    let order_id = Uuid::new_v4().to_string();

    // 使用临时用户ID（未登录用户）
    let user_id = "guest".to_string();

    // 创建订单
    let order = Order {
        id: order_id.clone(),
        user_id: user_id.clone(),
        plan: req.plan.clone(),
        amount: plan.price,
        payment_method: payment_method.clone(),
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: Utc::now(),
    };

    // 保存订单到数据库
    state
        .db
        .create_order(&order)
        .map_err(|e| format!("Failed to create order: {}", e))?;

    tracing::info!("Created order: {} for plan: {}", order.id, req.plan);

    let mut response: CreateOrderResponse = (&order).into();

    // 生成模拟支付信息
    response.qr_code = Some(generate_mock_qr_code(&order_id, &payment_method));
    response.payment_url = Some(format!("/payment/{}", order_id));

    Ok(Json(response))
}

/// 获取当前订阅状态
pub async fn get_subscription_status_handler(
    State(state): State<AppState>,
) -> Json<SubscriptionStatusResponse> {
    let user_id = "demo_user"; // TODO: 从认证中获取

    match state.db.get_active_subscription(user_id) {
        Ok(Some(sub)) => Json(SubscriptionStatusResponse::from(&sub)),
        _ => Json(SubscriptionStatusResponse {
            active: false,
            plan: None,
            status: None,
            start_date: None,
            end_date: None,
        }),
    }
}

/// 支付回调
pub async fn payment_callback_handler(
    State(state): State<AppState>,
    Json(req): Json<PaymentCallbackRequest>,
) -> Result<Json<serde_json::Value>, String> {
    // 查找订单
    let order = state
        .db
        .find_order_by_id(&req.order_id)
        .map_err(|e| format!("Failed to find order: {}", e))?
        .ok_or_else(|| "Order not found".to_string())?;

    // 更新订单状态为已支付
    state
        .db
        .update_order_status(&req.order_id, OrderStatus::Paid, Some(req.trade_no.clone()))
        .map_err(|e| format!("Failed to update order: {}", e))?;

    // 计算订阅结束日期（从现在起一个月）
    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(30);

    // 创建或更新订阅
    let subscription = Subscription {
        id: Uuid::new_v4().to_string(),
        user_id: order.user_id.clone(),
        plan: order.plan.clone(),
        status: SubscriptionStatus::Active,
        start_date,
        end_date,
        created_at: Utc::now(),
    };

    // 检查用户是否已有订阅
    if let Ok(Some(existing_sub)) = state.db.get_active_subscription(&order.user_id) {
        // 更新现有订阅
        state
            .db
            .update_subscription_status(&existing_sub.id, SubscriptionStatus::Expired)
            .ok();
    }

    // 创建新订阅
    state
        .db
        .create_subscription(&subscription)
        .map_err(|e| format!("Failed to create subscription: {}", e))?;

    tracing::info!(
        "Payment successful: order_id: {}, trade_no: {}",
        req.order_id,
        req.trade_no
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Payment processed successfully"
    })))
}

/// 获取订单状态
pub async fn get_order_status_handler(
    State(state): State<AppState>,
    Path(order_id): Path<String>,
) -> Result<Json<CreateOrderResponse>, String> {
    let order = state
        .db
        .find_order_by_id(&order_id)
        .map_err(|e| format!("Failed to find order: {}", e))?
        .ok_or_else(|| "Order not found".to_string())?;

    Ok(Json(CreateOrderResponse::from(&order)))
}

/// 获取订单历史
pub async fn get_order_history_handler(State(state): State<AppState>) -> Json<Vec<OrderHistoryResponse>> {
    let user_id = "demo_user"; // TODO: 从认证中获取

    match state.db.get_user_orders(user_id) {
        Ok(orders) => Json(orders.iter().map(|o| OrderHistoryResponse::from(o)).collect()),
        _ => Json(Vec::new()),
    }
}

/// 生成模拟支付二维码
fn generate_mock_qr_code(order_id: &str, payment_method: &PaymentMethod) -> String {
    let prefix = match payment_method {
        PaymentMethod::Wechat => "wechat://pay/",
        PaymentMethod::Alipay => "alipay://",
        PaymentMethod::Yunshanfu => "yunshanfu://",
    };
    format!("{}mock_order_{}", prefix, order_id)
}
