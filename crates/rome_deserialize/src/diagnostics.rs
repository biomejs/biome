use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{Advices, Diagnostic, LogCategory, MessageAndDescription, Severity, Visit};
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_rowan::{SyntaxError, TextRange};
use serde::{Deserialize, Serialize};

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
}

impl DeserializationDiagnostic {
    pub fn new(reason: impl Display) -> Self {
        Self {
            reason: markup! {{reason}}.to_owned().into(),
            range: None,
            deserialization_advice: DeserializationAdvice::default(),
            severity: Severity::Error,
        }
    }

    /// Emitted when the type of a value is incorrect.
    pub fn new_incorrect_type_for_value(
        key_name: impl Display,
        expected_type: impl Display,
        range: impl AsSpan,
    ) -> Self {
        Self::new(markup! {
                "The value of key "<Emphasis>{{key_name}}</Emphasis>" is incorrect. Expected a "<Emphasis>{{expected_type}}"."</Emphasis>
            }).with_range(range)
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_incorrect_type(expected_type: impl Display, range: impl AsSpan) -> Self {
        Self::new(markup! {
            "Incorrect type, expected a "<Emphasis>{{expected_type}}"."</Emphasis>
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
