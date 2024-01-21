use biome_console::fmt::Display;
use biome_console::{markup, MarkupBuf};
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{
    Advices, Diagnostic, DiagnosticTags, LogCategory, MessageAndDescription, Severity, Visit,
};
use biome_rowan::{SyntaxError, TextRange};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct VisitableType: u8 {
        const NULL = 1 << 0;
        const BOOL = 1 << 1;
        const NUMBER = 1 << 2;
        const STR = 1 << 3;
        const ARRAY = 1 << 4;
        const MAP = 1 << 5;
    }
}

impl std::fmt::Display for VisitableType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return write!(fmt, "no value");
        }
        for (i, expected_type) in self.iter().enumerate() {
            if i != 0 {
                write!(fmt, ", or ")?;
            }
            let expected_type = match expected_type {
                VisitableType::NULL => "null",
                VisitableType::BOOL => "a boolean",
                VisitableType::NUMBER => "a number",
                VisitableType::STR => "a string",
                VisitableType::ARRAY => "an array",
                VisitableType::MAP => "an object",
                _ => unreachable!("Unhandled deserialization type."),
            };
            write!(fmt, "{}", expected_type)?;
        }
        Ok(())
    }
}

/// Diagnostic emitted during the deserialization
#[derive(Debug, Serialize, Clone, Deserialize, Diagnostic)]
#[diagnostic(category = "deserialize")]
pub struct DeserializationDiagnostic {
    #[message]
    #[description]
    reason: MessageAndDescription,
    #[location(span)]
    range: Option<TextRange>,
    #[advice]
    deserialization_advice: DeserializationAdvice,
    #[severity]
    severity: Severity,
    #[tags]
    tags: DiagnosticTags,
}

impl DeserializationDiagnostic {
    pub fn new(reason: impl Display) -> Self {
        Self {
            reason: markup! {{reason}}.to_owned().into(),
            range: None,
            deserialization_advice: DeserializationAdvice::default(),
            severity: Severity::Error,
            tags: DiagnosticTags::empty(),
        }
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_incorrect_type(
        actual_type: VisitableType,
        expected_type: VisitableType,
        range: impl AsSpan,
    ) -> Self {
        Self::new(markup! {
            "Incorrect type, expected "<Emphasis>{format_args!("{}", expected_type)}</Emphasis>", but received "<Emphasis>{format_args!("{}", actual_type)}</Emphasis>"."
        })
        .with_range(range)
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_incorrect_type_with_name(
        actual_type: VisitableType,
        expected_type: VisitableType,
        name: &str,
        range: impl AsSpan,
    ) -> Self {
        if name.is_empty() {
            return Self::new_incorrect_type(actual_type, expected_type, range);
        }
        Self::new(markup! {
            <Emphasis>{name}</Emphasis>" has an incorrect type, expected "<Emphasis>{format_args!("{}", expected_type)}</Emphasis>", but received "<Emphasis>{format_args!("{}", actual_type)}</Emphasis>"."
        })
        .with_range(range)
    }

    /// Emitted when a key is missing, against a set of required ones
    pub fn new_missing_key(key_name: &str, range: impl AsSpan, required_keys: &[&str]) -> Self {
        let diagnostic =
            Self::new(markup!("The key `"<Emphasis>{key_name}</Emphasis>"` is missing." ))
                .with_range(range);

        if required_keys.len() > 1 {
            diagnostic.note_with_list("Required keys", required_keys)
        } else {
            diagnostic
        }
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_out_of_bound_integer(
        min: impl std::fmt::Display,
        max: impl std::fmt::Display,
        range: impl AsSpan,
    ) -> Self {
        Self::new(markup! {
            "The number should be an integer between "<Emphasis>{format_args!("{}", min)}</Emphasis>" and "<Emphasis>{format_args!("{}", max)}</Emphasis>"."
        })
        .with_range(range)
    }

    /// Emitted when there's an unknown key, against a set of known ones
    pub fn new_unknown_key(key_name: &str, range: impl AsSpan, allowed_keys: &[&str]) -> Self {
        Self::new(markup!("Found an unknown key `"<Emphasis>{key_name}</Emphasis>"`." ))
            .with_range(range)
            .note_with_list("Accepted keys", allowed_keys)
    }

    /// Emitted when there's an unknown value, against a set of known ones
    pub fn new_unknown_value(
        variant_name: &str,
        range: impl AsSpan,
        allowed_variants: &[&str],
    ) -> Self {
        Self::new(markup! {"Found an unknown value `"<Emphasis>{variant_name}</Emphasis>"`."})
            .with_range(range)
            .note_with_list("Accepted values:", allowed_variants)
    }

    /// Emitted when there's a deprecated property and you can suggest an alternative solution
    pub fn new_deprecated_use_instead(key_name: &str, range: impl AsSpan, instead: &str) -> Self {
        Self::new(
            markup! { "The property "<Emphasis>{key_name}</Emphasis>" is deprecated. Use "<Emphasis>{{instead}}</Emphasis>" instead." },
        )
        .with_range(range)
        .with_tags(DiagnosticTags::DEPRECATED_CODE).with_custom_severity(Severity::Warning)
    }

    /// Emitted when there's a deprecated property
    pub fn new_deprecated(key_name: &str, range: impl AsSpan) -> Self {
        Self::new(markup! { "The property "<Emphasis>{key_name}</Emphasis>" is deprecated." })
            .with_range(range)
            .with_tags(DiagnosticTags::DEPRECATED_CODE)
            .with_custom_severity(Severity::Warning)
    }

    /// Adds a range to the diagnostic
    pub fn with_range(mut self, span: impl AsSpan) -> Self {
        self.range = span.as_span();
        self
    }

    /// Adds a note to the diagnostic
    pub fn with_note(mut self, message: impl Display) -> Self {
        self.deserialization_advice
            .notes
            .push((markup! {{message}}.to_owned(), vec![]));
        self
    }

    /// Changes the severity of the diagnostic
    pub fn with_custom_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Add a tag to the list of tags
    pub fn with_tags(mut self, tag: DiagnosticTags) -> Self {
        self.tags |= tag;
        self
    }

    /// Adds a note with a list of strings
    pub fn note_with_list(mut self, message: impl Display, list: &[impl Display]) -> Self {
        self.deserialization_advice.notes.push((
            markup! {{message}}.to_owned(),
            list.iter()
                .map(|message| markup! {{message}}.to_owned())
                .collect::<Vec<_>>(),
        ));
        self
    }
}

impl From<SyntaxError> for DeserializationDiagnostic {
    fn from(_: SyntaxError) -> Self {
        DeserializationDiagnostic::new("Syntax error")
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DeserializationAdvice {
    notes: Vec<(MarkupBuf, Vec<MarkupBuf>)>,
}

impl DeserializationAdvice {
    pub fn note(mut self, message: impl Display) -> Self {
        self.notes
            .push((markup! {{message}}.to_owned(), Vec::new()));
        self
    }
}

impl Advices for DeserializationAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for (message, known_keys) in &self.notes {
            visitor.record_log(LogCategory::Info, message)?;
            if !known_keys.is_empty() {
                let list: Vec<_> = known_keys
                    .iter()
                    .map(|message| message as &dyn Display)
                    .collect();
                visitor.record_list(&list)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visitable_type_fmt() {
        assert_eq!(VisitableType::empty().to_string(), "no value");
        assert_eq!(VisitableType::NULL.to_string(), "null");
        assert_eq!(
            VisitableType::NULL.union(VisitableType::BOOL).to_string(),
            "null, or a boolean"
        );
        assert_eq!(
            VisitableType::all().to_string(),
            "null, or a boolean, or a number, or a string, or an array, or an object"
        );
    }
}
