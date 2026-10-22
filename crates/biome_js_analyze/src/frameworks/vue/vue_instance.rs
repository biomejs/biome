//! Helpers for recognizing accesses to the instance of an Options API Vue
//! component, such as `this.$slots` inside a component method.

use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueOptionsApiBasedComponent,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsIdentifierExpression, JsLanguage, JsThisExpression, JsVariableDeclaration,
    JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use camino::Utf8Path;

/// Returns `true` when `object` is `this`, or a `const` variable initialized
/// with `this`, as in `const vm = this`.
pub fn is_this_or_direct_const_alias(object: AnyJsExpression, model: &SemanticModel) -> bool {
    let object = object.omit_parentheses();
    if JsThisExpression::can_cast(object.syntax().kind()) {
        return true;
    }

    let Some(identifier) = object.as_js_identifier_expression() else {
        return false;
    };
    is_direct_const_this_alias(identifier, model)
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

/// Returns `true` when `node` is written inside the definition object of an
/// Options API component, a `defineComponent()` call, or a `createApp()` call.
///
/// `<script setup>` components have no definition object, so nodes in them
/// are never inside one.
pub fn is_in_vue_component_definition(
    node: &impl AstNode<Language = JsLanguage>,
    model: &SemanticModel,
    source: &JsFileSource,
    path: &Utf8Path,
) -> bool {
    let node = node.syntax();
    node.ancestors().skip(1).any(|ancestor| {
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
                node.ancestors()
                    .any(|ancestor| ancestor == *definition.syntax())
            })
    })
}
