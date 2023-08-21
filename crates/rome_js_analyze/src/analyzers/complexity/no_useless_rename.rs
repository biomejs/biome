use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsExportNamedFromSpecifier, JsExportNamedSpecifier, JsNamedImportSpecifier,
    JsObjectBindingPatternProperty, JsSyntaxElement,
};
use rome_rowan::{declare_node_union, trim_leading_trivia_pieces, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow renaming import, export, and destructured assignments to the same name.
    ///
    /// ES2015 allows for the renaming of references in import and export statements as well as destructuring assignments.
    /// This gives programmers a concise syntax for performing these operations while renaming these references:
    ///
    /// ```js
    /// import { foo as bar } from "baz";
    /// export { foo as bar };
    /// let { foo: bar } = baz;
    /// ```
    ///
    /// With this syntax, it is possible to rename a reference to the same name.
    /// This is a completely redundant operation, as this is the same as not renaming at all.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-useless-rename
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { foo as foo } from "bar";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export { foo as foo };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let { foo: foo } = bar;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { foo as bar } from "baz";
    /// ```
    ///
    /// ```js
    /// export { foo as bar };
    /// ```
    ///
    /// ```js
    /// let { foo: bar } = baz;
    /// ```
    ///
    pub(crate) NoUselessRename {
        version: "1.0.0",
        name: "noUselessRename",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) JsRenaming = JsExportNamedFromSpecifier | JsExportNamedSpecifier | JsNamedImportSpecifier | JsObjectBindingPatternProperty
}

impl Rule for NoUselessRename {
    type Query = Ast<JsRenaming>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let renaming = ctx.query();
        let (old_name, new_name) = match renaming {
            JsRenaming::JsExportNamedFromSpecifier(x) => (
                x.source_name().ok()?.value().ok()?,
                x.export_as()?.exported_name().ok()?.value().ok()?,
            ),
            JsRenaming::JsExportNamedSpecifier(x) => (
                x.local_name().ok()?.value_token().ok()?,
                x.exported_name().ok()?.value().ok()?,
            ),
            JsRenaming::JsNamedImportSpecifier(x) => (
                x.name().ok()?.value().ok()?,
                x.local_name()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?,
            ),
            JsRenaming::JsObjectBindingPatternProperty(x) => (
                x.member().ok()?.as_js_literal_member_name()?.value().ok()?,
                x.pattern()
                    .ok()?
                    .as_any_js_binding()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?,
            ),
        };
        (old_name.text_trimmed() == new_name.text_trimmed()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let renaming = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            renaming.syntax().text_trimmed_range(),
            markup! {
                "Useless rename."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let renaming = ctx.query();
        let mut mutation = ctx.root().begin();
        match renaming {
            JsRenaming::JsExportNamedFromSpecifier(x) => {
                let last_token = x.source_name().ok()?.value().ok()?;
                let export_as = x.export_as()?;
                let export_as_last_token = export_as.exported_name().ok()?.value().ok()?;
                let replacing_token = last_token.append_trivia_pieces(trim_leading_trivia_pieces(
                    export_as_last_token.trailing_trivia().pieces(),
                ));
                mutation.remove_node(export_as);
                mutation.replace_token_discard_trivia(last_token, replacing_token);
            }
            JsRenaming::JsExportNamedSpecifier(x) => {
                let replacing =
                    make::js_export_named_shorthand_specifier(x.local_name().ok()?).build();
                mutation.replace_element(
                    JsSyntaxElement::Node(x.syntax().clone()),
                    JsSyntaxElement::Node(replacing.syntax().clone()),
                );
            }
            JsRenaming::JsNamedImportSpecifier(x) => {
                let replacing =
                    make::js_shorthand_named_import_specifier(x.local_name().ok()?).build();
                mutation.replace_element(
                    JsSyntaxElement::Node(x.syntax().clone()),
                    JsSyntaxElement::Node(replacing.syntax().clone()),
                );
            }
            JsRenaming::JsObjectBindingPatternProperty(x) => {
                let mut replacing_builder = make::js_object_binding_pattern_shorthand_property(
                    x.pattern().ok()?.as_any_js_binding()?.clone(),
                );
                if let Some(init) = x.init() {
                    replacing_builder = replacing_builder.with_init(init);
                }
                mutation.replace_element(
                    JsSyntaxElement::Node(x.syntax().clone()),
                    JsSyntaxElement::Node(replacing_builder.build().syntax().clone()),
                );
            }
        }
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove the renaming." }.to_owned(),
            mutation,
        })
    }
}
