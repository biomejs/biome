//! Selects explicit local function predicates without resolving their signatures or bodies.

use super::ResolutionCtx;
use crate::JsModuleInfo;
use biome_js_semantic::{Binding, JsDeclarationKind};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter,
    AnyJsParameter, AnyTsReturnType, AnyTsTypePredicateParameterName, JsCallExpression,
    JsReferenceIdentifier, binding_ext::AnyJsBindingDeclaration, unescape_js_identifier,
};
use biome_js_type_info::{
    FunctionParameter, Literal, NarrowingPredicate, RawTypeData, RawTypeId, ReturnType,
    TypeReference, TypeofKind, interned_types::TypeData,
};
use biome_rowan::AstSeparatedList;
use rustc_hash::FxHashSet;

const MAX_GUARD_ITEMS: usize = 128;
const MAX_GUARD_REFERENCES: usize = 1024;
const MAX_GUARD_NAME_BYTES: usize = 1024;
const MAX_GUARD_PARENTHESES: usize = 32;

pub(super) fn call_predicate_subject(
    info: &JsModuleInfo,
    call: &JsCallExpression,
) -> Option<JsReferenceIdentifier> {
    let (argument, _) = select_call_predicate(info, call)?;
    let AnyJsExpression::JsIdentifierExpression(identifier) = argument else {
        return None;
    };
    identifier.name().ok()
}

/// Selects an argument and raw predicate target from one unwritten local function.
///
/// Unsupported or malformed signatures and calls return `None`. Parameter
/// and argument lists are limited to 128 entries, and identifier decoding to
/// 1024 bytes per name. Callees with more than 1024 references are unsupported.
/// Neither signature types nor the function body are resolved.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only ordinary function declarations can supply a guard."
)]
fn select_call_predicate<'a>(
    info: &'a JsModuleInfo,
    call: &JsCallExpression,
) -> Option<(AnyJsExpression, &'a TypeReference)> {
    if call.is_optional_chain() || call.type_arguments().is_some() {
        return None;
    }
    let AnyJsExpression::JsIdentifierExpression(callee) = guard_operand(call.callee().ok()?)?
    else {
        return None;
    };
    let guard = info.semantic_model.binding(&callee.name().ok()?)?;
    if guard.is_imported() || guard.declaration_kind() != JsDeclarationKind::Function {
        return None;
    }
    let TypeReference::Resolved(RawTypeId::Local(id)) =
        info.raw_binding_types.get(&guard.range())?
    else {
        return None;
    };
    // Overload sets are callable objects, not a single raw function.
    let RawTypeData::Function(function) = info.raw_types.get(id.index())? else {
        return None;
    };
    if function.is_async || !function.type_parameters.is_empty() {
        return None;
    }
    let ReturnType::Predicate(predicate) = &function.return_type else {
        return None;
    };
    if guard
        .all_references()
        .take(MAX_GUARD_REFERENCES + 1)
        .enumerate()
        .any(|(index, reference)| index == MAX_GUARD_REFERENCES || reference.is_write())
    {
        return None;
    }
    if predicate.parameter_name.text().len() > MAX_GUARD_NAME_BYTES {
        return None;
    }
    let predicate_name = unescape_js_identifier(predicate.parameter_name.text());
    if predicate_name == "this" || predicate_name.contains('\\') {
        return None;
    }

    let (parameters, annotation) = match guard.tree().declaration()? {
        AnyJsBindingDeclaration::JsFunctionDeclaration(declaration) => {
            if declaration.async_token().is_some()
                || declaration.star_token().is_some()
                || declaration.type_parameters().is_some()
            {
                return None;
            }
            declaration.function_token().ok()?;
            (
                declaration.parameters().ok()?,
                declaration.return_type_annotation()?,
            )
        }
        AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(declaration) => {
            if declaration.async_token().is_some() || declaration.type_parameters().is_some() {
                return None;
            }
            declaration.function_token().ok()?;
            (
                declaration.parameters().ok()?,
                declaration.return_type_annotation()?,
            )
        }
        _ => return None,
    };
    let AnyTsReturnType::TsPredicateReturnType(annotation) = annotation.ty().ok()? else {
        return None;
    };
    let AnyTsTypePredicateParameterName::JsReferenceIdentifier(_) =
        annotation.parameter_name().ok()?
    else {
        return None;
    };
    annotation.is_token().ok()?;
    annotation.ty().ok()?;
    parameters.l_paren_token().ok()?;
    parameters.r_paren_token().ok()?;
    let parameters = parameters.items();
    if parameters.len() > MAX_GUARD_ITEMS || parameters.len() != function.parameters.len() {
        return None;
    }

    let mut names = FxHashSet::default();
    let mut argument_index = 0;
    let mut selected_index = None;
    for (index, (parameter, raw)) in parameters
        .iter()
        .zip(function.parameters.iter())
        .enumerate()
    {
        let FunctionParameter::Named(raw) = raw else {
            return None;
        };
        if raw.is_rest || raw.name.text().len() > MAX_GUARD_NAME_BYTES {
            return None;
        }
        let name = unescape_js_identifier(raw.name.text());
        let parameter = parameter.ok()?;
        if let AnyJsParameter::TsThisParameter(_) = parameter {
            if index != 0 || name != "this" {
                return None;
            }
            // TypeScript's synthetic receiver does not occupy a call argument.
            continue;
        }
        let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
            parameter,
        )) = parameter
        else {
            return None;
        };
        if parameter.initializer().is_some()
            || !matches!(
                parameter.binding().ok()?,
                AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(_))
            )
            || name == "this"
            || name.contains('\\')
        {
            return None;
        }
        if name == predicate_name {
            selected_index = Some(argument_index);
        }
        if !names.insert(name) {
            return None;
        }
        argument_index += 1;
    }
    let selected_index = selected_index?;
    let arguments = call.arguments().ok()?;
    arguments.l_paren_token().ok()?;
    arguments.r_paren_token().ok()?;
    let arguments = arguments.args();
    if arguments.len() > MAX_GUARD_ITEMS {
        return None;
    }
    let mut selected_argument = None;
    for (index, argument) in arguments.iter().enumerate() {
        let AnyJsCallArgument::AnyJsExpression(argument) = argument.ok()? else {
            return None;
        };
        if index == selected_index {
            selected_argument = Some(guard_operand(argument)?);
        }
    }
    Some((selected_argument?, &predicate.ty))
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Resolves only the selected primitive or literal target of a local predicate.
    pub(super) fn call_predicate(
        &mut self,
        call: &JsCallExpression,
        binding: &Binding,
    ) -> Option<NarrowingPredicate<'db>> {
        let (argument, target) = select_call_predicate(self.js_info, call)?;
        if !self.is_binding_read(&argument, binding) {
            return None;
        }
        let target = target.clone();
        self.primitive_guard_predicate(&target)
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Guard targets must be direct primitive types or supported literals."
    )]
    fn primitive_guard_predicate(
        &mut self,
        target: &TypeReference,
    ) -> Option<NarrowingPredicate<'db>> {
        let TypeReference::Resolved(id) = target else {
            return None;
        };
        let ty = match id {
            RawTypeId::Global(_) => self.resolve(target),
            RawTypeId::Local(id) => {
                match self.js_info.raw_types.get(id.index())? {
                    RawTypeData::BigInt
                    | RawTypeData::Boolean
                    | RawTypeData::Null
                    | RawTypeData::Number
                    | RawTypeData::String
                    | RawTypeData::Symbol
                    | RawTypeData::Undefined => {}
                    RawTypeData::Literal(literal)
                        if matches!(
                            literal.as_ref(),
                            Literal::BigInt(_)
                                | Literal::Boolean(_)
                                | Literal::Number(_)
                                | Literal::String(_)
                        ) => {}
                    _ => return None,
                }
                // A primitive entry may share its raw ID with a named type alias.
                self.resolve_raw_type_id(*id)
            }
        };
        Some(match ty {
            TypeData::BigInt => NarrowingPredicate::Typeof(TypeofKind::BigInt),
            TypeData::Boolean => NarrowingPredicate::Typeof(TypeofKind::Boolean),
            TypeData::Number => NarrowingPredicate::Typeof(TypeofKind::Number),
            TypeData::String => NarrowingPredicate::Typeof(TypeofKind::String),
            TypeData::Symbol => NarrowingPredicate::Typeof(TypeofKind::Symbol),
            TypeData::Null | TypeData::Undefined | TypeData::Literal(_) => {
                NarrowingPredicate::Literal(ty)
            }
            _ => return None,
        })
    }
}

fn guard_operand(mut expression: AnyJsExpression) -> Option<AnyJsExpression> {
    for _ in 0..MAX_GUARD_PARENTHESES {
        let AnyJsExpression::JsParenthesizedExpression(parenthesized) = expression else {
            return Some(expression);
        };
        parenthesized.l_paren_token().ok()?;
        parenthesized.r_paren_token().ok()?;
        expression = parenthesized.expression().ok()?;
    }
    None
}
