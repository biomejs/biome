use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyTsType, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsInitializerClause, JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange};
use biome_rule_options::use_vue_define_props_destructuring::UseVueDefinePropsDestructuringOptions;

declare_lint_rule! {
    /// Require destructuring the props returned by Vue's `defineProps` macro.
    ///
    /// Since Vue 3.5, the variables you destructure from `defineProps()` stay reactive, so they
    /// update when the parent component passes new values. Destructuring lets you read each prop
    /// as a plain variable and set its default value with regular JavaScript default value syntax.
    /// This rule reports `defineProps()` calls whose result is assigned to a variable without
    /// destructuring it into separate props.
    ///
    /// Once props are destructured, `withDefaults()` is no longer needed to set default values,
    /// so this rule also reports `withDefaults()` calls whose result is destructured or not
    /// assigned to a variable.
    ///
    /// Calls to `defineProps()` without arguments are ignored, and so are calls that receive only
    /// an empty object, an empty array, or the empty type `{}`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const props = defineProps(["foo"]);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const props = withDefaults(defineProps<{ foo?: string }>(), { foo: "default" });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const { foo } = withDefaults(defineProps<{ foo?: string }>(), { foo: "default" });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// withDefaults(defineProps<{ foo?: string }>(), { foo: "default" });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const { foo } = defineProps<{ foo?: string }>();
    /// const { bar = "default" } = defineProps<{ bar?: string }>();
    /// defineProps<{ baz?: string }>();
    /// const props = defineProps();
    /// </script>
    /// ```
    ///
    pub UseVueDefinePropsDestructuring {
        version: "2.6.4",
        name: "useVueDefinePropsDestructuring",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("define-props-destructuring").same()],
    }
}

impl Rule for UseVueDefinePropsDestructuring {
    type Query = Semantic<JsCallExpression>;
    type State = UseVueDefinePropsDestructuringState;
    type Signals = Option<Self::State>;
    type Options = UseVueDefinePropsDestructuringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
        {
            return None;
        }

        let define_props = ctx.query();
        let model = ctx.model();
        if !is_vue_compiler_macro_call(define_props, model, "defineProps")
            || !declares_props(define_props)
        {
            return None;
        }

        let with_defaults = find_with_defaults_call(define_props, model);
        let assigned_value = with_defaults.as_ref().unwrap_or(define_props);
        if find_assignment_target(assigned_value)
            .is_some_and(|target| target.as_js_object_binding_pattern().is_none())
        {
            return Some(UseVueDefinePropsDestructuringState::NotDestructured);
        }

        // An unassigned `withDefaults()` call is reported too, because its defaults can be
        // written directly in a destructuring pattern instead.
        let with_defaults_callee = with_defaults?.callee().ok()?;
        Some(UseVueDefinePropsDestructuringState::WithDefaults(
            with_defaults_callee.range(),
        ))
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            UseVueDefinePropsDestructuringState::NotDestructured => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The props returned by "<Emphasis>"defineProps()"</Emphasis>" are not destructured."
                },
            )
            .note(markup! {
                "Destructured props stay reactive and can be read as plain variables."
            })
            .note(markup! {
                "Destructure the props you need, for example "<Emphasis>"const { foo } = defineProps(...)"</Emphasis>"."
            }),
            UseVueDefinePropsDestructuringState::WithDefaults(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Default values for props are set with "<Emphasis>"withDefaults()"</Emphasis>" instead of in the destructuring."
                },
            )
            .note(markup! {
                "With destructuring, each default value sits next to the variable that holds the prop, so "<Emphasis>"withDefaults()"</Emphasis>" isn't needed."
            })
            .note(markup! {
                "Remove "<Emphasis>"withDefaults()"</Emphasis>" and give each prop its default value in the destructuring, for example "<Emphasis>"const { foo = \"default\" } = defineProps(...)"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }
}

pub enum UseVueDefinePropsDestructuringState {
    /// The props are assigned to a pattern other than object destructuring.
    NotDestructured,
    /// The props are wrapped in `withDefaults()`. Holds the range of the `withDefaults` callee.
    WithDefaults(TextRange),
}

/// Returns `true` unless the call has no arguments, or only an empty object, array, or object
/// type literal.
fn declares_props(call: &JsCallExpression) -> bool {
    if let Some(type_arguments) = call.type_arguments() {
        return match type_arguments.ts_type_argument_list().iter().next() {
            Some(Ok(AnyTsType::TsObjectType(object_type))) => !object_type.members().is_empty(),
            Some(_) => true,
            None => false,
        };
    }

    let Some(Ok(argument)) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().iter().next())
    else {
        return false;
    };
    match argument
        .as_any_js_expression()
        .and_then(AnyJsExpression::inner_expression)
    {
        Some(AnyJsExpression::JsObjectExpression(object)) => !object.members().is_empty(),
        Some(AnyJsExpression::JsArrayExpression(array)) => !array.elements().is_empty(),
        _ => true,
    }
}

/// Returns the `withDefaults()` call that receives `define_props` as its first argument.
fn find_with_defaults_call(
    define_props: &JsCallExpression,
    model: &SemanticModel,
) -> Option<JsCallExpression> {
    let outer = AnyJsExpression::from(define_props.clone()).outer_expression()?;
    let argument_list = outer.parent::<JsCallArgumentList>()?;
    let first_argument = argument_list.iter().next()?.ok()?;
    if first_argument.syntax() != outer.syntax() {
        return None;
    }
    let call = argument_list
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;
    is_vue_compiler_macro_call(&call, model, "withDefaults").then_some(call)
}

/// Returns the pattern on the left side of `const <pattern> = value`.
fn find_assignment_target(value: &JsCallExpression) -> Option<AnyJsBindingPattern> {
    AnyJsExpression::from(value.clone())
        .outer_expression()?
        .parent::<JsInitializerClause>()?
        .parent::<JsVariableDeclarator>()?
        .id()
        .ok()
}
