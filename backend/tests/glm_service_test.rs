use ai_screen_code::services::glm_service::GlmService;
use ai_screen_code::services::ai_service::AiService;

mod glm_service_tests {
    use super::*;

    #[test]
    fn test_service_creation_default() {
        let service = GlmService::new("");
        let _ = service;
    }

    #[test]
    fn test_service_creation_custom_model() {
        let service = GlmService::new("glm-4v-plus");
        let _ = service;
    }

    #[test]
    fn test_service_creation_with_various_models() {
        for model in ["glm-4v", "glm-4v-plus", "glm-4-flash"] {
            let service = GlmService::new(model);
            let _ = service;
        }
    }

    #[tokio::test]
    async fn test_generate_code_network_error() {
        let service = GlmService::new("glm-4v");
        let result = service
            .generate_code("test-image-base64", "react", "test-api-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_code_different_languages() {
        let service = GlmService::new("glm-4v");

        for lang in ["react", "vue", "swift", "kotlin", "python"] {
            let result = service
                .generate_code("test-image-base64", lang, "test-api-key", Some("http://invalid:99999"))
                .await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_validate_api_key_network_error() {
        let service = GlmService::new("glm-4v");
        let result = service
            .validate_api_key("test-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

}
