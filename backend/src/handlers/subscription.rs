use crate::models::{
    ApiResponse, CreateOrderRequest, CreateOrderResponse, Order, OrderHistoryResponse, OrderStatus,
    PaymentCallbackRequest, PaymentMethod, Subscription, SubscriptionPlan,
    SubscriptionPlanResponse, SubscriptionStatus, SubscriptionStatusResponse,
};
use crate::services::payment_service::PaymentService;
use crate::state::AppState;
use axum::{
    extract::{Path, State, Extension},
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use crate::handlers::auth::Claims;

/// 获取订阅计划列表
pub async fn get_plans_handler() -> Json<Vec<SubscriptionPlanResponse>> {
    let plans = SubscriptionPlan::get_plans();
    Json(plans.into_iter().map(|p| p.into()).collect())
}

/// 创建订单
pub async fn create_order_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateOrderRequest>,
) -> Json<ApiResponse<CreateOrderResponse>> {
    // 验证请求参数
    if let Err(e) = req.validate() {
        return Json(ApiResponse::error(e.to_string()));
    }

    // 验证套餐
    let plans = SubscriptionPlan::get_plans();
    let plan = match plans.iter().find(|p| p.id == req.plan) {
        Some(p) => p,
        None => return Json(ApiResponse::error("Invalid plan".to_string())),
    };

    // 验证支付方式
    let payment_method: PaymentMethod = match req.payment_method.parse() {
        Ok(pm) => pm,
        Err(_) => return Json(ApiResponse::error("Invalid payment method".to_string())),
    };

    // 生成订单ID
    let order_id = Uuid::new_v4().to_string();

    // 从认证获取用户ID
    let user_id = claims.sub.clone();

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
    if let Err(e) = state.db.create_order(&order) {
        tracing::error!("Failed to create order: {}", e);
        return Json(ApiResponse::error(format!("Failed to create order: {}", e)));
    }

    tracing::info!("Created order: {} for plan: {}", order.id, req.plan);

    let mut response: CreateOrderResponse = (&order).into();

    // 使用支付服务生成模拟支付信息
    let payment_info = PaymentService::generate_qr_code(&order_id, &payment_method);
    let payment_url = PaymentService::generate_payment_url(&order_id, &payment_method);
    response.qr_code = Some(payment_info);
    response.payment_url = Some(payment_url);

    Json(ApiResponse::success(response))
}

/// 获取当前订阅状态
pub async fn get_subscription_status_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Json<ApiResponse<SubscriptionStatusResponse>> {
    let user_id = &claims.sub;

    match state.db.get_active_subscription(user_id) {
        Ok(Some(sub)) => Json(ApiResponse::success(SubscriptionStatusResponse::from(&sub))),
        _ => Json(ApiResponse::success(SubscriptionStatusResponse {
            active: false,
            plan: None,
            status: None,
            start_date: None,
            end_date: None,
        })),
    }
}

/// 支付回调
pub async fn payment_callback_handler(
    State(state): State<AppState>,
    Json(req): Json<PaymentCallbackRequest>,
) -> Result<Json<serde_json::Value>, String> {
    // 验证请求
    req.validate().map_err(|e| e.to_string())?;

    // 查找订单
    let order = state
        .db
        .find_order_by_id(&req.order_id)
        .map_err(|e| format!("Failed to find order: {}", e))?
        .ok_or_else(|| "Order not found".to_string())?;

    // 幂等性检查：订单已支付则直接返回成功
    if order.status == OrderStatus::Paid {
        tracing::info!("Order already paid, skipping duplicate callback: {}", req.order_id);
        return Ok(Json(serde_json::json!({
            "success": true,
            "message": "Order already processed"
        })));
    }

    // 验证订单状态为 pending
    if order.status != OrderStatus::Pending {
        return Err(format!("Invalid order status: {}", order.status));
    }

    // 验证支付状态
    if req.status != "paid" {
        return Err("Payment not successful".to_string());
    }

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
        // 更新现有订阅为过期
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
pub async fn get_order_history_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Json<ApiResponse<Vec<OrderHistoryResponse>>> {
    let user_id = &claims.sub;

    match state.db.get_user_orders(user_id) {
        Ok(orders) => Json(ApiResponse::success(orders.iter().map(|o| OrderHistoryResponse::from(o)).collect())),
        _ => Json(ApiResponse::success(Vec::new())),
    }
}
