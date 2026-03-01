use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::stream::{self};
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use validator::Validate;
use crate::error::{AppError, AppResult};
use crate::models::response::ApiResponse;
use crate::models::{GenerateRequest, GenerateResponse};
use crate::services::ai_service::AiService;
use crate::services::glm_service::GlmService;
use crate::services::history_service::HistoryService;
use crate::services::kimi_service::KimiService;
use crate::services::minimax_service::MiniMaxService;
use crate::services::qwen_service::QwenService;
use crate::state::AppState;

fn get_ai_service(model: &str, model_name: &str) -> Arc<dyn AiService + Send + Sync> {
    match model.to_lowercase().as_str() {
        "qwen" => Arc::new(QwenService::new(model_name)),
        "minimax" => Arc::new(MiniMaxService::new(model_name)),
        "kimi" => Arc::new(KimiService::new(model_name)),
        "glm" => Arc::new(GlmService::new(model_name)),
        _ => Arc::new(QwenService::new(model_name)),
    }
}

pub async fn generate_code_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateRequest>,
) -> AppResult<Json<ApiResponse<GenerateResponse>>> {
    // Validate request
    payload.validate().map_err(|e: validator::ValidationErrors| AppError::ValidationError(e.to_string()))?;

    // Get configured model name from runtime config
    let model_name = state.get_model_name(&payload.model).await;

    // Get AI service with configured model
    let ai_service = get_ai_service(&payload.model, &model_name);

    // Get API key from runtime config or request
    let api_key = if let Some(ref req_key) = payload.api_key {
        if !req_key.is_empty() {
            req_key.clone()
        } else {
            state.get_api_key(&payload.model).await
        }
    } else {
        state.get_api_key(&payload.model).await
    };

    if api_key.is_empty() {
        return Err(AppError::Unauthorized(format!(
            "API key is required for model: {}. Please configure in settings or provide in request.",
            payload.model
        )));
    }

    // Get base URL from request or runtime config
    let base_url = payload.base_url.clone().unwrap_or_else(|| {
        futures::executor::block_on(state.get_base_url(&payload.model))
    });

    tracing::info!(
        "Generating code with model: {}, language: {}",
        payload.model,
        payload.language
    );

    // Generate code
    let code = ai_service
        .generate_code(
            &payload.image,
            &payload.language,
            &api_key,
            Some(&base_url),
        )
        .await?;

    // Create history record
    let history = crate::models::History::new(
        payload.image.clone(),
        code.clone(),
        payload.language.clone(),
        payload.model.clone(),
    );

    let history_service = HistoryService::new(state.history.clone());
    let saved_history = history_service.add(history).await;

    tracing::info!("Code generated successfully, id: {}", saved_history.id);

    Ok(Json(ApiResponse::success(GenerateResponse {
        code,
        language: payload.language,
        model: payload.model,
        id: saved_history.id,
    })))
}

/// Streaming code generation handler using SSE
pub async fn generate_code_streaming_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    // Validate request
    payload.validate().map_err(|e: validator::ValidationErrors| AppError::ValidationError(e.to_string()))?;

    // Get configured model name from runtime config
    let model_name = state.get_model_name(&payload.model).await;

    // Get AI service with configured model
    let ai_service = get_ai_service(&payload.model, &model_name);

    // Get API key from runtime config or request
    let api_key = if let Some(ref req_key) = payload.api_key {
        if !req_key.is_empty() {
            req_key.clone()
        } else {
            state.get_api_key(&payload.model).await
        }
    } else {
        state.get_api_key(&payload.model).await
    };

    if api_key.is_empty() {
        return Err(AppError::Unauthorized(format!(
            "API key is required for model: {}. Please configure in settings or provide in request.",
            payload.model
        )));
    }

    // Get base URL from request or runtime config
    let base_url = payload.base_url.clone().unwrap_or_else(|| {
        futures::executor::block_on(state.get_base_url(&payload.model))
    });

    tracing::info!(
        "Streaming code generation with model: {}, language: {}",
        payload.model,
        payload.language
    );

    // Clone required data for the stream
    let image = payload.image.clone();
    let language = payload.language.clone();

    // Get streaming response (non-async function)
    let mut stream = ai_service.generate_code_streaming(
        &image,
        &language,
        &api_key,
        Some(&base_url),
    );

    // Convert to SSE stream
    let mut full_code = String::new();

    let sse_stream = stream::poll_fn(move |cx| {
        match Pin::new(&mut stream).poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(chunk))) => {
                full_code.push_str(&chunk);
                std::task::Poll::Ready(Some(Ok(Event::default().data(chunk))))
            }
            std::task::Poll::Ready(Some(Err(e))) => {
                tracing::error!("Streaming error: {}", e);
                std::task::Poll::Ready(Some(Ok(Event::default().data(format!("Error: {}", e)))))
            }
            std::task::Poll::Ready(None) => {
                // Send completion event
                std::task::Poll::Ready(Some(Ok(Event::default().event("done").data(""))))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    });

    Ok(Sse::new(sse_stream))
}
