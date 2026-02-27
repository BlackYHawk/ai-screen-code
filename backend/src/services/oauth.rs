use crate::config::{OAuthConfig, OAuthProviderConfig};
use crate::error::AppError;
use crate::services::douyin_oauth::DouyinOAuthService;
use crate::services::qq_oauth::QqOAuthService;
use crate::services::wechat_oauth::WechatOAuthService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Common OAuth user info returned by all providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub openid: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

/// OAuth service trait for unified interface
#[async_trait]
pub trait OAuthProvider: Send + Sync {
    /// Generate authorization URL
    fn generate_authorization_url(&self, state: &str) -> String;

    /// Exchange code for user info
    async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError>;
}

#[async_trait]
impl OAuthProvider for QqOAuthService {
    fn generate_authorization_url(&self, state: &str) -> String {
        QqOAuthService::generate_authorization_url(self, state)
    }

    async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        QqOAuthService::get_user_from_code(self, code).await
    }
}

#[async_trait]
impl OAuthProvider for WechatOAuthService {
    fn generate_authorization_url(&self, state: &str) -> String {
        WechatOAuthService::generate_authorization_url(self, state)
    }

    async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        WechatOAuthService::get_user_from_code(self, code).await
    }
}

#[async_trait]
impl OAuthProvider for DouyinOAuthService {
    fn generate_authorization_url(&self, state: &str) -> String {
        DouyinOAuthService::generate_authorization_url(self, state)
    }

    async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        DouyinOAuthService::get_user_from_code(self, code).await
    }
}

/// OAuth service factory
pub struct OAuthServiceFactory;

impl OAuthServiceFactory {
    /// Create OAuth service for the specified provider
    pub fn create_service(provider: &str, config: &OAuthProviderConfig) -> Result<Box<dyn OAuthProvider>, AppError> {
        match provider {
            "qq" => Ok(Box::new(QqOAuthService::new(config.clone()))),
            "wechat" => Ok(Box::new(WechatOAuthService::new(config.clone()))),
            "douyin" => Ok(Box::new(DouyinOAuthService::new(config.clone()))),
            _ => Err(AppError::BadRequest(format!("Unsupported OAuth provider: {}", provider))),
        }
    }

    /// Get OAuth config for the specified provider
    pub fn get_provider_config(provider: &str, oauth_config: &OAuthConfig) -> Option<OAuthProviderConfig> {
        match provider {
            "qq" => oauth_config.qq.clone(),
            "wechat" => oauth_config.wechat.clone(),
            "douyin" => oauth_config.douyin.clone(),
            _ => None,
        }
    }
}
