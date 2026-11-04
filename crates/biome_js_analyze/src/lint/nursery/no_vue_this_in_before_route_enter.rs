use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsRoot, JsConstructorClassMember,
    JsConstructorParameters, JsFormalParameter, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsInitializerClause,
    JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsObjectExpression, JsParameters,
    JsPropertyClassMember, JsSetterClassMember, JsSetterObjectMember,
    JsStaticInitializationBlockClassMember, JsStatementList, JsSyntaxNode, JsThisExpression,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::no_vue_this_in_before_route_enter::NoVueThisInBeforeRouteEnterOptions;
use biome_vue_semantic::{Component, ComponentKind};

declare_lint_rule! {
    /// Disallow `this` in the `beforeRouteEnter` navigation guard of a Vue component.
    ///
    /// Vue Router calls `beforeRouteEnter` before it creates the component, so the component
    /// doesn't exist yet and `this` is `undefined`. Reading a property through `this`, such as
    /// `this.loadUser()`, throws a `TypeError` when the user navigates to the page.
    ///
    /// To use the component, pass a callback to `next`. Vue Router calls it with the component
    /// once the navigation is confirmed.
    ///
    /// The rule checks `beforeRouteEnter` when it is written as a method or a `function`
    /// expression, in `export default {}`, `defineComponent()`, `createApp()`, and
    /// `defineOptions()` in `<script setup>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   beforeRouteEnter(to, from, next) {
    ///     this.loadUser(to.params.id);
    ///     next();
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
    ///   beforeRouteEnter(to, from, next) {
    ///     next((vm) => {
    ///       vm.loadUser(to.params.id);
    ///     });
    ///   },
    ///   beforeRouteUpdate(to) {
    ///     this.loadUser(to.params.id);
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueThisInBeforeRouteEnter {
        version: "2.6.5",
        name: "noVueThisInBeforeRouteEnter",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-this-in-before-route-enter").same()],
    }
}

impl Rule for NoVueThisInBeforeRouteEnter {
    type Query = VueComponent;
    /// The range of a `this` expression that refers to the `beforeRouteEnter` guard's `this`.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueThisInBeforeRouteEnterOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(options) = options_object(&ctx.root(), ctx.query(), ctx.model()) else {
            return Box::default();
        };
        let mut ranges = Vec::new();
        for member in options.members().iter().flatten() {
            if member
                .name()
                .is_none_or(|name| name.text() != "beforeRouteEnter")
            {
                continue;
            }
            if let Some(guard) = guard_function(member) {
                collect_this_expressions(&guard, &mut ranges);
            }
        }
        ranges.into_boxed_slice()
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
                    <Emphasis>"this"</Emphasis>" is used in "<Emphasis>"beforeRouteEnter"</Emphasis>", where it is "<Emphasis>"undefined"</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue Router calls "<Emphasis>"beforeRouteEnter"</Emphasis>" before it creates the component, so the component can't be reached through "<Emphasis>"this"</Emphasis>"."
            })
            .note(markup! {
                "Pass a callback to "<Emphasis>"next"</Emphasis>" to use the component once it is created, for example "<Emphasis>"next((vm) => vm.loadData())"</Emphasis>"."
            }),
        )
    }
}

/// Returns the options object that defines `component`.
///
/// The Vue semantic model records only the range of the definition, so the
/// node is found again in `root`. Returns `None` when the options aren't an
/// object literal.
fn options_object(
    root: &AnyJsRoot,
    component: &Component,
    model: &SemanticModel,
) -> Option<JsObjectExpression> {
    let options = if component.kind() == ComponentKind::Setup {
        define_options_argument(root, model)?
    } else {
        let definition = root
            .syntax()
            .covering_element(component.range())
            .into_node()
            .and_then(AnyJsExpression::cast)?;
        match component.kind() {
            ComponentKind::OptionsObject => definition,
            kind => {
                let arguments = definition
                    .as_js_call_expression()?
                    .arguments()
                    .ok()?
                    .args();
                // `createApp(options, rootProps)`, `defineComponent(options)`,
                // and `defineComponent(setup, options)`.
                let argument = if kind == ComponentKind::CreateApp {
                    arguments.first()
                } else {
                    arguments.last()
                };
                argument?.ok()?.as_any_js_expression()?.clone()
            }
        }
    };
    options.inner_expression()?.as_js_object_expression().cloned()
}

/// Returns the argument of the `defineOptions()` call at the top level of a
/// `<script setup>` block.
///
/// `defineOptions` is a compiler macro, so a call to a function that the file
/// declares or imports under that name isn't one.
fn define_options_argument(root: &AnyJsRoot, model: &SemanticModel) -> Option<AnyJsExpression> {
    root.as_js_module()?.items().iter().find_map(|item| {
        let call = item
            .as_any_js_statement()?
            .as_js_expression_statement()?
            .expression()
            .ok()?
            .inner_expression()?;
        let call = call.as_js_call_expression()?;
        let callee = call.callee().ok()?.inner_expression()?;
        let reference = callee.as_js_reference_identifier()?;
        if !reference.has_name("defineOptions") || model.binding(&reference).is_some() {
            return None;
        }
        call.arguments()
            .ok()?
            .args()
            .first()?
            .ok()?
            .as_any_js_expression()
            .cloned()
    })
}

/// Returns the function that `member` defines as the guard, when it is a
/// method or a `function` expression.
///
/// An arrow function doesn't receive the `this` that Vue Router passes to the
/// guard, so its `this` is left to rules about the surrounding code.
fn guard_function(member: AnyJsObjectMember) -> Option<JsSyntaxNode> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => Some(method.into_syntax()),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            let value = property.value().ok()?.inner_expression()?;
            JsFunctionExpression::cast(value.into_syntax()).map(AstNode::into_syntax)
        }
        _ => None,
    }
}

/// Pushes the range of every `this` expression in `guard` that refers to the
/// guard's own `this`.
///
/// Arrow functions are searched, since they use the `this` of the function
/// around them. The bodies and parameters of other functions and of methods
/// are skipped, since they have a `this` of their own. Computed keys,
/// decorators, and `extends` clauses are searched, since they are evaluated
/// with the `this` around them.
fn collect_this_expressions(guard: &JsSyntaxNode, ranges: &mut Vec<TextRange>) {
    let mut preorder = guard.preorder_of_kinds(THIS_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if !JsThisExpression::can_cast(node.kind()) {
            if node != *guard {
                preorder.skip_subtree();
            }
        } else if !is_in_nested_member(&node, guard) {
            ranges.push(node.text_trimmed_range());
        }
    }
}

/// Returns `true` when `this` is in the body, parameters, or initializer of a
/// class or object member inside `guard`, where it refers to that member's
/// `this`.
fn is_in_nested_member(this: &JsSyntaxNode, guard: &JsSyntaxNode) -> bool {
    let mut child = this.clone();
    for node in this.ancestors().skip(1) {
        if node == *guard {
            return false;
        }
        if THIS_BINDING_MEMBERS.matches(node.kind())
            && MEMBER_EXECUTION_CONTEXTS.matches(child.kind())
        {
            return true;
        }
        child = node;
    }
    false
}

/// The nodes the search for `this` stops at: `this` expressions, and the
/// functions that give everything inside them a different `this`.
const THIS_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsThisExpression::KIND_SET
    .union(JsFunctionExpression::KIND_SET)
    .union(JsFunctionDeclaration::KIND_SET);

/// Class and object members that have their own `this`.
const THIS_BINDING_MEMBERS: SyntaxKindSet<JsLanguage> = JsMethodClassMember::KIND_SET
    .union(JsConstructorClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsPropertyClassMember::KIND_SET)
    .union(JsStaticInitializationBlockClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Children of [THIS_BINDING_MEMBERS] that are evaluated with the member's `this`.
const MEMBER_EXECUTION_CONTEXTS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsInitializerClause::KIND_SET)
    .union(JsStatementList::KIND_SET);
