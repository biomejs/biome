use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyTsType, JsSyntaxKind, JsSyntaxNode, TsConditionalType, TsVoidType, T};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow `void` type outside of generic or return types.
    ///
    /// `void` in TypeScript refers to a function return that is meant to be ignored.
    /// Attempting to use a void type outside of a return type or a type parameter is often a sign of programmer error.
    /// `void` can also be misleading for other developers even if used correctly.
    ///
    /// > The `void` type means cannot be mixed with any other types, other than `never`, which accepts all types.
    /// > If you think you need this then you probably want the `undefined` type instead.
    ///
    /// The code action suggests using `undefined` instead of `void`.
    /// It is unsafe because a variable with the `void` type cannot be asigned to a variable with the `undefined` type.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let foo: void;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function logSomething(thing: void) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface Interface {
    ///     prop: void;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type PossibleValues = number | void;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function foo(): void {};
    /// ```
    ///
    /// ```ts
    /// function doSomething(this: void) {}
    /// ```
    ///
    /// ```ts
    /// function printArg<T = void>(arg: T) {}
    /// ```
    pub NoConfusingVoidType {
        version: "1.2.0",
        name: "noConfusingVoidType",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-invalid-void-type")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

/// We only focus on union type
pub enum VoidTypeContext {
    Union,
    Unknown,
}

impl Rule for NoConfusingVoidType {
    type Query = Ast<TsVoidType>;
    type State = VoidTypeContext;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        decide_void_type_context(node.syntax())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let message = match state {
            VoidTypeContext::Union => "inside a union type.",
            VoidTypeContext::Unknown => "outside a return type or a type parameter.",
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
            <Emphasis>"void"</Emphasis>" is confusing "{message}},
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyTsType::from(node.clone()),
            AnyTsType::from(make::ts_undefined_type(make::token(T![undefined]))),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::MaybeIncorrect,
            markup! { "Use "<Emphasis>"undefined"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn decide_void_type_context(node: &JsSyntaxNode) -> Option<VoidTypeContext> {
    for parent in node.parent()?.ancestors() {
        match parent.kind() {
            JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                // checks if the union type contains a generic type has a void type as argument
                for child in parent.descendants() {
                    if child.kind() == JsSyntaxKind::TS_TYPE_ARGUMENT_LIST {
                        let found_void_type = child
                            .descendants()
                            .any(|descendant| descendant.kind() == JsSyntaxKind::TS_VOID_TYPE);
                        if found_void_type {
                            return None;
                        }
                    }
                }
            }

            // (string | void)
            // string & void
            // arg: void
            // fn<T = void>() {}
            JsSyntaxKind::TS_PARENTHESIZED_TYPE
            | JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST
            | JsSyntaxKind::TS_TYPE_ANNOTATION
            | JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                continue;
            }

            JsSyntaxKind::TS_UNION_TYPE => {
                return Some(VoidTypeContext::Union);
            }

            // type Conditional<T> = T extends void ? Record<string, never> : T
            JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                let conditional = TsConditionalType::unwrap_cast(parent.clone());
                let is_extends_type = conditional
                    .extends_type()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node);
                if is_extends_type {
                    return None;
                }
            }

            // Promise<void>
            // functionGeneric<void>(undefined)
            JsSyntaxKind::TS_TYPE_ARGUMENT_LIST
            // function fn(this: void) {}
            | JsSyntaxKind::TS_THIS_PARAMETER
            // function fn(): void;
            | JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION
            // function fn<T = void>() {}
            | JsSyntaxKind::TS_TYPE_PARAMETER
            // () => void
            | JsSyntaxKind::TS_FUNCTION_TYPE => {
                return None;
            }

            _ => return Some(VoidTypeContext::Unknown),
        }
    }

    Some(VoidTypeContext::Unknown)
}
