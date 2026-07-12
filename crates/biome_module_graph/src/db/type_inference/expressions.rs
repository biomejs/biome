use super::{
    collected_type_result, expand_canonical_global,
    lookup::{
        StaticMemberMode, apply_substitutions, class_side_type, find_member_in_members_for_mode,
        substitutions_for_instance,
    },
    normalize_structural_type,
    resolver::ResolutionCtx,
};
use crate::db::queries::{ResolvedCallArgument, infer_call_expression_return_type_from_args};
use biome_js_semantic::ScopeId;
use biome_js_type_info::{
    CallArgumentType as RawCallArgumentType, DestructureField as RawDestructureField,
    GLOBAL_RESOLVER, Literal as RawLiteral, Path, RawTypeData, TypeDb, TypeId,
    TypeReferenceQualifier, TypeResolver, TypeofExpression as RawTypeofExpression,
    literal::NumberLiteral,
    resolved::{
        ConditionalSubset, ConditionalType, InferredCallArgumentType, InferredClass,
        InferredConstructor, InferredFunction, InferredInternedLiteral, InferredLiteral,
        InferredLocalTypeHandle, InferredReturnType, InferredTuple, InferredTupleElementType,
        InferredTypeData, InferredTypeMember, InferredTypeSubstitution, InferredTypeofExpression,
    },
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

const MAX_CONDITIONAL_TYPE_STEPS: usize = 1024;
const MAX_CONDITIONAL_FILTER_STEPS: usize = 1024;
const MAX_PROMISE_UNWRAP_STEPS: usize = 64;
const MAX_REST_MEMBER_STEPS: usize = 1024;
const MAX_STATIC_MEMBER_LOOKUP_STEPS: usize = 1024;
const MAX_AWAIT_EXPRESSION_STEPS: usize = 1024;
const MAX_CALL_CALLEE_SPINE_DEPTH: usize = 64;

enum PromiseValueResolution<'db> {
    Found(InferredTypeData<'db>),
    NotPromise,
    Indeterminate,
}

enum ConditionalFilterResult<'db> {
    Complete(Option<InferredTypeData<'db>>),
    Indeterminate,
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
                let object = self.resolve(&expression.object);
                self.resolve_static_member_expression(object, expression.member.text())
            }
            RawTypeofExpression::Super(expression) => {
                let parent = self.resolve(&expression.parent);
                Some(self.resolve_super_expression(parent))
            }
            RawTypeofExpression::This(expression) => {
                let parent = self.resolve(&expression.parent);
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
            InferredTypeofExpression::IterableValueOf(expression) => {
                self.resolve_iterable_value_type(expression.ty)
            }
            InferredTypeofExpression::LogicalAnd(expression) => {
                self.resolve_logical_and_expression(expression.left, expression.right)
            }
            InferredTypeofExpression::LogicalOr(expression) => {
                self.resolve_logical_or_expression(expression.left, expression.right)
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
                let left = match self.filter_type_to_subset(left, ConditionalSubset::Falsy) {
                    ConditionalFilterResult::Complete(Some(filtered)) => filtered,
                    ConditionalFilterResult::Complete(None) => left,
                    ConditionalFilterResult::Indeterminate => InferredTypeData::Unknown,
                };
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
                let left = match self.filter_type_to_subset(left, ConditionalSubset::Truthy) {
                    ConditionalFilterResult::Complete(Some(filtered)) => filtered,
                    ConditionalFilterResult::Complete(None) => left,
                    ConditionalFilterResult::Indeterminate => InferredTypeData::Unknown,
                };
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
                let left = match self.filter_type_to_subset(left, ConditionalSubset::NonNullish) {
                    ConditionalFilterResult::Complete(Some(filtered)) => filtered,
                    ConditionalFilterResult::Complete(None) => left,
                    ConditionalFilterResult::Indeterminate => InferredTypeData::Unknown,
                };
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

    fn resolve_this_expression(&self, parent: InferredTypeData<'db>) -> InferredTypeData<'db> {
        if matches!(parent, InferredTypeData::InstanceOf(_)) {
            return parent;
        }
        let type_parameters: &[InferredTypeData<'db>] = match parent {
            InferredTypeData::Class(class) => class.type_parameters(self.db),
            InferredTypeData::Interface(interface) => interface.type_parameters(self.db),
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::VoidKeyword => &[],
        };
        InferredTypeData::instance_of(self.db, parent, type_parameters.to_vec().into_boxed_slice())
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

    fn resolve_call_callee(&mut self, callee: InferredTypeData<'db>) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();
        let db = self.db;
        let callee = self.resolve_call_callee_spine(callee, MAX_CALL_CALLEE_SPINE_DEPTH, &mut seen);

        normalize_structural_type(db, callee, |ty| {
            if let InferredTypeData::Local(local) = ty
                && local.module(db) == self.module_key
            {
                ty
            } else {
                self.resolve_inferred_type(ty)
            }
        })
        .unwrap_or(InferredTypeData::Unknown)
    }

    fn resolve_call_callee_spine(
        &mut self,
        ty: InferredTypeData<'db>,
        remaining_depth: usize,
        seen: &mut FxHashSet<InferredTypeData<'db>>,
    ) -> InferredTypeData<'db> {
        if remaining_depth == 0 {
            return InferredTypeData::Unknown;
        }

        let ty = self.resolve_inferred_type(ty);
        let ty = expand_canonical_global(self.db, ty);
        if !seen.insert(ty) {
            return InferredTypeData::Unknown;
        }
        let InferredTypeData::InstanceOf(instance) = ty else {
            return ty;
        };

        let target =
            self.resolve_call_callee_spine(instance.ty(self.db), remaining_depth - 1, seen);
        if target.should_flatten_instance(instance.type_parameters(self.db)) {
            target
        } else {
            InferredTypeData::instance_of(
                self.db,
                target,
                instance
                    .type_parameters(self.db)
                    .to_vec()
                    .into_boxed_slice(),
            )
        }
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
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::Divergent(_)
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
                | InferredTypeData::Divergent(_)
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
            match constructor {
                Some(constructor) => {
                    let Some(type_parameters) =
                        infer_constructor_type_parameters(self.db, class, constructor, args)
                    else {
                        return Some(InferredTypeData::Unknown);
                    };
                    type_parameters
                }
                None => Box::default(),
            }
        } else {
            Box::default()
        };

        Some(InferredTypeData::instance_of(
            self.db,
            constructed_ty,
            type_parameters,
        ))
    }

    fn resolve_await_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let mut types = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([argument]);

        for step in 0..MAX_AWAIT_EXPRESSION_STEPS {
            let Some(ty) = pending.pop() else {
                return collected_type_result(self.db, types);
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }

            if matches!(
                ty,
                InferredTypeData::Unknown
                    | InferredTypeData::Divergent(_)
                    | InferredTypeData::Local(_)
                    | InferredTypeData::TypeofExpression(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::UnknownKeyword
            ) {
                return Some(InferredTypeData::Unknown);
            } else if let InferredTypeData::Union(union) = ty {
                let remaining_steps = MAX_AWAIT_EXPRESSION_STEPS - step - 1;
                if union.types(self.db).len() > remaining_steps.saturating_sub(pending.len()) {
                    return Some(InferredTypeData::Unknown);
                }
                pending.extend(union.types(self.db).iter().rev().copied());
            } else if matches!(ty, InferredTypeData::InstanceOf(_)) {
                match self.resolve_promise_value_type(ty) {
                    PromiseValueResolution::Found(value_ty) => {
                        let remaining_steps = MAX_AWAIT_EXPRESSION_STEPS - step - 1;
                        if pending.len() >= remaining_steps {
                            return Some(InferredTypeData::Unknown);
                        }
                        pending.push(value_ty);
                    }
                    PromiseValueResolution::NotPromise => types.push(ty),
                    PromiseValueResolution::Indeterminate => {
                        return Some(InferredTypeData::Unknown);
                    }
                }
            } else {
                types.push(ty);
            }
        }

        Some(InferredTypeData::Unknown)
    }

    fn resolve_promise_value_type(
        &mut self,
        ty: InferredTypeData<'db>,
    ) -> PromiseValueResolution<'db> {
        let mut completed = FxHashSet::default();
        let mut pending = VecDeque::from([(ty, Vec::new())]);
        let mut processed = 0;
        let mut indeterminate = false;

        while let Some((ty, path)) = pending.pop_front() {
            let ty = self.resolve_inferred_type(ty);
            if path.contains(&ty) {
                indeterminate = true;
                continue;
            }
            if !completed.insert(ty) {
                continue;
            }
            if processed == MAX_PROMISE_UNWRAP_STEPS {
                indeterminate = true;
                continue;
            }
            processed += 1;

            if matches!(
                ty,
                InferredTypeData::Unknown
                    | InferredTypeData::Divergent(_)
                    | InferredTypeData::Local(_)
                    | InferredTypeData::TypeofExpression(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::UnknownKeyword
            ) {
                indeterminate = true;
                continue;
            }
            if let InferredTypeData::Union(union) = ty {
                let remaining_steps = MAX_PROMISE_UNWRAP_STEPS - processed;
                if union.types(self.db).len() > remaining_steps.saturating_sub(pending.len()) {
                    return PromiseValueResolution::Indeterminate;
                }
                let mut child_path = path;
                child_path.push(ty);
                pending.extend(
                    union
                        .types(self.db)
                        .iter()
                        .copied()
                        .map(|ty| (ty, child_path.clone())),
                );
                continue;
            }

            if let InferredTypeData::Class(class) = ty
                && let Some(base) = class.extends(self.db)
            {
                if pending.len() >= MAX_PROMISE_UNWRAP_STEPS - processed {
                    return PromiseValueResolution::Indeterminate;
                }
                let mut child_path = path;
                child_path.push(ty);
                pending.push_back((base, child_path));
                continue;
            }
            if let InferredTypeData::Interface(interface) = ty
                && !interface.extends(self.db).is_empty()
            {
                let remaining_steps = MAX_PROMISE_UNWRAP_STEPS - processed;
                if interface.extends(self.db).len() > remaining_steps.saturating_sub(pending.len())
                {
                    return PromiseValueResolution::Indeterminate;
                }
                let mut child_path = path;
                child_path.push(ty);
                pending.extend(
                    interface
                        .extends(self.db)
                        .iter()
                        .copied()
                        .map(|base| (base, child_path.clone())),
                );
                continue;
            }

            let InferredTypeData::InstanceOf(instance) = ty else {
                continue;
            };
            let target = self.resolve_inferred_type(instance.ty(self.db));
            match self.is_promise_like_target(target) {
                Some(true) => {
                    return PromiseValueResolution::Found(
                        instance
                            .type_parameters(self.db)
                            .first()
                            .map_or(InferredTypeData::Unknown, |ty| {
                                self.resolve_inferred_type(*ty)
                            }),
                    );
                }
                None => {
                    indeterminate = true;
                    continue;
                }
                Some(false) => {}
            }

            if let InferredTypeData::InstanceOf(_) = target {
                let mut child_path = path;
                child_path.push(ty);
                pending.push_back((target, child_path));
                continue;
            }
            if let InferredTypeData::Union(union) = target {
                let mut child_path = path;
                child_path.push(ty);
                pending.extend(union.types(self.db).iter().copied().map(|target| {
                    (
                        InferredTypeData::instance_of(
                            self.db,
                            target,
                            instance
                                .type_parameters(self.db)
                                .to_vec()
                                .into_boxed_slice(),
                        ),
                        child_path.clone(),
                    )
                }));
                continue;
            }

            let bases: Vec<InferredTypeData<'db>> = if let InferredTypeData::Class(class) = target {
                class.extends(self.db).into_iter().collect()
            } else if let InferredTypeData::Interface(interface) = target {
                interface.extends(self.db).to_vec()
            } else {
                Vec::new()
            };
            if !bases.is_empty() {
                let Ok(substitutions) = substitutions_for_instance(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                    &[],
                ) else {
                    return PromiseValueResolution::Indeterminate;
                };
                let mut child_path = path;
                child_path.push(ty);
                for base in bases {
                    let Ok(base) = apply_substitutions(self.db, base, &substitutions) else {
                        return PromiseValueResolution::Indeterminate;
                    };
                    pending.push_back((base, child_path.clone()));
                }
            }
        }

        if indeterminate {
            PromiseValueResolution::Indeterminate
        } else {
            PromiseValueResolution::NotPromise
        }
    }

    fn is_promise_like_target(&mut self, target: InferredTypeData<'db>) -> Option<bool> {
        match self.resolve_inferred_type(target) {
            target if target.is_promise_class(self.db) => Some(true),
            InferredTypeData::Class(class) => class
                .name(self.db)
                .as_ref()
                .map_or(Some(false), |name| Some(name.text() == "PromiseLike")),
            InferredTypeData::Interface(interface) => {
                Some(interface.name(self.db).text() == "PromiseLike")
            }
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::UnknownKeyword => None,
            InferredTypeData::Global
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
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::VoidKeyword => Some(false),
            InferredTypeData::Generic(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_) => None,
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
            | InferredTypeData::Divergent(_)
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
        let object = expand_canonical_global(self.db, object);
        match object {
            InferredTypeData::Class(_) => self
                .find_static_member_on_resolved_type(object, member_name, StaticMemberMode::Class)
                .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
            InferredTypeData::InstanceOf(instance) => {
                let target = instance.ty(self.db);
                if let InferredTypeData::Local(local) = target
                    && let Some(ty) = self.resolve_in_progress_local_member(local, member_name)
                {
                    return Some(ty);
                }
                let target = self.resolve_inferred_type(target);
                let target = expand_canonical_global(self.db, target);
                if let Some((ty, is_optional)) = self.promise_member_type(target, member_name) {
                    return Some(self.member_type(ty, is_optional));
                }
                let substitutions = substitutions_for_instance(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                    &[],
                )
                .ok();
                let Some(substitutions) = substitutions else {
                    return Some(InferredTypeData::Unknown);
                };
                if matches!(target, InferredTypeData::Union(_)) {
                    let ty = self.resolve_static_member_expression(target, member_name)?;
                    return Some(self.apply_member_substitutions(ty, &substitutions));
                }
                self.find_static_member_on_resolved_type(
                    target,
                    member_name,
                    StaticMemberMode::Instance,
                )
                .map(|(ty, is_optional)| {
                    let ty = self.apply_member_substitutions(ty, &substitutions);
                    self.member_type(ty, is_optional)
                })
            }
            InferredTypeData::Union(union) => {
                let mut types = Vec::new();
                for ty in union.types(self.db) {
                    match self.resolve_inferred_type(*ty) {
                        InferredTypeData::Undefined => {}
                        InferredTypeData::Unknown => types.push(InferredTypeData::Unknown),
                        ty @ (InferredTypeData::Divergent(_)
                        | InferredTypeData::Global
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
                            if let Some((member_ty, is_optional)) = self
                                .find_static_member_on_resolved_type(
                                    ty,
                                    member_name,
                                    if matches!(ty, InferredTypeData::Class(_)) {
                                        StaticMemberMode::Class
                                    } else {
                                        StaticMemberMode::Instance
                                    },
                                )
                            {
                                types.push(self.member_type(member_ty, is_optional));
                            }
                        }
                    }
                }
                collected_type_result(self.db, types).or(Some(InferredTypeData::Unknown))
            }
            InferredTypeData::Global => self.resolve_global_name(member_name),
            InferredTypeData::GlobalType(_) => None,
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
                let substitutions =
                    substitutions_for_instance(self.db, target, &[element_ty], &[]).ok();
                let Some(substitutions) = substitutions else {
                    return Some(InferredTypeData::Unknown);
                };
                self.find_static_member_on_resolved_type(
                    target,
                    member_name,
                    StaticMemberMode::Instance,
                )
                .map(|(ty, is_optional)| {
                    apply_substitutions(self.db, ty, &substitutions)
                        .map_or(InferredTypeData::Unknown, |ty| {
                            self.member_type(ty, is_optional)
                        })
                })
            }
            ty @ (InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::VoidKeyword) => self
                .find_static_member_on_resolved_type(ty, member_name, StaticMemberMode::Instance)
                .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
        }
    }

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
            let parent = expression.parent.clone();
            let parent = self.resolve(&parent);
            return self.resolve_static_member_expression(parent, member_name);
        }

        let member = match raw {
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
            | RawTypeData::Class(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Interface(_)
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
        let reference = member.ty.clone();
        let is_getter = member.is_getter();
        let is_optional = member.is_optional();
        let mut ty = self.resolve(&reference);
        if is_getter
            && let InferredTypeData::Function(function) = ty
            && let InferredReturnType::Type(return_ty) = function.return_type(self.db)
        {
            ty = *return_ty;
        }
        Some(self.member_type(ty, is_optional))
    }

    fn find_static_member_on_resolved_type(
        &mut self,
        ty: InferredTypeData<'db>,
        member_name: &str,
        mode: StaticMemberMode,
    ) -> Option<(InferredTypeData<'db>, bool)> {
        let mut seen_types = FxHashSet::default();
        let mut pending = Vec::from([ty]);
        while let Some(ty) = pending.pop() {
            let ty = self.resolve_inferred_type(ty);
            let ty = expand_canonical_global(self.db, ty);
            if !seen_types.insert(ty) {
                continue;
            }
            if seen_types.len() > MAX_STATIC_MEMBER_LOOKUP_STEPS {
                return Some((InferredTypeData::Unknown, false));
            }

            match ty {
                InferredTypeData::Class(class) => {
                    if let Some(member) = find_member_in_members_for_mode(
                        self.db,
                        class.members(self.db),
                        member_name,
                        mode,
                    ) {
                        return Some(member);
                    }
                    if let Some(mut extends) = class.extends(self.db) {
                        if pending.len() >= MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len() {
                            return Some((InferredTypeData::Unknown, false));
                        }
                        if matches!(mode, StaticMemberMode::Class) {
                            extends = class_side_type(self.db, extends);
                        }
                        pending.push(extends);
                    }
                }
                InferredTypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(self.db) {
                        if pending.len() >= MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len() {
                            return Some((InferredTypeData::Unknown, false));
                        }
                        pending.push(constraint);
                    } else {
                        return Some((InferredTypeData::Unknown, false));
                    }
                }
                InferredTypeData::InstanceOf(instance) => {
                    if pending.len() >= MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len() {
                        return Some((InferredTypeData::Unknown, false));
                    }
                    pending.push(instance.ty(self.db));
                }
                InferredTypeData::Intersection(intersection) => {
                    if intersection.types(self.db).len()
                        > (MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len())
                            .saturating_sub(pending.len())
                    {
                        return Some((InferredTypeData::Unknown, false));
                    }
                    pending.extend(intersection.types(self.db).iter().rev().copied());
                }
                InferredTypeData::Interface(interface) => {
                    if let Some(member) = find_member_in_members_for_mode(
                        self.db,
                        interface.members(self.db),
                        member_name,
                        mode,
                    ) {
                        return Some(member);
                    }
                    if interface.extends(self.db).len()
                        > (MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len())
                            .saturating_sub(pending.len())
                    {
                        return Some((InferredTypeData::Unknown, false));
                    }
                    pending.extend(interface.extends(self.db).iter().rev().copied());
                }
                InferredTypeData::Literal(literal) => {
                    if let InferredLiteral::Object(members) = literal.literal(self.db)
                        && let Some(member) =
                            find_member_in_members_for_mode(self.db, members, member_name, mode)
                    {
                        return Some(member);
                    }
                }
                InferredTypeData::Module(module) => {
                    if let Some(member) = [StaticMemberMode::Instance, StaticMemberMode::Class]
                        .into_iter()
                        .find_map(|mode| {
                            find_member_in_members_for_mode(
                                self.db,
                                module.members(self.db),
                                member_name,
                                mode,
                            )
                        })
                    {
                        return Some(member);
                    }
                }
                InferredTypeData::Namespace(namespace) => {
                    if let Some(member) = [StaticMemberMode::Instance, StaticMemberMode::Class]
                        .into_iter()
                        .find_map(|mode| {
                            find_member_in_members_for_mode(
                                self.db,
                                namespace.members(self.db),
                                member_name,
                                mode,
                            )
                        })
                    {
                        return Some(member);
                    }
                }
                InferredTypeData::Object(object) => {
                    if let Some(member) = find_member_in_members_for_mode(
                        self.db,
                        object.members(self.db),
                        member_name,
                        mode,
                    ) {
                        return Some(member);
                    }
                    if let Some(prototype) = object.prototype(self.db) {
                        if pending.len() >= MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len() {
                            return Some((InferredTypeData::Unknown, false));
                        }
                        pending.push(prototype);
                    }
                }
                InferredTypeData::MergedReference(reference) => {
                    if reference.targets(self.db).count()
                        > (MAX_STATIC_MEMBER_LOOKUP_STEPS - seen_types.len())
                            .saturating_sub(pending.len())
                    {
                        return Some((InferredTypeData::Unknown, false));
                    }
                    pending.extend(reference.targets(self.db));
                }
                InferredTypeData::Union(union) => {
                    if union.types(self.db).is_empty() {
                        return None;
                    }
                    return Some((InferredTypeData::Unknown, false));
                }
                InferredTypeData::Unknown
                | InferredTypeData::Divergent(_)
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
                | InferredTypeData::Local(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {
                    if matches!(
                        ty,
                        InferredTypeData::Unknown
                            | InferredTypeData::Divergent(_)
                            | InferredTypeData::Global
                            | InferredTypeData::GlobalType(_)
                            | InferredTypeData::Local(_)
                            | InferredTypeData::TypeofExpression(_)
                            | InferredTypeData::AnyKeyword
                            | InferredTypeData::UnknownKeyword
                    ) {
                        return Some((InferredTypeData::Unknown, false));
                    }
                }
            }
        }

        None
    }

    fn promise_member_type(
        &self,
        target: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<(InferredTypeData<'db>, bool)> {
        if !target.is_promise_class(self.db) || !matches!(member_name, "catch" | "finally" | "then")
        {
            return None;
        }

        let return_type = InferredTypeData::instance_of(self.db, target, Box::default());
        Some((
            InferredTypeData::Function(InferredFunction::new(
                self.db,
                Box::default(),
                Box::default(),
                InferredReturnType::Type(return_type),
                false,
                None,
            )),
            false,
        ))
    }

    fn resolve_global_name(&mut self, name: &str) -> Option<InferredTypeData<'db>> {
        GLOBAL_RESOLVER
            .resolve_qualifier(&TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Path::from(Text::new_owned(name.into())),
            ))
            .map(|resolved_id| self.resolve_resolved_id(resolved_id))
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

    fn apply_member_substitutions(
        &mut self,
        ty: InferredTypeData<'db>,
        substitutions: &[InferredTypeSubstitution<'db>],
    ) -> InferredTypeData<'db> {
        let db = self.db;
        let Ok(ty) = normalize_structural_type(db, ty, |ty| self.resolve_inferred_type(ty)) else {
            return InferredTypeData::Unknown;
        };
        apply_substitutions(db, ty, substitutions).unwrap_or(InferredTypeData::Unknown)
    }

    fn resolve_element_type_at_index(
        &mut self,
        subject: InferredTypeData<'db>,
        index: usize,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(subject) {
            InferredTypeData::Tuple(tuple) => {
                let element = tuple.elements(self.db).get(index)?;
                Some(self.optional_element_type(element.ty, element.is_optional || element.is_rest))
            }
            InferredTypeData::InstanceOf(instance)
                if self
                    .resolve_inferred_type(instance.ty(self.db))
                    .is_array_class(self.db) =>
            {
                instance
                    .type_parameters(self.db)
                    .first()
                    .map(|ty| self.optional_element_type(*ty, true))
            }
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::Divergent(_)
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
                let mut seen_names = FxHashSet::default();
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
            | InferredTypeData::Divergent(_)
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
        let db = self.db;
        collect_rest_object_members(db, ty, excluded_names, |ty| self.resolve_inferred_type(ty))
            .map_or(InferredTypeData::Unknown, |members| {
                InferredTypeData::object_from_members(db, members)
            })
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
            InferredTypeData::Divergent(_)
            | InferredTypeData::Global
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
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::Divergent(_)
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
        InferredTypeData::union_from_types(
            self.db,
            [
                "bigint",
                "boolean",
                "function",
                "number",
                "object",
                "string",
                "symbol",
                "undefined",
            ]
            .into_iter()
            .map(|value| self.typeof_string_literal(value))
            .collect(),
        )
    }

    fn typeof_string_literal(&self, value: &'static str) -> InferredTypeData<'db> {
        // TODO: Replace this with canonical `global_types(db)` literal entries in Phase 6e.
        InferredTypeData::Literal(InferredInternedLiteral::new(
            self.db,
            InferredLiteral::String(Text::new_static(value).into()),
        ))
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
                    | InferredTypeData::Divergent(_)
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

    fn filter_type_to_subset(
        &mut self,
        ty: InferredTypeData<'db>,
        subset: ConditionalSubset,
    ) -> ConditionalFilterResult<'db> {
        let mut types = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_CONDITIONAL_FILTER_STEPS {
            let Some(ty) = pending.pop() else {
                return ConditionalFilterResult::Complete(collected_type_result(self.db, types));
            };
            let ty = self.resolve_inferred_type(ty);
            if !seen.insert(ty) {
                continue;
            }

            match self.filter_action(ty, subset) {
                FilterAction::Mapped(ty) => types.push(ty),
                FilterAction::Retained => match ty {
                    InferredTypeData::InstanceOf(instance) => {
                        let target = self.resolve_inferred_type(instance.ty(self.db));
                        if target.should_flatten_instance(instance.type_parameters(self.db)) {
                            pending.push(target);
                        } else {
                            types.push(ty);
                        }
                    }
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
                    | InferredTypeData::Divergent(_)
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
                    | InferredTypeData::TypeOperator(_)
                    | InferredTypeData::Literal(_)
                    | InferredTypeData::MergedReference(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::NeverKeyword
                    | InferredTypeData::ObjectKeyword
                    | InferredTypeData::ThisKeyword
                    | InferredTypeData::UnknownKeyword
                    | InferredTypeData::VoidKeyword => types.push(ty),
                },
                FilterAction::Stripped => {}
            }
        }

        ConditionalFilterResult::Indeterminate
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
                | InferredTypeData::Divergent(_)
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
                    if ty
                        .conditional_type_shallow(self.db)
                        .is_none_or(|conditional| !conditional.is_truthy())
                    {
                        FilterAction::Retained
                    } else {
                        FilterAction::Stripped
                    }
                }
            },
            ConditionalSubset::Truthy => match ty {
                InferredTypeData::Boolean => FilterAction::Mapped(self.boolean_literal(true)),
                InferredTypeData::Unknown
                | InferredTypeData::Divergent(_)
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
                    if ty
                        .conditional_type_shallow(self.db)
                        .is_none_or(|conditional| !conditional.is_falsy())
                    {
                        FilterAction::Retained
                    } else {
                        FilterAction::Stripped
                    }
                }
            },
            ConditionalSubset::NonNullish => {
                if ty
                    .conditional_type_shallow(self.db)
                    .is_none_or(|conditional| !conditional.is_nullish())
                {
                    FilterAction::Retained
                } else {
                    FilterAction::Stripped
                }
            }
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

fn infer_constructor_type_parameters<'db>(
    db: &'db dyn crate::ModuleDb,
    class: InferredClass<'db>,
    constructor: InferredConstructor<'db>,
    args: &[InferredTypeData<'db>],
) -> Option<Box<[InferredTypeData<'db>]>> {
    let declared_parameters = class.type_parameters(db);
    if declared_parameters.is_empty() {
        return Some(Box::default());
    }

    let mut inferred_parameters = declared_parameters.to_vec();
    for (parameter, arg) in constructor.parameters(db).iter().zip(args) {
        let parameter_ty = parameter.parameter.ty();
        let substitutions = parameter_ty.collect_generic_replacements(db, *arg)?;
        for substitution in substitutions {
            for (index, declared_parameter) in declared_parameters.iter().enumerate() {
                if substitution.generic == *declared_parameter
                    || substitution.generic
                        == InferredTypeData::instance_of(db, *declared_parameter, Box::default())
                {
                    inferred_parameters[index] = substitution.replacement;
                }
            }
        }

        let Some(parameter_function) = parameter_ty.callable_function(db) else {
            continue;
        };
        let InferredReturnType::Type(parameter_return_ty) = parameter_function.return_type(db)
        else {
            continue;
        };
        let Some(argument_function) = arg.callable_function(db) else {
            continue;
        };
        let InferredReturnType::Type(argument_return_ty) = argument_function.return_type(db) else {
            continue;
        };

        let substitutions =
            parameter_return_ty.collect_generic_replacements(db, *argument_return_ty)?;
        for substitution in substitutions {
            for (index, declared_parameter) in declared_parameters.iter().enumerate() {
                if substitution.generic == *declared_parameter
                    || substitution.generic
                        == InferredTypeData::instance_of(db, *declared_parameter, Box::default())
                {
                    inferred_parameters[index] = substitution.replacement;
                }
            }
        }
    }

    Some(inferred_parameters.into_boxed_slice())
}

#[derive(Clone, Copy)]
enum RestMemberMode {
    Instance,
    ClassStatic,
}

fn collect_rest_object_members<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    excluded_names: &[Text],
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> Option<Vec<InferredTypeMember<'db>>> {
    let mut members = Vec::new();
    let mut seen_names = FxHashSet::default();
    let mut seen_types = FxHashSet::default();
    let mut pending = Vec::from([ty]);

    while let Some(ty) = pending.pop() {
        let ty = resolve(ty);
        if seen_types.contains(&ty) {
            continue;
        }
        if seen_types.len() == MAX_REST_MEMBER_STEPS {
            return None;
        }
        seen_types.insert(ty);

        match ty {
            InferredTypeData::Class(class) => {
                collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    class.members(db),
                    excluded_names,
                    RestMemberMode::Instance,
                );
                if let Some(extends) = class.extends(db) {
                    pending.push(extends);
                }
            }
            InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
            InferredTypeData::Interface(interface) => {
                collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    interface.members(db),
                    excluded_names,
                    RestMemberMode::Instance,
                );
                pending.extend(interface.extends(db).iter().rev().copied());
            }
            InferredTypeData::Literal(literal) => {
                let InferredLiteral::Object(own_members) = literal.literal(db) else {
                    return None;
                };
                collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    own_members,
                    excluded_names,
                    RestMemberMode::Instance,
                );
            }
            InferredTypeData::Module(module) => collect_rest_members(
                &mut members,
                &mut seen_names,
                module.members(db),
                excluded_names,
                RestMemberMode::Instance,
            ),
            InferredTypeData::Namespace(namespace) => collect_rest_members(
                &mut members,
                &mut seen_names,
                namespace.members(db),
                excluded_names,
                RestMemberMode::Instance,
            ),
            InferredTypeData::Object(object) => {
                collect_rest_members(
                    &mut members,
                    &mut seen_names,
                    object.members(db),
                    excluded_names,
                    RestMemberMode::Instance,
                );
                if let Some(prototype) = object.prototype(db) {
                    pending.push(prototype);
                }
            }
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
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
            | InferredTypeData::VoidKeyword => return None,
        }
    }

    Some(members)
}

fn collect_rest_members<'db>(
    members: &mut Vec<InferredTypeMember<'db>>,
    seen_names: &mut FxHashSet<Text>,
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
        if !seen_names.insert(name) {
            continue;
        }

        members.push(member.clone());
    }
}

fn rest_member_mode_allows(member: &InferredTypeMember<'_>, mode: RestMemberMode) -> bool {
    match mode {
        RestMemberMode::Instance => !member.kind.is_static(),
        RestMemberMode::ClassStatic => member.kind.is_static() && !member.kind.is_constructor(),
    }
}

enum FilterAction<'db> {
    Mapped(InferredTypeData<'db>),
    Retained,
    Stripped,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ModuleGraphGeneration, module_graph::ModuleInfoKind};
    use biome_db::ParsedSource;
    use biome_js_type_info::{
        TypeDb,
        resolved::{
            InferredConstructorParameter, InferredFunctionParameter,
            InferredInternedGenericTypeParameter, InferredNamedFunctionParameter, InferredObject,
            InferredTypeMemberKind,
        },
    };
    use camino::Utf8Path;
    use salsa::Storage;

    #[salsa::db]
    struct TestDb {
        storage: Storage<Self>,
    }

    #[salsa::input]
    struct RestChainSteps {
        steps: usize,
    }

    #[salsa::tracked]
    fn infer_rest_for_chain_steps<'db>(
        db: &'db dyn crate::ModuleDb,
        input: RestChainSteps,
    ) -> InferredTypeData<'db> {
        rest_type_for_chain(db, input.steps(db))
    }

    impl Default for TestDb {
        fn default() -> Self {
            let db = Self {
                storage: Storage::default(),
            };
            ModuleGraphGeneration::new(&db, 0);
            db
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

    #[salsa::db]
    impl crate::ModuleDb for TestDb {
        fn module_graph_generation(&self) -> u64 {
            ModuleGraphGeneration::get(self).value(self)
        }

        fn module_for_path(&self, _path: &Utf8Path) -> Option<crate::ModuleInfo> {
            let _ = self.module_graph_generation();
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
            let _ = self.module_graph_generation();
        }
    }

    fn replacement_chain<'db>(
        db: &'db TestDb,
        steps: usize,
        leaf: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let wrapper = InferredTypeData::Class(InferredClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Box::default(),
            Some(Text::new_static("Wrapper")),
        ));
        (0..steps - 2).fold(leaf, |ty, _| {
            InferredTypeData::instance_of(db, wrapper, Box::new([ty]))
        })
    }

    fn rest_member<'db>(name: &'static str) -> InferredTypeMember<'db> {
        InferredTypeMember {
            kind: InferredTypeMemberKind::Named(Text::new_static(name)),
            ty: InferredTypeData::Number,
        }
    }

    fn rest_class_chain<'db>(db: &'db dyn TypeDb, steps: usize) -> InferredTypeData<'db> {
        assert!(steps > 0);
        let leaf = InferredTypeData::Class(InferredClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Vec::from([rest_member("kept")]).into_boxed_slice(),
            None,
        ));
        (1..steps).fold(leaf, |extends, _| {
            InferredTypeData::Class(InferredClass::new(
                db,
                Box::default(),
                Some(extends),
                Box::default(),
                Box::default(),
                None,
            ))
        })
    }

    fn rest_type_for_chain<'db>(db: &'db dyn TypeDb, steps: usize) -> InferredTypeData<'db> {
        collect_rest_object_members(db, rest_class_chain(db, steps), &[], |ty| ty)
            .map_or(InferredTypeData::Unknown, |members| {
                InferredTypeData::object_from_members(db, members)
            })
    }

    #[test]
    fn constructor_generic_replacement_budget_boundaries() {
        const LIMIT: usize = 64;

        let db = TestDb::default();
        for steps in [LIMIT - 1, LIMIT, LIMIT + 1] {
            let generic = InferredTypeData::Generic(InferredInternedGenericTypeParameter::new(
                &db,
                None,
                None,
                Text::new_static("T"),
            ));
            let class = InferredClass::new(
                &db,
                Vec::from([generic]).into_boxed_slice(),
                None,
                Box::default(),
                Box::default(),
                Some(Text::new_static("Box")),
            );
            let constructor = InferredConstructor::new(
                &db,
                Box::default(),
                Vec::from([InferredConstructorParameter {
                    parameter: InferredFunctionParameter::Named(InferredNamedFunctionParameter {
                        name: Text::new_static("value"),
                        ty: replacement_chain(&db, steps, generic),
                        is_optional: false,
                        is_rest: false,
                    }),
                    accessibility: None,
                }])
                .into_boxed_slice(),
                None,
            );
            let argument = replacement_chain(&db, steps, InferredTypeData::Number);
            let result = infer_constructor_type_parameters(&db, class, constructor, &[argument]);

            if steps <= LIMIT {
                assert_eq!(
                    result.as_deref(),
                    Some([InferredTypeData::Number].as_slice())
                );
            } else {
                assert_eq!(result, None);
            }
        }
    }

    #[test]
    fn object_rest_member_budget_boundaries_and_repeated_result() {
        let db = TestDb::default();

        for steps in [
            MAX_REST_MEMBER_STEPS - 1,
            MAX_REST_MEMBER_STEPS,
            MAX_REST_MEMBER_STEPS + 1,
        ] {
            let result = rest_type_for_chain(&db, steps);
            if steps <= MAX_REST_MEMBER_STEPS {
                let InferredTypeData::Object(object) = result else {
                    panic!("chain with {steps} steps must produce a complete object");
                };
                assert_eq!(object.members(&db).as_ref(), [rest_member("kept")]);
            } else {
                assert_eq!(result, InferredTypeData::Unknown);
            }
        }

        let first = rest_type_for_chain(&db, MAX_REST_MEMBER_STEPS);
        let second = rest_type_for_chain(&db, MAX_REST_MEMBER_STEPS);
        assert_eq!(first, second);
    }

    #[test]
    fn object_rest_unknown_container_discards_partial_members() {
        let db = TestDb::default();

        for prototype in [InferredTypeData::Unknown, InferredTypeData::UnknownKeyword] {
            let object = InferredTypeData::Object(InferredObject::new(
                &db,
                Some(prototype),
                Vec::from([rest_member("partial")]).into_boxed_slice(),
            ));
            assert_eq!(collect_rest_object_members(&db, object, &[], |ty| ty), None);
        }
    }

    #[test]
    fn object_rest_deduplicated_prototype_cycle_is_complete() {
        let db = TestDb::default();
        let first = InferredTypeData::Object(InferredObject::new(
            &db,
            Some(InferredTypeData::String),
            Vec::from([rest_member("first")]).into_boxed_slice(),
        ));
        let second = InferredTypeData::Object(InferredObject::new(
            &db,
            Some(InferredTypeData::Number),
            Vec::from([rest_member("second")]).into_boxed_slice(),
        ));
        let members = collect_rest_object_members(&db, InferredTypeData::Number, &[], |ty| {
            if ty == InferredTypeData::Number {
                first
            } else if ty == InferredTypeData::String {
                second
            } else {
                ty
            }
        })
        .expect("deduplicated cycle must drain");

        assert_eq!(members, [rest_member("first"), rest_member("second")]);
    }

    #[test]
    fn object_rest_unsupported_aggregate_discards_partial_members() {
        let db = TestDb::default();
        let object = InferredTypeData::Object(InferredObject::new(
            &db,
            None,
            Vec::from([rest_member("partial")]).into_boxed_slice(),
        ));
        let intersection = InferredTypeData::intersection_from_types(
            &db,
            Vec::from([object, InferredTypeData::ObjectKeyword]),
        );

        assert_eq!(
            collect_rest_object_members(&db, intersection, &[], |ty| ty),
            None
        );
    }

    #[test]
    fn object_rest_budget_unknown_invalidates_to_complete_object() {
        let mut db = TestDb::default();
        let input = RestChainSteps::new(&db, MAX_REST_MEMBER_STEPS + 1);
        assert_eq!(
            infer_rest_for_chain_steps(&db, input),
            InferredTypeData::Unknown
        );

        salsa::Setter::to(input.set_steps(&mut db), MAX_REST_MEMBER_STEPS);
        let InferredTypeData::Object(object) = infer_rest_for_chain_steps(&db, input) else {
            panic!("under-budget chain must invalidate to a complete object");
        };
        assert_eq!(object.members(&db).as_ref(), [rest_member("kept")]);
    }
}
