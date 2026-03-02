use crate::error::{AppError, AppResult};
use crate::models::response::ApiResponse;
use crate::models::{ModelInfo, ModelsResponse, ValidateModelRequest, ValidateModelResponse};
use crate::services::ai_service::AiService;
use crate::services::glm_service::GlmService;
use crate::services::kimi_service::KimiService;
use crate::services::minimax_service::MiniMaxService;
use crate::services::qwen_service::QwenService;
use crate::state::AppState;
use axum::{Json, extract::State};
use validator::Validate;

fn get_ai_service(model: &str, model_name: &str) -> Box<dyn AiService> {
    match model.to_lowercase().as_str() {
        "qwen" => Box::new(QwenService::new(model_name)),
        "minimax" => Box::new(MiniMaxService::new(model_name)),
        "kimi" => Box::new(KimiService::new(model_name)),
        "glm" => Box::new(GlmService::new(model_name)),
        _ => Box::new(QwenService::new(model_name)),
    }
}

pub async fn list_models_handler(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<ModelsResponse>>> {
    let config = &state.config;

    let models = vec![
        ModelInfo {
            id: "qwen".to_string(),
            name: "Qwen VL".to_string(),
            provider: "Alibaba Cloud".to_string(),
            enabled: config.models.qwen.enabled,
            is_configured: config.models.qwen.is_configured(),
        },
        ModelInfo {
            id: "minimax".to_string(),
            name: "MiniMax VL".to_string(),
            provider: "MiniMax".to_string(),
            enabled: config.models.minimax.enabled,
            is_configured: config.models.minimax.is_configured(),
        },
        ModelInfo {
            id: "kimi".to_string(),
            name: "Kimi VL".to_string(),
            provider: "Moonshot AI".to_string(),
            enabled: config.models.kimi.enabled,
            is_configured: config.models.kimi.is_configured(),
        },
        ModelInfo {
            id: "glm".to_string(),
            name: "GLM-4V".to_string(),
            provider: "Zhipu AI".to_string(),
            enabled: config.models.glm.enabled,
            is_configured: config.models.glm.is_configured(),
        },
    ];

    Ok(Json(ApiResponse::success(ModelsResponse {
        success: true,
        models,
    })))
}

pub async fn validate_model_handler(
    State(state): State<AppState>,
    Json(payload): Json<ValidateModelRequest>,
) -> AppResult<Json<ApiResponse<ValidateModelResponse>>> {
    payload
        .validate()
        .map_err(|e: validator::ValidationErrors| AppError::ValidationError(e.to_string()))?;

    let ai_service = get_ai_service(&payload.model, "");

    // 获取base_url
    let base_url = payload
        .base_url
        .clone()
        .unwrap_or_else(|| futures::executor::block_on(state.get_base_url(&payload.model)));

    let result = ai_service
        .validate_api_key(&payload.api_key, Some(&base_url))
        .await;

    let (is_valid, message) = match result {
        Ok(true) => {
            // 验证成功后更新运行时配置
            state
                .update_config(
                    &payload.model,
                    Some(payload.api_key.clone()),
                    Some(base_url.clone()),
                    None,
                )
                .await;
            (true, format!("API Key验证成功: {}", payload.model))
        }
        Ok(false) => (false, format!("API Key无效: {}", payload.model)),
        Err(e) => {
            let error_msg = match e {
                AppError::AiServiceError(msg) => msg,
                _ => e.to_string(),
            };
            (false, error_msg)
        }
    };

    Ok(Json(ApiResponse::success(ValidateModelResponse {
        success: true,
        valid: is_valid,
        message,
    })))
}
