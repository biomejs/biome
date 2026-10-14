use crate::{
    AddVisitor, AnalysisFilter, GroupCategory, QueryMatcher, Rule, RuleCategories, RuleFilter,
    RuleGroup, RuleKey, RuleMetadata, ServiceBag, SignalEntry, Visitor,
    context::RuleContext,
    matcher::{GroupKey, MatchQueryParams},
    query::{QueryKey, Queryable},
    signals::RuleSignal,
};
use biome_diagnostics::Error;
use biome_rowan::{AstNode, Language, RawSyntaxKind, SyntaxKind, SyntaxKindSet, SyntaxNode};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    any::TypeId,
    borrow,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
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
    /// Which rules run on which query matches. Only depends on the
    /// [AnalysisFilter], so a [RuleRegistryCache] can share it between runs.
    rules: Arc<RegistryRules<L>>,
    /// Holds a state for every rule in [RegistryRules], indexed by
    /// [RegistryRule::state_index]
    rule_states: Vec<RuleState<L>>,
}

impl<L: Language + Default> RuleRegistry<L> {
    pub fn builder<'a>(
        filter: &'a AnalysisFilter<'a>,
        root: &'a L::Root,
    ) -> RuleRegistryBuilder<'a, L> {
        RuleRegistryBuilder {
            filter,
            root,
            rules: RegistryRules::default(),
            services: ServiceBag::default(),
            diagnostics: Vec::new(),
        }
    }
}

/// The part of a [RuleRegistry] that does not depend on the analyzed file.
#[derive(Default)]
struct RegistryRules<L: Language> {
    /// Holds a collection of rules for each phase.
    phase_rules: [PhaseRules<L>; 2],
    /// Number of rules in all phases.
    rule_count: usize,
    /// Registers the visitors of each rule's query, in the order the rules
    /// were recorded.
    visitor_builders: Vec<VisitorBuilder<L>>,
}

/// Holds a collection of rules for each phase.
#[derive(Default)]
struct PhaseRules<L: Language> {
    /// Maps the [TypeId] of known query matches types to the corresponding list of rules
    type_rules: FxHashMap<TypeId, TypeRules<L>>,
    /// Union of the kinds of all the [QueryKey::Syntax] rules in this phase
    syntax_kinds: SyntaxKindSet<L>,
}

enum TypeRules<L: Language> {
    SyntaxRules { rules: Vec<SyntaxKindRules<L>> },
    TypeRules { rules: Vec<RegistryRule<L>> },
}

type VisitorMap<L> = BTreeMap<(Phases, TypeId), Box<dyn Visitor<Language = L>>>;

type VisitorBuilder<L> = fn(&mut VisitorMap<L>, &LanguageRoot<L>);

fn build_query_visitor<Q: Queryable>(
    visitors: &mut VisitorMap<Q::Language>,
    root: &LanguageRoot<Q::Language>,
) where
    Q::Language: 'static,
{
    Q::build_visitor(visitors, root);
}

pub struct RuleRegistryBuilder<'a, L: Language> {
    filter: &'a AnalysisFilter<'a>,
    root: &'a L::Root,
    rules: RegistryRules<L>,
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

        let rules = &mut self.rules;
        rules.phase_rules[R::phase() as usize].insert_rule(
            <R::Query as Queryable>::key(),
            RegistryRule::new::<R>(rules.rule_count),
        );
        rules.rule_count += 1;
        rules.visitor_builders.push(build_query_visitor::<R::Query>);
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
    }
}

impl<L: Language> AddVisitor<L> for VisitorMap<L> {
    fn add_visitor<F, V>(&mut self, phase: Phases, visitor: F)
    where
        F: FnOnce() -> V,
        V: Visitor<Language = L> + 'static,
    {
        self.entry((phase, TypeId::of::<V>()))
            .or_insert_with(move || Box::new((visitor)()));
    }
}

type BuilderResult<L> = (RuleRegistry<L>, ServiceBag, Vec<Error>, VisitorMap<L>);

impl<L: Language + Default> RuleRegistryBuilder<'_, L> {
    pub fn build(self) -> BuilderResult<L> {
        let (registry, visitors) = RuleRegistry::from_rules(Arc::new(self.rules), self.root);
        (registry, self.services, self.diagnostics, visitors)
    }
}

impl<L: Language + Default> RuleRegistry<L> {
    /// Creates a registry for one analyzer run over `root`, with fresh rule
    /// states and the visitors of every rule's query.
    fn from_rules(rules: Arc<RegistryRules<L>>, root: &L::Root) -> (Self, VisitorMap<L>) {
        let mut visitors = VisitorMap::default();
        for build_visitor in &rules.visitor_builders {
            build_visitor(&mut visitors, root);
        }

        let rule_states = std::iter::repeat_with(RuleState::default)
            .take(rules.rule_count)
            .collect();

        (Self { rules, rule_states }, visitors)
    }
}

/// Reuses the file-independent part of a [RuleRegistry] across analyzer runs
/// whose [AnalysisFilter] selects the same rules.
///
/// Which rules a registry holds depends on the categories, enabled rules and
/// disabled rules of the filter, and on the rules a language's
/// `visit_registry` function records. A cache must therefore only ever be
/// used with one `visit_registry` function. The filter's range, the analyzed
/// file, and the analyzer options are not part of the key: rule states and
/// query visitors are created anew for every run, and rules read their
/// options when they run.
pub struct RuleRegistryCache<L: Language> {
    /// Most recently used entry first.
    entries: RefCell<Vec<RegistryCacheEntry<L>>>,
}

struct RegistryCacheEntry<L: Language> {
    categories: RuleCategories,
    enabled_rules: Option<Box<[OwnedRuleFilter]>>,
    disabled_rules: Box<[OwnedRuleFilter]>,
    rules: Arc<RegistryRules<L>>,
}

impl<L: Language> RegistryCacheEntry<L> {
    fn matches(&self, filter: &AnalysisFilter) -> bool {
        self.categories == filter.categories
            && match (&self.enabled_rules, filter.enabled_rules) {
                (Some(cached), Some(enabled)) => OwnedRuleFilter::slice_eq(cached, enabled),
                (None, None) => true,
                _ => false,
            }
            && OwnedRuleFilter::slice_eq(&self.disabled_rules, filter.disabled_rules)
    }
}

#[derive(Debug)]
enum OwnedRuleFilter {
    Group(Box<str>),
    Rule(Box<str>, Box<str>),
}

impl OwnedRuleFilter {
    fn from_filters(filters: &[RuleFilter]) -> Box<[Self]> {
        filters
            .iter()
            .map(|filter| match *filter {
                RuleFilter::Group(group) => Self::Group(group.into()),
                RuleFilter::Rule(group, rule) => Self::Rule(group.into(), rule.into()),
            })
            .collect()
    }

    fn slice_eq(owned: &[Self], filters: &[RuleFilter]) -> bool {
        owned.len() == filters.len()
            && owned
                .iter()
                .zip(filters)
                .all(|(owned, filter)| match (owned, *filter) {
                    (Self::Group(a), RuleFilter::Group(b)) => **a == *b,
                    (Self::Rule(a_group, a_rule), RuleFilter::Rule(b_group, b_rule)) => {
                        **a_rule == *b_rule && **a_group == *b_group
                    }
                    _ => false,
                })
    }
}

impl<L: Language> RuleRegistryCache<L> {
    /// Number of filters remembered at once. An analyzer typically sees one
    /// filter per kind of request (lint, fix, code actions), plus one per
    /// override that changes the rule set.
    const CAPACITY: usize = 8;

    pub const fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
        }
    }
}

impl<L: Language + Default> RuleRegistryCache<L> {
    /// Returns the same result as calling `visit_registry` on
    /// [RuleRegistry::builder] and then [RuleRegistryBuilder::build], but only
    /// calls `visit_registry` when no remembered entry has an equal filter.
    pub fn build<'a>(
        &self,
        filter: &'a AnalysisFilter<'a>,
        root: &'a L::Root,
        visit_registry: impl FnOnce(&mut RuleRegistryBuilder<'a, L>),
    ) -> BuilderResult<L> {
        let cached = {
            let mut entries = self.entries.borrow_mut();
            entries
                .iter()
                .position(|entry| entry.matches(filter))
                .map(|index| {
                    entries[..=index].rotate_right(1);
                    entries[0].rules.clone()
                })
        };

        if let Some(rules) = cached {
            let (registry, visitors) = RuleRegistry::from_rules(rules, root);
            return (registry, ServiceBag::default(), Vec::new(), visitors);
        }

        let mut builder = RuleRegistry::builder(filter, root);
        visit_registry(&mut builder);
        let RuleRegistryBuilder {
            rules,
            services,
            diagnostics,
            ..
        } = builder;
        let rules = Arc::new(rules);

        let mut entries = self.entries.borrow_mut();
        entries.truncate(Self::CAPACITY - 1);
        entries.insert(
            0,
            RegistryCacheEntry {
                categories: filter.categories,
                enabled_rules: filter.enabled_rules.map(OwnedRuleFilter::from_filters),
                disabled_rules: OwnedRuleFilter::from_filters(filter.disabled_rules),
                rules: rules.clone(),
            },
        );
        drop(entries);

        let (registry, visitors) = RuleRegistry::from_rules(rules, root);
        (registry, services, diagnostics, visitors)
    }
}

impl<L: Language> Default for RuleRegistryCache<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Language + 'static> QueryMatcher<L> for RuleRegistry<L> {
    fn match_query(&mut self, mut params: MatchQueryParams<L>) {
        let phase = &self.rules.phase_rules[params.phase as usize];

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

        // Run all the rules registered to this QueryMatch
        for rule in rules {
            let state = &mut self.rule_states[rule.state_index];
            // TODO: #3394 track error in the signal queue
            let _ = (rule.run)(&mut params, state);
        }
    }

    fn syntax_query_kinds(&self, phase: Phases) -> Option<SyntaxKindSet<L>> {
        Some(self.rules.phase_rules[phase as usize].syntax_kinds)
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
        }
    }
}
