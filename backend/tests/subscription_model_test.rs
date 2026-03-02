use ai_screen_code::models::{
    CreateOrderRequest, Order, OrderStatus, PaymentCallbackRequest, PaymentMethod, Subscription,
    SubscriptionError, SubscriptionPlan, SubscriptionStatus,
};
use chrono::Utc;

#[test]
fn test_subscription_plan_get_plans() {
    let plans = SubscriptionPlan::get_plans();

    assert_eq!(plans.len(), 3);

    // Check lite plan
    assert_eq!(plans[0].id, "lite");
    assert_eq!(plans[0].name, "lite");
    assert_eq!(plans[0].price, 1000);
    assert_eq!(plans[0].features.len(), 3);

    // Check pro plan
    assert_eq!(plans[1].id, "pro");
    assert_eq!(plans[1].name, "pro");
    assert_eq!(plans[1].price, 3000);
    assert_eq!(plans[1].features.len(), 4);

    // Check max plan
    assert_eq!(plans[2].id, "max");
    assert_eq!(plans[2].name, "max");
    assert_eq!(plans[2].price, 5000);
    assert_eq!(plans[2].features.len(), 5);
}

#[test]
fn test_subscription_status_display() {
    assert_eq!(SubscriptionStatus::Active.to_string(), "active");
    assert_eq!(SubscriptionStatus::Expired.to_string(), "expired");
    assert_eq!(SubscriptionStatus::Cancelled.to_string(), "cancelled");
}

#[test]
fn test_subscription_status_serialization() {
    let active = SubscriptionStatus::Active;
    let json = serde_json::to_string(&active).unwrap();
    assert!(json.contains("active"));

    let expired = SubscriptionStatus::Expired;
    let json = serde_json::to_string(&expired).unwrap();
    assert!(json.contains("expired"));

    let cancelled = SubscriptionStatus::Cancelled;
    let json = serde_json::to_string(&cancelled).unwrap();
    assert!(json.contains("cancelled"));
}

#[test]
fn test_payment_method_display() {
    assert_eq!(PaymentMethod::Wechat.to_string(), "wechat");
    assert_eq!(PaymentMethod::Alipay.to_string(), "alipay");
    assert_eq!(PaymentMethod::Yunshanfu.to_string(), "yunshanfu");
}

#[test]
fn test_payment_method_from_str() {
    assert_eq!(
        "wechat".parse::<PaymentMethod>().unwrap(),
        PaymentMethod::Wechat
    );
    assert_eq!(
        "alipay".parse::<PaymentMethod>().unwrap(),
        PaymentMethod::Alipay
    );
    assert_eq!(
        "yunshanfu".parse::<PaymentMethod>().unwrap(),
        PaymentMethod::Yunshanfu
    );
    assert_eq!(
        "WECHAT".parse::<PaymentMethod>().unwrap(),
        PaymentMethod::Wechat
    );
    assert_eq!(
        "ALIPAY".parse::<PaymentMethod>().unwrap(),
        PaymentMethod::Alipay
    );
}

#[test]
fn test_payment_method_from_str_invalid() {
    let result: Result<PaymentMethod, _> = "invalid".parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown payment method"));
}

#[test]
fn test_payment_method_serialization() {
    let wechat = PaymentMethod::Wechat;
    let json = serde_json::to_string(&wechat).unwrap();
    assert!(json.contains("wechat"));
}

#[test]
fn test_order_status_display() {
    assert_eq!(OrderStatus::Pending.to_string(), "pending");
    assert_eq!(OrderStatus::Paid.to_string(), "paid");
    assert_eq!(OrderStatus::Cancelled.to_string(), "cancelled");
}

#[test]
fn test_order_status_serialization() {
    let pending = OrderStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert!(json.contains("pending"));

    let paid = OrderStatus::Paid;
    let json = serde_json::to_string(&paid).unwrap();
    assert!(json.contains("paid"));

    let cancelled = OrderStatus::Cancelled;
    let json = serde_json::to_string(&cancelled).unwrap();
    assert!(json.contains("cancelled"));
}

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
    assert_eq!(order.trade_no, None);
}

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
fn test_create_order_request_serialization() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "alipay".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("pro"));
    assert!(json.contains("alipay"));

    let parsed: CreateOrderRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.plan, "pro");
    assert_eq!(parsed.payment_method, "alipay");
}

#[test]
fn test_payment_callback_request_serialization() {
    let callback = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "trade_456".to_string(),
        status: "paid".to_string(),
    };

    let json = serde_json::to_string(&callback).unwrap();
    assert!(json.contains("order_123"));
    assert!(json.contains("trade_456"));
    assert!(json.contains("paid"));

    let parsed: PaymentCallbackRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.order_id, "order_123");
    assert_eq!(parsed.trade_no, "trade_456");
    assert_eq!(parsed.status, "paid");
}

#[test]
fn test_create_order_request_validate_valid() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "alipay".to_string(),
    };

    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_create_order_request_validate_invalid_plan() {
    let request = CreateOrderRequest {
        plan: "invalid".to_string(),
        payment_method: "alipay".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SubscriptionError::InvalidPlan(_)
    ));
}

#[test]
fn test_create_order_request_validate_invalid_payment_method() {
    let request = CreateOrderRequest {
        plan: "pro".to_string(),
        payment_method: "credit_card".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SubscriptionError::InvalidPaymentMethod(_)
    ));
}

#[test]
fn test_create_order_request_validate_all_valid_plans() {
    for plan in ["lite", "pro", "max"] {
        for method in ["alipay", "wechat", "yunshanfu"] {
            let request = CreateOrderRequest {
                plan: plan.to_string(),
                payment_method: method.to_string(),
            };
            assert!(
                request.validate().is_ok(),
                "plan: {}, method: {}",
                plan,
                method
            );
        }
    }
}

#[test]
fn test_payment_callback_request_validate_valid() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "trade_456".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_payment_callback_request_validate_empty_order_id() {
    let request = PaymentCallbackRequest {
        order_id: "".to_string(),
        trade_no: "trade_456".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_payment_callback_request_validate_empty_trade_no() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "".to_string(),
        status: "paid".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_payment_callback_request_validate_invalid_status() {
    let request = PaymentCallbackRequest {
        order_id: "order_123".to_string(),
        trade_no: "trade_456".to_string(),
        status: "invalid_status".to_string(),
    };

    let result = request.validate();
    assert!(result.is_err());
}

#[test]
fn test_payment_callback_request_validate_all_valid_statuses() {
    for status in ["paid", "pending", "failed"] {
        let request = PaymentCallbackRequest {
            order_id: "order_123".to_string(),
            trade_no: "trade_456".to_string(),
            status: status.to_string(),
        };
        assert!(request.validate().is_ok(), "status: {}", status);
    }
}
