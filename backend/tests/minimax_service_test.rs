use ai_screen_code::services::minimax_service::MiniMaxService;
use ai_screen_code::services::ai_service::AiService;

mod minimax_service_tests {
    use super::*;

    #[test]
    fn test_service_creation_default() {
        let service = MiniMaxService::new("");
        let _ = service;
    }

    #[test]
    fn test_service_creation_custom_model() {
        let service = MiniMaxService::new("MiniMax-VL02");
        let _ = service;
    }

    #[test]
    fn test_service_creation_with_various_models() {
        for model in ["MiniMax-VL01", "MiniMax-VL02", "abab6.5s-chat"] {
            let service = MiniMaxService::new(model);
            let _ = service;
        }
    }

    #[tokio::test]
    async fn test_generate_code_network_error() {
        let service = MiniMaxService::new("MiniMax-VL01");
        let result = service
            .generate_code("test-image-base64", "react", "test-api-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_code_different_languages() {
        let service = MiniMaxService::new("MiniMax-VL01");

        for lang in ["react", "vue", "swift", "kotlin", "python"] {
            let result = service
                .generate_code("test-image-base64", lang, "test-api-key", Some("http://invalid:99999"))
                .await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_validate_api_key_network_error() {
        let service = MiniMaxService::new("MiniMax-VL01");
        let result = service
            .validate_api_key("test-key", Some("http://invalid:99999"))
            .await;

        assert!(result.is_err());
    }

}
