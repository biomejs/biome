use std::io;

use biome_console::fmt::{Formatter, HTML, Termcolor};
use biome_console::markup;
use biome_diagnostics::{
    Advices, DiagnosticTags, LogCategory, MessageAndDescription, PrintDiagnostic, Visit,
};
use biome_diagnostics_macros::Diagnostic;
use biome_text_edit::TextEdit;
use biome_text_size::TextRange;

#[derive(Diagnostic, Debug)]
#[diagnostic(
    severity = Error,
    category = "check"
)]
struct TestDiagnostic {
    #[message]
    #[description]
    message_and_description: MessageAndDescription,

    #[tags]
    tags: DiagnosticTags,

    #[location(resource)]
    path: String,

    #[location(source_code)]
    source_code: String,

    #[location(span)]
    span: TextRange,
}

impl Default for TestDiagnostic {
    fn default() -> Self {
        Self {
            message_and_description: MessageAndDescription::from(markup!(
                "This is the message of the diagnostic. It will appear in different colours based on the severity of the diagnostic."
            ).to_owned()),
            tags: DiagnosticTags::FIXABLE,
            path: "code_block".to_string(),
            source_code: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
            span: TextRange::new(10.into(), 20.into())
        }
    }
}

#[derive(Diagnostic, Debug)]
#[diagnostic(
    severity = Error,
    category = "check"
)]
struct LongLineDiagnostic {
    #[message]
    #[description]
    message_and_description: MessageAndDescription,

    #[location(resource)]
    path: String,

    #[location(source_code)]
    source_code: String,

    #[location(span)]
    span: TextRange,

    #[advice]
    advice: LongLineAdvice,
}

#[derive(Debug)]
struct LongLineAdvice {
    source_code: String,
    fixed_code: String,
}

impl Advices for LongLineAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(LogCategory::Info, &markup! { "Unsafe fix: Remove target" })?;

        let diff = TextEdit::from_unicode_words(&self.source_code, &self.fixed_code);
        visitor.record_diff(&diff)
    }
}

impl LongLineDiagnostic {
    fn new(prefix: &str, suffix: &str) -> Self {
        Self::with_target(prefix, "debugger;", suffix)
    }

    fn with_target(prefix: &str, target: &str, suffix: &str) -> Self {
        let source_code = format!("{prefix}{target}{suffix}");
        let fixed_code = format!("{prefix}{suffix}");
        let span_start = prefix.len().try_into().unwrap();
        let span_end = (prefix.len() + target.len()).try_into().unwrap();

        Self {
            message_and_description: MessageAndDescription::from(
                markup!("This is an unexpected use of the debugger statement.").to_owned(),
            ),
            path: "long-line.js".to_string(),
            source_code: source_code.clone(),
            span: TextRange::new(span_start, span_end),
            advice: LongLineAdvice {
                source_code,
                fixed_code,
            },
        }
    }
}

fn print_diagnostic(diag: &impl biome_diagnostics::Diagnostic) -> String {
    let mut buffer = biome_diagnostics::termcolor::Buffer::no_color();

    Formatter::new(&mut Termcolor(&mut buffer))
        .write_markup(markup! {
            {PrintDiagnostic::verbose(diag)}
        })
        .unwrap();

    String::from_utf8(buffer.into_inner()).unwrap()
}

#[test]
fn console_print_diagnostic() {}

#[test]
fn truncates_long_diagnostic_lines_with_left_ellipsis() {
    let content = print_diagnostic(&LongLineDiagnostic::new(&"a++;".repeat(150), ""));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn truncates_long_diagnostic_span_with_left_ellipsis() {
    let content = print_diagnostic(&LongLineDiagnostic::with_target(
        &"a++;".repeat(150),
        &format!("0b{}", "0".repeat(60)),
        "",
    ));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn truncates_long_diagnostic_span_with_right_ellipsis() {
    let content = print_diagnostic(&LongLineDiagnostic::with_target(
        "var x = ",
        &format!("0b{}", "0".repeat(600)),
        &";a--;".repeat(150),
    ));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn truncates_long_diagnostic_span_with_both_ellipses() {
    let content = print_diagnostic(&LongLineDiagnostic::with_target(
        &"a++;".repeat(150),
        &format!("0b{}", "0".repeat(600)),
        &";a--;".repeat(150),
    ));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn truncates_long_diagnostic_lines_with_right_ellipsis() {
    let content = print_diagnostic(&LongLineDiagnostic::new("", &"a--;".repeat(150)));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn truncates_long_diagnostic_lines_with_both_ellipses() {
    let content = print_diagnostic(&LongLineDiagnostic::new(
        &"a++;".repeat(150),
        &"a--;".repeat(150),
    ));

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn html_print_diagnostic() {
    let mut content = vec![];
    let mut writer = HTML::new(&mut content);

    Formatter::new(&mut writer)
        .write_markup(markup! {
            {PrintDiagnostic::simple(&TestDiagnostic::default())}
        })
        .unwrap();

    let content = String::from_utf8(content).unwrap();
    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}

#[test]
fn html_mdx_print_diagnostic() {
    let mut content = vec![];
    let mut writer = HTML::new(&mut content).with_mdx();

    Formatter::new(&mut writer)
        .write_markup(markup! {
            {PrintDiagnostic::simple(&TestDiagnostic::default())}
        })
        .unwrap();

    let content = String::from_utf8(content).unwrap();
    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}
