use crate::error::{AppError, AppResult};
use crate::models::response::ApiResponse;
use crate::models::{DeleteHistoryResponse, HistoryItem, HistoryQuery, HistoryResponse};
use crate::services::history_service::HistoryService;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list_history_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<HistoryQuery>,
) -> AppResult<Json<ApiResponse<HistoryResponse>>> {
    let history_service = HistoryService::new(state.history.clone());
    let response = history_service.list(query).await;
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_history_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<HistoryItem>>> {
    let history_service = HistoryService::new(state.history.clone());

    let history = history_service
        .get(&id)
        .await
        .ok_or_else(|| AppError::NotFound(format!("History not found: {}", id)))?;

    let item = HistoryItem {
        id: history.id,
        image_base64: history.image_base64,
        image_url: history.image_url,
        code: history.code,
        language: history.language,
        model: history.model,
        created_at: history.created_at.to_rfc3339(),
    };
    Ok(Json(ApiResponse::success(item)))
}

pub async fn delete_history_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<DeleteHistoryResponse>>> {
    let history_service = HistoryService::new(state.history.clone());

    let deleted = history_service.delete(&id).await;

    if deleted {
        Ok(Json(ApiResponse::success(DeleteHistoryResponse {
            success: true,
            message: "History deleted successfully".to_string(),
        })))
    } else {
        Err(AppError::NotFound(format!("History not found: {}", id)))
    }
}
