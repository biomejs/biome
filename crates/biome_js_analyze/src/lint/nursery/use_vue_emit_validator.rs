use crate::JsRuleAction;
use crate::services::vue::VueEmit;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember,
    AnyJsRoot, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::use_vue_emit_validator::UseVueEmitValidatorOptions;
use biome_vue_semantic::EmitForm;

declare_lint_rule! {
    /// Require a validator function for each event declared in the `emits` option or in `defineEmits()`.
    ///
    /// A component can declare the events it sends to its parent with the `emits` option, or with
    /// `defineEmits()` in `<script setup>`. When the events are declared as an object, each event
    /// can have a validator: a function that receives the values passed with the event, and returns
    /// `true` when they are valid. Vue warns during development when a validator returns `false`.
    ///
    /// A validator also documents which values the event sends. In TypeScript, the parameter
    /// types of the validator become the types that the `emit` function accepts for that event.
    ///
    /// This rule reports events that are declared without a validator, either because they are
    /// listed in an array of names, or because their value in the object is `null` or another
    /// value that isn't a function. A variable used as the value is accepted, because it may hold
    /// a function declared elsewhere.
    ///
    /// Events declared with a type, as in `defineEmits<{ change: [id: number] }>()`, already
    /// describe their values and are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   emits: ["change"],
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineEmits({
    ///   change: null,
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { isNumber } from "./validators";
    ///
    /// export default {
    ///   emits: {
    ///     change: (id) => typeof id === "number",
    ///     close: () => true,
    ///     select: isNumber,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineEmits<{ change: [id: number] }>();
    /// </script>
    /// ```
    ///
    pub UseVueEmitValidator {
        version: "next",
        name: "useVueEmitValidator",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-emit-validator").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueEmitValidator {
    type Query = VueEmit;
    type State = MissingValidator;
    type Signals = Option<Self::State>;
    type Options = UseVueEmitValidatorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let emit = ctx.query();
        match emit.form()? {
            EmitForm::ArrayString => Some(MissingValidator::NameOnly),
            EmitForm::Object => {
                if emit.validator_range().is_some() {
                    return None;
                }
                let member = object_member_at(&ctx.root(), emit.declaration_range())?;
                // A shorthand member, as in `{ change }`, refers to a variable that may hold a
                // validator declared elsewhere.
                let AnyJsObjectMember::JsPropertyObjectMember(property) = member else {
                    return None;
                };
                let value = property.value().ok()?;
                let inner = value.inner_expression()?;
                if inner
                    .as_static_value()
                    .is_some_and(|value| value.is_null_or_undefined())
                {
                    Some(MissingValidator::Skipped(value))
                } else if matches!(inner, AnyJsExpression::JsIdentifierExpression(_)) {
                    None
                } else {
                    Some(MissingValidator::NotFunction)
                }
            }
            EmitForm::CallSignature | EmitForm::TypeMember => None,
        }
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().declaration_range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let emit = ctx.query();
        let name = emit.name();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            emit.declaration_range(),
            markup! {
                "The event "<Emphasis>{name}</Emphasis>" is declared "{state}"."
            },
        )
        .note(markup! {
            "A validator function checks the values passed with the event, and documents what the event sends. In TypeScript, its parameter types also become the types the "<Emphasis>"emit"</Emphasis>" function accepts for this event."
        });
        Some(match state {
            MissingValidator::NameOnly => diagnostic.note(markup! {
                "Declare the events in an object instead of an array, and give this event a function that receives its values and returns "<Emphasis>"true"</Emphasis>" when they are valid. If the event sends no values, use "<Emphasis>"() => true"</Emphasis>"."
            }),
            MissingValidator::Skipped(_) | MissingValidator::NotFunction => diagnostic.note(markup! {
                "Replace the value with a function that receives the event's values and returns "<Emphasis>"true"</Emphasis>" when they are valid. If the event sends no values, use "<Emphasis>"() => true"</Emphasis>"."
            }),
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let MissingValidator::Skipped(value) = state else {
            return None;
        };
        let validator = make::js_arrow_function_expression(
            AnyJsArrowFunctionParameters::JsParameters(make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list([], []),
                make::token(T![')']).with_trailing_space(),
            )),
            make::token(T![=>]).with_trailing_space(),
            AnyJsFunctionBody::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
                make::js_boolean_literal_expression(make::token(T![true])).into(),
            )),
        )
        .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(value.clone(), validator.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace it with a validator function that accepts no values." }.to_owned(),
            mutation,
        ))
    }
}

/// Why an event declaration has no validator function.
pub enum MissingValidator {
    /// The event is listed in an array of names, as in `emits: ['change']`.
    NameOnly,
    /// The event's value is `null` or `undefined`, which tells Vue to skip validation. Holds the
    /// value, which the fix replaces.
    Skipped(AnyJsExpression),
    /// The event's value is neither a function nor a variable, as in `{ change: {} }`.
    NotFunction,
}

impl biome_console::fmt::Display for MissingValidator {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Skipped(_) => "with a value that turns off validation",
            Self::NameOnly | Self::NotFunction => "without a validator function",
        })
    }
}

/// Returns the object member that spans `range` in `root`.
fn object_member_at(root: &AnyJsRoot, range: TextRange) -> Option<AnyJsObjectMember> {
    root.syntax()
        .covering_element(range)
        .ancestors()
        .find_map(AnyJsObjectMember::cast)
}
