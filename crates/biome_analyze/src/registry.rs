use crate::{
    AddVisitor, AnalysisFilter, GroupCategory, QueryMatcher, Rule, RuleCategory, RuleGroup,
    RuleKey, RuleMetadata, ServiceBag, SignalEntry, SignalRuleKey, Visitor,
    context::RuleContext,
    diagnostics::AnalyzerPanicDiagnostic,
    matcher::{GroupKey, MatchQueryParams},
    query::{QueryKey, Queryable},
    signals::{DiagnosticSignal, RuleSignal},
};
use biome_diagnostics::{Error, panic::catch_unwind};
use biome_rowan::{AstNode, Language, RawSyntaxKind, SyntaxKind, SyntaxKindSet, SyntaxNode};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    any::TypeId,
    borrow,
    collections::{BTreeMap, BTreeSet},
    panic::AssertUnwindSafe,
};

/// Defines all the phases that the [RuleRegistry] supports.
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phases {
    Syntax = 0,
    Semantic = 1,
}

/// Defines which phase a rule will run. This will be defined
/// by the set of services a rule demands.
pub trait Phase {
    fn phase() -> Phases;
}

/// If a rule do not need any service it can run on the syntax phase.
impl Phase for () {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

pub trait RegistryVisitor<L: Language> {
    /// Record the category `C` to this visitor
    fn record_category<C: GroupCategory<Language = L>>(&mut self) {
        C::record_groups(self);
    }

    /// Record the group `G` to this visitor
    fn record_group<G: RuleGroup<Language = L>>(&mut self) {
        G::record_rules(self);
    }

    /// Record the rule `R` to this visitor
    fn record_rule<R>(&mut self)
    where
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static;
}

/// Stores metadata information for all the rules in the registry, sorted
/// alphabetically
#[derive(Debug, Default)]
pub struct MetadataRegistry {
    inner: BTreeSet<MetadataKey>,
}

impl MetadataRegistry {
    /// Adds the rule names registered in `other`.
    pub fn extend(&mut self, other: &Self) {
        self.inner.extend(other.inner.iter().copied());
    }

    /// Return a unique identifier for a rule group if it's known by this registry
    pub fn find_group(&self, group: &str) -> Option<GroupKey> {
        let key = self.inner.get(group)?;
        Some(key.into_group_key())
    }

    /// Return a unique identifier for a rule if it's known by this registry
    pub fn find_rule(&self, group: &str, rule: &str) -> Option<RuleKey> {
        let key = self.inner.get(&(group, rule))?;
        Some(key.into_rule_key())
    }

    pub(crate) fn insert_rule(&mut self, group: &'static str, rule: &'static str) {
        self.inner.insert(MetadataKey {
            inner: (group, rule),
        });
    }
}

impl<L: Language> RegistryVisitor<L> for MetadataRegistry {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        self.insert_rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);
    }
}

/// The rule registry holds type-erased instances of all active analysis rules
/// for each phase.
/// What defines a phase is the set of services that a phase offers. Currently
/// we have:
/// - Syntax Phase: No services are offered, thus its rules can be run immediately;
/// - Semantic Phase: Offers the semantic model, thus these rules can only run
///   after the "SemanticModel" is ready, which demands a whole transverse of the parsed tree.
pub struct RuleRegistry<L: Language> {
    /// Holds a collection of rules for each phase.
    phase_rules: [PhaseRules<L>; 2],
    panicked: bool,
}

impl<L: Language + Default> RuleRegistry<L> {
    pub fn builder<'a>(
        filter: &'a AnalysisFilter<'a>,
        root: &'a L::Root,
    ) -> RuleRegistryBuilder<'a, L> {
        RuleRegistryBuilder {
            filter,
            root,
            registry: Self {
                phase_rules: Default::default(),
                panicked: false,
            },
            visitors: BTreeMap::default(),
            services: ServiceBag::default(),
            diagnostics: Vec::new(),
        }
    }
}

/// Holds a collection of rules for each phase.
#[derive(Default)]
struct PhaseRules<L: Language> {
    /// Maps the [TypeId] of known query matches types to the corresponding list of rules
    type_rules: FxHashMap<TypeId, TypeRules<L>>,
    /// Union of the kinds of all the [QueryKey::Syntax] rules in this phase
    syntax_kinds: SyntaxKindSet<L>,
    /// Holds a list of states for all the rules in this phase
    rule_states: Vec<RuleState<L>>,
}

enum TypeRules<L: Language> {
    SyntaxRules { rules: Vec<SyntaxKindRules<L>> },
    TypeRules { rules: Vec<RegistryRule<L>> },
}

pub struct RuleRegistryBuilder<'a, L: Language> {
    filter: &'a AnalysisFilter<'a>,
    root: &'a L::Root,
    // Rule Registry
    registry: RuleRegistry<L>,
    // Analyzer Visitors
    visitors: BTreeMap<(Phases, TypeId), Box<dyn Visitor<Language = L>>>,
    // Service Bag
    services: ServiceBag,
    diagnostics: Vec<Error>,
}

impl<L: Language + Default + 'static> RegistryVisitor<L> for RuleRegistryBuilder<'_, L> {
    fn record_category<C: GroupCategory<Language = L>>(&mut self) {
        if self.filter.match_category::<C>() {
            C::record_groups(self);
        }
    }

    fn record_group<G: RuleGroup<Language = L>>(&mut self) {
        if self.filter.match_group::<G>() {
            G::record_rules(self);
        }
    }

    /// Add the rule `R` to the list of rules stores in this registry instance
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        if !self.filter.match_rule::<R>() {
            return;
        }

        let phase = R::phase() as usize;
        let phase = &mut self.registry.phase_rules[phase];

        phase.insert_rule(
            <R::Query as Queryable>::key(),
            RegistryRule::new::<R>(phase.rule_states.len()),
        );

        <R::Query as Queryable>::build_visitor(&mut self.visitors, self.root);
    }
}

impl<L: Language + Default + 'static> PhaseRules<L> {
    fn insert_rule(&mut self, key: QueryKey<L>, rule: RegistryRule<L>) {
        match key {
            QueryKey::Syntax(key) => {
                let TypeRules::SyntaxRules { rules } = self
                    .type_rules
                    .entry(TypeId::of::<SyntaxNode<L>>())
                    .or_insert_with(|| TypeRules::SyntaxRules { rules: Vec::new() })
                else {
                    unreachable!(
                        "the SyntaxNode type has already been registered as a TypeRules instead of a SyntaxRules, this is generally caused by an implementation of `Queryable::key` returning a `QueryKey::TypeId` with the type ID of `SyntaxNode`"
                    )
                };
                self.syntax_kinds = self.syntax_kinds.union(key);

                // Iterate on all the SyntaxKind variants this node can match
                for kind in key.iter() {
                    // Convert the numerical value of `kind` to an index in the
                    // `nodes` vector
                    let RawSyntaxKind(index) = kind.to_raw();
                    let index = usize::from(index);

                    // Ensure the vector has enough capacity by inserting empty
                    // `SyntaxKindRules` as required
                    if rules.len() <= index {
                        rules.resize_with(index + 1, SyntaxKindRules::new);
                    }

                    // Insert a handle to the rule into the `SyntaxKindRules` entry
                    // corresponding to the SyntaxKind index
                    let node = &mut rules[index];
                    node.rules.push(rule);
                }
            }
            QueryKey::TypeId(key) => {
                let TypeRules::TypeRules { rules } = self
                    .type_rules
                    .entry(key)
                    .or_insert_with(|| TypeRules::TypeRules { rules: Vec::new() })
                else {
                    unreachable!(
                        "the query type has already been registered as a SyntaxRules instead of a TypeRules, this is generally caused by an implementation of `Queryable::key` returning a `QueryKey::TypeId` with the type ID of `SyntaxNode`"
                    )
                };

                rules.push(rule);
            }
        }

        self.rule_states.push(RuleState::default());
    }
}

impl<L: Language> AddVisitor<L> for BTreeMap<(Phases, TypeId), Box<dyn Visitor<Language = L>>> {
    fn add_visitor<F, V>(&mut self, phase: Phases, visitor: F)
    where
        F: FnOnce() -> V,
        V: Visitor<Language = L> + 'static,
    {
        self.entry((phase, TypeId::of::<V>()))
            .or_insert_with(move || Box::new((visitor)()));
    }
}

type BuilderResult<L> = (
    RuleRegistry<L>,
    ServiceBag,
    Vec<Error>,
    BTreeMap<(Phases, TypeId), Box<dyn Visitor<Language = L>>>,
);

impl<L: Language> RuleRegistryBuilder<'_, L> {
    pub fn build(self) -> BuilderResult<L> {
        (
            self.registry,
            self.services,
            self.diagnostics,
            self.visitors,
        )
    }
}

impl<L: Language + 'static> QueryMatcher<L> for RuleRegistry<L> {
    fn match_query(&mut self, mut params: MatchQueryParams<L>) {
        if self.panicked {
            return;
        }

        let phase = &mut self.phase_rules[params.phase as usize];

        let query_type = params.query.type_id();
        let Some(rules) = phase.type_rules.get(&query_type) else {
            return;
        };

        let rules = match rules {
            TypeRules::SyntaxRules { rules } => {
                let node = params.query.downcast_ref::<SyntaxNode<L>>().unwrap();

                // Convert the numerical value of the SyntaxKind to an index in the
                // `syntax` vector
                let RawSyntaxKind(kind) = node.kind().to_raw();
                let kind = usize::from(kind);

                // Lookup the syntax entry corresponding to the SyntaxKind index
                match rules.get(kind) {
                    Some(entry) => &entry.rules,
                    None => return,
                }
            }
            TypeRules::TypeRules { rules } => rules,
        };

        let query_range = params.query.text_range();
        let mut panicked = false;

        // Run all the rules registered to this QueryMatch
        for rule in rules {
            let state = &mut phase.rule_states[rule.state_index];
            match catch_unwind(AssertUnwindSafe(|| (rule.run)(&mut params, state))) {
                Ok(result) => {
                    // TODO: #3394 track error in the signal queue
                    let _ = result;
                }
                Err(panic) => {
                    let thread_name = std::thread::current().name().map(str::to_owned);
                    let diagnostic = AnalyzerPanicDiagnostic::new(
                        rule.key,
                        rule.category,
                        query_range,
                        panic,
                        thread_name,
                    );
                    params.signal_queue.push(SignalEntry {
                        signal: Box::new(DiagnosticSignal::new(move || diagnostic.clone())),
                        rule: SignalRuleKey::Internal,
                        instances: Default::default(),
                        text_range: query_range,
                        category: rule.category,
                    });
                    panicked = true;
                    break;
                }
            }
        }

        if panicked {
            self.panicked = true;
        }
    }

    fn syntax_query_kinds(&self, phase: Phases) -> Option<SyntaxKindSet<L>> {
        Some(self.phase_rules[phase as usize].syntax_kinds)
    }

    fn has_panicked(&self) -> bool {
        self.panicked
    }
}

/// [SyntaxKindRules] holds a collection of [Rule]s that match a specific [SyntaxKind] value
struct SyntaxKindRules<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> SyntaxKindRules<L> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

pub(crate) type RuleLanguage<R> = QueryLanguage<<R as Rule>::Query>;
pub(crate) type QueryLanguage<N> = <N as Queryable>::Language;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

/// Key struct for a rule in the metadata map, sorted alphabetically
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataKey {
    inner: (&'static str, &'static str),
}

impl MetadataKey {
    fn into_group_key(self) -> GroupKey {
        let (group, _) = self.inner;
        GroupKey::new(group)
    }

    fn into_rule_key(self) -> RuleKey {
        let (group, rule) = self.inner;
        RuleKey::new(group, rule)
    }
}

impl<'a> borrow::Borrow<(&'a str, &'a str)> for MetadataKey {
    fn borrow(&self) -> &(&'a str, &'a str) {
        &self.inner
    }
}

impl borrow::Borrow<str> for MetadataKey {
    fn borrow(&self) -> &str {
        self.inner.0
    }
}

/// Metadata entry for a rule and its group in the registry
pub struct RegistryRuleMetadata {
    pub group: &'static str,
    pub rule: RuleMetadata,
}

impl RegistryRuleMetadata {
    pub fn to_rule_key(&self) -> RuleKey {
        RuleKey::new(self.group, self.rule.name)
    }
}

/// Internal representation of a single rule in the registry
#[derive(Copy, Clone)]
pub struct RegistryRule<L: Language> {
    run: RuleExecutor<L>,
    state_index: usize,
    key: RuleKey,
    category: RuleCategory,
}

/// Internal state for a given rule
#[derive(Default)]
struct RuleState<L: Language> {
    suppressions: RuleSuppressions<L>,
}

/// Set of nodes this rule has suppressed from matching its query
#[derive(Default)]
pub struct RuleSuppressions<L: Language> {
    inner: FxHashSet<SyntaxNode<L>>,
}

impl<L: Language> RuleSuppressions<L> {
    /// Suppress query matching for the given node
    pub fn suppress_node(&mut self, node: SyntaxNode<L>) {
        self.inner.insert(node);
    }
}

/// Executor for rule as a generic function pointer
type RuleExecutor<L> = fn(&mut MatchQueryParams<L>, &mut RuleState<L>) -> Result<(), Error>;

impl<L: Language + Default> RegistryRule<L> {
    fn new<R>(state_index: usize) -> Self
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<R>(
            params: &mut MatchQueryParams<RuleLanguage<R>>,
            state: &mut RuleState<RuleLanguage<R>>,
        ) -> Result<(), Error>
        where
            R: Rule<Options: Default, Query: Queryable<Output: Clone>> + 'static,
        {
            if let Some(node) = params.query.downcast_ref::<SyntaxNode<RuleLanguage<R>>>()
                && state.suppressions.inner.contains(node)
            {
                return Ok(());
            }

            // SAFETY: The rule should never get executed in the first place
            // if the query doesn't match
            let query_result = params.query.downcast_ref().unwrap();
            let query_result = <R::Query as Queryable>::unwrap_match(params.services, query_result);
            let globals = params.options.globals();
            let preferred_quote = params.options.preferred_quote();
            let preferred_jsx_quote = params.options.preferred_jsx_quote();
            let preferred_indentation = params.options.preferred_indentation();
            let jsx_runtime = params.options.jsx_runtime();
            let jsx_factory = params.options.jsx_factory();
            let jsx_fragment_factory = params.options.jsx_fragment_factory();
            let options = params.options.rule_options::<R>().unwrap_or_default();
            let working_directory = params.options.working_directory.as_deref();
            let ctx = RuleContext::new(
                &query_result,
                params.root,
                params.services,
                globals,
                params.options.file_path.as_path(),
                &options,
                preferred_quote,
                preferred_jsx_quote,
                preferred_indentation,
                jsx_runtime,
                jsx_factory,
                jsx_fragment_factory,
                working_directory,
            )?;

            let rule_timer = crate::profiling::start_rule::<R>();
            let signals = R::run(&ctx);
            rule_timer.stop();

            for result in signals {
                let text_range =
                    R::text_range(&ctx, &result).unwrap_or_else(|| params.query.text_range());

                R::suppressed_nodes(&ctx, &result, &mut state.suppressions);

                let instances = R::instances_for_signal(&result);
                let signal = Box::new(RuleSignal::<R>::new(
                    params.root,
                    query_result.clone(),
                    result,
                    params.services,
                    params.suppression_action,
                    params.options,
                ));

                params.signal_queue.push(SignalEntry {
                    signal,
                    rule: RuleKey::rule::<R>().into(),
                    instances,
                    text_range,
                    category: <R::Group as RuleGroup>::Category::CATEGORY,
                });
            }

            Ok(())
        }

        Self {
            run: run::<R>,
            state_index,
            key: RuleKey::rule::<R>(),
            category: <R::Group as RuleGroup>::Category::CATEGORY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnalysisFilter, AnalyzerOptions, ApplySuppression, Ast, GroupCategory, RuleMeta,
        SuppressionAction, context::RuleContext, matcher::Query,
    };
    use biome_diagnostics::{
        Diagnostic, DiagnosticExt, Error, category, print_diagnostic_to_string,
    };
    use biome_rowan::{
        AstNode, BatchMutation, SyntaxToken, TextRange,
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
    };
    use std::collections::BinaryHeap;

    enum TestCategory {}

    impl GroupCategory for TestCategory {
        type Language = RawLanguage;
        const CATEGORY: RuleCategory = RuleCategory::Lint;

        fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(_: &mut V) {}
    }

    enum TestGroup {}

    impl RuleGroup for TestGroup {
        type Language = RawLanguage;
        type Category = TestCategory;
        const NAME: &'static str = "test";

        fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(_: &mut V) {}
    }

    enum PanicRule {}

    impl RuleMeta for PanicRule {
        type Group = TestGroup;
        const METADATA: RuleMetadata = RuleMetadata::new("next", "panicRule", "Test rule.", "raw");
    }

    impl Rule for PanicRule {
        type Query = Ast<RawLanguageRoot>;
        type State = ();
        type Signals = Option<Self::State>;
        type Options = ();

        fn run(_: &RuleContext<Self>) -> Self::Signals {
            panic!("intentional analyzer panic");
        }
    }

    struct TestSuppressionAction;

    impl SuppressionAction for TestSuppressionAction {
        type Language = RawLanguage;

        fn find_token_for_inline_suppression(
            &self,
            _: SyntaxToken<Self::Language>,
        ) -> Option<ApplySuppression<Self::Language>> {
            None
        }

        fn apply_inline_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: ApplySuppression<Self::Language>,
            _: &str,
            _: &str,
            _: &TextRange,
        ) {
        }

        fn apply_top_level_suppression(
            &self,
            _: &mut BatchMutation<Self::Language>,
            _: SyntaxToken<Self::Language>,
            _: &str,
        ) {
        }

        fn suppression_top_level_comment(&self, _: &str) -> String {
            String::new()
        }
    }

    #[test]
    fn reports_rule_panics_as_diagnostics() {
        let root = RawLanguageRoot::unwrap_cast(RawSyntaxTreeBuilder::wrap_with_node(
            RawLanguageKind::ROOT,
            |builder| {
                builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
            },
        ));
        let filter = AnalysisFilter::default();
        let mut builder = RuleRegistry::builder(&filter, &root);
        builder.record_rule::<PanicRule>();
        let (mut registry, services, diagnostics, _) = builder.build();
        assert!(diagnostics.is_empty());

        let options = AnalyzerOptions::default();
        let suppression_action = TestSuppressionAction;
        let mut signal_queue = BinaryHeap::new();
        registry.match_query(MatchQueryParams {
            phase: Phases::Syntax,
            root: &root,
            query: Query::new(root.syntax().clone()),
            services: &services,
            signal_queue: &mut signal_queue,
            suppression_action: &suppression_action,
            options: &options,
        });

        assert!(registry.has_panicked());
        let entry = signal_queue.pop().expect("panic diagnostic");
        assert!(matches!(entry.rule, SignalRuleKey::Internal));

        let diagnostic = entry.signal.diagnostic().expect("panic diagnostic");
        assert_eq!(
            diagnostic.category(),
            Some(category!("internalError/panic"))
        );
        assert_eq!(
            diagnostic.get_span(),
            Some(root.syntax().text_trimmed_range())
        );
        let diagnostic = Error::from(diagnostic)
            .with_file_path("example.raw")
            .with_file_source_code("1");
        let output = print_diagnostic_to_string(&diagnostic);
        assert!(output.contains("test/panicRule"), "{output}");
        assert!(output.contains("intentional analyzer panic"), "{output}");

        registry.match_query(MatchQueryParams {
            phase: Phases::Syntax,
            root: &root,
            query: Query::new(root.syntax().clone()),
            services: &services,
            signal_queue: &mut signal_queue,
            suppression_action: &suppression_action,
            options: &options,
        });
        assert!(signal_queue.is_empty());
    }
}
