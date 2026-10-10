use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsImportLike, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsNamedImportSpecifier, AnyJsTemplateElement, JsAssignmentOperator, JsBinaryExpression,
    JsBinaryOperator, JsImport, JsImportMetaExpression, JsObjectBindingPattern,
    JsReferenceIdentifier, JsSyntaxToken, JsTemplateExpression, JsVariableDeclarator,
    binding_ext::AnyJsBindingDeclaration, unescape_js_string,
};
use biome_rowan::{AstNode, AstNodeList, TokenText, declare_node_union};
use biome_rule_options::no_path_concat::NoPathConcatOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow building file paths and URLs by joining strings to `__dirname`, `__filename`, or `import.meta` values.
    ///
    /// In Node.js, `__dirname` and `__filename` hold the folder and the path of the current file.
    /// `import.meta.dirname` and `import.meta.filename` hold the same values in ES modules.
    /// Adding a string to one of them is a common way to point at another file,
    /// but the character that separates folders depends on the operating system:
    /// Windows uses `\`, while Linux and macOS use `/`.
    /// Joining strings can also leave two separators in a row.
    ///
    /// Use `path.join()` or `path.resolve()` from the `node:path` module instead.
    /// They insert the right separator for the system the code runs on.
    ///
    /// `import.meta.url` is a URL such as `file:///project/src/index.js`, not a file path.
    /// Adding `/foo.js` to it gives `file:///project/src/index.js/foo.js`, which doesn't point to a file next to the current one.
    /// Use `new URL()` to build a URL that is relative to `import.meta.url`.
    ///
    /// The rule reports one of these values only when the text added right after it starts with `/`, `\`,
    /// or the `sep` value of the `node:path` module.
    /// Adding other text, such as a file extension, is allowed.
    /// The rule doesn't look up the value of a variable that is added, so `__dirname + suffix` is never reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const fullPath = __dirname + "/foo.js";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fullPath = `${__filename}/foo.js`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fullPath = import.meta.dirname + "/foo.js";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import path from "node:path";
    /// const fullPath = __dirname + path.sep + "foo.js";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fullUrl = import.meta.url + "/foo.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import path from "node:path";
    ///
    /// const fullPath1 = path.join(__dirname, "foo.js");
    /// const fullPath2 = path.resolve(import.meta.dirname, "foo.js");
    /// const fullPath3 = __filename + ".map";
    /// const fullPath4 = `${__dirname}_backup`;
    /// const fullUrl = new URL("./foo.js", import.meta.url);
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow `__dirname` and `__filename` in ES modules, see [`noGlobalDirnameFilename`](https://biomejs.dev/linter/rules/no-global-dirname-filename/).
    ///
    pub NoPathConcat {
        version: "next",
        name: "noPathConcat",
        language: "js",
        sources: &[RuleSource::EslintN("no-path-concat").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    /// The two ways of joining a value with the text that follows it:
    /// the `+` operator, and a template literal.
    pub AnyJsPathConcatenation = JsBinaryExpression | JsTemplateExpression
}

/// What the value at the start of the reported concatenation holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathKind {
    /// A file system path: `__dirname`, `__filename`, `import.meta.dirname`,
    /// or `import.meta.filename`.
    FilePath,
    /// The URL in `import.meta.url`.
    Url,
}

impl Rule for NoPathConcat {
    type Query = Semantic<AnyJsPathConcatenation>;
    type State = PathKind;
    type Signals = Option<Self::State>;
    type Options = NoPathConcatOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            AnyJsPathConcatenation::JsBinaryExpression(binary) => {
                if binary.operator().ok()? != JsBinaryOperator::Plus {
                    return None;
                }
                let kind = path_kind(&binary.left().ok()?, model)?;
                starts_with_path_separator(&binary.right().ok()?, model).then_some(kind)
            }
            AnyJsPathConcatenation::JsTemplateExpression(template) => {
                let elements = template.elements();
                elements
                    .iter()
                    .zip(elements.iter().skip(1))
                    .find_map(|(element, next_element)| {
                        let placeholder = element.as_js_template_element()?;
                        let kind = path_kind(&placeholder.expression().ok()?, model)?;
                        template_element_starts_with_path_separator(&next_element, model)
                            .then_some(kind)
                    })
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        let diagnostic = match state {
            PathKind::FilePath => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This file path is built by joining strings."
                },
            )
            .note(markup! {
                "The character that separates folders depends on the operating system, and joining strings can leave two separators in a row."
            })
            .note(markup! {
                "Use "<Emphasis>"path.join()"</Emphasis>" or "<Emphasis>"path.resolve()"</Emphasis>" from the "<Emphasis>"node:path"</Emphasis>" module instead."
            }),
            PathKind::Url => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This URL is built by joining strings."
                },
            )
            .note(markup! {
                <Emphasis>"import.meta.url"</Emphasis>" is the URL of the current file, so text added to its end doesn't point to a file next to it."
            })
            .note(markup! {
                "Use "<Emphasis>"new URL()"</Emphasis>" instead, for example "<Emphasis>"new URL(\"./foo.js\", import.meta.url)"</Emphasis>"."
            }),
        };
        Some(diagnostic)
    }
}

/// Module specifiers that resolve to the Node.js `path` module.
const PATH_MODULE_NAMES: [&str; 2] = ["path", "node:path"];

/// Classifies `expression` when it reads the path or the URL of the current
/// file: the Node.js globals `__dirname` and `__filename`, or the `dirname`,
/// `filename`, and `url` properties of `import.meta`.
///
/// Returns `None` for any other expression, including a `__dirname` or
/// `__filename` that refers to a variable declared in the file.
fn path_kind(expression: &AnyJsExpression, model: &SemanticModel) -> Option<PathKind> {
    let expression = expression.inner_expression()?;
    if let Some(reference) = expression.as_js_reference_identifier() {
        let name = reference.value_token().ok()?;
        if !matches!(name.text_trimmed(), "__dirname" | "__filename") {
            return None;
        }
        return model
            .binding(&reference)
            .is_none()
            .then_some(PathKind::FilePath);
    }

    let member = AnyJsMemberExpression::cast(expression.into_syntax())?;
    let object = member.object().ok()?.omit_parentheses();
    if !JsImportMetaExpression::can_cast(object.syntax().kind()) {
        return None;
    }
    match member.member_name()?.text() {
        "dirname" | "filename" => Some(PathKind::FilePath),
        "url" => Some(PathKind::Url),
        _ => None,
    }
}

/// Whether the string that `expression` evaluates to can start with a path
/// separator.
///
/// The check follows the parts of an expression that decide how its value
/// starts: the left side of `+`, both branches of `a ? b : c`, both sides of
/// `||`, `&&`, and `??`, the assigned value of `=`, and the first part of a
/// template literal. Variables are not followed to their values, except for
/// the `sep` value of the Node.js `path` module.
fn starts_with_path_separator(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    match &expression {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => string
            .inner_string_text()
            .is_ok_and(text_starts_with_path_separator),
        AnyJsExpression::JsTemplateExpression(template) => {
            template.tag().is_none()
                && template.elements().first().is_some_and(|element| {
                    template_element_starts_with_path_separator(&element, model)
                })
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            binary.operator() == Ok(JsBinaryOperator::Plus)
                && binary
                    .left()
                    .is_ok_and(|left| starts_with_path_separator(&left, model))
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            conditional
                .consequent()
                .is_ok_and(|consequent| starts_with_path_separator(&consequent, model))
                || conditional
                    .alternate()
                    .is_ok_and(|alternate| starts_with_path_separator(&alternate, model))
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            logical
                .left()
                .is_ok_and(|left| starts_with_path_separator(&left, model))
                || logical
                    .right()
                    .is_ok_and(|right| starts_with_path_separator(&right, model))
        }
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            assignment.operator() == Ok(JsAssignmentOperator::Assign)
                && assignment
                    .right()
                    .is_ok_and(|right| starts_with_path_separator(&right, model))
        }
        expression => is_path_module_separator(expression, model),
    }
}

/// Whether a part of a template literal can start with a path separator:
/// either its text, or the value of its `${}` placeholder.
fn template_element_starts_with_path_separator(
    element: &AnyJsTemplateElement,
    model: &SemanticModel,
) -> bool {
    match element {
        AnyJsTemplateElement::JsTemplateChunkElement(chunk) => chunk
            .template_chunk_token()
            .is_ok_and(|token| text_starts_with_path_separator(token.token_text_trimmed())),
        AnyJsTemplateElement::JsTemplateElement(element) => element
            .expression()
            .is_ok_and(|expression| starts_with_path_separator(&expression, model)),
    }
}

/// Whether a string starts with `/` or `\` once its escape sequences are
/// decoded, so that `"\\foo"` and `"\x2Ffoo"` both count.
///
/// `text` is the source text of a string literal without its quotes, or of
/// the text part of a template literal. Both separators count on every
/// operating system, so that the result of the rule doesn't depend on the
/// system it runs on.
fn text_starts_with_path_separator(text: TokenText) -> bool {
    // Only a string whose source starts with an escape sequence needs decoding.
    text.starts_with('/')
        || (text.starts_with('\\') && unescape_js_string(text).starts_with(['/', '\\']))
}

/// Whether `expression` reads the `sep` value of the Node.js `path` module:
/// `path.sep`, `path["sep"]`, `require("path").sep`, or a name that was
/// imported or destructured from the module as `sep`.
fn is_path_module_separator(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    if let Some(reference) = expression.as_js_reference_identifier() {
        return declaration(&reference, model)
            .and_then(|declaration| path_module_export_name(&declaration))
            .is_some_and(|name| name.text_trimmed() == "sep");
    }

    let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) else {
        return false;
    };
    if member.member_name().is_none_or(|name| name.text() != "sep") {
        return false;
    }
    let Ok(object) = member.object() else {
        return false;
    };
    let object = object.omit_parentheses();
    if is_path_module_require(&object) {
        return true;
    }
    object
        .as_js_reference_identifier()
        .and_then(|reference| declaration(&reference, model))
        .is_some_and(|declaration| declares_path_module(&declaration))
}

/// Returns the declaration of the variable or import that `reference` refers to.
fn declaration(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<AnyJsBindingDeclaration> {
    model.binding(reference)?.tree().declaration()
}

/// Returns the name of the `path` module export that `declaration` gives a
/// local name to: `join` for `import { join } from "node:path"`, and `sep` for
/// `const { sep: separator } = require("node:path")`.
///
/// Returns `None` when `declaration` doesn't take a single export from the
/// `path` module.
fn path_module_export_name(declaration: &AnyJsBindingDeclaration) -> Option<JsSyntaxToken> {
    match declaration {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => {
            let specifier = AnyJsNamedImportSpecifier::cast_ref(declaration.syntax())?;
            let import = specifier.import_clause()?.parent::<JsImport>()?;
            if !is_path_module_import(&import) {
                return None;
            }
            specifier.imported_name()
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => {
            if !is_destructured_path_module_require(declaration) {
                return None;
            }
            let identifier = property.identifier().ok()?;
            identifier.as_js_identifier_binding()?.name_token().ok()
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
            if !is_destructured_path_module_require(declaration) {
                return None;
            }
            let member = property.member().ok()?;
            member.as_js_literal_member_name()?.value().ok()
        }
        _ => None,
    }
}

/// Whether `declaration` gives a name to the whole `path` module:
/// `import path from "node:path"`, `import * as path from "node:path"`, or
/// `const path = require("node:path")`.
fn declares_path_module(declaration: &AnyJsBindingDeclaration) -> bool {
    match declaration {
        AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => declaration
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsImport::cast)
            .is_some_and(|import| is_path_module_import(&import)),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            is_initialized_with_path_module_require(declarator)
        }
        _ => false,
    }
}

/// Whether `property` is a property of the object pattern that directly
/// destructures `require("path")`, as in `const { sep } = require("path")`.
///
/// A property of a nested pattern, such as `sep` in
/// `const { posix: { sep } } = require("path")`, doesn't count because it
/// isn't an export of the module itself.
fn is_destructured_path_module_require(property: &AnyJsBindingDeclaration) -> bool {
    property
        .syntax()
        .grand_parent()
        .and_then(JsObjectBindingPattern::cast)
        .and_then(|pattern| pattern.parent::<JsVariableDeclarator>())
        .is_some_and(|declarator| is_initialized_with_path_module_require(&declarator))
}

fn is_initialized_with_path_module_require(declarator: &JsVariableDeclarator) -> bool {
    declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .is_some_and(|expression| is_path_module_require(&expression.omit_parentheses()))
}

fn is_path_module_import(import: &JsImport) -> bool {
    import
        .source_text()
        .is_ok_and(|source| PATH_MODULE_NAMES.contains(&source.text()))
}

/// Whether `expression` is `require("path")` or `require("node:path")`.
fn is_path_module_require(expression: &AnyJsExpression) -> bool {
    expression
        .as_js_call_expression()
        .and_then(|call| AnyJsImportLike::JsCallExpression(call.clone()).inner_string_text())
        .is_some_and(|source| PATH_MODULE_NAMES.contains(&source.text()))
}
