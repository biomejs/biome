//! Helpers that tell whether an expression in a script refers to the instance
//! of a Vue component defined with the Options API.

use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueOptionsApiBasedComponent,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsIdentifierExpression, JsThisExpression, JsVariableDeclaration,
    JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use camino::Utf8Path;

/// Returns `true` if `expression` is `this`, or a variable declared with
/// `const vm = this`.
pub(crate) fn is_this_or_direct_const_alias(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    let expression = expression.clone().omit_parentheses();
    if JsThisExpression::can_cast(expression.syntax().kind()) {
        return true;
    }

    expression
        .as_js_identifier_expression()
        .is_some_and(|identifier| is_direct_const_this_alias(identifier, model))
}

fn is_direct_const_this_alias(identifier: &JsIdentifierExpression, model: &SemanticModel) -> bool {
    let Ok(reference) = identifier.name() else {
        return false;
    };
    let Some(binding) = model.binding(&reference) else {
        return false;
    };
    let binding = binding.tree();
    let Some(declarator) = binding
        .syntax()
        .parent()
        .and_then(JsVariableDeclarator::cast)
    else {
        return false;
    };
    let Some(declaration) = declarator
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsVariableDeclaration::cast)
    else {
        return false;
    };
    if !declaration.is_const() {
        return false;
    }

    declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .map(AnyJsExpression::omit_parentheses)
        .is_some_and(|initializer| JsThisExpression::can_cast(initializer.syntax().kind()))
}

/// Returns `true` if `expression` is inside the options object of a Vue
/// component: the default export of a component file, or the object passed to
/// `createApp()` or `defineComponent()`.
///
/// Components written with `<script setup>` have no options object, so
/// expressions inside them return `false`.
pub(crate) fn is_in_options_component_definition(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    source: &JsFileSource,
    path: &Utf8Path,
) -> bool {
    expression.syntax().ancestors().skip(1).any(|ancestor| {
        let Some(potential_component) = AnyPotentialVueComponent::cast(ancestor) else {
            return false;
        };
        let Some(component) =
            VueComponent::from_potential_component(&potential_component, model, source, path)
        else {
            return false;
        };

        let definition = match component.kind() {
            AnyVueComponent::OptionsApi(component) => component.definition_expression(),
            AnyVueComponent::CreateApp(component) => component.definition_expression(),
            AnyVueComponent::DefineComponent(component) => component.definition_expression(),
            AnyVueComponent::Setup(_) => None,
        };
        definition
            .and_then(|definition| definition.inner_expression())
            .and_then(|definition| definition.as_js_object_expression().cloned())
            .is_some_and(|definition| {
                expression
                    .syntax()
                    .ancestors()
                    .any(|ancestor| ancestor == *definition.syntax())
            })
    })
}
