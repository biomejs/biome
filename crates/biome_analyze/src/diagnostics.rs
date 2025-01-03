use biome_console::{markup, MarkupBuf};
use biome_diagnostics::{
    advice::CodeSuggestionAdvice, category, Advices, Category, Diagnostic, DiagnosticExt,
    DiagnosticTags, Error, Location, LogCategory, MessageAndDescription, Severity, Visit,
};
use biome_rowan::TextRange;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use crate::rule::RuleDiagnostic;

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
            kind: DiagnosticKind::Rule(rule_diagnostic),
            code_suggestion_list: vec![],
        }
    }
}

#[derive(Debug)]
enum DiagnosticKind {
    /// It holds various info related to diagnostics emitted by the rules
    Rule(RuleDiagnostic),
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
            DiagnosticKind::Rule(rule_diagnostic) => rule_diagnostic.advices().record(visitor)?,
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
}

impl Diagnostic for RuleError {}

impl std::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuleError::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "the rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            RuleError::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "a code action replaced the root of the file with a non-root node."
                )
            }
        }
    }
}

impl biome_console::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            RuleError::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "the rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            RuleError::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "a code action replaced the root of the file with a non-root node."
                )
            }
        }
    }
}

impl std::error::Error for RuleError {}
