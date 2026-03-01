use ai_screen_code::models::{
    CreateOrderRequest, CreateOrderResponse, Order, OrderHistoryResponse, OrderStatus,
    PaymentCallbackRequest, PaymentMethod, Subscription, SubscriptionPlan, SubscriptionStatus,
    SubscriptionStatusResponse,
};
use chrono::Utc;

// ============ Order Tests ============

#[test]
fn test_order_creation() {
    let now = Utc::now();
    let order = Order {
        id: "test_order_id".to_string(),
        user_id: "test_user".to_string(),
        plan: "pro".to_string(),
        amount: 3000,
        payment_method: PaymentMethod::Alipay,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    assert_eq!(order.id, "test_order_id");
    assert_eq!(order.user_id, "test_user");
    assert_eq!(order.plan, "pro");
    assert_eq!(order.amount, 3000);
    assert_eq!(order.payment_method, PaymentMethod::Alipay);
    assert_eq!(order.status, OrderStatus::Pending);
    assert!(order.trade_no.is_none());
}

#[test]
fn test_order_with_trade_no() {
    let now = Utc::now();
    let order = Order {
        id: "test_order_id".to_string(),
        user_id: "test_user".to_string(),
        plan: "max".to_string(),
        amount: 5000,
        payment_method: PaymentMethod::Wechat,
        status: OrderStatus::Paid,
        trade_no: Some("wx_trade_123456".to_string()),
        created_at: now,
    };

    assert_eq!(order.status, OrderStatus::Paid);
    assert_eq!(order.trade_no.unwrap(), "wx_trade_123456");
}

#[test]
fn test_order_status_transitions() {
    let now = Utc::now();

    // Pending order
    let mut order = Order {
        id: "order_1".to_string(),
        user_id: "user_1".to_string(),
        plan: "lite".to_string(),
        amount: 1000,
        payment_method: PaymentMethod::Alipay,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    assert_eq!(order.status, OrderStatus::Pending);

    // Transition to Paid
    order.status = OrderStatus::Paid;
    order.trade_no = Some("ali_trade_789".to_string());
    assert_eq!(order.status, OrderStatus::Paid);
    assert!(order.trade_no.is_some());

    // Transition to Cancelled
    order.status = OrderStatus::Cancelled;
    assert_eq!(order.status, OrderStatus::Cancelled);
}

// ============ Payment Method Tests ============

#[test]
fn test_payment_method_all_variants() {
    let methods = vec![
        PaymentMethod::Wechat,
        PaymentMethod::Alipay,
        PaymentMethod::Yunshanfu,
    ];

    assert_eq!(methods.len(), 3);
    assert_eq!(methods[0].to_string(), "wechat");
    assert_eq!(methods[1].to_string(), "alipay");
    assert_eq!(methods[2].to_string(), "yunshanfu");
}

#[test]
fn test_payment_method_from_str() {
    assert_eq!("wechat".parse::<PaymentMethod>().unwrap(), PaymentMethod::Wechat);
    assert_eq!("alipay".parse::<PaymentMethod>().unwrap(), PaymentMethod::Alipay);
    assert_eq!("yunshanfu".parse::<PaymentMethod>().unwrap(), PaymentMethod::Yunshanfu);
    assert_eq!("WECHAT".parse::<PaymentMethod>().unwrap(), PaymentMethod::Wechat);
    assert_eq!("ALIPAY".parse::<PaymentMethod>().unwrap(), PaymentMethod::Alipay);
    assert_eq!("YUNSHANFU".parse::<PaymentMethod>().unwrap(), PaymentMethod::Yunshanfu);
}

#[test]
fn test_payment_method_from_str_invalid() {
    let result: Result<PaymentMethod, _> = "credit_card".parse();
    assert!(result.is_err());

    let result: Result<PaymentMethod, _> = "".parse();
    assert!(result.is_err());

    let result: Result<PaymentMethod, _> = "paypal".parse();
    assert!(result.is_err());
}

// ============ CreateOrderRequest Tests ============

#[test]
fn test_create_order_request_validation_valid() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "alipay".to_string(),
    };

    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_create_order_request_validation_all_valid_plans() {
    for plan in ["lite", "pro", "max"] {
        for method in ["alipay", "wechat", "yunshanfu"] {
            let request = CreateOrderRequest {
                plan: plan.to_string(),
                payment_method: method.to_string(),
            };
            let result = request.validate();
            assert!(result.is_ok(), "plan: {}, method: {}", plan, method);
        }
    }
}

#[test]
fn test_create_order_request_validation_invalid_plan() {
    let request = CreateOrderRequest {
        plan: "enterprise".to_string(),
        payment_method: "alipay".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ai_screen_code::models::SubscriptionError::InvalidPlan(_)));
}

#[test]
fn test_create_order_request_validation_invalid_payment_method() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "credit_card".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ai_screen_code::models::SubscriptionError::InvalidPaymentMethod(_)));
}

#[test]
fn test_create_order_request_validation_empty_plan() {
    let request = CreateOrderRequest {
        plan: "".to_string(),
        payment_method: "alipay".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_create_order_request_validation_empty_payment_method() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

// ============ PaymentCallbackRequest Tests ============

#[test]
fn test_payment_callback_request_validation_valid() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "trade_456".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_payment_callback_request_validation_all_valid_statuses() {
    for status in ["paid", "pending", "failed"] {
        let request = PaymentCallbackRequest {
            order_id: "order_123".to_string(),
            trade_no: "trade_456".to_string(),
            status: status.to_string(),
        };
        let result = request.validate();
        assert!(result.is_ok(), "status: {}", status);
    }
}

#[test]
fn test_payment_callback_request_validation_empty_order_id() {
    let request = PaymentCallbackRequest {
        order_id: "".to_string(),
        trade_no: "trade_456".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_payment_callback_request_validation_empty_trade_no() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_payment_callback_request_validation_invalid_status() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "trade_456".to_string(),
        status: "completed".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

// ============ SubscriptionPlan Tests ============

#[test]
fn test_subscription_plan_get_plans() {
    let plans = SubscriptionPlan::get_plans();

    assert_eq!(plans.len(), 3);

    // Verify lite plan
    let lite = &plans[0];
    assert_eq!(lite.id, "lite");
    assert_eq!(lite.price, 1000);

    // Verify pro plan
    let pro = &plans[1];
    assert_eq!(pro.id, "pro");
    assert_eq!(pro.price, 3000);

    // Verify max plan
    let max = &plans[2];
    assert_eq!(max.id, "max");
    assert_eq!(max.price, 5000);
}

#[test]
fn test_subscription_plan_get_by_id() {
    let lite = SubscriptionPlan::get_by_id("lite");
    assert!(lite.is_some());
    assert_eq!(lite.unwrap().price, 1000);

    let pro = SubscriptionPlan::get_by_id("pro");
    assert!(pro.is_some());
    assert_eq!(pro.unwrap().price, 3000);

    let max = SubscriptionPlan::get_by_id("max");
    assert!(max.is_some());
    assert_eq!(max.unwrap().price, 5000);

    let invalid = SubscriptionPlan::get_by_id("enterprise");
    assert!(invalid.is_none());

    let empty = SubscriptionPlan::get_by_id("");
    assert!(empty.is_none());
}

// ============ Subscription Tests ============

#[test]
fn test_subscription_creation() {
    let now = Utc::now();
    let end_date = now + chrono::Duration::days(30);

    let subscription = Subscription {
        id: "sub_123".to_string(),
        user_id: "user_456".to_string(),
        plan: "max".to_string(),
        status: SubscriptionStatus::Active,
        start_date: now,
        end_date,
        created_at: now,
    };

    assert_eq!(subscription.id, "sub_123");
    assert_eq!(subscription.user_id, "user_456");
    assert_eq!(subscription.plan, "max");
    assert_eq!(subscription.status, SubscriptionStatus::Active);
}

#[test]
fn test_subscription_status_all_variants() {
    let active = SubscriptionStatus::Active;
    let expired = SubscriptionStatus::Expired;
    let cancelled = SubscriptionStatus::Cancelled;

    assert_eq!(active.to_string(), "active");
    assert_eq!(expired.to_string(), "expired");
    assert_eq!(cancelled.to_string(), "cancelled");
}

// ============ Response Conversion Tests ============

#[test]
fn test_create_order_response_from_order() {
    let now = Utc::now();
    let order = Order {
        id: "order_123".to_string(),
        user_id: "user_456".to_string(),
        plan: "pro".to_string(),
        amount: 3000,
        payment_method: PaymentMethod::Alipay,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    let response = CreateOrderResponse::from(&order);

    assert_eq!(response.order_id, "order_123");
    assert_eq!(response.plan, "pro");
    assert_eq!(response.amount, 3000);
    assert_eq!(response.amount_display, "30.00");
    assert_eq!(response.payment_method, "alipay");
    assert_eq!(response.status, "pending");
}

#[test]
fn test_subscription_status_response_from_subscription() {
    let now = Utc::now();
    let end_date = now + chrono::Duration::days(30);

    let subscription = Subscription {
        id: "sub_123".to_string(),
        user_id: "user_456".to_string(),
        plan: "pro".to_string(),
        status: SubscriptionStatus::Active,
        start_date: now,
        end_date,
        created_at: now,
    };

    let response = SubscriptionStatusResponse::from(&subscription);

    assert!(response.active);
    assert_eq!(response.plan, Some("pro".to_string()));
    assert_eq!(response.status, Some("active".to_string()));
    assert!(response.start_date.is_some());
    assert!(response.end_date.is_some());
}

#[test]
fn test_subscription_status_response_inactive() {
    let now = Utc::now();
    let end_date = now - chrono::Duration::days(1);

    let subscription = Subscription {
        id: "sub_123".to_string(),
        user_id: "user_456".to_string(),
        plan: "pro".to_string(),
        status: SubscriptionStatus::Expired,
        start_date: now - chrono::Duration::days(31),
        end_date,
        created_at: now - chrono::Duration::days(31),
    };

    let response = SubscriptionStatusResponse::from(&subscription);

    assert!(!response.active);
    assert_eq!(response.status, Some("expired".to_string()));
}

#[test]
fn test_order_history_response_from_order() {
    let now = Utc::now();
    let order = Order {
        id: "order_123".to_string(),
        user_id: "user_456".to_string(),
        plan: "max".to_string(),
        amount: 5000,
        payment_method: PaymentMethod::Wechat,
        status: OrderStatus::Paid,
        trade_no: Some("wx_trade_123".to_string()),
        created_at: now,
    };

    let response = OrderHistoryResponse::from(&order);

    assert_eq!(response.id, "order_123");
    assert_eq!(response.plan, "max");
    assert_eq!(response.amount, 5000);
    assert_eq!(response.amount_display, "50.00");
    assert_eq!(response.payment_method, "wechat");
    assert_eq!(response.status, "paid");
    assert_eq!(response.trade_no, Some("wx_trade_123".to_string()));
}

// ============ Edge Cases ============

#[test]
fn test_order_amount_various_plans() {
    // Test all plan prices
    let lite = SubscriptionPlan::get_by_id("lite").unwrap();
    let pro = SubscriptionPlan::get_by_id("pro").unwrap();
    let max = SubscriptionPlan::get_by_id("max").unwrap();

    assert_eq!(lite.price, 1000); // 10.00 yuan
    assert_eq!(pro.price, 3000);  // 30.00 yuan
    assert_eq!(max.price, 5000);  // 50.00 yuan
}

#[test]
fn test_price_display_conversion() {
    let order = Order {
        id: "order_1".to_string(),
        user_id: "user_1".to_string(),
        plan: "pro".to_string(),
        amount: 3050,
        payment_method: PaymentMethod::Alipay,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: Utc::now(),
    };

    let response = CreateOrderResponse::from(&order);
    assert_eq!(response.amount_display, "30.50");

    let order2 = Order {
        id: "order_2".to_string(),
        user_id: "user_1".to_string(),
        plan: "lite".to_string(),
        amount: 999,
        payment_method: PaymentMethod::Wechat,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: Utc::now(),
    };

    let response2 = CreateOrderResponse::from(&order2);
    assert_eq!(response2.amount_display, "9.99");
}

#[test]
fn test_all_payment_methods_in_orders() {
    let now = Utc::now();

    let alipay_order = Order {
        id: "order_1".to_string(),
        user_id: "user_1".to_string(),
        plan: "pro".to_string(),
        amount: 3000,
        payment_method: PaymentMethod::Alipay,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    let wechat_order = Order {
        id: "order_2".to_string(),
        user_id: "user_1".to_string(),
        plan: "pro".to_string(),
        amount: 3000,
        payment_method: PaymentMethod::Wechat,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    let yunshanfu_order = Order {
        id: "order_3".to_string(),
        user_id: "user_1".to_string(),
        plan: "pro".to_string(),
        amount: 3000,
        payment_method: PaymentMethod::Yunshanfu,
        status: OrderStatus::Pending,
        trade_no: None,
        created_at: now,
    };

    assert_eq!(alipay_order.payment_method.to_string(), "alipay");
    assert_eq!(wechat_order.payment_method.to_string(), "wechat");
    assert_eq!(yunshanfu_order.payment_method.to_string(), "yunshanfu");
}
