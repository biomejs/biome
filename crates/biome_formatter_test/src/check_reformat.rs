use crate::TestFormatLanguage;
use biome_diagnostics::console::fmt::{Formatter, Termcolor};
use biome_diagnostics::console::markup;
use biome_diagnostics::termcolor;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_rowan::SyntaxNode;
use std::fmt;

#[derive(Debug)]
pub enum ReformatError {
    SyntaxErrors(String),
    Format(String),
    Print(String),
    OutputMismatch {
        output_diff: String,
        ir_diff: Option<String>,
    },
}

impl fmt::Display for ReformatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxErrors(diagnostics) => {
                write!(
                    f,
                    "reformat: formatter output had syntax errors where input had none:\n{diagnostics}"
                )
            }
            Self::Format(message) => write!(f, "reformat: {message}"),
            Self::Print(message) => write!(f, "reformat: {message}"),
            Self::OutputMismatch {
                output_diff,
                ir_diff,
            } => {
                writeln!(f, "reformat: output mismatch")?;
                write!(f, "{output_diff}")?;

                if let Some(ir_diff) = ir_diff {
                    if !output_diff.ends_with('\n') {
                        writeln!(f)?;
                    }
                    writeln!(f)?;
                    writeln!(f, "IR differences:")?;
                    write!(f, "{ir_diff}")?;
                }

                Ok(())
            }
        }
    }
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
///
pub struct CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    root: &'a SyntaxNode<L::ServiceLanguage>,
    text: &'a str,
    file_name: &'a str,

    language: &'a L,
    format_language: L::FormatLanguage,
}

impl<'a, L> CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        root: &'a SyntaxNode<L::ServiceLanguage>,
        text: &'a str,
        file_name: &'a str,
        language: &'a L,
        format_language: L::FormatLanguage,
    ) -> Self {
        CheckReformat {
            root,
            text,
            file_name,
            language,
            format_language,
        }
    }

    pub fn check_reformat(&self) -> Result<(), ReformatError> {
        let re_parse = self.language.parse(self.text);

        if re_parse.has_errors() {
            let mut buffer = termcolor::Buffer::no_color();

            for diagnostic in re_parse.diagnostics() {
                let error = diagnostic
                    .clone()
                    .with_file_path(self.file_name)
                    .with_file_source_code(self.text.to_string());
                Formatter::new(&mut Termcolor(&mut buffer))
                    .write_markup(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    })
                    .expect("failed to emit diagnostic");
            }

            return Err(ReformatError::SyntaxErrors(
                std::str::from_utf8(buffer.as_slice())
                    .expect("non utf8 in error buffer")
                    .to_string(),
            ));
        }

        let re_formatted = self
            .language
            .format_node(self.format_language.clone(), &re_parse.syntax())
            .map_err(|err| ReformatError::Format(format!("failed to format: {err}")))?;

        let re_printed = re_formatted
            .print()
            .map_err(|err| ReformatError::Print(format!("failed to print: {err}")))?;

        if self.text != re_printed.as_code() {
            let input_format_element = self
                .language
                .format_node(self.format_language.clone(), self.root)
                .map_err(|err| {
                    ReformatError::Format(format!(
                        "failed to format original input for IR comparison: {err}"
                    ))
                })?;
            let pretty_reformat_ir = format!("{}", re_formatted.into_document());
            let pretty_input_ir = format!("{}", input_format_element.into_document());
            let output_diff = {
                let mut buf = Vec::new();
                similar::TextDiff::from_lines(re_printed.as_code(), self.text)
                    .unified_diff()
                    .header("re-formatted", "formatted")
                    .to_writer(&mut buf)
                    .unwrap();
                String::from_utf8(buf).unwrap()
            };
            let ir_diff = (pretty_reformat_ir != pretty_input_ir).then(|| {
                let mut buf = Vec::new();
                similar::TextDiff::from_lines(&pretty_reformat_ir, &pretty_input_ir)
                    .unified_diff()
                    .header("re-formatted IR", "formatted IR")
                    .to_writer(&mut buf)
                    .unwrap();
                String::from_utf8(buf).unwrap()
            });

            return Err(ReformatError::OutputMismatch {
                output_diff,
                ir_diff,
            });
        }

        Ok(())
    }
}
