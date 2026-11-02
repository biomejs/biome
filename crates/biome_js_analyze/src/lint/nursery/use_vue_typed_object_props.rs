use crate::services::vue::VueProp;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsRoot, JsObjectExpression, JsObjectMemberList,
    JsPropertyObjectMember, JsSyntaxKind,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, NodeOrToken, TextRange};
use biome_rule_options::use_vue_typed_object_props::UseVueTypedObjectPropsOptions;
use biome_vue_semantic::PropForm;

declare_lint_rule! {
    /// Require a TypeScript type for Vue props declared with `Object` or `Array`.
    ///
    /// When a prop is declared with `Object` or `Array` as its type, TypeScript only knows that
    /// the prop is an object or an array. It doesn't know which properties the object has or
    /// what the array holds, so it can't catch mistakes when the prop is passed or used.
    ///
    /// Add the full type with `as PropType<...>`, where `PropType` is imported from `vue`.
    ///
    /// This rule checks props declared through the `props` option and `defineProps()`. Only
    /// TypeScript files and Vue files with `<script lang="ts">` are checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineProps({
    ///   user: Object,
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script lang="ts">
    /// export default {
    ///   props: {
    ///     items: { type: Array, required: true },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// import type { PropType } from "vue";
    ///
    /// interface User {
    ///   name: string;
    /// }
    ///
    /// defineProps({
    ///   user: Object as PropType<User>,
    ///   items: { type: Array as PropType<string[]>, required: true },
    ///   label: [String, Number],
    /// });
    /// </script>
    /// ```
    ///
    pub UseVueTypedObjectProps {
        version: "next",
        name: "useVueTypedObjectProps",
        language: "ts",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-typed-object-prop").same()],
    }
}

impl Rule for UseVueTypedObjectProps {
    type Query = VueProp;
    /// The range of an `Object` or `Array` written as the prop's type without a TypeScript
    /// type, and which of the two it is.
    type State = (TextRange, UntypedConstructor);
    type Signals = Box<[Self::State]>;
    type Options = UseVueTypedObjectPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().language().is_typescript() {
            return Box::default();
        }
        let prop = ctx.query();
        let Some(runtime_type) = prop
            .runtime_type_range()
            .and_then(|range| expression_at(&ctx.root(), range))
        else {
            return Box::default();
        };
        if prop.form() == Some(PropForm::Options) && is_in_wrapped_options(&runtime_type) {
            return Box::default();
        }
        let model = ctx.model();
        match runtime_type.omit_parentheses() {
            // `user: { type: [Object, String] }`
            AnyJsExpression::JsArrayExpression(array) => array
                .elements()
                .iter()
                .filter_map(|element| match element.ok()? {
                    AnyJsArrayElement::AnyJsExpression(expression) => {
                        untyped_constructor(expression, model)
                    }
                    _ => None,
                })
                .collect(),
            // `user: Object` or `user: { type: Object }`
            expression => untyped_constructor(expression, model).into_iter().collect(),
        }
    }

    fn text_range(_ctx: &RuleContext<Self>, (range, _): &Self::State) -> Option<TextRange> {
        Some(*range)
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (range, constructor): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This prop is declared with "<Emphasis>{constructor}</Emphasis>" but has no TypeScript type."
                },
            )
            .note(match constructor {
                UntypedConstructor::Object => markup! {
                    "TypeScript only knows that the prop is an object, not which properties it has, so it can't catch mistakes when the prop is passed or used."
                },
                UntypedConstructor::Array => markup! {
                    "TypeScript only knows that the prop is an array, not what it holds, so it can't catch mistakes when the prop is passed or used."
                },
            })
            .note(match constructor {
                UntypedConstructor::Object => markup! {
                    "Add the type with "<Emphasis>"PropType"</Emphasis>" from "<Emphasis>"vue"</Emphasis>", for example "<Emphasis>"Object as PropType<User>"</Emphasis>"."
                },
                UntypedConstructor::Array => markup! {
                    "Add the type with "<Emphasis>"PropType"</Emphasis>" from "<Emphasis>"vue"</Emphasis>", for example "<Emphasis>"Array as PropType<string[]>"</Emphasis>"."
                },
            }),
        )
    }
}

/// A built-in constructor that gives a prop a runtime type but no TypeScript type for its
/// contents.
#[derive(Clone, Copy)]
pub enum UntypedConstructor {
    Object,
    Array,
}

impl biome_console::fmt::Display for UntypedConstructor {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Object => "Object",
            Self::Array => "Array",
        })
    }
}

/// Returns the expression in `root` whose range is exactly `range`.
fn expression_at(root: &AnyJsRoot, range: TextRange) -> Option<AnyJsExpression> {
    let start = match root.syntax().covering_element(range) {
        NodeOrToken::Node(node) => node,
        NodeOrToken::Token(token) => token.parent()?,
    };
    start
        .ancestors()
        .take_while(|node| node.text_trimmed_range() == range)
        .filter_map(AnyJsExpression::cast)
        .last()
}

/// Returns `true` when `runtime_type` is the `type` member of a prop options object that is
/// wrapped in something other than parentheses, as in
/// `user: { type: Object } as PropOptions<User>`. A type assertion around the options object
/// already gives the prop its type.
fn is_in_wrapped_options(runtime_type: &AnyJsExpression) -> bool {
    runtime_type
        .parent::<JsPropertyObjectMember>()
        .and_then(|member| member.parent::<JsObjectMemberList>())
        .and_then(|members| members.parent::<JsObjectExpression>())
        .and_then(|options| {
            options
                .syntax()
                .ancestors()
                .skip(1)
                .find(|node| node.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
        })
        .is_some_and(|parent| !JsPropertyObjectMember::can_cast(parent.kind()))
}

/// Returns the range of `expression` and which constructor it is, when it is a reference to
/// the global `Object` or `Array`, without `as` or another TypeScript type around it.
fn untyped_constructor(
    expression: AnyJsExpression,
    model: &SemanticModel,
) -> Option<(TextRange, UntypedConstructor)> {
    let range = expression.range();
    let reference = expression.omit_parentheses().as_js_reference_identifier()?;
    if model.binding(&reference).is_some() {
        return None;
    }
    let constructor = match reference.value_token().ok()?.text_trimmed() {
        "Object" => UntypedConstructor::Object,
        "Array" => UntypedConstructor::Array,
        _ => return None,
    };
    Some((range, constructor))
}
