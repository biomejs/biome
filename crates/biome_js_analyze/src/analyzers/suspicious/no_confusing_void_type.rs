use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{JsLanguage, JsSyntaxKind, TsVoidType};
use biome_rowan::{AstNode, SyntaxNode};

declare_rule! {
    /// Disallow `void` type outside of generic or return types.
    ///
    /// `void` in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. `void` can also be misleading for other developers even if used correctly.
    ///
    /// > The `void` type means cannot be mixed with any other types, other than `never`, which accepts all types.
    /// > If you think you need this then you probably want the `undefined` type instead.
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
    pub(crate) NoConfusingVoidType {
        version: "1.2.0",
        name: "noConfusingVoidType",
        source: RuleSource::EslintTypeScript("no-invalid-void-type"),
        recommended: true,
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
            VoidTypeContext::Union => "void is not valid as a constituent in a union type",
            VoidTypeContext::Unknown => {
                "void is only valid as a return type or a type argument in generic type"
            }
        };

        Some(
            RuleDiagnostic::new(rule_category!(), node.range(), markup! {{message}}).note(
                markup! {
                    "Remove "<Emphasis>"void"</Emphasis>
                },
            ),
        )
    }
}

fn decide_void_type_context(node: &SyntaxNode<JsLanguage>) -> Option<VoidTypeContext> {
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
