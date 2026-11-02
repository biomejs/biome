use crate::JsRuleAction;
use crate::services::vue::VueProp;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMember,
    JsPropertyObjectMember, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::use_vue_prop_type_constructor::UseVuePropTypeConstructorOptions;
use biome_unicode_table::is_js_ident;
use biome_vue_semantic::PropForm;

declare_lint_rule! {
    /// Require the type of a Vue prop to be a constructor.
    ///
    /// Vue checks the value passed to a prop against the type the prop declares. That type must be
    /// a constructor, such as `String`, `Number` or a class, or an array of constructors. A common
    /// mistake is to write the name of the type as a string, as in `type: 'String'`. Vue can't
    /// check values against a string, so the prop loses the checks and the behavior of its type.
    /// For example, a prop declared with `type: 'Boolean'` isn't set to `true` when the attribute
    /// is written without a value.
    ///
    /// This rule reports prop types written as a string, a template string, a number, a boolean, a
    /// regular expression, a binary operation such as `Foo + ''`, or an increment or decrement such
    /// as `i++`. `null` is allowed, because Vue treats it as "any type". It checks props declared
    /// through the `props` option and through `defineProps()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     size: 'Number',
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     value: ['Number', String],
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({
    ///   options: {
    ///     type: 'Object',
    ///     default: () => ({}),
    ///   },
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: {
    ///     size: Number,
    ///     value: [Number, String],
    ///     options: {
    ///       type: Object,
    ///       default: () => ({}),
    ///     },
    ///     anything: null,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub UseVuePropTypeConstructor {
        version: "2.6.5",
        name: "useVuePropTypeConstructor",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-prop-type-constructor").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVuePropTypeConstructor {
    type Query = VueProp;
    /// A prop type, or an element of an array of prop types, that is not a constructor.
    type State = AnyJsExpression;
    type Signals = Box<[Self::State]>;
    type Options = UseVuePropTypeConstructorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        if !matches!(prop.form(), Some(PropForm::Constructor | PropForm::Options)) {
            return Box::default();
        }
        let Some(types) = prop_types(ctx) else {
            return Box::default();
        };
        match types {
            AnyJsExpression::JsArrayExpression(array) => array
                .elements()
                .iter()
                .filter_map(|element| match element.ok()? {
                    AnyJsArrayElement::AnyJsExpression(expression) => {
                        expression.inner_expression()
                    }
                    _ => None,
                })
                .filter(is_not_constructor)
                .collect(),
            types if is_not_constructor(&types) => Box::new([types]),
            _ => Box::default(),
        }
    }

    fn text_range(_ctx: &RuleContext<Self>, types: &Self::State) -> Option<TextRange> {
        Some(types.range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, types: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                types.range(),
                markup! {
                    "The type of the prop "<Emphasis>{ctx.query().name()}</Emphasis>" is not a constructor."
                },
            )
            .note(markup! {
                "Vue uses the type of a prop to check the values passed to it. It can only do that with a constructor, such as "<Emphasis>"String"</Emphasis>" or "<Emphasis>"Number"</Emphasis>", or with an array of constructors."
            })
            .note(markup! {
                "Replace this value with the constructor of the type the prop accepts."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, types: &Self::State) -> Option<JsRuleAction> {
        let static_value = types.as_static_value()?;
        let name = static_value.as_string_constant()?;
        // A reserved word such as `null` or `class` would turn into a keyword or a syntax error.
        // `await` is reserved too, because Vue scripts are modules.
        if !is_js_ident(name)
            || JsSyntaxKind::from_keyword(name).is_some_and(|keyword| {
                keyword.is_non_contextual_keyword() || keyword == T![await]
            })
        {
            return None;
        }
        let constructor = AnyJsExpression::JsIdentifierExpression(make::js_identifier_expression(
            make::js_reference_identifier(make::ident(name)),
        ));
        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(types.clone(), constructor)?;
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the constructor "<Emphasis>{name}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the expression that declares the types of the queried prop: the value of the prop in
/// `size: Number`, or the value of its `type` option in `size: { type: Number }`.
fn prop_types(ctx: &RuleContext<UseVuePropTypeConstructor>) -> Option<AnyJsExpression> {
    let declaration = ctx
        .root()
        .syntax()
        .covering_element(ctx.query().declaration_range())
        .into_node()?;
    let value = JsPropertyObjectMember::cast(declaration)?
        .value()
        .ok()?
        .inner_expression()?;
    let AnyJsExpression::JsObjectExpression(options) = value else {
        return Some(value);
    };
    options.members().iter().find_map(|member| {
        let AnyJsObjectMember::JsPropertyObjectMember(member) = member.ok()? else {
            return None;
        };
        if member.name().ok()?.name()?.text() != "type" {
            return None;
        }
        member.value().ok()?.inner_expression()
    })
}

/// Returns `true` for expressions that can never evaluate to a constructor, such as a string or
/// a number. `null` is excluded, because Vue accepts it as a prop type that allows any value.
fn is_not_constructor(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            !matches!(literal, AnyJsLiteralExpression::JsNullLiteralExpression(_))
        }
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        AnyJsExpression::JsBinaryExpression(_)
        | AnyJsExpression::JsInExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_) => true,
        _ => false,
    }
}
