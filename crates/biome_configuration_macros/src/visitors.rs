use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "lang_css")]
use biome_css_syntax::CssLanguage;
#[cfg(feature = "lang_graphql")]
use biome_graphql_syntax::GraphqlLanguage;
#[cfg(feature = "lang_html")]
use biome_html_syntax::HtmlLanguage;
#[cfg(feature = "lang_js")]
use biome_js_syntax::JsLanguage;
#[cfg(feature = "lang_json")]
use biome_json_syntax::JsonLanguage;

// ======= LINT ======
#[derive(Default)]
pub struct LintRulesVisitor {
    pub groups: BTreeMap<&'static str, BTreeMap<&'static str, biome_analyze::RuleMetadata>>,
    /// Mapping from domain to group/rule
    /// e.g next => (<group>/<rule>, <group>/<rule>)
    #[expect(
        clippy::allow_attributes,
        reason = "`dead_code` is feature-dependent here; `expect(dead_code)` is unfulfilled when language features use this field."
    )]
    #[allow(
        dead_code,
        reason = "`domains` is only used when feature-gated visitors populate rule domains."
    )]
    pub domains: BTreeMap<&'static str, BTreeSet<(&'static str, &'static str)>>,
}

#[cfg(feature = "lang_js")]
impl biome_analyze::RegistryVisitor<JsLanguage> for LintRulesVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = JsLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);

        for domain in R::METADATA.domains.iter() {
            self.domains.entry(domain.as_str()).or_default().insert((
                <R::Group as biome_analyze::RuleGroup>::NAME,
                R::METADATA.name,
            ));
        }
    }
}

#[cfg(feature = "lang_json")]
impl biome_analyze::RegistryVisitor<JsonLanguage> for LintRulesVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = JsonLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = JsonLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_css")]
impl biome_analyze::RegistryVisitor<CssLanguage> for LintRulesVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = CssLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = CssLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_graphql")]
impl biome_analyze::RegistryVisitor<GraphqlLanguage> for LintRulesVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = GraphqlLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_html")]
impl biome_analyze::RegistryVisitor<HtmlLanguage> for LintRulesVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = HtmlLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

// ======= ASSIST ======
#[derive(Default)]
pub struct AssistActionsVisitor {
    pub groups: BTreeMap<&'static str, BTreeMap<&'static str, biome_analyze::RuleMetadata>>,
}

#[cfg(feature = "lang_js")]
impl biome_analyze::RegistryVisitor<JsLanguage> for AssistActionsVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = JsLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_json")]
impl biome_analyze::RegistryVisitor<JsonLanguage> for AssistActionsVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = JsonLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = JsonLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_css")]
impl biome_analyze::RegistryVisitor<CssLanguage> for AssistActionsVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = CssLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = CssLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_graphql")]
impl biome_analyze::RegistryVisitor<GraphqlLanguage> for AssistActionsVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = GraphqlLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

#[cfg(feature = "lang_html")]
impl biome_analyze::RegistryVisitor<HtmlLanguage> for AssistActionsVisitor {
    fn record_category<C: biome_analyze::GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, biome_analyze::RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: biome_analyze::Rule<
                Options: Default,
                Query: biome_analyze::Queryable<Language = HtmlLanguage, Output: Clone>,
            > + 'static,
    {
        self.groups
            .entry(<R::Group as biome_analyze::RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}
