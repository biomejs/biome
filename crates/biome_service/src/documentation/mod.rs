use biome_analyze::{
    FixKind, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleDomain, RuleGroup,
    RuleMetadata,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Padding, markup};
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

impl ExplainRule {
    pub fn name(&self) -> &'static str {
        self.metadata.name
    }

    pub fn group(&self) -> &'static str {
        self.group
    }

    /// Returns the first line of the rule's documentation.
    pub fn summary(&self) -> &'static str {
        self.metadata.docs.lines().next().unwrap_or_default().trim()
    }

    pub fn domains(&self) -> &'static [RuleDomain] {
        self.metadata.domains
    }

    /// Returns whether this is an assist action rather than a lint rule.
    pub fn is_action(&self) -> bool {
        self.category == RuleCategory::Action.as_suppression_category()
    }
}

/// Collects the lint rules and assist actions of every language, keyed by
/// rule name. Each rule name belongs to exactly one group, so rules that
/// several languages implement share one entry.
pub struct RulesVisitor {
    rules_metadata: BTreeMap<&'static str, ExplainRule>,
}

impl Default for RulesVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl RulesVisitor {
    pub fn new() -> Self {
        let mut visitor = Self {
            rules_metadata: BTreeMap::new(),
        };

        #[cfg(feature = "lang_graphql")]
        biome_graphql_analyze::visit_registry(&mut visitor);
        #[cfg(feature = "lang_html")]
        biome_html_analyze::visit_registry(&mut visitor);
        #[cfg(feature = "lang_css")]
        biome_css_analyze::visit_registry(&mut visitor);
        biome_json_analyze::visit_registry(&mut visitor);
        #[cfg(feature = "lang_js")]
        biome_js_analyze::visit_registry(&mut visitor);
        #[cfg(feature = "lang_md")]
        biome_markdown_analyze::visit_registry(&mut visitor);
        #[cfg(feature = "lang_yaml")]
        biome_yaml_analyze::visit_registry(&mut visitor);

        visitor
    }

    /// Returns the collected rules, sorted by name.
    pub fn rules(&self) -> impl Iterator<Item = &ExplainRule> {
        self.rules_metadata.values()
    }

    fn get_metadata(&mut self, name: &str) -> Option<ExplainRule> {
        self.rules_metadata.remove(name)
    }
}

impl<L: Language> RegistryVisitor<L> for RulesVisitor {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let category = <R::Group as RuleGroup>::Category::CATEGORY;
        if matches!(category, RuleCategory::Lint | RuleCategory::Action) {
            let explain_rule = ExplainRule {
                metadata: R::METADATA,
                group: <R::Group as RuleGroup>::NAME,
                category: category.as_suppression_category(),
            };
            self.rules_metadata.insert(R::METADATA.name, explain_rule);
        }
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
