use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsAssignmentExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsConditionalExpression, JsExpressionStatement,
    JsInitializerClause, JsLanguage, JsModule, JsModuleItemList, JsParenthesizedExpression,
    JsScript, JsStatementList, JsStaticMemberAssignment, JsStaticMemberExpression, JsVariableDeclaration,
    JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement, TsAsExpression,
    TsNonNullAssertionExpression, TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::use_top_level_require::UseTopLevelRequireOptions;

declare_lint_rule! {
    /// Require `require()` calls to be placed at the top level of the module.
    ///
    /// When `require()` calls are spread across functions, blocks, and other expressions,
    /// it's hard to see at a glance which modules a file depends on. `require()` also loads
    /// modules synchronously, so a call inside a function that runs later can pause the
    /// program at a moment you don't expect.
    ///
    /// A `require()` call is allowed only at the top level of the module, and only when it is
    /// used as a statement on its own, in a variable declaration, in an assignment, in a
    /// conditional (`? :`) expression, as an argument of another call, or when its result is
    /// accessed with a property or called. The rule reports every other `require()` call,
    /// including top-level calls inside objects, arrays, `export` declarations, or loops.
    ///
    /// The rule doesn't report `require` when it's a variable, parameter, or function declared
    /// in your own code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function readFile(filename, callback) {
    ///     const fs = require("fs");
    ///     fs.readFile(filename, callback);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (DEBUG) {
    ///     require("debug");
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const getModule = (name) => require(name);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     require(unsafeModule);
    /// } catch (e) {
    ///     console.log(e);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// require("x");
    /// const y = require("y");
    /// let z;
    /// z = require("z").initialize();
    ///
    /// const fs = require("fs");
    /// function readFile(filename, callback) {
    ///     fs.readFile(filename, callback);
    /// }
    ///
    /// const logger = DEBUG ? require("dev-logger") : require("logger");
    ///
    /// function localRequire(require) {
    ///     require("y");
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow `require` entirely, see [`noCommonJs`](https://biomejs.dev/linter/rules/no-common-js/).
    ///
    pub UseTopLevelRequire {
        version: "next",
        name: "useTopLevelRequire",
        language: "js",
        sources: &[
            RuleSource::Eslint("global-require").same(),
            RuleSource::EslintN("global-require").same(),
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for UseTopLevelRequire {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseTopLevelRequireOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let reference = node
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_identifier_expression()?
            .name()
            .ok()?;
        if reference.value_token().ok()?.text_trimmed() != "require" {
            return None;
        }
        // A local binding named `require` isn't the module loader.
        if ctx.model().binding(&reference).is_some() {
            return None;
        }
        let is_top_level = node
            .syntax()
            .ancestors()
            .skip(1)
            .all(|ancestor| TOP_LEVEL_REQUIRE_ANCESTOR_KINDS.matches(ancestor.kind()));
        (!is_top_level).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"require()"</Emphasis>" call isn't directly part of a top-level statement."
                },
            )
            .note(markup! {
                "Modules loaded inside functions, blocks, or other expressions make it hard to see what the file depends on, and can be loaded at unexpected times."
            })
            .note(markup! {
                "Load the module in a top-level declaration, for example "<Emphasis>"const name = require(\"module\")"</Emphasis>", and use that variable here."
            }),
        )
    }
}

/// The nodes that may surround a `require()` call that is still at the top level of the
/// module: the module or script root, statements, variable declarations, assignments,
/// conditional expressions, calls, member accesses and member assignment targets, parentheses, and TypeScript expressions
/// that only add type information.
const TOP_LEVEL_REQUIRE_ANCESTOR_KINDS: SyntaxKindSet<JsLanguage> = JsModule::KIND_SET
    .union(JsScript::KIND_SET)
    .union(JsModuleItemList::KIND_SET)
    .union(JsStatementList::KIND_SET)
    .union(JsExpressionStatement::KIND_SET)
    .union(JsVariableStatement::KIND_SET)
    .union(JsVariableDeclaration::KIND_SET)
    .union(JsVariableDeclaratorList::KIND_SET)
    .union(JsVariableDeclarator::KIND_SET)
    .union(JsInitializerClause::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsConditionalExpression::KIND_SET)
    .union(JsCallExpression::KIND_SET)
    .union(JsCallArguments::KIND_SET)
    .union(JsCallArgumentList::KIND_SET)
    .union(JsStaticMemberExpression::KIND_SET)
    .union(JsComputedMemberExpression::KIND_SET)
    .union(JsStaticMemberAssignment::KIND_SET)
    .union(JsComputedMemberAssignment::KIND_SET)
    .union(JsParenthesizedExpression::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET);
