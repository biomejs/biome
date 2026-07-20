use crate::DestructureField;
use crate::format_type_info::FormatTypeOptions;
use crate::interned_types::{
    CallArgumentType, ConstructorParameter, FunctionParameter, FunctionParameterBinding,
    InternedClass, InternedConstructor, InternedFunction, InternedGenericTypeParameter,
    InternedInterface, InternedLiteral, InternedMergedReference, InternedModule, InternedNamespace,
    InternedObject, InternedTuple, InternedTypeInstance, InternedTypeofExpression, Literal,
    NamedFunctionParameter, PatternFunctionParameter, ReturnType, TupleElementType, TypeData,
    TypeDb, TypeMember, TypeMemberKind, TypeofExpression,
};
use biome_formatter::prelude::*;
use biome_formatter::{FormatContext, TransformSourceMap, format_args, write};
use biome_rowan::Text;
use std::fmt::Display;

pub struct FormatInferredTypeContext<'db> {
    db: &'db dyn TypeDb,
}

impl<'db> FormatInferredTypeContext<'db> {
    pub fn new(db: &'db dyn TypeDb) -> Self {
        Self { db }
    }

    fn db(&self) -> &'db dyn TypeDb {
        self.db
    }
}

impl<'db> FormatContext for FormatInferredTypeContext<'db> {
    type Options = FormatTypeOptions;

    fn options(&self) -> &Self::Options {
        &FormatTypeOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

pub struct InferredTypeDisplay<'db> {
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
}

impl<'db> InferredTypeDisplay<'db> {
    pub fn new(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Self {
        Self { db, ty }
    }
}

impl Display for InferredTypeDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted =
            biome_formatter::format!(FormatInferredTypeContext::new(self.db), [self.ty])
                .map_err(|_| std::fmt::Error)?;
        let printed = formatted.print().map_err(|_| std::fmt::Error)?;
        f.write_str(printed.as_code())
    }
}

/// Formats an inferred type, returning `unknown` if the formatter cannot build
/// or print its internal document.
pub fn format_inferred_type<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> String {
    let Ok(formatted) = biome_formatter::format!(FormatInferredTypeContext::new(db), [ty]) else {
        return "unknown".to_string();
    };
    formatted.print().map_or_else(
        |_| "unknown".to_string(),
        |printed| printed.as_code().to_string(),
    )
}

impl<'db> Format<FormatInferredTypeContext<'db>> for TypeData<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        match *self {
            Self::Unknown => write!(f, [token("unknown")]),
            Self::Divergent(_) => write!(f, [token("divergent")]),
            Self::Global => write!(f, [token("globalThis")]),
            Self::BigInt => write!(f, [token("BigInt")]),
            Self::Boolean => write!(f, [token("boolean")]),
            Self::Null => write!(f, [token("null")]),
            Self::Number => write!(f, [token("number")]),
            Self::String => write!(f, [token("string")]),
            Self::Symbol => write!(f, [token("symbol")]),
            Self::Undefined => write!(f, [token("undefined")]),
            Self::Conditional => write!(f, [token("conditional")]),
            Self::Class(class) => write!(f, [class]),
            Self::Constructor(constructor) => write!(f, [constructor]),
            Self::Function(function) => write!(f, [function]),
            Self::Interface(interface) => write!(f, [interface]),
            Self::Module(module) => write!(f, [module]),
            Self::Namespace(namespace) => write!(f, [namespace]),
            Self::Object(object) => write!(f, [object]),
            Self::Tuple(tuple) => write!(f, [tuple]),
            Self::Generic(generic) => write!(f, [generic]),
            Self::Local(local) => {
                let module = local.module(db);
                let type_id = local.type_id(db);
                if let Some(name) = db.local_type_name(module, type_id) {
                    write!(f, [text(name.text(), None)])
                } else {
                    write!(
                        f,
                        [&format_args![
                            token("local"),
                            space(),
                            token("type"),
                            space(),
                            text(&type_id.index().to_string(), None)
                        ]]
                    )
                }
            }
            Self::GlobalType(id) => {
                if let Some(name) = crate::globals_ids::global_type_name(id.as_type_id()) {
                    write!(f, [text(name, None)])
                } else {
                    write!(
                        f,
                        [&format_args![
                            token("global"),
                            space(),
                            token("type"),
                            space(),
                            text(&id.index().to_string(), None)
                        ]]
                    )
                }
            }
            Self::Intersection(intersection) => write!(
                f,
                [FmtInferredTypeList {
                    types: intersection.types(db),
                    separator: "&",
                }]
            ),
            Self::Union(union) => write!(
                f,
                [FmtInferredTypeList {
                    types: union.types(db),
                    separator: "|",
                }]
            ),
            Self::TypeOperator(operator) => write!(
                f,
                [&format_args![
                    text(&std::format!("{:?}", operator.operator(db)), None),
                    space(),
                    operator.ty(db)
                ]]
            ),
            Self::Literal(literal) => write!(f, [literal]),
            Self::InstanceOf(instance) => {
                write!(f, [&format_args![token("instanceof"), space(), instance]])
            }
            Self::MergedReference(reference) => write!(f, [reference]),
            Self::TypeofExpression(expression) => write!(f, [expression]),
            Self::TypeofType(ty) => {
                write!(f, [&format_args![token("typeof"), space(), ty.ty(db)]])
            }
            Self::TypeofValue(ty) => write!(
                f,
                [&format_args![token("typeof"), space(), ty.identifier(db)]]
            ),
            Self::AnyKeyword => write!(f, [token("any")]),
            Self::NeverKeyword => write!(f, [token("never")]),
            Self::ObjectKeyword => write!(f, [token("object")]),
            Self::ThisKeyword => write!(f, [token("this")]),
            Self::UnknownKeyword => write!(f, [token("unknown")]),
            Self::VoidKeyword => write!(f, [token("void")]),
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedObject<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        let prototype = format_with(|f| {
            if let Some(prototype) = self.prototype(db) {
                write!(f, [prototype])
            } else {
                write!(f, [token("No prototype")])
            }
        });
        write!(
            f,
            [&format_args![
                token("Object"),
                space(),
                token("{"),
                &group(&block_indent(&format_args![
                    token("prototype:"),
                    space(),
                    prototype,
                    hard_line_break(),
                    token("members:"),
                    space(),
                    FmtInferredTypeMembers(self.members(db)),
                ])),
                token("}")
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedFunction<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        let is_async = format_with(|f| {
            if self.is_async(db) {
                write!(f, [token("async")])
            } else {
                write!(f, [token("sync")])
            }
        });
        let name = format_with(|f| {
            if let Some(name) = self.name(db).as_ref() {
                write!(f, [text(&std::format!("\"{name}\""), None)])
            } else {
                Ok(())
            }
        });

        write!(
            f,
            [&format_args![
                is_async,
                space(),
                token("Function"),
                space(),
                name,
                space(),
                token("{"),
                &group(&soft_block_indent(&format_args![
                    token("accepts:"),
                    space(),
                    token("{"),
                    &group(&block_indent(&format_args![
                        token("params:"),
                        space(),
                        FmtInferredFunctionParameters(self.parameters(db)),
                        hard_line_break(),
                        token("type_args:"),
                        space(),
                        FmtInferredTypes(self.type_parameters(db)),
                    ])),
                    token("}"),
                    hard_line_break(),
                    token("returns:"),
                    space(),
                    self.return_type(db),
                    space(),
                ])),
                token("}"),
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for ReturnType<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self {
            Self::Type(ty) => write!(f, [ty]),
            Self::Predicate(predicate) => write!(
                f,
                [&format_args![
                    token("predicate"),
                    space(),
                    &predicate.parameter_name,
                    token(":"),
                    space(),
                    &predicate.ty
                ]]
            ),
            Self::Asserts(asserts) => write!(
                f,
                [&format_args![
                    token("asserts"),
                    space(),
                    &asserts.parameter_name,
                    token(":"),
                    space(),
                    &asserts.ty
                ]]
            ),
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for FunctionParameter<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self {
            Self::Named(named) => write!(f, [named]),
            Self::Pattern(pattern) => write!(f, [pattern]),
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for NamedFunctionParameter<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let optional = format_with(|f| {
            if self.is_optional {
                write!(f, [token("optional")])
            } else {
                write!(f, [token("required")])
            }
        });
        let rest = format_with(|f| {
            if self.is_rest {
                write!(f, [token("...")])
            } else {
                Ok(())
            }
        });
        write!(
            f,
            [&group(&block_indent(&format_args![
                optional,
                space(),
                rest,
                &self.name,
                token(":"),
                space(),
                &self.ty,
            ]))]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for PatternFunctionParameter<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let bindings = format_with(|f| {
            if !self.bindings.is_empty() {
                write!(
                    f,
                    [&format_args![
                        space(),
                        token("(bindings:"),
                        space(),
                        FmtInferredFunctionParameterBindings(&self.bindings),
                        token(")")
                    ]]
                )
            } else {
                Ok(())
            }
        });
        let rest = format_with(|f| {
            if self.is_rest {
                write!(f, [token("...")])
            } else {
                Ok(())
            }
        });
        write!(
            f,
            [&group(&block_indent(&format_args![
                rest,
                bindings,
                token(":"),
                space(),
                &self.ty
            ]))]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for TypeMember<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                &self.kind,
                token(":"),
                space(),
                &group(&soft_block_indent(&self.ty)),
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for TypeMemberKind<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self {
            Self::CallSignature | Self::ConstAssertedCallSignature => write!(f, [token("()")]),
            Self::Constructor | Self::ConstAssertedConstructor => write!(f, [token("constructor")]),
            Self::Getter(name) | Self::ConstAssertedGetter(name) => {
                write!(f, [text(&std::format!("get \"{name}\""), None)])
            }
            Self::IndexSignature(ty) | Self::ConstAssertedIndexSignature(ty) => {
                write!(f, [token("["), ty, token("]")])
            }
            Self::ComputedValue(ty) | Self::ConstAssertedComputedValue(ty) => {
                write!(f, [token("computed"), space(), token("["), ty, token("]")])
            }
            Self::ComputedValueNamed(name, _) | Self::ConstAssertedComputedValueNamed(name, _) => {
                write!(f, [text(&std::format!("computed [{name}]"), None)])
            }
            Self::Named(name) | Self::ConstAssertedNamed(name) => {
                write!(f, [text(&std::format!("\"{name}\""), None)])
            }
            Self::NamedOptional(name) | Self::ConstAssertedNamedOptional(name) => {
                write!(f, [text(&std::format!("\"{name}\"?"), None)])
            }
            Self::NamedStatic(name) | Self::ConstAssertedNamedStatic(name) => {
                write!(f, [text(&std::format!("static \"{name}\""), None)])
            }
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedClass<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        let name = format_with(|f| {
            if let Some(name) = self.name(db).as_ref() {
                write!(f, [text(&std::format!("\"{name}\""), None)])
            } else {
                Ok(())
            }
        });
        let extends = format_with(|f| {
            if let Some(extends) = self.extends(db) {
                write!(f, [extends])
            } else {
                write!(f, [token("none")])
            }
        });
        write!(
            f,
            [&format_args![
                token("class"),
                space(),
                name,
                space(),
                token("{"),
                &group(&block_indent(&format_args![
                    token("extends:"),
                    space(),
                    extends,
                    hard_line_break(),
                    token("implements:"),
                    space(),
                    FmtInferredTypes(self.implements(db)),
                    hard_line_break(),
                    token("type_args:"),
                    space(),
                    FmtInferredTypes(self.type_parameters(db)),
                ])),
                token("}")
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedInterface<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [&format_args![
                token("interface"),
                space(),
                text(&std::format!("\"{}\"", self.name(db)), None),
                space(),
                token("{"),
                &group(&block_indent(&format_args![
                    token("extends:"),
                    space(),
                    FmtInferredTypes(self.extends(db)),
                    hard_line_break(),
                    token("type_args:"),
                    space(),
                    FmtInferredTypes(self.type_parameters(db)),
                    hard_line_break(),
                    token("members:"),
                    space(),
                    FmtInferredTypeMembers(self.members(db)),
                ])),
                token("}")
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedLiteral<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(f, [self.literal(db)])
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for Literal<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self {
            Self::BigInt(bigint_text) => {
                write!(f, [token("bigint:"), space(), text(bigint_text, None)])
            }
            Self::Boolean(lit) => write!(
                f,
                [
                    token("bool:"),
                    space(),
                    text(lit.as_bool().to_string().as_str(), None)
                ]
            ),
            Self::Number(lit) => write!(f, [token("number:"), space(), text(lit.as_str(), None)]),
            Self::Object(members) => write!(
                f,
                [
                    token("ObjectLiteral"),
                    space(),
                    token("{"),
                    &group(&soft_block_indent(&format_args![
                        token("members:"),
                        space(),
                        FmtInferredTypeMembers(members)
                    ])),
                    token("}")
                ]
            ),
            Self::RegExp(regex) => write!(
                f,
                [
                    token("regex:"),
                    space(),
                    token("/"),
                    text(&regex.pattern, None),
                    token("/"),
                    text(&regex.flags, None)
                ]
            ),
            Self::String(lit) => write!(f, [token("string:"), space(), text(lit.as_str(), None)]),
            Self::Template(tmpl) => write!(f, [token("string:"), space(), text(tmpl, None)]),
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedTypeInstance<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        let type_args = format_with(|f| {
            if self.type_parameters(db).is_empty() {
                Ok(())
            } else {
                write!(f, [token("<")])?;
                for (index, param) in self.type_parameters(db).iter().enumerate() {
                    write!(f, [param])?;
                    if index != self.type_parameters(db).len() - 1 {
                        write!(f, [token(","), space()])?;
                    }
                }
                write!(f, [token(">")])
            }
        });

        write!(
            f,
            [&format_args![
                FmtInferredInstanceTarget(self.ty(db)),
                type_args
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedGenericTypeParameter<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        let constraint = format_with(|f| {
            if let Some(constraint) = self.constraint(db) {
                write!(f, [space(), token("extends"), space(), constraint])
            } else {
                Ok(())
            }
        });
        let default = format_with(|f| {
            if let Some(default) = self.default(db) {
                write!(f, [space(), token("="), space(), default])
            } else {
                Ok(())
            }
        });

        write!(f, [&format_args![self.name(db), constraint, default]])
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedMergedReference<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [&format_args![
                token("("),
                token("type:"),
                space(),
                FmtInferredOptionalType(self.ty(db)),
                token(","),
                space(),
                token("value:"),
                space(),
                FmtInferredOptionalType(self.value_ty(db)),
                token(","),
                space(),
                token("namespace:"),
                space(),
                FmtInferredOptionalType(self.namespace_ty(db)),
                token(")")
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedConstructor<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [&format_args![
                token("constructor"),
                space(),
                token("{"),
                &group(&block_indent(&format_args![
                    token("params:"),
                    space(),
                    FmtInferredConstructorParameters(self.parameters(db)),
                    hard_line_break(),
                    token("type_args:"),
                    space(),
                    FmtInferredTypes(self.type_parameters(db)),
                    hard_line_break(),
                    token("returns:"),
                    space(),
                    FmtInferredOptionalType(self.return_type(db)),
                ])),
                token("}")
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedTuple<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [
                token("["),
                FmtInferredTupleElements(self.elements(db)),
                token("]")
            ]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedModule<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [&format_args![
                token("module"),
                space(),
                text(&std::format!("\"{}\"", self.name(db)), None),
                space(),
                FmtInferredTypeMembers(self.members(db))
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedNamespace<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(
            f,
            [&format_args![
                token("namespace"),
                space(),
                text(&std::format!("{:?}", self.path(db)), None),
                space(),
                FmtInferredTypeMembers(self.members(db))
            ]]
        )
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for InternedTypeofExpression<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        write!(f, [self.expression(db)])
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for TypeofExpression<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self {
            Self::Addition(expr) => write!(
                f,
                [&group(&format_args![
                    &expr.left,
                    soft_line_break_or_space(),
                    token("+"),
                    soft_line_break_or_space(),
                    &expr.right,
                ])]
            ),
            Self::Await(expr) => write!(
                f,
                [&format_args![
                    token("Await"),
                    token("("),
                    group(&soft_block_indent(&expr.argument)),
                    token(")")
                ]]
            ),
            Self::BitwiseNot(expr) => write!(f, [&format_args![token("~"), &expr.argument]]),
            Self::Call(call) => write!(
                f,
                [&format_args![
                    token("Call"),
                    space(),
                    &call.callee,
                    token("("),
                    group(&soft_block_indent(&FmtInferredCallArgumentTypes(
                        &call.arguments
                    ))),
                    token(")")
                ]]
            ),
            Self::Conditional(expr) => write!(
                f,
                [&group(&format_args![
                    &expr.test,
                    soft_line_break_or_space(),
                    token("?"),
                    soft_line_break_or_space(),
                    &expr.consequent,
                    soft_line_break_or_space(),
                    token(":"),
                    soft_line_break_or_space(),
                    &expr.alternate,
                ])]
            ),
            Self::Destructure(expr) => match &expr.destructure_field {
                DestructureField::Index(index) => write!(
                    f,
                    [&format_args![
                        &expr.ty,
                        token("["),
                        text(&index.to_string(), None),
                        token("]")
                    ]]
                ),
                DestructureField::Name(name) => {
                    write!(f, [&format_args![&expr.ty, token("."), name]])
                }
                DestructureField::RestExcept(names) => write!(
                    f,
                    [&format_args![
                        token("{"),
                        FmtNames(names),
                        token("..."),
                        &expr.ty,
                        token("}")
                    ]]
                ),
                DestructureField::RestFrom(index) => write!(
                    f,
                    [&format_args![
                        token("["),
                        text(&std::format!("({index} others)"), None),
                        token("..."),
                        &expr.ty,
                        token("]")
                    ]]
                ),
            },
            Self::Index(expr) => write!(
                f,
                [&format_args![
                    &expr.object,
                    text(&std::format!("[{}]", expr.index), None)
                ]]
            ),
            Self::IterableValueOf(expr) => write!(
                f,
                [&format_args![&group(&format_args![
                    token("iterable_value_of"),
                    soft_line_break_or_space(),
                    &expr.ty
                ])]]
            ),
            Self::LogicalAnd(expr) => write!(
                f,
                [&format_args![&group(&format_args![
                    &expr.left,
                    soft_line_break_or_space(),
                    token("&&"),
                    soft_line_break_or_space(),
                    &expr.right
                ])]]
            ),
            Self::LogicalOr(expr) => write!(
                f,
                [&format_args![&group(&format_args![
                    &expr.left,
                    soft_line_break_or_space(),
                    token("||"),
                    soft_line_break_or_space(),
                    &expr.right
                ])]]
            ),
            Self::New(expr) => write!(
                f,
                [&format_args![
                    token("new"),
                    space(),
                    &expr.callee,
                    token("("),
                    group(&soft_block_indent(&FmtInferredCallArgumentTypes(
                        &expr.arguments
                    ))),
                    token(")")
                ]]
            ),
            Self::NullishCoalescing(expr) => write!(
                f,
                [&format_args![&group(&format_args![
                    &expr.left,
                    soft_line_break_or_space(),
                    token("??"),
                    soft_line_break_or_space(),
                    &expr.right
                ])]]
            ),
            Self::StaticMember(expr) => {
                write!(f, [&format_args![&expr.object, token("."), &expr.member]])
            }
            Self::Super(_) => write!(f, [token("super")]),
            Self::This(_) => write!(f, [token("this")]),
            Self::Typeof(expr) => {
                write!(f, [&format_args![token("typeof"), space(), &expr.argument]])
            }
            Self::UnaryMinus(expr) => write!(f, [&format_args![token("-"), &expr.argument]]),
        }
    }
}

impl<'db> Format<FormatInferredTypeContext<'db>> for Text {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        write!(f, [text(self, None)])
    }
}

struct FmtInferredOptionalType<'db>(Option<TypeData<'db>>);

impl<'db> Format<FormatInferredTypeContext<'db>> for FmtInferredOptionalType<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        match self.0 {
            Some(ty) => write!(f, [ty]),
            None => write!(f, [token("none")]),
        }
    }
}

struct FmtInferredInstanceTarget<'db>(TypeData<'db>);

impl<'db> Format<FormatInferredTypeContext<'db>> for FmtInferredInstanceTarget<'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let db = f.context().db();
        match self.0 {
            TypeData::Class(class) => {
                if let Some(name) = class.name(db).as_ref() {
                    write!(f, [text(name, None)])
                } else {
                    write!(f, [self.0])
                }
            }
            TypeData::Interface(interface) => write!(f, [text(interface.name(db), None)]),
            TypeData::Generic(generic) => write!(f, [generic.name(db)]),
            ty => write!(f, [ty]),
        }
    }
}

struct FmtInferredTypeList<'a, 'db> {
    types: &'a [TypeData<'db>],
    separator: &'static str,
}

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredTypeList<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let needs_break = inferred_type_list_needs_break(f.context().db(), self.types);
        for (index, ty) in self.types.iter().enumerate() {
            if needs_break && index != 0 {
                write!(f, [hard_line_break(), token(self.separator), space()])?;
            }
            write!(f, [ty])?;
            if !needs_break && index != self.types.len() - 1 {
                write!(f, [space(), token(self.separator), space()])?;
            }
        }
        Ok(())
    }
}

fn inferred_type_list_needs_break<'db>(db: &'db dyn TypeDb, types: &[TypeData<'db>]) -> bool {
    types.iter().any(|ty| {
        matches!(
            ty,
            TypeData::Class(_)
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Interface(_)
                | TypeData::MergedReference(_)
                | TypeData::Module(_)
                | TypeData::Namespace(_)
                | TypeData::Object(_)
        )
        || matches!(ty, TypeData::Literal(literal) if matches!(literal.literal(db), Literal::Object(_)))
    })
}

struct FmtInferredTypes<'a, 'db>(&'a [TypeData<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredTypes<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        write!(f, [token("[")])?;
        let types = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![token(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for ty in self.0 {
                joiner.entry(&format_args![ty]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![group(&soft_block_indent(&types)), token("]")]]
        )
    }
}

struct FmtInferredTypeMembers<'a, 'db>(&'a [TypeMember<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredTypeMembers<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        write!(f, [token("[")])?;
        let members = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![token(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for member in self.0 {
                joiner.entry(&format_args![member]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![
                group(&soft_block_indent(&members)),
                token("]")
            ]]
        )
    }
}

struct FmtInferredFunctionParameters<'a, 'db>(&'a [FunctionParameter<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredFunctionParameters<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [token("[]")]);
        }

        let parameters = format_with(|f| {
            let separator = format_with(|f| write!(f, [&format_args![soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for parameter in self.0 {
                joiner.entry(&format_args![parameter]);
            }
            joiner.finish()
        });
        write!(f, [&format_args![token("["), parameters, token("]")]])
    }
}

struct FmtInferredConstructorParameters<'a, 'db>(&'a [ConstructorParameter<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredConstructorParameters<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [token("[]")]);
        }

        let parameters = format_with(|f| {
            let separator = format_with(|f| write!(f, [&format_args![soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for parameter in self.0 {
                joiner.entry(&format_args![&parameter.parameter]);
            }
            joiner.finish()
        });
        write!(f, [&format_args![token("["), parameters, token("]")]])
    }
}

struct FmtInferredFunctionParameterBindings<'a, 'db>(&'a [FunctionParameterBinding<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>>
    for FmtInferredFunctionParameterBindings<'a, 'db>
{
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }

        let bindings = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![token(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for binding in self.0 {
                joiner.entry(&format_args![
                    &binding.name,
                    token(":"),
                    space(),
                    &binding.ty
                ]);
            }
            joiner.finish()
        });
        write!(f, [bindings])
    }
}

struct FmtInferredTupleElements<'a, 'db>(&'a [TupleElementType<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredTupleElements<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let elements = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![token(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for element in self.0 {
                joiner.entry(&format_args![FmtInferredTupleElement(element)]);
            }
            joiner.finish()
        });
        write!(f, [elements])
    }
}

struct FmtInferredTupleElement<'a, 'db>(&'a TupleElementType<'db>);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredTupleElement<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        let rest = format_with(|f| {
            if self.0.is_rest {
                write!(f, [token("...")])
            } else {
                Ok(())
            }
        });
        let name = format_with(|f| {
            if let Some(name) = &self.0.name {
                write!(f, [name, token(":"), space()])
            } else {
                Ok(())
            }
        });
        let optional = format_with(|f| {
            if self.0.is_optional {
                write!(f, [token("?")])
            } else {
                Ok(())
            }
        });
        write!(f, [&format_args![rest, name, &self.0.ty, optional]])
    }
}

struct FmtInferredCallArgumentTypes<'a, 'db>(&'a [CallArgumentType<'db>]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtInferredCallArgumentTypes<'a, 'db> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [token("No parameters")]);
        }

        let arguments = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for argument in self.0 {
                match argument {
                    CallArgumentType::Argument(ty) => joiner.entry(&format_args![ty]),
                    CallArgumentType::Spread(ty) => joiner.entry(&format_args![token("..."), ty]),
                };
            }
            joiner.finish()
        });
        write!(f, [arguments])
    }
}

struct FmtNames<'a>(&'a [Text]);

impl<'a, 'db> Format<FormatInferredTypeContext<'db>> for FmtNames<'a> {
    fn fmt(&self, f: &mut Formatter<FormatInferredTypeContext<'db>>) -> FormatResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }

        let names = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for name in self.0 {
                joiner.entry(&format_args![name]);
            }
            joiner.finish()
        });
        write!(f, [&format_args![&names]])
    }
}
