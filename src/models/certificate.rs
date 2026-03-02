use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 证书模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Certificate {
    pub id: Uuid,
    pub name: String,
    pub issuer: String,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建证书请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificate {
    pub name: String,
    pub issuer: String,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}

/// 更新证书请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCertificate {
    pub name: Option<String>,
    pub issuer: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}

impl From<CreateCertificate> for Certificate {
    fn from(req: CreateCertificate) -> Self {
        Certificate {
            id: Uuid::new_v4(),
            name: req.name,
            issuer: req.issuer,
            issue_date: req.issue_date,
            expiry_date: req.expiry_date,
            credential_id: req.credential_id,
            credential_url: req.credential_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_certificate_from_request() {
        let req = CreateCertificate {
            name: "AWS认证解决方案架构师".to_string(),
            issuer: "Amazon Web Services".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
            credential_id: Some("AWS-SAA-123456".to_string()),
            credential_url: Some("https://aws.amazon.com/verify".to_string()),
        };

        let certificate = Certificate::from(req);

        assert_eq!(certificate.name, "AWS认证解决方案架构师");
        assert_eq!(certificate.issuer, "Amazon Web Services");
        assert_eq!(certificate.credential_id, Some("AWS-SAA-123456".to_string()));
    }

    #[test]
    fn test_certificate_serialization() {
        let certificate = Certificate {
            id: Uuid::new_v4(),
            name: "AWS认证解决方案架构师".to_string(),
            issuer: "Amazon Web Services".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
            credential_id: Some("AWS-SAA-123456".to_string()),
            credential_url: Some("https://aws.amazon.com/verify".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&certificate).unwrap();
        let deserialized: Certificate = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, certificate.id);
        assert_eq!(deserialized.name, certificate.name);
        assert_eq!(deserialized.issuer, certificate.issuer);
    }

    #[test]
    fn test_certificate_without_optional_fields() {
        let req = CreateCertificate {
            name: "内部培训证书".to_string(),
            issuer: "公司内部".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            expiry_date: None,
            credential_id: None,
            credential_url: None,
        };

        let certificate = Certificate::from(req);

        assert!(certificate.expiry_date.is_none());
        assert!(certificate.credential_id.is_none());
        assert!(certificate.credential_url.is_none());
    }
}
