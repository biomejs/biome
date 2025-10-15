use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, JsIdentifierBinding, JsVariableDeclaration,
    JsVariableDeclarationClause, JsVariableDeclarator, JsVariableDeclaratorList,
    TsDeclareStatement,
};
use biome_rowan::AstNode;
use biome_rule_options::no_unassigned_variables::NoUnassignedVariablesOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow `let` or `var` variables that are read but never assigned.
    ///
    /// This rule flags let or var declarations that are never assigned a value but are still read or used in the code.
    /// Since these variables will always be undefined, their usage is likely a programming mistake.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let status;
    /// if (status === 'ready') {
    ///     console.log('Status is ready');
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let value: number | undefined;
    /// console.log(value);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let message = "hello";
    /// console.log(message);
    ///
    /// let user;
    /// user = getUser();
    /// console.log(user.name);
    ///
    /// let count;
    /// count = 0;
    /// count++;
    /// ```
    ///
    /// ```ts
    /// declare let value: number | undefined;
    /// console.log(value);
    ///
    /// declare module "my-module" {
    ///     let value: string;
    ///     export = value;
    /// }
    /// ```
    ///
    pub NoUnassignedVariables {
        version: "2.1.0",
        name: "noUnassignedVariables",
        language: "js",
        sources: &[RuleSource::Eslint("no-unassigned-vars").same()],
        recommended: false,
    }
}

impl Rule for NoUnassignedVariables {
    type Query = Semantic<JsVariableDeclarator>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = NoUnassignedVariablesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();
        let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(id))) =
            declarator.id()
        else {
            return None;
        };
        let declaration = declarator
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;
        if declaration.is_const() || declarator.initializer().is_some() {
            return None;
        }
        // e.g. `declare let value: number | undefined;`
        if declaration
            .parent::<JsVariableDeclarationClause>()
            .is_some_and(|clause| clause.parent::<TsDeclareStatement>().is_some())
        {
            return None;
        }
        // check if the variable is declared in a function or module
        // e.g. `declare module "my-module" { let value: string; export = value; }`
        if is_inside_ts_declare_statement(&declaration) {
            return None;
        }
        let model = ctx.model();

        if id.all_writes(model).next().is_some() {
            return None;
        }
        if id.all_reads(model).next().is_some() {
            return Some(id);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        // let state = state.to_trimmed_text();
        let node = ctx.query();
        let name_token = state.name_token().ok()?;
        let name = name_token.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The variable '"<Emphasis>{name}</Emphasis>"' is declared but never assigned a value."
                },
            )
            .note(markup! {
                "Variable declared without assignment. Either assign a value or remove the declaration."
            }),
        )
    }
}

fn is_inside_ts_declare_statement(node: &JsVariableDeclaration) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| TsDeclareStatement::can_cast(ancestor.kind()))
}
