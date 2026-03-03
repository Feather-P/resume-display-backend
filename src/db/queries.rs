use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::{
    Certificate, CreateCertificate, CreateEducation, CreateExperience, CreateLanguage,
    CreatePersonalInfo, CreateProject, CreateResume, CreateResumeSingle, CreateSkill, Education,
    Experience, Language, PersonalInfo, Project, Resume, ResumeDetail, ResumeSingle, Skill,
    UpdateCertificate, UpdateEducation, UpdateExperience, UpdateLanguage, UpdatePersonalInfo,
    UpdateProject, UpdateResume, UpdateResumeRequest, UpdateSkill,
};

/// 硬编码的锁定简历ID
/// 用于单简历模式，所有查询都使用此ID
pub const LOCKED_RESUME_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000001);

/// 数据库查询操作
pub struct Queries {
    pool: PgPool,
}

impl Queries {
    /// 创建新的查询实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ==================== PersonalInfo 操作 ====================

    /// 创建个人信息
    pub async fn create_personal_info(&self, req: CreatePersonalInfo) -> Result<PersonalInfo> {
        let personal_info = PersonalInfo::from(req);
        sqlx::query(
            r#"
            INSERT INTO personal_info (id, name, title, email, phone, location, website, github, avatar, bio, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(personal_info.id)
        .bind(&personal_info.name)
        .bind(&personal_info.title)
        .bind(&personal_info.email)
        .bind(&personal_info.phone)
        .bind(&personal_info.location)
        .bind(&personal_info.website)
        .bind(&personal_info.github)
        .bind(&personal_info.avatar)
        .bind(&personal_info.bio)
        .bind(personal_info.created_at)
        .bind(personal_info.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(personal_info)
    }

    /// 获取个人信息
    pub async fn get_personal_info(&self, id: Uuid) -> Result<Option<PersonalInfo>> {
        let result = sqlx::query(
            r#"SELECT id, name, title, email, phone, location, website, github, avatar, bio, created_at, updated_at FROM personal_info WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(PersonalInfo {
                id: row.get("id"),
                name: row.get("name"),
                title: row.get("title"),
                email: row.get("email"),
                phone: row.get("phone"),
                location: row.get("location"),
                website: row.get("website"),
                github: row.get("github"),
                avatar: row.get("avatar"),
                bio: row.get("bio"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新个人信息
    pub async fn update_personal_info(&self, id: Uuid, req: UpdatePersonalInfo) -> Result<Option<PersonalInfo>> {
        let current = self.get_personal_info(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let name = req.name.unwrap_or(current.name);
        let title = req.title.unwrap_or(current.title);
        let email = req.email.unwrap_or(current.email);
        let phone = req.phone.unwrap_or(current.phone);
        let location = req.location.unwrap_or(current.location);
        let website = req.website.or(current.website);
        let github = req.github.or(current.github);
        let avatar = req.avatar.or(current.avatar);
        let bio = req.bio.or(current.bio);

        sqlx::query(
            r#"
            UPDATE personal_info
            SET name = $1, title = $2, email = $3, phone = $4, location = $5, website = $6, github = $7, avatar = $8, bio = $9, updated_at = CURRENT_TIMESTAMP
            WHERE id = $10
            "#,
        )
        .bind(&name)
        .bind(&title)
        .bind(&email)
        .bind(&phone)
        .bind(&location)
        .bind(&website)
        .bind(&github)
        .bind(&avatar)
        .bind(&bio)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_personal_info(id).await
    }

    /// 删除个人信息
    pub async fn delete_personal_info(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM personal_info WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Experience 操作 ====================

    /// 创建工作经验
    pub async fn create_experience(&self, req: CreateExperience) -> Result<Experience> {
        let experience = Experience::from(req);
        sqlx::query(
            r#"
            INSERT INTO experience (id, company, position, duration, start_date, end_date, current, description, technologies, achievements, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(experience.id)
        .bind(&experience.company)
        .bind(&experience.position)
        .bind(&experience.duration)
        .bind(experience.start_date)
        .bind(experience.end_date)
        .bind(experience.current)
        .bind(&experience.description)
        .bind(&experience.technologies)
        .bind(&experience.achievements)
        .bind(experience.created_at)
        .bind(experience.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(experience)
    }

    /// 获取工作经验
    pub async fn get_experience(&self, id: Uuid) -> Result<Option<Experience>> {
        let result = sqlx::query(
            r#"SELECT id, company, position, duration, start_date, end_date, current, description, technologies, achievements, created_at, updated_at FROM experience WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Experience {
                id: row.get("id"),
                company: row.get("company"),
                position: row.get("position"),
                duration: row.get("duration"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                current: row.get("current"),
                description: row.get("description"),
                technologies: row.get("technologies"),
                achievements: row.get("achievements"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新工作经验
    pub async fn update_experience(&self, id: Uuid, req: UpdateExperience) -> Result<Option<Experience>> {
        let current = self.get_experience(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let company = req.company.unwrap_or(current.company);
        let position = req.position.unwrap_or(current.position);
        let duration = req.duration.unwrap_or(current.duration);
        let start_date = req.start_date.unwrap_or(current.start_date);
        let end_date = req.end_date.or(current.end_date);
        let current_flag = req.current.unwrap_or(current.current);
        let description = req.description.map(|d| serde_json::to_value(d).unwrap_or(serde_json::json!([]))).unwrap_or(current.description);
        let technologies = req.technologies.map(|t| serde_json::to_value(t).unwrap_or(serde_json::json!([]))).unwrap_or(current.technologies);
        let achievements = req.achievements.map(|a| serde_json::to_value(a).unwrap_or(serde_json::json!([]))).unwrap_or(current.achievements);

        sqlx::query(
            r#"
            UPDATE experience
            SET company = $1, position = $2, duration = $3, start_date = $4, end_date = $5, current = $6, description = $7, technologies = $8, achievements = $9, updated_at = CURRENT_TIMESTAMP
            WHERE id = $10
            "#,
        )
        .bind(&company)
        .bind(&position)
        .bind(&duration)
        .bind(start_date)
        .bind(end_date)
        .bind(current_flag)
        .bind(&description)
        .bind(&technologies)
        .bind(&achievements)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_experience(id).await
    }

    /// 删除工作经验
    pub async fn delete_experience(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM experience WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Education 操作 ====================

    /// 创建教育经历
    pub async fn create_education(&self, req: CreateEducation) -> Result<Education> {
        let education = Education::from(req);
        sqlx::query(
            r#"
            INSERT INTO education (id, institution, degree, major, duration, start_date, end_date, current, description, gpa, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(education.id)
        .bind(&education.institution)
        .bind(&education.degree)
        .bind(&education.major)
        .bind(&education.duration)
        .bind(education.start_date)
        .bind(education.end_date)
        .bind(education.current)
        .bind(&education.description)
        .bind(education.gpa)
        .bind(education.created_at)
        .bind(education.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(education)
    }

    /// 获取教育经历
    pub async fn get_education(&self, id: Uuid) -> Result<Option<Education>> {
        let result = sqlx::query(
            r#"SELECT id, institution, degree, major, duration, start_date, end_date, current, description, gpa, created_at, updated_at FROM education WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Education {
                id: row.get("id"),
                institution: row.get("institution"),
                degree: row.get("degree"),
                major: row.get("major"),
                duration: row.get("duration"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                current: row.get("current"),
                description: row.get("description"),
                gpa: row.get("gpa"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新教育经历
    pub async fn update_education(&self, id: Uuid, req: UpdateEducation) -> Result<Option<Education>> {
        let current = self.get_education(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let institution = req.institution.unwrap_or(current.institution);
        let degree = req.degree.unwrap_or(current.degree);
        let major = req.major.unwrap_or(current.major);
        let duration = req.duration.unwrap_or(current.duration);
        let start_date = req.start_date.unwrap_or(current.start_date);
        let end_date = req.end_date.or(current.end_date);
        let current_flag = req.current.unwrap_or(current.current);
        let description = req.description.or(current.description);
        let gpa = req.gpa.map(|g| rust_decimal::Decimal::from_f64_retain(g).unwrap_or(rust_decimal::Decimal::ZERO)).or(current.gpa);

        sqlx::query(
            r#"
            UPDATE education
            SET institution = $1, degree = $2, major = $3, duration = $4, start_date = $5, end_date = $6, current = $7, description = $8, gpa = $9, updated_at = CURRENT_TIMESTAMP
            WHERE id = $10
            "#,
        )
        .bind(&institution)
        .bind(&degree)
        .bind(&major)
        .bind(&duration)
        .bind(start_date)
        .bind(end_date)
        .bind(current_flag)
        .bind(&description)
        .bind(gpa)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_education(id).await
    }

    /// 删除教育经历
    pub async fn delete_education(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM education WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Skill 操作 ====================

    /// 创建技能
    pub async fn create_skill(&self, req: CreateSkill) -> Result<Skill> {
        let skill = Skill::from(req);
        sqlx::query(
            r#"
            INSERT INTO skill (id, name, level, category, years_of_experience, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(skill.id)
        .bind(&skill.name)
        .bind(skill.level)
        .bind(&skill.category)
        .bind(skill.years_of_experience)
        .bind(skill.created_at)
        .bind(skill.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(skill)
    }

    /// 获取技能
    pub async fn get_skill(&self, id: Uuid) -> Result<Option<Skill>> {
        let result = sqlx::query(
            r#"SELECT id, name, level, category, years_of_experience, created_at, updated_at FROM skill WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Skill {
                id: row.get("id"),
                name: row.get("name"),
                level: row.get("level"),
                category: row.get("category"),
                years_of_experience: row.get("years_of_experience"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新技能
    pub async fn update_skill(&self, id: Uuid, req: UpdateSkill) -> Result<Option<Skill>> {
        let current = self.get_skill(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let name = req.name.unwrap_or(current.name);
        let level = req.level.unwrap_or(current.level);
        let category = req.category.unwrap_or(current.category);
        let years_of_experience = req.years_of_experience.map(|y| rust_decimal::Decimal::from_f64_retain(y).unwrap_or(rust_decimal::Decimal::ZERO)).or(current.years_of_experience);

        sqlx::query(
            r#"
            UPDATE skill
            SET name = $1, level = $2, category = $3, years_of_experience = $4, updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            "#,
        )
        .bind(&name)
        .bind(level)
        .bind(&category)
        .bind(years_of_experience)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_skill(id).await
    }

    /// 删除技能
    pub async fn delete_skill(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM skill WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Project 操作 ====================

    /// 创建项目
    pub async fn create_project(&self, req: CreateProject) -> Result<Project> {
        let project = Project::from(req);
        sqlx::query(
            r#"
            INSERT INTO project (id, name, description, technologies, duration, start_date, end_date, current, link, repository, highlights, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.technologies)
        .bind(&project.duration)
        .bind(project.start_date)
        .bind(project.end_date)
        .bind(project.current)
        .bind(&project.link)
        .bind(&project.repository)
        .bind(&project.highlights)
        .bind(project.created_at)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(project)
    }

    /// 获取项目
    pub async fn get_project(&self, id: Uuid) -> Result<Option<Project>> {
        let result = sqlx::query(
            r#"SELECT id, name, description, technologies, duration, start_date, end_date, current, link, repository, highlights, created_at, updated_at FROM project WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Project {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                technologies: row.get("technologies"),
                duration: row.get("duration"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                current: row.get("current"),
                link: row.get("link"),
                repository: row.get("repository"),
                highlights: row.get("highlights"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新项目
    pub async fn update_project(&self, id: Uuid, req: UpdateProject) -> Result<Option<Project>> {
        let current = self.get_project(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let name = req.name.unwrap_or(current.name);
        let description = req.description.unwrap_or(current.description);
        let technologies = req.technologies.map(|t| serde_json::to_value(t).unwrap_or(serde_json::json!([]))).unwrap_or(current.technologies);
        let duration = req.duration.unwrap_or(current.duration);
        let start_date = req.start_date.unwrap_or(current.start_date);
        let end_date = req.end_date.or(current.end_date);
        let current_flag = req.current.unwrap_or(current.current);
        let link = req.link.or(current.link);
        let repository = req.repository.or(current.repository);
        let highlights = req.highlights.map(|h| serde_json::to_value(h).unwrap_or(serde_json::json!([]))).unwrap_or(current.highlights);

        sqlx::query(
            r#"
            UPDATE project
            SET name = $1, description = $2, technologies = $3, duration = $4, start_date = $5, end_date = $6, current = $7, link = $8, repository = $9, highlights = $10, updated_at = CURRENT_TIMESTAMP
            WHERE id = $11
            "#,
        )
        .bind(&name)
        .bind(&description)
        .bind(&technologies)
        .bind(&duration)
        .bind(start_date)
        .bind(end_date)
        .bind(current_flag)
        .bind(&link)
        .bind(&repository)
        .bind(&highlights)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_project(id).await
    }

    /// 删除项目
    pub async fn delete_project(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM project WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Certificate 操作 ====================

    /// 创建证书
    pub async fn create_certificate(&self, req: CreateCertificate) -> Result<Certificate> {
        let certificate = Certificate::from(req);
        sqlx::query(
            r#"
            INSERT INTO certificate (id, name, issuer, issue_date, expiry_date, credential_id, credential_url, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(certificate.id)
        .bind(&certificate.name)
        .bind(&certificate.issuer)
        .bind(certificate.issue_date)
        .bind(certificate.expiry_date)
        .bind(&certificate.credential_id)
        .bind(&certificate.credential_url)
        .bind(certificate.created_at)
        .bind(certificate.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(certificate)
    }

    /// 获取证书
    pub async fn get_certificate(&self, id: Uuid) -> Result<Option<Certificate>> {
        let result = sqlx::query(
            r#"SELECT id, name, issuer, issue_date, expiry_date, credential_id, credential_url, created_at, updated_at FROM certificate WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Certificate {
                id: row.get("id"),
                name: row.get("name"),
                issuer: row.get("issuer"),
                issue_date: row.get("issue_date"),
                expiry_date: row.get("expiry_date"),
                credential_id: row.get("credential_id"),
                credential_url: row.get("credential_url"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新证书
    pub async fn update_certificate(&self, id: Uuid, req: UpdateCertificate) -> Result<Option<Certificate>> {
        let current = self.get_certificate(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let name = req.name.unwrap_or(current.name);
        let issuer = req.issuer.unwrap_or(current.issuer);
        let issue_date = req.issue_date.unwrap_or(current.issue_date);
        let expiry_date = req.expiry_date.or(current.expiry_date);
        let credential_id = req.credential_id.or(current.credential_id);
        let credential_url = req.credential_url.or(current.credential_url);

        sqlx::query(
            r#"
            UPDATE certificate
            SET name = $1, issuer = $2, issue_date = $3, expiry_date = $4, credential_id = $5, credential_url = $6, updated_at = CURRENT_TIMESTAMP
            WHERE id = $7
            "#,
        )
        .bind(&name)
        .bind(&issuer)
        .bind(issue_date)
        .bind(expiry_date)
        .bind(&credential_id)
        .bind(&credential_url)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_certificate(id).await
    }

    /// 删除证书
    pub async fn delete_certificate(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM certificate WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Language 操作 ====================

    /// 创建语言能力
    pub async fn create_language(&self, req: CreateLanguage) -> Result<Language> {
        let language = Language::from(req);
        sqlx::query(
            r#"
            INSERT INTO language (id, name, proficiency, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(language.id)
        .bind(&language.name)
        .bind(language.proficiency)
        .bind(language.created_at)
        .bind(language.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(language)
    }

    /// 获取语言能力
    pub async fn get_language(&self, id: Uuid) -> Result<Option<Language>> {
        let result = sqlx::query(
            r#"SELECT id, name, proficiency, created_at, updated_at FROM language WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Language {
                id: row.get("id"),
                name: row.get("name"),
                proficiency: row.get("proficiency"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新语言能力
    pub async fn update_language(&self, id: Uuid, req: UpdateLanguage) -> Result<Option<Language>> {
        let current = self.get_language(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let name = req.name.unwrap_or(current.name);
        let proficiency = req.proficiency.unwrap_or(current.proficiency);

        sqlx::query(
            r#"
            UPDATE language
            SET name = $1, proficiency = $2, updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            "#,
        )
        .bind(&name)
        .bind(proficiency)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_language(id).await
    }

    /// 删除语言能力
    pub async fn delete_language(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM language WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Resume 操作 ====================

    /// 创建简历
    pub async fn create_resume(&self, req: CreateResume) -> Result<Resume> {
        let resume = Resume::from(req);
        sqlx::query(
            r#"
            INSERT INTO resume (id, personal_info_id, summary, last_updated, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(resume.id)
        .bind(resume.personal_info_id)
        .bind(&resume.summary)
        .bind(resume.last_updated)
        .bind(resume.created_at)
        .execute(&self.pool)
        .await?;

        Ok(resume)
    }

    /// 获取简历
    pub async fn get_resume(&self, id: Uuid) -> Result<Option<Resume>> {
        let result = sqlx::query(
            r#"SELECT id, personal_info_id, summary, last_updated, created_at FROM resume WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(Resume {
                id: row.get("id"),
                personal_info_id: row.get("personal_info_id"),
                summary: row.get("summary"),
                last_updated: row.get("last_updated"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 获取所有简历
    pub async fn list_resumes(&self) -> Result<Vec<Resume>> {
        let results = sqlx::query(
            r#"SELECT id, personal_info_id, summary, last_updated, created_at FROM resume ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|row| Resume {
                id: row.get("id"),
                personal_info_id: row.get("personal_info_id"),
                summary: row.get("summary"),
                last_updated: row.get("last_updated"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    /// 获取完整简历详情（包含所有关联数据）
    pub async fn get_resume_detail(&self, id: Uuid) -> Result<Option<ResumeDetail>> {
        let resume = match self.get_resume(id).await? {
            Some(r) => r,
            None => return Ok(None),
        };

        let personal_info = match self.get_personal_info(resume.personal_info_id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        // 获取关联的工作经验
        let experience_rows = sqlx::query(
            r#"SELECT e.id, e.company, e.position, e.duration, e.start_date, e.end_date, e.current, e.description, e.technologies, e.achievements, e.created_at, e.updated_at
               FROM experience e
               INNER JOIN resume_experience re ON e.id = re.experience_id
               WHERE re.resume_id = $1
               ORDER BY e.start_date DESC"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let experiences: Vec<Experience> = experience_rows
            .into_iter()
            .map(|e| Experience {
                id: e.get("id"),
                company: e.get("company"),
                position: e.get("position"),
                duration: e.get("duration"),
                start_date: e.get("start_date"),
                end_date: e.get("end_date"),
                current: e.get("current"),
                description: e.get("description"),
                technologies: e.get("technologies"),
                achievements: e.get("achievements"),
                created_at: e.get("created_at"),
                updated_at: e.get("updated_at"),
            })
            .collect();

        // 获取关联的教育经历
        let education_rows = sqlx::query(
            r#"SELECT e.id, e.institution, e.degree, e.major, e.duration, e.start_date, e.end_date, e.current, e.description, e.gpa, e.created_at, e.updated_at
               FROM education e
               INNER JOIN resume_education re ON e.id = re.education_id
               WHERE re.resume_id = $1
               ORDER BY e.start_date DESC"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let education: Vec<Education> = education_rows
            .into_iter()
            .map(|e| Education {
                id: e.get("id"),
                institution: e.get("institution"),
                degree: e.get("degree"),
                major: e.get("major"),
                duration: e.get("duration"),
                start_date: e.get("start_date"),
                end_date: e.get("end_date"),
                current: e.get("current"),
                description: e.get("description"),
                gpa: e.get("gpa"),
                created_at: e.get("created_at"),
                updated_at: e.get("updated_at"),
            })
            .collect();

        // 获取关联的技能
        let skill_rows = sqlx::query(
            r#"SELECT s.id, s.name, s.level, s.category, s.years_of_experience, s.created_at, s.updated_at
               FROM skill s
               INNER JOIN resume_skill rs ON s.id = rs.skill_id
               WHERE rs.resume_id = $1
               ORDER BY s.category, s.name"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let skills: Vec<Skill> = skill_rows
            .into_iter()
            .map(|s| Skill {
                id: s.get("id"),
                name: s.get("name"),
                level: s.get("level"),
                category: s.get("category"),
                years_of_experience: s.get("years_of_experience"),
                created_at: s.get("created_at"),
                updated_at: s.get("updated_at"),
            })
            .collect();

        // 获取关联的项目
        let project_rows = sqlx::query(
            r#"SELECT p.id, p.name, p.description, p.technologies, p.duration, p.start_date, p.end_date, p.current, p.link, p.repository, p.highlights, p.created_at, p.updated_at
               FROM project p
               INNER JOIN resume_project rp ON p.id = rp.project_id
               WHERE rp.resume_id = $1
               ORDER BY p.start_date DESC"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let projects: Vec<Project> = project_rows
            .into_iter()
            .map(|p| Project {
                id: p.get("id"),
                name: p.get("name"),
                description: p.get("description"),
                technologies: p.get("technologies"),
                duration: p.get("duration"),
                start_date: p.get("start_date"),
                end_date: p.get("end_date"),
                current: p.get("current"),
                link: p.get("link"),
                repository: p.get("repository"),
                highlights: p.get("highlights"),
                created_at: p.get("created_at"),
                updated_at: p.get("updated_at"),
            })
            .collect();

        // 获取关联的证书
        let certificate_rows = sqlx::query(
            r#"SELECT c.id, c.name, c.issuer, c.issue_date, c.expiry_date, c.credential_id, c.credential_url, c.created_at, c.updated_at
               FROM certificate c
               INNER JOIN resume_certificate rc ON c.id = rc.certificate_id
               WHERE rc.resume_id = $1
               ORDER BY c.issue_date DESC"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let certificates: Vec<Certificate> = certificate_rows
            .into_iter()
            .map(|c| Certificate {
                id: c.get("id"),
                name: c.get("name"),
                issuer: c.get("issuer"),
                issue_date: c.get("issue_date"),
                expiry_date: c.get("expiry_date"),
                credential_id: c.get("credential_id"),
                credential_url: c.get("credential_url"),
                created_at: c.get("created_at"),
                updated_at: c.get("updated_at"),
            })
            .collect();

        // 获取关联的语言能力
        let language_rows = sqlx::query(
            r#"SELECT l.id, l.name, l.proficiency, l.created_at, l.updated_at
               FROM language l
               INNER JOIN resume_language rl ON l.id = rl.language_id
               WHERE rl.resume_id = $1
               ORDER BY l.name"#
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let languages: Vec<Language> = language_rows
            .into_iter()
            .map(|l| Language {
                id: l.get("id"),
                name: l.get("name"),
                proficiency: l.get("proficiency"),
                created_at: l.get("created_at"),
                updated_at: l.get("updated_at"),
            })
            .collect();

        Ok(Some(ResumeDetail {
            resume,
            personal_info,
            experiences,
            education,
            skills,
            projects,
            certificates,
            languages,
        }))
    }

    /// 更新简历
    pub async fn update_resume(&self, id: Uuid, req: UpdateResume) -> Result<Option<Resume>> {
        let current = self.get_resume(id).await?;
        if current.is_none() {
            return Ok(None);
        }

        let current = current.unwrap();
        let personal_info_id = req.personal_info_id.unwrap_or(current.personal_info_id);
        let summary = req.summary.or(current.summary);

        sqlx::query(
            r#"
            UPDATE resume
            SET personal_info_id = $1, summary = $2, last_updated = CURRENT_TIMESTAMP
            WHERE id = $3
            "#,
        )
        .bind(personal_info_id)
        .bind(&summary)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_resume(id).await
    }

    /// 删除简历
    pub async fn delete_resume(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM resume WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== 关联操作 ====================

    /// 为简历添加工作经验
    pub async fn add_experience_to_resume(&self, resume_id: Uuid, experience_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_experience (resume_id, experience_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(experience_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除工作经验
    pub async fn remove_experience_from_resume(&self, resume_id: Uuid, experience_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_experience WHERE resume_id = $1 AND experience_id = $2"#
        )
        .bind(resume_id)
        .bind(experience_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 为简历添加教育经历
    pub async fn add_education_to_resume(&self, resume_id: Uuid, education_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_education (resume_id, education_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(education_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除教育经历
    pub async fn remove_education_from_resume(&self, resume_id: Uuid, education_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_education WHERE resume_id = $1 AND education_id = $2"#
        )
        .bind(resume_id)
        .bind(education_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 为简历添加技能
    pub async fn add_skill_to_resume(&self, resume_id: Uuid, skill_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_skill (resume_id, skill_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(skill_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除技能
    pub async fn remove_skill_from_resume(&self, resume_id: Uuid, skill_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_skill WHERE resume_id = $1 AND skill_id = $2"#
        )
        .bind(resume_id)
        .bind(skill_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 为简历添加项目
    pub async fn add_project_to_resume(&self, resume_id: Uuid, project_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_project (resume_id, project_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(project_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除项目
    pub async fn remove_project_from_resume(&self, resume_id: Uuid, project_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_project WHERE resume_id = $1 AND project_id = $2"#
        )
        .bind(resume_id)
        .bind(project_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 为简历添加证书
    pub async fn add_certificate_to_resume(&self, resume_id: Uuid, certificate_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_certificate (resume_id, certificate_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(certificate_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除证书
    pub async fn remove_certificate_from_resume(&self, resume_id: Uuid, certificate_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_certificate WHERE resume_id = $1 AND certificate_id = $2"#
        )
        .bind(resume_id)
        .bind(certificate_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 为简历添加语言能力
    pub async fn add_language_to_resume(&self, resume_id: Uuid, language_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"INSERT INTO resume_language (resume_id, language_id, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"#
        )
        .bind(resume_id)
        .bind(language_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 从简历移除语言能力
    pub async fn remove_language_from_resume(&self, resume_id: Uuid, language_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"DELETE FROM resume_language WHERE resume_id = $1 AND language_id = $2"#
        )
        .bind(resume_id)
        .bind(language_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== ResumeSingle 操作（单简历模式）====================

    /// 获取单一简历（使用硬编码的锁定ID）
    pub async fn get_resume_single(&self) -> Result<Option<ResumeSingle>> {
        let result = sqlx::query(
            r#"
            SELECT
                id, name, title, email, phone, location, website, github, avatar, bio,
                summary, experience, education, skills, projects, certificates, languages,
                created_at, updated_at, last_updated
            FROM resume_single
            WHERE id = $1
            "#,
        )
        .bind(LOCKED_RESUME_ID)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            Ok(Some(ResumeSingle {
                id: row.get("id"),
                name: row.get("name"),
                title: row.get("title"),
                email: row.get("email"),
                phone: row.get("phone"),
                location: row.get("location"),
                website: row.get("website"),
                github: row.get("github"),
                avatar: row.get("avatar"),
                bio: row.get("bio"),
                summary: row.get("summary"),
                experience: row.get("experience"),
                education: row.get("education"),
                skills: row.get("skills"),
                projects: row.get("projects"),
                certificates: row.get("certificates"),
                languages: row.get("languages"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_updated: row.get("last_updated"),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新单一简历（使用硬编码的锁定ID）
    pub async fn update_resume_single(&self, req: UpdateResumeRequest) -> Result<ResumeSingle> {
        // 先获取当前记录
        let current = self.get_resume_single().await?
            .ok_or_else(|| anyhow::anyhow!("简历不存在，请先创建简历"))?;

        // 构建更新值
        let name = req.name.unwrap_or(current.name);
        let title = req.title.unwrap_or(current.title);
        let email = req.email.unwrap_or(current.email);
        let phone = req.phone.unwrap_or(current.phone);
        let location = req.location.unwrap_or(current.location);
        let website = req.website.or(current.website);
        let github = req.github.or(current.github);
        let avatar = req.avatar.or(current.avatar);
        let bio = req.bio.or(current.bio);
        let summary = req.summary.or(current.summary);
        let experience = req.experience.unwrap_or(current.experience);
        let education = req.education.unwrap_or(current.education);
        let skills = req.skills.unwrap_or(current.skills);
        let projects = req.projects.unwrap_or(current.projects);
        let certificates = req.certificates.unwrap_or(current.certificates);
        let languages = req.languages.unwrap_or(current.languages);

        // 执行更新
        sqlx::query(
            r#"
            UPDATE resume_single
            SET
                name = $1, title = $2, email = $3, phone = $4, location = $5,
                website = $6, github = $7, avatar = $8, bio = $9, summary = $10,
                experience = $11, education = $12, skills = $13, projects = $14,
                certificates = $15, languages = $16
            WHERE id = $17
            "#,
        )
        .bind(&name)
        .bind(&title)
        .bind(&email)
        .bind(&phone)
        .bind(&location)
        .bind(&website)
        .bind(&github)
        .bind(&avatar)
        .bind(&bio)
        .bind(&summary)
        .bind(&experience)
        .bind(&education)
        .bind(&skills)
        .bind(&projects)
        .bind(&certificates)
        .bind(&languages)
        .bind(LOCKED_RESUME_ID)
        .execute(&self.pool)
        .await?;

        // 返回更新后的记录
        self.get_resume_single().await?
            .ok_or_else(|| anyhow::anyhow!("更新后无法获取简历"))
    }

    /// 创建单一简历（使用硬编码的锁定ID）
    pub async fn create_resume_single(&self, req: CreateResumeSingle) -> Result<ResumeSingle> {
        let now = chrono::Utc::now();
        let experience = req.experience.unwrap_or(serde_json::json!([]));
        let education = req.education.unwrap_or(serde_json::json!([]));
        let skills = req.skills.unwrap_or(serde_json::json!([]));
        let projects = req.projects.unwrap_or(serde_json::json!([]));
        let certificates = req.certificates.unwrap_or(serde_json::json!([]));
        let languages = req.languages.unwrap_or(serde_json::json!([]));

        sqlx::query(
            r#"
            INSERT INTO resume_single (
                id, name, title, email, phone, location, website, github, avatar, bio,
                summary, experience, education, skills, projects, certificates, languages,
                created_at, updated_at, last_updated
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17,
                $18, $19, $20
            )
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name, title = EXCLUDED.title, email = EXCLUDED.email,
                phone = EXCLUDED.phone, location = EXCLUDED.location, website = EXCLUDED.website,
                github = EXCLUDED.github, avatar = EXCLUDED.avatar, bio = EXCLUDED.bio,
                summary = EXCLUDED.summary, experience = EXCLUDED.experience,
                education = EXCLUDED.education, skills = EXCLUDED.skills,
                projects = EXCLUDED.projects, certificates = EXCLUDED.certificates,
                languages = EXCLUDED.languages
            "#,
        )
        .bind(LOCKED_RESUME_ID)
        .bind(&req.name)
        .bind(&req.title)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.location)
        .bind(&req.website)
        .bind(&req.github)
        .bind(&req.avatar)
        .bind(&req.bio)
        .bind(&req.summary)
        .bind(&experience)
        .bind(&education)
        .bind(&skills)
        .bind(&projects)
        .bind(&certificates)
        .bind(&languages)
        .bind(now)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        // 返回创建/更新后的记录
        self.get_resume_single().await?
            .ok_or_else(|| anyhow::anyhow!("创建后无法获取简历"))
    }
}

// ==================== 独立查询函数（非方法版本）====================

/// 获取单一简历（独立函数，使用硬编码的锁定ID）
pub async fn get_resume_single(pool: &PgPool) -> Result<Option<ResumeSingle>> {
    let result = sqlx::query(
        r#"
        SELECT
            id, name, title, email, phone, location, website, github, avatar, bio,
            summary, experience, education, skills, projects, certificates, languages,
            created_at, updated_at, last_updated
        FROM resume_single
        WHERE id = $1
        "#,
    )
    .bind(LOCKED_RESUME_ID)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = result {
        Ok(Some(ResumeSingle {
            id: row.get("id"),
            name: row.get("name"),
            title: row.get("title"),
            email: row.get("email"),
            phone: row.get("phone"),
            location: row.get("location"),
            website: row.get("website"),
            github: row.get("github"),
            avatar: row.get("avatar"),
            bio: row.get("bio"),
            summary: row.get("summary"),
            experience: row.get("experience"),
            education: row.get("education"),
            skills: row.get("skills"),
            projects: row.get("projects"),
            certificates: row.get("certificates"),
            languages: row.get("languages"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_updated: row.get("last_updated"),
        }))
    } else {
        Ok(None)
    }
}

/// 更新单一简历（独立函数，使用硬编码的锁定ID）
pub async fn update_resume_single(pool: &PgPool, req: UpdateResumeRequest) -> Result<ResumeSingle> {
    // 先获取当前记录
    let current = get_resume_single(pool).await?
        .ok_or_else(|| anyhow::anyhow!("简历不存在，请先创建简历"))?;

    // 构建更新值
    let name = req.name.unwrap_or(current.name);
    let title = req.title.unwrap_or(current.title);
    let email = req.email.unwrap_or(current.email);
    let phone = req.phone.unwrap_or(current.phone);
    let location = req.location.unwrap_or(current.location);
    let website = req.website.or(current.website);
    let github = req.github.or(current.github);
    let avatar = req.avatar.or(current.avatar);
    let bio = req.bio.or(current.bio);
    let summary = req.summary.or(current.summary);
    let experience = req.experience.unwrap_or(current.experience);
    let education = req.education.unwrap_or(current.education);
    let skills = req.skills.unwrap_or(current.skills);
    let projects = req.projects.unwrap_or(current.projects);
    let certificates = req.certificates.unwrap_or(current.certificates);
    let languages = req.languages.unwrap_or(current.languages);

    // 执行更新
    sqlx::query(
        r#"
        UPDATE resume_single
        SET
            name = $1, title = $2, email = $3, phone = $4, location = $5,
            website = $6, github = $7, avatar = $8, bio = $9, summary = $10,
            experience = $11, education = $12, skills = $13, projects = $14,
            certificates = $15, languages = $16
        WHERE id = $17
        "#,
    )
    .bind(&name)
    .bind(&title)
    .bind(&email)
    .bind(&phone)
    .bind(&location)
    .bind(&website)
    .bind(&github)
    .bind(&avatar)
    .bind(&bio)
    .bind(&summary)
    .bind(&experience)
    .bind(&education)
    .bind(&skills)
    .bind(&projects)
    .bind(&certificates)
    .bind(&languages)
    .bind(LOCKED_RESUME_ID)
    .execute(pool)
    .await?;

    // 返回更新后的记录
    get_resume_single(pool).await?
        .ok_or_else(|| anyhow::anyhow!("更新后无法获取简历"))
}

/// 创建单一简历（独立函数，使用硬编码的锁定ID）
pub async fn create_resume_single(pool: &PgPool, req: CreateResumeSingle) -> Result<ResumeSingle> {
    let now = chrono::Utc::now();
    let experience = req.experience.unwrap_or(serde_json::json!([]));
    let education = req.education.unwrap_or(serde_json::json!([]));
    let skills = req.skills.unwrap_or(serde_json::json!([]));
    let projects = req.projects.unwrap_or(serde_json::json!([]));
    let certificates = req.certificates.unwrap_or(serde_json::json!([]));
    let languages = req.languages.unwrap_or(serde_json::json!([]));

    sqlx::query(
        r#"
        INSERT INTO resume_single (
            id, name, title, email, phone, location, website, github, avatar, bio,
            summary, experience, education, skills, projects, certificates, languages,
            created_at, updated_at, last_updated
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17,
            $18, $19, $20
        )
        ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name, title = EXCLUDED.title, email = EXCLUDED.email,
            phone = EXCLUDED.phone, location = EXCLUDED.location, website = EXCLUDED.website,
            github = EXCLUDED.github, avatar = EXCLUDED.avatar, bio = EXCLUDED.bio,
            summary = EXCLUDED.summary, experience = EXCLUDED.experience,
            education = EXCLUDED.education, skills = EXCLUDED.skills,
            projects = EXCLUDED.projects, certificates = EXCLUDED.certificates,
            languages = EXCLUDED.languages
        "#,
    )
    .bind(LOCKED_RESUME_ID)
    .bind(&req.name)
    .bind(&req.title)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(&req.location)
    .bind(&req.website)
    .bind(&req.github)
    .bind(&req.avatar)
    .bind(&req.bio)
    .bind(&req.summary)
    .bind(&experience)
    .bind(&education)
    .bind(&skills)
    .bind(&projects)
    .bind(&certificates)
    .bind(&languages)
    .bind(now)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    // 返回创建/更新后的记录
    get_resume_single(pool).await?
        .ok_or_else(|| anyhow::anyhow!("创建后无法获取简历"))
}
