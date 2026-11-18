use std::sync::Arc;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsNamedImportSpecifier, AnyJsRoot,
    JsIdentifierExpression, JsImport, JsNamedImportSpecifierList, JsReferenceIdentifier,
    JsUnaryExpression, JsUnaryOperator, T,
};
use biome_package::PackageJson;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_test_globals::NoTestGlobalsOptions;
use camino::Utf8PathBuf;
use enumflags2::{BitFlags, bitflags, make_bitflags};

use crate::{
    JsRuleAction, lint::style::use_import_type::add_module_items, services::semantic::Semantic,
};

declare_lint_rule! {
    /// Disallow using test APIs as global variables.
    ///
    /// Test runners such as Jest and Vitest can provide `describe`, `it`, `expect`, and their other test APIs as global variables.
    /// Whether these global variables exist depends on the configuration of the test runner, and nothing in a test file shows where they come from.
    /// Importing the test APIs works with any configuration.
    ///
    /// The rule reports a test API that a file uses without importing or declaring it.
    /// It ignores a test API that is only used as a type, or only as the operand of `typeof`, as in `typeof jest === "undefined"`.
    ///
    /// Biome reads the nearest `package.json` file to choose the test runner:
    ///
    /// - If `vitest` is listed as a dependency, the rule checks the global variables of Vitest, which are imported from `vitest`.
    /// - Otherwise, the rule checks the global variables of Jest, which are imported from `@jest/globals`.
    ///
    /// The fix adds an `import` declaration.
    /// It is unsafe because Biome can't know whether the test runner loads the file as an ES module, which is the only kind of file that allows `import` declarations.
    /// Biome doesn't offer the fix in files that it treats as CommonJS, such as `.cjs` files.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe("sum", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// jest.useFakeTimers();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { describe, expect, it } from "@jest/globals";
    ///
    /// describe("sum", () => {
    ///     it("adds two numbers", () => {
    ///         expect(1 + 2).toBe(3);
    ///     });
    /// });
    ///
    /// if (typeof jest !== "undefined") {
    ///     console.log("Running inside Jest.");
    /// }
    /// ```
    ///
    pub NoTestGlobals {
        version: "next",
        name: "noTestGlobals",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-importing-jest-globals").inspired(),
            RuleSource::EslintVitest("prefer-importing-vitest-globals").inspired(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoTestGlobals {
    type Query = Semantic<JsReferenceIdentifier>;
    /// The module that exports the test API.
    type State = TestModule;
    type Signals = Option<Self::State>;
    type Options = NoTestGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        let name = reference.value_token().ok()?;
        let index = TEST_GLOBALS
            .binary_search_by(|(global, _)| (*global).cmp(name.text_trimmed()))
            .ok()?;
        if ctx.model().binding(reference).is_some() || !needs_global_variable(reference) {
            return None;
        }

        let (_, modules) = TEST_GLOBALS[index];
        let module = TestModule::from_context(ctx);
        modules.contains(module).then_some(module)
    }

    fn diagnostic(ctx: &RuleContext<Self>, module: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query().value_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The test API "<Emphasis>{name}</Emphasis>" is used as a global variable."
                },
            )
            .note(markup! {
                "Whether this global variable exists depends on the configuration of the test runner. An import shows where the test API comes from and works with any configuration."
            })
            .note(markup! {
                "Import "<Emphasis>{name}</Emphasis>" from "<Emphasis>{module}</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, module: &Self::State) -> Option<JsRuleAction> {
        let AnyJsRoot::JsModule(root) = ctx.root() else {
            return None;
        };
        let name = ctx.query().value_token().ok()?;
        let name = name.text_trimmed();
        let specifier = AnyJsNamedImportSpecifier::from(
            make::js_shorthand_named_import_specifier(
                make::js_identifier_binding(make::ident(name)).into(),
            )
            .build(),
        );

        let items = root.items();
        let mut mutation = ctx.root().begin();
        let specifiers = items
            .iter()
            .filter_map(|item| JsImport::cast(item.into_syntax()))
            .find_map(|import| named_value_specifiers(&import, *module));
        if let Some(specifiers) = specifiers {
            let new_specifiers = append_specifier(&specifiers, specifier)?;
            mutation.replace_node_discard_trivia(specifiers, new_specifiers);
        } else {
            // `add_module_items` moves the leading trivia of the first item,
            // such as a file header comment, to the new import.
            let first_item = items.first()?;
            let module_name = if ctx.preferred_quote().is_double() {
                make::js_string_literal(module.as_str())
            } else {
                make::js_string_literal_single_quotes(module.as_str())
            };
            let clause = make::js_import_named_clause(
                make::js_named_import_specifiers(
                    make::token(T!['{']).with_trailing_space(),
                    make::js_named_import_specifier_list([specifier], []),
                    make::token(T!['}']).with_surrounding_spaces(),
                ),
                make::token(T![from]).with_trailing_space(),
                make::js_module_source(module_name).into(),
            )
            .build();
            let import = make::js_import(make::token(T![import]).with_trailing_space(), clause.into())
                .with_semicolon_token(
                    make::token(T![;]).with_trailing_trivia([(TriviaPieceKind::Newline, "\n")]),
                )
                .build();
            add_module_items(
                &mut mutation,
                first_item.syntax(),
                [
                    import.into(),
                    first_item.clone().with_leading_trivia_pieces([])?,
                ],
            );
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Import "<Emphasis>{name}</Emphasis>" from "<Emphasis>{module}</Emphasis>"."
            },
            mutation,
        ))
    }
}

/// A module that exports the test APIs that a test runner can also provide as
/// global variables.
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestModule {
    JestGlobals = 1 << 0,
    Vitest = 1 << 1,
}

impl TestModule {
    /// Returns the module of the test runner that the nearest `package.json`
    /// depends on.
    ///
    /// Falls back to Jest when Vitest isn't a dependency. A project can run
    /// Jest without depending on the `jest` package, because wrappers such as
    /// `react-scripts` bring it in.
    fn from_context(ctx: &RuleContext<NoTestGlobals>) -> Self {
        let depends_on_vitest = ctx
            .get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
            .and_then(|manifest| manifest.as_ref())
            .is_some_and(|(_, manifest)| manifest.contains_dependency("vitest"));
        if depends_on_vitest {
            Self::Vitest
        } else {
            Self::JestGlobals
        }
    }

    /// Returns the name to import the module with.
    const fn as_str(self) -> &'static str {
        match self {
            Self::JestGlobals => "@jest/globals",
            Self::Vitest => "vitest",
        }
    }
}

impl biome_console::fmt::Display for TestModule {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.as_str())
    }
}

const JEST: BitFlags<TestModule> = make_bitflags!(TestModule::{JestGlobals});
const VITEST: BitFlags<TestModule> = make_bitflags!(TestModule::{Vitest});
const JEST_AND_VITEST: BitFlags<TestModule> = make_bitflags!(TestModule::{JestGlobals | Vitest});

/// The test APIs that a test runner can provide as global variables, with the
/// modules that export each of them.
///
/// Sorted by name, so that it can be searched with a binary search.
const TEST_GLOBALS: &[(&str, BitFlags<TestModule>)] = &[
    ("afterAll", JEST_AND_VITEST),
    ("afterEach", JEST_AND_VITEST),
    ("aroundAll", VITEST),
    ("aroundEach", VITEST),
    ("assert", VITEST),
    ("assertType", VITEST),
    ("beforeAll", JEST_AND_VITEST),
    ("beforeEach", JEST_AND_VITEST),
    ("chai", VITEST),
    ("describe", JEST_AND_VITEST),
    ("expect", JEST_AND_VITEST),
    ("expectTypeOf", VITEST),
    ("fdescribe", JEST),
    ("fit", JEST),
    ("it", JEST_AND_VITEST),
    ("jest", JEST),
    ("onTestFailed", VITEST),
    ("onTestFinished", VITEST),
    ("suite", VITEST),
    ("test", JEST_AND_VITEST),
    ("vi", VITEST),
    ("vitest", VITEST),
    ("xdescribe", JEST),
    ("xit", JEST),
    ("xtest", JEST),
];

/// Returns `true` if evaluating `reference` fails when no global variable has
/// its name.
///
/// A reference that is only used as a type is erased before the code runs,
/// and `typeof` accepts a name that doesn't exist.
fn needs_global_variable(reference: &JsReferenceIdentifier) -> bool {
    if AnyJsIdentifierReference::from(reference.clone()).is_only_type() {
        return false;
    }
    let is_typeof_operand = reference
        .parent::<JsIdentifierExpression>()
        .and_then(|expression| AnyJsExpression::from(expression).outer_expression())
        .and_then(|expression| expression.parent::<JsUnaryExpression>())
        .is_some_and(|unary| matches!(unary.operator(), Ok(JsUnaryOperator::Typeof)));
    !is_typeof_operand
}

/// Returns the named specifiers of `import` if it imports values from `module`,
/// as in `import { describe } from "vitest"`.
fn named_value_specifiers(
    import: &JsImport,
    module: TestModule,
) -> Option<JsNamedImportSpecifierList> {
    let clause = import.import_clause().ok()?;
    if clause.type_token().is_some() {
        return None;
    }
    let source = clause.source().ok()?.inner_string_text().ok()?;
    if source.text() != module.as_str() {
        return None;
    }
    Some(clause.named_specifiers()?.specifiers())
}

/// Returns a copy of `specifiers` with `specifier` added after the last
/// specifier.
///
/// The new specifier takes over the trailing trivia of the specifier it
/// follows, and a trailing comma stays at the end of the list.
fn append_specifier(
    specifiers: &JsNamedImportSpecifierList,
    specifier: AnyJsNamedImportSpecifier,
) -> Option<JsNamedImportSpecifierList> {
    let Some(last) = specifiers.last() else {
        return Some(make::js_named_import_specifier_list([specifier], []));
    };
    let last = last.ok()?;
    let index = last.syntax().index();
    let specifier =
        specifier.with_trailing_trivia_pieces(last.syntax().last_trailing_trivia()?.pieces())?;
    let last = last.with_trailing_trivia_pieces([])?;
    JsNamedImportSpecifierList::cast(specifiers.syntax().clone().splice_slots(
        index..=index,
        [
            Some(last.into_syntax().into()),
            Some(make::token(T![,]).with_trailing_space().into()),
            Some(specifier.into_syntax().into()),
        ],
    ))
}

#[cfg(test)]
mod tests {
    use super::TEST_GLOBALS;

    #[test]
    fn test_globals_are_sorted() {
        for pair in TEST_GLOBALS.windows(2) {
            assert!(
                pair[0].0.cmp(pair[1].0).is_lt(),
                "{} must come before {}",
                pair[1].0,
                pair[0].0
            );
        }
    }
}
