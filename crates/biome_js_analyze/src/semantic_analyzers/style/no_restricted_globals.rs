use crate::semantic_services::SemanticServices;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_js_semantic::{Binding, BindingExtensions};
use biome_js_syntax::{AnyJsIdentifierUsage, TextRange};
use biome_rowan::{AstNode, TokenText};
use bpaf::Bpaf;
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

const RESTRICTED_GLOBALS: [&str; 2] = ["event", "error"];

/// Options for the rule `noRestrictedGlobals`.
#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedGlobalsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide, argument::<String>("NUM"), many, optional)]
    denied_globals: Option<Vec<String>>,
}

// Required by [Bpaf].
impl FromStr for RestrictedGlobalsOptions {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // WARNING: should not be used.
        Ok(Self::default())
    }
}

impl Deserializable for RestrictedGlobalsOptions {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(RestrictedGlobalsOptionsVisitor, diagnostics)
    }
}

struct RestrictedGlobalsOptionsVisitor;
impl DeserializationVisitor for RestrictedGlobalsOptionsVisitor {
    type Output = RestrictedGlobalsOptions;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["deniedGlobals"];
        let mut denied_globals = None;
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "deniedGlobals" => {
                    denied_globals = Deserializable::deserialize(value, diagnostics);
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key.text(),
                    key_range,
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(Self::Output { denied_globals })
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
                let node = AnyJsIdentifierUsage::unwrap_cast(node);
                let (token, binding) = match node {
                    AnyJsIdentifierUsage::JsReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyJsIdentifierUsage::JsxReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyJsIdentifierUsage::JsIdentifierAssignment(node) => {
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
                is_restricted(text, &binding, denied_globals.as_slice())
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

fn is_restricted(name: &str, binding: &Option<Binding>, denied_globals: &[&str]) -> Option<String> {
    if binding.is_none() && (RESTRICTED_GLOBALS.contains(&name) || denied_globals.contains(&name)) {
        Some(name.to_string())
    } else {
        None
    }
}
