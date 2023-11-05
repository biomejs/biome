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
    pub struct ExpectedType: u8 {
        const NULL = 1 << 0;
        const BOOL = 1 << 1;
        const NUMBER = 1 << 2;
        const STR = 1 << 3;
        const ARRAY = 1 << 4;
        const MAP = 1 << 5;
    }
}

impl Display for ExpectedType {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        if self.is_empty() {
            return write!(fmt, "no value");
        }
        let mut is_not_first = false;
        for expected_type in self.iter() {
            if is_not_first {
                write!(fmt, " or ")?;
            }
            let expected_type = match expected_type {
                ExpectedType::NULL => "null",
                ExpectedType::BOOL => "a boolean",
                ExpectedType::NUMBER => "a number",
                ExpectedType::STR => "a string",
                ExpectedType::ARRAY => "an array",
                ExpectedType::MAP => "an object",
                _ => unreachable!("Unhandled deserialization type."),
            };
            write!(fmt, "{}", expected_type)?;
            is_not_first = true;
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
    pub fn new_incorrect_type(expected_type: ExpectedType, range: impl AsSpan) -> Self {
        Self::new(markup! {
            "Incorrect type, expected "<Emphasis>{expected_type}"."</Emphasis>
        })
        .with_range(range)
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_out_of_bound_integer(
        min: impl Display,
        max: impl Display,
        range: impl AsSpan,
    ) -> Self {
        Self::new(markup! {
            "The number should be an integer between "<Emphasis>{min}</Emphasis>" and "<Emphasis>{max}</Emphasis>"."
        })
        .with_range(range)
    }

    /// Emitted when there's an unknown key, against a set of known ones
    pub fn new_unknown_key(
        key_name: impl Display,
        range: impl AsSpan,
        known_members: &[&str],
    ) -> Self {
        Self::new(markup!("Found an unknown key `"<Emphasis>{{ key_name }}</Emphasis>"`." ))
            .with_range(range)
            .note_with_list("Accepted keys", known_members)
    }

    /// Emitted when there's an unknown value, against a set of known ones
    pub fn new_unknown_value(
        variant_name: impl Display,
        range: impl AsSpan,
        known_variants: &[&str],
    ) -> Self {
        Self::new(markup! {"Found an unknown value `"<Emphasis>{{ variant_name }}</Emphasis>"`."})
            .with_range(range)
            .note_with_list("Accepted values:", known_variants)
    }

    /// Emitted when there's a deprecated property
    pub fn new_deprecated(key_name: impl Display, range: impl AsSpan, instead: &str) -> Self {
        Self::new(
            markup! { "The property "<Emphasis>{{key_name}}</Emphasis>" is deprecated. Use "<Emphasis>{{instead}}</Emphasis>" instead." },
        )
        .with_range(range)
        .with_tags(DiagnosticTags::DEPRECATED_CODE).with_custom_severity(Severity::Warning)
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
