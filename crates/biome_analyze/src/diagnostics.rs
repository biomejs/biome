use crate::rule::RuleDiagnostic;
use biome_console::{MarkupBuf, markup};
use biome_diagnostics::{
    Advices, Category, Diagnostic, DiagnosticExt, DiagnosticTags, Error, Location, LogCategory,
    MessageAndDescription, Severity, Visit, advice::CodeSuggestionAdvice, category,
};
use biome_rowan::{TextRange, TextSize};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

/// Small wrapper for diagnostics during the analysis phase.
///
/// During these phases, analyzers can create various type diagnostics and some of them
/// don't have all the info to actually create a real [Diagnostic].
///
/// This wrapper serves as glue, which eventually is able to spit out full fledged diagnostics.
///
#[derive(Debug)]
pub struct AnalyzerDiagnostic {
    kind: DiagnosticKind,
    /// Series of code suggestions offered by rule code actions
    code_suggestion_list: Vec<CodeSuggestionAdvice<MarkupBuf>>,
}

impl From<RuleDiagnostic> for AnalyzerDiagnostic {
    fn from(rule_diagnostic: RuleDiagnostic) -> Self {
        Self {
            kind: DiagnosticKind::Rule(Box::new(rule_diagnostic)),
            code_suggestion_list: vec![],
        }
    }
}

#[derive(Debug)]
enum DiagnosticKind {
    /// It holds various info related to diagnostics emitted by the rules
    Rule(Box<RuleDiagnostic>),
    /// We have raw information to create a basic [Diagnostic]
    Raw(Error),
}

impl Diagnostic for AnalyzerDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => Some(rule_diagnostic.category),
            DiagnosticKind::Raw(error) => error.category(),
        }
    }
    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => Debug::fmt(&rule_diagnostic.message, fmt),
            DiagnosticKind::Raw(error) => error.description(fmt),
        }
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => {
                biome_console::fmt::Display::fmt(&rule_diagnostic.message, fmt)
            }
            DiagnosticKind::Raw(error) => error.message(fmt),
        }
    }

    fn severity(&self) -> Severity {
        match &self.kind {
            DiagnosticKind::Rule(diagnostic) => diagnostic.severity(),
            DiagnosticKind::Raw(error) => error.severity(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => rule_diagnostic.tags,
            DiagnosticKind::Raw(error) => error.tags(),
        }
    }

    fn location(&self) -> Location<'_> {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => {
                Location::builder().span(&rule_diagnostic.span).build()
            }
            DiagnosticKind::Raw(error) => error.location(),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => rule_diagnostic.record(visitor)?,
            DiagnosticKind::Raw(error) => error.advices(visitor)?,
        }

        // finally, we print possible code suggestions on how to fix the issue
        for suggestion in &self.code_suggestion_list {
            suggestion.record(visitor)?;
        }

        Ok(())
    }
}

impl AnalyzerDiagnostic {
    /// Creates a diagnostic from a generic [Error]
    pub fn from_error(error: Error) -> Self {
        Self {
            kind: DiagnosticKind::Raw(error),
            code_suggestion_list: vec![],
        }
    }

    pub fn get_span(&self) -> Option<TextRange> {
        match &self.kind {
            DiagnosticKind::Rule(rule_diagnostic) => rule_diagnostic.span,
            DiagnosticKind::Raw(error) => error.location().span,
        }
    }

    /// It adds a code suggestion, use this API to tell the user that a rule can benefit from
    /// a automatic code fix.
    pub fn add_code_suggestion(mut self, suggestion: CodeSuggestionAdvice<MarkupBuf>) -> Self {
        self.kind = match self.kind {
            DiagnosticKind::Rule(mut rule_diagnostic) => {
                rule_diagnostic.tags = DiagnosticTags::FIXABLE;
                DiagnosticKind::Rule(rule_diagnostic)
            }
            DiagnosticKind::Raw(error) => {
                DiagnosticKind::Raw(error.with_tags(DiagnosticTags::FIXABLE))
            }
        };

        self.code_suggestion_list.push(suggestion);
        self
    }

    /// The location of the diagnostic is shifted using this offset.
    /// This is only applied when the [Self::kind] is [DiagnosticKind::Rule]
    pub fn add_diagnostic_offset(&mut self, offset: TextSize) {
        if let DiagnosticKind::Rule(rule_diagnostic) = &mut self.kind {
            let diagnostic = rule_diagnostic.as_mut();
            if let Some(span) = &diagnostic.span {
                diagnostic.span = Some(span.add(offset));
            }
            diagnostic.set_advice_offset(offset);
        }
    }

    pub const fn is_raw(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Raw(_))
    }
}

#[derive(Debug, Diagnostic, Clone)]
#[diagnostic(severity = Warning)]
pub struct AnalyzerSuppressionDiagnostic {
    #[category]
    category: &'static Category,
    #[location(span)]
    range: TextRange,
    #[message]
    #[description]
    message: MessageAndDescription,
    #[tags]
    tags: DiagnosticTags,

    #[advice]
    advice: SuppressionAdvice,
}

impl AnalyzerSuppressionDiagnostic {
    pub(crate) fn new(
        category: &'static Category,
        range: TextRange,
        message: impl biome_console::fmt::Display,
    ) -> Self {
        Self {
            category,
            range,
            message: MessageAndDescription::from(markup! { {message} }.to_owned()),
            tags: DiagnosticTags::empty(),
            advice: SuppressionAdvice::default(),
        }
    }

    pub(crate) fn note(mut self, message: MarkupBuf, range: impl Into<TextRange>) -> Self {
        self.advice.messages.push((message, Some(range.into())));
        self
    }

    pub(crate) fn hint(mut self, message: MarkupBuf) -> Self {
        self.advice.messages.push((message, None));
        self
    }

    pub(crate) fn new_unknown_lint_rule(group: &str, rule: &str, range: TextRange) -> Self {
        Self::new(
            category!("suppressions/unknownRule"),
            range,
            format_args!("Unknown lint rule {group}/{rule} in suppression comment"),
        )
    }

    pub(crate) fn new_unknown_lint_group(group: &str, range: TextRange) -> Self {
        Self::new(
            category!("suppressions/unknownGroup"),
            range,
            format_args!("Unknown lint group {group} in suppression comment"),
        )
    }

    pub(crate) fn new_unknown_assist_group(group: &str, range: TextRange) -> Self {
        Self::new(
            category!("suppressions/unknownGroup"),
            range,
            format_args!("Unknown assist group {group} in suppression comment"),
        )
    }

    pub(crate) fn new_unknown_assist_action(group: &str, action: &str, range: TextRange) -> Self {
        Self::new(
            category!("suppressions/unknownAction"),
            range,
            format_args!("Unknown assist action {group}/{action} in suppression comment"),
        )
    }
}

#[derive(Debug, Default, Clone)]
struct SuppressionAdvice {
    messages: Vec<(MarkupBuf, Option<TextRange>)>,
}

impl Advices for SuppressionAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for (message, range) in &self.messages {
            visitor.record_log(LogCategory::Info, &markup! {{message}})?;
            let location = Location::builder().span(range);

            visitor.record_frame(location.build())?
        }
        Ok(())
    }
}

/// Series of errors encountered when running rules on a file
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum RuleError {
    /// The rule with the specified name replaced the root of the file with a node that is not a valid root for that language.
    ReplacedRootWithNonRootError {
        rule_name: Option<(Cow<'static, str>, Cow<'static, str>)>,
    },
    /// The rules listed below caused an infinite loop when applying fixes to the file.
    ConflictingRuleFixesError {
        rules: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    },
}

impl Diagnostic for RuleError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/panic"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "An internal error occurred when analyzing this file.\n\n{}\n\nThis is likely a bug in Biome, not an error in your code. Please consider filing an issue on GitHub with a reproduction of this error.",
            self
        )?;
        Ok(())
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            "An internal error occurred when analyzing this file."
        })?;
        fmt.write_markup(markup! {
            {self}
        })?;
        fmt.write_markup(markup! {
            "This is likely a bug in Biome, not an error in your code. Please consider filing an issue on "<Hyperlink href="https://github.com/biomejs/biome/issues/new/choose">"GitHub"</Hyperlink>" with a reproduction of this error."
        })?;
        Ok(())
    }
}

impl std::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "The rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            Self::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "A code action replaced the root of the file with a non-root node."
                )
            }
            Self::ConflictingRuleFixesError { rules } => {
                if rules.is_empty() {
                    return std::write!(fmt, "conflicting rule fixes detected");
                }
                let rules_list = rules
                    .iter()
                    .map(|(group, rule)| format!("'{group}/{rule}'"))
                    .collect::<Vec<_>>()
                    .join(", ");
                std::write!(
                    fmt,
                    "The rules {rules_list} caused an infinite loop when applying fixes to the file."
                )
            }
        }
    }
}

impl biome_console::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            Self::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "The rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            Self::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "A code action replaced the root of the file with a non-root node."
                )
            }
            Self::ConflictingRuleFixesError { rules } => {
                if rules.is_empty() {
                    return std::write!(fmt, "Conflicting rule fixes detected.");
                }
                let rules_list = rules
                    .iter()
                    .map(|(group, rule)| format!("'{group}/{rule}'"))
                    .collect::<Vec<_>>()
                    .join(", ");
                std::write!(
                    fmt,
                    "The rules {rules_list} caused an infinite loop when applying fixes to the file."
                )
            }
        }
    }
}

impl std::error::Error for RuleError {}
