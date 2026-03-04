// 加载 .env 文件中的环境变量

use ai_screen_code::config::Config;
use ai_screen_code::handlers::{
    bind_card_handler, create_order_handler, delete_card_handler, delete_history_handler,
    generate_code_handler, generate_code_streaming_handler, get_history_handler,
    get_model_config_handler, get_oauth_url_handler, get_order_history_handler,
    get_order_status_handler, get_plans_handler, get_profile_handler, get_settings_handler,
    get_subscription_status_handler, list_cards_handler, list_history_handler, list_models_handler,
    login_handler, payment_callback_handler, register_handler, reset_password_handler,
    send_code_handler, third_party_bind_handler, third_party_login_handler,
    update_model_config_handler, update_profile_handler, update_settings_handler,
    upload_avatar_handler, validate_model_handler, verify_code_handler,
};
use ai_screen_code::middleware::auth_middleware;
use ai_screen_code::state::AppState;
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Root endpoint
async fn root() -> &'static str {
    "AI Screen Code Backend API"
}

#[tokio::main]
async fn main() {
    // 加载 .env 文件中的环境变量 (从项目根目录)
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|p| p.join(".env"))
        .unwrap_or_else(|| std::path::PathBuf::from(".env"));
    if dotenv::from_path(&project_root).is_ok() {
        println!("Loaded .env from {:?}", project_root);
    } else {
        println!("Failed to load .env from {:?}", project_root);
    }

    // 检查 MINIMAX_API_KEY 是否加载
    if let Ok(key) = std::env::var("MINIMAX_API_KEY") {
        println!("MINIMAX_API_KEY loaded: {} chars", key.len());
    } else {
        println!("MINIMAX_API_KEY NOT loaded");
    }

    // 检查 SMTP_HOST 是否加载
    if let Ok(host) = std::env::var("SMTP_HOST") {
        println!("SMTP_HOST loaded: {}", host);
    } else {
        println!("SMTP_HOST NOT loaded");
    }

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_screen_code=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AI Screen Code Backend");

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Get port before moving config into app_state
    let port = config.server.port;

    // Create application state
    let app_state = AppState::new(config);

    // Configure CORS - allow GitHub Pages and local development
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "http://localhost:3000"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "https://blackyhawk.github.io"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "https://blackyhawk.github.io/ai-screen-code/"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers([
            axum::http::HeaderName::from_static("content-type"),
            axum::http::HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true);

    // Build router - separate public and protected routes
    let public_routes = Router::new()
        // Root and health
        .route("/", get(root))
        .route("/health", get(health_check))
        // Auth routes (public)
        .route("/api/v1/auth/register", post(register_handler))
        .route("/api/v1/auth/login", post(login_handler))
        // Verification code routes
        .route("/api/v1/auth/code/send", post(send_code_handler))
        .route("/api/v1/auth/code/verify", post(verify_code_handler))
        // Third party login routes
        .route(
            "/api/v1/auth/third-party/login",
            post(third_party_login_handler),
        )
        .route(
            "/api/v1/auth/third-party/bind",
            post(third_party_bind_handler),
        )
        .route(
            "/api/v1/auth/third-party/url/:provider",
            get(get_oauth_url_handler),
        )
        // Password reset route
        .route("/api/v1/auth/password/reset", post(reset_password_handler))
        // API v1 routes (public)
        .route("/api/v1/generate", post(generate_code_handler))
        .route(
            "/api/v1/generate/stream",
            post(generate_code_streaming_handler),
        )
        .route("/api/v1/models", get(list_models_handler))
        .route("/api/v1/models/validate", post(validate_model_handler))
        // Model config routes
        .route("/api/v1/models/:model", get(get_model_config_handler))
        .route("/api/v1/models/:model", post(update_model_config_handler))
        // History routes (public)
        .route("/api/v1/history", get(list_history_handler))
        .route("/api/v1/history/:id", get(get_history_handler))
        .route("/api/v1/history/:id", delete(delete_history_handler))
        // Settings routes (public)
        .route("/api/v1/settings", get(get_settings_handler))
        .route("/api/v1/settings", post(update_settings_handler))
        // Subscription routes (public)
        .route("/api/v1/subscriptions/plans", get(get_plans_handler))
        .route(
            "/api/v1/subscriptions/webhook",
            post(payment_callback_handler),
        )
        .route(
            "/api/v1/subscriptions/orders/:order_id",
            get(get_order_status_handler),
        );

    // Protected routes with auth middleware
    let jwt_secret = Arc::new(app_state.config.jwt_secret.clone());
    let protected_routes = Router::new()
        .route("/api/v1/auth/profile", get(get_profile_handler))
        .route("/api/v1/auth/profile", put(update_profile_handler))
        .route("/api/v1/auth/avatar", post(upload_avatar_handler))
        .route("/api/v1/auth/cards", get(list_cards_handler))
        .route("/api/v1/auth/cards", post(bind_card_handler))
        .route("/api/v1/auth/cards/:id", delete(delete_card_handler))
        // Subscription routes (need auth)
        .route(
            "/api/v1/subscriptions/status",
            get(get_subscription_status_handler),
        )
        .route("/api/v1/subscriptions/create", post(create_order_handler))
        .route(
            "/api/v1/subscriptions/orders",
            get(get_order_history_handler),
        )
        .layer(middleware::from_fn_with_state(jwt_secret, auth_middleware));

    // Merge routes
    let app = public_routes
        .merge(protected_routes)
        .layer(cors)
        .with_state(app_state)
        // Serve static files (avatars)
        .fallback_service(ServeDir::new("static"));

    // Get server address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
