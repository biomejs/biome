use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression,
    JsComputedMemberName, JsConstructorClassMember, JsDecorator, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsPropertyClassMember, JsSetterClassMember,
    JsSetterObjectMember, JsStaticInitializationBlockClassMember, JsSyntaxNode, JsThisExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet};
use biome_rule_options::no_useless_bind::NoUselessBindOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary `.bind()` calls on functions.
    ///
    /// Calling `.bind()` on a function creates a new function with a fixed value for `this`.
    /// When the function never uses `this`, the call does nothing useful: it only makes the
    /// code harder to read and creates an extra function at runtime.
    ///
    /// Arrow functions always use the `this` of the code around them, so calling `.bind()` on an
    /// arrow function can't change `this` either.
    ///
    /// This rule only reports `.bind()` calls with a single argument. Calls with more arguments
    /// also fix the values of the first parameters, so they still have an effect.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x = function () {
    ///     foo();
    /// }.bind(bar);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = (() => {
    ///     this.foo();
    /// }).bind(bar);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = function () {
    ///     (function () {
    ///         this.foo();
    ///     })();
    /// }.bind(bar);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = function () {
    ///     this.foo();
    /// }.bind(bar);
    ///
    /// const b = function () {
    ///     (() => this.foo())();
    /// }.bind(bar);
    ///
    /// const c = function (a) {
    ///     return a + 1;
    /// }.bind(bar, 1);
    /// ```
    ///
    pub NoUselessBind {
        version: "next",
        name: "noUselessBind",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-extra-bind").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessBind {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let (_, function) = bind_callee(call)?;
        let arguments = call.arguments().ok()?.args();
        if arguments.len() != 1
            || matches!(arguments.first()?.ok()?, AnyJsCallArgument::JsSpread(_))
        {
            return None;
        }
        match function {
            AnyJsExpression::JsArrowFunctionExpression(_) => Some(()),
            AnyJsExpression::JsFunctionExpression(function) => {
                (!uses_own_this(&function)).then_some(())
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let (member, function) = bind_callee(ctx.query())?;
        let range = match &member {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This "<Emphasis>"bind()"</Emphasis>" call is unnecessary."
            },
        );
        let diagnostic = if matches!(function, AnyJsExpression::JsArrowFunctionExpression(_)) {
            diagnostic.note(markup! {
                "Arrow functions always use the "<Emphasis>"this"</Emphasis>" of the code around them, so "<Emphasis>"bind()"</Emphasis>" can't change it."
            })
        } else {
            diagnostic.note(markup! {
                "This function doesn't use "<Emphasis>"this"</Emphasis>", so binding it has no effect."
            })
        };
        Some(diagnostic.note(markup! {
            "Remove the "<Emphasis>"bind()"</Emphasis>" call and use the function directly."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let (member, _) = bind_callee(call)?;
        // In `(function () {}.bind)(bar)`, removing the call would also remove the parentheses
        // that keep the function from being read as a declaration.
        if call.callee().ok()?.syntax() != member.syntax() {
            return None;
        }

        // Removing the call also removes its argument, so the argument must not run any code.
        let argument = call.arguments().ok()?.args().first()?.ok()?;
        let argument = argument.as_any_js_expression()?.clone().omit_parentheses();
        if !matches!(
            argument,
            AnyJsExpression::AnyJsLiteralExpression(_)
                | AnyJsExpression::JsIdentifierExpression(_)
                | AnyJsExpression::JsThisExpression(_)
                | AnyJsExpression::JsFunctionExpression(_)
                | AnyJsExpression::JsArrowFunctionExpression(_)
        ) {
            return None;
        }

        let object = member.object().ok()?;
        let object_last_token = object.syntax().last_token()?;
        let call_last_token = call.syntax().last_token()?;
        if object_last_token.has_trailing_comments() {
            return None;
        }
        let mut removed_tokens = std::iter::successors(object_last_token.next_token(), |token| {
            (*token != call_last_token)
                .then(|| token.next_token())
                .flatten()
        });
        if removed_tokens.any(|token| {
            token.has_leading_comments()
                || (token != call_last_token && token.has_trailing_comments())
        }) {
            return None;
        }

        let replacement =
            object.with_trailing_trivia_pieces(call_last_token.trailing_trivia().pieces())?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(AnyJsExpression::from(call.clone()), replacement);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"bind()"</Emphasis>" call." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the `.bind` member expression called by `call`, and the function expression or arrow
/// function it's called on.
fn bind_callee(call: &JsCallExpression) -> Option<(AnyJsMemberExpression, AnyJsExpression)> {
    let member = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
    if member.member_name()?.text() != "bind" {
        return None;
    }
    let function = member.object().ok()?.omit_type_wrappers()?;
    Some((member, function))
}

/// Returns `true` if `function` contains a `this` expression that refers to the `this` of
/// `function` itself, rather than to the `this` of a nested function or class member.
fn uses_own_this(function: &JsFunctionExpression) -> bool {
    function
        .syntax()
        .descendants()
        .filter(|node| JsThisExpression::can_cast(node.kind()))
        .any(|this| refers_to_function_this(&this, function.syntax()))
}

/// Returns `true` if the `this` expression `this` takes its value from `function`.
fn refers_to_function_this(this: &JsSyntaxNode, function: &JsSyntaxNode) -> bool {
    let mut child = this.clone();
    for ancestor in this.ancestors().skip(1) {
        if ancestor == *function {
            return true;
        }
        if JsDecorator::can_cast(ancestor.kind()) {
            // Decorators run outside the member they decorate. Counting `this` inside them as
            // used avoids reporting a function that might need it.
            return true;
        }
        // A computed key, such as `[this.key]() {}`, uses the `this` of the code around the member.
        if OWN_THIS_KINDS.matches(ancestor.kind()) && !JsComputedMemberName::can_cast(child.kind())
        {
            return false;
        }
        child = ancestor;
    }
    false
}

/// Nodes that give the code inside them their own value for `this`.
const OWN_THIS_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionExpression::KIND_SET
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsConstructorClassMember::KIND_SET)
    .union(JsMethodClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsPropertyClassMember::KIND_SET)
    .union(JsStaticInitializationBlockClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);
