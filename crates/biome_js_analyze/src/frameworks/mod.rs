use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, JsIdentifierBinding,
    JsImport, JsSyntaxToken, static_value::StaticValue,
};
use biome_rowan::AstNode;

pub(crate) mod drizzle;
pub(crate) mod playwright;
pub(crate) mod unit_tests;
pub(crate) mod vue;

pub(crate) fn is_framework_api_reference(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    api_name: &str,
    package_names: &[&str],
    global_name: Option<&str>,
) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };

    if let Some(callee) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        let Some(object) = callee.object().ok() else {
            return false;
        };
        let Some(reference) = object.omit_parentheses().as_js_reference_identifier() else {
            return false;
        };
        let Some(member_name) = callee.member_name() else {
            return false;
        };
        if member_name.text() != api_name {
            return false;
        }
        return match model.binding(&reference) {
            Some(decl) => is_framework_lib_export(&decl, package_names),
            None => match global_name {
                Some(global_name) => reference.has_name(global_name),
                None => false,
            },
        };
    }

    if let Some(ident) = expression.as_js_reference_identifier() {
        return model
            .binding(&ident)
            .and_then(|binding| is_named_framework_lib_export(&binding, api_name, package_names))
            .unwrap_or(false);
    }

    false
}

/// Returns the name of the framework API that `expression` refers to, or `None` if it doesn't
/// refer to one.
///
/// The expression can be:
/// - a named import from one of `package_names`, which returns the imported name even when the
///   import is renamed (`import { onMounted as mounted } from "vue"` makes `mounted` return
///   `onMounted`);
/// - a member of an object imported from one of `package_names`, or of the global object
///   `global_name` when no variable shadows it (`Vue.onMounted` returns `onMounted`).
pub(crate) fn framework_api_name(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    package_names: &[&str],
    global_name: Option<&str>,
) -> Option<StaticValue> {
    let expression = expression.inner_expression()?;

    if let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
        let reference = member
            .object()
            .ok()?
            .omit_parentheses()
            .as_js_reference_identifier()?;
        let is_framework_object = match model.binding(&reference) {
            Some(decl) => is_framework_lib_export(&decl, package_names),
            None => global_name.is_some_and(|global_name| reference.has_name(global_name)),
        };
        return if is_framework_object {
            member.member_name()
        } else {
            None
        };
    }

    let binding = model.binding(&expression.as_js_reference_identifier()?)?;
    named_framework_lib_export(&binding, package_names).map(StaticValue::String)
}

pub(crate) fn is_framework_lib_export(binding: &Binding, package_names: &[&str]) -> bool {
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| package_names.contains(&source.text()))
}

pub(crate) fn is_named_framework_lib_export(
    binding: &Binding,
    name: &str,
    package_names: &[&str],
) -> Option<bool> {
    named_framework_lib_export(binding, package_names).map(|token| token.text_trimmed() == name)
}

/// Returns the imported name of `binding` if it is declared by a named import from one of
/// `package_names`. For `import { a as b } from "pkg"`, the binding `b` returns `a`.
fn named_framework_lib_export(binding: &Binding, package_names: &[&str]) -> Option<JsSyntaxToken> {
    let ident = JsIdentifierBinding::cast_ref(&binding.syntax())?;
    let import_specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;
    let name_token = match &import_specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
            named_import.name().ok()?.value().ok()?
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => ident.name_token().ok()?,
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => return None,
    };

    let import = import_specifier.import_clause()?.parent::<JsImport>()?;
    let source = import.source_text().ok()?;
    package_names.contains(&source.text()).then_some(name_token)
}
