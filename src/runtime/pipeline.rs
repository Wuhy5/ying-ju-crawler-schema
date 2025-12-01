//! 管道运行时扩展
//!
//! 提供管道的运行时验证和分析功能。

use std::collections::HashSet;

use super::TemplateExt;
use crate::{
    error::{CrawlerError, ValidationErrors},
    schema::{Pipeline, Step, StepTrait},
};

/// 管道扩展 trait
/// 提供管道的运行时验证和分析功能
pub trait PipelineExt {
    /// 验证管道中的所有步骤
    fn validate(&self) -> Result<(), CrawlerError>;

    /// 获取管道中所有步骤的输出变量
    fn output_variables(&self) -> HashSet<String>;

    /// 获取管道依赖的所有外部变量（需要从外部上下文提供的变量）
    fn required_external_variables(&self) -> HashSet<String>;
}

impl PipelineExt for Pipeline {
    fn validate(&self) -> Result<(), CrawlerError> {
        let mut errors = ValidationErrors::new();

        for (index, step) in self.iter().enumerate() {
            // 验证每个步骤中的模板语法
            for template in step.templates() {
                if let Err(e) = template.validate() {
                    errors.push(CrawlerError::PipelineValidation {
                        step_index: index,
                        message: e.to_string(),
                    });
                }
            }

            // 递归验证嵌套的管道（如 LoopForEach）
            if let Step::LoopForEach(loop_step) = step
                && let Err(e) = loop_step.pipeline.validate()
            {
                errors.push(CrawlerError::PipelineValidation {
                    step_index: index,
                    message: format!("子管道验证失败: {}", e),
                });
            }
        }

        errors.into_result()
    }

    fn output_variables(&self) -> HashSet<String> {
        self.iter()
            .filter_map(|step| step.output_variable())
            .map(|s| s.to_string())
            .collect()
    }

    fn required_external_variables(&self) -> HashSet<String> {
        let mut required: HashSet<String> = HashSet::new();
        let mut defined: HashSet<String> = HashSet::new();

        for step in self.iter() {
            // 收集此步骤需要的变量（排除已定义的）
            for template in step.templates() {
                for var in template.extract_variables() {
                    let root_var = var.split('.').next().unwrap_or(&var);
                    let root_var = root_var.split('[').next().unwrap_or(root_var);
                    if !defined.contains(root_var) {
                        required.insert(root_var.to_string());
                    }
                }
            }

            // 添加此步骤定义的变量
            if let Some(output) = step.output_variable() {
                defined.insert(output.to_string());
            }
        }

        required
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pipeline::StepHttpRequest, schema::template::Template};

    #[test]
    fn test_output_variables() {
        let pipeline: Pipeline = vec![Step::HttpRequest(StepHttpRequest {
            url: Template::new("https://example.com"),
            output: "response".to_string(),
            method: None,
            body: None,
            headers: None,
        })];

        let outputs = pipeline.output_variables();
        assert!(outputs.contains("response"));
    }

    #[test]
    fn test_required_external_variables() {
        let pipeline: Pipeline = vec![Step::HttpRequest(StepHttpRequest {
            url: Template::new("https://example.com/{{ page }}"),
            output: "response".to_string(),
            method: None,
            body: None,
            headers: None,
        })];

        let required = pipeline.required_external_variables();
        assert!(required.contains("page"));
    }
}
