use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of dependency arrays in `createEffect` and `createMemo`.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal()])
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal]); 
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// const deps = [signal];
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, deps)
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// createEffect(() => {
    ///  console.log(signal());
    /// });
    /// ```
    /// 
    /// ```js
    /// createEffect((prev) => {
    ///  console.log(signal());
    ///  return prev + 1;
    /// }, 0);
    /// ```
    /// 
    /// ```js
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return (prev || 0) + 1;
    /// });
    /// ```
    /// 
    /// ```js
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return prev ? prev + 1 : 1;
    /// }, undefined);
    /// ```
    /// 
    /// ```js
    /// const value = createMemo(() => computeExpensiveValue(a(), b()));
    /// ```
    /// 
    /// ```js
    /// const sum = createMemo((prev) => input() + prev, 0);
    /// ```
    /// 
    /// ```js
    /// const args = [() => { console.log(signal()); }, [signal()]];
    /// createEffect(...args);
    /// ```
    ///
    pub NoReactDeps {
        version: "next",
        name: "noReactDeps",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoReactDeps {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
