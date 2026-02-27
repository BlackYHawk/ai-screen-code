use crate::config::Config;
use crate::database::Database;
use crate::models::History;
use crate::services::EmailService;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 运行时配置 - 可以从前端更新
#[derive(Clone, Default)]
pub struct RuntimeConfig {
    /// 各模型的API Key (从前端设置)
    pub api_keys: HashMap<String, String>,
    /// 各模型的Base URL (从前端设置)
    pub base_urls: HashMap<String, String>,
    /// 各模型的默认模型名
    pub models: HashMap<String, String>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub history: Arc<RwLock<Vec<History>>>,
    /// 运行时配置 - 可以从前端更新
    pub runtime_config: Arc<RwLock<RuntimeConfig>>,
    /// 数据库实例
    pub db: Arc<Database>,
    /// 邮件服务
    pub email_service: Arc<Option<EmailService>>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        // 从配置文件初始化运行时配置
        let mut runtime_config = RuntimeConfig::default();

        // 预填充配置文件中的值
        runtime_config.api_keys.insert("qwen".to_string(), config.models.qwen.api_key.clone());
        runtime_config.base_urls.insert("qwen".to_string(), config.models.qwen.base_url.clone());
        runtime_config.models.insert("qwen".to_string(), config.models.qwen.default_model.clone());

        runtime_config.api_keys.insert("minimax".to_string(), config.models.minimax.api_key.clone());
        runtime_config.base_urls.insert("minimax".to_string(), config.models.minimax.base_url.clone());
        runtime_config.models.insert("minimax".to_string(), config.models.minimax.default_model.clone());

        runtime_config.api_keys.insert("kimi".to_string(), config.models.kimi.api_key.clone());
        runtime_config.base_urls.insert("kimi".to_string(), config.models.kimi.base_url.clone());
        runtime_config.models.insert("kimi".to_string(), config.models.kimi.default_model.clone());

        runtime_config.api_keys.insert("glm".to_string(), config.models.glm.api_key.clone());
        runtime_config.base_urls.insert("glm".to_string(), config.models.glm.base_url.clone());
        runtime_config.models.insert("glm".to_string(), config.models.glm.default_model.clone());

        tracing::info!("Runtime config initialized from config file");

        // 初始化数据库
        let db = match Database::new() {
            Ok(db) => Arc::new(db),
            Err(e) => {
                tracing::error!("Failed to initialize database: {}", e);
                panic!("Failed to initialize database: {}", e);
            }
        };

        // 初始化邮件服务
        let email_service = config.email.as_ref().map(|email_config| {
            EmailService::new(email_config.clone())
        });

        Self {
            config,
            history: Arc::new(RwLock::new(Vec::new())),
            runtime_config: Arc::new(RwLock::new(runtime_config)),
            db,
            email_service: Arc::new(email_service),
        }
    }

    /// 获取模型的API Key (优先使用运行时配置)
    pub async fn get_api_key(&self, model: &str) -> String {
        let runtime = self.runtime_config.read().await;
        runtime.api_keys.get(model).cloned().unwrap_or_default()
    }

    /// 获取模型的Base URL
    pub async fn get_base_url(&self, model: &str) -> String {
        let runtime = self.runtime_config.read().await;
        runtime.base_urls.get(model).cloned().unwrap_or_else(|| {
            // 回退到配置文件
            match model {
                "qwen" => self.config.models.qwen.base_url.clone(),
                "minimax" => self.config.models.minimax.base_url.clone(),
                "kimi" => self.config.models.kimi.base_url.clone(),
                "glm" => self.config.models.glm.base_url.clone(),
                _ => String::new(),
            }
        })
    }

    /// 获取模型的默认名称
    pub async fn get_model_name(&self, model: &str) -> String {
        let runtime = self.runtime_config.read().await;
        runtime.models.get(model).cloned().unwrap_or_else(|| {
            // 回退到配置文件
            match model {
                "qwen" => self.config.models.qwen.default_model.clone(),
                "minimax" => self.config.models.minimax.default_model.clone(),
                "kimi" => self.config.models.kimi.default_model.clone(),
                "glm" => self.config.models.glm.default_model.clone(),
                _ => String::new(),
            }
        })
    }

    /// 更新运行时配置
    pub async fn update_config(&self, model: &str, api_key: Option<String>, base_url: Option<String>, model_name: Option<String>) {
        let mut runtime = self.runtime_config.write().await;
        if let Some(key) = api_key {
            runtime.api_keys.insert(model.to_string(), key);
        }
        if let Some(url) = base_url {
            runtime.base_urls.insert(model.to_string(), url);
        }
        if let Some(m) = model_name {
            runtime.models.insert(model.to_string(), m);
        }
        tracing::info!("Updated runtime config for model: {}", model);
    }
}
