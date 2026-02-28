use ai_screen_code::models::{SubscriptionPlan, SubscriptionPlanResponse};

#[test]
fn test_get_plans() {
    let plans = SubscriptionPlan::get_plans();

    assert_eq!(plans.len(), 3);

    // Verify lite plan
    let lite = &plans[0];
    assert_eq!(lite.id, "lite");
    assert_eq!(lite.name, "lite");
    assert_eq!(lite.price, 1000);
    assert_eq!(lite.features.len(), 3);
    assert!(lite.features.contains(&"基础代码生成".to_string()));

    // Verify pro plan
    let pro = &plans[1];
    assert_eq!(pro.id, "pro");
    assert_eq!(pro.price, 3000);
    assert!(pro.features.contains(&"高级代码生成".to_string()));

    // Verify max plan
    let max = &plans[2];
    assert_eq!(max.id, "max");
    assert_eq!(max.price, 5000);
    assert!(max.features.contains(&"全部功能".to_string()));
}

#[test]
fn test_get_plan_by_id() {
    let lite = SubscriptionPlan::get_by_id("lite");
    assert!(lite.is_some());
    assert_eq!(lite.unwrap().price, 1000);

    let pro = SubscriptionPlan::get_by_id("pro");
    assert!(pro.is_some());
    assert_eq!(pro.unwrap().price, 3000);

    let max = SubscriptionPlan::get_by_id("max");
    assert!(max.is_some());
    assert_eq!(max.unwrap().price, 5000);

    let invalid = SubscriptionPlan::get_by_id("invalid");
    assert!(invalid.is_none());
}

#[test]
fn test_subscription_plan_response_conversion() {
    let plan = SubscriptionPlan {
        id: "test".to_string(),
        name: "test".to_string(),
        price: 1999,
        features: vec!["feature1".to_string()],
    };

    let response: SubscriptionPlanResponse = plan.into();

    assert_eq!(response.id, "test");
    assert_eq!(response.price, 1999);
    assert_eq!(response.price_display, "19.99");
    assert_eq!(response.features.len(), 1);
}

#[test]
fn test_plan_features() {
    let plans = SubscriptionPlan::get_plans();

    // Lite plan features
    assert!(plans[0].features.contains(&"基础代码生成".to_string()));
    assert!(plans[0].features.contains(&"每日50次生成".to_string()));
    assert!(plans[0].features.contains(&"标准支持".to_string()));

    // Pro plan features
    assert!(plans[1].features.contains(&"高级代码生成".to_string()));
    assert!(plans[1].features.contains(&"每日200次生成".to_string()));
    assert!(plans[1].features.contains(&"优先支持".to_string()));
    assert!(plans[1].features.contains(&"高级模板".to_string()));

    // Max plan features
    assert!(plans[2].features.contains(&"全部功能".to_string()));
    assert!(plans[2].features.contains(&"无限次数生成".to_string()));
    assert!(plans[2].features.contains(&"7x24支持".to_string()));
    assert!(plans[2].features.contains(&"专属客服".to_string()));
    assert!(plans[2].features.contains(&"API访问".to_string()));
}

#[test]
fn test_plan_pricing() {
    let plans = SubscriptionPlan::get_plans();

    // Lite: 10 yuan (stored in cents)
    assert_eq!(plans[0].price, 1000);

    // Pro: 30 yuan
    assert_eq!(plans[1].price, 3000);

    // Max: 50 yuan
    assert_eq!(plans[2].price, 5000);
}

#[test]
fn test_subscription_plan_response_price_display() {
    // Test that the response conversion includes price display
    let plan = SubscriptionPlan {
        id: "test".to_string(),
        name: "test".to_string(),
        price: 1999,
        features: vec!["feature1".to_string()],
    };

    let response: SubscriptionPlanResponse = plan.into();

    // 1999 cents = 19.99 yuan
    assert_eq!(response.price_display, "19.99");
}
