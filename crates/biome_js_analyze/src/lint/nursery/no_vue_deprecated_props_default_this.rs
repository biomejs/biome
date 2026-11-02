use crate::services::vue::VueProp;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsExpression, AnyJsObjectMember, JsFunctionDeclaration,
    JsFunctionExpression, JsLanguage, JsMethodObjectMember, JsSyntaxNode, JsThisExpression,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::no_vue_deprecated_props_default_this::NoVueDeprecatedPropsDefaultThisOptions;
use biome_vue_semantic::{DefaultSource, PropType};

declare_lint_rule! {
    /// Disallow `this` in the default value function of a Vue prop.
    ///
    /// When a prop expects an object or an array, its default value is written as a function
    /// that returns that value. Vue doesn't give this function access to the component, so `this`
    /// doesn't point to the component inside it. Reading another prop with `this.name` fails.
    /// The function receives the props passed to the component as its first argument, so read
    /// other props from that argument instead.
    ///
    /// Props whose `type` is `Function` aren't checked, because Vue uses their default value as
    /// the function itself instead of calling it.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/props-default-this.html)
    /// for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     size: Number,
    ///     sizes: {
    ///       type: Array,
    ///       default() {
    ///         return [this.size];
    ///       },
    ///     },
    ///   },
    /// };
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
    ///     sizes: {
    ///       type: Array,
    ///       default(props) {
    ///         return [props.size];
    ///       },
    ///     },
    ///     onSelect: {
    ///       type: Function,
    ///       default() {
    ///         console.log(this.size);
    ///       },
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueDeprecatedPropsDefaultThis {
        version: "2.6.5",
        name: "noVueDeprecatedPropsDefaultThis",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-props-default-this").same()],
    }
}

impl Rule for NoVueDeprecatedPropsDefaultThis {
    type Query = VueProp;
    /// The range of a `this` expression that belongs to a prop's default value function.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDeprecatedPropsDefaultThisOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        if prop.types() == PropType::Function {
            return Box::default();
        }
        let root = ctx.root();
        let mut signals = Vec::new();
        for default in prop
            .defaults()
            .filter(|default| default.source() == DefaultSource::Inline)
        {
            let Some(node) = root.syntax().covering_element(default.range()).into_node() else {
                continue;
            };
            // Arrow functions are not matched: their `this` is the one of the
            // surrounding code, which Vue doesn't change.
            let function = match AnyJsExpression::cast_ref(&node) {
                Some(expression) => expression
                    .inner_expression()
                    .and_then(|expression| AnyPropDefaultFunction::cast(expression.into_syntax())),
                None => AnyPropDefaultFunction::cast(node),
            };
            let Some(function) = function else {
                continue;
            };
            collect_this_ranges(&function, &mut signals);
        }
        signals.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<TextRange> {
        Some(*range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>"this"</Emphasis>" is used in the default value function of a prop."
                },
            )
            .note(markup! {
                "Vue doesn't give this function access to the component, so "<Emphasis>"this"</Emphasis>" doesn't point to the component."
            })
            .note(markup! {
                "Read other props from the first argument of the function instead, for example "<Emphasis>"default(props) { return props.size }"</Emphasis>"."
            }),
        )
    }
}

declare_node_union! {
    /// A function that Vue calls to compute the default value of a prop:
    /// `default: function () {}` or `default() {}`.
    pub AnyPropDefaultFunction = JsFunctionExpression | JsMethodObjectMember
}

/// Pushes the range of every `this` expression in `function` that refers to
/// the `this` of `function` itself. Nested arrow functions share that `this`;
/// nested functions, and the bodies of nested methods and class members, have
/// their own and are skipped.
fn collect_this_ranges(function: &AnyPropDefaultFunction, ranges: &mut Vec<TextRange>) {
    let root = function.syntax();
    let mut preorder = root.preorder_of_kinds(THIS_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if JsThisExpression::can_cast(node.kind()) {
            let range = node.text_trimmed_range();
            if !node
                .ancestors()
                .skip(1)
                .take_while(|ancestor| ancestor != root)
                .any(|ancestor| is_member_with_own_this(&ancestor, range))
            {
                ranges.push(range);
            }
        } else if node != *root {
            preorder.skip_subtree();
        }
    }
}

/// Returns `true` when `node` is an object or class member that gives `this`
/// a value of its own at `range`.
///
/// Only the code that runs when the member is called or created gets that
/// value: the parameters and body of a method, getter, setter, or constructor,
/// the value of a class field, and a static block. Computed names and
/// decorators run with the `this` of the surrounding code.
fn is_member_with_own_this(node: &JsSyntaxNode, range: TextRange) -> bool {
    let contains = |inner: Option<&JsSyntaxNode>| {
        inner.is_some_and(|inner| inner.text_trimmed_range().contains_range(range))
    };
    if let Some(member) = AnyJsObjectMember::cast_ref(node) {
        return match member {
            AnyJsObjectMember::JsMethodObjectMember(method) => {
                contains(method.parameters().ok().as_ref().map(AstNode::syntax))
                    || contains(method.body().ok().as_ref().map(AstNode::syntax))
            }
            AnyJsObjectMember::JsGetterObjectMember(getter) => {
                contains(getter.body().ok().as_ref().map(AstNode::syntax))
            }
            AnyJsObjectMember::JsSetterObjectMember(setter) => {
                contains(setter.parameter().ok().as_ref().map(AstNode::syntax))
                    || contains(setter.body().ok().as_ref().map(AstNode::syntax))
            }
            _ => false,
        };
    }
    let Some(member) = AnyJsClassMember::cast_ref(node) else {
        return false;
    };
    match member {
        AnyJsClassMember::JsMethodClassMember(method) => {
            contains(method.parameters().ok().as_ref().map(AstNode::syntax))
                || contains(method.body().ok().as_ref().map(AstNode::syntax))
        }
        AnyJsClassMember::JsConstructorClassMember(constructor) => {
            contains(constructor.parameters().ok().as_ref().map(AstNode::syntax))
                || contains(constructor.body().ok().as_ref().map(AstNode::syntax))
        }
        AnyJsClassMember::JsGetterClassMember(getter) => {
            contains(getter.body().ok().as_ref().map(AstNode::syntax))
        }
        AnyJsClassMember::JsSetterClassMember(setter) => {
            contains(setter.parameter().ok().as_ref().map(AstNode::syntax))
                || contains(setter.body().ok().as_ref().map(AstNode::syntax))
        }
        AnyJsClassMember::JsPropertyClassMember(property) => {
            contains(property.value().as_ref().map(AstNode::syntax))
        }
        AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true,
        _ => false,
    }
}

/// The nodes the search for `this` stops at: `this` expressions, and the
/// functions that give everything inside them a `this` of their own.
const THIS_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsThisExpression::KIND_SET
    .union(JsFunctionExpression::KIND_SET)
    .union(JsFunctionDeclaration::KIND_SET);
