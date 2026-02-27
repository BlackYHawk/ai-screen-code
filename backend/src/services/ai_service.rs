use async_trait::async_trait;
use crate::error::AppResult;
use futures::stream::BoxStream;

#[async_trait]
pub trait AiService: Send + Sync {
    /// Generate code from image
    async fn generate_code(
        &self,
        image_base64: &str,
        language: &str,
        api_key: &str,
        base_url: Option<&str>,
    ) -> AppResult<String>;

    /// Generate code with streaming support (non-async for static lifetime)
    fn generate_code_streaming(
        &self,
        image_base64: &str,
        language: &str,
        api_key: &str,
        base_url: Option<&str>,
    ) -> BoxStream<'static, AppResult<String>>;

    /// Validate API key
    async fn validate_api_key(
        &self,
        api_key: &str,
        base_url: Option<&str>,
    ) -> AppResult<bool>;
}
