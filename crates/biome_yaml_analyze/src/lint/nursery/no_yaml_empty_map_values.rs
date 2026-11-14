use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_yaml_empty_map_values::NoYamlEmptyMapValuesOptions;
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlFlowNode, YamlBlockMapExplicitEntry, YamlBlockMapImplicitEntry,
    YamlFlowMapExplicitEntry, YamlFlowMapImplicitEntry,
};

declare_lint_rule! {
    /// Disallow keys without a value in YAML maps.
    ///
    /// When a key in a map has nothing after it, YAML reads its value as `null`.
    /// This is usually a mistake, such as a value that was forgotten or deleted by accident.
    ///
    /// This rule also reports values that only have a tag (such as `!!str`) or an anchor (such as `&name`),
    /// because these values are empty too.
    ///
    /// If the value is meant to be empty, write it out, for example as `null`, so the intent is clear.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    /// name:
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// person: { name: }
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// ? name
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// name: !!str
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// name: John Doe
    /// nickname: null
    /// title: ""
    /// person: { name: John Doe }
    /// ```
    ///
    pub NoYamlEmptyMapValues {
        version: "2.6.8",
        name: "noYamlEmptyMapValues",
        language: "yaml",
        sources: &[RuleSource::EslintYml("no-empty-mapping-value").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoYamlEmptyMapValues {
    type Query = Ast<AnyYamlMapEntry>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoYamlEmptyMapValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().has_empty_value().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This key has no value."
                },
            )
            .note(markup! {
                "An empty value is easy to miss, and is often a value that was forgotten."
            })
            .note(markup! {
                "Add a value. If the value is meant to be empty, write it out, for example as "<Emphasis>"null"</Emphasis>"."
            }),
        )
    }
}

declare_node_union! {
    pub AnyYamlMapEntry =
        YamlBlockMapImplicitEntry
        | YamlBlockMapExplicitEntry
        | YamlFlowMapImplicitEntry
        | YamlFlowMapExplicitEntry
}

impl AnyYamlMapEntry {
    /// Returns `true` when the entry has no value, or a value that only has a tag or an anchor.
    fn has_empty_value(&self) -> bool {
        match self {
            Self::YamlBlockMapImplicitEntry(entry) => is_empty_block_value(entry.value()),
            Self::YamlBlockMapExplicitEntry(entry) => is_empty_block_value(entry.value()),
            Self::YamlFlowMapImplicitEntry(entry) => is_empty_flow_value(entry.value()),
            Self::YamlFlowMapExplicitEntry(entry) => is_empty_flow_value(entry.value()),
        }
    }
}

fn is_empty_block_value(value: Option<AnyYamlBlockNode>) -> bool {
    match value {
        None => true,
        Some(AnyYamlBlockNode::YamlFlowInBlockNode(node)) => {
            node.flow().is_ok_and(|flow| has_no_content(&flow))
        }
        // A value made only of properties and followed by more lines, such as `key: !!str`, is
        // parsed as a block node whose content is missing.
        Some(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => node.content().is_err(),
        Some(AnyYamlBlockNode::YamlBogusBlockNode(_)) => false,
    }
}

fn is_empty_flow_value(value: Option<AnyYamlFlowNode>) -> bool {
    value.is_none_or(|flow| has_no_content(&flow))
}

/// Returns `true` for a plain node that only has properties, such as `!!str` or `&anchor`.
fn has_no_content(node: &AnyYamlFlowNode) -> bool {
    node.as_yaml_flow_yaml_node()
        .is_some_and(|node| node.content().is_none())
}
