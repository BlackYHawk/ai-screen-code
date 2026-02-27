use axum::{extract::State, Json};
use crate::error::AppResult;
use crate::models::{SettingsResponse, UpdateSettingsRequest, ModelRuntimeConfig};
use crate::state::AppState;

pub async fn get_settings_handler(
    State(state): State<AppState>,
) -> AppResult<Json<SettingsResponse>> {
    let config = &state.config;
    let runtime = state.runtime_config.read().await;

    let mut configured_models = Vec::new();

    // 检查配置文件中的API Key
    if config.models.qwen.is_configured() {
        configured_models.push("qwen".to_string());
    }
    if config.models.minimax.is_configured() {
        configured_models.push("minimax".to_string());
    }
    if config.models.kimi.is_configured() {
        configured_models.push("kimi".to_string());
    }
    if config.models.glm.is_configured() {
        configured_models.push("glm".to_string());
    }

    // 获取默认模型
    let default_model = runtime.models.get("qwen")
        .cloned()
        .unwrap_or_else(|| config.models.qwen.default_model.clone());

    Ok(Json(SettingsResponse {
        success: true,
        default_model,
        default_language: "react".to_string(),
        configured_models,
    }))
}

pub async fn update_settings_handler(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsRequest>,
) -> AppResult<Json<SettingsResponse>> {
    // 更新运行时配置
    if let Some(api_keys) = payload.api_keys {
        for (model, api_key) in api_keys {
            state.update_config(&model, Some(api_key), None, None).await;
        }
    }

    if let Some(base_urls) = payload.custom_base_urls {
        for (model, base_url) in base_urls {
            state.update_config(&model, None, Some(base_url), None).await;
        }
    }

    // 获取当前配置
    let runtime = state.runtime_config.read().await;
    let default_model = runtime.models.get("qwen")
        .cloned()
        .unwrap_or_else(|| "qwen".to_string());

    let mut configured_models = Vec::new();
    for (model, key) in &runtime.api_keys {
        if !key.is_empty() {
            configured_models.push(model.clone());
        }
    }

    Ok(Json(SettingsResponse {
        success: true,
        default_model,
        default_language: "react".to_string(),
        configured_models,
    }))
}

/// 获取指定模型的运行时配置
pub async fn get_model_config_handler(
    State(state): State<AppState>,
    axum::extract::Path(model): axum::extract::Path<String>,
) -> AppResult<Json<ModelRuntimeConfig>> {
    let runtime = state.runtime_config.read().await;

    let api_key = runtime.api_keys.get(&model).cloned().unwrap_or_default();
    let base_url = runtime.base_urls.get(&model).cloned().unwrap_or_else(|| {
        match model.as_str() {
            "qwen" => state.config.models.qwen.base_url.clone(),
            "minimax" => state.config.models.minimax.base_url.clone(),
            "kimi" => state.config.models.kimi.base_url.clone(),
            "glm" => state.config.models.glm.base_url.clone(),
            _ => String::new(),
        }
    });
    let default_model = runtime.models.get(&model).cloned().unwrap_or_else(|| {
        match model.as_str() {
            "qwen" => state.config.models.qwen.default_model.clone(),
            "minimax" => state.config.models.minimax.default_model.clone(),
            "kimi" => state.config.models.kimi.default_model.clone(),
            "glm" => state.config.models.glm.default_model.clone(),
            _ => String::new(),
        }
    });

    Ok(Json(ModelRuntimeConfig {
        api_key,
        base_url,
        default_model,
    }))
}

/// 更新指定模型的运行时配置
pub async fn update_model_config_handler(
    State(state): State<AppState>,
    axum::extract::Path(model): axum::extract::Path<String>,
    Json(payload): Json<ModelRuntimeConfig>,
) -> AppResult<Json<ModelRuntimeConfig>> {
    let api_key = payload.api_key.clone();
    let base_url = payload.base_url.clone();
    let default_model = payload.default_model.clone();

    state.update_config(
        &model,
        Some(api_key.clone()),
        Some(base_url.clone()),
        Some(default_model.clone()),
    ).await;

    tracing::info!("Updated model config: {} - API Key: {}, Base URL: {}, Model: {}",
        model, api_key, base_url, default_model);

    Ok(Json(payload))
}
