use super::{
    collected_type_result,
    lookup::{
        MemberLookupMode, MemberLookupResolver, apply_substitutions,
        find_member_type_with_resolver, substitutions_for_instance,
    },
    normalize_structural_type,
    resolver::ResolutionCtx,
};
use crate::db::queries::{
    ResolvedCallArgument, infer_call_expression_return_type_from_args, resolve_callable_function,
};
use biome_js_semantic::ScopeId;
use biome_js_syntax::unescape_js_string_text;
use biome_js_type_info::{
    CallArgumentType as RawCallArgumentType, DestructureField as RawDestructureField,
    Literal as RawLiteral, MemberEqualsPredicate, NarrowingPredicate as RawNarrowingPredicate,
    Path, RawTypeData, TypeId, TypeReference, TypeReferenceQualifier, TypeResolverLevel,
    TypeofExpression as RawTypeofExpression, TypeofTag, global_type_id_for_qualifier, global_types,
    interned_types::{
        CallArgumentType as InferredCallArgumentType, ConditionalSubset, ConditionalType,
        FunctionParameter as InferredFunctionParameter, InternedClass as InferredClass,
        InternedConstructor as InferredConstructor, InternedFunction as InferredFunction,
        InternedGenericTypeParameter as InferredGenericTypeParameter,
        InternedLiteral as InferredInternedLiteral, InternedTuple as InferredTuple,
        Literal as InferredLiteral, LocalTypeHandle as InferredLocalTypeHandle,
        NamedFunctionParameter as InferredNamedFunctionParameter,
        NarrowingPredicate as InferredNarrowingPredicate,
        PredicateCallPredicate as InferredPredicateCallPredicate, ReturnType as InferredReturnType,
        TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeofExpression as InferredTypeofExpression,
    },
    literal::NumberLiteral,
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_CONDITIONAL_TYPE_STEPS: usize = 1024;
const MAX_CONDITIONAL_FILTER_STEPS: usize = 1024;
const MAX_PROMISE_UNWRAP_STEPS: usize = 64;
const MAX_REST_MEMBER_STEPS: usize = 1024;
const MAX_AWAIT_EXPRESSION_STEPS: usize = 1024;
const MAX_CALL_CALLEE_STEPS: usize = 64;
const MAX_ELEMENT_INDEX_STEPS: usize = 1024;

/// `Promise.prototype` methods that receive synthesized signatures during
/// member lookup, parsed from the member name.
#[derive(Clone, Copy)]
enum PromiseInstanceMethod {
    Catch,
    Finally,
    Then,
}

impl std::str::FromStr for PromiseInstanceMethod {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "catch" => Ok(Self::Catch),
            "finally" => Ok(Self::Finally),
            "then" => Ok(Self::Then),
            _ => Err(()),
        }
    }
}

impl<'db> MemberLookupResolver<'db> for ResolutionCtx<'db, '_> {
    fn resolve_type(
        &mut self,
        _db: &'db dyn crate::ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_inferred_type(ty)
    }

    fn finalize_member_type(
        &mut self,
        db: &'db dyn crate::ModuleDb,
        ty: InferredTypeData<'db>,
        is_optional: bool,
        substitutions: &[biome_js_type_info::interned_types::TypeSubstitution<'db>],
        crossed_instance: bool,
    ) -> InferredTypeData<'db> {
        let ty = if crossed_instance {
            self.resolve_member_references(ty)
        } else {
            ty
        };
        let Ok(ty) = normalize_structural_type(db, ty, |ty| ty) else {
            return InferredTypeData::Unknown;
        };
        let ty = apply_substitutions(db, ty, substitutions);
        self.member_type(ty, is_optional)
    }
}

impl<'db> ResolutionCtx<'db, '_> {
    pub(in crate::db::type_inference) fn resolve_typeof_expression(
        &mut self,
        expression: &RawTypeofExpression,
    ) -> Option<InferredTypeData<'db>> {
        match expression {
            RawTypeofExpression::Addition(expression) => {
                let left = self.resolve(&expression.left);
                let right = self.resolve(&expression.right);
                self.resolve_addition_expression(left, right)
            }
            RawTypeofExpression::Await(expression) => {
                let argument = self.resolve(&expression.argument);
                self.resolve_await_expression(argument)
            }
            RawTypeofExpression::BitwiseNot(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_number_or_bigint_unary_expression(argument))
            }
            RawTypeofExpression::Call(expression) => {
                let callee = self.resolve(&expression.callee);
                Some(self.resolve_call_expression(callee, &expression.arguments))
            }
            RawTypeofExpression::Conditional(expression) => {
                let test = self.resolve(&expression.test);
                let consequent = self.resolve(&expression.consequent);
                let alternate = self.resolve(&expression.alternate);
                self.resolve_conditional_expression(test, consequent, alternate)
            }
            RawTypeofExpression::Destructure(expression) => {
                let subject = self.resolve(&expression.ty);
                match &expression.destructure_field {
                    RawDestructureField::Index(index) => {
                        self.resolve_element_type_at_index(subject, *index)
                    }
                    RawDestructureField::Name(name) => {
                        self.resolve_static_member_expression(subject, name.text())
                    }
                    RawDestructureField::RestExcept(names) => {
                        Some(self.resolve_rest_except_expression(subject, names))
                    }
                    RawDestructureField::RestFrom(index) => {
                        self.resolve_element_types_from_index(subject, *index)
                    }
                }
            }
            RawTypeofExpression::Index(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_element_type_at_index(object, expression.index)
            }
            RawTypeofExpression::OptionalChainIndex(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_element_type_at_index(object, expression.index)
                    .map(|result| self.optional_chain_result(object, result))
            }
            RawTypeofExpression::IterableValueOf(expression) => {
                let ty = self.resolve(&expression.ty);
                self.resolve_iterable_value_type(ty)
            }
            RawTypeofExpression::LogicalAnd(expression) => {
                let left = self.resolve(&expression.left);
                let right = self.resolve(&expression.right);
                self.resolve_logical_and_expression(left, right)
            }
            RawTypeofExpression::LogicalOr(expression) => {
                let left = self.resolve(&expression.left);
                let right = self.resolve(&expression.right);
                self.resolve_logical_or_expression(left, right)
            }
            RawTypeofExpression::Narrowed(expression) => {
                let ty = self.resolve(&expression.ty);
                let predicate = match &expression.predicate {
                    RawNarrowingPredicate::Assigned(assigned) => {
                        InferredNarrowingPredicate::Assigned(self.resolve(assigned))
                    }
                    RawNarrowingPredicate::Falsy => InferredNarrowingPredicate::Falsy,
                    RawNarrowingPredicate::InstanceOf(guard) => {
                        InferredNarrowingPredicate::InstanceOf(self.resolve(guard))
                    }
                    RawNarrowingPredicate::MemberEquals(predicate) => {
                        InferredNarrowingPredicate::MemberEquals(predicate.clone())
                    }
                    RawNarrowingPredicate::PredicateCall(predicate) => {
                        InferredNarrowingPredicate::PredicateCall(InferredPredicateCallPredicate {
                            callee: self.resolve(&predicate.callee),
                            argument_index: predicate.argument_index,
                        })
                    }
                    RawNarrowingPredicate::StringEquals(value) => {
                        InferredNarrowingPredicate::StringEquals(value.clone())
                    }
                    RawNarrowingPredicate::Truthy => InferredNarrowingPredicate::Truthy,
                    RawNarrowingPredicate::Typeof(tag) => InferredNarrowingPredicate::Typeof(*tag),
                };
                Some(self.resolve_narrowed_expression(ty, &predicate))
            }
            RawTypeofExpression::New(expression) => {
                let callee = self.resolve(&expression.callee);
                let arguments = self.resolve_call_arguments(&expression.arguments);
                let arguments = arguments
                    .into_iter()
                    .map(ResolvedCallArgument::ty)
                    .collect::<Vec<_>>();
                self.resolve_new_expression(callee, &arguments)
            }
            RawTypeofExpression::NullishCoalescing(expression) => {
                let left = self.resolve(&expression.left);
                let right = self.resolve(&expression.right);
                self.resolve_nullish_coalescing_expression(left, right)
            }
            RawTypeofExpression::StaticMember(expression) => {
                let object = self.resolve_static_member_object(&expression.object);
                self.resolve_static_member_expression(object, expression.member.text())
            }
            RawTypeofExpression::OptionalChainStaticMember(expression) => {
                let object = self.resolve_static_member_object(&expression.object);
                self.resolve_static_member_expression(object, expression.member.text())
                    .map(|result| self.optional_chain_result(object, result))
            }
            RawTypeofExpression::Super(expression) => {
                let parent = self.resolve(&expression.parent);
                Some(self.resolve_super_expression(parent))
            }
            RawTypeofExpression::This(expression) => {
                let parent = self.resolve_this_parent(&expression.parent);
                Some(self.resolve_this_expression(parent))
            }
            RawTypeofExpression::Typeof(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_typeof_operator(argument))
            }
            RawTypeofExpression::UnaryMinus(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_number_or_bigint_unary_expression(argument))
            }
        }
    }

    pub(in crate::db::type_inference) fn resolve_inferred_typeof_expression(
        &mut self,
        expression: &InferredTypeofExpression<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match expression {
            InferredTypeofExpression::Addition(expression) => {
                self.resolve_addition_expression(expression.left, expression.right)
            }
            InferredTypeofExpression::Await(expression) => {
                self.resolve_await_expression(expression.argument)
            }
            InferredTypeofExpression::BitwiseNot(expression) => {
                Some(self.resolve_number_or_bigint_unary_expression(expression.argument))
            }
            InferredTypeofExpression::Call(expression) => Some(
                self.resolve_inferred_call_expression(expression.callee, &expression.arguments),
            ),
            InferredTypeofExpression::Conditional(expression) => self
                .resolve_conditional_expression(
                    expression.test,
                    expression.consequent,
                    expression.alternate,
                ),
            InferredTypeofExpression::Destructure(expression) => {
                match &expression.destructure_field {
                    RawDestructureField::Index(index) => {
                        self.resolve_element_type_at_index(expression.ty, *index)
                    }
                    RawDestructureField::Name(name) => {
                        self.resolve_static_member_expression(expression.ty, name.text())
                    }
                    RawDestructureField::RestExcept(names) => {
                        Some(self.resolve_rest_except_expression(expression.ty, names))
                    }
                    RawDestructureField::RestFrom(index) => {
                        self.resolve_element_types_from_index(expression.ty, *index)
                    }
                }
            }
            InferredTypeofExpression::Index(expression) => {
                self.resolve_element_type_at_index(expression.object, expression.index)
            }
            InferredTypeofExpression::OptionalChainIndex(expression) => self
                .resolve_element_type_at_index(expression.object, expression.index)
                .map(|result| self.optional_chain_result(expression.object, result)),
            InferredTypeofExpression::IterableValueOf(expression) => {
                self.resolve_iterable_value_type(expression.ty)
            }
            InferredTypeofExpression::LogicalAnd(expression) => {
                self.resolve_logical_and_expression(expression.left, expression.right)
            }
            InferredTypeofExpression::LogicalOr(expression) => {
                self.resolve_logical_or_expression(expression.left, expression.right)
            }
            InferredTypeofExpression::Narrowed(expression) => {
                Some(self.resolve_narrowed_expression(expression.ty, &expression.predicate))
            }
            InferredTypeofExpression::New(expression) => {
                let arguments = self.resolve_inferred_call_arguments(&expression.arguments);
                let arguments = arguments
                    .into_iter()
                    .map(ResolvedCallArgument::ty)
                    .collect::<Vec<_>>();
                self.resolve_new_expression(expression.callee, &arguments)
            }
            InferredTypeofExpression::NullishCoalescing(expression) => {
                self.resolve_nullish_coalescing_expression(expression.left, expression.right)
            }
            InferredTypeofExpression::StaticMember(expression) => {
                self.resolve_static_member_expression(expression.object, expression.member.text())
            }
            InferredTypeofExpression::OptionalChainStaticMember(expression) => self
                .resolve_static_member_expression(expression.object, expression.member.text())
                .map(|result| self.optional_chain_result(expression.object, result)),
            InferredTypeofExpression::Super(expression) => {
                Some(self.resolve_super_expression(expression.parent))
            }
            InferredTypeofExpression::This(expression) => {
                Some(self.resolve_this_expression(expression.parent))
            }
            InferredTypeofExpression::Typeof(expression) => {
                Some(self.resolve_typeof_operator(expression.argument))
            }
            InferredTypeofExpression::UnaryMinus(expression) => {
                Some(self.resolve_number_or_bigint_unary_expression(expression.argument))
            }
        }
    }

    fn resolve_conditional_expression(
        &mut self,
        test: InferredTypeData<'db>,
        consequent: InferredTypeData<'db>,
        alternate: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let conditional = self.conditional_type(test);
        if conditional.is_truthy() {
            Some(consequent)
        } else if conditional.is_falsy() {
            Some(alternate)
        } else {
            conditional.is_inferred().then(|| {
                InferredTypeData::union_from_types(self.db, Vec::from([consequent, alternate]))
            })
        }
    }

    fn resolve_logical_and_expression(
        &mut self,
        left: InferredTypeData<'db>,
        right: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let conditional = self.conditional_type(left);
        if conditional.is_falsy() {
            Some(left)
        } else if conditional.is_truthy() {
            Some(right)
        } else {
            conditional.is_inferred().then(|| {
                let left = self
                    .filter_type_to_subset(left, ConditionalSubset::Falsy)
                    .unwrap_or(left);
                InferredTypeData::union_from_types(self.db, Vec::from([left, right]))
            })
        }
    }

    fn resolve_logical_or_expression(
        &mut self,
        left: InferredTypeData<'db>,
        right: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let conditional = self.conditional_type(left);
        if conditional.is_truthy() {
            Some(left)
        } else if conditional.is_falsy() {
            Some(right)
        } else {
            conditional.is_inferred().then(|| {
                let left = self
                    .filter_type_to_subset(left, ConditionalSubset::Truthy)
                    .unwrap_or(left);
                InferredTypeData::union_from_types(self.db, Vec::from([left, right]))
            })
        }
    }

    fn resolve_nullish_coalescing_expression(
        &mut self,
        left: InferredTypeData<'db>,
        right: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let conditional = self.conditional_type(left);
        if conditional.is_non_nullish() {
            Some(left)
        } else if conditional.is_nullish() {
            Some(right)
        } else {
            conditional.is_inferred().then(|| {
                let left = self
                    .filter_type_to_subset(left, ConditionalSubset::NonNullish)
                    .unwrap_or(left);
                InferredTypeData::union_from_types(self.db, Vec::from([left, right]))
            })
        }
    }

    fn resolve_call_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        arguments: &[RawCallArgumentType],
    ) -> InferredTypeData<'db> {
        let args = self.resolve_call_arguments(arguments);
        let callee = self.resolve_call_callee(callee);
        infer_call_expression_return_type_from_args(self.db, callee, &args)
    }

    /// Resolves the parent reference of a `this` expression.
    ///
    /// A parent whose type is still being inferred resolves to a local type
    /// handle instead of recursing into it; member lookups on the handle read
    /// the raw declaration through
    /// [`Self::resolve_in_progress_local_member`]. This covers both thin
    /// references to the enclosing declaration and qualifiers that name a
    /// binding under inference.
    fn resolve_this_parent(&mut self, parent: &TypeReference) -> InferredTypeData<'db> {
        if let TypeReference::Resolved(resolved_id) = parent
            && resolved_id.level() == TypeResolverLevel::Thin
            && self.in_progress.contains(&resolved_id.id())
        {
            return InferredTypeData::Local(InferredLocalTypeHandle::new(
                self.db,
                self.module_key,
                biome_js_type_info::interned_types::LocalTypeId::new(resolved_id.id().index()),
            ));
        }
        if let TypeReference::Qualifier(qualifier) = parent
            && let Some(parent) = self.resolve_in_progress_this_qualifier(qualifier)
        {
            return parent;
        }
        self.resolve(parent)
    }

    /// Resolves the object of a static member expression.
    ///
    /// An object that is a thin reference to a `typeof this` expression
    /// resolves through the `this` handling, so the member lookup receives an
    /// instance type that carries the enclosing type's parameters. Any other
    /// object resolves through [`Self::resolve`].
    fn resolve_static_member_object(&mut self, object: &TypeReference) -> InferredTypeData<'db> {
        if let TypeReference::Resolved(resolved_id) = object
            && resolved_id.level() == TypeResolverLevel::Thin
            && let Some(RawTypeData::TypeofExpression(expression)) =
                self.js_info.raw_types.get(resolved_id.id().index())
            && let RawTypeofExpression::This(expression) = expression.as_ref()
        {
            let parent = self.resolve_this_parent(&expression.parent);
            return self.resolve_this_expression(parent);
        }
        self.resolve(object)
    }

    /// Resolves a `this` expression to an instance of its parent type.
    ///
    /// The instance carries the parent's declared type parameters as its type
    /// arguments, so members looked up through `this` keep their references
    /// to those parameters intact. A parent that already is an instance
    /// passes through unchanged; a parent without type parameters produces a
    /// bare instance.
    fn resolve_this_expression(&mut self, parent: InferredTypeData<'db>) -> InferredTypeData<'db> {
        if matches!(parent, InferredTypeData::InstanceOf(_)) {
            return parent;
        }
        let type_parameters = match parent {
            InferredTypeData::Class(class) => class.type_parameters(self.db).to_vec(),
            InferredTypeData::Interface(interface) => interface.type_parameters(self.db).to_vec(),
            InferredTypeData::Local(local) if local.module(self.db) == self.module_key => {
                let type_id = TypeId::new(local.type_id(self.db).index());
                match self.js_info.raw_types.get(type_id.index()) {
                    Some(RawTypeData::Class(class)) => class
                        .type_parameters
                        .iter()
                        .map(|ty| self.resolve(ty))
                        .collect(),
                    Some(RawTypeData::Interface(interface)) => interface
                        .type_parameters
                        .iter()
                        .map(|ty| self.resolve(ty))
                        .collect(),
                    _ => Vec::new(),
                }
            }
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => Vec::new(),
        };
        InferredTypeData::instance_of(self.db, parent, type_parameters.into_boxed_slice())
    }

    fn resolve_inferred_call_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        arguments: &[InferredCallArgumentType<'db>],
    ) -> InferredTypeData<'db> {
        let args = self.resolve_inferred_call_arguments(arguments);
        let callee = self.resolve_call_callee(callee);
        infer_call_expression_return_type_from_args(self.db, callee, &args)
    }

    fn resolve_call_callee(&mut self, mut callee: InferredTypeData<'db>) -> InferredTypeData<'db> {
        let mut instances = Vec::new();
        let mut resolved = InferredTypeData::Unknown;
        for _ in 0..MAX_CALL_CALLEE_STEPS {
            callee = self.resolve_inferred_type(callee);
            callee = callee.expand_canonical_global(self.db);
            let InferredTypeData::InstanceOf(instance) = callee else {
                while let Some(type_parameters) = instances.pop() {
                    callee = InferredTypeData::instance_of(self.db, callee, type_parameters);
                }
                resolved = callee;
                break;
            };
            let type_parameters = instance.type_parameters(self.db);
            callee = self.resolve_inferred_type(instance.ty(self.db));
            if !callee.should_flatten_instance(type_parameters) {
                instances.push(type_parameters.to_vec().into_boxed_slice());
            }
        }

        resolved
    }

    fn resolve_call_arguments(
        &mut self,
        arguments: &[RawCallArgumentType],
    ) -> Vec<ResolvedCallArgument<'db>> {
        let mut args = Vec::new();
        for argument in arguments {
            match argument {
                RawCallArgumentType::Argument(ty) => {
                    args.push(ResolvedCallArgument::Argument(self.resolve(ty)))
                }
                RawCallArgumentType::Spread(ty) => {
                    let ty = self.resolve(ty);
                    self.push_spread_argument(ty, &mut args);
                }
            }
        }
        args
    }

    fn resolve_inferred_call_arguments(
        &mut self,
        arguments: &[InferredCallArgumentType<'db>],
    ) -> Vec<ResolvedCallArgument<'db>> {
        let mut args = Vec::new();
        for argument in arguments {
            match argument {
                InferredCallArgumentType::Argument(ty) => args.push(
                    ResolvedCallArgument::Argument(self.resolve_inferred_type(*ty)),
                ),
                InferredCallArgumentType::Spread(ty) => {
                    let ty = self.resolve_inferred_type(*ty);
                    self.push_spread_argument(ty, &mut args);
                }
            }
        }
        args
    }

    fn push_spread_argument(
        &mut self,
        ty: InferredTypeData<'db>,
        args: &mut Vec<ResolvedCallArgument<'db>>,
    ) {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::InstanceOf(instance) => {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                if let InferredTypeData::Tuple(tuple) = target {
                    self.push_tuple_spread_arguments(tuple, args);
                } else {
                    args.push(ResolvedCallArgument::Spread(InferredTypeData::InstanceOf(
                        instance,
                    )));
                }
            }
            InferredTypeData::Tuple(tuple) => self.push_tuple_spread_arguments(tuple, args),
            ty @ (InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword) => args.push(ResolvedCallArgument::Spread(ty)),
        }
    }

    fn push_tuple_spread_arguments(
        &mut self,
        tuple: InferredTuple<'db>,
        args: &mut Vec<ResolvedCallArgument<'db>>,
    ) {
        for element in tuple.elements(self.db) {
            let ty = self.optional_element_type(element.ty, element.is_optional || element.is_rest);
            if element.is_optional || element.is_rest {
                args.push(ResolvedCallArgument::Optional(ty));
            } else {
                args.push(ResolvedCallArgument::Argument(ty));
            }
        }
    }

    fn resolve_new_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        args: &[InferredTypeData<'db>],
    ) -> Option<InferredTypeData<'db>> {
        let callee = self.resolve_inferred_type(callee);
        let (class_ty, class, explicit_type_parameters) = match callee {
            InferredTypeData::Class(class) => (callee, class, Box::default()),
            InferredTypeData::InstanceOf(instance) => {
                let class_ty = self.resolve_inferred_type(instance.ty(self.db));
                let InferredTypeData::Class(class) = class_ty else {
                    return None;
                };
                (
                    class_ty,
                    class,
                    instance
                        .type_parameters(self.db)
                        .to_vec()
                        .into_boxed_slice(),
                )
            }
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => return None,
        };

        let constructor = class
            .members(self.db)
            .iter()
            .filter(|member| member.kind.is_constructor())
            .find_map(|member| match self.resolve_inferred_type(member.ty) {
                InferredTypeData::Constructor(constructor)
                    if constructor.accepts_argument_count(self.db, args.len()) =>
                {
                    Some(constructor)
                }
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => None,
            });
        let constructed_ty = constructor
            .and_then(|constructor| constructor.return_type(self.db))
            .unwrap_or(class_ty);
        let type_parameters = if !explicit_type_parameters.is_empty() {
            explicit_type_parameters
        } else if constructed_ty == class_ty {
            constructor
                .and_then(|constructor| {
                    self.infer_constructor_type_parameters(class, constructor, args)
                })
                .unwrap_or_default()
        } else {
            Box::default()
        };

        Some(InferredTypeData::instance_of(
            self.db,
            constructed_ty,
            type_parameters,
        ))
    }

    fn infer_constructor_type_parameters(
        &self,
        class: InferredClass<'db>,
        constructor: InferredConstructor<'db>,
        args: &[InferredTypeData<'db>],
    ) -> Option<Box<[InferredTypeData<'db>]>> {
        let declared_parameters = class.type_parameters(self.db);
        if declared_parameters.is_empty() {
            return Some(Box::default());
        }

        let mut inferred_parameters = declared_parameters.to_vec();
        for (parameter, arg) in constructor.parameters(self.db).iter().zip(args) {
            let parameter_ty = parameter.parameter.ty();
            let substitutions = parameter_ty.collect_generic_replacements(self.db, *arg)?;
            for substitution in substitutions {
                for (index, declared_parameter) in declared_parameters.iter().enumerate() {
                    if substitution.generic == *declared_parameter
                        || substitution.generic
                            == InferredTypeData::instance_of(
                                self.db,
                                *declared_parameter,
                                Box::default(),
                            )
                    {
                        inferred_parameters[index] = substitution.replacement;
                    }
                }
            }

            let Some(parameter_function) = resolve_callable_function(self.db, parameter_ty) else {
                continue;
            };
            let InferredReturnType::Type(parameter_return_ty) =
                parameter_function.return_type(self.db)
            else {
                continue;
            };
            let Some(argument_function) = resolve_callable_function(self.db, *arg) else {
                continue;
            };
            let InferredReturnType::Type(argument_return_ty) =
                argument_function.return_type(self.db)
            else {
                continue;
            };

            let substitutions =
                parameter_return_ty.collect_generic_replacements(self.db, *argument_return_ty)?;
            for substitution in substitutions {
                for (index, declared_parameter) in declared_parameters.iter().enumerate() {
                    if substitution.generic == *declared_parameter
                        || substitution.generic
                            == InferredTypeData::instance_of(
                                self.db,
                                *declared_parameter,
                                Box::default(),
                            )
                    {
                        inferred_parameters[index] = substitution.replacement;
                    }
                }
            }
        }

        Some(inferred_parameters.into_boxed_slice())
    }

    pub(in crate::db::type_inference) fn resolve_await_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let mut types = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([argument]);

        for _ in 0..MAX_AWAIT_EXPRESSION_STEPS {
            let Some(ty) = pending.pop() else {
                return collected_type_result(self.db, types);
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }

            if let InferredTypeData::Union(union) = ty {
                pending.extend(union.types(self.db).iter().rev().copied());
            } else if matches!(ty, InferredTypeData::InstanceOf(_)) {
                if let Some(value_ty) = self.resolve_promise_value_type(ty) {
                    pending.push(value_ty);
                } else {
                    types.push(ty);
                }
            } else {
                types.push(ty);
            }
        }

        None
    }

    fn resolve_promise_value_type(
        &mut self,
        ty: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_PROMISE_UNWRAP_STEPS {
            let ty = self.resolve_inferred_type(pending.pop()?);
            if !seen.insert(ty) {
                continue;
            }

            let InferredTypeData::InstanceOf(instance) = ty else {
                continue;
            };
            let target = self.resolve_inferred_type(instance.ty(self.db));
            if self.is_promise_like_target(target) {
                return Some(
                    instance
                        .type_parameters(self.db)
                        .first()
                        .map_or(InferredTypeData::Unknown, |ty| {
                            self.resolve_inferred_type(*ty)
                        }),
                );
            }

            if let InferredTypeData::Class(class) = target
                && let Some(extends) = class.extends(self.db)
            {
                let substitutions = substitutions_for_instance(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                    &[],
                );
                pending.push(apply_substitutions(self.db, extends, &substitutions));
            }
        }

        None
    }

    fn is_promise_like_target(&mut self, target: InferredTypeData<'db>) -> bool {
        match self.resolve_inferred_type(target) {
            target if target.is_promise_class(self.db) => true,
            InferredTypeData::Class(class) => class
                .name(self.db)
                .as_ref()
                .is_some_and(|name| name.text() == "PromiseLike"),
            InferredTypeData::Interface(interface) => {
                interface.name(self.db).text() == "PromiseLike"
            }
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => false,
        }
    }

    fn resolve_super_expression(&mut self, parent: InferredTypeData<'db>) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(parent) {
            InferredTypeData::Class(class) => class
                .extends(self.db)
                .map_or(InferredTypeData::Unknown, |extends| {
                    InferredTypeData::instance_of(self.db, extends, Box::default())
                }),
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => InferredTypeData::Unknown,
        }
    }

    pub(in crate::db::type_inference) fn resolve_static_member_expression(
        &mut self,
        object: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        if let InferredTypeData::Local(local) = object
            && let Some(ty) = self.resolve_in_progress_local_member(local, member_name)
        {
            return Some(ty);
        }
        let object = self.resolve_inferred_type(object);
        let object = object.expand_canonical_global(self.db);
        match object {
            ty @ InferredTypeData::Class(_) => {
                if ty.is_promise_class(self.db) && member_name == "resolve" {
                    return Some(self.promise_resolve_type(ty));
                }
                self.find_member_type_on_resolved_type(ty, member_name, MemberLookupMode::Class)
            }
            InferredTypeData::InstanceOf(instance) => {
                let instance_target = instance.ty(self.db);
                if let InferredTypeData::Local(local) = instance_target
                    && let Some(ty) = self.resolve_in_progress_local_member(local, member_name)
                {
                    return Some(ty);
                }
                let mut target = self.resolve_inferred_type(instance_target);
                target = target.expand_canonical_global(self.db);
                if target.is_array_class(self.db) {
                    target = self.resolve_global_name("Array").unwrap_or(target);
                }
                if target.is_promise_class(self.db)
                    && let Ok(method) = member_name.parse::<PromiseInstanceMethod>()
                {
                    let ty = self.promise_instance_method_type(
                        target,
                        instance.type_parameters(self.db),
                        method,
                    );
                    return Some(self.member_type(ty, false));
                }
                let object = InferredTypeData::instance_of(
                    self.db,
                    target,
                    instance
                        .type_parameters(self.db)
                        .to_vec()
                        .into_boxed_slice(),
                );
                self.find_member_type_on_resolved_type(
                    object,
                    member_name,
                    MemberLookupMode::Instance,
                )
            }
            InferredTypeData::Union(union) => {
                let mut types = Vec::new();
                for ty in union.types(self.db) {
                    match self.resolve_inferred_type(*ty) {
                        InferredTypeData::Undefined => {}
                        InferredTypeData::Unknown => types.push(InferredTypeData::Unknown),
                        ty @ (InferredTypeData::Global
                        | InferredTypeData::GlobalType(_)
                        | InferredTypeData::BigInt
                        | InferredTypeData::Boolean
                        | InferredTypeData::Null
                        | InferredTypeData::Number
                        | InferredTypeData::String
                        | InferredTypeData::Symbol
                        | InferredTypeData::Conditional
                        | InferredTypeData::Class(_)
                        | InferredTypeData::Constructor(_)
                        | InferredTypeData::Function(_)
                        | InferredTypeData::Interface(_)
                        | InferredTypeData::Module(_)
                        | InferredTypeData::Namespace(_)
                        | InferredTypeData::Object(_)
                        | InferredTypeData::Tuple(_)
                        | InferredTypeData::Generic(_)
                        | InferredTypeData::Local(_)
                        | InferredTypeData::Intersection(_)
                        | InferredTypeData::Union(_)
                        | InferredTypeData::TypeOperator(_)
                        | InferredTypeData::Literal(_)
                        | InferredTypeData::InstanceOf(_)
                        | InferredTypeData::MergedReference(_)
                        | InferredTypeData::TypeofExpression(_)
                        | InferredTypeData::TypeofType(_)
                        | InferredTypeData::TypeofValue(_)
                        | InferredTypeData::AnyKeyword
                        | InferredTypeData::NeverKeyword
                        | InferredTypeData::ObjectKeyword
                        | InferredTypeData::ThisKeyword
                        | InferredTypeData::UnknownKeyword
                        | InferredTypeData::VoidKeyword) => {
                            if let Some(member_ty) = self.find_member_type_on_resolved_type(
                                ty,
                                member_name,
                                if matches!(ty, InferredTypeData::Class(_)) {
                                    MemberLookupMode::Class
                                } else {
                                    MemberLookupMode::Instance
                                },
                            ) {
                                types.push(member_ty);
                            }
                        }
                    }
                }
                collected_type_result(self.db, types).or(Some(InferredTypeData::Unknown))
            }
            InferredTypeData::Global => self.resolve_global_name(member_name),
            InferredTypeData::Tuple(tuple) => {
                let element_ty = InferredTypeData::union_from_types(
                    self.db,
                    tuple
                        .elements(self.db)
                        .iter()
                        .map(|element| element.ty)
                        .collect(),
                );
                let target = self.resolve_global_name("Array")?;
                let substitutions = substitutions_for_instance(self.db, target, &[element_ty], &[]);
                self.find_member_type_on_resolved_type(
                    target,
                    member_name,
                    MemberLookupMode::Instance,
                )
                .map(|ty| apply_substitutions(self.db, ty, &substitutions))
            }
            ty @ (InferredTypeData::Unknown
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword) => {
                self.find_member_type_on_resolved_type(ty, member_name, MemberLookupMode::Instance)
            }
        }
    }

    /// Resolves the return type reference of a function-typed member.
    ///
    /// Instance member lookups substitute the instance's type arguments into
    /// the member type; a return type left as an unresolved reference would
    /// hide any generic references from that substitution. Non-function types
    /// are returned unchanged.
    fn resolve_member_references(&mut self, ty: InferredTypeData<'db>) -> InferredTypeData<'db> {
        let InferredTypeData::Function(function) = ty else {
            return ty;
        };
        let return_type = match function.return_type(self.db) {
            InferredReturnType::Type(ty) => {
                InferredReturnType::Type(self.resolve_inferred_type(*ty))
            }
            return_type @ (InferredReturnType::Predicate(_) | InferredReturnType::Asserts(_)) => {
                return_type.clone()
            }
        };
        InferredTypeData::Function(InferredFunction::new(
            self.db,
            function.type_parameters(self.db).clone(),
            function.parameters(self.db).clone(),
            return_type,
            function.is_async(self.db),
            function.name(self.db).clone(),
        ))
    }

    /// Looks up a member on a local type whose inference is still in
    /// progress.
    ///
    /// Inferring a type's members can require a member of the very type being
    /// inferred, for example through `this` or a self-referential binding.
    /// The member is read from the raw class, interface, object, or object
    /// literal declaration and only that member's type is resolved, which
    /// avoids waiting on the containing type. Getters resolve to their return
    /// type. A raw `typeof this` re-enters static member resolution on its
    /// resolved parent.
    ///
    /// Returns `None` when the handle belongs to another module, the type is
    /// not in progress, or the declaration has no member named `member_name`;
    /// callers fall back to regular resolution.
    fn resolve_in_progress_local_member(
        &mut self,
        local: InferredLocalTypeHandle<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        if local.module(self.db) != self.module_key {
            return None;
        }

        let type_id = TypeId::new(local.type_id(self.db).index());
        if !self.in_progress.contains(&type_id) {
            return None;
        }

        let raw = self.js_info.raw_types.get(type_id.index())?;
        if let RawTypeData::TypeofExpression(expression) = raw
            && let RawTypeofExpression::This(expression) = expression.as_ref()
        {
            let parent = self.resolve_this_parent(&expression.parent);
            return self.resolve_static_member_expression(parent, member_name);
        }

        let member = match raw {
            RawTypeData::Class(class) => class
                .members
                .iter()
                .find(|member| member.kind.has_name(member_name)),
            RawTypeData::Interface(interface) => interface
                .members
                .iter()
                .find(|member| member.kind.has_name(member_name)),
            RawTypeData::Object(object) => object
                .members
                .iter()
                .find(|member| member.kind.has_name(member_name)),
            RawTypeData::Literal(literal) => {
                let RawLiteral::Object(object) = literal.as_ref() else {
                    return None;
                };
                object
                    .members()
                    .iter()
                    .find(|member| member.kind.has_name(member_name))
            }
            RawTypeData::Unknown
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
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Module(_)
            | RawTypeData::Namespace(_)
            | RawTypeData::Tuple(_)
            | RawTypeData::Generic(_)
            | RawTypeData::Intersection(_)
            | RawTypeData::Union(_)
            | RawTypeData::TypeOperator(_)
            | RawTypeData::InstanceOf(_)
            | RawTypeData::Reference(_)
            | RawTypeData::MergedReference(_)
            | RawTypeData::TypeofExpression(_)
            | RawTypeData::TypeofType(_)
            | RawTypeData::TypeofValue(_)
            | RawTypeData::AnyKeyword
            | RawTypeData::NeverKeyword
            | RawTypeData::ObjectKeyword
            | RawTypeData::ThisKeyword
            | RawTypeData::UnknownKeyword
            | RawTypeData::VoidKeyword => return None,
        }?;
        let mut ty = self.resolve(&member.ty);
        if member.is_getter()
            && let InferredTypeData::Function(function) = ty
            && let InferredReturnType::Type(return_ty) = function.return_type(self.db)
        {
            ty = *return_ty;
        }
        Some(self.member_type(ty, member.is_optional()))
    }

    fn find_member_type_on_resolved_type(
        &mut self,
        ty: InferredTypeData<'db>,
        member_name: &str,
        mode: MemberLookupMode,
    ) -> Option<InferredTypeData<'db>> {
        find_member_type_with_resolver(self.db, self, ty, member_name, mode)
    }

    /// Builds the simplified `Promise` method type used by call inference.
    ///
    /// `V` is the receiver's first type argument, or `Unknown` when it has no
    /// type argument. The model keeps one fulfillment type and one rejection
    /// type and accepts either a direct value or a `Promise` from each handler.
    /// It omits the library overloads and the `null` and `undefined` alternatives
    /// accepted for handlers. Call inference uses these shapes:
    ///
    /// - `then<F = V, R = never>(onfulfilled?: (value: V) => F | Promise<F>,
    ///   onrejected?: (reason: any) => R | Promise<R>): Promise<F | R>`
    /// - `catch<R = never>(onrejected?: (reason: any) => R | Promise<R>):
    ///   Promise<V | R>`
    /// - `finally(onfinally?: () => void): Promise<V>`
    ///
    /// The handler types default to `V` and `never`. Calls with an omitted
    /// handler therefore retain the receiver value type instead of leaving an
    /// unresolved generic in the result.
    fn promise_instance_method_type(
        &self,
        target: InferredTypeData<'db>,
        receiver_type_parameters: &[InferredTypeData<'db>],
        method: PromiseInstanceMethod,
    ) -> InferredTypeData<'db> {
        let receiver_value = receiver_type_parameters
            .first()
            .copied()
            .unwrap_or(InferredTypeData::Unknown);
        let generic = |name, default| {
            InferredTypeData::Generic(InferredGenericTypeParameter::new(
                self.db,
                None,
                Some(default),
                Text::new_static(name),
            ))
        };
        let function = |type_parameters, parameters, return_type, name| {
            InferredTypeData::Function(InferredFunction::new(
                self.db,
                type_parameters,
                parameters,
                InferredReturnType::Type(return_type),
                false,
                Some(Text::new_static(name)),
            ))
        };
        let callback = |parameters, return_type| {
            function(Box::default(), parameters, return_type, "Promise callback")
        };
        let parameter = |name, ty, is_optional| {
            biome_js_type_info::interned_types::FunctionParameter::Named(
                InferredNamedFunctionParameter {
                    name: Text::new_static(name),
                    ty,
                    is_optional,
                    is_rest: false,
                },
            )
        };
        let promise = |value| {
            InferredTypeData::instance_of(self.db, target, Vec::from([value]).into_boxed_slice())
        };

        match method {
            PromiseInstanceMethod::Finally => {
                let on_finally = callback(Box::default(), InferredTypeData::VoidKeyword);
                function(
                    Box::default(),
                    Vec::from([parameter("onfinally", on_finally, true)]).into_boxed_slice(),
                    promise(receiver_value),
                    "finally",
                )
            }
            PromiseInstanceMethod::Then => {
                let fulfilled = generic("TFulfillmentHandlerValue", receiver_value);
                let rejected = generic("TRejectionHandlerValue", InferredTypeData::NeverKeyword);
                let on_fulfilled = callback(
                    Vec::from([parameter("value", receiver_value, false)]).into_boxed_slice(),
                    InferredTypeData::union_from_types(
                        self.db,
                        Vec::from([fulfilled, promise(fulfilled)]),
                    ),
                );
                let on_rejected = callback(
                    Vec::from([parameter("reason", InferredTypeData::AnyKeyword, false)])
                        .into_boxed_slice(),
                    InferredTypeData::union_from_types(
                        self.db,
                        Vec::from([rejected, promise(rejected)]),
                    ),
                );
                function(
                    Vec::from([fulfilled, rejected]).into_boxed_slice(),
                    Vec::from([
                        parameter("onfulfilled", on_fulfilled, true),
                        parameter("onrejected", on_rejected, true),
                    ])
                    .into_boxed_slice(),
                    promise(InferredTypeData::union_from_types(
                        self.db,
                        Vec::from([fulfilled, rejected]),
                    )),
                    "then",
                )
            }
            PromiseInstanceMethod::Catch => {
                let rejected = generic("TRejectionHandlerValue", InferredTypeData::NeverKeyword);
                let on_rejected = callback(
                    Vec::from([parameter("reason", InferredTypeData::AnyKeyword, false)])
                        .into_boxed_slice(),
                    InferredTypeData::union_from_types(
                        self.db,
                        Vec::from([rejected, promise(rejected)]),
                    ),
                );
                function(
                    Vec::from([rejected]).into_boxed_slice(),
                    Vec::from([parameter("onrejected", on_rejected, true)]).into_boxed_slice(),
                    promise(InferredTypeData::union_from_types(
                        self.db,
                        Vec::from([receiver_value, rejected]),
                    )),
                    "catch",
                )
            }
        }
    }

    /// Builds the single `<T>(value: T) => Promise<T>` shape used for
    /// `Promise.resolve` call inference.
    ///
    /// This omits the overload and unwrapping details of the TypeScript library
    /// declarations. The returned `Promise` is an instance of `target`.
    fn promise_resolve_type(&self, target: InferredTypeData<'db>) -> InferredTypeData<'db> {
        let value = InferredTypeData::Generic(InferredGenericTypeParameter::new(
            self.db,
            None,
            None,
            Text::new_static("TResolveValue"),
        ));
        let parameter = biome_js_type_info::interned_types::FunctionParameter::Named(
            InferredNamedFunctionParameter {
                name: Text::new_static("value"),
                ty: value,
                is_optional: false,
                is_rest: false,
            },
        );
        InferredTypeData::Function(InferredFunction::new(
            self.db,
            Vec::from([value]).into_boxed_slice(),
            Vec::from([parameter]).into_boxed_slice(),
            InferredReturnType::Type(InferredTypeData::instance_of(
                self.db,
                target,
                Vec::from([value]).into_boxed_slice(),
            )),
            false,
            Some(Text::new_static("resolve")),
        ))
    }

    fn resolve_global_name(&mut self, name: &str) -> Option<InferredTypeData<'db>> {
        global_type_id_for_qualifier(&TypeReferenceQualifier::from_path(
            ScopeId::GLOBAL,
            Path::from(Text::new_owned(name.into())),
        ))
        .map(|id| super::globals::global_type(self.db, id))
    }

    fn member_type(
        &mut self,
        ty: InferredTypeData<'db>,
        is_optional: bool,
    ) -> InferredTypeData<'db> {
        if is_optional {
            InferredTypeData::union_from_types(
                self.db,
                Vec::from([ty, InferredTypeData::Undefined]),
            )
        } else {
            self.resolve_inferred_type(ty)
        }
    }

    fn optional_chain_result(
        &mut self,
        object: InferredTypeData<'db>,
        result: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        if self.type_contains_nullish(object) {
            InferredTypeData::union_from_types(
                self.db,
                Vec::from([result, InferredTypeData::Undefined]),
            )
        } else {
            result
        }
    }

    fn type_contains_nullish(&mut self, ty: InferredTypeData<'db>) -> bool {
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_CONDITIONAL_TYPE_STEPS {
            let Some(ty) = pending.pop() else {
                return false;
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }
            match ty {
                InferredTypeData::Null
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword => return true,
                InferredTypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied());
                }
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword => {}
            }
        }

        false
    }

    /// Resolves the type held at a fixed index of `subject`.
    ///
    /// Each member of a union contributes the type it holds at that index, and
    /// the contributions are collected into a union. A member that holds
    /// nothing there, which includes `null` and `undefined`, contributes
    /// nothing rather than discarding what the other members contributed. In
    /// this example the result is `string | number`:
    ///
    /// ```ts
    /// declare const rows: string[] | number[] | null;
    /// rows?.[0];
    /// ```
    ///
    /// The result is `None` when nothing contributes. Nesting deeper than the
    /// work limit yields `Unknown`, because the unvisited members may hold any
    /// type.
    fn resolve_element_type_at_index(
        &mut self,
        subject: InferredTypeData<'db>,
        index: usize,
    ) -> Option<InferredTypeData<'db>> {
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([subject]);
        let mut cursor = 0;
        let mut remaining_steps = MAX_ELEMENT_INDEX_STEPS;
        let mut types = Vec::new();

        // Members are read in order so the collected union keeps the order the
        // source wrote them in.
        while cursor < pending.len() {
            let subject = self.resolve_inferred_type(pending[cursor]);
            cursor += 1;
            if !seen.insert(subject) {
                continue;
            }
            if remaining_steps == 0 {
                return Some(InferredTypeData::Unknown);
            }
            remaining_steps -= 1;

            match subject {
                InferredTypeData::Tuple(tuple) => {
                    if let Some(element) = tuple.elements(self.db).get(index) {
                        let element_ty = self.optional_element_type(
                            element.ty,
                            element.is_optional || element.is_rest,
                        );
                        types.push(element_ty);
                    }
                }
                InferredTypeData::InstanceOf(instance)
                    if self
                        .resolve_inferred_type(instance.ty(self.db))
                        .is_array_class(self.db) =>
                {
                    if let Some(ty) = instance.type_parameters(self.db).first() {
                        let element_ty = self.optional_element_type(*ty, true);
                        types.push(element_ty);
                    }
                }
                InferredTypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied());
                }
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {}
            }
        }

        collected_type_result(self.db, types)
    }

    fn resolve_element_types_from_index(
        &mut self,
        subject: InferredTypeData<'db>,
        index: usize,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(subject) {
            InferredTypeData::Tuple(tuple) => {
                let elements = tuple
                    .elements(self.db)
                    .iter()
                    .skip(index)
                    .cloned()
                    .collect::<Box<[InferredTupleElementType<'db>]>>();
                Some(InferredTypeData::Tuple(InferredTuple::new(
                    self.db, elements,
                )))
            }
            InferredTypeData::InstanceOf(instance)
                if self
                    .resolve_inferred_type(instance.ty(self.db))
                    .is_array_class(self.db) =>
            {
                let type_parameters = instance
                    .type_parameters(self.db)
                    .first()
                    .copied()
                    .into_iter()
                    .collect::<Box<[InferredTypeData<'db>]>>();
                Some(InferredTypeData::array_instance(self.db, type_parameters))
            }
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => None,
        }
    }

    fn resolve_rest_except_expression(
        &mut self,
        subject: InferredTypeData<'db>,
        excluded_names: &[Text],
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(subject) {
            InferredTypeData::Class(class) => {
                let mut members = Vec::new();
                let mut seen_names = Vec::new();
                collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    class.members(self.db),
                    excluded_names,
                    RestMemberMode::ClassStatic,
                );
                InferredTypeData::object_from_members(self.db, members)
            }
            InferredTypeData::InstanceOf(instance) => {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                self.rest_object_from_type(target, excluded_names)
            }
            subject @ (InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword) => self.rest_object_from_type(subject, excluded_names),
        }
    }

    fn rest_object_from_type(
        &mut self,
        ty: InferredTypeData<'db>,
        excluded_names: &[Text],
    ) -> InferredTypeData<'db> {
        let mut members = Vec::new();
        let mut seen_names = Vec::new();
        let mut seen_types = FxHashSet::default();
        let mut pending = Vec::from([ty]);
        for _ in 0..MAX_REST_MEMBER_STEPS {
            let Some(ty) = pending.pop() else {
                break;
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen_types.insert(ty) {
                continue;
            }

            match ty {
                InferredTypeData::Class(class) => {
                    collect_rest_members(
                        &mut members,
                        &mut seen_names,
                        class.members(self.db),
                        excluded_names,
                        RestMemberMode::Instance,
                    );
                    if let Some(extends) = class.extends(self.db) {
                        pending.push(extends);
                    }
                }
                InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                InferredTypeData::Interface(interface) => collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    interface.members(self.db),
                    excluded_names,
                    RestMemberMode::Instance,
                ),
                InferredTypeData::Literal(literal) => {
                    if let InferredLiteral::Object(own_members) = literal.literal(self.db) {
                        collect_rest_members(
                            &mut members,
                            &mut seen_names,
                            own_members,
                            excluded_names,
                            RestMemberMode::Instance,
                        );
                    }
                }
                InferredTypeData::Module(module) => collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    module.members(self.db),
                    excluded_names,
                    RestMemberMode::Instance,
                ),
                InferredTypeData::Namespace(namespace) => collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    namespace.members(self.db),
                    excluded_names,
                    RestMemberMode::Instance,
                ),
                InferredTypeData::Object(object) => {
                    collect_rest_members(
                        &mut members,
                        &mut seen_names,
                        object.members(self.db),
                        excluded_names,
                        RestMemberMode::Instance,
                    );
                    if let Some(prototype) = object.prototype(self.db) {
                        pending.push(prototype);
                    }
                }
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {}
            }
        }

        InferredTypeData::object_from_members(self.db, members)
    }

    fn resolve_iterable_value_type(
        &mut self,
        subject: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let subject = self.resolve_inferred_type(subject);
        let InferredTypeData::InstanceOf(instance) = subject else {
            return None;
        };
        self.resolve_inferred_type(instance.ty(self.db))
            .is_array_class(self.db)
            .then(|| instance.type_parameters(self.db).first().copied())
            .flatten()
    }

    fn optional_element_type(
        &mut self,
        ty: InferredTypeData<'db>,
        is_optional: bool,
    ) -> InferredTypeData<'db> {
        let ty = self.resolve_inferred_type(ty);
        if is_optional {
            InferredTypeData::union_from_types(
                self.db,
                Vec::from([ty, InferredTypeData::Undefined]),
            )
        } else {
            ty
        }
    }

    fn resolve_addition_expression(
        &mut self,
        left: InferredTypeData<'db>,
        right: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match (
            self.coerced_addition_operand_type(left),
            self.coerced_addition_operand_type(right),
        ) {
            (Some(InferredTypeData::BigInt), Some(InferredTypeData::BigInt)) => {
                Some(InferredTypeData::BigInt)
            }
            (Some(InferredTypeData::Number), Some(InferredTypeData::Number)) => {
                Some(InferredTypeData::Number)
            }
            (Some(InferredTypeData::String), _) | (_, Some(InferredTypeData::String)) => {
                Some(InferredTypeData::String)
            }
            (Some(InferredTypeData::Unknown), Some(InferredTypeData::Unknown)) => {
                Some(InferredTypeData::Unknown)
            }
            _ => None,
        }
    }

    fn coerced_addition_operand_type(
        &mut self,
        ty: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::BigInt => Some(InferredTypeData::BigInt),
            InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::Undefined => Some(InferredTypeData::Number),
            InferredTypeData::Class(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::Tuple(_)
            | InferredTypeData::String => Some(InferredTypeData::String),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::BigInt(_) => Some(InferredTypeData::BigInt),
                InferredLiteral::Boolean(_) | InferredLiteral::Number(_) => {
                    Some(InferredTypeData::Number)
                }
                InferredLiteral::Object(_)
                | InferredLiteral::RegExp(_)
                | InferredLiteral::String(_)
                | InferredLiteral::Template(_) => Some(InferredTypeData::String),
            },
            InferredTypeData::Unknown => Some(InferredTypeData::Unknown),
            InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::Symbol
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => None,
        }
    }

    fn resolve_number_or_bigint_unary_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(argument) {
            InferredTypeData::BigInt => InferredTypeData::BigInt,
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => InferredTypeData::Number,
        }
    }

    fn resolve_typeof_operator(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(argument) {
            InferredTypeData::BigInt => self.typeof_string_literal("bigint"),
            InferredTypeData::Boolean => self.typeof_string_literal("boolean"),
            InferredTypeData::Function(_) => self.typeof_string_literal("function"),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::BigInt(_) => self.typeof_string_literal("bigint"),
                InferredLiteral::Boolean(_) => self.typeof_string_literal("boolean"),
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => {
                    self.typeof_string_literal("object")
                }
                InferredLiteral::Number(_) => self.typeof_string_literal("number"),
                InferredLiteral::String(_) | InferredLiteral::Template(_) => {
                    self.typeof_string_literal("string")
                }
            },
            InferredTypeData::Null => self.typeof_string_literal("object"),
            InferredTypeData::Number => self.typeof_string_literal("number"),
            InferredTypeData::Object(_) | InferredTypeData::Tuple(_) => {
                self.typeof_string_literal("object")
            }
            InferredTypeData::String => self.typeof_string_literal("string"),
            InferredTypeData::Symbol => self.typeof_string_literal("symbol"),
            InferredTypeData::Undefined => self.typeof_string_literal("undefined"),
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => self.typeof_return_union(),
        }
    }

    fn typeof_return_union(&self) -> InferredTypeData<'db> {
        global_types(self.db).typeof_return_union()
    }

    fn typeof_string_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        global_types(self.db).typeof_literal(value)
    }

    fn conditional_type(&mut self, ty: InferredTypeData<'db>) -> ConditionalType {
        let mut conditional = ConditionalType::Unknown;
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_CONDITIONAL_TYPE_STEPS {
            let Some(ty) = pending.pop() else {
                return conditional;
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }

            if let Some(next) = ty.conditional_type_shallow(self.db) {
                conditional = if conditional == ConditionalType::Unknown {
                    next
                } else {
                    conditional.merged_with(next)
                };
            } else {
                match ty {
                    InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                    InferredTypeData::Intersection(intersection) => {
                        pending.extend(intersection.types(self.db).iter().rev().copied())
                    }
                    InferredTypeData::MergedReference(reference) => pending.extend(
                        [
                            reference.namespace_ty(self.db),
                            reference.value_ty(self.db),
                            reference.ty(self.db),
                        ]
                        .into_iter()
                        .flatten(),
                    ),
                    InferredTypeData::Union(union) => {
                        pending.extend(union.types(self.db).iter().rev().copied());
                    }
                    InferredTypeData::TypeofExpression(expression) => pending.push(
                        self.resolve_inferred_typeof_expression(expression.expression(self.db))
                            .unwrap_or(InferredTypeData::Unknown),
                    ),
                    InferredTypeData::TypeofType(ty) => pending.push(ty.ty(self.db)),
                    InferredTypeData::TypeofValue(value) => pending.push(value.ty(self.db)),
                    InferredTypeData::Unknown
                    | InferredTypeData::Global
                    | InferredTypeData::GlobalType(_)
                    | InferredTypeData::BigInt
                    | InferredTypeData::Boolean
                    | InferredTypeData::Null
                    | InferredTypeData::Number
                    | InferredTypeData::String
                    | InferredTypeData::Symbol
                    | InferredTypeData::Undefined
                    | InferredTypeData::Conditional
                    | InferredTypeData::Class(_)
                    | InferredTypeData::Constructor(_)
                    | InferredTypeData::Function(_)
                    | InferredTypeData::Interface(_)
                    | InferredTypeData::Module(_)
                    | InferredTypeData::Namespace(_)
                    | InferredTypeData::Object(_)
                    | InferredTypeData::Tuple(_)
                    | InferredTypeData::Generic(_)
                    | InferredTypeData::Local(_)
                    | InferredTypeData::TypeOperator(_)
                    | InferredTypeData::Literal(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::NeverKeyword
                    | InferredTypeData::ObjectKeyword
                    | InferredTypeData::ThisKeyword
                    | InferredTypeData::UnknownKeyword
                    | InferredTypeData::VoidKeyword => return ConditionalType::Unknown,
                }
            }

            if conditional != ConditionalType::Unknown && !conditional.is_mergeable() {
                return conditional;
            }
        }

        ConditionalType::Unknown
    }

    /// Narrows `ty` to the union variants that belong to the given `subset`.
    ///
    /// An empty result means no variant belongs to the subset. For a `typeof`
    /// guard that is conclusive, since a value has exactly one `typeof` tag:
    ///
    /// ```js
    /// function f(x: Promise<void>) {
    ///   if (typeof x === "number") { x; } // x is `never`
    /// }
    /// ```
    ///
    /// The other subsets treat an empty result as indeterminate instead, so
    /// their callers fall back to the un-narrowed type.
    ///
    /// Returns `None` if the type cannot be made any more specific.
    fn filter_type_to_subset(
        &mut self,
        ty: InferredTypeData<'db>,
        subset: ConditionalSubset,
    ) -> Option<InferredTypeData<'db>> {
        let types = self.collect_union_leaves(ty, |ctx, ty| {
            let action = ctx.filter_action(ty, subset);
            let (FilterAction::Retained, InferredTypeData::InstanceOf(instance)) = (&action, ty)
            else {
                return action;
            };

            // An instance classifies by the type it is an instance of, not by
            // the instance type itself, which carries neither a tag nor a
            // truthiness of its own.
            let target = ctx.resolve_inferred_type(instance.ty(ctx.db));
            match subset {
                ConditionalSubset::Typeof(tag) => match ctx.instance_typeof_tag(target) {
                    Some(known_tag) if known_tag == tag => FilterAction::Retained,
                    Some(_) => FilterAction::Stripped,
                    // We cannot determine the tag statically, so we cannot
                    // rule the type out.
                    None => FilterAction::Retained,
                },
                ConditionalSubset::Falsy
                | ConditionalSubset::Truthy
                | ConditionalSubset::NonNullish => {
                    if ctx.instance_excluded_from_subset(target, subset) {
                        FilterAction::Stripped
                    } else {
                        FilterAction::Retained
                    }
                }
            }
        })?;

        match subset {
            ConditionalSubset::Typeof(_) => Some(
                collected_type_result(self.db, types).unwrap_or(InferredTypeData::NeverKeyword),
            ),
            ConditionalSubset::Falsy
            | ConditionalSubset::Truthy
            | ConditionalSubset::NonNullish => collected_type_result(self.db, types),
        }
    }

    fn filter_action(
        &self,
        ty: InferredTypeData<'db>,
        subset: ConditionalSubset,
    ) -> FilterAction<'db> {
        match subset {
            ConditionalSubset::Falsy => match ty {
                InferredTypeData::BigInt => FilterAction::Mapped(self.bigint_literal("0n")),
                InferredTypeData::Boolean => FilterAction::Mapped(self.boolean_literal(false)),
                InferredTypeData::Number => FilterAction::Mapped(self.number_literal("0")),
                InferredTypeData::String => FilterAction::Mapped(self.string_literal("")),
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::Null
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {
                    if self.excluded_from_subset(ty, subset) {
                        FilterAction::Stripped
                    } else {
                        FilterAction::Retained
                    }
                }
            },
            ConditionalSubset::Truthy => match ty {
                InferredTypeData::Boolean => FilterAction::Mapped(self.boolean_literal(true)),
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {
                    if self.excluded_from_subset(ty, subset) {
                        FilterAction::Stripped
                    } else {
                        FilterAction::Retained
                    }
                }
            },
            ConditionalSubset::NonNullish => {
                if self.excluded_from_subset(ty, subset) {
                    FilterAction::Stripped
                } else {
                    FilterAction::Retained
                }
            }
            ConditionalSubset::Typeof(tag) => {
                match self.typeof_tag_of(ty) {
                    Some(known_tag) if known_tag == tag => FilterAction::Retained,
                    Some(_) => FilterAction::Stripped,
                    // We cannot determine the tag statically, so we cannot
                    // rule the type out.
                    None => FilterAction::Retained,
                }
            }
        }
    }

    /// Returns the tag the `typeof` operator evaluates to for values of the
    /// given type, or `None` if the tag cannot be determined statically.
    ///
    /// This mirrors [`Self::resolve_typeof_operator()`], except that types
    /// with call or construct signatures map to the `function` tag, following
    /// TypeScript's narrowing semantics.
    fn typeof_tag_of(&self, ty: InferredTypeData<'db>) -> Option<TypeofTag> {
        match ty {
            InferredTypeData::BigInt => Some(TypeofTag::Bigint),
            InferredTypeData::Boolean => Some(TypeofTag::Boolean),
            // A class value is a constructor function at runtime.
            InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_) => Some(TypeofTag::Function),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::BigInt(_) => Some(TypeofTag::Bigint),
                InferredLiteral::Boolean(_) => Some(TypeofTag::Boolean),
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => Some(TypeofTag::Object),
                InferredLiteral::Number(_) => Some(TypeofTag::Number),
                InferredLiteral::String(_) | InferredLiteral::Template(_) => {
                    Some(TypeofTag::String)
                }
            },
            InferredTypeData::Null => Some(TypeofTag::Object),
            InferredTypeData::Number => Some(TypeofTag::Number),
            InferredTypeData::Interface(interface) => {
                if is_callable_at_runtime(interface.members(self.db)) {
                    Some(TypeofTag::Function)
                } else if interface.extends(self.db).is_empty() {
                    Some(TypeofTag::Object)
                } else {
                    // A base interface could contribute a call or construct
                    // signature.
                    None
                }
            }
            InferredTypeData::Object(object) => {
                if is_callable_at_runtime(object.members(self.db)) {
                    Some(TypeofTag::Function)
                } else {
                    Some(TypeofTag::Object)
                }
            }
            InferredTypeData::Tuple(_) => Some(TypeofTag::Object),
            InferredTypeData::String => Some(TypeofTag::String),
            InferredTypeData::Symbol => Some(TypeofTag::Symbol),
            InferredTypeData::Undefined => Some(TypeofTag::Undefined),
            // A canonical global handle classifies as its definition.
            InferredTypeData::GlobalType(_) => {
                let expanded = ty.expand_canonical_global(self.db);
                if matches!(expanded, InferredTypeData::GlobalType(_)) {
                    None
                } else {
                    self.typeof_tag_of(expanded)
                }
            }
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::Conditional
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => None,
        }
    }

    /// Narrows `ty` according to the given guard `predicate`.
    ///
    /// Returns `ty` unchanged if the predicate cannot make it any more
    /// specific.
    fn resolve_narrowed_expression(
        &mut self,
        ty: InferredTypeData<'db>,
        predicate: &InferredNarrowingPredicate<'db>,
    ) -> InferredTypeData<'db> {
        let narrowed = match predicate {
            InferredNarrowingPredicate::Assigned(assigned) => self.narrow_to_assigned(*assigned),
            InferredNarrowingPredicate::Falsy => {
                self.filter_type_to_subset(ty, ConditionalSubset::Falsy)
            }
            InferredNarrowingPredicate::InstanceOf(guard) => self.narrow_to_instance_of(ty, *guard),
            InferredNarrowingPredicate::MemberEquals(predicate) => {
                self.narrow_by_member_equals(ty, predicate)
            }
            InferredNarrowingPredicate::PredicateCall(predicate) => {
                self.narrow_by_predicate_call(predicate)
            }
            InferredNarrowingPredicate::StringEquals(value) => {
                self.narrow_by_string_equals(ty, value)
            }
            InferredNarrowingPredicate::Truthy => {
                self.filter_type_to_subset(ty, ConditionalSubset::Truthy)
            }
            InferredNarrowingPredicate::Typeof(tag) => {
                self.filter_type_to_subset(ty, ConditionalSubset::Typeof(*tag))
            }
        };
        narrowed.unwrap_or(ty)
    }

    /// Narrows a value to the type it was assigned.
    ///
    /// Returns `None` when the assigned type resolves to `Unknown`, so the
    /// value keeps its declared type.
    fn narrow_to_assigned(
        &mut self,
        assigned: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let assigned = self.resolve_inferred_type(assigned);
        (assigned != InferredTypeData::Unknown).then_some(assigned)
    }

    /// Narrows the union variants of `ty` to those the `leaf` callback
    /// retains.
    ///
    /// See [`Self::collect_union_leaves`] for which types `leaf` gets to
    /// decide on.
    ///
    /// Returns `None` if the type cannot be made any more specific.
    fn narrow_union_leaves(
        &mut self,
        ty: InferredTypeData<'db>,
        leaf: impl FnMut(&mut Self, InferredTypeData<'db>) -> FilterAction<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let types = self.collect_union_leaves(ty, leaf)?;
        collected_type_result(self.db, types)
    }

    /// Collects the union variants of `ty` that the `leaf` callback retains.
    ///
    /// This is the traversal behind every narrowing operation. It expands the
    /// types that stand for other types — nested unions, `typeof` types, and
    /// instances of a type that [should be flattened](InferredTypeData::should_flatten_instance)
    /// — so `leaf` only sees the types that remain once nothing can be
    /// expanded any further, and decides for each whether it is retained,
    /// stripped, or mapped to another type.
    ///
    /// Returns `None` if the traversal does not settle within
    /// [`MAX_CONDITIONAL_FILTER_STEPS`] steps, which bounds the work spent on
    /// cyclic or pathologically nested types.
    fn collect_union_leaves(
        &mut self,
        ty: InferredTypeData<'db>,
        mut leaf: impl FnMut(&mut Self, InferredTypeData<'db>) -> FilterAction<'db>,
    ) -> Option<Vec<InferredTypeData<'db>>> {
        let mut types = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_CONDITIONAL_FILTER_STEPS {
            let Some(ty) = pending.pop() else {
                return Some(types);
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }

            if let InferredTypeData::InstanceOf(instance) = ty {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                if target.should_flatten_instance(instance.type_parameters(self.db)) {
                    pending.push(target);
                    continue;
                }
            }

            match ty {
                InferredTypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().rev().copied());
                }
                InferredTypeData::TypeofExpression(expression) => pending.push(
                    self.resolve_inferred_typeof_expression(expression.expression(self.db))
                        .unwrap_or(InferredTypeData::Unknown),
                ),
                InferredTypeData::TypeofType(inner) => pending.push(inner.ty(self.db)),
                InferredTypeData::TypeofValue(value) => pending.push(value.ty(self.db)),
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => match leaf(self, ty) {
                    FilterAction::Mapped(mapped) => types.push(mapped),
                    FilterAction::Retained => types.push(ty),
                    FilterAction::Stripped => {}
                },
            }
        }

        None
    }

    /// Narrows `ty` to the union variants whose member may strictly equal
    /// the string of the given `predicate`.
    ///
    /// Only variants whose member resolves to a literal type that provably
    /// differs from the string are stripped.
    ///
    /// Returns `None` if the type cannot be made any more specific.
    fn narrow_by_member_equals(
        &mut self,
        ty: InferredTypeData<'db>,
        predicate: &MemberEqualsPredicate,
    ) -> Option<InferredTypeData<'db>> {
        self.narrow_union_leaves(ty, |ctx, ty| {
            if ctx.member_may_equal_string(ty, predicate) {
                FilterAction::Retained
            } else {
                FilterAction::Stripped
            }
        })
    }

    /// Returns whether the member named by the given `predicate` may
    /// strictly equal its string on values of type `ty`.
    ///
    /// Only a member that resolves to a literal type of a provably different
    /// value yields `false`.
    fn member_may_equal_string(
        &mut self,
        ty: InferredTypeData<'db>,
        predicate: &MemberEqualsPredicate,
    ) -> bool {
        let Some(member_ty) = self.resolve_static_member_expression(ty, predicate.member.text())
        else {
            // We cannot tell whether the member exists, let alone its value.
            return true;
        };
        let InferredTypeData::Literal(literal) = self.resolve_inferred_type(member_ty) else {
            return true;
        };
        match literal.literal(self.db) {
            // Literal types retain their escape sequences, while the guard
            // value is unescaped; compare the unescaped values.
            InferredLiteral::String(string) => {
                literal_string_may_equal(string.as_str(), predicate.value.text())
            }
            // A template literal's value is not statically known.
            InferredLiteral::Template(_) => true,
            // A literal of another kind is never strictly equal to a string.
            InferredLiteral::BigInt(_)
            | InferredLiteral::Boolean(_)
            | InferredLiteral::Number(_)
            | InferredLiteral::Object(_)
            | InferredLiteral::RegExp(_) => false,
        }
    }

    /// Narrows a value passed as an argument to a call, to the type the
    /// callee's type predicate establishes for it.
    ///
    /// This replaces the value's declared type rather than intersecting
    /// with it; the predicate's type is taken at face value.
    ///
    /// Returns `None` if the callee does not turn out to be a type
    /// predicate over the parameter in the position the value was passed
    /// at.
    fn narrow_by_predicate_call(
        &mut self,
        predicate: &InferredPredicateCallPredicate<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let callee = self.resolve_inferred_type(predicate.callee);
        let function = callee.callable_function(self.db)?;
        let InferredReturnType::Predicate(predicate_return) = function.return_type(self.db) else {
            return None;
        };

        let parameters = function.parameters(self.db);
        // A TS `this` parameter occupies the first slot of the parameter
        // list but no argument position, so it must not count when mapping
        // the argument index. `this` cannot be a formal parameter name, so
        // matching on the name is unambiguous.
        let parameters = match parameters.split_first() {
            Some((InferredFunctionParameter::Named(first), rest)) if first.name == "this" => rest,
            _ => parameters,
        };
        let parameter = parameters.get(predicate.argument_index)?;
        let InferredFunctionParameter::Named(parameter) = parameter else {
            return None;
        };
        if parameter.is_rest || parameter.name != predicate_return.parameter_name {
            return None;
        }

        // A value satisfying the predicate is an instance of its type.
        let target = self.resolve_inferred_type(predicate_return.ty);
        if let InferredTypeData::InstanceOf(_) = target {
            Some(target)
        } else {
            Some(InferredTypeData::instance_of(
                self.db,
                target,
                Box::default(),
            ))
        }
    }

    /// Narrows `ty` to the union variants that may strictly equal the given
    /// string `value`.
    ///
    /// Returns `None` if the type cannot be made any more specific.
    fn narrow_by_string_equals(
        &mut self,
        ty: InferredTypeData<'db>,
        value: &Text,
    ) -> Option<InferredTypeData<'db>> {
        self.narrow_union_leaves(ty, |ctx, ty| {
            if ctx.value_may_equal_string(ty, value) {
                FilterAction::Retained
            } else {
                FilterAction::Stripped
            }
        })
    }

    /// Returns whether values of type `ty` may strictly equal the string
    /// `value`.
    ///
    /// Only types whose values are provably never strings yield `false`.
    fn value_may_equal_string(&mut self, ty: InferredTypeData<'db>, value: &Text) -> bool {
        match ty {
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                // Literal types retain their escape sequences, while the
                // guard value is unescaped; compare the unescaped values.
                InferredLiteral::String(string) => {
                    literal_string_may_equal(string.as_str(), value.text())
                }
                // A template literal's value is not statically known.
                InferredLiteral::Template(_) => true,
                // A literal of another kind is never strictly equal to a
                // string.
                InferredLiteral::BigInt(_)
                | InferredLiteral::Boolean(_)
                | InferredLiteral::Number(_)
                | InferredLiteral::Object(_)
                | InferredLiteral::RegExp(_) => false,
            },
            // A string can only satisfy an object-like type whose members
            // all exist on strings.
            InferredTypeData::Interface(interface) => {
                let members = interface.members(self.db).to_vec();
                self.string_may_satisfy_members(&members)
            }
            InferredTypeData::Object(object) => {
                let members = object.members(self.db).to_vec();
                self.string_may_satisfy_members(&members)
            }
            // A non-flattened instance can only be ruled out when its
            // `typeof` tag is statically known to differ from `"string"`.
            // An unresolvable target, such as a generic type parameter,
            // could stand for a string, so it stays in.
            InferredTypeData::InstanceOf(instance) => {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                match self.instance_typeof_tag(target) {
                    Some(tag) => tag == TypeofTag::String,
                    None => true,
                }
            }
            // Values of these types are never strings.
            InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Function(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Class(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::NeverKeyword
            | InferredTypeData::VoidKeyword => false,
            // A string may satisfy these, or we cannot tell.
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::String
            | InferredTypeData::Conditional
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword => true,
        }
    }

    /// Returns whether a string value may satisfy a type with the given
    /// members, i.e. whether every named instance member also exists on
    /// `String`.
    ///
    /// Members are looked up on an instance of the global `String` class;
    /// the bare `String` primitive contributes no members in member lookup.
    fn string_may_satisfy_members(&mut self, members: &[InferredTypeMember<'db>]) -> bool {
        let Some(string_instance) = self.string_instance() else {
            // Without the global String type we cannot prove any member
            // absent.
            return true;
        };
        for member in members {
            if member.kind.is_static() || member.kind.is_constructor() {
                continue;
            }
            let Some(name) = member.kind.name() else {
                // We cannot reason about unnamed members, such as index
                // signatures.
                continue;
            };
            if self
                .resolve_static_member_expression(string_instance, name.text())
                .is_none()
            {
                return false;
            }
        }
        true
    }

    /// Returns an instance of the global `String` class, resolving it on
    /// first use.
    fn string_instance(&mut self) -> Option<InferredTypeData<'db>> {
        if let Some(instance) = self.string_instance {
            return Some(instance);
        }

        let string_class = self.resolve_global_name("String")?;
        let instance = InferredTypeData::instance_of(self.db, string_class, Box::default());
        self.string_instance = Some(instance);
        Some(instance)
    }

    /// Narrows `ty` to the subset that may be an instance of the `guard`
    /// class.
    ///
    /// Union variants that provably cannot be an instance of the guard class
    /// are stripped, and variants the guard class derives from are replaced
    /// by an instance of the guard class itself.
    ///
    /// Returns `None` if the type cannot be made any more specific.
    fn narrow_to_instance_of(
        &mut self,
        ty: InferredTypeData<'db>,
        guard: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let guard = self
            .resolve_inferred_type(guard)
            .expand_canonical_global(self.db);
        let InferredTypeData::Class(guard_class) = guard else {
            return None;
        };

        self.narrow_union_leaves(ty, |ctx, ty| match ty {
            InferredTypeData::InstanceOf(instance) => {
                let target = ctx.resolve_inferred_type(instance.ty(ctx.db));
                match target.expand_canonical_global(ctx.db) {
                    InferredTypeData::Class(target_class) => {
                        let target_contains_guard =
                            ctx.class_extends_chain_contains(target_class, guard_class);
                        let guard_contains_target =
                            ctx.class_extends_chain_contains(guard_class, target_class);
                        match (target_contains_guard, guard_contains_target) {
                            // The variant is at least as specific as the
                            // guard already.
                            (
                                ExtendsChainLookup::Contains,
                                ExtendsChainLookup::Contains
                                | ExtendsChainLookup::DoesNotContain
                                | ExtendsChainLookup::Unknown,
                            ) => FilterAction::Retained,
                            // The guard is more specific; downcast to it.
                            (
                                ExtendsChainLookup::DoesNotContain | ExtendsChainLookup::Unknown,
                                ExtendsChainLookup::Contains,
                            ) => FilterAction::Mapped(InferredTypeData::instance_of(
                                ctx.db,
                                guard,
                                Box::default(),
                            )),
                            // Both chains were walked to their roots without
                            // meeting the other class; the variant provably
                            // cannot be an instance of the guard class.
                            (
                                ExtendsChainLookup::DoesNotContain,
                                ExtendsChainLookup::DoesNotContain,
                            ) => FilterAction::Stripped,
                            // Without a full proof either way, we cannot
                            // rule the variant out; keep it.
                            (ExtendsChainLookup::DoesNotContain, ExtendsChainLookup::Unknown)
                            | (
                                ExtendsChainLookup::Unknown,
                                ExtendsChainLookup::DoesNotContain | ExtendsChainLookup::Unknown,
                            ) => FilterAction::Retained,
                        }
                    }
                    // We cannot reason about non-class instance targets.
                    InferredTypeData::Unknown
                    | InferredTypeData::Global
                    | InferredTypeData::GlobalType(_)
                    | InferredTypeData::BigInt
                    | InferredTypeData::Boolean
                    | InferredTypeData::Null
                    | InferredTypeData::Number
                    | InferredTypeData::String
                    | InferredTypeData::Symbol
                    | InferredTypeData::Undefined
                    | InferredTypeData::Conditional
                    | InferredTypeData::Constructor(_)
                    | InferredTypeData::Function(_)
                    | InferredTypeData::Interface(_)
                    | InferredTypeData::Module(_)
                    | InferredTypeData::Namespace(_)
                    | InferredTypeData::Object(_)
                    | InferredTypeData::Tuple(_)
                    | InferredTypeData::Generic(_)
                    | InferredTypeData::Local(_)
                    | InferredTypeData::Intersection(_)
                    | InferredTypeData::Union(_)
                    | InferredTypeData::TypeOperator(_)
                    | InferredTypeData::Literal(_)
                    | InferredTypeData::InstanceOf(_)
                    | InferredTypeData::MergedReference(_)
                    | InferredTypeData::TypeofExpression(_)
                    | InferredTypeData::TypeofType(_)
                    | InferredTypeData::TypeofValue(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::NeverKeyword
                    | InferredTypeData::ObjectKeyword
                    | InferredTypeData::ThisKeyword
                    | InferredTypeData::UnknownKeyword
                    | InferredTypeData::VoidKeyword => FilterAction::Retained,
                }
            }
            InferredTypeData::Literal(literal) => match literal.literal(ctx.db) {
                // Object and regular expression literals are objects.
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => FilterAction::Retained,
                // Primitive literals are never class instances.
                InferredLiteral::BigInt(_)
                | InferredLiteral::Boolean(_)
                | InferredLiteral::Number(_)
                | InferredLiteral::String(_)
                | InferredLiteral::Template(_) => FilterAction::Stripped,
            },
            // Values of these types are never class instances.
            InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::NeverKeyword
            | InferredTypeData::VoidKeyword => FilterAction::Stripped,
            // We cannot rule these out; keep them.
            InferredTypeData::Unknown
            | InferredTypeData::Global
            | InferredTypeData::GlobalType(_)
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword => FilterAction::Retained,
        })
    }

    /// Returns whether the extends chain of `class`, including `class`
    /// itself, contains `needle`.
    ///
    /// Only returns [`ExtendsChainLookup::DoesNotContain`] when the chain
    /// was walked all the way to a class without a base class; a chain that
    /// contains a link we cannot resolve to a class, such as a mixin call or
    /// an unresolved import, yields [`ExtendsChainLookup::Unknown`].
    ///
    /// `class A extends B {}` and `class B extends A {}` parse, so the walk
    /// stops as soon as it revisits a class.
    fn class_extends_chain_contains(
        &mut self,
        class: InferredClass<'db>,
        needle: InferredClass<'db>,
    ) -> ExtendsChainLookup {
        let mut current = class;
        let mut seen = FxHashSet::default();
        for _ in 0..MAX_CONDITIONAL_FILTER_STEPS {
            if current == needle {
                return ExtendsChainLookup::Contains;
            }
            if !seen.insert(current) {
                return ExtendsChainLookup::Unknown;
            }
            let Some(extends) = current.extends(self.db) else {
                return ExtendsChainLookup::DoesNotContain;
            };
            match self
                .resolve_inferred_type(extends)
                .expand_canonical_global(self.db)
            {
                InferredTypeData::Class(parent) => current = parent,
                InferredTypeData::Unknown
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::Conditional
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => return ExtendsChainLookup::Unknown,
            }
        }
        ExtendsChainLookup::Unknown
    }

    /// Returns whether `ty` provably cannot belong to `subset`, judged from
    /// its own shallow classification.
    fn excluded_from_subset(&self, ty: InferredTypeData<'db>, subset: ConditionalSubset) -> bool {
        ty.conditional_type_shallow(self.db)
            .is_some_and(|conditional| excluded_from_subset(conditional, subset))
    }

    /// Returns whether instances of the given `target` type provably fall
    /// outside the given `subset`.
    ///
    /// The conditional class of an instance comes from its target: instances
    /// of a truthy target, such as a class or an interface, are objects that
    /// can never be falsy. Targets without a conditional class, such as
    /// generic type parameters, exclude nothing.
    fn instance_excluded_from_subset(
        &self,
        target: InferredTypeData<'db>,
        subset: ConditionalSubset,
    ) -> bool {
        target
            .expand_canonical_global(self.db)
            .conditional_type_shallow(self.db)
            .is_some_and(|conditional| excluded_from_subset(conditional, subset))
    }

    /// Returns the tag the `typeof` operator evaluates to for instances of
    /// the given `target` type, or `None` if the tag cannot be determined
    /// statically.
    fn instance_typeof_tag(&self, target: InferredTypeData<'db>) -> Option<TypeofTag> {
        let target = target.expand_canonical_global(self.db);
        // A class value is itself a function, but its instances are objects.
        if matches!(target, InferredTypeData::Class(_)) {
            Some(TypeofTag::Object)
        } else {
            self.typeof_tag_of(target)
        }
    }

    fn bigint_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::BigInt(Text::new_static(value)),
        ))
    }

    fn boolean_literal(&self, value: bool) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::Boolean(value.into()),
        ))
    }

    fn number_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::Number(NumberLiteral::new(Text::new_static(value))),
        ))
    }

    fn string_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::String(Text::new_static(value).into()),
        ))
    }
}

#[derive(Clone, Copy)]
enum RestMemberMode {
    Instance,
    ClassStatic,
}

fn collect_rest_members<'db>(
    members: &mut Vec<InferredTypeMember<'db>>,
    seen_names: &mut Vec<Text>,
    source_members: &[InferredTypeMember<'db>],
    excluded_names: &[Text],
    mode: RestMemberMode,
) {
    for member in source_members {
        if !rest_member_mode_allows(member, mode) {
            continue;
        }
        let Some(name) = member.kind.name() else {
            continue;
        };
        if excluded_names
            .iter()
            .any(|excluded_name| excluded_name.text() == name.text())
        {
            continue;
        }
        if seen_names
            .iter()
            .any(|seen_name| seen_name.text() == name.text())
        {
            continue;
        }

        seen_names.push(name);
        members.push(member.clone());
    }
}

fn rest_member_mode_allows(member: &InferredTypeMember<'_>, mode: RestMemberMode) -> bool {
    match mode {
        RestMemberMode::Instance => !member.kind.is_static(),
        RestMemberMode::ClassStatic => member.kind.is_static() && !member.kind.is_constructor(),
    }
}

/// Returns whether values of a type with these members are functions at
/// runtime. Construct signatures count, since `typeof Ctor` is `"function"`
/// for an `interface Ctor { new (): T }`.
fn is_callable_at_runtime(members: &[InferredTypeMember<'_>]) -> bool {
    members
        .iter()
        .any(|member| member.kind.is_call_signature() || member.kind.is_constructor())
}

/// Returns whether a value classified as `conditional` provably cannot
/// belong to `subset`.
///
/// A `typeof` subset never excludes anything here: truthiness says nothing
/// about which `typeof` tag a value has, so those variants are decided by
/// their tag instead.
fn excluded_from_subset(conditional: ConditionalType, subset: ConditionalSubset) -> bool {
    match subset {
        ConditionalSubset::Falsy => conditional.is_truthy(),
        ConditionalSubset::Truthy => conditional.is_falsy(),
        ConditionalSubset::NonNullish => conditional.is_nullish(),
        ConditionalSubset::Typeof(_) => false,
    }
}

/// Returns whether a literal string with the given raw `literal` source
/// text may strictly equal the unescaped `value`.
///
/// A replacement character marks a lossy unescape, such as a lone surrogate
/// escape; equality can be neither proven nor refuted then.
fn literal_string_may_equal(literal: &str, value: &str) -> bool {
    let unescaped = unescape_js_string_text(literal);
    unescaped == value || unescaped.contains('\u{fffd}') || value.contains('\u{fffd}')
}

/// Result of searching a class extends chain for a specific class.
#[derive(Clone, Copy)]
enum ExtendsChainLookup {
    /// The chain contains the class.
    Contains,
    /// The chain was walked to its root without finding the class.
    DoesNotContain,
    /// The chain contains a link that could not be resolved to a class.
    Unknown,
}

enum FilterAction<'db> {
    Mapped(InferredTypeData<'db>),
    Retained,
    Stripped,
}
