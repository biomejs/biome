use crate::TestFormatLanguage;
use biome_diagnostics::console::fmt::{Formatter, Termcolor};
use biome_diagnostics::console::markup;
use biome_diagnostics::termcolor;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_rowan::SyntaxNode;

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

    pub fn check_reformat(&self) {
        let re_parse = self.language.parse(self.text);

        // Panic if the result from the formatter has syntax errors
        if re_parse.has_errors() {
            let mut buffer = termcolor::Buffer::ansi();

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

            panic!(
                "formatter output had syntax errors where input had none:\n{}",
                std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
            )
        }

        let re_formatted = match self
            .language
            .format_node(self.format_language.clone(), &re_parse.syntax())
        {
            Ok(formatted) => formatted,
            Err(err) => {
                panic!("failed to format: {err}");
            }
        };

        let re_printed = re_formatted.print().unwrap();

        if self.text != re_printed.as_code() {
            let input_format_element = self
                .language
                .format_node(self.format_language.clone(), self.root)
                .unwrap();
            let pretty_reformat_ir = format!("{}", re_formatted.into_document());
            let pretty_input_ir = format!("{}", input_format_element.into_document());

            // Print a diff of the Formatter IR emitted for the input and the output
            let diff = similar_asserts::SimpleDiff::from_str(
                &pretty_input_ir,
                &pretty_reformat_ir,
                "input",
                "output",
            );
            println!("{diff}");

            similar_asserts::assert_eq!(
                re_printed.as_code(),
                self.text,
                "left is the re-formatted"
            );
            similar_asserts::assert_eq!(
                pretty_reformat_ir,
                pretty_input_ir,
                "left is the re-formatted"
            );
        }
    }
}
