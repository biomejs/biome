use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyTsType, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};

declare_rule! {
    ///
    /// Disallow `void` type outside of generic or return types.
    ///
    /// `void` in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. void can also be misleading for other developers even if used correctly.
    ///
    /// > The `void` type means cannot be mixed with any other types, other than `never`, which accepts all types.
    /// > If you think you need this then you probably want the undefined type instead.
    ///
    /// ## Examples
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type PossibleValues = number | void;
    /// type MorePossibleValues = string | ((number & any) | (string | void));
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
    /// let foo: void;
    /// let bar = 1 as unknown as void;
    /// let baz = 1 as unknown as void | string;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function foo(): void {};
    /// function doSomething(this: void) {}
    /// function printArg<T = void>(arg: T) {}
    /// logAndReturn<void>(undefined);
    /// let voidPromise: Promise<void> = new Promise<void>(() => { });
    /// let voidMap: Map<string, void> = new Map<string, void>();
    /// ```
    ///
    pub(crate) NoConfusingVoidType {
        version: "1.0.0",
        name: "noConfusingVoidType",
        recommended: false,
    }
}

type Language = <AnyTsType as AstNode>::Language;

// We only focus on union type
pub enum VoidTypeIn {
    Union,
    Unknown,
}

impl Rule for NoConfusingVoidType {
    type Query = Ast<AnyTsType>;
    type State = VoidTypeIn;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let AnyTsType::TsVoidType(node) = node {
            let result = node_in(node.syntax());
            return result;
        }

        None
    }
    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        return Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {{match_message(state)}},
        ));
    }
}

fn node_in(node: &SyntaxNode<Language>) -> Option<VoidTypeIn> {
    for parent in node.parent()?.ancestors() {
        match parent.kind() {
            // (string | void)
            // string | void
            // string & void
            // arg: void
            // fn<T = void>() {}
            JsSyntaxKind::TS_PARENTHESIZED_TYPE
            | JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST
            | JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST
            | JsSyntaxKind::TS_TYPE_ANNOTATION
            | JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                continue;
            }

            JsSyntaxKind::TS_UNION_TYPE => {
                return Some(VoidTypeIn::Union);
            }

            // function fn(this: void) {}
            // fn(): void;
            // fn<T = void>() {}
            // Promise<void>
            JsSyntaxKind::TS_THIS_PARAMETER
            | JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION
            | JsSyntaxKind::TS_TYPE_PARAMETER
            | JsSyntaxKind::TS_TYPE_ARGUMENT_LIST => {
                return None;
            }

            _ => return Some(VoidTypeIn::Unknown),
        }
    }

    Some(VoidTypeIn::Unknown)
}

fn match_message(node: &VoidTypeIn) -> String {
    if matches!(node, VoidTypeIn::Union) {
        return "void is not valid as a constituent in a union type".into();
    }

    "void is only valid as a return type or a type argument in generic type".into()
}
