use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, JsIdentifierBinding,
    JsImport,
};
use biome_rowan::AstNode;

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

pub(crate) fn is_framework_lib_export(binding: &Binding, package_names: &[&str]) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| package_names.contains(&source.text()))
}

pub(crate) fn is_named_framework_lib_export(
    binding: &Binding,
    name: &str,
    package_names: &[&str],
) -> Option<bool> {
    let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
    let import_specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;
    let name_token = match &import_specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
            named_import.name().ok()?.value().ok()?
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => ident.name_token().ok()?,
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => {
            return Some(false);
        }
    };

    if name_token.text_trimmed() != name {
        return Some(false);
    }

    let import = import_specifier.import_clause()?.parent::<JsImport>()?;
    import
        .source_text()
        .ok()
        .map(|import_name| package_names.contains(&import_name.text()))
}
