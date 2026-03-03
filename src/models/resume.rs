use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use super::{Certificate, Education, Experience, Language, PersonalInfo, Project, Skill};
use rust_decimal::Decimal;

// =====================================================
// 单一表架构模型
// =====================================================

/// 简历单一表模型 - 对应 resume_single 表
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResumeSingle {
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
    pub summary: Option<String>,
    pub experience: Value,
    pub education: Value,
    pub skills: Value,
    pub projects: Value,
    pub certificates: Value,
    pub languages: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// 创建简历请求（单一表架构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResumeSingle {
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub website: Option<String>,
    pub github: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub summary: Option<String>,
    pub experience: Option<Value>,
    pub education: Option<Value>,
    pub skills: Option<Value>,
    pub projects: Option<Value>,
    pub certificates: Option<Value>,
    pub languages: Option<Value>,
}

impl CreateResumeSingle {
    pub fn new(name: String, title: String, email: String, phone: String, location: String) -> Self {
        Self {
            name,
            title,
            email,
            phone,
            location,
            website: None,
            github: None,
            avatar: None,
            bio: None,
            summary: None,
            experience: Some(Value::Array(vec![])),
            education: Some(Value::Array(vec![])),
            skills: Some(Value::Array(vec![])),
            projects: Some(Value::Array(vec![])),
            certificates: Some(Value::Array(vec![])),
            languages: Some(Value::Array(vec![])),
        }
    }
}

/// 更新简历请求（单一表架构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResumeRequest {
    pub name: Option<String>,
    pub title: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub summary: Option<String>,
    pub experience: Option<Value>,
    pub education: Option<Value>,
    pub skills: Option<Value>,
    pub projects: Option<Value>,
    pub certificates: Option<Value>,
    pub languages: Option<Value>,
}

// =====================================================
// 响应模型（camelCase 格式）
// =====================================================

/// 个人信息响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfoResponse {
    pub id: String,
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

impl From<ResumeSingle> for PersonalInfoResponse {
    fn from(resume: ResumeSingle) -> Self {
        Self {
            id: resume.id.to_string(),
            name: resume.name,
            title: resume.title,
            email: resume.email,
            phone: resume.phone,
            location: resume.location,
            website: resume.website,
            github: resume.github,
            avatar: resume.avatar,
            bio: resume.bio,
        }
    }
}

/// 工作经验响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceResponse {
    pub id: String,
    pub company: String,
    pub position: String,
    pub duration: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub description: Vec<String>,
    pub technologies: Vec<String>,
}

/// 教育经历响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationResponse {
    pub id: String,
    pub institution: String,
    pub degree: String,
    pub major: String,
    pub duration: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub gpa: Option<String>,
    pub description: Option<String>,
}

/// 技能响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillResponse {
    pub id: String,
    pub name: String,
    pub level: String,
    pub category: String,
    #[serde(rename = "yearsOfExperience")]
    pub years_of_experience: Option<f64>,
}

/// 项目响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub duration: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub highlights: Vec<String>,
}

/// 证书响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateResponse {
    pub id: String,
    pub name: String,
    pub issuer: String,
    #[serde(rename = "issueDate")]
    pub issue_date: String,
}

/// 语言能力响应（camelCase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageResponse {
    pub id: String,
    pub name: String,
    pub proficiency: String,
}

/// 完整简历响应（单一表架构，camelCase 格式）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeSingleResponse {
    pub id: String,
    #[serde(rename = "personalInfo")]
    pub personal_info: PersonalInfoResponse,
    pub summary: Option<String>,
    pub experience: Vec<ExperienceResponse>,
    pub education: Vec<EducationResponse>,
    pub skills: Vec<SkillResponse>,
    pub projects: Vec<ProjectResponse>,
    pub certificates: Vec<CertificateResponse>,
    pub languages: Vec<LanguageResponse>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

impl TryFrom<ResumeSingle> for ResumeSingleResponse {
    type Error = serde_json::Error;

    fn try_from(resume: ResumeSingle) -> Result<Self, Self::Error> {
        // 先构建 PersonalInfoResponse（在移动字段之前）
        let personal_info = PersonalInfoResponse {
            id: resume.id.to_string(),
            name: resume.name.clone(),
            title: resume.title.clone(),
            email: resume.email.clone(),
            phone: resume.phone.clone(),
            location: resume.location.clone(),
            website: resume.website.clone(),
            github: resume.github.clone(),
            avatar: resume.avatar.clone(),
            bio: resume.bio.clone(),
        };

        // 解析 JSONB 字段
        let experience: Vec<ExperienceResponse> = serde_json::from_value(resume.experience).unwrap_or_default();
        let education: Vec<EducationResponse> = serde_json::from_value(resume.education).unwrap_or_default();
        let skills: Vec<SkillResponse> = serde_json::from_value(resume.skills).unwrap_or_default();
        let projects: Vec<ProjectResponse> = serde_json::from_value(resume.projects).unwrap_or_default();
        let certificates: Vec<CertificateResponse> = serde_json::from_value(resume.certificates).unwrap_or_default();
        let languages: Vec<LanguageResponse> = serde_json::from_value(resume.languages).unwrap_or_default();

        Ok(Self {
            id: resume.id.to_string(),
            personal_info,
            summary: resume.summary,
            experience,
            education,
            skills,
            projects,
            certificates,
            languages,
            created_at: resume.created_at.to_rfc3339(),
            updated_at: resume.updated_at.to_rfc3339(),
            last_updated: resume.last_updated.to_rfc3339(),
        })
    }
}

// =====================================================
// 旧模型（保留以保持向后兼容）
// =====================================================

/// 简历模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Resume {
    pub id: Uuid,
    pub personal_info_id: Uuid,
    pub summary: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// 创建简历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResume {
    pub personal_info_id: Uuid,
    pub summary: Option<String>,
}

/// 更新简历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResume {
    pub personal_info_id: Option<Uuid>,
    pub summary: Option<String>,
}

/// 完整简历响应（包含所有关联数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeDetail {
    pub resume: Resume,
    pub personal_info: PersonalInfo,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
    pub certificates: Vec<Certificate>,
    pub languages: Vec<Language>,
}

impl From<PersonalInfo> for PersonalInfoResponse {
    fn from(info: PersonalInfo) -> Self {
        Self {
            id: info.id.to_string(),
            name: info.name,
            title: info.title,
            email: info.email,
            phone: info.phone,
            location: info.location,
            website: info.website,
            github: info.github,
            avatar: info.avatar,
            bio: info.bio,
        }
    }
}

impl From<Experience> for ExperienceResponse {
    fn from(exp: Experience) -> Self {
        let description: Vec<String> = serde_json::from_value(exp.description).unwrap_or_default();
        let technologies: Vec<String> = serde_json::from_value(exp.technologies).unwrap_or_default();
        let end_date = exp.end_date.map(|d| d.format("%Y-%m").to_string());
        
        Self {
            id: exp.id.to_string(),
            company: exp.company,
            position: exp.position,
            duration: exp.duration,
            start_date: exp.start_date.format("%Y-%m").to_string(),
            end_date,
            description,
            technologies,
        }
    }
}

impl From<Education> for EducationResponse {
    fn from(edu: Education) -> Self {
        let end_date = edu.end_date.map(|d| d.format("%Y-%m").to_string());
        let gpa = edu.gpa.map(|g| format!("{:.2}", g));
        
        Self {
            id: edu.id.to_string(),
            institution: edu.institution,
            degree: edu.degree,
            major: edu.major,
            duration: edu.duration,
            start_date: edu.start_date.format("%Y-%m").to_string(),
            end_date,
            gpa,
            description: edu.description,
        }
    }
}

impl From<Skill> for SkillResponse {
    fn from(skill: Skill) -> Self {
        let level = match skill.level {
            crate::models::SkillLevel::Beginner => "beginner".to_string(),
            crate::models::SkillLevel::Intermediate => "intermediate".to_string(),
            crate::models::SkillLevel::Advanced => "advanced".to_string(),
            crate::models::SkillLevel::Expert => "expert".to_string(),
        };
        let years_of_experience = skill.years_of_experience.and_then(|d| {
            d.to_string().parse::<f64>().ok()
        });
        
        Self {
            id: skill.id.to_string(),
            name: skill.name,
            level,
            category: skill.category,
            years_of_experience,
        }
    }
}

impl From<Project> for ProjectResponse {
    fn from(proj: Project) -> Self {
        let technologies: Vec<String> = serde_json::from_value(proj.technologies).unwrap_or_default();
        let highlights: Vec<String> = serde_json::from_value(proj.highlights).unwrap_or_default();
        let end_date = proj.end_date.map(|d| d.format("%Y-%m").to_string());
        
        Self {
            id: proj.id.to_string(),
            name: proj.name,
            description: proj.description,
            technologies,
            duration: proj.duration,
            start_date: proj.start_date.format("%Y-%m").to_string(),
            end_date,
            highlights,
        }
    }
}

impl From<Certificate> for CertificateResponse {
    fn from(cert: Certificate) -> Self {
        Self {
            id: cert.id.to_string(),
            name: cert.name,
            issuer: cert.issuer,
            issue_date: cert.issue_date.format("%Y-%m").to_string(),
        }
    }
}

impl From<Language> for LanguageResponse {
    fn from(lang: Language) -> Self {
        let proficiency = match lang.proficiency {
            crate::models::LanguageProficiency::Basic => "basic".to_string(),
            crate::models::LanguageProficiency::Conversational => "conversational".to_string(),
            crate::models::LanguageProficiency::Professional => "professional".to_string(),
            crate::models::LanguageProficiency::Native => "native".to_string(),
        };
        
        Self {
            id: lang.id.to_string(),
            name: lang.name,
            proficiency,
        }
    }
}

/// 完整简历响应（扁平化，符合前端期望格式）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeDetailResponse {
    pub id: String,
    #[serde(rename = "personalInfo")]
    pub personal_info: PersonalInfoResponse,
    pub summary: Option<String>,
    pub experience: Vec<ExperienceResponse>,
    pub education: Vec<EducationResponse>,
    pub skills: Vec<SkillResponse>,
    pub projects: Vec<ProjectResponse>,
    pub certificates: Vec<CertificateResponse>,
    pub languages: Vec<LanguageResponse>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

impl From<ResumeDetail> for ResumeDetailResponse {
    fn from(detail: ResumeDetail) -> Self {
        Self {
            id: detail.resume.id.to_string(),
            personal_info: PersonalInfoResponse::from(detail.personal_info),
            summary: detail.resume.summary,
            experience: detail.experiences.into_iter().map(ExperienceResponse::from).collect(),
            education: detail.education.into_iter().map(EducationResponse::from).collect(),
            skills: detail.skills.into_iter().map(SkillResponse::from).collect(),
            projects: detail.projects.into_iter().map(ProjectResponse::from).collect(),
            certificates: detail.certificates.into_iter().map(CertificateResponse::from).collect(),
            languages: detail.languages.into_iter().map(LanguageResponse::from).collect(),
            last_updated: detail.resume.last_updated.to_rfc3339(),
        }
    }
}

impl From<CreateResume> for Resume {
    fn from(req: CreateResume) -> Self {
        Resume {
            id: Uuid::new_v4(),
            personal_info_id: req.personal_info_id,
            summary: req.summary,
            last_updated: Utc::now(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_resume_from_request() {
        let personal_info_id = Uuid::new_v4();
        let req = CreateResume {
            personal_info_id,
            summary: Some("5年全栈开发经验".to_string()),
        };

        let resume = Resume::from(req);

        assert_eq!(resume.personal_info_id, personal_info_id);
        assert_eq!(resume.summary, Some("5年全栈开发经验".to_string()));
    }

    #[test]
    fn test_resume_serialization() {
        let resume = Resume {
            id: Uuid::new_v4(),
            personal_info_id: Uuid::new_v4(),
            summary: Some("5年全栈开发经验".to_string()),
            last_updated: Utc::now(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&resume).unwrap();
        let deserialized: Resume = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, resume.id);
        assert_eq!(deserialized.personal_info_id, resume.personal_info_id);
        assert_eq!(deserialized.summary, resume.summary);
    }

    #[test]
    fn test_resume_detail_serialization() {
        let resume_detail = ResumeDetail {
            resume: Resume {
                id: Uuid::new_v4(),
                personal_info_id: Uuid::new_v4(),
                summary: Some("测试摘要".to_string()),
                last_updated: Utc::now(),
                created_at: Utc::now(),
            },
            personal_info: PersonalInfo {
                id: Uuid::new_v4(),
                name: "张三".to_string(),
                title: "软件工程师".to_string(),
                email: "zhangsan@example.com".to_string(),
                phone: "13800138000".to_string(),
                location: "北京".to_string(),
                website: None,
                github: None,
                avatar: None,
                bio: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            experiences: vec![],
            education: vec![],
            skills: vec![],
            projects: vec![],
            certificates: vec![],
            languages: vec![],
        };

        let json = serde_json::to_string(&resume_detail).unwrap();
        let deserialized: ResumeDetail = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.resume.id, resume_detail.resume.id);
        assert_eq!(deserialized.personal_info.name, resume_detail.personal_info.name);
    }

    #[test]
    fn test_resume_without_summary() {
        let req = CreateResume {
            personal_info_id: Uuid::new_v4(),
            summary: None,
        };

        let resume = Resume::from(req);

        assert!(resume.summary.is_none());
    }

    #[test]
    fn test_create_resume_single() {
        let req = CreateResumeSingle::new(
            "张三".to_string(),
            "软件工程师".to_string(),
            "zhangsan@example.com".to_string(),
            "13800138000".to_string(),
            "北京".to_string(),
        );

        assert_eq!(req.name, "张三");
        assert_eq!(req.title, "软件工程师");
        assert_eq!(req.email, "zhangsan@example.com");
        assert!(req.experience.is_some());
    }

    #[test]
    fn test_update_resume_request() {
        let req = UpdateResumeRequest {
            name: Some("李四".to_string()),
            title: None,
            email: None,
            phone: None,
            location: None,
            website: None,
            github: None,
            avatar: None,
            bio: None,
            summary: None,
            experience: None,
            education: None,
            skills: None,
            projects: None,
            certificates: None,
            languages: None,
        };

        assert_eq!(req.name, Some("李四".to_string()));
        assert!(req.title.is_none());
    }
}
