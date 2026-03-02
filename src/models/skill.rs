use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 技能等级枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "skill_level_enum", rename_all = "lowercase")]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// 技能模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub level: SkillLevel,
    pub category: String,
    pub years_of_experience: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建技能请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkill {
    pub name: String,
    pub level: SkillLevel,
    pub category: String,
    pub years_of_experience: Option<f64>,
}

/// 更新技能请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSkill {
    pub name: Option<String>,
    pub level: Option<SkillLevel>,
    pub category: Option<String>,
    pub years_of_experience: Option<f64>,
}

impl From<CreateSkill> for Skill {
    fn from(req: CreateSkill) -> Self {
        Skill {
            id: Uuid::new_v4(),
            name: req.name,
            level: req.level,
            category: req.category,
            years_of_experience: req.years_of_experience.map(|y| rust_decimal::Decimal::from_f64_retain(y).unwrap_or(rust_decimal::Decimal::ZERO)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_skill_from_request() {
        let req = CreateSkill {
            name: "Rust".to_string(),
            level: SkillLevel::Advanced,
            category: "编程语言".to_string(),
            years_of_experience: Some(3.5),
        };

        let skill = Skill::from(req);

        assert_eq!(skill.name, "Rust");
        assert_eq!(skill.level, SkillLevel::Advanced);
        assert_eq!(skill.category, "编程语言");
        assert_eq!(skill.years_of_experience, rust_decimal::Decimal::from_f64_retain(3.5));
    }

    #[test]
    fn test_skill_serialization() {
        let skill = Skill {
            id: Uuid::new_v4(),
            name: "Rust".to_string(),
            level: SkillLevel::Expert,
            category: "编程语言".to_string(),
            years_of_experience: Some(rust_decimal::Decimal::from_f64_retain(5.0).unwrap_or(rust_decimal::Decimal::ZERO)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&skill).unwrap();
        let deserialized: Skill = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, skill.id);
        assert_eq!(deserialized.name, skill.name);
        assert_eq!(deserialized.level, skill.level);
    }

    #[test]
    fn test_skill_level_serialization() {
        let level = SkillLevel::Advanced;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: SkillLevel = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, level);
    }

    #[test]
    fn test_skill_without_years_of_experience() {
        let req = CreateSkill {
            name: "Python".to_string(),
            level: SkillLevel::Intermediate,
            category: "编程语言".to_string(),
            years_of_experience: None,
        };

        let skill = Skill::from(req);

        assert!(skill.years_of_experience.is_none());
    }
}
