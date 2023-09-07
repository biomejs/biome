use crate::semantic_services::SemanticServices;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use bpaf::Bpaf;
use rome_console::markup;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_js_semantic::{Binding, BindingExtensions};
use rome_js_syntax::{
    JsIdentifierAssignment, JsReferenceIdentifier, JsxReferenceIdentifier, TextRange,
};
use rome_json_syntax::JsonLanguage;
use rome_rowan::{declare_node_union, AstNode, SyntaxNode};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

declare_rule! {
    /// This rule allows you to specify global variable names that you don’t want to use in your application.
    ///
    /// > Disallowing usage of specific global variables can be useful if you want to allow a set of
    /// global variables by enabling an environment, but still want to disallow some of those.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log(event)
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// function f(event) {
    ///     console.log(event)
    /// }
    /// ```
    /// ## Options
    ///
    /// Use the options to specify additional globals that you want to restrict in your
    /// source code.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "deniedGlobals": ["$", "MooTools"]
    ///     }
    /// }
    /// ```
    ///
    /// In the example above, the rule will emit a diagnostics if tried to use `$` or `MooTools` without
    /// creating a local variable.
    ///
    pub(crate) NoRestrictedGlobals {
        version: "1.0.0",
        name: "noRestrictedGlobals",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) AnyIdentifier = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

const RESTRICTED_GLOBALS: [&str; 2] = ["event", "error"];

/// Options for the rule `noRestrictedGlobals`.
#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedGlobalsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide, argument::<String>("NUM"), many, optional)]
    denied_globals: Option<Vec<String>>,
}

impl RestrictedGlobalsOptions {
    pub const KNOWN_KEYS: &'static [&'static str] = &["deniedGlobals"];
}

// Required by [Bpaf].
impl FromStr for RestrictedGlobalsOptions {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // WARNING: should not be used.
        Ok(Self::default())
    }
}

impl VisitJsonNode for RestrictedGlobalsOptions {}
impl VisitNode<JsonLanguage> for RestrictedGlobalsOptions {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Self::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "deniedGlobals" {
            self.denied_globals = self.map_to_array_of_strings(&value, name_text, diagnostics);
        }

        Some(())
    }
}

impl Rule for NoRestrictedGlobals {
    type Query = SemanticServices;
    type State = (TextRange, String);
    type Signals = Vec<Self::State>;
    type Options = RestrictedGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let options = ctx.options();

        let unresolved_reference_nodes = model
            .all_unresolved_references()
            .map(|reference| reference.syntax().clone());
        let global_references_nodes = model
            .all_global_references()
            .map(|reference| reference.syntax().clone());

        unresolved_reference_nodes
            .chain(global_references_nodes)
            .filter_map(|node| {
                let node = AnyIdentifier::unwrap_cast(node);
                let (token, binding) = match node {
                    AnyIdentifier::JsReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyIdentifier::JsxReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyIdentifier::JsIdentifierAssignment(node) => {
                        (node.name_token(), node.binding(model))
                    }
                };
                let token = token.ok()?;
                let text = token.text_trimmed();
                let denied_globals = if let Some(denied_globals) = options.denied_globals.as_ref() {
                    denied_globals.iter().map(AsRef::as_ref).collect::<Vec<_>>()
                } else {
                    vec![]
                };
                is_restricted(text, binding, denied_globals.as_slice())
                    .map(|text| (token.text_trimmed_range(), text))
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (span, text): &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *span,
                markup! {
                    "Do not use the global variable "<Emphasis>{text}</Emphasis>"."
                },
            )
            .note(markup! {
                "Use a local variable instead."
            }),
        )
    }
}

fn is_restricted(name: &str, binding: Option<Binding>, denied_globals: &[&str]) -> Option<String> {
    if binding.is_none() && (RESTRICTED_GLOBALS.contains(&name) || denied_globals.contains(&name)) {
        Some(name.to_string())
    } else {
        None
    }
}
