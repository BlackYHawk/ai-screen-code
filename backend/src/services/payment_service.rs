use crate::models::{Order, OrderStatus, PaymentMethod};
use uuid::Uuid;

/// 支付服务
pub struct PaymentService;

impl PaymentService {
    /// 创建新的支付服务实例
    pub fn new() -> Self {
        Self
    }

    /// 生成模拟订单ID
    pub fn generate_order_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// 生成模拟交易号
    pub fn generate_trade_no(payment_method: &PaymentMethod) -> String {
        let prefix = match payment_method {
            PaymentMethod::Wechat => "WX",
            PaymentMethod::Alipay => "ALI",
            PaymentMethod::Yunshanfu => "YSF",
        };
        format!("{}_{}_{}", prefix, Utc::now().timestamp_millis(), Self::generate_order_id()[..8].to_string())
    }

    /// 生成支付二维码（模拟）
    pub fn generate_qr_code(order_id: &str, payment_method: &PaymentMethod) -> String {
        let prefix = match payment_method {
            PaymentMethod::Wechat => "wechat://pay/",
            PaymentMethod::Alipay => "alipay://",
            PaymentMethod::Yunshanfu => "yunshanfu://",
        };
        format!("{}mock_order_{}", prefix, order_id)
    }

    /// 生成支付链接（模拟）
    pub fn generate_payment_url(order_id: &str, payment_method: &PaymentMethod) -> String {
        let base_url = match payment_method {
            PaymentMethod::Wechat => "https://wx.tenpay.com",
            PaymentMethod::Alipay => "https://mapi.alipay.com",
            PaymentMethod::Yunshanfu => "https://payment.95516.com",
        };
        format!("{}/gateway?out_trade_no={}", base_url, order_id)
    }

    /// 生成支付页面HTML（用于模拟支付）
    pub fn generate_payment_page_html(order: &Order) -> String {
        let qr_code = Self::generate_qr_code(&order.id, &order.payment_method);
        let _payment_url = Self::generate_payment_url(&order.id, &order.payment_method);

        let method_name = match order.payment_method {
            PaymentMethod::Wechat => "微信支付",
            PaymentMethod::Alipay => "支付宝",
            PaymentMethod::Yunshanfu => "云闪付",
        };

        format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>模拟支付页面</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}
        .container {{
            background: white;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            padding: 40px;
            max-width: 420px;
            width: 100%;
        }}
        .header {{
            text-align: center;
            margin-bottom: 30px;
        }}
        .header h1 {{
            color: #333;
            font-size: 24px;
            margin-bottom: 8px;
        }}
        .header p {{
            color: #666;
            font-size: 14px;
        }}
        .order-info {{
            background: #f8f9fa;
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 24px;
        }}
        .info-row {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 12px;
            font-size: 14px;
        }}
        .info-row:last-child {{
            margin-bottom: 0;
        }}
        .info-label {{
            color: #666;
        }}
        .info-value {{
            color: #333;
            font-weight: 500;
        }}
        .amount {{
            font-size: 28px;
            color: #e74c3c;
            font-weight: 700;
        }}
        .qr-section {{
            text-align: center;
            margin-bottom: 24px;
        }}
        .qr-code {{
            width: 200px;
            height: 200px;
            margin: 0 auto 16px;
            background: #f0f0f0;
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            color: #666;
        }}
        .qr-hint {{
            font-size: 13px;
            color: #666;
        }}
        .btn-group {{
            display: flex;
            gap: 12px;
            margin-bottom: 16px;
        }}
        .btn {{
            flex: 1;
            padding: 14px 20px;
            border: none;
            border-radius: 8px;
            font-size: 15px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
        }}
        .btn-success {{
            background: #4CAF50;
            color: white;
        }}
        .btn-success:hover {{
            background: #45a049;
        }}
        .btn-cancel {{
            background: #f5f5f5;
            color: #666;
        }}
        .btn-cancel:hover {{
            background: #e0e0e0;
        }}
        .tips {{
            font-size: 12px;
            color: #999;
            text-align: center;
            line-height: 1.6;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>订单支付</h1>
            <p>订单号: {order_id}</p>
        </div>

        <div class="order-info">
            <div class="info-row">
                <span class="info-label">商品</span>
                <span class="info-value">{plan_name}</span>
            </div>
            <div class="info-row">
                <span class="info-label">支付方式</span>
                <span class="info-value">{method_name}</span>
            </div>
            <div class="info-row">
                <span class="info-label">应付金额</span>
                <span class="info-value amount">¥{amount}</span>
            </div>
        </div>

        <div class="qr-section">
            <div class="qr-code">
                <div>
                    <div>📱</div>
                    <div>扫码支付</div>
                    <div style="font-size: 10px; margin-top: 8px;">{qr_code}</div>
                </div>
            </div>
            <p class="qr-hint">请使用{method_name}扫码完成支付</p>
        </div>

        <div class="btn-group">
            <button class="btn btn-success" onclick="simulatePayment()">模拟支付成功</button>
            <button class="btn btn-cancel" onclick="cancelPayment()">取消支付</button>
        </div>

        <p class="tips">
            ⚠️ 这是一个模拟支付页面<br>
            仅用于演示和测试
        </p>
    </div>

    <script>
        function simulatePayment() {{
            fetch('/api/subscription/payment/callback', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{
                    order_id: '{order_id}',
                    trade_no: 'MOCK_{timestamp}',
                    status: 'paid'
                }})
            }})
            .then(res => res.json())
            .then(data => {{
                if (data.success) {{
                    alert('支付成功！');
                    window.location.href = '/subscription';
                }} else {{
                    alert('支付失败: ' + (data.error || '未知错误'));
                }}
            }})
            .catch(err => {{
                alert('请求失败: ' + err);
            }});
        }}

        function cancelPayment() {{
            if (confirm('确定要取消支付吗？')) {{
                window.location.href = '/subscription';
            }}
        }}
    </script>
</body>
</html>"#,
            order_id = order.id,
            plan_name = Self::get_plan_display_name(&order.plan),
            method_name = method_name,
            amount = format!("{:.2}", order.amount as f64 / 100.0),
            qr_code = qr_code,
            timestamp = Utc::now().timestamp()
        )
    }

    /// 获取套餐显示名称
    fn get_plan_display_name(plan_id: &str) -> String {
        match plan_id {
            "lite" => "Lite 基础版".to_string(),
            "pro" => "Pro 专业版".to_string(),
            "max" => "Max 旗舰版".to_string(),
            _ => plan_id.to_string(),
        }
    }

    /// 验证支付回调签名（模拟实现）
    pub fn verify_callback_signature(
        _order_id: &str,
        _trade_no: &str,
        _amount: i32,
        _sign: Option<&str>,
    ) -> bool {
        // 模拟支付不需要验证签名
        // 实际生产环境需要验证签名
        true
    }

    /// 处理支付回调
    pub fn process_callback(
        order: &Order,
        trade_no: &str,
        status: &str,
    ) -> Result<bool, String> {
        // 验证订单状态
        if order.status != OrderStatus::Pending {
            return Err(format!(
                "Order already processed: {}",
                order.status
            ));
        }

        // 验证支付状态
        if status != "paid" {
            return Err(format!(
                "Payment not successful: {}",
                status
            ));
        }

        tracing::info!(
            "Processing payment callback: order_id={}, trade_no={}, status={}",
            order.id,
            trade_no,
            status
        );

        Ok(true)
    }

    /// 刷新支付二维码（生成新的）
    pub fn refresh_qr_code(order_id: &str, payment_method: &PaymentMethod) -> String {
        Self::generate_qr_code(order_id, payment_method)
    }

    /// 获取支付方式信息
    pub fn get_payment_method_info(method: &PaymentMethod) -> PaymentMethodInfo {
        match method {
            PaymentMethod::Wechat => PaymentMethodInfo {
                name: "微信支付".to_string(),
                code: "wechat".to_string(),
                icon: "wechat".to_string(),
                min_amount: 1,
                max_amount: 50000,
            },
            PaymentMethod::Alipay => PaymentMethodInfo {
                name: "支付宝".to_string(),
                code: "alipay".to_string(),
                icon: "alipay".to_string(),
                min_amount: 1,
                max_amount: 100000,
            },
            PaymentMethod::Yunshanfu => PaymentMethodInfo {
                name: "云闪付".to_string(),
                code: "yunshanfu".to_string(),
                icon: "yunshanfu".to_string(),
                min_amount: 1,
                max_amount: 50000,
            },
        }
    }
}

impl Default for PaymentService {
    fn default() -> Self {
        Self::new()
    }
}

/// 支付方式信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodInfo {
    pub name: String,
    pub code: String,
    pub icon: String,
    pub min_amount: i32,
    pub max_amount: i32,
}

/// 订单支付信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderPaymentInfo {
    pub order_id: String,
    pub qr_code: String,
    pub payment_url: String,
    pub payment_page_html: String,
    pub expire_time: i64,
}

impl OrderPaymentInfo {
    pub fn new(order: &Order) -> Self {
        let expire_time = Utc::now().timestamp() + 900; // 15分钟有效

        Self {
            order_id: order.id.clone(),
            qr_code: PaymentService::generate_qr_code(&order.id, &order.payment_method),
            payment_url: PaymentService::generate_payment_url(&order.id, &order.payment_method),
            payment_page_html: PaymentService::generate_payment_page_html(order),
            expire_time,
        }
    }
}

// 导入 Utc
use chrono::Utc;
