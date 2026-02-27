use crate::config::EmailConfig;
use crate::error::AppError;
use lettre::message::{Mailbox, Message, header, MultiPart, SinglePart};
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::client::TlsParameters;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Transport;

pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    /// 构建 SMTP 传输层
    fn build_transport(&self) -> Result<SmtpTransport, AppError> {
        let smtp_host = &self.config.smtp_host;
        let smtp_port = self.config.smtp_port;
        let smtp_user = &self.config.smtp_user;
        let smtp_password = &self.config.smtp_password;

        // 构建 TLS 参数
        let tls_params = TlsParameters::new(smtp_host.clone())
            .map_err(|e| AppError::Internal(format!("Failed to build TLS parameters: {}", e)))?;

        // 根据端口选择 TLS 模式
        let tls_mode = match smtp_port {
            // Port 465: 使用 SSL 直接连接 (Wrapper 模式)
            465 => lettre::transport::smtp::client::Tls::Wrapper(tls_params),
            // Port 587: 使用 STARTTLS (Required 模式)
            _ => lettre::transport::smtp::client::Tls::Required(tls_params),
        };

        // 构建传输层
        let transport = SmtpTransport::builder_dangerous(&format!("{}:{}", smtp_host, smtp_port))
            .tls(tls_mode)
            .credentials(Credentials::new(smtp_user.clone(), smtp_password.clone()))
            .build();

        Ok(transport)
    }

    /// 构建邮件
    fn build_message(&self, to_email: &str, subject: &str, html_body: &str) -> Result<Message, AppError> {
        let from_email = &self.config.from_email;
        let from_name = &self.config.from_name;

        let from_mailbox = format!("{} <{}>", from_name, from_email)
            .parse::<Mailbox>()
            .map_err(|e| AppError::Internal(format!("Invalid from email: {}", e)))?;

        let to_mailbox = to_email
            .parse::<Mailbox>()
            .map_err(|e| AppError::Internal(format!("Invalid to email: {}", e)))?;

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body("您的验证码为: 请在网页中查看完整邮件获取验证码".to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .map_err(|e| AppError::Internal(format!("Failed to build email: {}", e)))?;

        Ok(email)
    }

    /// 生成 HTML 邮件内容
    fn generate_html_content(&self, code: &str, code_type: &str) -> String {
        let title = match code_type {
            "register" => "注册验证码",
            "login" => "登录验证码",
            "reset_password" => "重置密码验证码",
            "bind_email" => "绑定邮箱验证码",
            _ => "验证码",
        };

        format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background-color: #f5f5f5;
            margin: 0;
            padding: 20px;
        }}
        .container {{
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }}
        .header {{
            background-color: #4F46E5;
            color: #ffffff;
            padding: 30px 20px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            font-size: 24px;
            font-weight: 600;
        }}
        .content {{
            padding: 40px 30px;
        }}
        .code-box {{
            background-color: #F3F4F6;
            border-radius: 8px;
            padding: 24px;
            text-align: center;
            margin: 30px 0;
        }}
        .code {{
            font-size: 36px;
            font-weight: 700;
            color: #4F46E5;
            letter-spacing: 8px;
            font-family: 'Courier New', monospace;
        }}
        .tips {{
            color: #6B7280;
            font-size: 14px;
            line-height: 1.6;
        }}
        .footer {{
            background-color: #F9FAFB;
            padding: 20px;
            text-align: center;
            color: #9CA3AF;
            font-size: 12px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{title}</h1>
        </div>
        <div class="content">
            <p class="tips">您好，</p>
            <p class="tips">您的验证码如下，请尽快完成验证:</p>
            <div class="code-box">
                <div class="code">{code}</div>
            </div>
            <p class="tips">验证码有效期为 10 分钟，请勿泄露给他人。</p>
            <p class="tips">如果这不是您的操作，请忽略此邮件。</p>
        </div>
        <div class="footer">
            <p>此邮件由系统自动发送，请勿回复</p>
        </div>
    </div>
</body>
</html>"#,
            title = title,
            code = code
        )
    }

    /// 发送验证码邮件
    pub fn send_verification_email(&self, to_email: &str, code: &str, code_type: &str) -> Result<(), AppError> {
        tracing::info!("Sending verification email to: {} with code: {} (type: {})", to_email, code, code_type);

        // 构建 SMTP 传输
        let transport = self.build_transport()?;

        // 生成邮件主题和内容
        let subject = match code_type {
            "register" => "注册验证码",
            "login" => "登录验证码",
            "reset_password" => "重置密码验证码",
            "bind_email" => "绑定邮箱验证码",
            _ => "验证码",
        };

        let html_content = self.generate_html_content(code, code_type);

        // 构建邮件
        let email = self.build_message(to_email, subject, &html_content)?;

        // 发送邮件
        transport
            .send(&email)
            .map_err(|e| AppError::Internal(format!("Failed to send email: {}", e)))?;

        tracing::info!("Verification email sent successfully to: {}", to_email);

        Ok(())
    }
}
