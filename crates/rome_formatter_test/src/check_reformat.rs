use crate::TestFormatLanguage;
use biome_diagnostics::console::fmt::{Formatter, Termcolor};
use biome_diagnostics::console::markup;
use biome_diagnostics::termcolor;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use rome_rowan::SyntaxNode;

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
///
pub struct CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    root: &'a SyntaxNode<L::SyntaxLanguage>,
    text: &'a str,
    file_name: &'a str,

    language: &'a L,
    options: L::Options,
}

impl<'a, L> CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        root: &'a SyntaxNode<L::SyntaxLanguage>,
        text: &'a str,
        file_name: &'a str,

        language: &'a L,
        options: L::Options,
    ) -> Self {
        CheckReformat {
            root,
            text,
            file_name,

            language,
            options,
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

        let formatted = self
            .language
            .format_node(self.options.clone(), &re_parse.syntax())
            .unwrap();

        let printed = formatted.print().unwrap();

        if self.text != printed.as_code() {
            let input_format_element = self
                .language
                .format_node(self.options.clone(), self.root)
                .unwrap();
            let pretty_input_ir = format!("{}", formatted.into_document());
            let pretty_reformat_ir = format!("{}", input_format_element.into_document());

            // Print a diff of the Formatter IR emitted for the input and the output
            let diff = similar_asserts::SimpleDiff::from_str(
                &pretty_input_ir,
                &pretty_reformat_ir,
                "input",
                "output",
            );

            println!("{diff}");

            similar_asserts::assert_eq!(self.text, printed.as_code());
        }
    }
}
