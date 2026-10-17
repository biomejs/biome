use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    options::PreferredQuote,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier,
    AnyJsObjectMember, JsCallExpression, JsReferenceIdentifier, JsSyntaxToken, T,
    binding_ext::AnyJsBindingDeclaration, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, TriviaPieceKind};
use biome_rule_options::use_consistent_json_file_read::{
    JsonFileReadAs, UseConsistentJsonFileReadOptions,
};

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce a consistent way of reading JSON files before passing them to `JSON.parse()`.
    ///
    /// The encoding of a file tells Node.js how to turn the file's bytes into text, for example `'utf8'`.
    /// With an encoding, `fs.readFile()` and `fs.readFileSync()` return a string.
    /// Without one, they return a `Buffer`, which holds the file's raw bytes.
    /// [`JSON.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse)
    /// accepts both: JavaScript turns a `Buffer` into a string automatically, using UTF-8.
    /// This rule makes every JSON file read use the same style.
    ///
    /// By default, the rule asks for strings, because TypeScript reports an error when a `Buffer` is passed to `JSON.parse()`.
    /// Use the [`readAs`](#readas) option to ask for `Buffer`s instead.
    ///
    /// The rule checks `readFile()` and `readFileSync()` calls whose result is passed to `JSON.parse()`,
    /// either directly or through a variable that is used only once.
    /// It checks these methods on any object, such as `fs.readFile()` or `fs.promises.readFile()`,
    /// and functions imported by name from `fs`, `node:fs`, `fs/promises`, or `node:fs/promises`.
    /// Calls that pass options other than `encoding`, such as `signal`, aren't checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const packageJson = JSON.parse(await fs.readFile("./package.json"));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const promise = fs.readFile("./package.json");
    /// const packageJson = JSON.parse(await promise);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { readFileSync } from "node:fs";
    /// const packageJson = JSON.parse(readFileSync("./package.json", { encoding: null }));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const packageJson = JSON.parse(await fs.readFile("./package.json", "utf8"));
    ///
    /// // Not checked, because the options include more than `encoding`.
    /// const promise = fs.readFile("./package.json", { encoding: null, signal });
    /// const config = JSON.parse(await promise);
    /// ```
    ///
    /// ## Options
    ///
    /// ### `readAs`
    ///
    /// Default: `"string"`
    ///
    /// How JSON files should be read before they are passed to `JSON.parse()`:
    ///
    /// - `"string"`: read files with the `'utf8'` encoding. The rule reports reads without an encoding,
    ///   or with a `null` or `undefined` encoding.
    /// - `"buffer"`: read files without an encoding. The rule reports reads with the `'utf8'` or `'utf-8'` encoding.
    ///
    /// The following examples use `"buffer"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "readAs": "buffer"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// const packageJson = JSON.parse(await fs.readFile("./package.json", "utf8"));
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const packageJson = JSON.parse(fs.readFileSync("./package.json", { encoding: "utf8" }));
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// const packageJson = JSON.parse(await fs.readFile("./package.json"));
    /// const config = JSON.parse(fs.readFileSync("./config.json", null));
    /// ```
    ///
    pub UseConsistentJsonFileRead {
        version: "2.6.0",
        name: "useConsistentJsonFileRead",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("consistent-json-file-read").same()],
        recommended: false,
        // The object in `x.readFile()` isn't proven to be the `fs` module.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseConsistentJsonFileRead {
    type Query = Semantic<JsCallExpression>;
    type State = InconsistentRead;
    type Signals = Option<Self::State>;
    type Options = UseConsistentJsonFileReadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let argument = json_parse_argument(call, model)?;
        let read_call = resolve_read_call(argument, model)?;
        if !is_read_file_call(&read_call, model) {
            return None;
        }

        let arguments = read_call.arguments().ok()?.args();
        match (ctx.options().read_as(), arguments.len()) {
            (JsonFileReadAs::String, 1) => Some(InconsistentRead::MissingEncoding(read_call)),
            (JsonFileReadAs::String, 2) => {
                let encoding = arguments.last()?.ok()?;
                let encoding = encoding.as_any_js_expression()?;
                is_buffer_encoding(encoding, model)
                    .then(|| InconsistentRead::NullEncoding(encoding.clone()))
            }
            (JsonFileReadAs::Buffer, 2) => {
                let encoding = arguments.last()?.ok()?;
                let encoding = encoding.as_any_js_expression()?;
                is_utf8_encoding(encoding).then(|| InconsistentRead::Utf8Encoding(read_call))
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            InconsistentRead::MissingEncoding(read_call) => read_call.range(),
            InconsistentRead::NullEncoding(encoding) => encoding.range(),
            InconsistentRead::Utf8Encoding(read_call) => {
                read_call.arguments().ok()?.args().last()?.ok()?.range()
            }
        };
        let diagnostic = match state {
            InconsistentRead::MissingEncoding(_) | InconsistentRead::NullEncoding(_) => {
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This JSON file is read as a "<Emphasis>"Buffer"</Emphasis>" before being passed to "<Emphasis>"JSON.parse()"</Emphasis>"."
                    },
                )
                .note(markup! {
                    <Emphasis>"JSON.parse()"</Emphasis>" expects a string. A "<Emphasis>"Buffer"</Emphasis>" only works because JavaScript turns it into a string automatically, and TypeScript reports it as an error."
                })
                .note(markup! {
                    "Read the file with the "<Emphasis>"'utf8'"</Emphasis>" encoding instead."
                })
            }
            InconsistentRead::Utf8Encoding(_) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This JSON file is read as a string, but this project reads JSON files as "<Emphasis>"Buffer"</Emphasis>"s."
                },
            )
            .note(markup! {
                <Emphasis>"JSON.parse()"</Emphasis>" turns a "<Emphasis>"Buffer"</Emphasis>" into a string using UTF-8 automatically, so the encoding isn't needed."
            })
            .note(markup! {
                "Remove the encoding."
            }),
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = match state {
            InconsistentRead::MissingEncoding(read_call) => {
                let arguments = read_call.arguments().ok()?.args();
                let path = arguments.first()?.ok()?;
                let mut separators = vec![
                    make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                ];
                separators.extend(arguments.trailing_separator());
                mutation.replace_node(
                    arguments,
                    make::js_call_argument_list(
                        [
                            path,
                            AnyJsCallArgument::AnyJsExpression(utf8_literal(ctx.preferred_quote())),
                        ],
                        separators,
                    ),
                );
                markup! { "Read the file with the "<Emphasis>"'utf8'"</Emphasis>" encoding." }
            }
            InconsistentRead::NullEncoding(encoding) => {
                // Replacing the options object would drop the comments inside it.
                if has_comments_inside(encoding) {
                    return None;
                }
                mutation.replace_node_transfer_trivia(
                    encoding.clone(),
                    utf8_literal(ctx.preferred_quote()),
                )?;
                markup! { "Read the file with the "<Emphasis>"'utf8'"</Emphasis>" encoding." }
            }
            InconsistentRead::Utf8Encoding(read_call) => {
                let arguments = read_call.arguments().ok()?.args();
                let path = arguments.first()?.ok()?;
                let encoding = arguments.last()?.ok()?;
                let separator = arguments.separators().next()?.ok()?;
                // Removing the encoding drops every comment attached to it or to the comma before it.
                if encoding.syntax().has_comments_descendants() || separator.has_trailing_comments()
                {
                    return None;
                }
                mutation.replace_node(
                    arguments.clone(),
                    make::js_call_argument_list([path], arguments.trailing_separator()),
                );
                markup! { "Remove the encoding." }
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message.to_owned(),
            mutation,
        ))
    }
}

pub enum InconsistentRead {
    /// `readFile(path)` with `readAs: "string"`; the diagnostic covers the call.
    MissingEncoding(JsCallExpression),
    /// `readFile(path, null)` or `readFile(path, { encoding: null })` with `readAs: "string"`;
    /// the diagnostic covers the second argument.
    NullEncoding(AnyJsExpression),
    /// `readFile(path, "utf8")` or `readFile(path, { encoding: "utf8" })` with `readAs: "buffer"`;
    /// the diagnostic covers the second argument.
    Utf8Encoding(JsCallExpression),
}

const FS_MODULES: [&str; 4] = ["fs", "node:fs", "fs/promises", "node:fs/promises"];

/// Returns the argument of `call` if it's a call to the global `JSON.parse()` with exactly one argument.
///
/// Returns `None` for optional calls such as `JSON?.parse(text)`, calls with a reviver or a spread argument,
/// and calls where `JSON` is a local variable.
fn json_parse_argument(call: &JsCallExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    if call.optional_chain_token().is_some() {
        return None;
    }
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
    if callee.is_optional_chain() || callee.member_name()?.text() != "parse" {
        return None;
    }
    let object = callee.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    if name.text() != "JSON" || model.binding(&reference).is_some() {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    arguments.first()?.ok()?.as_any_js_expression().cloned()
}

/// Follows `await` expressions and variables from the argument of `JSON.parse()`
/// back to the expression that produced the value, and returns it if it's a call.
///
/// A variable is followed only if it's declared with an initializer, such as `const data = readFile(path)`,
/// and the argument is its only reference. Otherwise this returns `None`.
fn resolve_read_call(
    mut expression: AnyJsExpression,
    model: &SemanticModel,
) -> Option<JsCallExpression> {
    loop {
        expression = expression.omit_parentheses();
        if let AnyJsExpression::JsAwaitExpression(await_expression) = expression {
            expression = await_expression.argument().ok()?;
            continue;
        }
        let AnyJsExpression::JsIdentifierExpression(identifier) = &expression else {
            break;
        };
        let binding = model.binding(&identifier.name().ok()?)?;
        // Adding an encoding changes the value seen by every use of the variable,
        // and a reassignment means the initializer may not be the value that's parsed.
        if binding.all_references().nth(1).is_some() {
            return None;
        }
        let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
            binding.tree().declaration()
        else {
            return None;
        };
        expression = declarator.initializer()?.expression().ok()?;
    }

    expression.as_js_call_expression().cloned()
}

/// Whether `call` is a `readFile()` or `readFileSync()` call with a path and an optional second argument.
///
/// Method calls such as `fs.readFile(path)` match by name on any object.
/// Plain calls such as `readFile(path)` match only when the function is a named import from one of [`FS_MODULES`].
fn is_read_file_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    if call.optional_chain_token().is_some() {
        return false;
    }
    let Ok(arguments) = call.arguments() else {
        return false;
    };
    let arguments = arguments.args();
    if !matches!(arguments.len(), 1 | 2)
        || arguments
            .iter()
            .any(|argument| !matches!(argument, Ok(AnyJsCallArgument::AnyJsExpression(_))))
    {
        return false;
    }

    let Ok(callee) = call.callee() else {
        return false;
    };
    match callee.omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .is_ok_and(|reference| imported_read_file_name(&reference, model).is_some()),
        callee => AnyJsMemberExpression::cast(callee.into_syntax()).is_some_and(|member| {
            !member.is_optional_chain()
                && member
                    .member_name()
                    .is_some_and(|name| is_read_file_name(name.text()))
        }),
    }
}

/// Returns the imported name when `reference` is bound to a named import of `readFile` or `readFileSync`
/// from one of [`FS_MODULES`], such as `import { readFileSync as read } from "node:fs"`.
fn imported_read_file_name(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<JsSyntaxToken> {
    let binding = model.binding(reference)?;
    let specifier = AnyJsNamedImportSpecifier::cast(binding.syntax().parent()?)?;
    let source = specifier
        .import_clause()?
        .source()
        .ok()?
        .inner_string_text()
        .ok()?;
    if !FS_MODULES.contains(&source.text()) {
        return None;
    }
    specifier
        .imported_name()
        .filter(|name| is_read_file_name(name.text_trimmed()))
}

fn is_read_file_name(name: &str) -> bool {
    matches!(name, "readFile" | "readFileSync")
}

fn utf8_literal(preferred_quote: PreferredQuote) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(
        make::js_string_literal_expression(if preferred_quote.is_double() {
            make::js_string_literal("utf8")
        } else {
            make::js_string_literal_single_quotes("utf8")
        })
        .into(),
    )
}

/// Whether `encoding` makes `readFile()` return a `Buffer`: `null`, `undefined`,
/// or an object whose only property is a `null` or `undefined` `encoding`.
fn is_buffer_encoding(encoding: &AnyJsExpression, model: &SemanticModel) -> bool {
    let encoding = encoding.clone().omit_parentheses();
    if let AnyJsExpression::JsObjectExpression(object) = &encoding {
        let members = object.members();
        if members.len() != 1 {
            return false;
        }
        let Some(Ok(AnyJsObjectMember::JsPropertyObjectMember(property))) = members.first() else {
            return false;
        };
        return property
            .name()
            .ok()
            .and_then(|name| name.name())
            .is_some_and(|name| name.text() == "encoding")
            && property
                .value()
                .is_ok_and(|value| is_null_or_undefined(&value, model));
    }
    is_null_or_undefined(&encoding, model)
}

/// Whether `encoding` makes `readFile()` return a UTF-8 string: `"utf8"` or `"utf-8"` in any letter case,
/// or an object whose only property is an `encoding` with one of those values.
fn is_utf8_encoding(encoding: &AnyJsExpression) -> bool {
    let encoding = encoding.clone().omit_parentheses();
    if let AnyJsExpression::JsObjectExpression(object) = &encoding {
        let members = object.members();
        if members.len() != 1 {
            return false;
        }
        let Some(Ok(AnyJsObjectMember::JsPropertyObjectMember(property))) = members.first() else {
            return false;
        };
        return property
            .name()
            .ok()
            .and_then(|name| name.name())
            .is_some_and(|name| name.text() == "encoding")
            && property.value().is_ok_and(|value| is_utf8_string(&value));
    }
    is_utf8_string(&encoding)
}

fn is_utf8_string(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_static_value()
        .and_then(|value| {
            let text = value.as_string_constant()?;
            Some(text.eq_ignore_ascii_case("utf8") || text.eq_ignore_ascii_case("utf-8"))
        })
        .unwrap_or(false)
}

fn is_null_or_undefined(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.as_js_null_literal_expression().is_some()
        }
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .is_ok_and(|reference| reference.is_undefined() && model.binding(&reference).is_none()),
        _ => false,
    }
}

/// Whether `node` contains comments other than the ones before its first token and after its last token.
/// The fix keeps those outer comments, but replacing `node` drops any comment inside it.
fn has_comments_inside(node: &AnyJsExpression) -> bool {
    let node = node.syntax();
    let (Some(first_token), Some(last_token)) = (node.first_token(), node.last_token()) else {
        return false;
    };
    node.descendants_tokens(Direction::Next).any(|token| {
        (token != first_token && token.has_leading_comments())
            || (token != last_token && token.has_trailing_comments())
    })
}
