-- =====================================================
-- 简历单一表整合迁移
-- 将所有简历数据整合到一个单一的 resume 表中
-- PostgreSQL DDL 语句
-- =====================================================

-- =====================================================
-- 创建新的单一 resume 表
-- =====================================================
CREATE TABLE IF NOT EXISTS resume_single (
    -- 基本信息
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    location TEXT NOT NULL,
    website TEXT,
    github TEXT,
    avatar TEXT,
    bio TEXT,
    
    -- 摘要
    summary TEXT,
    
    -- 工作经验 (JSONB 数组)
    experience JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 教育经历 (JSONB 数组)
    education JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 技能 (JSONB 数组)
    skills JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 项目 (JSONB 数组)
    projects JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 证书 (JSONB 数组)
    certificates JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 语言 (JSONB 数组)
    languages JSONB NOT NULL DEFAULT '[]'::jsonb,
    
    -- 时间戳
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- CHECK 约束确保数据完整性
    CONSTRAINT chk_resume_single_name_not_empty CHECK (LENGTH(TRIM(name)) > 0),
    CONSTRAINT chk_resume_single_title_not_empty CHECK (LENGTH(TRIM(title)) > 0),
    CONSTRAINT chk_resume_single_email_not_empty CHECK (LENGTH(TRIM(email)) > 0),
    CONSTRAINT chk_resume_single_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT chk_resume_single_phone_not_empty CHECK (LENGTH(TRIM(phone)) > 0),
    CONSTRAINT chk_resume_single_location_not_empty CHECK (LENGTH(TRIM(location)) > 0),
    CONSTRAINT chk_resume_single_experience_is_array CHECK (jsonb_typeof(experience) = 'array'),
    CONSTRAINT chk_resume_single_education_is_array CHECK (jsonb_typeof(education) = 'array'),
    CONSTRAINT chk_resume_single_skills_is_array CHECK (jsonb_typeof(skills) = 'array'),
    CONSTRAINT chk_resume_single_projects_is_array CHECK (jsonb_typeof(projects) = 'array'),
    CONSTRAINT chk_resume_single_certificates_is_array CHECK (jsonb_typeof(certificates) = 'array'),
    CONSTRAINT chk_resume_single_languages_is_array CHECK (jsonb_typeof(languages) = 'array')
);

-- =====================================================
-- 为 JSONB 字段创建 GIN 索引以优化查询
-- =====================================================
CREATE INDEX IF NOT EXISTS idx_resume_single_experience ON resume_single USING GIN (experience);
CREATE INDEX IF NOT EXISTS idx_resume_single_education ON resume_single USING GIN (education);
CREATE INDEX IF NOT EXISTS idx_resume_single_skills ON resume_single USING GIN (skills);
CREATE INDEX IF NOT EXISTS idx_resume_single_projects ON resume_single USING GIN (projects);
CREATE INDEX IF NOT EXISTS idx_resume_single_certificates ON resume_single USING GIN (certificates);
CREATE INDEX IF NOT EXISTS idx_resume_single_languages ON resume_single USING GIN (languages);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_resume_single_email ON resume_single(email);
CREATE INDEX IF NOT EXISTS idx_resume_single_name ON resume_single(name);
CREATE INDEX IF NOT EXISTS idx_resume_single_last_updated ON resume_single(last_updated);

-- =====================================================
-- 自动更新 updated_at 触发器函数
-- =====================================================
CREATE OR REPLACE FUNCTION update_resume_single_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    NEW.last_updated = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为 resume_single 表创建触发器
DROP TRIGGER IF EXISTS update_resume_single_updated_at ON resume_single;
CREATE TRIGGER update_resume_single_updated_at
    BEFORE UPDATE ON resume_single
    FOR EACH ROW
    EXECUTE FUNCTION update_resume_single_updated_at();

-- =====================================================
-- 插入默认简历记录（使用硬编码的锁定ID）
-- =====================================================
INSERT INTO resume_single (
    id,
    name,
    title,
    email,
    phone,
    location,
    website,
    github,
    avatar,
    bio,
    summary,
    experience,
    education,
    skills,
    projects,
    certificates,
    languages
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    '张三',
    '高级软件工程师',
    'zhangsan@example.com',
    '+86 138 0000 0000',
    '北京市海淀区',
    'https://zhangsan.dev',
    'https://github.com/zhangsan',
    'https://example.com/avatar.jpg',
    '热爱编程，专注于后端开发和系统架构设计。',
    '拥有5年以上的软件开发经验，擅长使用Rust、Go等现代编程语言构建高性能、可扩展的后端系统。',
    '[]'::jsonb,
    '[]'::jsonb,
    '[]'::jsonb,
    '[]'::jsonb,
    '[]'::jsonb,
    '[]'::jsonb
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- 注释
-- =====================================================
COMMENT ON TABLE resume_single IS '单一简历表，包含所有简历数据';
COMMENT ON COLUMN resume_single.id IS '简历ID（使用硬编码UUID 00000000-0000-0000-0000-000000000001 作为锁定ID）';
COMMENT ON COLUMN resume_single.name IS '姓名';
COMMENT ON COLUMN resume_single.title IS '职位/头衔';
COMMENT ON COLUMN resume_single.email IS '邮箱地址';
COMMENT ON COLUMN resume_single.phone IS '电话号码';
COMMENT ON COLUMN resume_single.location IS '所在地';
COMMENT ON COLUMN resume_single.website IS '个人网站';
COMMENT ON COLUMN resume_single.github IS 'GitHub主页';
COMMENT ON COLUMN resume_single.avatar IS '头像URL';
COMMENT ON COLUMN resume_single.bio IS '个人简介';
COMMENT ON COLUMN resume_single.summary IS '简历摘要';
COMMENT ON COLUMN resume_single.experience IS '工作经验（JSONB数组）';
COMMENT ON COLUMN resume_single.education IS '教育经历（JSONB数组）';
COMMENT ON COLUMN resume_single.skills IS '技能（JSONB数组）';
COMMENT ON COLUMN resume_single.projects IS '项目（JSONB数组）';
COMMENT ON COLUMN resume_single.certificates IS '证书（JSONB数组）';
COMMENT ON COLUMN resume_single.languages IS '语言能力（JSONB数组）';
COMMENT ON COLUMN resume_single.created_at IS '创建时间';
COMMENT ON COLUMN resume_single.updated_at IS '更新时间';
COMMENT ON COLUMN resume_single.last_updated IS '最后更新时间';