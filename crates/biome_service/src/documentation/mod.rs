use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_rowan::Language;
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
            "daemon-logs" => Ok(Self::DaemonLogs),
            _ => {
                if let Some(metadata) = RulesVisitor::new().get_metadata(s) {
                    return Ok(Self::Rule(metadata));
                };

                Ok(Self::Unknown(s.to_string()))
            }
        }
    }
}

struct RulesVisitor {
    rules_metadata: BTreeMap<&'static str, RuleMetadata>,
}

impl RulesVisitor {
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

    fn store_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let category = <R::Group as RuleGroup>::Category::CATEGORY;
        if matches!(category, RuleCategory::Lint | RuleCategory::Action) {
            self.rules_metadata.insert(R::METADATA.name, R::METADATA);
        }
    }
}

impl RegistryVisitor<JsLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.store_rule::<R, JsLanguage>();
    }
}

impl RegistryVisitor<JsonLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, JsonLanguage>();
    }
}

impl RegistryVisitor<CssLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, CssLanguage>();
    }
}

impl RegistryVisitor<GraphqlLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, GraphqlLanguage>();
    }
}
