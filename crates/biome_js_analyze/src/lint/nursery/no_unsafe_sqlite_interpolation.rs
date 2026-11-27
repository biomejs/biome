use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, AnyJsTemplateElement,
    JsCallExpression, JsImport, JsReferenceIdentifier, binding_ext::AnyJsBindingDeclaration,
    inner_string_text, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange};
use biome_rule_options::no_unsafe_sqlite_interpolation::NoUnsafeSqliteInterpolationOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow inserting values into SQL strings passed to the `exec()` and `prepare()` methods of `node:sqlite` databases.
    ///
    /// The `DatabaseSync` class of the built-in `node:sqlite` module runs SQL code with its `exec()` and `prepare()` methods.
    /// When a template literal inserts a value with `${}`, the value becomes part of the SQL code itself.
    /// A value that contains SQL, such as text typed by a user, can then change what the query does.
    /// This is called SQL injection, and it can leak or destroy data.
    /// Even harmless values can break the query, for example a name that contains a quote.
    ///
    /// The rule also reports tagged templates, such as ``String.raw`SELECT 1` ``, passed to these methods.
    /// The tag function turns the template into a plain string before the method receives it,
    /// so the inserted values are never sent separately from the SQL code.
    ///
    /// Instead, write a `?` placeholder in the SQL string and pass the values when you run the prepared statement:
    ///
    /// ```js
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// const query = database.prepare("SELECT * FROM users WHERE id = ?");
    /// query.get(id);
    /// ```
    ///
    /// Starting from Node.js 24.9.0, you can also use the tagged templates of `createTagStore()`.
    /// They send each inserted value separately from the SQL code:
    ///
    /// ```js
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// const sql = database.createTagStore();
    /// sql.get`SELECT * FROM users WHERE id = ${id}`;
    /// ```
    ///
    /// The rule only checks databases created with `new` from the `DatabaseSync` class imported from `node:sqlite` with an `import` statement.
    /// It follows variables declared with `const`, such as `const sql = ...; database.exec(sql)`,
    /// but not variables declared with `let` or `var`, values returned by functions, or modules loaded with `require()`.
    /// It doesn't report SQL strings joined with `+`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// database.exec(`SELECT * FROM users WHERE id = ${id}`);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// const sql = `SELECT * FROM users WHERE id = ${id}`;
    /// database.prepare(sql);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// database.prepare(String.raw`SELECT * FROM users WHERE id = ${id}`);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { DatabaseSync } from "node:sqlite";
    ///
    /// const database = new DatabaseSync(":memory:");
    /// database.exec("CREATE TABLE users (id INTEGER)");
    /// database.exec(`SELECT 1`);
    /// database.prepare("SELECT * FROM users WHERE id = ?").get(id);
    /// ```
    ///
    pub NoUnsafeSqliteInterpolation {
        version: "2.6.9",
        name: "noUnsafeSqliteInterpolation",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unsafe-sqlite-interpolation").same()],
        recommended: false,
        severity: Severity::Error,
    }
}

/// An SQL string that a `node:sqlite` database receives unsafely.
pub struct UnsafeSqlCall {
    method: SqliteMethod,
    kind: UnsafeSqlKind,
    /// The range of the template that builds the SQL string. It lies outside
    /// the call argument when the argument is a `const` variable.
    template_range: TextRange,
}

impl Rule for NoUnsafeSqliteInterpolation {
    type Query = Semantic<JsCallExpression>;
    type State = UnsafeSqlCall;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeSqliteInterpolationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee = call.callee().ok()?.inner_expression()?;
        let callee = AnyJsMemberExpression::cast(callee.into_syntax())?;
        let method = match member_name(&callee, model)?.text() {
            "exec" => SqliteMethod::Exec,
            "prepare" => SqliteMethod::Prepare,
            _ => return None,
        };

        let sql = call.arguments().ok()?.args().first()?.ok()?;
        let sql = resolve_const_aliases(sql.as_any_js_expression()?, model)?;
        let template = sql.as_js_template_expression()?;
        let has_values = template
            .elements()
            .iter()
            .any(|element| matches!(element, AnyJsTemplateElement::JsTemplateElement(_)));
        let kind = match (template.tag().is_some(), has_values) {
            (true, true) => UnsafeSqlKind::TaggedTemplate,
            (true, false) => UnsafeSqlKind::TaggedTemplateWithoutValues,
            (false, true) => UnsafeSqlKind::Interpolation,
            (false, false) => return None,
        };

        if !is_database_sync_instance(&callee.object().ok()?, model) {
            return None;
        }

        Some(UnsafeSqlCall {
            method,
            kind,
            template_range: template.range(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let sql = ctx.query().arguments().ok()?.args().first()?.ok()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            sql.range(),
            markup! {
                "This SQL string passed to "<Emphasis>{state.method}</Emphasis>" "{state.kind}"."
            },
        );
        if !sql.range().contains_range(state.template_range) {
            diagnostic = diagnostic.detail(
                state.template_range,
                markup! { "The SQL string is built here." },
            );
        }
        Some(
            diagnostic
                .note(match state.kind {
                    UnsafeSqlKind::Interpolation => markup! {
                        "Inserted values become part of the SQL code, so a value that contains SQL can change what the query does. This is called SQL injection."
                    },
                    UnsafeSqlKind::TaggedTemplate => markup! {
                        "The tag function turns the template into a plain string before the database receives it, so the inserted values become part of the SQL code. A value that contains SQL can change what the query does. This is called SQL injection."
                    },
                    UnsafeSqlKind::TaggedTemplateWithoutValues => markup! {
                        "The tag function decides the text of the SQL code, so the database receives whatever string the function returns."
                    },
                })
                .note(match state.kind {
                    UnsafeSqlKind::Interpolation | UnsafeSqlKind::TaggedTemplate => markup! {
                        "Write a "<Emphasis>"?"</Emphasis>" placeholder in the SQL string, and pass the values when you run the prepared statement, for example "<Emphasis>"database.prepare(\"SELECT * FROM users WHERE id = ?\").get(id)"</Emphasis>"."
                    },
                    UnsafeSqlKind::TaggedTemplateWithoutValues => markup! {
                        "Remove the tag and pass a plain string instead."
                    },
                }),
        )
    }
}

/// The `DatabaseSync` methods that run SQL code.
#[derive(Clone, Copy)]
enum SqliteMethod {
    Exec,
    Prepare,
}

impl biome_console::fmt::Display for SqliteMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Exec => "exec()",
            Self::Prepare => "prepare()",
        })
    }
}

/// How the SQL string mixes values into the SQL code.
#[derive(Clone, Copy)]
enum UnsafeSqlKind {
    /// An untagged template literal with at least one `${}` placeholder.
    Interpolation,
    /// A tagged template with at least one `${}` placeholder.
    TaggedTemplate,
    /// A tagged template without placeholders.
    TaggedTemplateWithoutValues,
}

impl biome_console::fmt::Display for UnsafeSqlKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Interpolation => fmt.write_markup(markup! {
                "inserts values with "<Emphasis>"${}"</Emphasis>
            }),
            Self::TaggedTemplate | Self::TaggedTemplateWithoutValues => {
                fmt.write_str("is built by a tagged template")
            }
        }
    }
}

/// What a variable imported from `node:sqlite` refers to.
#[derive(Clone, Copy, Eq, PartialEq)]
enum SqliteImport {
    /// The `DatabaseSync` class.
    DatabaseSync,
    /// The whole module, imported as a namespace or as the default export.
    Module,
}

/// The module name that the rule recognizes.
const SQLITE_MODULE: &str = "node:sqlite";

/// Whether `expression` evaluates to an instance created with `new DatabaseSync()`.
fn is_database_sync_instance(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(AnyJsExpression::JsNewExpression(new_expression)) =
        resolve_const_aliases(expression, model)
    else {
        return false;
    };
    new_expression
        .callee()
        .is_ok_and(|callee| is_database_sync_class(&callee, model))
}

/// Whether `expression` evaluates to the `DatabaseSync` class of `node:sqlite`,
/// either through a named import or as a member of the imported module.
fn is_database_sync_class(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(expression) = resolve_const_aliases(expression, model) else {
        return false;
    };
    if let Some(reference) = expression.as_js_reference_identifier() {
        return sqlite_import(&reference, model) == Some(SqliteImport::DatabaseSync);
    }
    let Some(member) = AnyJsMemberExpression::cast(expression.into_syntax()) else {
        return false;
    };
    member_name(&member, model).is_some_and(|name| name.text() == "DatabaseSync")
        && member
            .object()
            .is_ok_and(|object| is_sqlite_module(&object, model))
}

/// Whether `expression` evaluates to the whole `node:sqlite` module.
fn is_sqlite_module(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    resolve_const_aliases(expression, model)
        .and_then(|expression| expression.as_js_reference_identifier())
        .and_then(|reference| sqlite_import(&reference, model))
        == Some(SqliteImport::Module)
}

/// Returns what `reference` refers to when it's a variable imported from
/// `node:sqlite` by an `import` statement that doesn't import only types.
fn sqlite_import(reference: &JsReferenceIdentifier, model: &SemanticModel) -> Option<SqliteImport> {
    let binding = model.binding(reference)?;
    let identifier = binding.tree();
    if identifier.is_type_only() {
        return None;
    }
    let declaration = identifier.declaration()?;
    let import = declaration
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsImport::cast)?;
    if import.source_text().ok()?.text() != SQLITE_MODULE {
        return None;
    }
    match declaration {
        AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => Some(SqliteImport::Module),
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => {
            let specifier = AnyJsNamedImportSpecifier::cast(declaration.into_syntax())?;
            match inner_string_text(&specifier.imported_name()?).text() {
                "DatabaseSync" => Some(SqliteImport::DatabaseSync),
                "default" => Some(SqliteImport::Module),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns the name of the property that `member` reads, such as `exec` for
/// `database.exec`, `database["exec"]`, and `database[method]` when `method`
/// is a `const` variable that holds `"exec"`.
fn member_name(member: &AnyJsMemberExpression, model: &SemanticModel) -> Option<StaticValue> {
    match member {
        AnyJsMemberExpression::JsStaticMemberExpression(_) => member.member_name(),
        AnyJsMemberExpression::JsComputedMemberExpression(computed) => {
            resolve_const_aliases(&computed.member().ok()?, model)?.as_static_value()
        }
    }
}

/// Returns the expression that produces the value of `expression`, following
/// references to `const` variables to their initial values.
///
/// Parentheses and TypeScript type syntax, such as `as` and `!`, are skipped.
/// The search stops at the first expression that isn't a reference to a
/// `const` variable with an initial value, and returns that expression.
/// Returns `None` when the variables refer to each other in a cycle, or when
/// the syntax tree is incomplete.
fn resolve_const_aliases(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let mut expression = expression.inner_expression()?;
    let mut visited = Vec::new();
    while let Some(reference) = expression.as_js_reference_identifier() {
        let Some(binding) = model.binding(&reference) else {
            break;
        };
        let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
            binding.tree().declaration()
        else {
            break;
        };
        if !declarator
            .declaration()
            .is_some_and(|declaration| declaration.is_const())
        {
            break;
        }
        let Some(initializer) = declarator.initializer() else {
            break;
        };
        let range = declarator.range();
        if visited.contains(&range) {
            return None;
        }
        visited.push(range);
        expression = initializer.expression().ok()?.inner_expression()?;
    }
    Some(expression)
}
