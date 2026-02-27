use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::{AppError, AppResult};
use crate::models::{DeleteHistoryResponse, HistoryQuery, HistoryResponse};
use crate::services::history_service::HistoryService;
use crate::state::AppState;

pub async fn list_history_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<HistoryQuery>,
) -> AppResult<Json<HistoryResponse>> {
    let history_service = HistoryService::new(state.history.clone());
    let response = history_service.list(query).await;
    Ok(Json(response))
}

pub async fn get_history_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<crate::models::HistoryItem>> {
    let history_service = HistoryService::new(state.history.clone());

    let history = history_service
        .get(&id)
        .await
        .ok_or_else(|| AppError::NotFound(format!("History not found: {}", id)))?;

    Ok(Json(crate::models::HistoryItem {
        id: history.id,
        image_base64: history.image_base64,
        image_url: history.image_url,
        code: history.code,
        language: history.language,
        model: history.model,
        created_at: history.created_at.to_rfc3339(),
    }))
}

pub async fn delete_history_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<DeleteHistoryResponse>> {
    let history_service = HistoryService::new(state.history.clone());

    let deleted = history_service.delete(&id).await;

    if deleted {
        Ok(Json(DeleteHistoryResponse {
            success: true,
            message: "History deleted successfully".to_string(),
        }))
    } else {
        Err(AppError::NotFound(format!(
            "History not found: {}",
            id
        )))
    }
}
