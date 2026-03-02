//! 简历管理系统后端库
//!
//! 这个库提供了简历管理系统的核心功能，包括：
//! - 数据库查询操作
//! - 数据模型
//! - 配置管理
//! - CLI命令
//! - Web服务器

pub mod cli;
pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod server;
