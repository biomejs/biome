use biome_analyze::{
    Ast, context::RuleContext, declare_rule, FixKind, Rule, RuleAction, RuleDiagnostic
};
use biome_js_syntax::{
    AnyTsType, TsReferenceType
};


use crate::JsRuleAction;

declare_rule!{
    /// When expressing array types, this rule promotes the usage of `T[]` shorthand instead of `Array<T>`.
    ///
    /// ESLint (typescript-eslint)
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ### Valid
    pub(crate) UseConsistentArrayType {
        version: "1.3.4",
        name: "useConsistentArrayType",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Copy, Clone)]
enum TsArrayKind {
    Simple,
    Generic,
}

impl Rule for UseConsistentArrayType {
    type Query = Ast<TsReferenceType>;
    type State = AnyTsType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {

    }

    fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {

    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State,
    ) -> Option<JsRuleAction> {

    }
}
