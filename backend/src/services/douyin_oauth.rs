use crate::config::OAuthProviderConfig;
use crate::error::AppError;
use crate::services::oauth::OAuthUserInfo;
use reqwest::Client;
use serde::Deserialize;

/// Douyin OAuth service
pub struct DouyinOAuthService {
    config: OAuthProviderConfig,
    http_client: Client,
}

#[derive(Debug, Deserialize)]
struct DouyinAccessTokenResponse {
    data: Option<DouyinTokenData>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DouyinTokenData {
    access_token: String,
    expires_in: i64,
    openid: String,
    refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DouyinUserInfoResponse {
    data: Option<DouyinUserData>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DouyinUserData {
    open_id: String,
    nickname: Option<String>,
    avatar: Option<String>,
    union_id: Option<String>,
}

impl DouyinOAuthService {
    pub fn new(config: OAuthProviderConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Generate authorization URL
    pub fn generate_authorization_url(&self, state: &str) -> String {
        let params = vec![
            ("client_key", self.config.client_id.clone()),
            ("redirect_uri", self.config.redirect_uri.clone()),
            ("scope", self.config.scope.clone()),
            ("state", state.to_string()),
            ("response_type", "code".to_string()),
        ];

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!("https://open.douyin.com/oauth/authorize/?{}", query_string)
    }

    /// Exchange code for access token
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<DouyinTokenData, AppError> {
        let params = vec![
            ("client_key", self.config.client_id.clone()),
            ("client_secret", self.config.client_secret.clone()),
            ("code", code.to_string()),
            ("grant_type", "authorization_code".to_string()),
        ];

        let response = self
            .http_client
            .get("https://open.douyin.com/oauth/access_token/")
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to exchange code: {}", e))
            })?;

        let token_response: DouyinAccessTokenResponse = response.json().await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse token response: {}", e))
        })?;

        if let Some(message) = token_response.message {
            if message != "success" {
                return Err(AppError::InternalServerError(format!(
                    "Douyin API error: {}",
                    message
                )));
            }
        }

        token_response
            .data
            .ok_or_else(|| AppError::InternalServerError("No data in token response".to_string()))
    }

    /// Get user info using access token
    pub async fn get_user_info(
        &self,
        access_token: &str,
        openid: &str,
    ) -> Result<OAuthUserInfo, AppError> {
        let params = vec![
            ("access_token", access_token.to_string()),
            ("open_id", openid.to_string()),
        ];

        let response = self
            .http_client
            .get("https://open.douyin.com/oauth/userinfo/")
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get user info: {}", e))
            })?;

        let user_info_response: DouyinUserInfoResponse = response.json().await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse user info response: {}", e))
        })?;

        if let Some(message) = user_info_response.message {
            if message != "success" {
                return Err(AppError::InternalServerError(format!(
                    "Douyin API error: {}",
                    message
                )));
            }
        }

        let user_data = user_info_response.data.ok_or_else(|| {
            AppError::InternalServerError("No data in user info response".to_string())
        })?;

        Ok(OAuthUserInfo {
            openid: user_data.open_id,
            nickname: user_data.nickname,
            avatar: user_data.avatar,
        })
    }

    /// Complete OAuth flow: exchange code for user info
    pub async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        let token_data = self.exchange_code_for_token(code).await?;
        self.get_user_info(&token_data.access_token, &token_data.openid)
            .await
    }
}
