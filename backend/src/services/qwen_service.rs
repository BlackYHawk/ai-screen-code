use crate::error::{AppError, AppResult};
use crate::services::ai_service::AiService;
use async_trait::async_trait;
use futures::StreamExt;
use futures::stream::{self, BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

const DEFAULT_BASE_URL: &str = "https://dashscope.aliyuncs.com/api/v1";
const DEFAULT_MODEL: &str = "qwen-vl-max";

#[derive(Debug, Serialize)]
struct QwenRequest {
    model: String,
    input: QwenInput,
    parameters: QwenParameters,
}

#[derive(Debug, Serialize)]
struct QwenInput {
    messages: Vec<QwenMessage>,
}

#[derive(Debug, Serialize)]
struct QwenMessage {
    role: String,
    content: Vec<QwenContent>,
}

#[derive(Debug, Serialize)]
struct QwenContent {
    image: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct QwenParameters {
    max_tokens: i32,
    result_format: String,
}

#[derive(Debug, Deserialize)]
struct QwenResponse {
    output: QwenOutput,
    request_id: String,
    usage: QwenUsage,
}

#[derive(Debug, Deserialize)]
struct QwenOutput {
    choices: Vec<QwenChoice>,
}

#[derive(Debug, Deserialize)]
struct QwenChoice {
    finish_reason: String,
    message: QwenResponseMessage,
}

#[derive(Debug, Deserialize)]
struct QwenResponseMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct QwenUsage {
    input_tokens: i32,
    output_tokens: i32,
}

pub struct QwenService {
    client: Client,
    default_model: String,
}

impl QwenService {
    pub fn new(model: &str) -> Self {
        let model_name = if model.is_empty() {
            DEFAULT_MODEL.to_string()
        } else {
            model.to_string()
        };
        Self {
            client: Client::new(),
            default_model: model_name,
        }
    }

    fn build_prompt(&self, language: &str) -> String {
        let lang_name = match language.to_lowercase().as_str() {
            "kotlin" => "Kotlin (Jetpack Compose)",
            "react" => "React with TypeScript",
            "swift" => "Swift (SwiftUI)",
            "vue" => "Vue 3 with TypeScript",
            _ => language,
        };

        format!(
            r#"请分析这张UI截图，并生成完整的{}代码。
要求：
1. 生成完整可运行的代码，包含所有必要的组件和样式
2. 使用现代的最佳实践
3. 代码要清晰、易读，添加适当的注释
4. 如果是Web项目，使用Tailwind CSS进行样式设计
5. 只返回代码，不要包含其他解释文字

请直接输出代码："#,
            lang_name
        )
    }
}

impl Default for QwenService {
    fn default() -> Self {
        Self::new(DEFAULT_MODEL)
    }
}

#[async_trait]
impl AiService for QwenService {
    async fn generate_code(
        &self,
        image_base64: &str,
        language: &str,
        api_key: &str,
        base_url: Option<&str>,
    ) -> AppResult<String> {
        let base_url = base_url.unwrap_or(DEFAULT_BASE_URL);
        let url = format!(
            "{}/services/aigc/multimodal-generation/generation",
            base_url
        );

        let prompt = self.build_prompt(language);
        let image_url = format!("data:image/png;base64,{}", image_base64);

        let request = QwenRequest {
            model: self.default_model.clone(),
            input: QwenInput {
                messages: vec![QwenMessage {
                    role: "user".to_string(),
                    content: vec![QwenContent {
                        image: image_url,
                        text: prompt,
                    }],
                }],
            },
            parameters: QwenParameters {
                max_tokens: 4000,
                result_format: "message".to_string(),
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiServiceError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::AiServiceError(format!(
                "API returned status {}: {}",
                status, error_text
            )));
        }

        let qwen_response: QwenResponse = response
            .json()
            .await
            .map_err(|e| AppError::AiServiceError(format!("Failed to parse response: {}", e)))?;

        let code = qwen_response
            .output
            .choices
            .first()
            .and_then(|choice| Some(choice.message.content.clone()))
            .ok_or_else(|| AppError::AiServiceError("No response content".to_string()))?;

        Ok(code)
    }

    fn generate_code_streaming(
        &self,
        image_base64: &str,
        language: &str,
        api_key: &str,
        base_url: Option<&str>,
    ) -> BoxStream<'static, AppResult<String>> {
        // Use block_on to call async generate_code
        let code = match tokio::runtime::Handle::current().block_on(self.generate_code(
            image_base64,
            language,
            api_key,
            base_url,
        )) {
            Ok(c) => c,
            Err(e) => return stream::once(async { Err(e) }).boxed(),
        };

        // Simulate streaming by splitting code into chunks
        let chunks: Vec<String> = code
            .chars()
            .collect::<Vec<char>>()
            .chunks(20)
            .map(|c: &[char]| c.iter().collect::<String>())
            .collect();

        stream::iter(chunks)
            .then(|chunk| async move {
                sleep(Duration::from_millis(30)).await;
                Ok(chunk)
            })
            .boxed()
    }

    async fn validate_api_key(&self, api_key: &str, base_url: Option<&str>) -> AppResult<bool> {
        let base_url = base_url.unwrap_or(DEFAULT_BASE_URL);
        let url = format!(
            "{}/services/aigc/multimodal-generation/generation",
            base_url
        );

        // Try a minimal request to validate the API key
        let request = QwenRequest {
            model: self.default_model.clone(),
            input: QwenInput {
                messages: vec![QwenMessage {
                    role: "user".to_string(),
                    content: vec![QwenContent {
                        image: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
                            .to_string(),
                        text: "hi".to_string(),
                    }],
                }],
            },
            parameters: QwenParameters {
                max_tokens: 1,
                result_format: "message".to_string(),
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::AiServiceError(format!("Request failed: {}", e)))?;

        // Check if response is successful
        if response.status().is_success() {
            return Ok(true);
        }

        // Try to parse error message from response
        let status = response.status();
        if status.as_u16() == 401 {
            return Err(AppError::AiServiceError(
                "API Key无效，请检查是否正确".to_string(),
            ));
        } else if status.as_u16() == 403 {
            return Err(AppError::AiServiceError("API Key没有权限".to_string()));
        } else if status.as_u16() == 429 {
            return Err(AppError::AiServiceError(
                "请求频率超限，请稍后重试".to_string(),
            ));
        } else {
            return Err(AppError::AiServiceError(format!(
                "API请求失败: HTTP {}",
                status.as_u16()
            )));
        }
    }
}
