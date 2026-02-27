use crate::database::Database;
use crate::error::AppError;
use crate::models::{
    BankCard, BankCardResponse, BindCardRequest, LoginRequest, LoginResponse, RegisterRequest,
    ResetPasswordRequest, SendCodeRequest, ThirdPartyLoginRequest, UpdateProfileRequest, User,
    UserResponse, VerificationCode, VerifyCodeRequest,
};
use crate::services::oauth::OAuthServiceFactory;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::Json,
    Extension,
};
use bcrypt::{hash, verify};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: String, email: String, expires_in_secs: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: user_id,
            email,
            exp: now + expires_in_secs,
            iat: now,
        }
    }
}

/// 生成JWT Token
fn generate_token(user_id: &str, email: &str, secret: &[u8]) -> Result<String, AppError> {
    let claims = Claims::new(user_id.to_string(), email.to_string(), 7 * 24 * 60 * 60); // 7 days
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
}

/// 验证JWT Token
fn verify_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))
}

/// 从State获取数据库实例
fn get_db(state: &AppState) -> std::sync::Arc<Database> {
    // We'll add this to AppState later
    state.db.clone()
}

/// 注册处理
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Input validation
    if req.username.len() < 3 || req.username.len() > 30 {
        return Err(AppError::BadRequest(
            "Username must be between 3 and 30 characters".to_string(),
        ));
    }
    if !req.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }
    if req.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    let db = get_db(&state);

    // Check if email already exists
    if db.find_user_by_email(&req.email)?.is_some() {
        return Err(AppError::BadRequest("Email already registered".to_string()));
    }

    // Check if username already exists
    if db.find_user_by_username(&req.username)?.is_some() {
        return Err(AppError::BadRequest("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = hash(&req.password, 10)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: req.username,
        email: req.email.clone(),
        password_hash,
        nickname: None,
        avatar: None,
        provider: None,
        provider_id: None,
        provider_token: None,
        created_at: Utc::now(),
    };

    db.create_user(&user)?;

    // Generate token
    let secret = state.config.jwt_secret.as_bytes();
    let token = generate_token(&user.id, &user.email, secret)?;

    tracing::info!("User registered: {}", user.email);

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(&user),
    }))
}

/// 登录处理
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let db = get_db(&state);

    // Find user by email
    let user = db
        .find_user_by_email(&req.email)?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // Verify password
    let valid = verify(&req.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    // Generate token
    let secret = state.config.jwt_secret.as_bytes();
    let token = generate_token(&user.id, &user.email, secret)?;

    tracing::info!("User logged in: {}", user.email);

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(&user),
    }))
}

/// 获取当前用户信息
pub async fn get_profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, AppError> {
    let db = get_db(&state);

    let user = db
        .find_user_by_id(&claims.sub)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(&user)))
}

/// 更新个人信息
pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let db = get_db(&state);

    let mut user = db
        .find_user_by_id(&claims.sub)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Update fields if provided
    if let Some(nickname) = req.nickname {
        user.nickname = Some(nickname);
    }
    if let Some(avatar) = req.avatar {
        user.avatar = Some(avatar);
    }

    db.update_user(&user)?;

    tracing::info!("User profile updated: {}", user.email);

    Ok(Json(UserResponse::from(&user)))
}

/// 获取银行卡列表
pub async fn list_cards_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BankCardResponse>>, AppError> {
    let db = get_db(&state);

    let cards = db.get_user_bank_cards(&claims.sub)?;

    Ok(Json(
        cards.iter().map(BankCardResponse::from).collect(),
    ))
}

/// 绑定银行卡
pub async fn bind_card_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<BindCardRequest>,
) -> Result<Json<BankCardResponse>, AppError> {
    // Validate card number (basic validation - should be 16-19 digits)
    let card_number = req.card_number.replace(' ', "");
    if card_number.len() < 13 || card_number.len() > 19 || !card_number.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::BadRequest("Invalid card number".to_string()));
    }

    // Validate bank name
    if req.bank_name.trim().is_empty() {
        return Err(AppError::BadRequest("Bank name is required".to_string()));
    }

    // Validate card holder name
    if req.card_holder_name.trim().is_empty() {
        return Err(AppError::BadRequest("Card holder name is required".to_string()));
    }

    let db = get_db(&state);

    // Only store last 4 digits
    let last4 = card_number.chars().rev().take(4).collect::<String>().chars().rev().collect();

    let card = BankCard {
        id: Uuid::new_v4().to_string(),
        user_id: claims.sub.clone(),
        card_number_last4: last4,
        bank_name: req.bank_name,
        card_holder_name: req.card_holder_name,
        created_at: Utc::now(),
    };

    db.add_bank_card(&card)?;

    tracing::info!("Bank card added for user");

    Ok(Json(BankCardResponse::from(&card)))
}

/// 删除银行卡
pub async fn delete_card_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(card_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let db = get_db(&state);

    let deleted = db.delete_bank_card(&card_id, &claims.sub)?;

    if !deleted {
        return Err(AppError::NotFound("Bank card not found".to_string()));
    }

    tracing::info!("Bank card deleted: {}", card_id);

    Ok(Json(serde_json::json!({ "success": true })))
}

/// 发送验证码
pub async fn send_code_handler(
    State(state): State<AppState>,
    Json(req): Json<SendCodeRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 验证邮箱格式
    if !req.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    let db = get_db(&state);

    // 检查是否可以发送验证码
    if !db.can_send_verification_code(&req.email)? {
        return Err(AppError::BadRequest("Please wait 60 seconds before requesting another code".to_string()));
    }

    // 生成6位数字验证码
    let code: String = (0..6).map(|_| {
        let mut rng = rand::thread_rng();
        char::from_digit(rng.gen_range(0..10), 10).unwrap()
    }).collect();

    // 创建验证码记录
    let verification_code = VerificationCode {
        id: Uuid::new_v4().to_string(),
        email: req.email.clone(),
        code: code.clone(),
        code_type: req.code_type.clone(),
        expires_at: Utc::now() + chrono::Duration::minutes(5),
        used: false,
        created_at: Utc::now(),
    };

    db.create_verification_code(&verification_code)?;

    // 发送邮件
    if let Some(email_service) = state.email_service.as_ref() {
        email_service.send_verification_email(&req.email, &code, &req.code_type)?;
    } else {
        // 如果没有配置邮件服务，返回验证码（仅用于开发环境）
        tracing::warn!("Email service not configured, returning code in response (dev only)");
        return Ok(Json(serde_json::json!({
            "success": true,
            "message": "Verification code sent",
            "dev_code": code  // 仅开发环境使用
        })));
    }

    tracing::info!("Verification code sent to: {}", req.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Verification code sent"
    })))
}

/// 验证验证码
pub async fn verify_code_handler(
    State(state): State<AppState>,
    Json(req): Json<VerifyCodeRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let db = get_db(&state);

    // 查找有效的验证码
    let verification = db.find_valid_verification_code(&req.email, &req.code, &req.code_type)?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired verification code".to_string()))?;

    // 标记验证码已使用
    db.mark_verification_code_used(&verification.id)?;

    tracing::info!("Verification code verified for: {}", req.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Verification successful"
    })))
}

/// 三方登录
pub async fn third_party_login_handler(
    State(state): State<AppState>,
    Json(req): Json<ThirdPartyLoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let db = get_db(&state);

    // Get OAuth configuration
    let oauth_config = state
        .config
        .oauth
        .as_ref()
        .ok_or_else(|| AppError::InternalServerError("OAuth not configured".to_string()))?;

    // Get provider-specific config
    let provider_config = OAuthServiceFactory::get_provider_config(&req.provider, oauth_config)
        .ok_or_else(|| AppError::BadRequest("Provider not configured".to_string()))?;

    // Create OAuth service and exchange code for user info
    let oauth_service = OAuthServiceFactory::create_service(&req.provider, &provider_config)?;
    let user_info = oauth_service.get_user_from_code(&req.code).await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to authenticate with {}: {}", req.provider, e))
    })?;

    // Find user by provider and openid
    let user = db
        .find_user_by_provider(&req.provider, &user_info.openid)?
        .ok_or_else(|| {
            AppError::NotFound("User not found. Please bind your account first.".to_string())
        })?;

    // Generate token
    let secret = state.config.jwt_secret.as_bytes();
    let token = generate_token(&user.id, &user.email, secret)?;

    tracing::info!("Third party login: {} - {}", req.provider, user.email);

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(&user),
    }))
}

/// 三方绑定（绑定已有账号）
pub async fn third_party_bind_handler(
    State(state): State<AppState>,
    Json(req): Json<super::super::models::ThirdPartyBindRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let db = get_db(&state);

    // 验证邮箱格式
    if !req.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    // 查找用户
    let user = db.find_user_by_email(&req.email)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // 验证密码
    let valid = verify(&req.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid password".to_string()));
    }

    // Get OAuth configuration
    let oauth_config = state
        .config
        .oauth
        .as_ref()
        .ok_or_else(|| AppError::InternalServerError("OAuth not configured".to_string()))?;

    // Get provider-specific config
    let provider_config = OAuthServiceFactory::get_provider_config(&req.provider, oauth_config)
        .ok_or_else(|| AppError::BadRequest("Provider not configured".to_string()))?;

    // Create OAuth service and exchange code for user info
    let oauth_service = OAuthServiceFactory::create_service(&req.provider, &provider_config)?;
    let user_info = oauth_service.get_user_from_code(&req.code).await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to authenticate with {}: {}", req.provider, e))
    })?;

    // 更新用户绑定第三方账号
    db.update_user_provider(&user.id, &req.provider, &user_info.openid, None)?;

    tracing::info!("Third party bind: {} - {}", req.provider, req.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Account bound successfully"
    })))
}

/// 重置密码
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let db = get_db(&state);

    // 验证验证码
    let verification = db.find_valid_verification_code(&req.email, &req.code, "reset_password")?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired verification code".to_string()))?;

    // 查找用户
    let user = db.find_user_by_email(&req.email)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // 验证新密码
    if req.new_password.len() < 6 {
        return Err(AppError::BadRequest("Password must be at least 6 characters".to_string()));
    }

    // 哈希新密码
    let password_hash = hash(&req.new_password, 10)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?;

    // 更新密码
    db.update_user_password(&user.id, &password_hash)?;

    // 标记验证码已使用
    db.mark_verification_code_used(&verification.id)?;

    tracing::info!("Password reset for: {}", req.email);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Password reset successfully"
    })))
}

/// Get OAuth authorization URL for a provider
pub async fn get_oauth_url_handler(
    State(state): State<AppState>,
    Path(provider): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate provider
    if !["qq", "wechat", "douyin"].contains(&provider.as_str()) {
        return Err(AppError::BadRequest("Unsupported provider".to_string()));
    }

    // Get OAuth configuration
    let oauth_config = state
        .config
        .oauth
        .as_ref()
        .ok_or_else(|| AppError::InternalServerError("OAuth not configured".to_string()))?;

    // Get provider-specific config
    let provider_config = OAuthServiceFactory::get_provider_config(&provider, oauth_config)
        .ok_or_else(|| AppError::BadRequest("Provider not configured".to_string()))?;

    // Create OAuth service and generate authorization URL
    let oauth_service = OAuthServiceFactory::create_service(&provider, &provider_config)?;
    let state_param = Uuid::new_v4().to_string();
    let auth_url = oauth_service.generate_authorization_url(&state_param);

    tracing::info!("Generated OAuth URL for provider: {}", provider);

    Ok(Json(serde_json::json!({
        "success": true,
        "url": auth_url,
        "state": state_param
    })))
}
