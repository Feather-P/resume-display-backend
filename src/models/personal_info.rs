use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 个人信息模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonalInfo {
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub website: Option<String>,
    pub github: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建个人信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonalInfo {
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub website: Option<String>,
    pub github: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

/// 更新个人信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonalInfo {
    pub name: Option<String>,
    pub title: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

impl From<CreatePersonalInfo> for PersonalInfo {
    fn from(req: CreatePersonalInfo) -> Self {
        PersonalInfo {
            id: Uuid::new_v4(),
            name: req.name,
            title: req.title,
            email: req.email,
            phone: req.phone,
            location: req.location,
            website: req.website,
            github: req.github,
            avatar: req.avatar,
            bio: req.bio,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_personal_info_from_request() {
        let req = CreatePersonalInfo {
            name: "张三".to_string(),
            title: "软件工程师".to_string(),
            email: "zhangsan@example.com".to_string(),
            phone: "13800138000".to_string(),
            location: "北京".to_string(),
            website: Some("https://zhangsan.dev".to_string()),
            github: Some("https://github.com/zhangsan".to_string()),
            avatar: Some("https://avatar.example.com/zhangsan.jpg".to_string()),
            bio: Some("全栈开发工程师".to_string()),
        };

        let personal_info = PersonalInfo::from(req);

        assert_eq!(personal_info.name, "张三");
        assert_eq!(personal_info.title, "软件工程师");
        assert_eq!(personal_info.email, "zhangsan@example.com");
        assert_eq!(personal_info.phone, "13800138000");
        assert_eq!(personal_info.location, "北京");
        assert_eq!(personal_info.website, Some("https://zhangsan.dev".to_string()));
        assert_eq!(personal_info.github, Some("https://github.com/zhangsan".to_string()));
        assert_eq!(personal_info.avatar, Some("https://avatar.example.com/zhangsan.jpg".to_string()));
        assert_eq!(personal_info.bio, Some("全栈开发工程师".to_string()));
    }

    #[test]
    fn test_personal_info_serialization() {
        let personal_info = PersonalInfo {
            id: Uuid::new_v4(),
            name: "张三".to_string(),
            title: "软件工程师".to_string(),
            email: "zhangsan@example.com".to_string(),
            phone: "18000000000".to_string(),
            location: "北京".to_string(),
            website: Some("https://zhangsan.dev".to_string()),
            github: Some("https://github.com/zhangsan".to_string()),
            avatar: Some("https://avatar.example.com/zhangsan.jpg".to_string()),
            bio: Some("全栈开发工程师".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&personal_info).unwrap();
        let deserialized: PersonalInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, personal_info.id);
        assert_eq!(deserialized.name, personal_info.name);
        assert_eq!(deserialized.title, personal_info.title);
    }

    #[test]
    fn test_update_personal_info_with_none_values() {
        let update = UpdatePersonalInfo {
            name: None,
            title: None,
            email: None,
            phone: None,
            location: None,
            website: None,
            github: None,
            avatar: None,
            bio: None,
        };

        assert!(update.name.is_none());
        assert!(update.title.is_none());
    }
}
