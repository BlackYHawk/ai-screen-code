use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub image_base64: String,
    pub image_url: Option<String>,
    pub code: String,
    pub language: String,
    pub model: String,
    pub created_at: DateTime<Utc>,
}

impl History {
    pub fn new(image_base64: String, code: String, language: String, model: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            image_base64,
            image_url: None,
            code,
            language,
            model,
            created_at: Utc::now(),
        }
    }
}
