use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsName, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::declare_node_union;
use biome_rule_options::no_dangling_underscore::NoDanglingUnderscoreOptions;

declare_lint_rule! {
    /// Disallow names that start or end with an underscore.
    ///
    /// An underscore at the start or end of a name, as in `_count` or `count_`,
    /// is often used to say that something is private and shouldn't be used
    /// outside of its own code. The underscore doesn't change how the code
    /// works: a property named `_id` can still be read and changed from
    /// anywhere. Readers may wrongly assume the underscore has some meaning
    /// to the language.
    ///
    /// This rule reports:
    ///
    /// - variable names, such as `const _count = 0`;
    /// - names of function declarations, such as `function _init() {}`;
    /// - property names written after a dot, such as `user._id` or `this._count`.
    ///
    /// These names are allowed:
    ///
    /// - `_` on its own, which is often the name of a utility library;
    /// - `__proto__` when written after a dot;
    /// - variables created by destructuring, such as `const { _id } = user`;
    /// - function parameters and names of function expressions;
    /// - names written where a class member or object key is declared, such as
    ///   `class A { _reset() {} }`;
    /// - private class members, whose names start with `#`, such as `this.#_count`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let _count = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function init_() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// user._id;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// this._count = 0;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let count = 0;
    /// const _ = require("underscore");
    /// const { _id } = user;
    /// function init(_options) {}
    /// const handler = function _handler() {};
    /// obj.__proto__;
    ///
    /// class Counter {
    ///     #count = 0;
    ///     _reset() {}
    /// }
    /// ```
    ///
    pub NoDanglingUnderscore {
        version: "next",
        name: "noDanglingUnderscore",
        language: "js",
        sources: &[RuleSource::Eslint("no-underscore-dangle").inspired()],
        recommended: false,
    }
}

declare_node_union! {
    pub AnyNoDanglingUnderscoreQuery =
        JsVariableDeclarator
        | JsFunctionDeclaration
        | JsFunctionExportDefaultDeclaration
        | JsStaticMemberExpression
        | JsStaticMemberAssignment
}

impl Rule for NoDanglingUnderscore {
    type Query = Ast<AnyNoDanglingUnderscoreQuery>;
    /// Where the underscores are in the reported name.
    type State = DanglingUnderscore;
    type Signals = Option<Self::State>;
    type Options = NoDanglingUnderscoreOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.name_token()?;
        let name = token.text_trimmed();
        if name == "_" || (node.is_member() && name == "__proto__") {
            return None;
        }
        match (name.starts_with('_'), name.ends_with('_')) {
            (true, true) => Some(DanglingUnderscore::Both),
            (true, false) => Some(DanglingUnderscore::Leading),
            (false, true) => Some(DanglingUnderscore::Trailing),
            (false, false) => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let token = ctx.query().name_token()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                token.text_trimmed_range(),
                markup! {
                    "The name "<Emphasis>{token.text_trimmed()}</Emphasis>" "{state}" with an underscore."
                },
            )
            .note(markup! {
                "An underscore at the start or end of a name is often used to mark something as private, but it doesn't change how the code works, so it can mislead readers."
            })
            .note(markup! {
                "Remove the underscore from the name. For a class member that must stay private, start its name with "<Emphasis>"#"</Emphasis>" instead."
            }),
        )
    }
}

impl AnyNoDanglingUnderscoreQuery {
    /// Returns the token of the name this rule checks, or `None` if the name
    /// isn't a plain identifier, for example a destructuring pattern or a
    /// private class member name.
    fn name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsVariableDeclarator(declarator) => match declarator.id().ok()? {
                AnyJsBindingPattern::AnyJsBinding(binding) => binding_name_token(&binding),
                _ => None,
            },
            Self::JsFunctionDeclaration(declaration) => binding_name_token(&declaration.id().ok()?),
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                binding_name_token(&declaration.id()?)
            }
            Self::JsStaticMemberExpression(member) => name_value_token(&member.member().ok()?),
            Self::JsStaticMemberAssignment(member) => name_value_token(&member.member().ok()?),
        }
    }

    const fn is_member(&self) -> bool {
        matches!(
            self,
            Self::JsStaticMemberExpression(_) | Self::JsStaticMemberAssignment(_)
        )
    }
}

fn binding_name_token(binding: &AnyJsBinding) -> Option<JsSyntaxToken> {
    binding.as_js_identifier_binding()?.name_token().ok()
}

fn name_value_token(name: &AnyJsName) -> Option<JsSyntaxToken> {
    name.as_js_name()?.value_token().ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DanglingUnderscore {
    Leading,
    Trailing,
    Both,
}

impl biome_console::fmt::Display for DanglingUnderscore {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Leading => "starts",
            Self::Trailing => "ends",
            Self::Both => "starts and ends",
        })
    }
}
