use crate::analyzer_plugin::{PluginCodeAction, PluginTextEdit};
use crate::categories::{
    SUPPRESSION_INLINE_ACTION_CATEGORY, SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY,
};
use crate::registry::LanguageRoot;
use crate::{
    AnalyzerDiagnostic, AnalyzerOptions, OtherActionCategory, Queryable, RuleCategory,
    RuleDiagnostic, RuleGroup, ServiceBag, SuppressionAction,
    categories::ActionCategory,
    context::RuleContext,
    registry::{RuleLanguage, RuleRoot},
    rule::Rule,
    suppression_action::{make_inline_suppression, make_top_level_suppression},
};
use biome_console::{MarkupBuf, markup};
use biome_diagnostics::{Applicability, CodeSuggestion, Error, advice::CodeSuggestionAdvice};
use biome_rowan::{AstNode, BatchMutation, Language, TextRange};
use biome_text_edit::TextEdit;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::sync::Arc;
use std::vec::IntoIter;

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
    fn actions(&self) -> AnalyzerActionIter<L>;
    fn transformations(&self) -> AnalyzerTransformationIter<L>;
}

/// Simple implementation of [AnalyzerSignal] generating a [AnalyzerDiagnostic]
/// from a provided factory function. Optionally, this signal can be configured
/// to also emit a code action, by calling `.with_action` with a secondary
/// factory function for said action.
pub struct DiagnosticSignal<D, A, L, T, Tr> {
    diagnostic: D,
    action: A,
    transformation: Tr,
    _diag: PhantomData<(L, T)>,
}

impl<L: Language, D, T>
    DiagnosticSignal<
        D,
        fn() -> Option<AnalyzerAction<L>>,
        L,
        T,
        fn() -> Option<AnalyzerTransformation<L>>,
    >
where
    D: Fn() -> T,
    Error: From<T>,
{
    pub fn new(factory: D) -> Self {
        Self {
            diagnostic: factory,
            action: || None,
            transformation: || None,
            _diag: PhantomData,
        }
    }
}

impl<L: Language, D, A, T, Tr> DiagnosticSignal<D, A, L, T, Tr> {
    pub fn with_action<B>(self, factory: B) -> DiagnosticSignal<D, B, L, T, Tr>
    where
        B: Fn() -> Option<AnalyzerAction<L>>,
    {
        DiagnosticSignal {
            diagnostic: self.diagnostic,
            action: factory,
            transformation: self.transformation,
            _diag: PhantomData,
        }
    }
}

impl<L: Language, D, A, T, Tr> AnalyzerSignal<L> for DiagnosticSignal<D, A, L, T, Tr>
where
    D: Fn() -> T,
    Error: From<T>,
    A: Fn() -> Option<AnalyzerAction<L>>,
    Tr: Fn() -> Option<AnalyzerTransformation<L>>,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let diag = (self.diagnostic)();
        let error = Error::from(diag);
        Some(AnalyzerDiagnostic::from_error(error))
    }

    fn actions(&self) -> AnalyzerActionIter<L> {
        if let Some(action) = (self.action)() {
            AnalyzerActionIter::new([action])
        } else {
            AnalyzerActionIter::new(vec![])
        }
    }

    fn transformations(&self) -> AnalyzerTransformationIter<L> {
        if let Some(transformation) = (self.transformation)() {
            AnalyzerTransformationIter::new([transformation])
        } else {
            AnalyzerTransformationIter::new(vec![])
        }
    }
}

/// Implementation of [AnalyzerSignal] for plugin diagnostics that preserves
/// the [RuleDiagnostic] as [DiagnosticKind::Rule](crate::diagnostics::DiagnosticKind::Rule),
/// ensuring diagnostic offset adjustments are correctly applied for embedded
/// languages (Vue, Svelte, Astro).
///
/// Unlike [DiagnosticSignal] which converts through [Error] into
/// [DiagnosticKind::Raw](crate::diagnostics::DiagnosticKind::Raw), this type
/// directly converts via `AnalyzerDiagnostic::from(RuleDiagnostic)`.
pub struct PluginSignal<'phase, L: Language> {
    diagnostic: RuleDiagnostic,
    actions: Vec<PluginCodeAction>,
    /// Full source text of the file being analyzed — needed to construct
    /// `TextEdit` diffs for plugin code actions. Wrapped in `Arc` to avoid
    /// cloning the full source string for each diagnostic signal.
    source_text: Arc<str>,
    root: &'phase LanguageRoot<L>,
    suppression_action: &'phase dyn SuppressionAction<Language = L>,
    options: &'phase AnalyzerOptions,
    category: RuleCategory,
    deprecated: Option<Arc<str>>,
    issue_number: Option<Arc<str>>,
}

impl<'phase, L: Language> PluginSignal<'phase, L> {
    pub fn new(
        diagnostic: RuleDiagnostic,
        actions: Vec<PluginCodeAction>,
        source_text: Arc<str>,
        root: &'phase LanguageRoot<L>,
        suppression_action: &'phase dyn SuppressionAction<Language = L>,
        options: &'phase AnalyzerOptions,
    ) -> Self {
        Self {
            diagnostic,
            actions,
            source_text,
            root,
            suppression_action,
            options,
            category: RuleCategory::Lint,
            deprecated: None,
            issue_number: None,
        }
    }

    pub fn with_category(mut self, category: RuleCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_deprecated(mut self, deprecated: Option<Arc<str>>) -> Self {
        self.deprecated = deprecated;
        self
    }

    pub fn with_issue_number(mut self, issue_number: Option<Arc<str>>) -> Self {
        self.issue_number = issue_number;
        self
    }
}

/// Apply plugin text edits to a source string, returning the (spanning range, TextEdit diff).
///
/// The edits are sorted by start position and applied from last to first so that
/// earlier offsets remain valid. The spanning range covers the union of all edits.
fn apply_plugin_edits(source: &str, edits: &[PluginTextEdit]) -> Option<(TextRange, TextEdit)> {
    if edits.is_empty() {
        return None;
    }

    // Sort by start offset (ascending).
    let mut sorted: Vec<&PluginTextEdit> = edits.iter().collect();
    sorted.sort_by_key(|e| e.range.start());

    // Compute the spanning range over all edits.
    let span_start = sorted.first()?.range.start();
    let span_end = sorted.iter().map(|e| e.range.end()).max()?;
    let span = TextRange::new(span_start, span_end);

    let old_slice = source.get(usize::from(span_start)..usize::from(span_end))?;

    // Build the new text by applying edits within the span.
    let mut new_text = String::new();
    let mut cursor = span_start;
    for edit in &sorted {
        // Copy text before this edit.
        if edit.range.start() > cursor {
            let before = source.get(usize::from(cursor)..usize::from(edit.range.start()))?;
            new_text.push_str(before);
        }
        new_text.push_str(&edit.replacement);
        cursor = edit.range.end();
    }
    // Copy remaining text after the last edit within the span.
    if cursor < span_end {
        let after = source.get(usize::from(cursor)..usize::from(span_end))?;
        new_text.push_str(after);
    }

    let text_edit = TextEdit::from_unicode_words(old_slice, &new_text);
    Some((span, text_edit))
}

impl<L: Language> AnalyzerSignal<L> for PluginSignal<'_, L> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let mut rule_diag = self.diagnostic.clone();

        if let Some(issue_number) = &self.issue_number {
            let url = format!("https://github.com/biomejs/biome/issues/{issue_number}");
            rule_diag = rule_diag.note(markup! {
                "This rule is still under development. Visit "<Hyperlink href={url.as_str()}>{url.as_str()}</Hyperlink>" for details."
            });
        }
        if let Some(reason) = &self.deprecated {
            let reason: &str = reason;
            rule_diag = rule_diag.note(markup! { "Deprecated: " {reason} });
        }

        let mut diag = AnalyzerDiagnostic::from(rule_diag);

        for action in &self.actions {
            if let Some((_span, text_edit)) = apply_plugin_edits(&self.source_text, &action.edits) {
                let suggestion = CodeSuggestionAdvice {
                    applicability: action.applicability,
                    msg: markup!({ action.message }).to_owned(),
                    suggestion: text_edit,
                };
                diag = diag.add_code_suggestion(suggestion);
            }
        }

        Some(diag)
    }

    fn actions(&self) -> AnalyzerActionIter<L> {
        let mut actions = Vec::new();

        let plugin_name = self
            .diagnostic
            .subcategory
            .as_deref()
            .unwrap_or("anonymous");
        let suppression_prefix = self.category.as_suppression_category();
        let rule_category = format!("{suppression_prefix}/plugin/{plugin_name}");
        let kind_label = if self.category == RuleCategory::Action {
            "action"
        } else {
            "rule"
        };
        let suppression_reason = self
            .options
            .suppression_reason
            .as_deref()
            .unwrap_or("<explanation>");

        // Inline suppression
        if let Some(text_range) = self.diagnostic.span() {
            let suppress = make_inline_suppression(
                &rule_category,
                kind_label,
                self.root,
                &text_range,
                self.suppression_action,
                suppression_reason,
            );
            actions.push(AnalyzerAction {
                rule_name: None,
                category: ActionCategory::Other(OtherActionCategory::InlineSuppression),
                applicability: Applicability::Always,
                mutation: suppress.mutation,
                message: suppress.message,
                text_edit: None,
            });
        }

        // Top-level suppression
        if let Some(suppress) = make_top_level_suppression(
            &rule_category,
            kind_label,
            self.root,
            self.suppression_action,
        ) {
            actions.push(AnalyzerAction {
                rule_name: None,
                category: ActionCategory::Other(OtherActionCategory::ToplevelSuppression),
                applicability: Applicability::Always,
                mutation: suppress.mutation,
                message: suppress.message,
                text_edit: None,
            });
        }

        // Plugin fix actions (quick-fixes from code actions)
        let fix_kind_override = self
            .options
            .plugin_rule_override(plugin_name)
            .and_then(|o| o.fix_kind);

        if fix_kind_override != Some(crate::FixKind::None) && !self.actions.is_empty() {
            let noop_mutation = BatchMutation::new(self.root.syntax().clone());
            for plugin_action in &self.actions {
                if let Some((span, text_edit)) =
                    apply_plugin_edits(&self.source_text, &plugin_action.edits)
                {
                    let applicability = match fix_kind_override {
                        Some(crate::FixKind::Safe) => Applicability::Always,
                        Some(crate::FixKind::Unsafe) => Applicability::MaybeIncorrect,
                        _ => plugin_action.applicability,
                    };
                    actions.push(AnalyzerAction {
                        rule_name: None,
                        category: ActionCategory::QuickFix(std::borrow::Cow::Borrowed(
                            "quickfix.plugin",
                        )),
                        applicability,
                        message: markup!({ plugin_action.message }).to_owned(),
                        mutation: noop_mutation.clone(),
                        text_edit: Some((span, text_edit)),
                    });
                }
            }
        }

        AnalyzerActionIter::new(actions)
    }

    fn transformations(&self) -> AnalyzerTransformationIter<L> {
        AnalyzerTransformationIter::new(vec![])
    }
}

/// Code Action object returned by the analyzer, generated from a [crate::RuleAction]
/// with additional information about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug, Clone)]
pub struct AnalyzerAction<L: Language> {
    pub rule_name: Option<(&'static str, &'static str)>,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
    /// Plugin text edit — when `Some`, `process_action` applies this instead of
    /// `mutation`. The tuple contains the spanning range and the text edit diff.
    pub text_edit: Option<(TextRange, TextEdit)>,
}

impl<L: Language> AnalyzerAction<L> {
    pub fn is_suppression(&self) -> bool {
        self.is_inline_suppression() || self.is_top_level_suppression()
    }

    pub fn is_inline_suppression(&self) -> bool {
        self.category.matches(SUPPRESSION_INLINE_ACTION_CATEGORY)
    }

    pub fn is_top_level_suppression(&self) -> bool {
        self.category.matches(SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY)
    }
}

pub struct AnalyzerActionIter<L: Language> {
    analyzer_actions: IntoIter<AnalyzerAction<L>>,
}

impl<L: Language> Default for AnalyzerActionIter<L> {
    fn default() -> Self {
        Self {
            analyzer_actions: vec![].into_iter(),
        }
    }
}

impl<L: Language> From<AnalyzerAction<L>> for CodeSuggestionAdvice<MarkupBuf> {
    fn from(action: AnalyzerAction<L>) -> Self {
        let suggestion = if let Some((_range, text_edit)) = action.text_edit {
            text_edit
        } else {
            let (_, suggestion) = action.mutation.to_text_range_and_edit().unwrap_or_default();
            suggestion
        };
        Self {
            applicability: action.applicability,
            msg: action.message,
            suggestion,
        }
    }
}

impl<L: Language> From<AnalyzerAction<L>> for CodeSuggestionItem {
    fn from(action: AnalyzerAction<L>) -> Self {
        let (range, suggestion) = if let Some((range, text_edit)) = action.text_edit {
            (range, text_edit)
        } else {
            action.mutation.to_text_range_and_edit().unwrap_or_default()
        };

        Self {
            rule_name: action.rule_name,
            category: action.category,
            suggestion: CodeSuggestion {
                span: range,
                applicability: action.applicability,
                msg: action.message,
                suggestion,
                labels: vec![],
            },
        }
    }
}

impl<L: Language> AnalyzerActionIter<L> {
    pub fn new<I>(actions: I) -> Self
    where
        I: IntoIterator<Item = AnalyzerAction<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        Self {
            analyzer_actions: actions
                .into_iter()
                .collect::<Vec<AnalyzerAction<L>>>()
                .into_iter(),
        }
    }
}

impl<L: Language> Iterator for AnalyzerActionIter<L> {
    type Item = AnalyzerAction<L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.analyzer_actions.next()
    }
}

impl<L: Language> FusedIterator for AnalyzerActionIter<L> {}

impl<L: Language> ExactSizeIterator for AnalyzerActionIter<L> {
    fn len(&self) -> usize {
        self.analyzer_actions.len()
    }
}

pub struct CodeSuggestionAdviceIter<L: Language> {
    iter: IntoIter<AnalyzerAction<L>>,
}

impl<L: Language> Iterator for CodeSuggestionAdviceIter<L> {
    type Item = CodeSuggestionAdvice<MarkupBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.iter.next()?;
        Some(action.into())
    }
}

impl<L: Language> FusedIterator for CodeSuggestionAdviceIter<L> {}

impl<L: Language> ExactSizeIterator for CodeSuggestionAdviceIter<L> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct CodeActionIter<L: Language> {
    iter: IntoIter<AnalyzerAction<L>>,
}

pub struct CodeSuggestionItem {
    pub category: ActionCategory,
    pub suggestion: CodeSuggestion,
    pub rule_name: Option<(&'static str, &'static str)>,
}

impl<L: Language> Iterator for CodeActionIter<L> {
    type Item = CodeSuggestionItem;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.iter.next()?;
        Some(action.into())
    }
}

impl<L: Language> FusedIterator for CodeActionIter<L> {}

impl<L: Language> ExactSizeIterator for CodeActionIter<L> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<L: Language> AnalyzerActionIter<L> {
    /// Returns an iterator that yields [CodeSuggestionAdvice]
    pub fn into_code_suggestion_advices(self) -> CodeSuggestionAdviceIter<L> {
        CodeSuggestionAdviceIter {
            iter: self.analyzer_actions,
        }
    }

    /// Returns an iterator that yields [CodeAction]
    pub fn into_code_action_iter(self) -> CodeActionIter<L> {
        CodeActionIter {
            iter: self.analyzer_actions,
        }
    }
}

pub struct AnalyzerTransformationIter<L: Language> {
    analyzer_transformations: IntoIter<AnalyzerTransformation<L>>,
}

impl<L: Language> Default for AnalyzerTransformationIter<L> {
    fn default() -> Self {
        Self {
            analyzer_transformations: vec![].into_iter(),
        }
    }
}

impl<L: Language> AnalyzerTransformationIter<L> {
    pub fn new<I>(transformations: I) -> Self
    where
        I: IntoIterator<Item = AnalyzerTransformation<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        Self {
            analyzer_transformations: transformations
                .into_iter()
                .collect::<Vec<AnalyzerTransformation<L>>>()
                .into_iter(),
        }
    }
}

impl<L: Language> Iterator for AnalyzerTransformationIter<L> {
    type Item = AnalyzerTransformation<L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.analyzer_transformations.next()
    }
}
impl<L: Language> FusedIterator for AnalyzerTransformationIter<L> {}

impl<L: Language> ExactSizeIterator for AnalyzerTransformationIter<L> {
    fn len(&self) -> usize {
        self.analyzer_transformations.len()
    }
}

#[derive(Debug, Clone)]
pub struct AnalyzerTransformation<L: Language> {
    pub mutation: BatchMutation<L>,
}

/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::registry::Rule)
pub(crate) struct RuleSignal<'phase, R: Rule> {
    root: &'phase RuleRoot<R>,
    query_result: <<R as Rule>::Query as Queryable>::Output,
    state: R::State,
    services: &'phase ServiceBag,
    /// An optional action to suppress the rule.
    suppression_action: &'phase dyn SuppressionAction<Language = RuleLanguage<R>>,
    /// A list of strings that are considered "globals" inside the analyzer
    options: &'phase AnalyzerOptions,
}

impl<'phase, R> RuleSignal<'phase, R>
where
    R: Rule + 'static,
{
    pub(crate) fn new(
        root: &'phase RuleRoot<R>,
        query_result: <<R as Rule>::Query as Queryable>::Output,
        state: R::State,
        services: &'phase ServiceBag,
        suppression_action: &'phase dyn SuppressionAction<
            Language = <<R as Rule>::Query as Queryable>::Language,
        >,
        options: &'phase AnalyzerOptions,
    ) -> Self {
        Self {
            root,
            query_result,
            state,
            services,
            suppression_action,
            options,
        }
    }
}

impl<R> AnalyzerSignal<RuleLanguage<R>> for RuleSignal<'_, R>
where
    R: Rule<Options: Default> + 'static,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let globals = self.options.globals();
        let preferred_quote = self.options.preferred_quote();
        let preferred_jsx_quote = self.options.preferred_jsx_quote();
        let preferred_indentation = self.options.preferred_indentation();
        let options = self.options.rule_options::<R>().unwrap_or_default();
        let ctx = RuleContext::new(
            &self.query_result,
            self.root,
            self.services,
            globals,
            self.options.file_path.as_path(),
            &options,
            preferred_quote,
            preferred_jsx_quote,
            preferred_indentation,
            self.options.jsx_runtime(),
            self.options.jsx_factory(),
            self.options.jsx_fragment_factory(),
            self.options.working_directory.as_deref(),
        )
        .ok()?;

        R::diagnostic(&ctx, &self.state).map(|mut diagnostic| {
            diagnostic.severity = ctx.metadata().severity;

            if let Some(issue_number) = ctx.metadata().issue_number {
                let url = format!("https://github.com/biomejs/biome/issues/{}", issue_number);
                diagnostic = diagnostic.note(markup! {
                 "This rule is still being actively worked on, so it may be missing features or have rough edges. Visit "<Hyperlink href={url.as_str()}>{url.as_str()}</Hyperlink>" for more information or to report possible bugs."
                });
            }
            if <R::Group as RuleGroup>::NAME == "nursery" {
                diagnostic = diagnostic.note(markup! {
                    "This rule belongs to the nursery group, which means it is not yet stable and may change in the future. Visit "<Hyperlink href="https://biomejs.dev/linter/#nursery">"https://biomejs.dev/linter/#nursery"</Hyperlink>" for more information."
                });
            }
            AnalyzerDiagnostic::from(diagnostic)
        })
    }

    fn actions(&self) -> AnalyzerActionIter<RuleLanguage<R>> {
        let globals = self.options.globals();

        let configured_applicability = if let Some(fix_kind) = self.options.rule_fix_kind::<R>() {
            match fix_kind {
                crate::FixKind::None => {
                    // The action is disabled
                    return AnalyzerActionIter::new(vec![]);
                }
                crate::FixKind::Safe => Some(Applicability::Always),
                crate::FixKind::Unsafe => Some(Applicability::MaybeIncorrect),
            }
        } else {
            None
        };
        let options = self.options.rule_options::<R>().unwrap_or_default();
        let ctx = RuleContext::new(
            &self.query_result,
            self.root,
            self.services,
            globals,
            self.options.file_path.as_path(),
            &options,
            self.options.preferred_quote(),
            self.options.preferred_jsx_quote(),
            self.options.preferred_indentation(),
            self.options.jsx_runtime(),
            self.options.jsx_factory(),
            self.options.jsx_fragment_factory(),
            self.options.working_directory.as_deref(),
        )
        .ok();
        let mut actions = Vec::new();
        if let Some(ctx) = ctx {
            if let Some(action) = R::action(&ctx, &self.state) {
                actions.push(AnalyzerAction {
                    rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
                    applicability: configured_applicability.unwrap_or(action.applicability()),
                    category: action.category,
                    mutation: action.mutation,
                    message: action.message,
                    text_edit: None,
                });
            };
            if let Some(text_range) = R::text_range(&ctx, &self.state)
                && let Some(suppression_action) = R::inline_suppression(
                    &ctx,
                    &text_range,
                    self.suppression_action,
                    self.options.suppression_reason.as_deref(),
                )
            {
                let action = AnalyzerAction {
                    rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
                    category: ActionCategory::Other(OtherActionCategory::InlineSuppression),
                    applicability: Applicability::Always,
                    mutation: suppression_action.mutation,
                    message: suppression_action.message,
                    text_edit: None,
                };
                actions.push(action);
            }

            if let Some(suppression_action) =
                R::top_level_suppression(&ctx, self.suppression_action)
            {
                let action = AnalyzerAction {
                    rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
                    category: ActionCategory::Other(OtherActionCategory::ToplevelSuppression),
                    applicability: Applicability::Always,
                    mutation: suppression_action.mutation,
                    message: suppression_action.message,
                    text_edit: None,
                };
                actions.push(action);
            }

            AnalyzerActionIter::new(actions)
        } else {
            AnalyzerActionIter::new(vec![])
        }
    }

    fn transformations(&self) -> AnalyzerTransformationIter<RuleLanguage<R>> {
        let globals = self.options.globals();
        let options = self.options.rule_options::<R>().unwrap_or_default();
        let ctx = RuleContext::new(
            &self.query_result,
            self.root,
            self.services,
            globals,
            self.options.file_path.as_path(),
            &options,
            self.options.preferred_quote(),
            self.options.preferred_jsx_quote(),
            self.options.preferred_indentation(),
            self.options.jsx_runtime(),
            self.options.jsx_factory(),
            self.options.jsx_fragment_factory(),
            self.options.working_directory.as_deref(),
        )
        .ok();
        if let Some(ctx) = ctx {
            let mut transformations = Vec::new();
            let mutation = R::transform(&ctx, &self.state);
            if let Some(mutation) = mutation {
                let transformation = AnalyzerTransformation { mutation };
                transformations.push(transformation)
            }
            AnalyzerTransformationIter::new(transformations)
        } else {
            AnalyzerTransformationIter::new(vec![])
        }
    }
}
