use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 工作经验模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Experience {
    pub id: Uuid,
    pub company: String,
    pub position: String,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub description: serde_json::Value,
    pub technologies: serde_json::Value,
    pub achievements: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建工作经验请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperience {
    pub company: String,
    pub position: String,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub description: Option<Vec<String>>,
    pub technologies: Option<Vec<String>>,
    pub achievements: Option<Vec<String>>,
}

/// 更新工作经验请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperience {
    pub company: Option<String>,
    pub position: Option<String>,
    pub duration: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: Option<bool>,
    pub description: Option<Vec<String>>,
    pub technologies: Option<Vec<String>>,
    pub achievements: Option<Vec<String>>,
}

impl From<CreateExperience> for Experience {
    fn from(req: CreateExperience) -> Self {
        Experience {
            id: Uuid::new_v4(),
            company: req.company,
            position: req.position,
            duration: req.duration,
            start_date: req.start_date,
            end_date: req.end_date,
            current: req.current,
            description: serde_json::to_value(req.description.unwrap_or_default()).unwrap_or(serde_json::json!([])),
            technologies: serde_json::to_value(req.technologies.unwrap_or_default()).unwrap_or(serde_json::json!([])),
            achievements: serde_json::to_value(req.achievements.unwrap_or_default()).unwrap_or(serde_json::json!([])),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_experience_from_request() {
        let req = CreateExperience {
            company: "科技公司".to_string(),
            position: "高级工程师".to_string(),
            duration: "2年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            current: true,
            description: Some(vec!["负责后端开发".to_string()]),
            technologies: Some(vec!["Rust".to_string(), "PostgreSQL".to_string()]),
            achievements: Some(vec!["提升了系统性能50%".to_string()]),
        };

        let experience = Experience::from(req);

        assert_eq!(experience.company, "科技公司");
        assert_eq!(experience.position, "高级工程师");
        assert_eq!(experience.duration, "2年");
        assert!(experience.current);
        assert_eq!(experience.description, serde_json::json!(["负责后端开发"]));
    }

    #[test]
    fn test_experience_serialization() {
        let experience = Experience {
            id: Uuid::new_v4(),
            company: "科技公司".to_string(),
            position: "高级工程师".to_string(),
            duration: "2年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            current: true,
            description: serde_json::json!(["负责后端开发"]),
            technologies: serde_json::json!(["Rust", "PostgreSQL"]),
            achievements: serde_json::json!(["提升了系统性能50%"]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&experience).unwrap();
        let deserialized: Experience = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, experience.id);
        assert_eq!(deserialized.company, experience.company);
        assert_eq!(deserialized.position, experience.position);
    }

    #[test]
    fn test_experience_with_empty_arrays() {
        let req = CreateExperience {
            company: "公司".to_string(),
            position: "工程师".to_string(),
            duration: "1年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            current: false,
            description: None,
            technologies: None,
            achievements: None,
        };

        let experience = Experience::from(req);

        assert_eq!(experience.description, serde_json::json!([]));
        assert_eq!(experience.technologies, serde_json::json!([]));
        assert_eq!(experience.achievements, serde_json::json!([]));
    }
}
