use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::Queries;
use crate::models::{
    CreateResume, Resume, ResumeDetail, ResumeDetailResponse, UpdateResume,
};

/// API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
        }
    }

    /// 创建无数据成功响应
    pub fn success_no_data(message: &str) -> Self {
        Self {
            success: true,
            data: None,
            message: message.to_string(),
        }
    }
}

impl ErrorResponse {
    /// 创建错误响应
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            success: false,
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}

/// 获取所有简历
pub async fn list_resumes(State(pool): State<PgPool>) -> Result<Json<ApiResponse<Vec<Resume>>>, (StatusCode, Json<ErrorResponse>)> {
    let queries = Queries::new(pool);
    match queries.list_resumes().await {
        Ok(resumes) => Ok(Json(ApiResponse::success(resumes, "获取简历列表成功"))),
        Err(e) => {
            tracing::error!("获取简历列表失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "获取简历列表失败")),
            ))
        }
    }
}

/// 获取单个简历详情
pub async fn get_resume(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ResumeDetailResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let queries = Queries::new(pool);
    match queries.get_resume_detail(id).await {
        Ok(Some(resume)) => {
            let response = ResumeDetailResponse::from(resume);
            Ok(Json(ApiResponse::success(response, "获取简历详情成功")))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "简历不存在")),
        )),
        Err(e) => {
            tracing::error!("获取简历详情失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "获取简历详情失败")),
            ))
        }
    }
}

/// 创建简历请求体
#[derive(Debug, Deserialize)]
pub struct CreateResumeRequest {
    pub personal_info_id: Uuid,
    pub summary: Option<String>,
}

/// 创建简历
pub async fn create_resume(
    State(pool): State<PgPool>,
    Json(req): Json<CreateResumeRequest>,
) -> Result<Json<ApiResponse<Resume>>, (StatusCode, Json<ErrorResponse>)> {
    let queries = Queries::new(pool);

    // 首先检查个人信息是否存在
    match queries.get_personal_info(req.personal_info_id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("INVALID_INPUT", "指定的个人信息不存在")),
            ))
        }
        Err(e) => {
            tracing::error!("检查个人信息失败: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "检查个人信息失败")),
            ))
        }
    }

    let create_req = CreateResume {
        personal_info_id: req.personal_info_id,
        summary: req.summary,
    };

    match queries.create_resume(create_req).await {
        Ok(resume) => Ok(Json(ApiResponse::success(resume, "创建简历成功"))),
        Err(e) => {
            tracing::error!("创建简历失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "创建简历失败")),
            ))
        }
    }
}

/// 更新简历请求体
#[derive(Debug, Deserialize)]
pub struct UpdateResumeRequest {
    pub personal_info_id: Option<Uuid>,
    pub summary: Option<String>,
}

/// 更新简历
pub async fn update_resume(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateResumeRequest>,
) -> Result<Json<ApiResponse<Resume>>, (StatusCode, Json<ErrorResponse>)> {
    let queries = Queries::new(pool);

    // 检查简历是否存在
    match queries.get_resume(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "简历不存在")),
            ))
        }
        Err(e) => {
            tracing::error!("检查简历失败: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "检查简历失败")),
            ))
        }
    }

    // 如果提供了新的personal_info_id，检查是否存在
    if let Some(personal_info_id) = req.personal_info_id {
        match queries.get_personal_info(personal_info_id).await {
            Ok(Some(_)) => {}
            Ok(None) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::new("INVALID_INPUT", "指定的个人信息不存在")),
                ))
            }
            Err(e) => {
                tracing::error!("检查个人信息失败: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("INTERNAL_ERROR", "检查个人信息失败")),
                ))
            }
        }
    }

    let update_req = UpdateResume {
        personal_info_id: req.personal_info_id,
        summary: req.summary,
    };

    match queries.update_resume(id, update_req).await {
        Ok(Some(resume)) => Ok(Json(ApiResponse::success(resume, "更新简历成功"))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "简历不存在")),
        )),
        Err(e) => {
            tracing::error!("更新简历失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "更新简历失败")),
            ))
        }
    }
}

/// 删除简历
pub async fn delete_resume(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ErrorResponse>)> {
    let queries = Queries::new(pool);
    match queries.delete_resume(id).await {
        Ok(true) => Ok(Json(ApiResponse::success_no_data("删除简历成功"))),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "简历不存在")),
        )),
        Err(e) => {
            tracing::error!("删除简历失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "删除简历失败")),
            ))
        }
    }
}

/// 健康检查
pub async fn health_check() -> Json<ApiResponse<()>> {
    Json(ApiResponse::success_no_data("服务运行正常"))
}
