use crate::react::components::ReactSuperClass;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClass, AnyJsExpression, AnyJsMemberExpression, JsGetterClassMember, JsMethodClassMember,
    JsPropertyClassMember, JsSetterClassMember, JsSyntaxKind, JsSyntaxNode, JsUnaryExpression,
    JsUnaryOperator, assign_ext::AnyJsMemberAssignment,
};
use biome_rowan::{AstNode, AstNodeList, declare_node_union};
use biome_rule_options::no_react_direct_state_mutation::NoReactDirectStateMutationOptions;

declare_lint_rule! {
    /// Disallow mutating `this.state` directly in React class components.
    ///
    /// State should only be updated through `this.setState()` so React can correctly schedule a
    /// re-render and combine state updates.
    /// Writing to `this.state` is only allowed in the constructor of a class component, to set the
    /// initial state.
    /// This rule applies to classes that extend `Component` or `PureComponent`, including
    /// `React.Component` and `React.PureComponent`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// class Counter extends React.Component {
    ///   increment() {
    ///     this.state.count = this.state.count + 1;
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// class Counter extends React.Component {
    ///   constructor(props) {
    ///     super(props);
    ///     loadCount((count) => {
    ///       this.state = { count };
    ///     });
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// class Counter extends React.Component {
    ///   constructor(props) {
    ///     super(props);
    ///     this.state = { count: 0 };
    ///   }
    ///
    ///   increment() {
    ///     this.setState({ count: this.state.count + 1 });
    ///   }
    /// }
    /// ```
    ///
    pub NoReactDirectStateMutation {
        version: "2.6.7",
        name: "noReactDirectStateMutation",
        language: "js",
        sources: &[RuleSource::EslintReact("no-direct-mutation-state").inspired(), RuleSource::EslintReactX("no-direct-mutation-state").inspired(), RuleSource::EslintReactXyz("no-direct-mutation-state").inspired()],
        domains: &[RuleDomain::React],
        recommended: false,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyStateMutation = AnyJsMemberAssignment | JsUnaryExpression
}

impl Rule for NoReactDirectStateMutation {
    type Query = Ast<AnyStateMutation>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReactDirectStateMutationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mutation = ctx.query();
        let targets_state = match mutation {
            AnyStateMutation::AnyJsMemberAssignment(assignment) => {
                assignment_starts_with_this_state(assignment)?
            }
            AnyStateMutation::JsUnaryExpression(unary) => {
                unary.operator().ok()? == JsUnaryOperator::Delete
                    && starts_with_this_state(unary.argument().ok()?.omit_type_wrappers()?)?
            }
        };
        (targets_state && is_mutation_in_react_class(mutation.syntax())).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This code changes "<Emphasis>"this.state"</Emphasis>" directly."
                },
            )
            .note(markup! {
                "React does not render the component again after a direct change, and a later "<Emphasis>"setState()"</Emphasis>" call can overwrite it."
            })
            .note(markup! {
                "Call "<Emphasis>"this.setState()"</Emphasis>" instead. Only assign to "<Emphasis>"this.state"</Emphasis>" directly in the constructor, to set the initial state."
            }),
        )
    }
}

/// Returns whether the assignment target is `this.state` or anything nested in it.
fn assignment_starts_with_this_state(assignment: &AnyJsMemberAssignment) -> Option<bool> {
    let object = assignment.object().ok()?.omit_type_wrappers()?;
    if object.as_js_this_expression().is_some() {
        return Some(match assignment {
            AnyJsMemberAssignment::JsStaticMemberAssignment(assignment) => {
                assignment
                    .member()
                    .ok()?
                    .as_js_name()?
                    .value_token()
                    .ok()?
                    .text_trimmed()
                    == "state"
            }
            AnyJsMemberAssignment::JsComputedMemberAssignment(assignment) => {
                assignment
                    .member()
                    .ok()?
                    .omit_parentheses()
                    .as_static_value()?
                    .text()
                    == "state"
            }
        });
    }

    starts_with_this_state(object)
}

/// Returns whether `expression` is a member access chain that starts with `this.state`.
fn starts_with_this_state(mut expression: AnyJsExpression) -> Option<bool> {
    while let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        let object = member.object().ok()?.omit_type_wrappers()?;
        if object.as_js_this_expression().is_some() {
            return Some(
                member
                    .member_name()
                    .is_some_and(|name| name.text() == "state"),
            );
        }
        expression = object;
    }

    Some(false)
}

/// Returns whether `this` at `node` is an instance of a React class component, and the
/// write isn't an initial state assignment that runs directly in the constructor.
///
/// Arrow functions keep the `this` of the enclosing method, so the search continues
/// through them. Regular functions, object members, and static class members have a
/// different `this`, so the search stops there.
fn is_mutation_in_react_class(node: &JsSyntaxNode) -> bool {
    let mut inside_arrow_or_call = false;

    for ancestor in node.ancestors() {
        match ancestor.kind() {
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION | JsSyntaxKind::JS_CALL_EXPRESSION => {
                inside_arrow_or_call = true;
            }
            JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => return false,
            // Code inside an arrow function or a call can run after the constructor
            // returns, for example as a callback, so only a write directly in the
            // constructor sets the initial state.
            JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER if !inside_arrow_or_call => return false,
            _ if is_static_class_member(&ancestor) => return false,
            _ => {
                if let Some(class) = AnyJsClass::cast_ref(&ancestor) {
                    return class
                        .extends_clause()
                        .and_then(|clause| ReactSuperClass::from_extends_clause(&clause))
                        .is_some();
                }
            }
        }
    }

    false
}

fn is_static_class_member(node: &JsSyntaxNode) -> bool {
    if let Some(property) = JsPropertyClassMember::cast_ref(node) {
        return property
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some());
    }

    JsMethodClassMember::cast_ref(node)
        .map(|method| method.modifiers())
        .or_else(|| JsGetterClassMember::cast_ref(node).map(|getter| getter.modifiers()))
        .or_else(|| JsSetterClassMember::cast_ref(node).map(|setter| setter.modifiers()))
        .is_some_and(|modifiers| {
            modifiers
                .iter()
                .any(|modifier| modifier.as_js_static_modifier().is_some())
        })
}
