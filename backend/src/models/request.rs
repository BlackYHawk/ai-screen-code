use validator::Validate;

#[derive(Debug, Clone, Validate, serde::Deserialize)]
pub struct GenerateRequest {
    #[validate(length(min = 1, max = 10485760, message = "Image data is required and must be less than 10MB"))]
    pub image: String,

    #[validate(length(min = 1, message = "Model is required"))]
    pub model: String,

    #[validate(length(min = 1, message = "Language is required"))]
    pub language: String,

    #[validate(length(min = 0, message = "Invalid API key"))]
    pub api_key: Option<String>,

    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Validate, serde::Deserialize)]
pub struct ValidateModelRequest {
    #[validate(length(min = 1, message = "Model is required"))]
    pub model: String,

    #[validate(length(min = 1, message = "API key is required"))]
    pub api_key: String,

    pub base_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct HistoryQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateSettingsRequest {
    pub default_model: Option<String>,
    pub default_language: Option<String>,
    pub api_keys: Option<std::collections::HashMap<String, String>>,
    pub custom_base_urls: Option<std::collections::HashMap<String, String>>,
}
