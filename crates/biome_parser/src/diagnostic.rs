use crate::token_source::TokenSource;
use crate::Parser;
use biome_diagnostics::console::fmt::Display;
use biome_diagnostics::console::{markup, MarkupBuf};
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{Advices, Diagnostic, Location, LogCategory, MessageAndDescription, Visit};
use rome_rowan::{SyntaxKind, TextLen, TextRange};
use std::cmp::Ordering;

/// A specialized diagnostic for the parser
///
/// Parser diagnostics are always **errors**.
///
/// A parser diagnostics structured in this way:
/// 1. a mandatory message and a mandatory [TextRange]
/// 2. a list of details, useful to give more information and context around the error
/// 3. a hint, which should tell the user how they could fix their issue
///
/// These information **are printed in this exact order**.
///
#[derive(Debug, Diagnostic, Clone)]
#[diagnostic(category = "parse", severity = Error)]
pub struct ParseDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    message: MessageAndDescription,
    #[advice]
    advice: ParserAdvice,
}

/// Possible details related to the diagnostic
#[derive(Debug, Default, Clone)]
struct ParserAdvice {
    /// A list a possible details that can be attached to the diagnostic.
    /// Useful to explain the nature errors.
    detail_list: Vec<ParserAdviceDetail>,
    /// A message for the user that should tell the user how to fix the issue
    hint: Option<MarkupBuf>,
}

/// The structure of the advice. A message that gives details, a possible range so
/// the diagnostic is able to highlight the part of the code we want to explain.
#[derive(Debug, Clone)]
struct ParserAdviceDetail {
    /// A message that should explain this detail
    message: MarkupBuf,
    /// An optional range that should highlight the details of the code
    span: Option<TextRange>,
}

impl ParserAdvice {
    fn add_detail(&mut self, message: impl Display, range: Option<TextRange>) {
        self.detail_list.push(ParserAdviceDetail {
            message: markup! { {message} }.to_owned(),
            span: range,
        });
    }

    fn add_hint(&mut self, message: impl Display) {
        self.hint = Some(markup! { { message } }.to_owned());
    }
}

impl Advices for ParserAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for detail in &self.detail_list {
            let ParserAdviceDetail { span, message } = detail;
            visitor.record_log(LogCategory::Info, &markup! { {message} }.to_owned())?;

            let location = Location::builder().span(span).build();
            visitor.record_frame(location)?;
        }
        if let Some(hint) = &self.hint {
            visitor.record_log(LogCategory::Info, &markup! { {hint} }.to_owned())?;
        }
        Ok(())
    }
}

impl ParseDiagnostic {
    pub fn new(message: impl Display, span: impl AsSpan) -> Self {
        Self {
            span: span.as_span(),
            message: MessageAndDescription::from(markup! { {message} }.to_owned()),
            advice: ParserAdvice::default(),
        }
    }

    pub const fn is_error(&self) -> bool {
        true
    }

    /// Use this API if you want to highlight more code frame, to help to explain where's the error.
    ///
    /// A detail is printed **after the actual error** and before the hint.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use rome_console::fmt::{Termcolor};
    /// # use rome_console::markup;
    /// # use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, console::fmt::Formatter};
    /// # use biome_parser::diagnostic::ParseDiagnostic;
    /// # use rome_rowan::{TextSize, TextRange};
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new("this is wrong!", range)
    ///     .detail(TextRange::new(TextSize::from(6), TextSize::from(7)), "This is reason why it's broken");
    ///
    /// let mut write = biome_diagnostics::termcolor::Buffer::no_color();
    /// let error = diagnostic
    ///     .clone()
    ///     .with_file_path("example.js")
    ///     .with_file_source_code(source.to_string());
    /// Formatter::new(&mut Termcolor(&mut write))
    ///     .write_markup(markup! {
    ///     {PrintDiagnostic::verbose(&error)}
    /// })
    ///     .expect("failed to emit diagnostic");
    ///
    /// let mut result = String::new();
    /// write!(
    ///     result,
    ///     "{}",
    ///     std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
    /// ).expect("");
    ///
    pub fn detail(mut self, range: impl AsSpan, message: impl Display) -> Self {
        self.advice.add_detail(message, range.as_span());
        self
    }

    /// Small message that should suggest the user how they could fix the error
    ///
    /// Hints are rendered a **last part** of the diagnostics
    ///
    /// ## Examples
    ///
    /// ```
    /// # use rome_console::fmt::{Termcolor};
    /// # use rome_console::markup;
    /// # use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, console::fmt::Formatter};
    /// # use biome_parser::diagnostic::ParseDiagnostic;
    /// # use rome_rowan::{TextSize, TextRange};
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new("this is wrong!", range)
    ///     .hint("You should delete the code");
    ///
    /// let mut write = biome_diagnostics::termcolor::Buffer::no_color();
    /// let error = diagnostic
    ///     .clone()
    ///     .with_file_path("example.js")
    ///     .with_file_source_code(source.to_string());
    /// Formatter::new(&mut Termcolor(&mut write))
    ///     .write_markup(markup! {
    ///     {PrintDiagnostic::verbose(&error)}
    /// })
    ///     .expect("failed to emit diagnostic");
    ///
    /// let mut result = String::new();
    /// write!(
    ///     result,
    ///     "{}",
    ///     std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
    /// ).expect("");
    ///
    /// assert!(result.contains("× this is wrong!"));
    /// assert!(result.contains("i You should delete the code"));
    /// assert!(result.contains("> 1 │ const a"));
    /// ```
    ///
    pub fn hint(mut self, message: impl Display) -> Self {
        self.advice.add_hint(message);
        self
    }

    /// Retrieves the range that belongs to the diagnostic
    pub(crate) fn diagnostic_range(&self) -> Option<&TextRange> {
        self.span.as_ref()
    }
}

pub trait ToDiagnostic<P>
where
    P: Parser,
{
    fn into_diagnostic(self, p: &P) -> ParseDiagnostic;
}

impl<P: Parser> ToDiagnostic<P> for ParseDiagnostic {
    fn into_diagnostic(self, _: &P) -> ParseDiagnostic {
        self
    }
}

#[must_use]
pub fn expected_token<K>(token: K) -> ExpectedToken
where
    K: SyntaxKind,
{
    ExpectedToken(
        token
            .to_string()
            .expect("Expected token to be a punctuation or keyword."),
    )
}

#[must_use]
pub fn expected_token_any<K: SyntaxKind>(tokens: &[K]) -> ExpectedTokens {
    use std::fmt::Write;
    let mut expected = String::new();

    for (index, token) in tokens.iter().enumerate() {
        if index > 0 {
            expected.push_str(", ");
        }

        if index == tokens.len() - 1 {
            expected.push_str("or ");
        }

        let _ = write!(
            &mut expected,
            "'{}'",
            token
                .to_string()
                .expect("Expected token to be a punctuation or keyword.")
        );
    }

    ExpectedTokens(expected)
}

pub struct ExpectedToken(&'static str);

impl<P> ToDiagnostic<P> for ExpectedToken
where
    P: Parser,
{
    fn into_diagnostic(self, p: &P) -> ParseDiagnostic {
        if p.cur() == P::Kind::EOF {
            p.err_builder(
                format!("expected `{}` but instead the file ends", self.0),
                p.cur_range(),
            )
            .detail(p.cur_range(), "the file ends here")
        } else {
            p.err_builder(
                format!("expected `{}` but instead found `{}`", self.0, p.cur_text()),
                p.cur_range(),
            )
            .hint(format!("Remove {}", p.cur_text()))
        }
    }
}

pub struct ExpectedTokens(String);

impl<P> ToDiagnostic<P> for ExpectedTokens
where
    P: Parser,
{
    fn into_diagnostic(self, p: &P) -> ParseDiagnostic {
        if p.cur() == P::Kind::EOF {
            p.err_builder(
                format!("expected {} but instead the file ends", self.0),
                p.cur_range(),
            )
            .detail(p.cur_range(), "the file ends here")
        } else {
            p.err_builder(
                format!("expected {} but instead found `{}`", self.0, p.cur_text()),
                p.cur_range(),
            )
            .hint(format!("Remove {}", p.cur_text()))
        }
    }
}

/// Creates a diagnostic saying that the node `name` was expected at range
pub fn expected_node(name: &str, range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_single_node(name, range)
}

/// Creates a diagnostic saying that any of the nodes in `names` was expected at range
pub fn expected_any(names: &[&str], range: TextRange) -> ExpectedNodeDiagnosticBuilder {
    ExpectedNodeDiagnosticBuilder::with_any(names, range)
}

pub struct ExpectedNodeDiagnosticBuilder {
    names: String,
    range: TextRange,
}

impl ExpectedNodeDiagnosticBuilder {
    fn with_single_node(name: &str, range: TextRange) -> Self {
        ExpectedNodeDiagnosticBuilder {
            names: format!("{} {}", article_for(name), name),
            range,
        }
    }

    fn with_any(names: &[&str], range: TextRange) -> Self {
        debug_assert!(names.len() > 1, "Requires at least 2 names");

        if names.len() < 2 {
            return Self::with_single_node(names.first().unwrap_or(&"<missing>"), range);
        }

        let mut joined_names = String::new();

        for (index, name) in names.iter().enumerate() {
            if index > 0 {
                joined_names.push_str(", ");
            }

            if index == names.len() - 1 {
                joined_names.push_str("or ");
            }

            joined_names.push_str(article_for(name));
            joined_names.push(' ');
            joined_names.push_str(name);
        }

        Self {
            names: joined_names,
            range,
        }
    }
}

impl<P: Parser> ToDiagnostic<P> for ExpectedNodeDiagnosticBuilder {
    fn into_diagnostic(self, p: &P) -> ParseDiagnostic {
        let range = &self.range;

        let msg = if p.source().text().text_len() <= range.start() {
            format!(
                "expected {} but instead found the end of the file",
                self.names
            )
        } else {
            format!(
                "expected {} but instead found '{}'",
                self.names,
                p.text(*range)
            )
        };

        let diag = p.err_builder(msg, self.range);
        diag.detail(self.range, format!("Expected {} here", self.names))
    }
}

fn article_for(name: &str) -> &'static str {
    match name.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

/// Merges two lists of parser diagnostics. Only keeps the error from the first collection if two start at the same range.
///
/// The two lists must be so sorted by their source range in increasing order.
pub fn merge_diagnostics(
    first: Vec<ParseDiagnostic>,
    second: Vec<ParseDiagnostic>,
) -> Vec<ParseDiagnostic> {
    if first.is_empty() {
        return second;
    }

    if second.is_empty() {
        return first;
    }

    let mut merged = Vec::new();

    let mut first_iter = first.into_iter();
    let mut second_iter = second.into_iter();

    let mut current_first: Option<ParseDiagnostic> = first_iter.next();
    let mut current_second: Option<ParseDiagnostic> = second_iter.next();

    loop {
        match (current_first, current_second) {
            (Some(first_item), Some(second_item)) => {
                let (first, second) = match (
                    first_item.diagnostic_range(),
                    second_item.diagnostic_range(),
                ) {
                    (Some(first_range), Some(second_range)) => {
                        match first_range.start().cmp(&second_range.start()) {
                            Ordering::Less => {
                                merged.push(first_item);
                                (first_iter.next(), Some(second_item))
                            }
                            Ordering::Equal => {
                                // Only keep one error, skip the one from the second list.
                                (Some(first_item), second_iter.next())
                            }
                            Ordering::Greater => {
                                merged.push(second_item);
                                (Some(first_item), second_iter.next())
                            }
                        }
                    }
                    (Some(_), None) => {
                        merged.push(second_item);
                        (Some(first_item), second_iter.next())
                    }
                    (None, Some(_)) => {
                        merged.push(first_item);
                        (first_iter.next(), Some(second_item))
                    }
                    (None, None) => {
                        merged.push(first_item);
                        merged.push(second_item);

                        (first_iter.next(), second_iter.next())
                    }
                };

                current_first = first;
                current_second = second;
            }

            (None, None) => return merged,
            (Some(first_item), None) => {
                merged.push(first_item);
                merged.extend(first_iter);
                return merged;
            }
            (None, Some(second_item)) => {
                merged.push(second_item);
                merged.extend(second_iter);
                return merged;
            }
        }
    }
}
