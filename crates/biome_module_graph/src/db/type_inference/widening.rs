//! Literal widening: the types that mutable locations take from the values
//! that initialize them.
//!
//! TypeScript gives `let n = 1` the type `number` and `const n = 1` the type
//! `1`. The type of a literal expression is a *fresh* literal type, which
//! widens to its primitive type where the value initializes a location that
//! the program can assign later: a `let` or `var` variable, a property of an
//! object expression, a class field that isn't `readonly`, an element of an
//! array expression, or a parameter with a default value. A return type
//! inferred from a function's body widens only when it is a single literal
//! type. A conditional or logical expression passes on the fresh literal
//! types of its operands, and a `const` declared without a type annotation
//! passes on those of its initializer, so `let m = n` also declares `m` as
//! `number`. A literal type that an annotation or `as const` declares is not
//! fresh and stays. A member of an enum, read as `E.A`, has a fresh literal
//! type too, which TypeScript widens to the enum's type. Biome widens it to
//! the primitive type of the member's literal, which holds the enum's values
//! and more.
//!
//! Collection wraps the value of each mutable location in a
//! [`TypeofWidenedLiteralExpression`](biome_js_type_info::TypeofWidenedLiteralExpression),
//! and resolution widens it here. Collection can't tell a fresh literal type
//! from a declared one, because the raw table stores equal types once, so
//! the `1` of `let n = 1` and the `1` of `let n: 1` share one id. This module
//! tells them apart by the raw expression that the value comes from.
//! [`fresh_literal_kinds`] lists the kinds of the value's literal
//! expressions, following conditional and logical expressions and `const`
//! declarations, also into the modules that export them.
//!
//! Widening every literal type of a kind that has a fresh literal gives the
//! same type as TypeScript, which widens only the fresh ones, because the
//! primitive type of that kind absorbs the others, as `string` absorbs `"a"`
//! in `"a" | string`. It is wider only when a literal type is both fresh and
//! declared, as `1` in `flag ? one : 1` with `one: 1`, where TypeScript keeps
//! `1`.

use super::imports::{ExportOriginResult, find_export_origin};
use super::resolver::ResolutionCtx;
use crate::js_module_info::flow::enum_member_name_range;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ResolutionMode, resolve_module_import};
use biome_js_semantic::{Binding, ScopeId};
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_type_info::{
    FreshLiteralKinds, ImportSymbol, Literal, Object as RawObject, ObjectKind,
    PrimitiveLiteralKind, RawTypeData, RawTypeId, TypeImportQualifier, TypeReference,
    TypeResolverLevel, TypeofExpression as RawTypeofExpression, TypeofValue as RawTypeofValue,
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

/// Returns the kinds of the fresh literal types of a value in `module`,
/// given the collected type of the value, `reference`.
///
/// A value has the fresh literal type of a literal expression, of an operand
/// of a conditional or logical expression, of a unary minus applied to a
/// number or bigint literal, of a member of an enum, read as `E.A` or
/// `E["A"]` where `E` names the enum or an import of it, and of the
/// initializer of a `const` declared without a type annotation that it
/// names, also through an import. With
/// `is_return_values`, the value is the union of the values that a function
/// returns, and has the fresh literal types of each of them. Other values,
/// such as calls, other member accesses, assertions such as `1 as 1`, and
/// the literal types of a union that a type annotation writes, have none.
///
/// The search visits at most 64 raw types, and at most 1024 scopes for the
/// declaration of each name. When it stops at either limit, it returns every
/// kind, because widening every literal kind is never narrower than
/// TypeScript.
pub(super) fn fresh_literal_kinds(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    reference: &TypeReference,
    is_return_values: bool,
) -> FreshLiteralKinds {
    let mut search = FreshLiteralSearch::new(db);
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

/// Returns whether [`fresh_literal_kinds`] can find a kind for `reference`,
/// the collected type of a value, with the same `is_return_values`. Decides
/// from the raw type of the value without resolving it. Returns `false` only
/// when the search finds no kind, so a caller can skip resolving a value
/// with no fresh literal type.
pub(super) fn may_have_fresh_literal(
    js_info: &JsModuleInfo,
    reference: &TypeReference,
    is_return_values: bool,
) -> bool {
    if is_return_values
        && let TypeReference::Resolved(id) = reference
        && id.level() == TypeResolverLevel::Thin
        && let Some(RawTypeData::Union(union)) = js_info.raw_types.get(id.id().index())
    {
        // The search takes every kind when the values outnumber its steps.
        let values = union.types();
        return values.len() > MAX_FRESH_LITERAL_STEPS
            || values
                .iter()
                .any(|value| leads_to_fresh_literal(js_info, value));
    }
    leads_to_fresh_literal(js_info, reference)
}

/// Returns whether the search, when it visits `reference`, reaches a raw
/// type that gives a fresh literal type or leads to one. A name or an import
/// always can. A read of a member can only when its object is a name, which
/// can name an enum. A value that the search can't trace counts as giving
/// none, because [`fresh_literal_kinds`] takes it so.
fn leads_to_fresh_literal(js_info: &JsModuleInfo, reference: &TypeReference) -> bool {
    match reference {
        TypeReference::Resolved(id) => {
            if id.level() != TypeResolverLevel::Thin || js_info.is_named_type(id.id()) {
                return false;
            }
            match js_info
                .raw_types
                .get(id.id().index())
                .and_then(|raw| fresh_literal_step(js_info, raw))
            {
                Some(FreshLiteralStep::Member(object, _)) => object_name(js_info, object).is_some(),
                Some(FreshLiteralStep::Untraced) | None => false,
                Some(
                    FreshLiteralStep::Literal(_)
                    | FreshLiteralStep::Value(_)
                    | FreshLiteralStep::Operands(..)
                    | FreshLiteralStep::ShorthandProperty(_),
                ) => true,
            }
        }
        TypeReference::Qualifier(_) | TypeReference::Import(_) => true,
    }
}

/// What the search for fresh literal types finds at a raw type.
enum FreshLiteralStep<'a> {
    /// A literal expression gives a fresh literal type of this kind.
    Literal(PrimitiveLiteralKind),
    /// A name, as `K` in `let n = K`, or a unary minus applied to a number
    /// or bigint literal leads to this value.
    Value(&'a TypeReference),
    /// A conditional or logical expression leads to both of its operands.
    Operands(&'a TypeReference, &'a TypeReference),
    /// A shorthand property, as `K` in `{ K }`, leads to the value it names.
    ShorthandProperty(&'a RawTypeofValue),
    /// A read of a member, as `E.A` or `E["A"]`, with its object and the
    /// member's name. It leads to the member when the object names an enum.
    /// The search can't trace the fresh literal types of other members.
    Member(&'a TypeReference, &'a str),
    /// The search can't trace the fresh literal types of a call, as `id(1)`,
    /// an `await`, or a destructured element. It takes them as giving none.
    Untraced,
}

/// Returns what the search for fresh literal types finds at `raw`, a raw
/// type of `js_info`, or `None` when `raw` neither gives a fresh literal type
/// nor leads to a value that can.
fn fresh_literal_step<'a>(
    js_info: &'a JsModuleInfo,
    raw: &'a RawTypeData,
) -> Option<FreshLiteralStep<'a>> {
    match raw {
        RawTypeData::Literal(literal) => {
            PrimitiveLiteralKind::from_literal_expression(literal).map(FreshLiteralStep::Literal)
        }
        RawTypeData::Reference(
            reference @ (TypeReference::Qualifier(_) | TypeReference::Import(_)),
        ) => Some(FreshLiteralStep::Value(reference)),
        RawTypeData::TypeofValue(value) => Some(FreshLiteralStep::ShorthandProperty(value)),
        RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
            RawTypeofExpression::UnaryMinus(expression) => expression
                .is_literal_argument
                .then_some(FreshLiteralStep::Value(&expression.argument)),
            RawTypeofExpression::Conditional(expression) => Some(FreshLiteralStep::Operands(
                &expression.consequent,
                &expression.alternate,
            )),
            RawTypeofExpression::LogicalAnd(expression) => Some(FreshLiteralStep::Operands(
                &expression.left,
                &expression.right,
            )),
            RawTypeofExpression::LogicalOr(expression) => Some(FreshLiteralStep::Operands(
                &expression.left,
                &expression.right,
            )),
            RawTypeofExpression::NullishCoalescing(expression) => Some(FreshLiteralStep::Operands(
                &expression.left,
                &expression.right,
            )),
            RawTypeofExpression::StaticMember(expression)
            | RawTypeofExpression::OptionalChainStaticMember(expression) => Some(
                FreshLiteralStep::Member(&expression.object, &expression.member),
            ),
            RawTypeofExpression::ComputedMember(expression) => {
                Some(match raw_string_literal(js_info, &expression.member) {
                    Some(key) => FreshLiteralStep::Member(&expression.object, key),
                    None => FreshLiteralStep::Untraced,
                })
            }
            // Biome can't tell whether these give a fresh literal type, as
            // `id(1)` and `await 1` do, so the search takes them as giving
            // none, and records that it can't tell.
            RawTypeofExpression::Await(_)
            | RawTypeofExpression::Call(_)
            | RawTypeofExpression::CallArgument(_)
            | RawTypeofExpression::Destructure(_)
            | RawTypeofExpression::Index(_)
            | RawTypeofExpression::OptionalChainIndex(_) => Some(FreshLiteralStep::Untraced),
            // A widened value has no fresh literal type left, the end of
            // a function gives `undefined` or nothing, and the target of
            // a type predicate is no value. The others give no literal
            // type, or one that a declaration states.
            RawTypeofExpression::WidenedLiteral(_)
            | RawTypeofExpression::ImplicitReturn(_)
            | RawTypeofExpression::InferredPredicate(_)
            | RawTypeofExpression::Addition(_)
            | RawTypeofExpression::BitwiseNot(_)
            | RawTypeofExpression::IterableValueOf(_)
            | RawTypeofExpression::New(_)
            | RawTypeofExpression::Parameter(_)
            | RawTypeofExpression::Super(_)
            | RawTypeofExpression::This(_)
            | RawTypeofExpression::Typeof(_) => None,
        },
        // A reference to another local type stands for an assertion, such
        // as `1 as 1`, whose literal type is not fresh. A union is a type
        // annotation, whose literal types aren't fresh either.
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
        | RawTypeData::VoidKeyword => None,
    }
}

/// Returns the kinds of the fresh literal types of the value in `module`
/// whose collected type is `reference`, as [`fresh_literal_kinds`] finds
/// them, or `None` when Biome can't tell them or can't widen them as
/// TypeScript does.
///
/// Biome can't tell them when the search stops at a limit, and when the
/// value comes from an expression whose type TypeScript can give a fresh
/// literal type that no literal expression of the value shows: a call, as
/// `id(1)`, an `await`, a read of a member, as of a `static readonly`
/// property or of a `const` in a namespace, and a destructured element. A
/// read of an enum member also gives `None`, because TypeScript widens its
/// literal type to the enum's type, and Biome to a primitive type.
pub(super) fn known_fresh_literal_kinds(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    reference: &TypeReference,
) -> Option<FreshLiteralKinds> {
    let mut search = FreshLiteralSearch::new(db);
    search.visit(module, js_info, reference);
    (!search.untraced).then_some(search.kinds)
}

struct FreshLiteralSearch<'a> {
    db: &'a dyn ModuleDb,
    remaining: usize,
    kinds: FreshLiteralKinds,
    /// Whether the search stopped at a limit, or met a value whose fresh
    /// literal types it can't trace or that TypeScript widens to another
    /// type, as an enum member (see [`known_fresh_literal_kinds`]).
    untraced: bool,
}

impl<'a> FreshLiteralSearch<'a> {
    fn new(db: &'a dyn ModuleDb) -> Self {
        Self {
            db,
            remaining: MAX_FRESH_LITERAL_STEPS,
            kinds: FreshLiteralKinds::default(),
            untraced: false,
        }
    }

    fn visit(&mut self, module: ModuleInfo, js_info: &JsModuleInfo, reference: &TypeReference) {
        if self.remaining == 0 {
            self.kinds = FreshLiteralKinds::all();
            self.untraced = true;
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
        match fresh_literal_step(js_info, raw) {
            Some(FreshLiteralStep::Literal(kind)) => self.kinds |= kind,
            Some(FreshLiteralStep::Value(value)) => self.visit(module, js_info, value),
            Some(FreshLiteralStep::Operands(first, second)) => {
                self.visit(module, js_info, first);
                self.visit(module, js_info, second);
            }
            Some(FreshLiteralStep::ShorthandProperty(value)) => {
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
            // Biome can't widen a member read as TypeScript does: it can't
            // trace the fresh literal types of most members, and widens those
            // of an enum member to a primitive type, where TypeScript widens
            // them to the enum's type.
            Some(FreshLiteralStep::Member(object, member)) => {
                self.untraced = true;
                self.visit_enum_member(module, js_info, object, member);
            }
            Some(FreshLiteralStep::Untraced) => self.untraced = true,
            None => {}
        }
    }

    /// Visits the value that `name`, read in the scope `scope_id`, refers to
    /// when it names a `const` declared without a type annotation, or an
    /// import. Takes every kind as fresh when the declaration of `name` lies
    /// beyond [`MAX_SCOPE_STEPS`] scopes.
    fn visit_name(
        &mut self,
        module: ModuleInfo,
        js_info: &JsModuleInfo,
        scope_id: ScopeId,
        name: &str,
    ) {
        match binding_named(js_info, scope_id, name) {
            Ok(Some(binding)) => self.visit_binding(module, js_info, &binding),
            Ok(None) => {}
            Err(ScopeLimit) => {
                self.kinds = FreshLiteralKinds::all();
                self.untraced = true;
            }
        }
    }

    /// Visits the member named `member` of the enum that `object`, the
    /// collected type of the object of a member access, names: an enum
    /// declaration of `module`, or one that an import of `module` binds,
    /// followed through re-exports and other imports. Takes every kind as
    /// fresh when the declaration of the enum's name lies beyond
    /// [`MAX_SCOPE_STEPS`] scopes, or when the imports outlast the steps that
    /// the search has left.
    fn visit_enum_member(
        &mut self,
        mut module: ModuleInfo,
        js_info: &JsModuleInfo,
        object: &TypeReference,
        member: &str,
    ) {
        let mut js_info = js_info;
        let Some((scope_id, name)) = object_name(js_info, object) else {
            return;
        };
        let mut binding = match binding_named(js_info, scope_id, name) {
            Ok(Some(binding)) => binding,
            Ok(None) => return,
            Err(ScopeLimit) => {
                self.kinds = FreshLiteralKinds::all();
                return;
            }
        };
        while self.remaining > 0 {
            self.remaining -= 1;
            if let Some(AnyJsBindingDeclaration::TsEnumDeclaration(declaration)) =
                binding.tree().declaration()
            {
                if let Some(range) = enum_member_name_range(&declaration, member)
                    && let Some(TypeReference::Resolved(id)) = js_info.raw_binding_types.get(&range)
                    && id.level() == TypeResolverLevel::Thin
                    && let Some(RawTypeData::Literal(literal)) =
                        js_info.raw_types.get(id.id().index())
                    && let Some(kind) = PrimitiveLiteralKind::from_literal_expression(literal)
                {
                    self.kinds |= kind;
                }
                return;
            }
            let Some(TypeReference::Import(import)) =
                js_info.raw_binding_types.get(&binding.range())
            else {
                return;
            };
            let Some((origin, origin_info, JsExport::Own(JsOwnExport::Binding(range)))) =
                self.import_origin(module, import)
            else {
                return;
            };
            let Some(next) = origin_info.semantic_model.as_binding_by_range(*range) else {
                return;
            };
            module = origin;
            js_info = origin_info;
            binding = next;
        }
        self.kinds = FreshLiteralKinds::all();
        self.untraced = true;
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
        let Some((module, js_info, export)) = self.import_origin(module, import) else {
            return;
        };
        match export {
            JsExport::Own(JsOwnExport::Binding(range)) => {
                if let Some(binding) = js_info.semantic_model.as_binding_by_range(*range) {
                    self.visit_binding(module, js_info, &binding);
                }
            }
            // The value of `export default 1`.
            JsExport::Own(JsOwnExport::Type(type_id)) => {
                let value = TypeReference::Resolved(RawTypeId::Local(*type_id));
                self.visit(module, js_info, &value);
            }
            JsExport::Own(JsOwnExport::Namespace(_))
            | JsExport::OwnType(_)
            | JsExport::Reexport(_)
            | JsExport::ReexportType(_) => {}
        }
    }

    /// Returns the module that declares what the value import `import` of
    /// `module` names, followed through re-exports, with that module's
    /// information and the export that declares it. Returns `None` for a
    /// type-only import, a namespace import, and an import that doesn't
    /// resolve to an export of a JavaScript module.
    fn import_origin(
        &self,
        module: ModuleInfo,
        import: &TypeImportQualifier,
    ) -> Option<(ModuleInfo, &'a JsModuleInfo, &'a JsExport)> {
        if import.type_only {
            return None;
        }
        let name = match &import.symbol {
            ImportSymbol::Named(name) => name.clone(),
            ImportSymbol::Default => Text::new_static("default"),
            ImportSymbol::All => return None,
        };
        let resolved = resolve_module_import(
            self.db,
            module,
            &import.specifier,
            ResolutionMode::JavaScript,
        );
        let target = self.db.module_for_path(resolved.path().as_path()?)?;
        let ExportOriginResult::Found { module, name } = find_export_origin(self.db, target, name)
        else {
            return None;
        };
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return None;
        };
        Some((module, js_info, js_info.exports.get(name.text())?))
    }
}

/// Reports that the declaration of a name lies beyond [`MAX_SCOPE_STEPS`]
/// scopes from where the name is read.
struct ScopeLimit;

/// Returns the binding that `name`, read in the scope `scope_id` of
/// `js_info`, refers to, or `None` when no scope declares `name`. Returns
/// [`ScopeLimit`] when the declaration lies beyond [`MAX_SCOPE_STEPS`]
/// scopes.
fn binding_named(
    js_info: &JsModuleInfo,
    scope_id: ScopeId,
    name: &str,
) -> Result<Option<Binding>, ScopeLimit> {
    let model = &js_info.semantic_model;
    let mut scope = model.scope_from_id(scope_id);
    for _ in 0..MAX_SCOPE_STEPS {
        if let Some(reference) = scope.get_binding_reference(name) {
            return Ok(model.binding_by_id(reference.value_ty_or_ty()));
        }
        let Some(parent) = scope.parent() else {
            return Ok(None);
        };
        scope = parent;
    }
    Err(ScopeLimit)
}

/// Returns the scope and the name that `object`, the collected type of an
/// expression, reads when the expression is an identifier, as `E` in `E.A`.
fn object_name<'a>(
    js_info: &'a JsModuleInfo,
    object: &'a TypeReference,
) -> Option<(ScopeId, &'a Text)> {
    let qualifier = match object {
        TypeReference::Qualifier(qualifier) => qualifier,
        TypeReference::Resolved(id) if id.level() == TypeResolverLevel::Thin => {
            let Some(RawTypeData::Reference(TypeReference::Qualifier(qualifier))) =
                js_info.raw_types.get(id.id().index())
            else {
                return None;
            };
            qualifier
        }
        TypeReference::Resolved(_) | TypeReference::Import(_) => return None,
    };
    if !qualifier.type_parameters.is_empty() {
        return None;
    }
    Some((qualifier.scope_id, qualifier.path.identifier()?))
}

/// Returns the text of the string literal that `reference`, the collected
/// type of an expression, is, as the key `"A"` of `E["A"]`.
fn raw_string_literal<'a>(js_info: &'a JsModuleInfo, reference: &TypeReference) -> Option<&'a str> {
    let TypeReference::Resolved(id) = reference else {
        return None;
    };
    if id.level() != TypeResolverLevel::Thin {
        return None;
    }
    let RawTypeData::Literal(literal) = js_info.raw_types.get(id.id().index())? else {
        return None;
    };
    let Literal::String(text) = literal.as_ref() else {
        return None;
    };
    Some(text.as_str())
}

impl<'db> ResolutionCtx<'db, '_> {
    /// Resolves the type that a mutable location takes from its value: the
    /// resolved value of `expression` with its fresh literal types widened.
    pub(super) fn resolve_widened_literal(
        &mut self,
        expression: &RawTypeofWidenedLiteralExpression,
    ) -> InferredTypeData<'db> {
        let ty = self.resolve(&expression.value);
        self.widen_fresh_literals(&expression.value, ty, expression.is_return_type)
    }

    /// Returns `ty`, the resolved type of the value whose collected type is
    /// `value`, with literal types widened to their primitive types. A literal
    /// type widens when the value has a fresh literal type of the same kind.
    /// With `is_return_type`, `ty` widens only when it is a single literal
    /// type, as a return type inferred from a function's body does.
    ///
    /// Returns `ty` unchanged, with its handles and wrappers, when it has no
    /// literal type to widen. Checks the raw type of `value` first, and
    /// resolves `ty` only when that raw type can give a fresh literal type.
    /// The full search for fresh literal types runs only when `ty` has a
    /// literal type.
    pub(super) fn widen_fresh_literals(
        &mut self,
        value: &TypeReference,
        ty: InferredTypeData<'db>,
        is_return_type: bool,
    ) -> InferredTypeData<'db> {
        if !may_have_fresh_literal(self.js_info, value, is_return_type) {
            return ty;
        }
        self.widen_literals_of_kinds(ty, is_return_type, |ctx| {
            fresh_literal_kinds(ctx.db, ctx.module, ctx.js_info, value, is_return_type)
        })
    }

    /// Returns `ty`, the resolved type of the value whose collected type is
    /// `value`, with its fresh literal types widened, and whether Biome could
    /// tell which literal types are fresh (see [`known_fresh_literal_kinds`]).
    /// When it can't, every literal type of `ty` widens, which can be wider
    /// than TypeScript.
    pub(super) fn widen_known_fresh_literals(
        &mut self,
        value: &TypeReference,
        ty: InferredTypeData<'db>,
    ) -> (InferredTypeData<'db>, bool) {
        let mut is_known = true;
        let widened = self.widen_literals_of_kinds(ty, false, |ctx| {
            known_fresh_literal_kinds(ctx.db, ctx.module, ctx.js_info, value).unwrap_or_else(|| {
                is_known = false;
                FreshLiteralKinds::all()
            })
        });
        (widened, is_known)
    }

    /// Returns `ty` with every primitive literal type that it includes
    /// widened, for a value whose raw expression isn't known. With
    /// `is_return_type`, `ty` widens only when it is a single literal type.
    pub(super) fn widen_all_literals(
        &mut self,
        ty: InferredTypeData<'db>,
        is_return_type: bool,
    ) -> InferredTypeData<'db> {
        self.widen_literals_of_kinds(ty, is_return_type, |_| FreshLiteralKinds::all())
    }

    /// Returns `ty` with its literal types of the kinds that `kinds` returns
    /// widened to their primitive types. With `is_return_type`, `ty` widens
    /// only when it is a single literal type. `kinds` runs only when `ty` has
    /// a literal type that could widen.
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

    /// Returns the literal type that `ty` stands for, or `None` when `ty`
    /// isn't a literal type. Looks through local handles and through the
    /// `typeof` of a named value, which a shorthand property such as `{ K }`
    /// has.
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

    /// Resolves `raw`, the type of an object expression whose data is
    /// `object`, and widens the fresh literal types of its properties, as
    /// TypeScript does where no contextual type keeps them. A property that
    /// `as const` asserts keeps its literal type, so `{ a: 1 }` is
    /// `{ a: number }`, while `{ a: 1 as const }` is `{ a: 1 }`.
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
