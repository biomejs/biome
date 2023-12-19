use crate::semantic_services::{Semantic, SemanticServices};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::{CanBeImportedExported, Reference, ReferencesExtensions};
use biome_js_syntax::{JsCallExpression, JsImportCallExpression, JsModuleSource, TextRange};
use biome_rowan::{declare_node_union, AstNode};
use rustc_hash::FxHashMap;

declare_node_union! {
    pub(crate) AnyJsImportLike = JsModuleSource | JsCallExpression | JsImportCallExpression
}

declare_rule! {
    /// Disallow duplicate module imports
    ///
    /// Using a single import statement per module will make the code clearer because you can see everything being imported from that module on one line.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-duplicate-imports
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { merge } from 'module';
    /// import something from 'another-module';
    /// import { find } from 'module';
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// import { merge, find } from 'module';
    /// import something from 'another-module';
    /// ```
    ///
    pub(crate) NoDuplicateImports {
        version: "next",
        name: "noDuplicateImports",
        recommended: false,
    }
}

#[derive(Debug)]
pub(crate) struct Redeclaration {
    name: String,
    declaration: TextRange,
    redeclaration: TextRange,
}

impl Rule for NoDuplicateImports {
    type Query = SemanticServices;
    type State = Redeclaration;
    type Signals = Vec<Redeclaration>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut redeclarations: Vec<Redeclaration> = Vec::new();
        let scope = ctx.query().global_scope();
        let mut declarations = FxHashMap::<String, (TextRange, AnyJsImportLike)>::default();

        // Is this the correct way to iterate through global scope import/require statements?
        for binding in scope.bindings() {
            let tree = binding.tree(); // ???

            match tree {
                AnyJsImportLike::JsModuleSource(source) => {
                    // call update_declarations in here.
                }
                AnyJsImportLike::JsCallExpression(expression) => {
                    // call update_declarations in here.
                }
                AnyJsImportLike::JsImportCallExpression(expr) => {
                    // call update_declarations in here.
                }
            }
        }
        redeclarations
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Redeclaration {
            name,
            declaration,
            redeclaration,
        } = state;
        let diag = RuleDiagnostic::new(
            rule_category!(),
            redeclaration,
            markup! {
               "Shouldn't redeclare '"{ name }"'. Consider to delete it or rename it."
            },
        )
        .detail(
            declaration,
            markup! {
               "'"{ name }"' is defined here:"
            },
        );
        Some(diag)
    }
}

fn update_declarations(
    redeclarations: &mut Vec<Redeclaration>,
    declarations: &mut FxHashMap<String, (TextRange, AnyJsImportLike)>,
    binding: &AnyJsImportLike,
) {
    if let Some(declaration) = declarations.get(&binding.name()) {
        redeclarations.push(Redeclaration {
            name: binding.name().to_string(),
            declaration: declaration.0,
            redeclaration: binding.range(),
        });
    } else {
        declarations.insert(
            binding.name().to_string(),
            (binding.range(), binding.clone()),
        );
    }
}
