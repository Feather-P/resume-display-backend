use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 语言熟练度枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "language_proficiency_enum", rename_all = "lowercase")]
pub enum LanguageProficiency {
    Basic,
    Conversational,
    Professional,
    Native,
}

/// 语言能力模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Language {
    pub id: Uuid,
    pub name: String,
    pub proficiency: LanguageProficiency,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建语言能力请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLanguage {
    pub name: String,
    pub proficiency: LanguageProficiency,
}

/// 更新语言能力请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLanguage {
    pub name: Option<String>,
    pub proficiency: Option<LanguageProficiency>,
}

impl From<CreateLanguage> for Language {
    fn from(req: CreateLanguage) -> Self {
        Language {
            id: Uuid::new_v4(),
            name: req.name,
            proficiency: req.proficiency,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_language_from_request() {
        let req = CreateLanguage {
            name: "英语".to_string(),
            proficiency: LanguageProficiency::Professional,
        };

        let language = Language::from(req);

        assert_eq!(language.name, "英语");
        assert_eq!(language.proficiency, LanguageProficiency::Professional);
    }

    #[test]
    fn test_language_serialization() {
        let language = Language {
            id: Uuid::new_v4(),
            name: "中文".to_string(),
            proficiency: LanguageProficiency::Native,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&language).unwrap();
        let deserialized: Language = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, language.id);
        assert_eq!(deserialized.name, language.name);
        assert_eq!(deserialized.proficiency, language.proficiency);
    }

    #[test]
    fn test_language_proficiency_serialization() {
        let proficiency = LanguageProficiency::Professional;
        let json = serde_json::to_string(&proficiency).unwrap();
        let deserialized: LanguageProficiency = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, proficiency);
    }
}
