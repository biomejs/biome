use crate::JsRuleAction;
use ::serde::{Deserialize, Serialize};
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_js_factory::make;
use biome_js_syntax::TsReferenceType;
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use biome_unicode_table::is_js_ident;
use rustc_hash::FxHashMap;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Disallow user defined types.
    ///
    /// This rule allows you to specify type names that you don’t want to use in your application.
    ///
    /// To prevent use of commonly misleading types, you can refer to [noBannedTypes](https://biomejs.dev/linter/rules/no-banned-types/)
    ///
    /// ## Options
    ///
    /// Use the options to specify additional types that you want to restrict in your
    /// source code.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "types": {
    ///            "Foo": {
    ///               "message": "Only bar is allowed",
    ///               "use": "bar"
    ///             },
    ///             "OldAPI": "Use NewAPI instead"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// In the example above, the rule will emit a diagnostics if `Foo` or `OldAPI` are used.
    ///
    pub NoRestrictedTypes {
        version: "1.9.0",
        name: "noRestrictedTypes",
        language: "ts",
        sources: &[
            RuleSource::EslintTypeScript("no-restricted-types"),
        ],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoRestrictedTypes {
    type Query = Ast<TsReferenceType>;
    type State = CustomRestrictedTypeOptions;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedTypesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ts_reference_type = ctx.query();
        let options = ctx.options();

        let ts_any_name = ts_reference_type.name().ok()?;
        let identifier = ts_any_name.as_js_reference_identifier()?;
        let identifier_token = identifier.value_token().ok()?;
        let token_name = identifier_token.text_trimmed();

        let restricted_type = options.types.get(token_name)?.clone();

        Some(restricted_type.into())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! { {state.message} }.to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let suggested_type = state.use_instead.as_ref()?;
        if !is_js_ident(suggested_type) {
            return None;
        }

        let mut mutation = ctx.root().begin();

        let ts_reference_type = ctx.query();
        let ts_any_name = ts_reference_type.name().ok()?;
        let identifier = ts_any_name.as_js_reference_identifier()?;
        let prev_token = identifier.value_token().ok()?;

        let new_token = make::ident(suggested_type);

        mutation.replace_element(prev_token.into(), new_token.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use '"{suggested_type}"' instead" }.to_owned(),
            mutation,
        ))
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
pub struct NoRestrictedTypesOptions {
    types: FxHashMap<Box<str>, CustomRestrictedType>,
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
pub struct CustomRestrictedTypeOptions {
    message: String,
    #[serde(rename = "use")]
    use_instead: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedType {
    Plain(String),
    WithOptions(CustomRestrictedTypeOptions),
}

impl From<CustomRestrictedType> for CustomRestrictedTypeOptions {
    fn from(options: CustomRestrictedType) -> Self {
        match options {
            CustomRestrictedType::Plain(message) => CustomRestrictedTypeOptions {
                message,
                use_instead: None,
            },
            CustomRestrictedType::WithOptions(options) => options,
        }
    }
}

impl Deserializable for CustomRestrictedType {
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
