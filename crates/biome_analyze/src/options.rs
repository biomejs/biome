use crate::{Rule, RuleKey};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug)]
pub struct RuleOptions((TypeId, Box<dyn Any>));

impl RuleOptions {
    /// It returns the deserialized rule option
    pub fn value<O: 'static>(&self) -> &O {
        let (type_id, value) = &self.0;
        let current_id = TypeId::of::<O>();
        debug_assert_eq!(type_id, &current_id);
        // SAFETY: the code should fail when asserting the types.
        // If the code throws an error here, it means that the developer didn't test
        // the rule with the options
        value.downcast_ref::<O>().unwrap()
    }

    /// Creates a new [RuleOptions]
    pub fn new<O: 'static>(options: O) -> Self {
        Self((TypeId::of::<O>(), Box::new(options)))
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Default)]
pub struct AnalyzerRules(HashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: RuleOptions) {
        self.0.insert(rule_key, options);
    }

    /// It retrieves the options of a stored rule, given its name
    pub fn get_rule_options<O: 'static>(&self, rule_key: &RuleKey) -> Option<&O> {
        self.0.get(rule_key).map(|o| o.value::<O>())
    }
}

/// A data structured derived from the `biome.json` file
#[derive(Debug, Default)]
pub struct AnalyzerConfiguration {
    /// A list of rules and their options
    pub rules: AnalyzerRules,

    /// A collections of bindings that the analyzers should consider as "external".
    ///
    /// For example, lint rules should ignore them.
    pub globals: Vec<String>,
}

/// A set of information useful to the analyzer infrastructure
#[derive(Debug, Default)]
pub struct AnalyzerOptions {
    /// A data structured derived from the [`biome.json`] file
    pub configuration: AnalyzerConfiguration,

    /// The file that is being analyzed
    pub file_path: PathBuf,
}
impl AnalyzerOptions {
    pub fn globals(&self) -> Vec<&str> {
        self.configuration
            .globals
            .iter()
            .map(|global| global.as_str())
            .collect()
    }

    pub fn rule_options<R: 'static>(&self) -> Option<R::Options>
    where
        R: Rule,
        R::Options: Clone,
    {
        self.configuration
            .rules
            .get_rule_options::<R::Options>(&RuleKey::rule::<R>())
            .map(R::Options::clone)
    }
}
