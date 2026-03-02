use anyhow::Result;
use axum::{
    http::Method,
    routing::get,
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::handlers::resume::{
    create_resume, delete_resume, get_resume, health_check, list_resumes, update_resume,
};

/// 启动Web服务器
pub async fn run_server(config: &Config, host: String, port: u16) -> Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "resume-display-backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 创建数据库连接池
    let pool = create_pool(&config.database_url).await?;

    // 测试数据库连接
    crate::db::test_connection(&pool).await?;
    tracing::info!("数据库连接成功");

    // 构建路由
    let app = create_router(pool);

    // 启动服务器
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("服务器启动于 http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

/// 创建数据库连接池
async fn create_pool(database_url: &str) -> Result<PgPool> {
    crate::db::create_pool(database_url).await
}

/// 创建路由
fn create_router(pool: PgPool) -> Router {
    // CORS中间件
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        // 简历API路由
        .route("/api/v1/resume/list", get(list_resumes))
        .route(
            "/api/v1/resume",
            get(get_resume)
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
        .with_state(pool)
}
