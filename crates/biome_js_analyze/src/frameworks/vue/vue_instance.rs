//! Helpers for detecting accesses to built-in Vue component instance properties, such as
//! `$slots` or `$attrs`.
//!
//! These properties are reachable in two ways: as bare identifiers in template expressions,
//! and as members of `this` inside an Options API component definition.

use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueOptionsApiBasedComponent,
};
use crate::services::embedded::EmbeddedService;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, JsIdentifierExpression, JsName, JsSyntaxToken,
    JsThisExpression, JsVariableDeclaration, JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use camino::Utf8Path;

/// Returns the name token of `reference` if it is the instance property `name` used in a Vue
/// template expression.
///
/// Returns `None` outside Vue template expressions, and when `name` resolves to a variable,
/// either inside the expression itself or among the bindings exposed to the template, such as
/// variables declared in `<script setup>`.
pub fn template_instance_property_reference(
    reference: &AnyJsIdentifierReference,
    name: &str,
    model: &SemanticModel,
    source: &JsFileSource,
    embedded: Option<&EmbeddedService>,
) -> Option<JsSyntaxToken> {
    if !source.as_embedding_kind().is_vue() || source.is_embedded_source() {
        return None;
    }

    let token = reference.value_token().ok()?;
    if token.text_trimmed() != name || !model.is_unresolved_reference(reference) {
        return None;
    }

    if embedded?.contains_binding(token.token_text_trimmed()) {
        return None;
    }

    Some(token)
}

/// Returns the name token of `member` if `object.member` reads or writes the instance property
/// `name` inside an Options API component definition.
///
/// `object` must be `this`, or a variable declared with `const alias = this`. The access must be
/// located inside the object literal that defines the component, such as the object passed to
/// `defineComponent()`.
pub fn component_instance_property_member(
    object: AnyJsExpression,
    member: &JsName,
    name: &str,
    model: &SemanticModel,
    source: &JsFileSource,
    path: &Utf8Path,
) -> Option<JsSyntaxToken> {
    let token = member.value_token().ok()?;
    if token.text_trimmed() != name
        || !is_this_or_direct_const_alias(object, model)
        || !is_in_component_definition_object(member, model, source, path)
    {
        return None;
    }

    Some(token)
}

fn is_this_or_direct_const_alias(object: AnyJsExpression, model: &SemanticModel) -> bool {
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

fn is_in_component_definition_object(
    member: &JsName,
    model: &SemanticModel,
    source: &JsFileSource,
    path: &Utf8Path,
) -> bool {
    let member = member.syntax();
    member.ancestors().skip(1).any(|ancestor| {
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
                member
                    .ancestors()
                    .any(|ancestor| ancestor == *definition.syntax())
            })
    })
}
