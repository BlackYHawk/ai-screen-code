use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub provider: Option<String>,
    pub provider_id: Option<String>,
    #[serde(skip_serializing)]
    pub provider_token: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 银行卡模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCard {
    pub id: String,
    pub user_id: String,
    pub card_number_last4: String, // 只存后4位
    pub bank_name: String,
    pub card_holder_name: String,
    pub created_at: DateTime<Utc>,
}

/// 注册请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

/// 用户响应（不包含敏感信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            nickname: user.nickname.clone(),
            avatar: user.avatar.clone(),
            created_at: user.created_at,
        }
    }
}

/// 更新个人信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

/// 绑定银行卡请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindCardRequest {
    pub card_number: String, // 完整卡号
    pub bank_name: String,
    pub card_holder_name: String,
}

/// 银行卡响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCardResponse {
    pub id: String,
    pub card_number_last4: String,
    pub bank_name: String,
    pub card_holder_name: String,
    pub created_at: DateTime<Utc>,
}

impl From<&BankCard> for BankCardResponse {
    fn from(card: &BankCard) -> Self {
        Self {
            id: card.id.clone(),
            card_number_last4: card.card_number_last4.clone(),
            bank_name: card.bank_name.clone(),
            card_holder_name: card.card_holder_name.clone(),
            created_at: card.created_at,
        }
    }
}

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
