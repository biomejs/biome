use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier,
    JsCallExpression, JsImport, JsNewOrCallExpression, JsObjectBindingPattern,
    JsObjectBindingPatternPropertyList, JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_buffer_constructor::NoBufferConstructorOptions;

use crate::services::semantic::Semantic;
use crate::shared::node_process::is_process_module_import;

declare_lint_rule! {
    /// Disallow the deprecated `Buffer()` constructor in Node.js.
    ///
    /// In Node.js, `Buffer()` and `new Buffer()` do different things depending on the type
    /// of the value you pass in. A number creates a buffer of that size, while a string or an
    /// array creates a buffer that holds a copy of that data. If the value comes from user input
    /// and its type isn't checked, an attacker can pick the type. This can expose data that was
    /// left in memory, or make the program use a lot of memory.
    ///
    /// For this reason, Node.js deprecated the `Buffer` constructor. Use one of these functions
    /// instead, which each do only one thing:
    ///
    /// - `Buffer.from()` creates a buffer from existing data, such as a string or an array.
    /// - `Buffer.alloc()` creates a buffer of a given size, filled with zeros.
    /// - `Buffer.allocUnsafe()` creates a buffer of a given size faster, but it may contain
    ///   old data from memory.
    ///
    /// The rule reports the global `Buffer`, including `globalThis.Buffer`, and `Buffer` when it
    /// comes from the `buffer`, `node:buffer`, or `safe-buffer` modules through `import`,
    /// `require()`, or `process.getBuiltinModule()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Buffer(5);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Buffer([1, 2, 3]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { Buffer } from "node:buffer";
    ///
    /// new Buffer(res.body.amount);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const buffer = require("buffer");
    ///
    /// new buffer.Buffer(res.body.values);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Buffer.alloc(5);
    /// Buffer.allocUnsafe(5);
    /// Buffer.from([1, 2, 3]);
    ///
    /// Buffer.alloc(res.body.amount);
    /// Buffer.from(res.body.values);
    /// ```
    ///
    pub NoBufferConstructor {
        version: "next",
        name: "noBufferConstructor",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-buffer-constructor").same(),
            RuleSource::EslintN("no-deprecated-api").inspired(),
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoBufferConstructor {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoBufferConstructorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        let model = ctx.model();
        if let Some(identifier) = callee.as_js_identifier_expression() {
            let reference = identifier.name().ok()?;
            let is_buffer = match model.binding(&reference) {
                None => reference.has_name("Buffer"),
                Some(binding) => is_buffer_binding(model, &binding),
            };
            return is_buffer.then_some(());
        }
        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        is_buffer_member(model, &member).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                match node {
                    JsNewOrCallExpression::JsNewExpression(_) => markup! {
                        "Don't use the deprecated "<Emphasis>"new Buffer()"</Emphasis>"."
                    },
                    JsNewOrCallExpression::JsCallExpression(_) => markup! {
                        "Don't use the deprecated "<Emphasis>"Buffer()"</Emphasis>"."
                    },
                },
            )
            .note(markup! {
                "It does different things depending on the type of its argument, which can expose data left in memory when the argument comes from user input."
            })
            .note(markup! {
                "Use "<Emphasis>"Buffer.from()"</Emphasis>" to create a buffer from existing data, or "<Emphasis>"Buffer.alloc()"</Emphasis>" to create a buffer of a given size."
            }),
        )
    }
}

/// Identifiers that refer to the global object when they are not shadowed.
const GLOBAL_OBJECT_NAMES: [&str; 4] = ["global", "globalThis", "self", "window"];

/// Module specifiers whose `Buffer` export is the Node.js `Buffer` constructor.
const BUFFER_MODULE_NAMES: [&str; 3] = ["buffer", "node:buffer", "safe-buffer"];

/// Whether `binding` is the `Buffer` constructor taken from a Buffer module or from the global
/// `Buffer`:
///
/// ```js
/// import { Buffer as B } from "buffer";
/// const { Buffer: B } = require("buffer");
/// const B = require("buffer").Buffer;
/// const B = Buffer;
/// ```
fn is_buffer_binding(model: &SemanticModel, binding: &Binding) -> bool {
    let tree = binding.tree();
    let Some(declaration) = tree.declaration() else {
        return false;
    };
    match declaration {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => {
            AnyJsNamedImportSpecifier::cast(declaration.into_syntax()).is_some_and(|specifier| {
                specifier
                    .imported_name()
                    .is_some_and(|name| name.text_trimmed() == "Buffer")
                    && is_buffer_module_import(&specifier)
            })
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => {
            tree.name_token()
                .is_ok_and(|name| name.text_trimmed() == "Buffer")
                && property
                    .parent::<JsObjectBindingPatternPropertyList>()
                    .and_then(|properties| destructured_value(&properties))
                    .is_some_and(|value| is_buffer_module_object(model, &value))
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
            property
                .member()
                .ok()
                .and_then(|member| member.name())
                .is_some_and(|name| name.text() == "Buffer")
                && property
                    .parent::<JsObjectBindingPatternPropertyList>()
                    .and_then(|properties| destructured_value(&properties))
                    .is_some_and(|value| is_buffer_module_object(model, &value))
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(value) = declarator_value(&declarator) else {
                return false;
            };
            if let Some(identifier) = value.as_js_identifier_expression() {
                return identifier.name().is_ok_and(|reference| {
                    reference.has_name("Buffer") && model.binding(&reference).is_none()
                });
            }
            AnyJsMemberExpression::cast(value.into_syntax())
                .is_some_and(|member| is_buffer_member(model, &member))
        }
        _ => false,
    }
}

/// Whether `member` reads `Buffer` from an object that holds the `Buffer` constructor, such as
/// `globalThis.Buffer` or `require("buffer").Buffer`.
fn is_buffer_member(model: &SemanticModel, member: &AnyJsMemberExpression) -> bool {
    member
        .member_name()
        .is_some_and(|name| name.text() == "Buffer")
        && member
            .object()
            .is_ok_and(|object| is_buffer_module_object(model, &object.omit_parentheses()))
}

/// Whether `object` holds the `Buffer` constructor as its `Buffer` property: the global object,
/// or the exports of a Buffer module.
///
/// ```js
/// globalThis // or `global`, `self`, `window`
/// require("buffer")
/// process.getBuiltinModule("buffer")
/// buffer // from `import * as buffer from "buffer"` or `const buffer = require("buffer")`
/// ```
fn is_buffer_module_object(model: &SemanticModel, object: &AnyJsExpression) -> bool {
    if let Some(call) = object.as_js_call_expression() {
        return is_buffer_module_load(model, call);
    }
    let Some(reference) = object
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok())
    else {
        return false;
    };
    let Some(binding) = model.binding(&reference) else {
        return GLOBAL_OBJECT_NAMES
            .iter()
            .any(|name| reference.has_name(name));
    };
    match binding.tree().declaration() {
        Some(
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_),
        ) => binding
            .syntax()
            .ancestors()
            .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
            .is_some_and(|source| BUFFER_MODULE_NAMES.contains(&source.text())),
        Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
            declarator_value(&declarator)
                .and_then(|value| value.as_js_call_expression().cloned())
                .is_some_and(|call| is_buffer_module_load(model, &call))
        }
        _ => false,
    }
}

/// Whether `call` loads a Buffer module, with `require("buffer")` or
/// `process.getBuiltinModule("buffer")`.
fn is_buffer_module_load(model: &SemanticModel, call: &JsCallExpression) -> bool {
    let Some(callee) = call.callee().ok().map(|callee| callee.omit_parentheses()) else {
        return false;
    };
    let is_loader = if let Some(identifier) = callee.as_js_identifier_expression() {
        identifier.name().is_ok_and(|reference| {
            reference.has_name("require") && model.binding(&reference).is_none()
        })
    } else {
        AnyJsMemberExpression::cast(callee.into_syntax()).is_some_and(|member| {
            member
                .member_name()
                .is_some_and(|name| name.text() == "getBuiltinModule")
                && member
                    .object()
                    .ok()
                    .and_then(|object| {
                        object
                            .omit_parentheses()
                            .as_js_identifier_expression()?
                            .name()
                            .ok()
                    })
                    .is_some_and(|reference| {
                        reference.has_name("process")
                            && model
                                .binding(&reference)
                                .is_none_or(|binding| is_process_module_import(&binding))
                    })
        })
    };
    is_loader
        && call
            .arguments()
            .ok()
            .and_then(|arguments| arguments.args().iter().next()?.ok())
            .and_then(|argument| argument.as_any_js_expression()?.as_static_value())
            .is_some_and(|value| {
                value
                    .as_string_constant()
                    .is_some_and(|source| BUFFER_MODULE_NAMES.contains(&source))
            })
}

/// Whether `specifier` belongs to an `import` statement that loads a Buffer module.
fn is_buffer_module_import(specifier: &AnyJsNamedImportSpecifier) -> bool {
    specifier
        .import_clause()
        .and_then(|clause| clause.source().ok()?.inner_string_text().ok())
        .is_some_and(|source| BUFFER_MODULE_NAMES.contains(&source.text()))
}

/// Returns the value that the object pattern holding `properties` destructures, such as
/// `require("buffer")` in `const { Buffer } = require("buffer")`.
///
/// Returns `None` when the pattern is nested in another pattern.
fn destructured_value(properties: &JsObjectBindingPatternPropertyList) -> Option<AnyJsExpression> {
    let declarator = properties
        .parent::<JsObjectBindingPattern>()?
        .parent::<JsVariableDeclarator>()?;
    declarator_value(&declarator)
}

/// Returns the initial value of `declarator`, without parentheses.
fn declarator_value(declarator: &JsVariableDeclarator) -> Option<AnyJsExpression> {
    Some(
        declarator
            .initializer()?
            .expression()
            .ok()?
            .omit_parentheses(),
    )
}
