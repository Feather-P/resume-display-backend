use anyhow::Result;
use clap::{Parser, Subcommand};
use uuid::Uuid;

use crate::config::Config;
use crate::db::{create_pool, Queries};
use crate::models::{CreateResume, ResumeDetail};

/// 简历管理系统 CLI
#[derive(Parser, Debug)]
#[command(name = "resume-cli")]
#[command(about = "简历管理系统命令行工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 启动Web服务器
    Server {
        /// 服务器端口
        #[arg(short, long)]
        port: Option<u16>,
        /// 服务器主机地址
        #[arg(short = 'H', long)]
        host: Option<String>,
    },
    /// 简历操作
    Resume {
        #[command(subcommand)]
        action: ResumeAction,
    },
    /// 运行数据库迁移
    Migrate,
}

#[derive(Subcommand, Debug)]
pub enum ResumeAction {
    /// 列出所有简历
    List,
    /// 获取指定简历详情
    Get {
        /// 简历ID
        id: String,
    },
    /// 创建新简历
    Create {
        /// 个人信息ID
        #[arg(short, long)]
        personal_info_id: String,
        /// 简历摘要
        #[arg(short, long)]
        summary: Option<String>,
    },
    /// 更新简历
    Update {
        /// 简历ID
        id: String,
        /// 新的个人信息ID
        #[arg(short, long)]
        personal_info_id: Option<String>,
        /// 新的简历摘要
        #[arg(short, long)]
        summary: Option<String>,
    },
    /// 删除简历
    Delete {
        /// 简历ID
        id: String,
    },
}

/// 运行CLI命令
pub async fn run_cli() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from_env()?;

    match cli.command {
        Commands::Server { port, host } => {
            let server_port = port.unwrap_or(config.server_port);
            let server_host = host.unwrap_or_else(|| config.server_host.clone());
            start_server(&config, server_host, server_port).await?;
        }
        Commands::Resume { action } => {
            handle_resume_action(&config, action).await?;
        }
        Commands::Migrate => {
            run_migration(&config).await?;
        }
    }

    Ok(())
}

/// 启动Web服务器
async fn start_server(config: &Config, host: String, port: u16) -> Result<()> {
    let address = format!("{}:{}", host, port);
    println!("启动服务器: http://{}", address);
    crate::server::run_server(config, host, port).await?;
    Ok(())
}

/// 处理简历操作
async fn handle_resume_action(config: &Config, action: ResumeAction) -> Result<()> {
    let pool = create_pool(&config.database_url).await?;
    let queries = Queries::new(pool);

    match action {
        ResumeAction::List => {
            let resumes = queries.list_resumes().await?;
            println!("简历列表 ({} 个):", resumes.len());
            for resume in resumes {
                println!("  - ID: {}, 个人信息ID: {}, 创建时间: {}", 
                    resume.id, resume.personal_info_id, resume.created_at.format("%Y-%m-%d %H:%M:%S"));
            }
        }
        ResumeAction::Get { id } => {
            let uuid = Uuid::parse_str(&id)?;
            match queries.get_resume_detail(uuid).await? {
                Some(detail) => {
                    print_resume_detail(&detail);
                }
                None => println!("简历不存在"),
            }
        }
        ResumeAction::Create { personal_info_id, summary } => {
            let personal_info_uuid = Uuid::parse_str(&personal_info_id)?;
            let create_req = CreateResume {
                personal_info_id: personal_info_uuid,
                summary,
            };
            match queries.create_resume(create_req).await {
                Ok(resume) => {
                    println!("简历创建成功!");
                    println!("  ID: {}", resume.id);
                    println!("  个人信息ID: {}", resume.personal_info_id);
                    if let Some(s) = resume.summary {
                        println!("  摘要: {}", s);
                    }
                }
                Err(e) => {
                    eprintln!("创建简历失败: {}", e);
                }
            }
        }
        ResumeAction::Update { id, personal_info_id, summary } => {
            let uuid = Uuid::parse_str(&id)?;
            let personal_info_uuid = personal_info_id.map(|p| Uuid::parse_str(&p)).transpose()?;
            
            let mut update_req = crate::models::UpdateResume {
                personal_info_id: None,
                summary: None,
            };
            
            if let Some(pii) = personal_info_uuid {
                update_req.personal_info_id = Some(pii);
            }
            if summary.is_some() {
                update_req.summary = summary;
            }
            
            match queries.update_resume(uuid, update_req).await? {
                Some(resume) => {
                    println!("简历更新成功!");
                    println!("  ID: {}", resume.id);
                    println!("  个人信息ID: {}", resume.personal_info_id);
                    if let Some(s) = resume.summary {
                        println!("  摘要: {}", s);
                    }
                }
                None => println!("简历不存在"),
            }
        }
        ResumeAction::Delete { id } => {
            let uuid = Uuid::parse_str(&id)?;
            match queries.delete_resume(uuid).await? {
                true => println!("简历删除成功"),
                false => println!("简历不存在"),
            }
        }
    }

    Ok(())
}

/// 打印简历详情
fn print_resume_detail(detail: &ResumeDetail) {
    println!("========================================");
    println!("简历详情");
    println!("========================================");
    println!("ID: {}", detail.resume.id);
    println!("创建时间: {}", detail.resume.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("最后更新: {}", detail.resume.last_updated.format("%Y-%m-%d %H:%M:%S"));
    
    if let Some(summary) = &detail.resume.summary {
        println!("摘要: {}", summary);
    }
    
    println!("\n--- 个人信息 ---");
    println!("姓名: {}", detail.personal_info.name);
    println!("职位: {}", detail.personal_info.title);
    println!("邮箱: {}", detail.personal_info.email);
    println!("电话: {}", detail.personal_info.phone);
    println!("位置: {}", detail.personal_info.location);
    if let Some(website) = &detail.personal_info.website {
        println!("网站: {}", website);
    }
    if let Some(github) = &detail.personal_info.github {
        println!("GitHub: {}", github);
    }
    if let Some(bio) = &detail.personal_info.bio {
        println!("简介: {}", bio);
    }
    
    if !detail.experiences.is_empty() {
        println!("\n--- 工作经验 ({} 个) ---", detail.experiences.len());
        for exp in &detail.experiences {
            println!("  - {} @ {} ({})", exp.position, exp.company, exp.duration);
            if let Some(desc) = exp.description.as_array() {
                for d in desc {
                    if let Some(s) = d.as_str() {
                        println!("    * {}", s);
                    }
                }
            }
        }
    }
    
    if !detail.education.is_empty() {
        println!("\n--- 教育经历 ({} 个) ---", detail.education.len());
        for edu in &detail.education {
            println!("  - {} @ {} ({})", edu.degree, edu.institution, edu.duration);
            println!("    专业: {}", edu.major);
            if let Some(gpa) = &edu.gpa {
                println!("    GPA: {}", gpa);
            }
        }
    }
    
    if !detail.skills.is_empty() {
        println!("\n--- 技能 ({} 个) ---", detail.skills.len());
        for skill in &detail.skills {
            println!("  - {} ({})", skill.name, skill.category);
            if let Some(years) = &skill.years_of_experience {
                println!("    经验: {} 年", years);
            }
        }
    }
    
    if !detail.projects.is_empty() {
        println!("\n--- 项目 ({} 个) ---", detail.projects.len());
        for proj in &detail.projects {
            println!("  - {} ({})", proj.name, proj.duration);
            println!("    {}", proj.description);
        }
    }
    
    if !detail.certificates.is_empty() {
        println!("\n--- 证书 ({} 个) ---", detail.certificates.len());
        for cert in &detail.certificates {
            println!("  - {} from {}", cert.name, cert.issuer);
            println!("    获取日期: {}", cert.issue_date);
        }
    }
    
    if !detail.languages.is_empty() {
        println!("\n--- 语言能力 ({} 个) ---", detail.languages.len());
        for lang in &detail.languages {
            println!("  - {} ({:?})", lang.name, lang.proficiency);
        }
    }
    
    println!("========================================");
}

/// 运行数据库迁移
async fn run_migration(config: &Config) -> Result<()> {
    let pool = create_pool(&config.database_url).await?;
    println!("运行数据库迁移...");
    crate::db::run_migrations(&pool).await?;
    println!("数据库迁移完成!");
    Ok(())
}
