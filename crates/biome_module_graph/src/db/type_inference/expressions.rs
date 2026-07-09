use super::{
    collected_type_result,
    lookup::{
        StaticMemberMode, apply_substitutions, find_member_in_members_for_mode,
        substitutions_for_instance,
    },
    resolver::ResolutionCtx,
};
use crate::db::queries::infer_call_expression_return_type;
use biome_js_type_info::{
    CallArgumentType as RawCallArgumentType, DestructureField as RawDestructureField,
    TypeofExpression as RawTypeofExpression,
    interned_types::{
        CallArgumentType as InferredCallArgumentType, InternedLiteral as InferredInternedLiteral,
        InternedTuple as InferredTuple, Literal as InferredLiteral,
        TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeofExpression as InferredTypeofExpression,
    },
    literal::NumberLiteral,
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_CONDITIONAL_TYPE_STEPS: usize = 1024;
const MAX_CONDITIONAL_FILTER_STEPS: usize = 1024;
const MAX_REST_MEMBER_STEPS: usize = 1024;

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
                self.resolve_new_expression(callee, expression.arguments.len())
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
                Some(InferredTypeData::instance_of(
                    self.db,
                    parent,
                    Box::default(),
                ))
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
                self.resolve_new_expression(expression.callee, expression.arguments.len())
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
            InferredTypeofExpression::This(expression) => Some(InferredTypeData::instance_of(
                self.db,
                expression.parent,
                Box::default(),
            )),
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
        let args = arguments
            .iter()
            .map(|argument| match argument {
                RawCallArgumentType::Argument(ty) | RawCallArgumentType::Spread(ty) => {
                    self.resolve(ty)
                }
            })
            .collect::<Vec<_>>();

        infer_call_expression_return_type(self.db, callee, &args)
    }

    fn resolve_inferred_call_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        arguments: &[InferredCallArgumentType<'db>],
    ) -> InferredTypeData<'db> {
        let args = arguments
            .iter()
            .map(|argument| match argument {
                InferredCallArgumentType::Argument(ty) | InferredCallArgumentType::Spread(ty) => {
                    self.resolve_inferred_type(*ty)
                }
            })
            .collect::<Vec<_>>();

        infer_call_expression_return_type(self.db, callee, &args)
    }

    fn resolve_new_expression(
        &mut self,
        callee: InferredTypeData<'db>,
        argument_count: usize,
    ) -> Option<InferredTypeData<'db>> {
        let callee = self.resolve_inferred_type(callee);
        let InferredTypeData::Class(class) = callee else {
            return None;
        };

        let constructed_ty = class
            .members(self.db)
            .iter()
            .filter(|member| member.kind.is_constructor())
            .find_map(|member| match self.resolve_inferred_type(member.ty) {
                InferredTypeData::Constructor(constructor)
                    if constructor.accepts_argument_count(self.db, argument_count) =>
                {
                    constructor.return_type(self.db)
                }
                _ => None,
            })
            .unwrap_or(callee);

        Some(InferredTypeData::instance_of(
            self.db,
            constructed_ty,
            Box::default(),
        ))
    }

    fn resolve_await_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(argument) {
            ty @ (InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Class(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::Object(_)
            | InferredTypeData::String) => Some(ty),
            InferredTypeData::InstanceOf(instance)
                if self
                    .resolve_inferred_type(instance.ty(self.db))
                    .is_promise_class(self.db) =>
            {
                instance
                    .type_parameters(self.db)
                    .first()
                    .map(|ty| self.resolve_inferred_type(*ty))
            }
            _ => None,
        }
    }

    fn resolve_super_expression(&mut self, parent: InferredTypeData<'db>) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(parent) {
            InferredTypeData::Class(class) => class
                .extends(self.db)
                .map(|extends| InferredTypeData::instance_of(self.db, extends, Box::default()))
                .unwrap_or(InferredTypeData::Unknown),
            _ => InferredTypeData::Unknown,
        }
    }

    pub(in crate::db::type_inference) fn resolve_static_member_expression(
        &mut self,
        object: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        match self.resolve_inferred_type(object) {
            InferredTypeData::Class(class) => find_member_in_members_for_mode(
                self.db,
                class.members(self.db),
                member_name,
                StaticMemberMode::Class,
            )
            .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
            InferredTypeData::InstanceOf(instance) => {
                let target = self.resolve_inferred_type(instance.ty(self.db));
                let substitutions = substitutions_for_instance(
                    self.db,
                    target,
                    instance.type_parameters(self.db),
                    &[],
                );
                self.find_static_member_on_resolved_type(target, member_name)
                    .map(|(ty, is_optional)| {
                        let ty = apply_substitutions(self.db, ty, &substitutions);
                        self.member_type(ty, is_optional)
                    })
            }
            InferredTypeData::Union(union) => {
                let mut types = Vec::new();
                for ty in union.types(self.db) {
                    match self.resolve_inferred_type(*ty) {
                        InferredTypeData::Undefined => {}
                        InferredTypeData::Unknown => types.push(InferredTypeData::Unknown),
                        ty => {
                            if let Some((member_ty, is_optional)) =
                                self.find_static_member_on_resolved_type(ty, member_name)
                            {
                                types.push(self.member_type(member_ty, is_optional));
                            }
                        }
                    }
                }
                collected_type_result(self.db, types).or(Some(InferredTypeData::Unknown))
            }
            ty => self
                .find_static_member_on_resolved_type(ty, member_name)
                .map(|(ty, is_optional)| self.member_type(ty, is_optional)),
        }
    }

    fn find_static_member_on_resolved_type(
        &mut self,
        ty: InferredTypeData<'db>,
        member_name: &str,
    ) -> Option<(InferredTypeData<'db>, bool)> {
        match ty {
            InferredTypeData::Class(class) => find_member_in_members_for_mode(
                self.db,
                class.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Interface(interface) => find_member_in_members_for_mode(
                self.db,
                interface.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::Object(members) => find_member_in_members_for_mode(
                    self.db,
                    members,
                    member_name,
                    StaticMemberMode::Instance,
                ),
                _ => None,
            },
            InferredTypeData::Module(module) => find_member_in_members_for_mode(
                self.db,
                module.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Namespace(namespace) => find_member_in_members_for_mode(
                self.db,
                namespace.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            InferredTypeData::Object(object) => find_member_in_members_for_mode(
                self.db,
                object.members(self.db),
                member_name,
                StaticMemberMode::Instance,
            ),
            _ => None,
        }
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
            _ => None,
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
            _ => None,
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
            subject => self.rest_object_from_type(subject, excluded_names),
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
                _ => {}
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
            _ => None,
        }
    }

    fn resolve_number_or_bigint_unary_expression(
        &mut self,
        argument: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        match self.resolve_inferred_type(argument) {
            InferredTypeData::BigInt => InferredTypeData::BigInt,
            _ => InferredTypeData::Number,
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
            _ => self.typeof_return_union(),
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

            if let Some(next) = self.conditional_type_shallow(ty) {
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
                    _ => return ConditionalType::Unknown,
                }
            }

            if conditional != ConditionalType::Unknown && !conditional.is_mergeable() {
                return conditional;
            }
        }

        ConditionalType::Unknown
    }

    fn conditional_type_shallow(&self, ty: InferredTypeData<'db>) -> Option<ConditionalType> {
        match ty {
            InferredTypeData::AnyKeyword
            | InferredTypeData::Conditional
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::Unknown
            | InferredTypeData::UnknownKeyword => Some(ConditionalType::Anything),
            InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Interface(_)
            | InferredTypeData::Number
            | InferredTypeData::String => Some(ConditionalType::NonNullish),
            InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Global
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::Symbol
            | InferredTypeData::Tuple(_) => Some(ConditionalType::Truthy),
            InferredTypeData::Literal(literal) => Some(match literal.literal(self.db) {
                InferredLiteral::BigInt(text) => match text.text() {
                    "0n" | "-0n" => ConditionalType::FalsyButNotNullish,
                    _ => ConditionalType::Truthy,
                },
                InferredLiteral::Boolean(boolean) => {
                    if boolean.as_bool() {
                        ConditionalType::Truthy
                    } else {
                        ConditionalType::FalsyButNotNullish
                    }
                }
                InferredLiteral::Number(number) => match number.to_f64() {
                    Some(number) if number == 0. || number.is_nan() => {
                        ConditionalType::FalsyButNotNullish
                    }
                    Some(_) => ConditionalType::Truthy,
                    None => ConditionalType::Anything,
                },
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => ConditionalType::Truthy,
                InferredLiteral::String(string) => {
                    if string.as_str().is_empty() {
                        ConditionalType::FalsyButNotNullish
                    } else {
                        ConditionalType::Truthy
                    }
                }
                InferredLiteral::Template(_) => ConditionalType::Anything,
            }),
            InferredTypeData::Null
            | InferredTypeData::Undefined
            | InferredTypeData::VoidKeyword => Some(ConditionalType::Nullish),
            InferredTypeData::Divergent(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_) => None,
            InferredTypeData::InstanceOf(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::Union(_) => None,
        }
    }

    fn filter_type_to_subset(
        &mut self,
        ty: InferredTypeData<'db>,
        subset: ConditionalSubset,
    ) -> Option<InferredTypeData<'db>> {
        let mut types = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([ty]);

        for _ in 0..MAX_CONDITIONAL_FILTER_STEPS {
            let Some(ty) = pending.pop() else {
                return collected_type_result(self.db, types);
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
                    _ => types.push(ty),
                },
                FilterAction::Stripped => {}
            }
        }

        None
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
                _ => {
                    if self
                        .conditional_type_shallow(ty)
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
                _ => {
                    if self
                        .conditional_type_shallow(ty)
                        .is_none_or(|conditional| !conditional.is_falsy())
                    {
                        FilterAction::Retained
                    } else {
                        FilterAction::Stripped
                    }
                }
            },
            ConditionalSubset::NonNullish => {
                if self
                    .conditional_type_shallow(ty)
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

#[derive(Clone, Copy, Eq, PartialEq)]
enum ConditionalType {
    Anything,
    Falsy,
    FalsyButNotNullish,
    NonNullish,
    Nullish,
    Truthy,
    Unknown,
}

impl ConditionalType {
    fn is_falsy(self) -> bool {
        matches!(self, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
    }

    fn is_inferred(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    fn is_non_nullish(self) -> bool {
        matches!(
            self,
            Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy
        )
    }

    fn is_nullish(self) -> bool {
        matches!(self, Self::Nullish)
    }

    fn is_truthy(self) -> bool {
        matches!(self, Self::Truthy)
    }

    fn is_mergeable(self) -> bool {
        !matches!(self, Self::Anything | Self::Unknown)
    }

    fn merged_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Anything, _)
            | (_, Self::Anything)
            | (Self::Falsy | Self::Nullish, Self::NonNullish)
            | (Self::Falsy | Self::FalsyButNotNullish | Self::Nullish, Self::Truthy)
            | (Self::NonNullish, Self::Falsy | Self::Nullish)
            | (Self::Truthy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish) => {
                Self::Anything
            }
            (Self::Falsy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
            | (Self::FalsyButNotNullish | Self::Nullish, Self::Falsy)
            | (Self::FalsyButNotNullish, Self::Nullish)
            | (Self::Nullish, Self::FalsyButNotNullish) => Self::Falsy,
            (Self::FalsyButNotNullish, Self::FalsyButNotNullish) => Self::FalsyButNotNullish,
            (Self::NonNullish, Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy)
            | (Self::FalsyButNotNullish | Self::Truthy, Self::NonNullish) => Self::NonNullish,
            (Self::Nullish, Self::Nullish) => Self::Nullish,
            (Self::Truthy, Self::Truthy) => Self::Truthy,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy)]
enum ConditionalSubset {
    Falsy,
    Truthy,
    NonNullish,
}

enum FilterAction<'db> {
    Mapped(InferredTypeData<'db>),
    Retained,
    Stripped,
}
