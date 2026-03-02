-- =====================================================
-- 简历数据库 Schema 设计
-- PostgreSQL DDL 语句
-- =====================================================

-- 启用 UUID 扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- 自定义枚举类型
-- =====================================================

-- 技能等级枚举
CREATE TYPE skill_level_enum AS ENUM (
    'beginner',
    'intermediate',
    'advanced',
    'expert'
);

-- 语言熟练度枚举
CREATE TYPE language_proficiency_enum AS ENUM (
    'basic',
    'conversational',
    'professional',
    'native'
);

-- =====================================================
-- 个人信息表 (PersonalInfo)
-- =====================================================
CREATE TABLE IF NOT EXISTS personal_info (
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_personal_info_email ON personal_info(email);
CREATE INDEX IF NOT EXISTS idx_personal_info_name ON personal_info(name);

-- =====================================================
-- 工作经验表 (Experience)
-- =====================================================
CREATE TABLE IF NOT EXISTS experience (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    company TEXT NOT NULL,
    position TEXT NOT NULL,
    duration TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    current BOOLEAN DEFAULT FALSE,
    description JSONB NOT NULL DEFAULT '[]'::jsonb,
    technologies JSONB NOT NULL DEFAULT '[]'::jsonb,
    achievements JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 约束：如果current为true，则end_date应为NULL
    CONSTRAINT chk_experience_current_dates CHECK (
        (current = TRUE AND end_date IS NULL) OR
        (current = FALSE)
    ),
    -- 约束：end_date应晚于start_date
    CONSTRAINT chk_experience_dates CHECK (
        end_date IS NULL OR end_date >= start_date
    )
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_experience_company ON experience(company);
CREATE INDEX IF NOT EXISTS idx_experience_position ON experience(position);
CREATE INDEX IF NOT EXISTS idx_experience_start_date ON experience(start_date);
CREATE INDEX IF NOT EXISTS idx_experience_current ON experience(current);

-- =====================================================
-- 教育经历表 (Education)
-- =====================================================
CREATE TABLE IF NOT EXISTS education (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    institution TEXT NOT NULL,
    degree TEXT NOT NULL,
    major TEXT NOT NULL,
    duration TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    current BOOLEAN DEFAULT FALSE,
    description TEXT,
    gpa NUMERIC(3, 2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 约束：如果current为true，则end_date应为NULL
    CONSTRAINT chk_education_current_dates CHECK (
        (current = TRUE AND end_date IS NULL) OR
        (current = FALSE)
    ),
    -- 约束：end_date应晚于start_date
    CONSTRAINT chk_education_dates CHECK (
        end_date IS NULL OR end_date >= start_date
    ),
    -- 约束：GPA应在0.00到4.00之间
    CONSTRAINT chk_education_gpa CHECK (
        gpa IS NULL OR (gpa >= 0.00 AND gpa <= 4.00)
    )
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_education_institution ON education(institution);
CREATE INDEX IF NOT EXISTS idx_education_degree ON education(degree);
CREATE INDEX IF NOT EXISTS idx_education_start_date ON education(start_date);

-- =====================================================
-- 技能表 (Skill)
-- =====================================================
CREATE TABLE IF NOT EXISTS skill (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    level skill_level_enum NOT NULL,
    category TEXT NOT NULL,
    years_of_experience NUMERIC(4, 1),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 约束：years_of_experience应为正数
    CONSTRAINT chk_skill_experience CHECK (
        years_of_experience IS NULL OR years_of_experience > 0
    )
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_skill_name ON skill(name);
CREATE INDEX IF NOT EXISTS idx_skill_level ON skill(level);
CREATE INDEX IF NOT EXISTS idx_skill_category ON skill(category);

-- =====================================================
-- 项目经历表 (Project)
-- =====================================================
CREATE TABLE IF NOT EXISTS project (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    technologies JSONB NOT NULL DEFAULT '[]'::jsonb,
    duration TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    current BOOLEAN DEFAULT FALSE,
    link TEXT,
    repository TEXT,
    highlights JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 约束：如果current为true，则end_date应为NULL
    CONSTRAINT chk_project_current_dates CHECK (
        (current = TRUE AND end_date IS NULL) OR
        (current = FALSE)
    ),
    -- 约束：end_date应晚于start_date
    CONSTRAINT chk_project_dates CHECK (
        end_date IS NULL OR end_date >= start_date
    )
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_project_name ON project(name);
CREATE INDEX IF NOT EXISTS idx_project_start_date ON project(start_date);
CREATE INDEX IF NOT EXISTS idx_project_current ON project(current);

-- =====================================================
-- 证书表 (Certificate)
-- =====================================================
CREATE TABLE IF NOT EXISTS certificate (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    issuer TEXT NOT NULL,
    issue_date DATE NOT NULL,
    expiry_date DATE,
    credential_id TEXT,
    credential_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 约束：expiry_date应晚于issue_date
    CONSTRAINT chk_certificate_dates CHECK (
        expiry_date IS NULL OR expiry_date >= issue_date
    )
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_certificate_name ON certificate(name);
CREATE INDEX IF NOT EXISTS idx_certificate_issuer ON certificate(issuer);
CREATE INDEX IF NOT EXISTS idx_certificate_issue_date ON certificate(issue_date);

-- =====================================================
-- 语言能力表 (Language)
-- =====================================================
CREATE TABLE IF NOT EXISTS language (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    proficiency language_proficiency_enum NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_language_name ON language(name);
CREATE INDEX IF NOT EXISTS idx_language_proficiency ON language(proficiency);

-- =====================================================
-- 简历主表 (Resume)
-- =====================================================
CREATE TABLE IF NOT EXISTS resume (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    personal_info_id UUID NOT NULL,
    summary TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- 外键约束
    CONSTRAINT fk_resume_personal_info
        FOREIGN KEY (personal_info_id)
        REFERENCES personal_info(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 为常用查询字段创建索引
CREATE INDEX IF NOT EXISTS idx_resume_personal_info ON resume(personal_info_id);
CREATE INDEX IF NOT EXISTS idx_resume_last_updated ON resume(last_updated);

-- =====================================================
-- 简历关联表 (多对多关系)
-- =====================================================

-- 简历-工作经验关联表
CREATE TABLE IF NOT EXISTS resume_experience (
    resume_id UUID NOT NULL,
    experience_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, experience_id),
    CONSTRAINT fk_re_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_re_experience
        FOREIGN KEY (experience_id)
        REFERENCES experience(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 简历-教育经历关联表
CREATE TABLE IF NOT EXISTS resume_education (
    resume_id UUID NOT NULL,
    education_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, education_id),
    CONSTRAINT fk_re_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_re_education
        FOREIGN KEY (education_id)
        REFERENCES education(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 简历-技能关联表
CREATE TABLE IF NOT EXISTS resume_skill (
    resume_id UUID NOT NULL,
    skill_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, skill_id),
    CONSTRAINT fk_rs_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_rs_skill
        FOREIGN KEY (skill_id)
        REFERENCES skill(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 简历-项目关联表
CREATE TABLE IF NOT EXISTS resume_project (
    resume_id UUID NOT NULL,
    project_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, project_id),
    CONSTRAINT fk_rp_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_rp_project
        FOREIGN KEY (project_id)
        REFERENCES project(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 简历-证书关联表
CREATE TABLE IF NOT EXISTS resume_certificate (
    resume_id UUID NOT NULL,
    certificate_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, certificate_id),
    CONSTRAINT fk_rc_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_rc_certificate
        FOREIGN KEY (certificate_id)
        REFERENCES certificate(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- 简历-语言关联表
CREATE TABLE IF NOT EXISTS resume_language (
    resume_id UUID NOT NULL,
    language_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (resume_id, language_id),
    CONSTRAINT fk_rl_resume
        FOREIGN KEY (resume_id)
        REFERENCES resume(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_rl_language
        FOREIGN KEY (language_id)
        REFERENCES language(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- =====================================================
-- 自动更新 updated_at 触发器函数
-- =====================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为所有包含 updated_at 字段的表创建触发器
DROP TRIGGER IF EXISTS update_personal_info_updated_at ON personal_info;
CREATE TRIGGER update_personal_info_updated_at
    BEFORE UPDATE ON personal_info
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_experience_updated_at ON experience;
CREATE TRIGGER update_experience_updated_at
    BEFORE UPDATE ON experience
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_education_updated_at ON education;
CREATE TRIGGER update_education_updated_at
    BEFORE UPDATE ON education
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_skill_updated_at ON skill;
CREATE TRIGGER update_skill_updated_at
    BEFORE UPDATE ON skill
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_project_updated_at ON project;
CREATE TRIGGER update_project_updated_at
    BEFORE UPDATE ON project
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_certificate_updated_at ON certificate;
CREATE TRIGGER update_certificate_updated_at
    BEFORE UPDATE ON certificate
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_language_updated_at ON language;
CREATE TRIGGER update_language_updated_at
    BEFORE UPDATE ON language
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =====================================================
-- 自动更新 last_updated 触发器函数
-- =====================================================
CREATE OR REPLACE FUNCTION update_resume_last_updated()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为 resume 表创建触发器
DROP TRIGGER IF EXISTS update_resume_last_updated ON resume;
CREATE TRIGGER update_resume_last_updated
    BEFORE UPDATE ON resume
    FOR EACH ROW
    EXECUTE FUNCTION update_resume_last_updated();

-- =====================================================
-- JSONB 字段 GIN 索引（用于数组查询优化）
-- =====================================================
CREATE INDEX IF NOT EXISTS idx_experience_description ON experience USING GIN (description);
CREATE INDEX IF NOT EXISTS idx_experience_technologies ON experience USING GIN (technologies);
CREATE INDEX IF NOT EXISTS idx_experience_achievements ON experience USING GIN (achievements);

CREATE INDEX IF NOT EXISTS idx_project_technologies ON project USING GIN (technologies);
CREATE INDEX IF NOT EXISTS idx_project_highlights ON project USING GIN (highlights);

-- =====================================================
-- 注释
-- =====================================================

COMMENT ON TYPE skill_level_enum IS '技能等级枚举类型';
COMMENT ON TYPE language_proficiency_enum IS '语言熟练度枚举类型';

COMMENT ON TABLE personal_info IS '个人信息表';
COMMENT ON TABLE experience IS '工作经验表';
COMMENT ON TABLE education IS '教育经历表';
COMMENT ON TABLE skill IS '技能表';
COMMENT ON TABLE project IS '项目经历表';
COMMENT ON TABLE certificate IS '证书表';
COMMENT ON TABLE language IS '语言能力表';
COMMENT ON TABLE resume IS '简历主表';

COMMENT ON TABLE resume_experience IS '简历-工作经验关联表';
COMMENT ON TABLE resume_education IS '简历-教育经历关联表';
COMMENT ON TABLE resume_skill IS '简历-技能关联表';
COMMENT ON TABLE resume_project IS '简历-项目关联表';
COMMENT ON TABLE resume_certificate IS '简历-证书关联表';
COMMENT ON TABLE resume_language IS '简历-语言关联表';
