use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyTsType, TsDefaultTypeClause, TsIntersectionTypeElementList, TsParenthesizedType,
    TsReturnTypeAnnotation, TsThisParameter, TsTypeAnnotation, TsTypeArgumentList, TsTypeParameter,
    TsUnionTypeVariantList,
};
use rome_rowan::{AstNode, SyntaxNode, SyntaxNodeCast};

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
    let parent = node.parent()?;

    // (string | void)
    if TsParenthesizedType::can_cast(parent.kind()) {
        return node_in(&parent);
    }

    // string | void
    if TsUnionTypeVariantList::can_cast(parent.kind()) {
        return node_in(&parent);
    }

    // string & void
    if TsIntersectionTypeElementList::can_cast(parent.kind()) {
        return node_in(&parent);
    }

    // arg: void
    if TsTypeAnnotation::can_cast(parent.kind()) {
        return node_in(&parent);
    }

    // fn<T = void>() {}
    // T = void
    if TsDefaultTypeClause::can_cast(parent.kind()) {
        return node_in(&parent);
    }

    // string | void or string & void
    if let Some(n) = parent.cast_ref::<AnyTsType>() {
        return match n {
            AnyTsType::TsUnionType(_) => Some(VoidTypeIn::Union),
            AnyTsType::TsIntersectionType(_) => Some(VoidTypeIn::Unknown),
            _ => None,
        };
    }

    // function fn(this: void) {}
    if TsThisParameter::can_cast(parent.kind()) {
        return None;
    }

    // fn(): void;
    if TsReturnTypeAnnotation::can_cast(parent.kind()) {
        return None;
    }

    // fn<T = void>() {} or Promise<void>
    if TsTypeParameter::can_cast(parent.kind()) || TsTypeArgumentList::can_cast(parent.kind()) {
        return None;
    }

    Some(VoidTypeIn::Unknown)
}

fn match_message(node: &VoidTypeIn) -> String {
    if matches!(node, VoidTypeIn::Union) {
        return "void is not valid as a constituent in a union type".into();
    }

    "void is only valid as a return type or a type argument in generic type".into()
}
