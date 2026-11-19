use std::sync::Arc;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsMemberExpression, AnyJsModuleItem,
    AnyJsNamedImportSpecifier, AnyJsObjectBindingPatternMember, AnyJsRoot, JsImport,
    JsInitializerClause, JsModuleItemList, JsNamedImportSpecifierList, JsUnaryExpression,
    JsUnaryOperator, JsVariableDeclarator, T, global_identifier,
};
use biome_package::PackageJson;
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxElement,
    TriviaPieceKind,
};
use biome_rule_options::no_convex_process_env::NoConvexProcessEnvOptions;
use camino::{Utf8Path, Utf8PathBuf};

use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use crate::shared::node_process::is_process_module_import;
use crate::utils::import::append_specifier;
use crate::utils::trivia::{attached_trivia, detached_trivia};

declare_lint_rule! {
    /// Disallow `process.env` in Convex functions.
    ///
    /// Convex can give you a typed `env` object for your environment variables. To get it,
    /// declare the variables in `convex/convex.config.ts`, and then import `env` from
    /// `_generated/server`. With `env`, TypeScript reports misspelled variable names,
    /// tells you which variables are optional, and a deploy fails early when a required
    /// variable is missing. With `process.env`, every variable has the type
    /// `string | undefined`, and misspelled names are only found when the code runs.
    ///
    /// This rule only checks files inside the `convex` folder of a package, which is the
    /// folder next to its `package.json`. Other code, such as frontend code, keeps using
    /// `process.env`. Files inside the `convex/_generated` folder are not checked.
    ///
    /// The fix replaces `process.env` with `env`, and imports `env` from `_generated/server`
    /// when the file doesn't import it yet. The fix is unsafe, because the code only
    /// compiles once the variable is declared in `convex/convex.config.ts`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": { "convex": "*" }
    /// }
    /// ```
    ///
    /// ```ts,file=convex/giphy.ts,expect_diagnostic
    /// const apiKey = process.env.GIPHY_KEY;
    /// ```
    ///
    /// ```ts,file=convex/keys.ts,expect_diagnostic
    /// const { GIPHY_KEY } = process.env;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": { "convex": "*" }
    /// }
    /// ```
    ///
    /// ```ts,file=convex/giphy.ts
    /// import { env } from "./_generated/server";
    ///
    /// const apiKey = env.GIPHY_KEY;
    /// ```
    ///
    /// The variable is declared in `convex/convex.config.ts`:
    ///
    /// ```ts,file=convex/convex.config.ts
    /// import { defineApp } from "convex/server";
    /// import { v } from "convex/values";
    ///
    /// const app = defineApp({
    ///   env: { GIPHY_KEY: v.string() },
    /// });
    ///
    /// export default app;
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow `process.env` in every file, see [`noProcessEnv`](https://biomejs.dev/linter/rules/no-process-env/).
    ///
    pub NoConvexProcessEnv {
        version: "next",
        name: "noConvexProcessEnv",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-process-env").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConvexProcessEnv {
    type Query = Semantic<AnyJsMemberExpression>;
    /// The number of folders between the `convex` folder and the file.
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoConvexProcessEnvOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.member_name()?.text() != "env" {
            return None;
        }
        let object = node.object().ok()?;
        let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
        if name.text() != "process" {
            return None;
        }
        if let Some(binding) = ctx.model().binding(&reference)
            && !is_process_module_import(&binding)
        {
            return None;
        }

        let (package_path, _) = ctx
            .get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()?
            .as_ref()?;
        convex_directory_depth(ctx.file_path(), package_path)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"process.env"</Emphasis>" in Convex functions."
                },
            )
            .note(markup! {
                "Variables read from "<Emphasis>"process.env"</Emphasis>" have no types, so misspelled or missing variables are only found when the code runs."
            })
            .note(markup! {
                "Declare the environment variables in "<Emphasis>"convex/convex.config.ts"</Emphasis>", and read them from the "<Emphasis>"env"</Emphasis>" object exported by "<Emphasis>"_generated/server"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, depth: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if !reads_named_variables(node) {
            return None;
        }
        let AnyJsRoot::JsModule(root) = ctx.root() else {
            return None;
        };
        let items = root.items();
        let process_env = AnyJsExpression::from(node.clone());
        let scope = ctx.model().scope(node.syntax());
        let mut mutation = ctx.root().begin();

        let env_import = items
            .iter()
            .filter_map(|item| generated_server_specifiers(&item))
            .flat_map(|specifiers| specifiers.iter())
            .filter_map(Result::ok)
            .find(|specifier| {
                specifier.type_token().is_none()
                    && specifier
                        .imported_name()
                        .is_some_and(|name| name.text_trimmed() == "env")
            });
        if let Some(specifier) = env_import {
            // Reuse the imported `env`, unless another variable with the same
            // name hides it where `process.env` is used.
            let local_name = specifier.local_name()?;
            let name = local_name.as_js_identifier_binding()?.name_token().ok()?;
            let binding = scope
                .ancestors()
                .find_map(|scope| scope.get_binding(name.text_trimmed()))?;
            if &binding.syntax() != local_name.syntax() {
                return None;
            }
            mutation.replace_node(process_env, env_reference(name.text_trimmed()));
        } else if scope
            .ancestors()
            .any(|scope| scope.get_binding("env").is_some())
        {
            // Importing `env` would conflict with the existing variable.
            return None;
        } else if let Some(specifiers) = items
            .iter()
            .find_map(|item| generated_server_specifiers(&item))
        {
            let new_specifiers = append_specifier(&specifiers, env_specifier())?;
            mutation.replace_node_discard_trivia(specifiers, new_specifiers);
            mutation.replace_node(process_env, env_reference("env"));
        } else {
            // The new import and the replaced `process.env` are both inside
            // the module items, so both changes must be made to the same list.
            let source = generated_server_source(*depth, uses_js_extensions(&items));
            let new_items = items.clone().replace_node(process_env, env_reference("env"))?;
            // A comment followed by a blank line at the start of the file,
            // such as a file header, stays above the new import. Other
            // comments describe the first item and stay attached to it.
            let first_item = new_items.first()?;
            let import = generated_server_import(ctx, &source);
            let import = match detached_trivia(first_item.syntax()) {
                Some(detached) => import.with_leading_trivia_pieces(detached)?,
                None => import,
            };
            let first_item = match attached_trivia(first_item.syntax()) {
                Some(attached) => first_item.with_leading_trivia_pieces(attached)?,
                None => first_item,
            };
            let new_items = JsModuleItemList::cast(new_items.into_syntax().splice_slots(
                0..1,
                [
                    Some(SyntaxElement::Node(import.into_syntax())),
                    Some(SyntaxElement::Node(first_item.into_syntax())),
                ],
            ))?;
            mutation.replace_element_discard_trivia(
                items.into_syntax().into(),
                new_items.into_syntax().into(),
            );
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use the "<Emphasis>"env"</Emphasis>" object from "<Emphasis>"_generated/server"</Emphasis>" instead."
            },
            mutation,
        ))
    }
}

/// Returns the number of folders between the `convex` folder of the package at
/// `package_path` and the file at `path`.
///
/// Returns `None` when the file isn't inside the `convex` folder, or is inside
/// `convex/_generated`. Convex writes `export const env = process.env` into
/// `convex/_generated/server`, so generated files must not be checked.
fn convex_directory_depth(path: &Utf8Path, package_path: &Utf8Path) -> Option<usize> {
    let mut directories = path
        .parent()?
        .strip_prefix(package_path)
        .ok()?
        .components();
    if directories.next()?.as_str() != "convex" {
        return None;
    }
    let mut depth = 0;
    for directory in directories {
        if directory.as_str() == "_generated" {
            return None;
        }
        depth += 1;
    }
    Some(depth)
}

/// Returns `true` if `process_env` is used to read variables by name, so that
/// reading them from `env` instead keeps the code working.
///
/// This is the case for `process.env.NAME`, `process.env["NAME"]`, and
/// `const { NAME } = process.env`. Other uses, such as `delete process.env.NAME`,
/// `process.env[name]`, or passing the whole object, have no equivalent on `env`,
/// which only holds the declared variables.
fn reads_named_variables(process_env: &AnyJsMemberExpression) -> bool {
    let Some(parent) = process_env.syntax().parent() else {
        return false;
    };
    if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
        let is_object = member
            .object()
            .is_ok_and(|object| object.syntax() == process_env.syntax());
        let is_delete_operand = AnyJsExpression::from(member.clone())
            .outer_expression()
            .and_then(|expression| expression.parent::<JsUnaryExpression>())
            .is_some_and(|unary| matches!(unary.operator(), Ok(JsUnaryOperator::Delete)));
        return is_object && member.member_name().is_some() && !is_delete_operand;
    }
    let Some(declarator) =
        JsInitializerClause::cast(parent).and_then(|clause| clause.parent::<JsVariableDeclarator>())
    else {
        return false;
    };
    let Ok(AnyJsBindingPattern::JsObjectBindingPattern(pattern)) = declarator.id() else {
        return false;
    };
    let properties = pattern.properties();
    !properties.is_empty()
        && properties.iter().all(|property| match property {
            Ok(AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(_)) => {
                true
            }
            Ok(AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property)) => {
                property.member().is_ok_and(|member| member.name().is_some())
            }
            _ => false,
        })
}

/// Returns the named specifiers of `item` if it imports values from the
/// generated `_generated/server` module, as in
/// `import { query } from "./_generated/server"`.
fn generated_server_specifiers(item: &AnyJsModuleItem) -> Option<JsNamedImportSpecifierList> {
    let clause = JsImport::cast_ref(item.syntax())?.import_clause().ok()?;
    if clause.type_token().is_some() {
        return None;
    }
    let source = clause.source().ok()?.inner_string_text().ok()?;
    let source = source.text();
    let module = source
        .strip_suffix(".js")
        .or_else(|| source.strip_suffix(".ts"))
        .unwrap_or(source);
    let is_generated_server = module
        .strip_suffix("_generated/server")
        .is_some_and(|prefix| prefix.is_empty() || prefix.ends_with('/'));
    if !is_generated_server {
        return None;
    }
    Some(clause.named_specifiers()?.specifiers())
}

/// Returns `true` if the file imports other files with a `.js` extension, as
/// TypeScript requires with the `node16` and `nodenext` module resolutions.
fn uses_js_extensions(items: &JsModuleItemList) -> bool {
    items.iter().any(|item| {
        JsImport::cast(item.into_syntax())
            .and_then(|import| import.source_text().ok())
            .is_some_and(|source| source.text().starts_with('.') && source.text().ends_with(".js"))
    })
}

/// Returns the import source of `_generated/server` for a file that is `depth`
/// folders below the `convex` folder.
fn generated_server_source(depth: usize, js_extension: bool) -> String {
    let prefix = if depth == 0 {
        "./".to_string()
    } else {
        "../".repeat(depth)
    };
    let extension = if js_extension { ".js" } else { "" };
    format!("{prefix}_generated/server{extension}")
}

/// Returns `import { env } from "<source>";`, followed by a newline.
fn generated_server_import(ctx: &RuleContext<NoConvexProcessEnv>, source: &str) -> AnyJsModuleItem {
    let source = if ctx.preferred_quote().is_double() {
        make::js_string_literal(source)
    } else {
        make::js_string_literal_single_quotes(source)
    };
    let clause = make::js_import_named_clause(
        make::js_named_import_specifiers(
            make::token(T!['{']).with_trailing_space(),
            make::js_named_import_specifier_list([env_specifier()], []),
            make::token(T!['}']).with_surrounding_spaces(),
        ),
        make::token(T![from]).with_trailing_space(),
        make::js_module_source(source).into(),
    )
    .build();
    make::js_import(make::token(T![import]).with_trailing_space(), clause.into())
        .with_semicolon_token(
            make::token(T![;]).with_trailing_trivia([(TriviaPieceKind::Newline, "\n")]),
        )
        .build()
        .into()
}

/// Returns the `env` specifier of `import { env }`.
fn env_specifier() -> AnyJsNamedImportSpecifier {
    make::js_shorthand_named_import_specifier(make::js_identifier_binding(make::ident("env")).into())
        .build()
        .into()
}

/// Returns a reference to the variable `name`.
fn env_reference(name: &str) -> AnyJsExpression {
    make::js_identifier_expression(make::js_reference_identifier(make::ident(name))).into()
}
