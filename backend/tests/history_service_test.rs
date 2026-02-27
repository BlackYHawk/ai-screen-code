use ai_screen_code::models::{History, HistoryQuery, HistoryResponse};
use ai_screen_code::services::history_service::HistoryService;
use std::sync::Arc;
use tokio::sync::RwLock;

fn create_test_history_service() -> HistoryService {
    let state: Arc<RwLock<Vec<History>>> = Arc::new(RwLock::new(Vec::new()));
    HistoryService::new(state)
}

#[tokio::test]
async fn test_add_history() {
    let service = create_test_history_service();

    let history = History::new(
        "base64_image_data".to_string(),
        "<html>test code</html>".to_string(),
        "html".to_string(),
        "qwen".to_string(),
    );

    let result = service.add(history).await;

    assert!(!result.id.is_empty());
    assert_eq!(result.code, "<html>test code</html>");
    assert_eq!(result.language, "html");
    assert_eq!(result.model, "qwen");
}

#[tokio::test]
async fn test_list_history_default() {
    let service = create_test_history_service();

    // Add multiple histories
    for i in 0..5 {
        let history = History::new(
            format!("base64_image_{}", i),
            format!("<html>code {}</html>", i),
            "html".to_string(),
            "qwen".to_string(),
        );
        service.add(history).await;
    }

    let query = HistoryQuery::default();
    let result = service.list(query).await;

    assert!(result.success);
    assert_eq!(result.items.len(), 5);
    assert_eq!(result.total, 5);
    // Newest first
    assert_eq!(result.items[0].code, "<html>code 4</html>");
}

#[tokio::test]
async fn test_list_history_with_pagination() {
    let service = create_test_history_service();

    // Add 10 histories
    for i in 0..10 {
        let history = History::new(
            format!("base64_image_{}", i),
            format!("<html>code {}</html>", i),
            "html".to_string(),
            "qwen".to_string(),
        );
        service.add(history).await;
    }

    // Test pagination - first page
    let query = HistoryQuery {
        limit: Some(3),
        offset: Some(0),
    };
    let result = service.list(query).await;

    assert_eq!(result.items.len(), 3);
    assert_eq!(result.total, 10);
    assert_eq!(result.items[0].code, "<html>code 9</html>"); // newest first

    // Test second page
    let query = HistoryQuery {
        limit: Some(3),
        offset: Some(3),
    };
    let result = service.list(query).await;

    assert_eq!(result.items.len(), 3);
    assert_eq!(result.items[0].code, "<html>code 6</html>");
}

#[tokio::test]
async fn test_get_history() {
    let service = create_test_history_service();

    let history = History::new(
        "base64_image".to_string(),
        "<html>test code</html>".to_string(),
        "html".to_string(),
        "qwen".to_string(),
    );

    let saved = service.add(history).await;
    let result = service.get(&saved.id).await;

    assert!(result.is_some());
    let h = result.unwrap();
    assert_eq!(h.code, "<html>test code</html>");
}

#[tokio::test]
async fn test_get_history_not_found() {
    let service = create_test_history_service();
    let result = service.get("nonexistent_id").await;
    assert!(result.is_none());
}

#[tokio::test]
async fn test_delete_history() {
    let service = create_test_history_service();

    let history = History::new(
        "base64_image".to_string(),
        "<html>test code</html>".to_string(),
        "html".to_string(),
        "qwen".to_string(),
    );

    let saved = service.add(history).await;

    // Verify it exists
    let found = service.get(&saved.id).await;
    assert!(found.is_some());

    // Delete it
    let deleted = service.delete(&saved.id).await;
    assert!(deleted);

    // Verify it's gone
    let found = service.get(&saved.id).await;
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_history_not_found() {
    let service = create_test_history_service();
    let deleted = service.delete("nonexistent_id").await;
    assert!(!deleted);
}

#[tokio::test]
async fn test_list_empty_history() {
    let service = create_test_history_service();
    let query = HistoryQuery::default();
    let result = service.list(query).await;

    assert!(result.success);
    assert!(result.items.is_empty());
    assert_eq!(result.total, 0);
}
