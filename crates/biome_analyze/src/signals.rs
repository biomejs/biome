use crate::categories::{
    SUPPRESSION_INLINE_ACTION_CATEGORY, SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY,
};
use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{RuleLanguage, RuleRoot},
    rule::Rule,
    AnalyzerDiagnostic, AnalyzerOptions, OtherActionCategory, Queryable, RuleGroup, ServiceBag,
    SuppressionAction,
};
use biome_console::MarkupBuf;
use biome_diagnostics::{advice::CodeSuggestionAdvice, Applicability, CodeSuggestion, Error};
use biome_rowan::{BatchMutation, Language};
use std::iter::FusedIterator;
use std::marker::PhantomData;
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
        let (_, suggestion) = action.mutation.as_text_range_and_edit().unwrap_or_default();
        CodeSuggestionAdvice {
            applicability: action.applicability,
            msg: action.message,
            suggestion,
        }
    }
}

impl<L: Language> From<AnalyzerAction<L>> for CodeSuggestionItem {
    fn from(action: AnalyzerAction<L>) -> Self {
        let (range, suggestion) = action.mutation.as_text_range_and_edit().unwrap_or_default();

        CodeSuggestionItem {
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
        let options = self.options.rule_options::<R>().unwrap_or_default();
        let ctx = RuleContext::new(
            &self.query_result,
            self.root,
            self.services,
            &globals,
            self.options.file_path.as_path(),
            &options,
            preferred_quote,
            preferred_jsx_quote,
            self.options.jsx_runtime(),
        )
        .ok()?;

        R::diagnostic(&ctx, &self.state).map(|mut diagnostic| {
            diagnostic.severity = ctx.metadata().severity;
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
            &globals,
            self.options.file_path.as_path(),
            &options,
            self.options.preferred_quote(),
            self.options.preferred_jsx_quote(),
            self.options.jsx_runtime(),
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
                });
            };
            if let Some(text_range) = R::text_range(&ctx, &self.state) {
                if let Some(suppression_action) = R::inline_suppression(
                    &ctx,
                    &text_range,
                    self.suppression_action,
                    self.options.suppression_reason.as_deref(),
                ) {
                    let action = AnalyzerAction {
                        rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
                        category: ActionCategory::Other(OtherActionCategory::InlineSuppression),
                        applicability: Applicability::Always,
                        mutation: suppression_action.mutation,
                        message: suppression_action.message,
                    };
                    actions.push(action);
                }
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
            &globals,
            self.options.file_path.as_path(),
            &options,
            self.options.preferred_quote(),
            self.options.preferred_jsx_quote(),
            self.options.jsx_runtime(),
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
