use biome_analyze::{GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleMetadata};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
pub enum Doc {
    Rule(RuleMetadata),
    DaemonLogs,
    Unknown(String),
}

impl FromStr for Doc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "daemon-logs" => Ok(Doc::DaemonLogs),
            _ => {
                if let Some(metadata) = LintRulesVisitor::new().get_metadata(s) {
                    return Ok(Doc::Rule(metadata));
                };

                Ok(Doc::Unknown(s.to_string()))
            }
        }
    }
}

struct LintRulesVisitor {
    rules_metadata: BTreeMap<&'static str, RuleMetadata>,
}

impl LintRulesVisitor {
    fn new() -> Self {
        let mut visitor = Self {
            rules_metadata: BTreeMap::new(),
        };

        biome_graphql_analyze::visit_registry(&mut visitor);
        biome_css_analyze::visit_registry(&mut visitor);
        biome_json_analyze::visit_registry(&mut visitor);
        biome_js_analyze::visit_registry(&mut visitor);

        visitor
    }

    fn get_metadata(&mut self, name: &str) -> Option<RuleMetadata> {
        self.rules_metadata.remove(name)
    }
}

impl RegistryVisitor<JsLanguage> for LintRulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.rules_metadata.insert(R::METADATA.name, R::METADATA);
    }

    fn record_category<C: biome_analyze::GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }
}

impl RegistryVisitor<JsonLanguage> for LintRulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.rules_metadata.insert(R::METADATA.name, R::METADATA);
    }

    fn record_category<C: biome_analyze::GroupCategory<Language = JsonLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }
}

impl RegistryVisitor<CssLanguage> for LintRulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.rules_metadata.insert(R::METADATA.name, R::METADATA);
    }

    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }
}

impl RegistryVisitor<GraphqlLanguage> for LintRulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.rules_metadata.insert(R::METADATA.name, R::METADATA);
    }

    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }
}
