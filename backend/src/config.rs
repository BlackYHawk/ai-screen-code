use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub models: ModelsConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
    pub jwt_secret: String,
    pub email: Option<EmailConfig>,
    pub oauth: Option<OAuthConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            models: ModelsConfig::default(),
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:1420".to_string()],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
                allow_credentials: true,
            },
            jwt_secret: "test_jwt_secret_key_12345".to_string(),
            email: None,
            oauth: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelsConfig {
    pub qwen: ModelProviderConfig,
    pub minimax: ModelProviderConfig,
    pub kimi: ModelProviderConfig,
    pub glm: ModelProviderConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelProviderConfig {
    pub api_key: String,
    pub base_url: String,
    pub default_model: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

/// OAuth configuration
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OAuthConfig {
    pub qq: Option<OAuthProviderConfig>,
    pub wechat: Option<OAuthProviderConfig>,
    pub douyin: Option<OAuthProviderConfig>,
}

/// OAuth provider configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
}

impl Default for ModelsConfig {
    fn default() -> Self {
        Self {
            qwen: ModelProviderConfig {
                api_key: String::new(),
                base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
                default_model: "qwen-vl-max".to_string(),
                enabled: true,
            },
            minimax: ModelProviderConfig {
                api_key: String::new(),
                base_url: "https://api.minimax.chat/v1".to_string(),
                default_model: "MiniMax-VL01".to_string(),
                enabled: true,
            },
            kimi: ModelProviderConfig {
                api_key: String::new(),
                base_url: "https://api.moonshot.cn/v1".to_string(),
                default_model: "moonshot-v1-vision-preview".to_string(),
                enabled: true,
            },
            glm: ModelProviderConfig {
                api_key: String::new(),
                base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
                default_model: "glm-4v".to_string(),
                enabled: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::current_dir()?.join("config.yaml");

        tracing::info!("Loading configuration from: {:?}", config_path);

        let config_content = fs::read_to_string(&config_path)?;

        // 替换环境变量 ${VAR:-default}
        let config_content = substitute_env_vars(&config_content);

        let config: Config = serde_yaml::from_str(&config_content)?;

        // 检查环境变量并覆盖配置
        let config = apply_env_overrides(config);

        tracing::info!("Configuration loaded successfully");

        Ok(config)
    }
}

/// 替换配置文件中的环境变量 ${VAR:-default}
fn substitute_env_vars(content: &str) -> String {
    let mut result = content.to_string();

    // 匹配 ${VAR:-default} 模式
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let _placeholder = &result[start..start + end + 1];
            let var_part = &result[start + 2..start + end];

            // 解析变量名和默认值
            let (var_name, default_value) = if let Some(pos) = var_part.find(":-") {
                (
                    var_part[..pos].to_string(),
                    Some(var_part[pos + 2..].to_string()),
                )
            } else {
                (var_part.to_string(), None)
            };

            // 获取环境变量值
            let value = std::env::var(&var_name).ok();

            let replacement = value.or(default_value).unwrap_or_default();
            result = result[..start].to_string() + &replacement + &result[start + end + 1..];
        } else {
            break;
        }
    }

    result
}

/// 应用环境变量覆盖
fn apply_env_overrides(mut config: Config) -> Config {
    // Qwen
    if let Ok(api_key) = std::env::var("QWEN_API_KEY")
        && !api_key.is_empty()
    {
        config.models.qwen.api_key = api_key;
    }
    if let Ok(base_url) = std::env::var("QWEN_BASE_URL")
        && !base_url.is_empty()
    {
        config.models.qwen.base_url = base_url;
    }
    if let Ok(model) = std::env::var("QWEN_MODEL")
        && !model.is_empty()
    {
        config.models.qwen.default_model = model;
    }

    // MiniMax
    if let Ok(api_key) = std::env::var("MINIMAX_API_KEY")
        && !api_key.is_empty()
    {
        config.models.minimax.api_key = api_key;
    }
    if let Ok(base_url) = std::env::var("MINIMAX_BASE_URL")
        && !base_url.is_empty()
    {
        config.models.minimax.base_url = base_url;
    }
    if let Ok(model) = std::env::var("MINIMAX_MODEL")
        && !model.is_empty()
    {
        config.models.minimax.default_model = model;
    }

    // Kimi
    if let Ok(api_key) = std::env::var("KIMI_API_KEY")
        && !api_key.is_empty()
    {
        config.models.kimi.api_key = api_key;
        tracing::info!("Loaded KIMI_API_KEY from environment");
    }
    if let Ok(base_url) = std::env::var("KIMI_BASE_URL")
        && !base_url.is_empty()
    {
        config.models.kimi.base_url = base_url;
    }
    if let Ok(model) = std::env::var("KIMI_MODEL")
        && !model.is_empty()
    {
        config.models.kimi.default_model = model;
    }

    // GLM
    if let Ok(api_key) = std::env::var("GLM_API_KEY")
        && !api_key.is_empty()
    {
        config.models.glm.api_key = api_key;
    }
    if let Ok(base_url) = std::env::var("GLM_BASE_URL")
        && !base_url.is_empty()
    {
        config.models.glm.base_url = base_url;
    }
    if let Ok(model) = std::env::var("GLM_MODEL")
        && !model.is_empty()
    {
        config.models.glm.default_model = model;
    }

    // Email/SMTP Configuration
    if let Some(email_config) = config.email.take() {
        let smtp_host = std::env::var("SMTP_HOST").unwrap_or(email_config.smtp_host.clone());
        let smtp_port: u16 = std::env::var("SMTP_PORT")
            .unwrap_or_else(|_| email_config.smtp_port.to_string())
            .parse()
            .unwrap_or(email_config.smtp_port);
        let smtp_user = std::env::var("SMTP_USER").unwrap_or(email_config.smtp_user.clone());
        let smtp_password =
            std::env::var("SMTP_PASSWORD").unwrap_or(email_config.smtp_password.clone());
        let from_email = std::env::var("SMTP_FROM").unwrap_or(email_config.from_email.clone());
        let from_name = std::env::var("SMTP_FROM_NAME").unwrap_or(email_config.from_name.clone());

        if !smtp_host.is_empty() {
            config.email = Some(EmailConfig {
                smtp_host,
                smtp_port,
                smtp_user,
                smtp_password,
                from_email,
                from_name,
            });
            tracing::info!("Email/SMTP configured from environment");
        } else {
            config.email = Some(email_config);
        }
    }

    config
}

impl ModelProviderConfig {
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && self.enabled
    }
}
