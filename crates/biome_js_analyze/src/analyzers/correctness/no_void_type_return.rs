use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    AnyTsReturnType, JsArrowFunctionExpression, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsGetterClassMember,
    JsGetterObjectMember, JsMethodClassMember, JsMethodObjectMember, JsReturnStatement,
};
use biome_rowan::{declare_node_union, AstNode};

use crate::control_flow::AnyJsControlFlowRoot;

declare_rule! {
    /// Disallow returning a value from a function with the return type 'void'
    ///
    /// 'void' signals the absence of value. The returned value is likely to be ignored by the caller.
    /// Thus, returning a value when the return type of function is 'void', is undoubtedly an error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// class A {
    ///     f(): void {
    ///         return undefined;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const a = {
    ///     f(): void {
    ///         return undefined;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(): void {
    ///     return undefined;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// export default function(): void {
    ///     return undefined;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const g = (): void => {
    ///     return undefined;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const h = function(): void {
    ///     return undefined;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class A {
    ///     f() {
    ///         return undefined;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts
    /// class B {
    ///     f(): void {}
    /// }
    /// ```
    ///
    /// ```ts
    /// function f(): void {
    ///     return;
    /// }
    /// ```
    ///
    pub(crate) NoVoidTypeReturn {
        version: "1.0.0",
        name: "noVoidTypeReturn",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) JsFunctionMethod = JsArrowFunctionExpression | JsFunctionDeclaration | JsFunctionExportDefaultDeclaration | JsFunctionExpression | JsGetterClassMember | JsGetterObjectMember | JsMethodClassMember | JsMethodObjectMember
}

pub(crate) fn return_type(func: &JsFunctionMethod) -> Option<AnyTsReturnType> {
    match func {
        JsFunctionMethod::JsArrowFunctionExpression(func) => {
            func.return_type_annotation()?.ty().ok()
        }
        JsFunctionMethod::JsFunctionDeclaration(func) => func.return_type_annotation()?.ty().ok(),
        JsFunctionMethod::JsFunctionExportDefaultDeclaration(func) => {
            func.return_type_annotation()?.ty().ok()
        }
        JsFunctionMethod::JsFunctionExpression(func) => func.return_type_annotation()?.ty().ok(),
        JsFunctionMethod::JsGetterClassMember(func) => {
            Some(AnyTsReturnType::AnyTsType(func.return_type()?.ty().ok()?))
        }
        JsFunctionMethod::JsGetterObjectMember(func) => {
            Some(AnyTsReturnType::AnyTsType(func.return_type()?.ty().ok()?))
        }
        JsFunctionMethod::JsMethodClassMember(func) => func.return_type_annotation()?.ty().ok(),
        JsFunctionMethod::JsMethodObjectMember(func) => func.return_type_annotation()?.ty().ok(),
    }
}

impl Rule for NoVoidTypeReturn {
    type Query = Ast<JsReturnStatement>;
    type State = JsFunctionMethod;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();
        // Do not take arg-less returns into account
        let _arg = ret.argument()?;
        let func = ret
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .and_then(JsFunctionMethod::cast)?;
        let ret_type = return_type(&func)?;
        ret_type.as_any_ts_type()?.as_ts_void_type().and(Some(func))
    }

    fn diagnostic(ctx: &RuleContext<Self>, func: &Self::State) -> Option<RuleDiagnostic> {
        let ret = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            ret.range(),
            markup! {
                "The function should not "<Emphasis>"return"</Emphasis>" a value because its return type is "<Emphasis>"void"</Emphasis>"."
            },
        ).detail(func.range(), "The function is here:").note(
            "'void' signals the absence of value. The returned value is likely to be ignored by the caller."
        ))
    }
}
