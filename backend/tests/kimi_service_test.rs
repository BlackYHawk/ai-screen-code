use ai_screen_code::services::kimi_service::KimiService;
use ai_screen_code::services::ai_service::AiService;

mod kimi_service_tests {
    use super::*;

    #[test]
    fn test_service_creation_default() {
        let service = KimiService::new("");
        let _ = service;
    }

    #[test]
    fn test_service_creation_custom_model() {
        let service = KimiService::new("kimi-k2.0");
        let _ = service;
    }

    #[test]
    fn test_service_creation_with_various_models() {
        for model in ["kimi-k2.5", "kimi-k2.0", "kimi-vl-max"] {
            let service = KimiService::new(model);
            let _ = service;
        }
    }

    #[tokio::test]
    async fn test_generate_code_network_error() {
        let service = KimiService::new("kimi-k2.5");
        let result = service
            .generate_code("test-image-base64", "react", "test-api-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_code_different_languages() {
        let service = KimiService::new("kimi-k2.5");

        for lang in ["react", "vue", "swift", "kotlin", "python"] {
            let result = service
                .generate_code("test-image-base64", lang, "test-api-key", Some("http://invalid:99999"))
                .await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_validate_api_key_network_error() {
        let service = KimiService::new("kimi-k2.5");
        let result = service
            .validate_api_key("test-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

}
