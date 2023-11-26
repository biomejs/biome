use biome_analyze::{
    Ast, context::RuleContext, declare_rule, FixKind, Rule,
    // RuleAction, RuleDiagnostic
};
use biome_js_syntax::{
    AnyTsType, TsReferenceType
};

use serde::{Deserialize, Serialize};

// use crate::JsRuleAction;

declare_rule!{
    /// When expressing array types, this rule promotes the usage of `T[]` shorthand instead of `Array<T>`.
    ///
    /// ESLint (typescript-eslint) equivalent: [array-type](https://typescript-eslint.io/rules/array-type)
    /// ## Example
    ///
    /// ### Invalid
    /// ```ts,expect_diagnostic
    /// let invalid: Array<foo>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid: Promise<Array<string>>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// const valid: Array<string | number> = ['a', 'b'];
    /// const valid: Array<{ prop: string }> = [{ prop: 'a' }];
    /// const valid: Array<() => void> = [() => {}];
    /// const valid: MyType[] = ['a', 'b'];
    /// const valid: string[] = ['a', 'b'];
    /// const valid: readonly string[] = ['a', 'b'];
    /// ```
    ///
    /// ## options
    /// Allows to use generic array type, the default value is false.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "allow_array_generic": true
    ///     }
    /// }
    /// ```
    ///
    pub(crate) UseConsistentArrayType {
        version: "1.3.4",
        name: "useConsistentArrayType",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

// #[derive(Debug, Copy, Clone)]
// enum TsArrayKind {
//     Simple,
//     Generic,
// }

impl Rule for UseConsistentArrayType {
    type Query = Ast<TsReferenceType>;
    type State = AnyTsType;
    type Signals = Option<Self::State>;
    type Options = AllowArrayGenericOption;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let type_arguments = node.type_arguments()?;
        let AllowArrayGenericOption {
            allow_array_generic
        } = ctx.options();

        println!("node:{:?}", node);
        println!("arguments:{:?}", type_arguments);
        println!("options:{:?}", allow_array_generic);

        None
    }

    // fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
    //     let AllowArrayGenericOption {
    //         allow
    //     } = ctx.options();
    // }

    // fn action(ctx: &RuleContext<Self>, state: &Self::State,
    // ) -> Option<JsRuleAction> {

    // }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AllowArrayGenericOption {
    pub allow_array_generic: bool
}

impl Default for AllowArrayGenericOption {
    fn default() -> Self {
        Self {
            allow_array_generic: false
        }
    }
}
