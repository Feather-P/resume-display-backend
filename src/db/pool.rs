use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// 创建数据库连接池
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}

/// 运行数据库迁移（从SQL文件）
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    // 读取迁移SQL文件并执行
    let sql = std::fs::read_to_string("migrations/002_single_resume_table.sql")?;
    // 使用 raw_sql 执行多条 SQL 语句（非准备模式）
    sqlx::raw_sql(&sql).execute(pool).await?;
    Ok(())
}

/// 测试数据库连接
pub async fn test_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}