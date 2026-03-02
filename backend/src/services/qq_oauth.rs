use crate::config::OAuthProviderConfig;
use crate::error::AppError;
use crate::services::oauth::OAuthUserInfo;
use reqwest::Client;
use serde::Deserialize;

/// QQ OAuth service
pub struct QqOAuthService {
    config: OAuthProviderConfig,
    http_client: Client,
}

#[derive(Debug, Deserialize)]
struct QqAccessTokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QqOpenIdResponse {
    client_id: Option<String>,
    openid: String,
}

#[derive(Debug, Deserialize)]
struct QqUserInfoResponse {
    ret: i32,
    msg: Option<String>,
    nickname: Option<String>,
    figureurl: Option<String>,
    figureurl_qq_2: Option<String>,
}

impl QqOAuthService {
    pub fn new(config: OAuthProviderConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Generate authorization URL
    pub fn generate_authorization_url(&self, state: &str) -> String {
        let params = [("response_type", "code".to_string()),
            ("client_id", self.config.client_id.clone()),
            ("redirect_uri", self.config.redirect_uri.clone()),
            ("scope", self.config.scope.clone()),
            ("state", state.to_string())];

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!("https://graph.qq.com/oauth2.0/authorize?{}", query_string)
    }

    /// Exchange code for access token
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<String, AppError> {
        let params = vec![
            ("grant_type", "authorization_code".to_string()),
            ("client_id", self.config.client_id.clone()),
            ("client_secret", self.config.client_secret.clone()),
            ("code", code.to_string()),
            ("redirect_uri", self.config.redirect_uri.clone()),
        ];

        let response = self
            .http_client
            .get("https://graph.qq.com/oauth2.0/token")
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to exchange code: {}", e))
            })?;

        let token_response: QqAccessTokenResponse =
            serde_urlencoded::from_str(&response.text().await.map_err(|e| {
                AppError::InternalServerError(format!("Failed to read response: {}", e))
            })?)
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to parse token response: {}", e))
            })?;

        Ok(token_response.access_token)
    }

    /// Get OpenID (unique user identifier)
    pub async fn get_openid(&self, access_token: &str) -> Result<String, AppError> {
        let response = self
            .http_client
            .get("https://graph.qq.com/oauth2.0/me")
            .query(&[("access_token", access_token), ("unionid", "0")])
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get openid: {}", e)))?;

        let openid_response: QqOpenIdResponse =
            serde_json::from_str(&response.text().await.map_err(|e| {
                AppError::InternalServerError(format!("Failed to read response: {}", e))
            })?)
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to parse openid response: {}", e))
            })?;

        Ok(openid_response.openid)
    }

    /// Get user info
    pub async fn get_user_info(&self, access_token: &str) -> Result<OAuthUserInfo, AppError> {
        let openid = self.get_openid(access_token).await?;

        let response = self
            .http_client
            .get("https://graph.qq.com/user/get_user_info")
            .query(&[
                ("access_token", access_token),
                ("oauth_consumer_key", &self.config.client_id),
                ("openid", &openid),
            ])
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get user info: {}", e))
            })?;

        let user_info: QqUserInfoResponse = response.json().await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse user info response: {}", e))
        })?;

        if user_info.ret != 0 {
            return Err(AppError::InternalServerError(format!(
                "QQ API error: {}",
                user_info.msg.unwrap_or_default()
            )));
        }

        let avatar = user_info.figureurl_qq_2.or(user_info.figureurl);

        Ok(OAuthUserInfo {
            openid,
            nickname: user_info.nickname,
            avatar,
        })
    }

    /// Complete OAuth flow: exchange code for user info
    pub async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        let access_token = self.exchange_code_for_token(code).await?;
        self.get_user_info(&access_token).await
    }
}
