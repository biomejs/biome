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
        InternedLiteral as InferredInternedLiteral, Literal as InferredLiteral,
        TypeData as InferredTypeData,
    },
};
use biome_rowan::Text;

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
            RawTypeofExpression::Destructure(expression) => match &expression.destructure_field {
                RawDestructureField::Index(index) => {
                    let subject = self.resolve(&expression.ty);
                    self.resolve_element_type_at_index(subject, *index)
                }
                RawDestructureField::Name(_)
                | RawDestructureField::RestExcept(_)
                | RawDestructureField::RestFrom(_) => None,
            },
            RawTypeofExpression::Index(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_element_type_at_index(object, expression.index)
            }
            RawTypeofExpression::IterableValueOf(expression) => {
                let ty = self.resolve(&expression.ty);
                self.resolve_iterable_value_type(ty)
            }
            RawTypeofExpression::New(expression) => {
                let callee = self.resolve(&expression.callee);
                self.resolve_new_expression(callee, expression.arguments.len())
            }
            RawTypeofExpression::StaticMember(expression) => {
                let object = self.resolve(&expression.object);
                self.resolve_static_member_expression(object, expression.member.text())
            }
            RawTypeofExpression::Typeof(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_typeof_operator(argument))
            }
            RawTypeofExpression::UnaryMinus(expression) => {
                let argument = self.resolve(&expression.argument);
                Some(self.resolve_number_or_bigint_unary_expression(argument))
            }
            RawTypeofExpression::Conditional(_)
            | RawTypeofExpression::LogicalAnd(_)
            | RawTypeofExpression::LogicalOr(_)
            | RawTypeofExpression::NullishCoalescing(_)
            | RawTypeofExpression::Super(_)
            | RawTypeofExpression::This(_) => None,
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
}
