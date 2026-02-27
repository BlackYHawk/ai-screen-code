use crate::models::{History, HistoryItem, HistoryQuery, HistoryResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HistoryService {
    state: Arc<RwLock<Vec<History>>>,
}

impl HistoryService {
    pub fn new(state: Arc<RwLock<Vec<History>>>) -> Self {
        Self { state }
    }

    pub async fn add(&self, history: History) -> History {
        let mut state = self.state.write().await;
        state.push(history.clone());
        history
    }

    pub async fn list(&self, query: HistoryQuery) -> HistoryResponse {
        let state = self.state.read().await;
        let total = state.len();

        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        let items: Vec<HistoryItem> = state
            .iter()
            .rev()
            .skip(offset)
            .take(limit)
            .map(|h| HistoryItem {
                id: h.id.clone(),
                image_base64: h.image_base64.clone(),
                image_url: h.image_url.clone(),
                code: h.code.clone(),
                language: h.language.clone(),
                model: h.model.clone(),
                created_at: h.created_at.to_rfc3339(),
            })
            .collect();

        HistoryResponse {
            success: true,
            items,
            total,
        }
    }

    pub async fn delete(&self, id: &str) -> bool {
        let mut state = self.state.write().await;
        let initial_len = state.len();
        state.retain(|h| h.id != id);
        state.len() < initial_len
    }

    pub async fn get(&self, id: &str) -> Option<History> {
        let state = self.state.read().await;
        state.iter().find(|h| h.id == id).cloned()
    }
}
