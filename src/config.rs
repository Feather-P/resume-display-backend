use anyhow::Result;
use serde::Deserialize;
use std::env;

/// 应用配置
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 数据库连接URL
    pub database_url: String,
    /// 服务器主机地址
    pub server_host: String,
    /// 服务器端口
    pub server_port: u16,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost:5432/resume_db".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        })
    }
}
