use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyTsType, TsDefaultTypeClause, TsIntersectionTypeElementList, TsParenthesizedType,
    TsReturnTypeAnnotation, TsThisParameter, TsTypeAnnotation, TsTypeArgumentList, TsTypeParameter,
    TsUnionTypeVariantList,
};
use rome_rowan::{AstNode, SyntaxNode, SyntaxNodeCast};

declare_rule! {
    ///
    /// Disallow void type outside of generic or return types.
    ///
    /// void in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. void can also be misleading for other developers even if used correctly.
    ///
    /// > The void type means cannot be mixed with any other types, other than never, which accepts all types. If you think you need this then you probably want the undefined type instead.
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

    fn run(ctx: &rome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let AnyTsType::TsVoidType(node) = node {
            let result = node_in(node.syntax());
            return result;
        }

        None
    }
    fn diagnostic(
        ctx: &rome_analyze::context::RuleContext<Self>,
        state: &Self::State,
    ) -> Option<rome_analyze::RuleDiagnostic> {
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
    if let Some(n) = parent.cast_ref::<TsParenthesizedType>() {
        return node_in(n.syntax());
    }

    // string | void
    if let Some(n) = parent.cast_ref::<TsUnionTypeVariantList>() {
        return node_in(n.syntax());
    }

    // string & void
    if let Some(n) = parent.cast_ref::<TsIntersectionTypeElementList>() {
        return node_in(n.syntax());
    }

    // arg: void
    if let Some(n) = parent.cast_ref::<TsTypeAnnotation>() {
        return node_in(n.syntax());
    }

    // fn<T = void>() {}
    // T = void
    if let Some(n) = parent.cast_ref::<TsDefaultTypeClause>() {
        return node_in(n.syntax());
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
    if parent.cast_ref::<TsThisParameter>().is_some() {
        return None;
    }

    // fn(): void;
    if parent.cast_ref::<TsReturnTypeAnnotation>().is_some() {
        return None;
    }

    // fn<T = void>() {} or Promise<void>
    if parent.cast_ref::<TsTypeParameter>().is_some()
        || parent.cast_ref::<TsTypeArgumentList>().is_some()
    {
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
