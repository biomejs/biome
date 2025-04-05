use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Disallow user JSX elements.
    ///
    /// This rule disallows the use of certain custom JSX elements as specified in the rule options.
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "elements": {
    ///             "button": "button is not allowed, use Button component instead"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx
    /// // {elements: {input: "input is not allowed, use TextField component instead"}}
    /// <input />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// // {elements: {input: "input is not allowed, use TextField component instead"}}
    /// <TextField />
    /// ```
    pub NoRestrictedElements {
        version: "next",
        name: "noRestrictedElements",
        language: "jsx",
        sources: &[
            RuleSource::EslintReact("forbid-elements"),
        ],
        recommended: true,
    }
}

impl Rule for NoRestrictedElements {
    type Query = Ast<AnyJsxElement>;
    type State = CustomRestrictedElementOptions;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        if options.elements.is_empty() {
            return None;
        }
        let node = ctx.query();
        let element_name = node.name().ok()?.to_trimmed_string();
        let restricted_element = options.elements.get(element_name.as_str())?.clone();
        Some(restricted_element.into())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! { {state.message} }.to_owned(),
        ))
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    biome_deserialize_macros::Deserializable,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct CustomRestrictedElementOptions {
    message: String,
    // We could add "use_instead" here later.
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedElement {
    Plain(String),
    WithOptions(CustomRestrictedElementOptions),
}

impl From<CustomRestrictedElement> for CustomRestrictedElementOptions {
    fn from(options: CustomRestrictedElement) -> Self {
        match options {
            CustomRestrictedElement::Plain(message) => CustomRestrictedElementOptions { message },
            CustomRestrictedElement::WithOptions(options) => options,
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    biome_deserialize_macros::Deserializable,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedElementsOptions {
    elements: FxHashMap<Box<str>, CustomRestrictedElement>,
}

impl Deserializable for CustomRestrictedElement {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}
