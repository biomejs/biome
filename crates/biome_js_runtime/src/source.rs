use std::slice;
use std::sync::Arc;

use boa_engine::{JsNativeError, JsResult};
use camino::Utf8Path;

use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_diagnostics::PrintDescription;
use biome_js_parser::JsParserOptions;
use biome_languages::JsFileSource;
use biome_resolver::FsWithResolverProxy;

/// Reads the ECMAScript source of the module at `path`.
///
/// TypeScript modules are transpiled by erasing their types, so plugins can be written in
/// TypeScript. Only *erasable* syntax is supported: constructs generating runtime code, like
/// `enum` or `namespace`, are reported as an error.
pub(crate) fn read_module_source(
    fs: &Arc<dyn FsWithResolverProxy>,
    path: &Utf8Path,
) -> JsResult<String> {
    let source = fs.read_file_from_path(path).map_err(|err| {
        JsNativeError::error().with_message(format!("Failed to read {path}: {err}"))
    })?;

    let source_type = JsFileSource::try_from(path).unwrap_or_else(|_| JsFileSource::js_module());
    if !source_type.is_typescript() {
        return Ok(source);
    }

    strip_types(&source, source_type).map_err(|message| {
        JsNativeError::syntax()
            .with_message(format!("{path}: {message}"))
            .into()
    })
}

/// Erases the TypeScript-only syntax of `source` by running the `stripTypes` transformation.
///
/// The output has the same length as the input: erased syntax is replaced with whitespace, so
/// positions are preserved and the module needs no source map.
fn strip_types(source: &str, source_type: JsFileSource) -> Result<String, String> {
    let parsed = biome_js_parser::parse(source, source_type, JsParserOptions::default());
    if parsed.has_errors() {
        return Err("failed to parse the TypeScript source".to_string());
    }

    let rule = RuleFilter::Rule("transformations", "stripTypes");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule)),
        ..AnalysisFilter::default()
    };

    let mut output = source.as_bytes().to_vec();
    let mut errors = Vec::new();

    let (_, analyzer_errors) = biome_js_transform::transform(
        &parsed.tree(),
        filter,
        &AnalyzerOptions::default(),
        source_type,
        |signal| {
            if let Some(diagnostic) = signal.diagnostic() {
                errors.push(PrintDescription(&diagnostic).to_string());
            }

            // Each transformation rewrites the whole source, blanking out the range it erases.
            // Since the length is preserved, the transformations are merged bytewise.
            for transformation in signal.transformations() {
                let stripped = transformation.mutation.commit().to_string();
                for ((byte, original), stripped) in
                    output.iter_mut().zip(source.bytes()).zip(stripped.bytes())
                {
                    if stripped != original {
                        *byte = stripped;
                    }
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    errors.extend(
        analyzer_errors
            .iter()
            .map(|error| PrintDescription(error).to_string()),
    );

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    Ok(String::from_utf8(output).expect("erasing types must keep the source valid UTF-8"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_types_erases_every_type() {
        let source = "type A = number;\nfunction f(a: A): A {\n  return a as A;\n}\n";

        let stripped = strip_types(source, JsFileSource::ts()).expect("types are erasable");

        assert_eq!(
            stripped,
            "                \nfunction f(a   )    {\n  return a     ;\n}\n"
        );
        assert_eq!(stripped.len(), source.len());
    }

    #[test]
    fn strip_types_reports_syntax_generating_runtime_code() {
        let error = strip_types("enum Foo { Lorem }", JsFileSource::ts())
            .expect_err("`enum` can't be erased");

        assert_eq!(
            error,
            "enum declarations cannot be stripped because they generate runtime code."
        );
    }
}
