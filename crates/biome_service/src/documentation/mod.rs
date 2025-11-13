use biome_analyze::{
    FixKind, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleDomain, RuleGroup,
    RuleMetadata,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Padding, markup};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_syntax::HtmlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_rowan::Language;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
pub enum Doc {
    Rule(ExplainRule),
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

#[derive(Debug, Clone)]
pub struct ExplainRule {
    metadata: RuleMetadata,
    group: &'static str,
    category: &'static str,
}

struct RulesVisitor {
    rules_metadata: BTreeMap<&'static str, ExplainRule>,
}

impl RulesVisitor {
    fn new() -> Self {
        let mut visitor = Self {
            rules_metadata: BTreeMap::new(),
        };

        biome_graphql_analyze::visit_registry(&mut visitor);
        biome_html_analyze::visit_registry(&mut visitor);
        biome_css_analyze::visit_registry(&mut visitor);
        biome_json_analyze::visit_registry(&mut visitor);
        biome_js_analyze::visit_registry(&mut visitor);

        visitor
    }

    fn get_metadata(&mut self, name: &str) -> Option<ExplainRule> {
        self.rules_metadata.remove(name)
    }

    fn store_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let category = <R::Group as RuleGroup>::Category::CATEGORY;
        if matches!(category, RuleCategory::Lint | RuleCategory::Action) {
            let explain_rule = ExplainRule {
                metadata: R::METADATA,
                group: <R::Group as RuleGroup>::NAME,
                category: <<R::Group as RuleGroup>::Category as GroupCategory>::CATEGORY
                    .as_suppression_category(),
            };
            self.rules_metadata.insert(R::METADATA.name, explain_rule);
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

impl RegistryVisitor<HtmlLanguage> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, HtmlLanguage>();
    }
}

impl biome_console::fmt::Display for ExplainRule {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let metadata = &self.metadata;
        fmt.write_markup(markup! {
            <Emphasis>"Summary"</Emphasis>
        })?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Name: "<Emphasis>{metadata.name}</Emphasis>
        })?;
        fmt.write_str("\n")?;
        match metadata.fix_kind {
            FixKind::None => {
                fmt.write_markup(markup! {
                    "- No fix available."
                })?;
            }
            kind => {
                fmt.write_markup(markup! {
                    "- Fix: "<Emphasis>{kind}</Emphasis>
                })?;
            }
        }
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Default severity: "<Emphasis>{metadata.severity}</Emphasis>
        })?;
        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Available from version: "<Emphasis>{metadata.version}</Emphasis>
        })?;

        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            "- Diagnostic category: "<Emphasis>{format!("{}/{}/{}", self.category, self.group, metadata.name)}</Emphasis>
        })?;

        fmt.write_str("\n")?;

        if metadata.domains.is_empty() && metadata.recommended {
            fmt.write_markup(markup! {
                "- This rule is recommended"
            })?;
        }

        let domains = DisplayDomains(metadata.domains, metadata.recommended);

        fmt.write_str("\n")?;

        fmt.write_markup(markup!({ domains }))?;

        fmt.write_str("\n")?;

        fmt.write_markup(markup! {
            <Emphasis>"Description"</Emphasis>
        })?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        for line in metadata.docs.lines() {
            if let Some((_, remainder)) = line.split_once("## ") {
                fmt.write_markup(markup! {
                    <Emphasis>{remainder.trim_start()}</Emphasis>
                })?;
            } else if let Some((_, remainder)) = line.split_once("### ") {
                fmt.write_markup(markup! {
                    <Emphasis>{remainder.trim_start()}</Emphasis>
                })?;
            } else {
                fmt.write_str(line)?;
            }

            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

struct DisplayDomains(&'static [RuleDomain], bool);

impl Display for DisplayDomains {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let domains = self.0;
        let recommended = self.1;

        if domains.is_empty() {
            return Ok(());
        }

        fmt.write_markup(markup!(
            <Emphasis>"Domains"</Emphasis>
        ))?;
        fmt.write_str("\n")?;
        fmt.write_str("\n")?;

        for domain in domains {
            let dependencies = domain.manifest_dependencies();

            fmt.write_markup(markup! {
                "- Name: "<Emphasis>{domain}</Emphasis>
            })?;
            fmt.write_str("\n")?;

            if recommended {
                fmt.write_markup(markup! {
                    "- The rule is recommended for this domain"
                })?;
                fmt.write_str("\n")?;
            }

            if !dependencies.is_empty() {
                fmt.write_markup(markup! {
                    "- The rule is enabled when one of these dependencies are detected:"
                })?;
                fmt.write_str("\n")?;
                let padding = Padding::new(2);
                for (index, (dep, range)) in dependencies.iter().enumerate() {
                    fmt.write_markup(
                        markup! { {padding}"- "<Emphasis>{dep}"@"{range}</Emphasis> },
                    )?;
                    if index + 1 < dependencies.len() {
                        fmt.write_str("\n")?;
                    }
                }
                fmt.write_str("\n")?;
            }

            let globals = domain.globals();

            if !globals.is_empty() {
                fmt.write_markup(markup! {
                    "- The rule adds the following globals: "
                })?;
                fmt.write_str("\n")?;

                let padding = Padding::new(2);
                for (index, global) in globals.iter().enumerate() {
                    fmt.write_markup(markup! { {padding}"- "<Emphasis>{global}</Emphasis> })?;
                    if index + 1 < globals.len() {
                        fmt.write_str("\n")?;
                    }
                }
                fmt.write_str("\n")?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}
