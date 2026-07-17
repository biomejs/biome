mod comments;
mod convert_ast;
mod convert_scope;
mod error;
mod prefilter;

use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, TextRange, TextSize};
use biome_languages::JsFileSource;
use biome_line_index::{LineIndex, WideEncoding, WideLineCol};
use biome_rowan::AstNode;
use react_compiler::entrypoint::compile_result::{CompileResult, LoggerEvent};
use react_compiler::entrypoint::plugin_options::{CompilerTarget, PluginOptions};
use react_compiler_ast::{File, scope::ScopeInfo};
use react_compiler_hir::environment_config::{EnvironmentConfig, ExhaustiveEffectDepsMode};

pub use crate::error::{ReactCompilerError, Result};
pub use react_compiler::entrypoint::compile_result::CompilerErrorDetailInfo;

const COMPILER_STACK_SIZE: usize = 64 * 1024 * 1024;

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
    let root =
        input.root.syntax().as_send().ok_or_else(|| {
            ReactCompilerError::CompilerOutput("expected root syntax node".into())
        })?;
    run_on_compiler_stack(move || {
        let root = root
            .into_language_root::<AnyJsRoot>()
            .ok_or_else(|| ReactCompilerError::CompilerOutput("invalid JavaScript root".into()))?;
        convert_ast::convert_file(&root, input.source, input.source_type)
    })
}

pub fn convert_scope_info(input: ScopeInput<'_>) -> Result<ScopeInfo> {
    run_on_compiler_stack(|| Ok(convert_scope::convert_scope_info(input.model)))
}

pub fn compile_program(input: CompileInput<'_>) -> Result<CompileOutput> {
    // In `infer` mode the compiler only analyzes functions that follow React
    // naming conventions, so a file without any react-like function can be
    // skipped before paying for AST/scope conversion and the compiler thread.
    // `annotation` mode is deliberately not prefiltered (unlike upstream):
    // a `"use memo"` directive can opt in any function, regardless of name.
    if input.options.compilation_mode == "infer" && !prefilter::has_react_like_functions(input.root)
    {
        return Ok(CompileOutput {
            file: None,
            diagnostics: Vec::new(),
            events: Vec::new(),
        });
    }

    let root =
        input.root.syntax().as_send().ok_or_else(|| {
            ReactCompilerError::CompilerOutput("expected root syntax node".into())
        })?;
    run_on_compiler_stack(move || {
        let root = root
            .into_language_root::<AnyJsRoot>()
            .ok_or_else(|| ReactCompilerError::CompilerOutput("invalid JavaScript root".into()))?;
        let file = convert_ast::convert_file(&root, input.source, input.source_type)?;
        let scope_info = convert_scope::convert_scope_info(input.model);
        let result =
            react_compiler::entrypoint::program::compile_program(file, scope_info, input.options);

        let line_index = LineIndex::new(input.source);
        Ok(compile_result_to_output(result, &line_index))
    })
}

/// Runs `f` on a dedicated thread with a stack large enough for the React
/// Compiler's recursion; its upstream entrypoint budgets the same
/// [`COMPILER_STACK_SIZE`]. A panic on the worker is re-raised on the calling
/// thread, where the analyzer's existing panic handling takes over.
fn run_on_compiler_stack<T, F>(f: F) -> Result<T>
where
    T: Send,
    F: FnOnce() -> Result<T> + Send,
{
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("biome-react-compiler".to_string())
            .stack_size(COMPILER_STACK_SIZE)
            .spawn_scoped(scope, f)
            .map_err(|error| ReactCompilerError::CompilerOutput(error.to_string()))?;

        match handle.join() {
            Ok(result) => result,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    })
}

pub fn default_lint_options(source: &str) -> PluginOptions {
    let environment = EnvironmentConfig {
        validate_exhaustive_effect_dependencies: ExhaustiveEffectDepsMode::All,
        validate_no_set_state_in_effects: true,
        validate_no_derived_computations_in_effects: true,
        validate_no_jsx_in_try_statements: true,
        validate_static_components: true,
        validate_no_capitalized_calls: Some(Vec::new()),
        validate_no_impure_functions_in_render: true,
        validate_no_freezing_known_mutable_functions: true,
        enable_treat_set_identifiers_as_state_setters: true,
        ..EnvironmentConfig::default()
    };

    PluginOptions {
        should_compile: true,
        enable_reanimated: false,
        is_dev: true,
        filename: None,
        compilation_mode: "infer".to_string(),
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
        environment,
        source_code: Some(source.to_string()),
        profiling: false,
        debug: false,
    }
}

fn compile_result_to_output(result: CompileResult, line_index: &LineIndex) -> CompileOutput {
    match result {
        CompileResult::Success { ast, events, .. } => {
            // `ast` is returned by value as a typed `File`, so no JSON round-trip
            // is needed here anymore.
            let diagnostics = diagnostics_from_events(&events, line_index);
            CompileOutput {
                file: ast,
                diagnostics,
                events,
            }
        }
        CompileResult::Error { error, events, .. } => {
            let mut diagnostics = diagnostics_from_events(&events, line_index);
            diagnostics.push(ReactCompilerError::CompilerOutput(error.reason));
            CompileOutput {
                file: None,
                diagnostics,
                events,
            }
        }
    }
}

fn diagnostics_from_events(
    events: &[LoggerEvent],
    line_index: &LineIndex,
) -> Vec<ReactCompilerError> {
    events
        .iter()
        .filter_map(|event| match event {
            LoggerEvent::CompileError { detail, .. }
            | LoggerEvent::CompileErrorWithLoc { detail, .. } => {
                Some(ReactCompilerError::CompilerDiagnostic {
                    range: detail
                        .loc
                        .as_ref()
                        .and_then(|loc| text_range_from_logger_location(line_index, loc))
                        .or_else(|| {
                            detail.details.as_ref()?.iter().find_map(|detail| {
                                detail.loc.as_ref().and_then(|loc| {
                                    text_range_from_logger_location(line_index, loc)
                                })
                            })
                        }),
                    detail: Box::new(detail.clone()),
                })
            }
            _ => None,
        })
        .collect()
}

fn text_range_from_logger_location(
    line_index: &LineIndex,
    loc: &react_compiler::entrypoint::compile_result::LoggerSourceLocation,
) -> Option<TextRange> {
    let start = offset_from_logger_position(line_index, &loc.start)?;
    let end = offset_from_logger_position(line_index, &loc.end)?;
    Some(TextRange::new(start, end))
}

/// Converts a Babel-style position (1-based line, 0-based UTF-16 column) back
/// into a UTF-8 byte offset.
fn offset_from_logger_position(
    line_index: &LineIndex,
    position: &react_compiler::entrypoint::compile_result::LoggerPosition,
) -> Option<TextSize> {
    let line = position.line.checked_sub(1)?;
    let line_col = line_index.to_utf8(
        WideEncoding::Utf16,
        WideLineCol {
            line,
            col: position.column,
        },
    );
    line_index.offset(line_col)
}
