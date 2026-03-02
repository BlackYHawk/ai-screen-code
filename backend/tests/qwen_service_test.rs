use ai_screen_code::services::ai_service::AiService;
use ai_screen_code::services::qwen_service::QwenService;

mod qwen_service_tests {
    use super::*;

    #[test]
    fn test_service_creation_default() {
        let service = QwenService::new("");
        // Service should be created with default model
        let _ = service;
    }

    #[test]
    fn test_service_creation_custom_model() {
        let service = QwenService::new("qwen-vl-plus");
        let _ = service;
    }

    #[test]
    fn test_service_creation_with_various_models() {
        // Test creating service with various model names
        for model in ["qwen-vl-max", "qwen-vl-plus", "qwen-coder-plus"] {
            let service = QwenService::new(model);
            let _ = service;
        }
    }

    #[tokio::test]
    async fn test_generate_code_network_error() {
        // Test with an invalid URL to trigger network error
        let service = QwenService::new("qwen-vl-max");
        let result = service
            .generate_code(
                "test-image-base64",
                "react",
                "test-api-key",
                Some("http://invalid:99999"),
            )
            .await;

        // Should fail due to network error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_code_different_languages() {
        // Test with various languages - should build requests correctly
        // We expect failures due to invalid endpoints, but the requests should be built
        let service = QwenService::new("qwen-vl-max");

        for lang in ["react", "vue", "swift", "kotlin", "python"] {
            let result = service
                .generate_code(
                    "test-image-base64",
                    lang,
                    "test-api-key",
                    Some("http://invalid:99999"),
                )
                .await;
            // Should fail at network level, not validation
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_validate_api_key_network_error() {
        let service = QwenService::new("qwen-vl-max");
        let result = service
            .validate_api_key("test-key", Some("http://invalid:99999"))
            .await;

        // Should fail due to network error
        assert!(result.is_err());
    }
}
