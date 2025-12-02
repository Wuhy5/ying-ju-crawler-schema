//! 规则验证
//!
//! 提供运行时的规则验证功能

use crawler_schema::CrawlerRule;

/// 规则验证 Trait
pub trait RuleValidate {
    /// 验证规则，返回错误列表（空表示通过）
    fn validate(&self) -> Vec<ValidationError>;
}

/// 验证错误
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// 必填字段缺失
    MissingRequiredField { field: String },
    /// 无效的URL模板
    InvalidUrlTemplate { url: String, reason: String },
    /// 无效的选择器
    InvalidSelector { selector: String, reason: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequiredField { field } => write!(f, "必填字段缺失: {}", field),
            Self::InvalidUrlTemplate { url, reason } => {
                write!(f, "无效的URL模板 '{}': {}", url, reason)
            }
            Self::InvalidSelector { selector, reason } => {
                write!(f, "无效的选择器 '{}': {}", selector, reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl RuleValidate for CrawlerRule {
    fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // 验证元数据
        if self.meta.name.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                field: "meta.name".to_string(),
            });
        }

        if self.meta.domain.is_empty() {
            errors.push(ValidationError::MissingRequiredField {
                field: "meta.domain".to_string(),
            });
        }

        // TODO: 添加更多验证逻辑
        // - 验证模板语法
        // - 验证选择器语法
        // - 验证字段提取器

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::MissingRequiredField {
            field: "meta.name".to_string(),
        };
        assert!(error.to_string().contains("meta.name"));
    }
}

