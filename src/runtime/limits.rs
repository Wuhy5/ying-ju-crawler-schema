//! 运行时限制检查
//!
//! 提供运行时资源限制的检查功能。

use crate::{config::RuntimeLimits, error::CrawlerError};

/// 限制检查扩展 trait
pub trait LimitsExt {
    /// 验证管道长度
    fn check_pipeline_length(&self, length: usize) -> Result<(), CrawlerError>;

    /// 验证递归深度
    fn check_recursion_depth(&self, depth: usize) -> Result<(), CrawlerError>;

    /// 验证变量数量
    fn check_variable_count(&self, count: usize) -> Result<(), CrawlerError>;

    /// 验证循环迭代次数
    fn check_loop_iterations(&self, iterations: usize) -> Result<(), CrawlerError>;
}

impl LimitsExt for RuntimeLimits {
    fn check_pipeline_length(&self, length: usize) -> Result<(), CrawlerError> {
        if length > self.max_pipeline_length {
            Err(CrawlerError::ResourceLimitExceeded {
                limit_type: "管道长度".to_string(),
                current: length,
                max: self.max_pipeline_length,
            })
        } else {
            Ok(())
        }
    }

    fn check_recursion_depth(&self, depth: usize) -> Result<(), CrawlerError> {
        if depth > self.max_recursion_depth {
            Err(CrawlerError::RecursionLimitExceeded {
                current: depth,
                max: self.max_recursion_depth,
            })
        } else {
            Ok(())
        }
    }

    fn check_variable_count(&self, count: usize) -> Result<(), CrawlerError> {
        if count > self.max_variables {
            Err(CrawlerError::ResourceLimitExceeded {
                limit_type: "变量数量".to_string(),
                current: count,
                max: self.max_variables,
            })
        } else {
            Ok(())
        }
    }

    fn check_loop_iterations(&self, iterations: usize) -> Result<(), CrawlerError> {
        if iterations > self.max_loop_iterations {
            Err(CrawlerError::ResourceLimitExceeded {
                limit_type: "循环迭代次数".to_string(),
                current: iterations,
                max: self.max_loop_iterations,
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pipeline_length() {
        let limits = RuntimeLimits::default();
        assert!(limits.check_pipeline_length(50).is_ok());
        assert!(limits.check_pipeline_length(200).is_err());
    }

    #[test]
    fn test_check_recursion_depth() {
        let limits = RuntimeLimits::default();
        assert!(limits.check_recursion_depth(5).is_ok());
        assert!(limits.check_recursion_depth(20).is_err());
    }
}
