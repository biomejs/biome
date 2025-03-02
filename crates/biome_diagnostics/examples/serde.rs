use biome_console::{ConsoleExt, EnvConsole, markup};
use biome_diagnostics::{Diagnostic, LineIndexBuf, PrintDiagnostic, Resource, Result, SourceCode};
use biome_rowan::{TextRange, TextSize};
use serde_json::Error;

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/io", tags(INTERNAL))]
struct SerdeDiagnostic {
    #[message]
    #[description]
    message: String,
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(span)]
    span: Option<TextRange>,
    #[location(source_code)]
    source_code: SourceCode<String, LineIndexBuf>,
}

impl SerdeDiagnostic {
    fn new(input: &str, error: Error) -> Self {
        let line_starts = LineIndexBuf::from_source_text(input);

        let line_index = error.line().checked_sub(1);
        let span = line_index.and_then(|line_index| {
            let line_start = line_starts.get(line_index)?;

            let column_index = error.column().checked_sub(1)?;
            let column_offset = TextSize::try_from(column_index).ok()?;

            let span_start = line_start + column_offset;
            Some(TextRange::at(span_start, TextSize::from(0)))
        });

        Self {
            message: error.to_string(),
            path: Resource::Memory,
            span,
            source_code: SourceCode {
                text: input.to_string(),
                line_starts: Some(line_starts),
            },
        }
    }
}

fn from_str(input: &str) -> Result<serde_json::Value> {
    match serde_json::from_str(input) {
        Ok(value) => Ok(value),
        Err(error) => Err(SerdeDiagnostic::new(input, error).into()),
    }
}

pub fn main() {
    if let Err(err) = from_str("{\"syntax_error\"") {
        EnvConsole::default().error(markup!({ PrintDiagnostic::verbose(&err) }));
    };
}
