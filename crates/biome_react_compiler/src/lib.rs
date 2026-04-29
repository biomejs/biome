mod comments;
mod convert_ast;
mod convert_scope;
mod error;

use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsFileSource, TextRange, TextSize};
use react_compiler::entrypoint::compile_result::{CompileResult, LoggerEvent};
use react_compiler::entrypoint::plugin_options::{CompilerTarget, PluginOptions};
use react_compiler_ast::{File, scope::ScopeInfo};
use react_compiler_hir::environment_config::EnvironmentConfig;

pub use crate::error::{ReactCompilerError, Result};
pub use react_compiler::entrypoint::compile_result::CompilerErrorDetailInfo;

pub struct ConvertInput<'a> {
    pub root: &'a AnyJsRoot,
    pub source: &'a str,
    pub source_type: JsFileSource,
}

pub struct ScopeInput<'a> {
    pub model: &'a SemanticModel,
}

pub struct CompileInput<'a> {
    pub root: &'a AnyJsRoot,
    pub model: &'a SemanticModel,
    pub source: &'a str,
    pub source_type: JsFileSource,
    pub options: PluginOptions,
}

#[derive(Debug, Clone)]
pub struct CompileOutput {
    pub file: Option<File>,
    pub diagnostics: Vec<ReactCompilerError>,
    pub events: Vec<LoggerEvent>,
}

pub fn convert_file(input: ConvertInput<'_>) -> Result<File> {
    convert_ast::convert_file(input.root, input.source, input.source_type)
}

pub fn convert_scope_info(input: ScopeInput<'_>) -> Result<ScopeInfo> {
    Ok(convert_scope::convert_scope_info(input.model))
}

pub fn compile_program(input: CompileInput<'_>) -> Result<CompileOutput> {
    let file = convert_file(ConvertInput {
        root: input.root,
        source: input.source,
        source_type: input.source_type,
    })?;
    let scope_info = convert_scope_info(ScopeInput { model: input.model })?;
    let result =
        react_compiler::entrypoint::program::compile_program(file, scope_info, input.options);

    Ok(compile_result_to_output(result))
}

pub fn default_lint_options(source: &str) -> PluginOptions {
    PluginOptions {
        should_compile: true,
        enable_reanimated: false,
        is_dev: true,
        filename: None,
        compilation_mode: "all".to_string(),
        panic_threshold: "none".to_string(),
        target: CompilerTarget::Version("19".to_string()),
        gating: None,
        dynamic_gating: None,
        no_emit: true,
        output_mode: Some("lint".to_string()),
        eslint_suppression_rules: None,
        flow_suppressions: true,
        ignore_use_no_forget: false,
        custom_opt_out_directives: None,
        environment: EnvironmentConfig::default(),
        source_code: Some(source.to_string()),
        profiling: false,
        debug: false,
    }
}

fn compile_result_to_output(result: CompileResult) -> CompileOutput {
    match result {
        CompileResult::Success { ast, events, .. } => {
            let mut diagnostics = diagnostics_from_events(&events);
            let file = ast.and_then(|raw_json| {
                let value = match serde_json::from_str::<serde_json::Value>(raw_json.get()) {
                    Ok(value) => value,
                    Err(error) => {
                        diagnostics.push(ReactCompilerError::CompilerOutput(error.to_string()));
                        return None;
                    }
                };

                match serde_json::from_value(value) {
                    Ok(file) => Some(file),
                    Err(error) => {
                        diagnostics.push(ReactCompilerError::CompilerOutput(error.to_string()));
                        None
                    }
                }
            });

            CompileOutput {
                file,
                diagnostics,
                events,
            }
        }
        CompileResult::Error { error, events, .. } => {
            let mut diagnostics = diagnostics_from_events(&events);
            diagnostics.push(ReactCompilerError::CompilerOutput(error.reason));
            CompileOutput {
                file: None,
                diagnostics,
                events,
            }
        }
    }
}

fn diagnostics_from_events(events: &[LoggerEvent]) -> Vec<ReactCompilerError> {
    events
        .iter()
        .filter_map(|event| match event {
            LoggerEvent::CompileError { detail, .. }
            | LoggerEvent::CompileErrorWithLoc { detail, .. } => {
                Some(ReactCompilerError::CompilerDiagnostic {
                    range: detail
                        .loc
                        .as_ref()
                        .and_then(text_range_from_logger_location)
                        .or_else(|| {
                            detail.details.as_ref()?.iter().find_map(|detail| {
                                detail
                                    .loc
                                    .as_ref()
                                    .and_then(text_range_from_logger_location)
                            })
                        }),
                    detail: detail.clone(),
                })
            }
            _ => None,
        })
        .collect()
}

fn text_range_from_logger_location(
    loc: &react_compiler::entrypoint::compile_result::LoggerSourceLocation,
) -> Option<TextRange> {
    Some(TextRange::new(
        TextSize::from(loc.start.index?),
        TextSize::from(loc.end.index?),
    ))
}
