use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsImportAssertionEntry, JsImport, JsImportDefaultClause, T};
use biome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, BatchMutationExt, TriviaPieceKind,
};
use biome_rule_options::use_json_import_attributes::UseJsonImportAttributesOptions;

use crate::{JsRuleAction, services::module_graph::ResolvedImports};

// Define an enum to represent the specific reason for the lint violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseJsonImportAttributeState {
    /// The import statement has no `with` clause at all.
    NoAssertionClause,
    /// The import statement has a `with` clause, but it's missing `type: "json"`.
    MissingTypeJsonInAssertion,
}

declare_lint_rule! {
    /// Enforces the use of `with { type: "json" }` for JSON module imports.
    ///
    /// ECMAScript modules can import JSON modules. However, the specific import assertion `with { type: "json" }`
    /// is required to inform the JavaScript runtime that the imported file should be parsed as JSON.
    /// Omitting this assertion can lead to runtime errors or misinterpretation of the imported module.
    ///
    /// This rule ensures that all imports of `.json` files include this assertion.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import jsonData from './data.json';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import jsonData from './data.json' with { someOtherAttribute: "value" };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import jsonData from './data.json' with { type: "json" };
    ///
    /// import jsonData from './data.json' with { type: "json", other: "value" };
    ///
    /// import code from './script.js'; // Not a JSON import
    /// ```
    ///
    pub UseJsonImportAttributes {
        version: "2.0.0",
        name: "useJsonImportAttributes",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for UseJsonImportAttributes {
    type Query = ResolvedImports<JsImportDefaultClause>;
    type State = UseJsonImportAttributeState;
    type Signals = Option<Self::State>;
    type Options = UseJsonImportAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source = node.source().ok()?;
        let source = source.as_js_module_source()?;
        let import_path = source.inner_string_text().ok()?;
        let import_path = import_path.text();

        // Only proceed if it's a JSON file
        if !import_path.ends_with(".json") {
            return None;
        }

        if let Some(assertion_clause) = node.assertion() {
            // There is a 'with {...}' clause.
            // Check if it contains 'type: "json"'.
            let mut found_type_json = false;
            let assertions = assertion_clause.assertions();
            for entry in assertions.into_iter().flatten() {
                if let AnyJsImportAssertionEntry::JsImportAssertionEntry(entry) = entry
                    && let Ok(key) = entry.key()
                    && key.text_trimmed() == "type"
                    && let Ok(value) = entry.value_token()
                {
                    if value.text_trimmed() == "json" {
                        found_type_json = true;
                        break;
                    } else {
                        return None; // A manual type other than "json" is present.
                    }
                }
            }

            if found_type_json {
                None // Correct 'with { type: "json" }' is present.
            } else {
                // 'with' clause is present, but 'type: "json"' is missing.
                Some(UseJsonImportAttributeState::MissingTypeJsonInAssertion)
            }
        } else {
            // No 'with {...}' clause at all.
            Some(UseJsonImportAttributeState::NoAssertionClause)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let parent_import = node.syntax().parent().and_then(JsImport::cast)?;

        match state {
            UseJsonImportAttributeState::NoAssertionClause => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    // Highlight the entire import statement.
                    parent_import.range(),
                    markup! {
                        "This JSON import is missing the "<Emphasis>"type: \"json\""</Emphasis>" import attribute."
                    },
                )
                .note(markup! {
                    "To explicitly declare the module type for JSON imports, add "<Emphasis>"with { type: \"json\" }"</Emphasis>" to this import statement."
                }),
            ),
            UseJsonImportAttributeState::MissingTypeJsonInAssertion => {
                // Highlight the existing 'with {...}' clause if possible, otherwise the whole import.
                let range = {
                    let assertion = node.assertion();
                    if let Some(assertion) = assertion {
                        assertion.range()
                    } else {
                        parent_import.range()
                    }
                };

                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "The import attributes for this JSON module are missing "<Emphasis>"type: \"json\""</Emphasis>"."
                        },
                    )
                    .note(markup! {
                        "Ensure the "<Emphasis>"with"</Emphasis>" clause includes "<Emphasis>"type: \"json\""</Emphasis>" for this JSON import."
                    }),
                )
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        let mut new_source = None;

        // Create the new type: "json" entry
        let entry = make::js_import_assertion_entry(
            make::ident("type"),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::js_string_literal("json"),
        );
        let entry = AnyJsImportAssertionEntry::JsImportAssertionEntry(entry);

        let assertion = match state {
            UseJsonImportAttributeState::NoAssertionClause => {
                // Replace the source trailing trivia with a space, and extract the existing trailing trivia
                // to ensure things like comments are preserved
                let prev_source = node.source().ok()?;
                let source = prev_source.clone();

                let js_module_source = source.as_js_module_source()?;
                let value_token = js_module_source.value_token().ok()?;
                let trailing_trivia = value_token.trailing_trivia();

                new_source = Some(source.with_trailing_trivia_pieces([])?);

                // Create a new assertion block from scratch
                make::js_import_assertion(
                    make::token(T![with])
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::js_import_assertion_entry_list([entry], []),
                    make::token(T!['}'])
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                        .with_trailing_trivia_pieces(trailing_trivia.pieces())
                        .trim_trailing_trivia(),
                )
            }
            UseJsonImportAttributeState::MissingTypeJsonInAssertion => {
                let prev_assertion = node.assertion()?;
                let prev_assertions = prev_assertion.assertions();

                let mut items = vec![entry];
                let mut separators = vec![];

                // if there is more than 1 assertion, we need to add a comma after the first attribute
                if prev_assertions.len() > 0 {
                    separators.push(
                        make::token(T![,])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    );
                }

                for AstSeparatedElement {
                    node,
                    trailing_separator,
                } in prev_assertions.elements()
                {
                    if let Ok(node) = node {
                        items.push(node);
                    }
                    if let Ok(Some(trailing_separator)) = trailing_separator {
                        separators.push(trailing_separator);
                    }
                }

                prev_assertion
                    .with_assertions(make::js_import_assertion_entry_list(items, separators))
            }
        };

        let mut new_node = node.clone().with_assertion(Some(assertion));
        if let Some(new_source) = new_source {
            new_node = new_node.with_source(new_source);
        }

        mutation.replace_node(node.clone(), new_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"'type: \"json\"'"</Emphasis>" import attribute." }
                .to_owned(),
            mutation,
        ))
    }
}
