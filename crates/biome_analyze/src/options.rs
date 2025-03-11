use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;

use crate::{FixKind, Rule, RuleKey};
use std::any::{Any, TypeId};
use std::fmt::Debug;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug)]
pub struct RuleOptions(TypeId, Box<dyn Any>, Option<FixKind>);

impl RuleOptions {
    /// Creates a new [RuleOptions]
    pub fn new<O: 'static>(options: O, fix_kind: Option<FixKind>) -> Self {
        Self(TypeId::of::<O>(), Box::new(options), fix_kind)
    }

    /// It returns the deserialized rule option
    pub fn value<O: 'static>(&self) -> &O {
        let RuleOptions(type_id, value, _) = &self;
        let current_id = TypeId::of::<O>();
        debug_assert_eq!(type_id, &current_id);
        // SAFETY: the code should fail when asserting the types.
        // If the code throws an error here, it means that the developer didn't test
        // the rule with the options
        value.downcast_ref::<O>().unwrap()
    }

    pub fn fix_kind(&self) -> Option<FixKind> {
        self.2
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Default)]
pub struct AnalyzerRules(FxHashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: RuleOptions) {
        self.0.insert(rule_key, options);
    }

    /// It retrieves the options of a stored rule, given its name
    pub fn get_rule_options<O: 'static>(&self, rule_key: &RuleKey) -> Option<&O> {
        self.0.get(rule_key).map(|o| o.value::<O>())
    }

    pub fn get_rule_fix_kind(&self, rule_key: &RuleKey) -> Option<FixKind> {
        self.0.get(rule_key).and_then(|options| options.fix_kind())
    }
}

/// A data structured derived from the `biome.json` file
#[derive(Debug, Default)]
pub struct AnalyzerConfiguration {
    /// A list of rules and their options
    pub(crate) rules: AnalyzerRules,

    /// A collections of bindings that the analyzers should consider as "external".
    ///
    /// For example, lint rules should ignore them.
    globals: Vec<Box<str>>,

    /// Allows to choose a different quote when applying fixes inside the lint rules
    preferred_quote: PreferredQuote,

    /// Allows to choose a different JSX quote when applying fixes inside the lint rules
    pub preferred_jsx_quote: PreferredQuote,

    /// Indicates the type of runtime or transformation used for interpreting JSX.
    jsx_runtime: Option<JsxRuntime>,
}

impl AnalyzerConfiguration {
    pub fn with_rules(mut self, rules: AnalyzerRules) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_globals(mut self, globals: Vec<Box<str>>) -> Self {
        self.globals = globals;
        self
    }

    pub fn with_jsx_runtime(mut self, jsx_runtime: JsxRuntime) -> Self {
        self.jsx_runtime = Some(jsx_runtime);
        self
    }

    pub fn with_preferred_quote(mut self, preferred_quote: PreferredQuote) -> Self {
        self.preferred_quote = preferred_quote;
        self
    }

    pub fn with_preferred_jsx_quote(mut self, preferred_jsx_quote: PreferredQuote) -> Self {
        self.preferred_jsx_quote = preferred_jsx_quote;
        self
    }
}

/// A set of information useful to the analyzer infrastructure
#[derive(Debug, Default)]
pub struct AnalyzerOptions {
    /// A data structured derived from the [`biome.json`] file
    pub(crate) configuration: AnalyzerConfiguration,

    /// The file that is being analyzed
    pub file_path: Utf8PathBuf,

    /// Suppression reason used when applying a suppression code action
    pub(crate) suppression_reason: Option<String>,
}

impl AnalyzerOptions {
    pub fn with_file_path(mut self, file_path: impl Into<Utf8PathBuf>) -> Self {
        self.file_path = file_path.into();
        self
    }

    pub fn with_configuration(mut self, analyzer_configuration: AnalyzerConfiguration) -> Self {
        self.configuration = analyzer_configuration;
        self
    }

    pub fn with_suppression_reason(mut self, reason: Option<&str>) -> Self {
        self.suppression_reason = reason.map(String::from);
        self
    }

    pub fn push_globals(&mut self, globals: Vec<Box<str>>) {
        self.configuration.globals.extend(globals);
    }

    pub fn globals(&self) -> Vec<&str> {
        self.configuration
            .globals
            .iter()
            .map(AsRef::as_ref)
            .collect()
    }

    pub fn jsx_runtime(&self) -> Option<JsxRuntime> {
        self.configuration.jsx_runtime
    }

    pub fn rule_options<R>(&self) -> Option<R::Options>
    where
        R: Rule<Options: Clone> + 'static,
    {
        self.configuration
            .rules
            .get_rule_options::<R::Options>(&RuleKey::rule::<R>())
            .cloned()
    }

    pub fn rule_fix_kind<R>(&self) -> Option<FixKind>
    where
        R: Rule<Options: Clone> + 'static,
    {
        self.configuration
            .rules
            .get_rule_fix_kind(&RuleKey::rule::<R>())
    }

    pub fn preferred_quote(&self) -> &PreferredQuote {
        &self.configuration.preferred_quote
    }

    pub fn preferred_jsx_quote(&self) -> &PreferredQuote {
        &self.configuration.preferred_jsx_quote
    }
}

#[derive(Debug, Default)]
pub enum PreferredQuote {
    /// Double quotes
    #[default]
    Double,
    /// Single quotes
    Single,
}

impl PreferredQuote {
    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Double)
    }

    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum JsxRuntime {
    #[default]
    Transparent,
    ReactClassic,
}
