use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 项目经历模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub technologies: serde_json::Value,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub link: Option<String>,
    pub repository: Option<String>,
    pub highlights: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建项目请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: String,
    pub technologies: Option<Vec<String>>,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub link: Option<String>,
    pub repository: Option<String>,
    pub highlights: Option<Vec<String>>,
}

/// 更新项目请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub duration: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: Option<bool>,
    pub link: Option<String>,
    pub repository: Option<String>,
    pub highlights: Option<Vec<String>>,
}

impl From<CreateProject> for Project {
    fn from(req: CreateProject) -> Self {
        Project {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            technologies: serde_json::to_value(req.technologies.unwrap_or_default()).unwrap_or(serde_json::json!([])),
            duration: req.duration,
            start_date: req.start_date,
            end_date: req.end_date,
            current: req.current,
            link: req.link,
            repository: req.repository,
            highlights: serde_json::to_value(req.highlights.unwrap_or_default()).unwrap_or(serde_json::json!([])),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project_from_request() {
        let req = CreateProject {
            name: "个人博客".to_string(),
            description: "个人博客".to_string(),
            technologies: Some(vec!["Rust".to_string(), "Axum".to_string(), "PostgreSQL".to_string()]),
            duration: "3个月".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            current: true,
            link: Some("https://blog.example.com".to_string()),
            repository: Some("https://github.com/example/blog".to_string()),
            highlights: Some(vec!["支持Markdown".to_string(), "响应式设计".to_string()]),
        };

        let project = Project::from(req);

        assert_eq!(project.name, "个人博客");
        assert_eq!(project.description, "个人博客");
        assert_eq!(project.technologies, serde_json::json!(["Rust", "Axum", "PostgreSQL"]));
        assert!(project.current);
    }

    #[test]
    fn test_project_serialization() {
        let project = Project {
            id: Uuid::new_v4(),
            name: "个人博客".to_string(),
            description: "的个人博客".to_string(),
            technologies: serde_json::json!(["Rust", "Axum"]),
            duration: "3个月".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            current: true,
            link: Some("https://blog.example.com".to_string()),
            repository: Some("https://github.com/example/blog".to_string()),
            highlights: serde_json::json!(["支持Markdown"]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, project.id);
        assert_eq!(deserialized.name, project.name);
        assert_eq!(deserialized.description, project.description);
    }

    #[test]
    fn test_project_with_empty_arrays() {
        let req = CreateProject {
            name: "测试项目".to_string(),
            description: "测试描述".to_string(),
            technologies: None,
            duration: "1个月".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            current: false,
            link: None,
            repository: None,
            highlights: None,
        };

        let project = Project::from(req);

        assert_eq!(project.technologies, serde_json::json!([]));
        assert_eq!(project.highlights, serde_json::json!([]));
    }
}
