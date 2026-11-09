use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_yaml_empty_sequence_entries::NoYamlEmptySequenceEntriesOptions;
use biome_yaml_syntax::{AnyYamlBlockNode, AnyYamlFlowNode, YamlBlockSequenceEntry};

declare_lint_rule! {
    /// Disallow empty items in YAML lists.
    ///
    /// In YAML, each item of a list written on separate lines starts with a `-`.
    /// A `-` with nothing after it adds an item with no value, which YAML reads as `null`.
    /// This usually happens by mistake, when a value is forgotten or a `-` is left behind.
    ///
    /// An item that has only a tag (such as `!!str`) or an anchor (such as `&name`), and no value, is also empty.
    ///
    /// If the item is meant to have no value, write `null` or `~` to make that clear.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    /// fruits:
    ///   - apple
    ///   -
    ///   - banana
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// - &item
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// fruits:
    ///   - apple
    ///   - banana
    ///   - null
    ///   - ~
    /// empty: []
    /// ```
    ///
    pub NoYamlEmptySequenceEntries {
        version: "next",
        name: "noYamlEmptySequenceEntries",
        language: "yaml",
        sources: &[RuleSource::EslintYml("no-empty-sequence-entry").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoYamlEmptySequenceEntries {
    type Query = Ast<YamlBlockSequenceEntry>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoYamlEmptySequenceEntriesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        is_empty(ctx.query().value()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let dash = ctx.query().minus_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                dash.text_trimmed_range(),
                markup! {
                    "This list item is empty."
                },
            )
            .note(markup! {
                "An item with no value is read as "<Emphasis>"null"</Emphasis>", and is often left by mistake."
            })
            .note(markup! {
                "Add a value to the item, or remove the item. If the item is meant to have no value, write "<Emphasis>"null"</Emphasis>" instead."
            }),
        )
    }
}

/// Whether the value of a list item has no content. A value that has only a
/// tag or an anchor has no content.
fn is_empty(value: Option<AnyYamlBlockNode>) -> bool {
    match value {
        None => true,
        // The parser keeps a tag or anchor without content as a flow node
        // with no content.
        Some(AnyYamlBlockNode::YamlFlowInBlockNode(node)) => match node.flow() {
            Ok(AnyYamlFlowNode::YamlFlowYamlNode(node)) => node.content().is_none(),
            _ => false,
        },
        Some(_) => false,
    }
}
