use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 教育经历模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Education {
    pub id: Uuid,
    pub institution: String,
    pub degree: String,
    pub major: String,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub description: Option<String>,
    pub gpa: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建教育经历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEducation {
    pub institution: String,
    pub degree: String,
    pub major: String,
    pub duration: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub description: Option<String>,
    pub gpa: Option<f64>,
}

/// 更新教育经历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEducation {
    pub institution: Option<String>,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub duration: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: Option<bool>,
    pub description: Option<String>,
    pub gpa: Option<f64>,
}

impl From<CreateEducation> for Education {
    fn from(req: CreateEducation) -> Self {
        Education {
            id: Uuid::new_v4(),
            institution: req.institution,
            degree: req.degree,
            major: req.major,
            duration: req.duration,
            start_date: req.start_date,
            end_date: req.end_date,
            current: req.current,
            description: req.description,
            gpa: req.gpa.map(|g| rust_decimal::Decimal::from_f64_retain(g).unwrap_or(rust_decimal::Decimal::ZERO)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_education_from_request() {
        let req = CreateEducation {
            institution: "清华大学".to_string(),
            degree: "学士".to_string(),
            major: "计算机科学".to_string(),
            duration: "4年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2018, 9, 1).unwrap(),
            end_date: None,
            current: false,
            description: Some("主修计算机科学与技术".to_string()),
            gpa: Some(3.8),
        };

        let education = Education::from(req);

        assert_eq!(education.institution, "清华大学");
        assert_eq!(education.degree, "学士");
        assert_eq!(education.major, "计算机科学");
        assert_eq!(education.gpa, rust_decimal::Decimal::from_f64_retain(3.8));
    }

    #[test]
    fn test_education_serialization() {
        let education = Education {
            id: Uuid::new_v4(),
            institution: "清华大学".to_string(),
            degree: "学士".to_string(),
            major: "计算机科学".to_string(),
            duration: "4年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2018, 9, 1).unwrap(),
            end_date: None,
            current: false,
            description: Some("主修计算机科学与技术".to_string()),
            gpa: Some(rust_decimal::Decimal::from_f64_retain(3.8).unwrap_or(rust_decimal::Decimal::ZERO)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&education).unwrap();
        let deserialized: Education = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, education.id);
        assert_eq!(deserialized.institution, education.institution);
        assert_eq!(deserialized.degree, education.degree);
    }

    #[test]
    fn test_education_without_gpa() {
        let req = CreateEducation {
            institution: "大学".to_string(),
            degree: "硕士".to_string(),
            major: "软件工程".to_string(),
            duration: "2年".to_string(),
            start_date: NaiveDate::from_ymd_opt(2020, 9, 1).unwrap(),
            end_date: None,
            current: true,
            description: None,
            gpa: None,
        };

        let education = Education::from(req);

        assert!(education.gpa.is_none());
    }
}
