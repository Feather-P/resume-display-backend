# 简历管理系统后端

基于 Rust Axum 框架的简历管理系统后端服务，提供 REST API 和 CLI 工具。

## 功能特性

- 完整的简历 CRUD 操作
- 个人信息、工作经验、教育经历、技能、项目、证书、语言能力管理
- RESTful API 接口
- 命令行工具 (CLI)
- PostgreSQL 数据库支持
- 异步/await 模式

## 项目结构

```
src/
├── main.rs          # 主入口，包含CLI和服务器启动
├── models/          # 数据模型
│   ├── mod.rs
│   ├── resume.rs
│   ├── personal_info.rs
│   ├── experience.rs
│   ├── education.rs
│   ├── skill.rs
│   ├── project.rs
│   ├── certificate.rs
│   └── language.rs
├── db/              # 数据库操作
│   ├── mod.rs
│   ├── pool.rs
│   └── queries.rs
├── handlers/        # API处理器
│   ├── mod.rs
│   └── resume.rs
├── cli/             # CLI命令
│   ├── mod.rs
│   └── commands.rs
├── config.rs        # 配置管理
└── server.rs        # 服务器启动
```

## 环境要求

- Rust 1.70+
- PostgreSQL 12+
- Cargo

## 安装

1. 克隆项目

```bash
git clone <repository-url>
cd resume-display
```

2. 配置环境变量

```bash
cp .env.example .env
# 编辑 .env 文件，设置数据库连接信息
```

3. 运行数据库迁移

```bash
cargo run -- migrate
```

## 使用方法

### 启动 Web 服务器

```bash
# 使用默认配置
cargo run -- server

# 指定端口和主机
cargo run -- server --port 8080 --host 0.0.0.0
```

服务器将在 `http://127.0.0.1:3000` 启动（默认）。

### CLI 命令

```bash
# 列出所有简历
cargo run -- resume list

# 获取指定简历
cargo run -- resume get <uuid>

# 创建新简历
cargo run -- resume create --personal-info-id <uuid> --summary "个人简介"

# 更新简历
cargo run -- resume update <uuid> --summary "新的简介"

# 删除简历
cargo run -- resume delete <uuid>
```

## API 端点

### 健康检查

- `GET /health` - 健康检查

### 简历 API

- `GET /api/v1/resume/list` - 获取所有简历列表
- `GET /api/v1/resume` - 获取指定简历详情

### API 响应格式

成功响应：

```json
{
  "success": true,
  "data": {...},
  "message": "操作成功"
}
```

错误响应：

```json
{
  "success": false,
  "error": "错误类型",
  "message": "错误描述"
}
```

## 数据库 Schema

主要表：

- `personal_info` - 个人信息
- `experience` - 工作经验
- `education` - 教育经历
- `skill` - 技能
- `project` - 项目
- `certificate` - 证书
- `language` - 语言能力
- `resume` - 简历主表
- 关联表 - 多对多关系

## 开发

### 构建项目

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
```

### 格式化代码

```bash
cargo fmt
```

