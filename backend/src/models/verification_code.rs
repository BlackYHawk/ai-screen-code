pub mod verification_code {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    /// 验证码模型
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VerificationCode {
        pub id: String,
        pub email: String,
        pub code: String,
        pub code_type: String,
        pub expires_at: DateTime<Utc>,
        pub used: bool,
        pub created_at: DateTime<Utc>,
    }

    /// 发送验证码请求
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendCodeRequest {
        pub email: String,
        pub code_type: String,
    }

    /// 验证验证码请求
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VerifyCodeRequest {
        pub email: String,
        pub code: String,
        pub code_type: String,
    }

    /// 三方登录请求
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ThirdPartyLoginRequest {
        pub provider: String,
        pub code: String,
    }

    /// 三方绑定请求
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ThirdPartyBindRequest {
        pub provider: String,
        pub code: String,
        pub email: String,
        pub password: String,
    }

    /// 重置密码请求
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResetPasswordRequest {
        pub email: String,
        pub code: String,
        pub new_password: String,
    }
}
