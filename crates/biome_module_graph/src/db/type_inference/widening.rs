//! Literal widening: the types that mutable locations take from the values
//! that initialize them.
//!
//! TypeScript gives `let n = 1` the type `number` and `const n = 1` the type
//! `1`. The type of literal expression is a *fresh* literal type, which
//! widens to its primitive type where the value initializes a location that
//! the program can assign later: a `let` or `var` variable, a property of an
//! object expression, a class field that isn't `readonly`, an element of an
//! array expression, or a parameter with a default value. A return type
//! inferred from a function's body widens only when it is a single literal
//! type. A conditional or logical expression passes on the fresh literal
//! types of its operands, and a `const` declared without a type annotation
//! passes on those of its initializer, so `let m = n` also declares `m` as
//! `number`. A literal type that an annotation or `as const` declares is not
//! fresh and stays.
//!
//! Collection marks the mutable locations, as
//! [`TypeofWidenedLiteralExpression`](biome_js_type_info::TypeofWidenedLiteralExpression)
//! describes, and resolution widens them here. Collection can't tell a fresh
//! literal type from a declared one, because the raw table stores equal types
//! once: the `1` of `let n = 1` and the `1` of `let n: 1` share one id. This
//! module tells them apart by the raw expression that the value comes from:
//! [`fresh_literal_kinds`] lists the kinds of literals that the value's
//! literal expressions have, following conditional and logical expressions
//! and `const` declarations, also into the modules that export them.
//!
//! Widening every literal type of a kind that has a fresh literal gives the
//! same type as TypeScript, which widens only the fresh ones, because the
//! primitive type of that kind absorbs the others, as `string` absorbs `"a"`
//! in `"a" | string`. It is wider only when a literal type is both fresh and
//! declared, as `1` in `flag ? one : 1` with `one: 1`, where TypeScript keeps
//! `1`.

use super::imports::{ExportOriginResult, find_export_origin};
use super::resolver::ResolutionCtx;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ResolutionMode, resolve_module_import};
use biome_js_semantic::{Binding, ScopeId};
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_type_info::{
    FreshLiteralKinds, ImportSymbol, Object as RawObject, ObjectKind, PrimitiveLiteralKind,
    RawTypeData, RawTypeId, TypeImportQualifier, TypeReference, TypeResolverLevel,
    TypeofExpression as RawTypeofExpression,
    TypeofWidenedLiteralExpression as RawTypeofWidenedLiteralExpression,
    interned_types::{InternedObject as InferredObject, TypeData as InferredTypeData},
    widen_literal_of_kinds,
};
use biome_rowan::Text;

/// Bounds the raw types that one search for fresh literal types visits,
/// including those of the `const` declarations and imports it follows.
const MAX_FRESH_LITERAL_STEPS: usize = 64;

/// Bounds the scopes searched for the declaration of a name.
const MAX_SCOPE_STEPS: usize = 1024;

/// Bounds the `typeof` wrappers looked through to find a literal type.
const MAX_WRAPPER_STEPS: usize = 16;

/// Returns the kinds of the fresh literal types that the value whose
/// collected type is `reference`, in `module`, has.
///
/// A value has the fresh literal type of a literal expression, of an operand
/// of a conditional or logical expression, of a unary minus applied to a
/// number or bigint literal, and of the initializer of a `const` declared
/// without a type annotation that it names, also through an import. With
/// `is_return_values`, the value is the union of the values that a function
/// returns, and has the fresh literal types of each of them. Other values,
/// such as calls, member accesses, assertions such as `1 as 1`, and the
/// literal types of a union that a type annotation writes, have none. The
/// search visits at most 64 raw types and keeps what it found when it stops.
pub(super) fn fresh_literal_kinds(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    reference: &TypeReference,
    is_return_values: bool,
) -> FreshLiteralKinds {
    let mut search = FreshLiteralSearch {
        db,
        remaining: MAX_FRESH_LITERAL_STEPS,
        kinds: FreshLiteralKinds::default(),
    };
    let returned_values = match reference {
        TypeReference::Resolved(id)
            if is_return_values && id.level() == TypeResolverLevel::Thin =>
        {
            match js_info.raw_types.get(id.id().index()) {
                Some(RawTypeData::Union(union)) => Some(union.types()),
                Some(_) | None => None,
            }
        }
        TypeReference::Resolved(_) | TypeReference::Qualifier(_) | TypeReference::Import(_) => None,
    };
    match returned_values {
        Some(values) => {
            for value in values {
                search.visit(module, js_info, value);
            }
        }
        None => search.visit(module, js_info, reference),
    }
    search.kinds
}

struct FreshLiteralSearch<'a> {
    db: &'a dyn ModuleDb,
    remaining: usize,
    kinds: FreshLiteralKinds,
}

impl FreshLiteralSearch<'_> {
    fn visit(&mut self, module: ModuleInfo, js_info: &JsModuleInfo, reference: &TypeReference) {
        if self.remaining == 0 {
            return;
        }
        self.remaining -= 1;
        match reference {
            TypeReference::Resolved(id) => {
                if id.level() != TypeResolverLevel::Thin || js_info.is_named_type(id.id()) {
                    return;
                }
                if let Some(raw) = js_info.raw_types.get(id.id().index()) {
                    self.visit_raw(module, js_info, raw);
                }
            }
            TypeReference::Qualifier(qualifier) => {
                if qualifier.type_parameters.is_empty()
                    && let Some(name) = qualifier.path.identifier()
                {
                    self.visit_name(module, js_info, qualifier.scope_id, name);
                }
            }
            TypeReference::Import(import) => self.visit_import(module, import),
        }
    }

    fn visit_raw(&mut self, module: ModuleInfo, js_info: &JsModuleInfo, raw: &RawTypeData) {
        match raw {
            RawTypeData::Literal(literal) => {
                if let Some(kind) = PrimitiveLiteralKind::from_literal_expression(literal) {
                    self.kinds |= kind;
                }
            }
            // A name, as `K` in `let n = K`.
            RawTypeData::Reference(
                reference @ (TypeReference::Qualifier(_) | TypeReference::Import(_)),
            ) => self.visit(module, js_info, reference),
            // A shorthand property, as `K` in `{ K }`.
            RawTypeData::TypeofValue(value) => {
                if value.ty.is_unknown() {
                    self.visit_name(
                        module,
                        js_info,
                        value.scope_id.unwrap_or(ScopeId::GLOBAL),
                        &value.identifier,
                    );
                } else {
                    self.visit(module, js_info, &value.ty);
                }
            }
            RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
                RawTypeofExpression::UnaryMinus(expression) => {
                    if expression.is_literal_argument {
                        self.visit(module, js_info, &expression.argument);
                    }
                }
                RawTypeofExpression::Conditional(expression) => {
                    self.visit(module, js_info, &expression.consequent);
                    self.visit(module, js_info, &expression.alternate);
                }
                RawTypeofExpression::LogicalAnd(expression) => {
                    self.visit(module, js_info, &expression.left);
                    self.visit(module, js_info, &expression.right);
                }
                RawTypeofExpression::LogicalOr(expression) => {
                    self.visit(module, js_info, &expression.left);
                    self.visit(module, js_info, &expression.right);
                }
                RawTypeofExpression::NullishCoalescing(expression) => {
                    self.visit(module, js_info, &expression.left);
                    self.visit(module, js_info, &expression.right);
                }
                // A widened value has no fresh literal type left. Biome can't
                // tell whether the others give one, as `id(1)` and `await 1`
                // do, so it takes them as giving none.
                RawTypeofExpression::WidenedLiteral(_)
                | RawTypeofExpression::Addition(_)
                | RawTypeofExpression::Await(_)
                | RawTypeofExpression::BitwiseNot(_)
                | RawTypeofExpression::Call(_)
                | RawTypeofExpression::CallArgument(_)
                | RawTypeofExpression::ComputedMember(_)
                | RawTypeofExpression::Destructure(_)
                | RawTypeofExpression::Index(_)
                | RawTypeofExpression::OptionalChainIndex(_)
                | RawTypeofExpression::IterableValueOf(_)
                | RawTypeofExpression::New(_)
                | RawTypeofExpression::Parameter(_)
                | RawTypeofExpression::StaticMember(_)
                | RawTypeofExpression::OptionalChainStaticMember(_)
                | RawTypeofExpression::Super(_)
                | RawTypeofExpression::This(_)
                | RawTypeofExpression::Typeof(_) => {}
            },
            // A reference to another local type stands for an assertion, as
            // `1 as 1`, whose literal type is not fresh, and a union is a type
            // annotation, whose literal types aren't either.
            RawTypeData::Reference(TypeReference::Resolved(_))
            | RawTypeData::Union(_)
            | RawTypeData::Unknown
            | RawTypeData::Global
            | RawTypeData::BigInt
            | RawTypeData::Boolean
            | RawTypeData::Null
            | RawTypeData::Number
            | RawTypeData::String
            | RawTypeData::Symbol
            | RawTypeData::Undefined
            | RawTypeData::Conditional
            | RawTypeData::ImportNamespace(_)
            | RawTypeData::Class(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Interface(_)
            | RawTypeData::Module(_)
            | RawTypeData::Namespace(_)
            | RawTypeData::Object(_)
            | RawTypeData::Tuple(_)
            | RawTypeData::Generic(_)
            | RawTypeData::Intersection(_)
            | RawTypeData::TypeOperator(_)
            | RawTypeData::IndexedAccess(_)
            | RawTypeData::MappedType(_)
            | RawTypeData::Extends(_)
            | RawTypeData::InstanceOf(_)
            | RawTypeData::MergedReference(_)
            | RawTypeData::TypeofType(_)
            | RawTypeData::AnyKeyword
            | RawTypeData::NeverKeyword
            | RawTypeData::ObjectKeyword
            | RawTypeData::ThisKeyword
            | RawTypeData::UnknownKeyword
            | RawTypeData::VoidKeyword => {}
        }
    }

    /// Visits the value that `name`, read in the scope `scope_id`, refers to
    /// when it names a `const` declared without a type annotation, or an
    /// import.
    fn visit_name(
        &mut self,
        module: ModuleInfo,
        js_info: &JsModuleInfo,
        scope_id: ScopeId,
        name: &str,
    ) {
        let model = &js_info.semantic_model;
        let mut scope = model.scope_from_id(scope_id);
        for _ in 0..MAX_SCOPE_STEPS {
            if let Some(reference) = scope.get_binding_reference(name) {
                if let Some(binding) = model.binding_by_id(reference.value_ty_or_ty()) {
                    self.visit_binding(module, js_info, &binding);
                }
                return;
            }
            let Some(parent) = scope.parent() else {
                return;
            };
            scope = parent;
        }
    }

    /// Visits the value of `binding` when it is a `const` declared without a
    /// type annotation, or an import.
    fn visit_binding(&mut self, module: ModuleInfo, js_info: &JsModuleInfo, binding: &Binding) {
        let is_fresh_value = binding.is_imported()
            || matches!(
                binding.tree().declaration(),
                Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator))
                    if declarator.variable_annotation().is_none()
                        && declarator
                            .declaration()
                            .is_some_and(|declaration| declaration.is_const())
            );
        if is_fresh_value && let Some(value) = js_info.raw_binding_types.get(&binding.range()) {
            self.visit(module, js_info, value);
        }
    }

    /// Visits the value of the `const` that `import` names in the module
    /// that declares it.
    fn visit_import(&mut self, module: ModuleInfo, import: &TypeImportQualifier) {
        if import.type_only {
            return;
        }
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => return,
        };
        let resolved = resolve_module_import(
            self.db,
            module,
            &import.specifier,
            ResolutionMode::JavaScript,
        );
        let Some(target) = resolved
            .path()
            .as_path()
            .and_then(|path| self.db.module_for_path(path))
        else {
            return;
        };
        let ExportOriginResult::Found { module, name } = find_export_origin(self.db, target, name)
        else {
            return;
        };
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return;
        };
        match js_info.exports.get(name.text()) {
            Some(JsExport::Own(JsOwnExport::Binding(range))) => {
                if let Some(binding) = js_info.semantic_model.as_binding_by_range(*range) {
                    self.visit_binding(module, js_info, &binding);
                }
            }
            // The value of `export default 1`.
            Some(JsExport::Own(JsOwnExport::Type(type_id))) => {
                let value = TypeReference::Resolved(RawTypeId::Local(*type_id));
                self.visit(module, js_info, &value);
            }
            Some(
                JsExport::Own(JsOwnExport::Namespace(_))
                | JsExport::OwnType(_)
                | JsExport::Reexport(_)
                | JsExport::ReexportType(_),
            )
            | None => {}
        }
    }
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Resolves the type that a mutable location takes from its value, as
    /// [`RawTypeofWidenedLiteralExpression`] describes.
    pub(super) fn resolve_widened_literal(
        &mut self,
        expression: &RawTypeofWidenedLiteralExpression,
    ) -> InferredTypeData<'db> {
        let ty = self.resolve(&expression.value);
        self.widen_fresh_literals(&expression.value, ty, expression.is_return_type)
    }

    /// Returns `ty`, the resolved type of the value whose collected type is
    /// `value`, with its literal types of each kind that the value has a
    /// fresh literal type of widened to their primitive types, as the module
    /// documentation describes. With `is_return_type`, `ty` widens only when
    /// it is a single literal type, as a return type inferred from a
    /// function's body does.
    ///
    /// `ty` is returned unchanged, with its handles and wrappers, when it has
    /// no literal type to widen, which skips the search of the raw tables for
    /// most values.
    pub(super) fn widen_fresh_literals(
        &mut self,
        value: &TypeReference,
        ty: InferredTypeData<'db>,
        is_return_type: bool,
    ) -> InferredTypeData<'db> {
        self.widen_literals_of_kinds(ty, is_return_type, |ctx| {
            fresh_literal_kinds(ctx.db, ctx.module, ctx.js_info, value, is_return_type)
        })
    }

    /// Returns `ty` with every primitive literal type that it includes
    /// widened, as [`Self::widen_fresh_literals`] widens the fresh ones, for
    /// a value whose raw expression is no longer known. With
    /// `is_return_type`, `ty` widens only when it is a single literal type.
    pub(super) fn widen_all_literals(
        &mut self,
        ty: InferredTypeData<'db>,
        is_return_type: bool,
    ) -> InferredTypeData<'db> {
        self.widen_literals_of_kinds(ty, is_return_type, |_| FreshLiteralKinds::all())
    }

    /// Returns `ty` with its literal types of the kinds that `kinds` returns
    /// widened, as [`Self::widen_fresh_literals`] describes. `kinds` runs
    /// only when `ty` has a literal type that could widen.
    fn widen_literals_of_kinds(
        &mut self,
        ty: InferredTypeData<'db>,
        is_return_type: bool,
        kinds: impl FnOnce(&mut Self) -> FreshLiteralKinds,
    ) -> InferredTypeData<'db> {
        let db = self.db;
        let resolved = self.resolve_inferred_type(ty);
        let single = [resolved];
        let members: &[InferredTypeData<'db>] = if let InferredTypeData::Union(union) = resolved {
            union.types(db)
        } else {
            &single
        };
        if is_return_type && members.len() != 1 {
            return ty;
        }
        if !members
            .iter()
            .any(|member| self.literal_of(*member).is_some())
        {
            return ty;
        }
        let kinds = kinds(self);
        if kinds.is_empty() {
            return ty;
        }
        let mut is_widened = false;
        let widened: Vec<_> = members
            .iter()
            .map(|member| {
                let literal = self.literal_of(*member);
                match literal.and_then(|literal| widen_literal_of_kinds(db, kinds, literal)) {
                    Some(primitive) => {
                        is_widened = true;
                        primitive
                    }
                    None => *member,
                }
            })
            .collect();
        if !is_widened {
            return ty;
        }
        match widened.as_slice() {
            [single] => *single,
            _ => InferredTypeData::union_from_types(db, widened),
        }
    }

    /// Returns the literal type that `ty` stands for, looking through local
    /// handles and the `typeof` of a named value, which a shorthand property
    /// such as `{ K }` has, or `None` when `ty` isn't a literal type.
    fn literal_of(&mut self, mut ty: InferredTypeData<'db>) -> Option<InferredTypeData<'db>> {
        for _ in 0..MAX_WRAPPER_STEPS {
            ty = self.resolve_inferred_type(ty);
            if matches!(ty, InferredTypeData::Literal(_)) {
                return Some(ty);
            }
            let InferredTypeData::TypeofValue(value) = ty else {
                return None;
            };
            ty = value.ty(self.db);
        }
        None
    }

    /// Resolves `raw`, the type of an object expression, whose data is
    /// `object`, with the fresh literal types of the properties that no
    /// `as const` asserts widened, as TypeScript widens them where no
    /// contextual type keeps them: `{ a: 1 }` is `{ a: number }`, while
    /// `{ a: 1 as const }` is `{ a: 1 }`.
    pub(super) fn resolve_object_expression(
        &mut self,
        raw: &RawTypeData,
        object: &RawObject,
    ) -> InferredTypeData<'db> {
        debug_assert_eq!(object.kind, ObjectKind::Expression);
        let db = self.db;
        let ty = InferredTypeData::from_raw_with_resolver(db, raw, false, &mut |reference| {
            self.resolve(reference)
        });
        let InferredTypeData::Object(inferred) = ty else {
            return ty;
        };
        let members = inferred.members(db);
        let mut widened_members = None;
        for (index, (raw_member, member)) in object.members.iter().zip(members.iter()).enumerate() {
            if raw_member.kind.is_const_asserted() {
                continue;
            }
            let widened = self.widen_fresh_literals(&raw_member.ty, member.ty, false);
            if widened != member.ty {
                widened_members.get_or_insert_with(|| members.to_vec())[index].ty = widened;
            }
        }
        match widened_members {
            Some(members) => InferredTypeData::Object(InferredObject::new(
                db,
                inferred.prototype(db),
                members.into_boxed_slice(),
                inferred.has_unknown_members(db),
                inferred.kind(db),
            )),
            None => ty,
        }
    }
}
