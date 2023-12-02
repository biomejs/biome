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
    /// Require consistently using either `T[]` or `Array<T>`
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
    /// ## Options
    /// The rule provides two options that help to specify what type of array declarations to use.
    ///
    /// Default: "simple"
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "consistentArrayType": "simple" | "generic"
    ///     },
    /// }
    /// ```
    ///
    /// By default, all array declarations will be converted to `T[]` or `readonly T[]`, which it means `simple`,
    /// or if the options is set to the "generic", that all will converted to `Array<T>` or `ReadonlyArray<T>`.
    ///
    pub(crate) UseConsistentArrayType {
        version: "1.4.2",
        name: "useConsistentArrayType",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Copy, Clone)]
enum TsArrayKind {
    /// `Array<T>` | `T[]`
    Simple,
    /// `readonly T[]`
    Readonly,
    /// `ReadonlyArray<T>`
    ReadonlyArray,
}

impl Rule for UseConsistentArrayType {
    type Query = Ast<TsReferenceType>;
    type State = AnyTsType;
    type Signals = Option<Self::State>;
    type Options = ConsistentArrayType;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let type_arguments = node.type_arguments()?;
        let ConsistentArrayType {
            consistent_array_type
        } = ctx.options();

        let name = node.name().ok()?;
        println!("name:{:?}", name);
        name.as_js_reference_identifier().and_then(|ident| {
            let name = ident.value_token().ok()?;
            match name.text_trimmed() {
                "Array" => println!("Array"),
                "Readonly" => println!("Readonly"),
                "ReadonlyArray" => println!("ReadonlyArray"),
                _ => (),
            }
            Some(())
        });

        println!("node:{:?}", node);
        println!("arguments:{:?}", type_arguments);
        println!("name:{:?}", name);
        println!("options:{:?}", consistent_array_type);

        None
    }

    // fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
    //     let consistentArrayType {
    //         allow
    //     } = ctx.options();
    // }

    // fn action(ctx: &RuleContext<Self>, state: &Self::State,
    // ) -> Option<JsRuleAction> {

    // }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConsistentArrayType {
    pub consistent_array_type: &'static str
}

impl Default for ConsistentArrayType {
    fn default() -> Self {
        Self {
            consistent_array_type: "simple"
        }
    }
}
