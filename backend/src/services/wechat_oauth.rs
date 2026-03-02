use crate::config::OAuthProviderConfig;
use crate::error::AppError;
use crate::services::oauth::OAuthUserInfo;
use reqwest::Client;
use serde::Deserialize;

/// WeChat OAuth service
pub struct WechatOAuthService {
    config: OAuthProviderConfig,
    http_client: Client,
}

#[derive(Debug, Deserialize)]
struct WechatAccessTokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
    openid: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct WechatUserInfoResponse {
    openid: Option<String>,
    nickname: Option<String>,
    sex: Option<i32>,
    province: Option<String>,
    city: Option<String>,
    country: Option<String>,
    headimgurl: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

impl WechatOAuthService {
    pub fn new(config: OAuthProviderConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Generate authorization URL
    pub fn generate_authorization_url(&self, state: &str) -> String {
        let params = vec![
            ("appid", self.config.client_id.clone()),
            ("redirect_uri", self.config.redirect_uri.clone()),
            ("response_type", "code".to_string()),
            ("scope", self.config.scope.clone()),
            ("state", state.to_string()),
        ];

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!(
            "https://open.weixin.qq.com/connect/qrconnect?{}#wechat_redirect",
            query_string
        )
    }

    /// Exchange code for access token
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
    ) -> Result<WechatAccessTokenResponse, AppError> {
        let params = vec![
            ("appid", self.config.client_id.clone()),
            ("secret", self.config.client_secret.clone()),
            ("code", code.to_string()),
            ("grant_type", "authorization_code".to_string()),
        ];

        let response = self
            .http_client
            .get("https://api.weixin.qq.com/sns/oauth2/access_token")
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to exchange code: {}", e))
            })?;

        let token_response: WechatAccessTokenResponse = response.json().await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(token_response)
    }

    /// Get user info using access token and openid
    pub async fn get_user_info(
        &self,
        access_token: &str,
        openid: &str,
    ) -> Result<OAuthUserInfo, AppError> {
        let params = vec![
            ("access_token", access_token.to_string()),
            ("openid", openid.to_string()),
            ("lang", "zh_CN".to_string()),
        ];

        let response = self
            .http_client
            .get("https://api.weixin.qq.com/sns/userinfo")
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get user info: {}", e))
            })?;

        let user_info: WechatUserInfoResponse = response.json().await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse user info response: {}", e))
        })?;

        if let Some(errcode) = user_info.errcode {
            if errcode != 0 {
                return Err(AppError::InternalServerError(format!(
                    "WeChat API error: {} - {}",
                    errcode,
                    user_info.errmsg.unwrap_or_default()
                )));
            }
        }

        let avatar = user_info.headimgurl;

        Ok(OAuthUserInfo {
            openid: openid.to_string(),
            nickname: user_info.nickname,
            avatar,
        })
    }

    /// Complete OAuth flow: exchange code for user info
    pub async fn get_user_from_code(&self, code: &str) -> Result<OAuthUserInfo, AppError> {
        let token_response = self.exchange_code_for_token(code).await?;
        self.get_user_info(&token_response.access_token, &token_response.openid)
            .await
    }
}
