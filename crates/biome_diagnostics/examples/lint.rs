use std::io;

use biome_console::{markup, ConsoleExt, EnvConsole};
use biome_diagnostics::{
    Advices, Diagnostic, Location, LogCategory, PrintDiagnostic, Resource, SourceCode, Visit,
};
use biome_rowan::{TextRange, TextSize};
use biome_text_edit::TextEdit;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    message = "Redundant constant reference",
    tags(FIXABLE)
)]
struct LintDiagnostic {
    #[location(resource)]
    path: String,
    #[location(span)]
    span: TextRange,
    #[location(source_code)]
    source_code: String,
    #[advice]
    advices: LintAdvices,
    #[verbose_advice]
    verbose_advices: LintVerboseAdvices,
}

#[derive(Debug)]
struct LintAdvices {
    path: String,
    declaration_span: TextRange,
    source_code: String,
    code_action: TextEdit,
}

impl Advices for LintAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(
            LogCategory::Info,
            &"You should avoid declaring constants with a string that's the same value as the variable name. It introduces a level of unnecessary indirection when it's only two additional characters to inline."
        )?;

        visitor.record_log(LogCategory::Info, &"This constant is declared here")?;
        visitor.record_frame(Location {
            resource: Some(Resource::File(&self.path)),
            span: Some(self.declaration_span),
            source_code: Some(SourceCode {
                text: &self.source_code,
                line_starts: None,
            }),
        })?;

        visitor.record_log(LogCategory::Info, &"Safe Fix")?;
        visitor.record_diff(&self.code_action)
    }
}

#[derive(Debug)]
struct LintVerboseAdvices {
    path: String,
}

impl Advices for LintVerboseAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(LogCategory::Info, &"Apply this fix using `--apply`:")?;
        visitor.record_command(&format!("biome check --write {}", self.path))
    }
}

pub fn main() {
    const SOURCE: &str = "const FOO = \"FOO\";
console.log(FOO);";

    const FIXED: &str = "console.log(\"FOO\");";

    let diag = LintDiagnostic {
        path: String::from("style/noShoutyConstants.js"),
        span: TextRange::at(TextSize::from(31), TextSize::from(3)),
        source_code: String::from(SOURCE),
        advices: LintAdvices {
            path: String::from("style/noShoutyConstants.js"),
            declaration_span: TextRange::at(TextSize::from(6), TextSize::from(3)),
            source_code: String::from(SOURCE),
            code_action: TextEdit::from_unicode_words(SOURCE, FIXED),
        },
        verbose_advices: LintVerboseAdvices {
            path: String::from("style/noShoutyConstants.js"),
        },
    };

    EnvConsole::default().error(markup!({ PrintDiagnostic::verbose(&diag) }));
}
