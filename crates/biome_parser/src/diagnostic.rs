use crate::token_source::TokenSource;
use crate::Parser;
use biome_diagnostics::console::fmt::Display;
use biome_diagnostics::console::{markup, MarkupBuf};
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{Advices, Diagnostic, Location, LogCategory, MessageAndDescription, Visit};
use biome_rowan::{SyntaxKind, TextLen, TextRange};
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
#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(category = "parse", severity = Error)]
pub struct ParseDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    pub message: MessageAndDescription,
    #[advice]
    advice: ParserAdvice,
}

/// Possible details related to the diagnostic
#[derive(Clone, Debug, Default)]
struct ParserAdvice {
    advice_list: Vec<ParserAdviceKind>,
}

/// The structure of the advice. A message that gives details, a possible range so
/// the diagnostic is able to highlight the part of the code we want to explain.
#[derive(Clone, Debug)]
struct ParserAdviceDetail {
    /// A message that should explain this detail
    message: MarkupBuf,
    /// An optional range that should highlight the details of the code
    span: Option<TextRange>,
}

#[derive(Clone, Debug)]
enum ParserAdviceKind {
    /// A list a possible details that can be attached to the diagnostic.
    /// Useful to explain the nature errors.
    Detail(ParserAdviceDetail),
    /// A message for the user that should tell the user how to fix the issue
    Hint(MarkupBuf),
    List(MarkupBuf, Vec<MarkupBuf>),
}

impl ParserAdvice {
    fn add_detail(&mut self, message: impl Display, range: impl AsSpan) {
        self.advice_list
            .push(ParserAdviceKind::Detail(ParserAdviceDetail {
                message: markup! { {message} }.to_owned(),
                span: range.as_span(),
            }));
    }

    fn add_hint(&mut self, message: impl Display) {
        self.advice_list
            .push(ParserAdviceKind::Hint(markup! { { message } }.to_owned()));
    }

    fn add_hint_with_alternatives(&mut self, message: impl Display, alternatives: &[impl Display]) {
        self.advice_list.push(ParserAdviceKind::List(
            markup! {{message}}.to_owned(),
            alternatives
                .iter()
                .map(|msg| markup! {{msg}}.to_owned())
                .collect(),
        ))
    }
}

impl Advices for ParserAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for advice_kind in &self.advice_list {
            match advice_kind {
                ParserAdviceKind::Detail(detail) => {
                    let ParserAdviceDetail { span, message } = detail;
                    visitor.record_log(LogCategory::Info, message)?;

                    let location = Location::builder().span(span).build();
                    visitor.record_frame(location)?;
                }
                ParserAdviceKind::Hint(hint) => {
                    visitor.record_log(LogCategory::Info, hint)?;
                }
                ParserAdviceKind::List(message, list) => {
                    visitor.record_log(LogCategory::Info, message)?;

                    let list: Vec<_> = list
                        .iter()
                        .map(|suggestion| suggestion as &dyn Display)
                        .collect();
                    visitor.record_list(&list)?;
                }
            }
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

    pub fn new_single_node(name: &str, range: TextRange, p: &impl Parser) -> Self {
        let names = format!("{} {}", article_for(name), name);
        let msg = if p.source().text().text_len() <= range.start() {
            format!("Expected {names} but instead found the end of the file.")
        } else {
            format!("Expected {} but instead found '{}'.", names, p.text(range))
        };
        Self {
            span: range.as_span(),
            message: MessageAndDescription::from(msg),
            advice: ParserAdvice::default(),
        }
        .with_detail(range, format!("Expected {names} here."))
    }

    pub fn new_with_any(names: &[&str], range: TextRange, p: &impl Parser) -> Self {
        debug_assert!(names.len() > 1, "Requires at least 2 names");

        if names.len() < 2 {
            return Self::new_single_node(names.first().unwrap_or(&"<missing>"), range, p);
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

        let msg = if p.source().text().text_len() <= range.start() {
            format!("Expected {joined_names} but instead found the end of the file.")
        } else {
            format!(
                "Expected {} but instead found '{}'.",
                joined_names,
                p.text(range)
            )
        };

        Self {
            span: range.as_span(),
            message: MessageAndDescription::from(msg),
            advice: ParserAdvice::default(),
        }
        .with_detail(range, format!("Expected {joined_names} here."))
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
    /// # use biome_console::fmt::{Termcolor};
    /// # use biome_console::markup;
    /// # use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, console::fmt::Formatter};
    /// # use biome_parser::diagnostic::ParseDiagnostic;
    /// # use biome_rowan::{TextSize, TextRange};
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new("this is wrong!", range)
    ///     .with_detail(TextRange::new(TextSize::from(6), TextSize::from(7)), "This is reason why it's broken");
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
    pub fn with_detail(mut self, range: impl AsSpan, message: impl Display) -> Self {
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
    /// # use biome_console::fmt::{Termcolor};
    /// # use biome_console::markup;
    /// # use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, console::fmt::Formatter};
    /// # use biome_parser::diagnostic::ParseDiagnostic;
    /// # use biome_rowan::{TextSize, TextRange};
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new("this is wrong!", range)
    ///     .with_hint("You should delete the code");
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
    pub fn with_hint(mut self, message: impl Display) -> Self {
        self.advice.add_hint(message);
        self
    }

    /// A message that also allows to list of alternatives in case a fixed range of values/characters are expected.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use biome_console::fmt::{Termcolor};
    /// # use biome_console::markup;
    /// # use biome_diagnostics::{DiagnosticExt, PrintDiagnostic, console::fmt::Formatter};
    /// # use biome_parser::diagnostic::ParseDiagnostic;
    /// # use biome_rowan::{TextSize, TextRange};
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new("this is wrong!", range)
    ///     .with_alternatives("Expected one of the following values:", &["foo", "bar"]);
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
    /// assert!(result.contains("i Expected one of the following values:"));
    /// assert!(result.contains("- foo"));
    /// assert!(result.contains("- bar"));
    /// ```
    ///
    pub fn with_alternatives(
        mut self,
        message: impl Display,
        alternatives: &[impl Display],
    ) -> Self {
        self.advice
            .add_hint_with_alternatives(message, alternatives);
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
            .with_detail(p.cur_range(), "the file ends here")
        } else {
            p.err_builder(
                format!("expected `{}` but instead found `{}`", self.0, p.cur_text()),
                p.cur_range(),
            )
            .with_hint(format!("Remove {}", p.cur_text()))
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
            .with_detail(p.cur_range(), "the file ends here")
        } else {
            p.err_builder(
                format!("expected {} but instead found `{}`", self.0, p.cur_text()),
                p.cur_range(),
            )
            .with_hint(format!("Remove {}", p.cur_text()))
        }
    }
}

/// Creates a diagnostic saying that the node `name` was expected at range
pub fn expected_node(name: &str, range: TextRange, p: &impl Parser) -> ParseDiagnostic {
    ParseDiagnostic::new_single_node(name, range, p)
}

/// Creates a diagnostic saying that any of the nodes in `names` was expected at range
pub fn expected_any(names: &[&str], range: TextRange, p: &impl Parser) -> ParseDiagnostic {
    ParseDiagnostic::new_with_any(names, range, p)
}

/// Creates a diagnostic with message "Unexpected value." and then it lists the values that should be expected.
pub fn expect_one_of(names: &[&str], range: TextRange) -> ParseDiagnostic {
    ParseDiagnostic::new("Unexpected value or character.", range)
        .with_alternatives("Expected one of:", names)
}

fn article_for(name: &str) -> &'static str {
    match name.bytes().next() {
        Some(b'a' | b'e' | b'i' | b'o' | b'u') => "an",
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
