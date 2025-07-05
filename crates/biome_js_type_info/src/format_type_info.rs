use crate::globals::global_type_name;
use crate::{
    CallArgumentType, Class, DestructureField, Function, FunctionParameter,
    FunctionParameterBinding, GenericTypeParameter, ImportSymbol, Interface, Literal,
    MergedReference, NUM_PREDEFINED_TYPES, Object, ObjectLiteral, ReturnType, Type, TypeData,
    TypeId, TypeImportQualifier, TypeInstance, TypeMember, TypeMemberKind, TypeReference,
    TypeReferenceQualifier, TypeResolverLevel, TypeofAwaitExpression, TypeofExpression, Union,
};
use biome_formatter::prelude::*;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TransformSourceMap,
};
use biome_formatter::{format_args, write};
use biome_js_syntax::TextSize;
use biome_resolver::ResolvedPath;
use biome_rowan::Text;
use std::fmt::Debug;
use std::ops::Deref;

pub struct FormatTypeOptions;

impl FormatOptions for FormatTypeOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::Space
    }

    fn indent_width(&self) -> IndentWidth {
        IndentWidth::try_from(2).unwrap()
    }

    fn line_width(&self) -> LineWidth {
        LineWidth::default()
    }

    fn line_ending(&self) -> LineEnding {
        LineEnding::Lf
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            indent_width: self.indent_width(),
            print_width: self.line_width().into(),
            line_ending: self.line_ending(),
            indent_style: self.indent_style(),
        }
    }
}

pub struct FormatTypeContext;

impl FormatContext for FormatTypeContext {
    type Options = FormatTypeOptions;

    fn options(&self) -> &Self::Options {
        &FormatTypeOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.deref(), f)
    }
}

impl std::fmt::Display for TypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatTypeContext, [&self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<FormatTypeContext> for TypeData {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            Self::Unknown => write!(f, [text("unknown")]),
            Self::Global => write!(f, [text("globalThis")]),
            Self::BigInt => write!(f, [text("BigInt")]),
            Self::Boolean => write!(f, [text("boolean")]),
            Self::Null => write!(f, [text("null")]),
            Self::Number => write!(f, [text("number")]),
            Self::String => write!(f, [text("string")]),
            Self::Symbol => write!(f, [text("symbol")]),
            Self::Undefined => write!(f, [text("undefined")]),
            Self::Conditional => write!(f, [text("conditional")]),
            Self::ImportNamespace(module_id) => write!(
                f,
                [dynamic_text(
                    &std::format!("namespace for {module_id:?}"),
                    TextSize::default()
                )]
            ),
            Self::Class(class) => write!(f, [&class.as_ref()]),
            Self::Constructor(ty) => write!(f, [FmtVerbatim(ty.as_ref())]),
            Self::Function(function) => write!(f, [&function.as_ref()]),
            Self::Interface(interface) => write!(f, [&interface.as_ref()]),
            Self::Module(ty) => write!(f, [FmtVerbatim(ty.as_ref())]),
            Self::Namespace(ty) => write!(f, [FmtVerbatim(ty.as_ref())]),
            Self::Object(object) => write!(f, [object.as_ref()]),
            Self::Tuple(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            Self::Generic(generic) => write!(f, [&generic.as_ref()]),
            Self::Intersection(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            Self::Union(union) => write!(f, [&union.as_ref()]),
            Self::TypeOperator(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            Self::Literal(ty) => write!(f, [&ty.as_ref()]),
            Self::InstanceOf(ty) => write!(
                f,
                [&format_args![text("instanceof"), space(), &ty.as_ref()]]
            ),
            Self::Reference(reference) => write!(f, [reference]),
            Self::MergedReference(reference) => write!(f, [reference.as_ref()]),
            Self::TypeofExpression(expression) => write!(f, [&expression.as_ref()]),
            Self::TypeofType(reference) => {
                write!(
                    f,
                    [&format_args![text("typeof"), space(), reference.as_ref()]]
                )
            }
            Self::TypeofValue(ty) => {
                write!(f, [&format_args![text("typeof"), space(), &ty.identifier]])
            }
            Self::AnyKeyword => write!(f, [text("any")]),
            Self::NeverKeyword => write!(f, [text("never")]),
            Self::ObjectKeyword => write!(f, [text("object")]),
            Self::ThisKeyword => write!(f, [text("this")]),
            Self::UnknownKeyword => write!(f, [text("unknown")]),
            Self::VoidKeyword => write!(f, [text("void")]),
        }
    }
}

impl Format<FormatTypeContext> for Object {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let prototype = format_with(|f| {
            if let Some(prototype) = self.prototype.as_ref() {
                write!(f, [prototype])
            } else {
                write!(f, [text("No prototype")])
            }
        });
        write!(
            f,
            [&format_args![
                text("Object"),
                space(),
                text("{"),
                &group(&block_indent(&format_args![
                    text("prototype:"),
                    space(),
                    prototype,
                    hard_line_break(),
                    text("members:"),
                    space(),
                    FmtTypeMembers(self.members.as_ref()),
                ])),
                text("}")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Function {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let is_async = format_with(|f| {
            if self.is_async {
                write!(f, [&format_args![text("async")]])
            } else {
                write!(f, [&format_args![text("sync")]])
            }
        });

        let name = format_with(|f| {
            if let Some(name) = &self.name {
                write!(
                    f,
                    [dynamic_text(
                        &std::format!("\"{name}\""),
                        TextSize::default()
                    )]
                )
            } else {
                Ok(())
            }
        });

        write!(
            f,
            [&format_args![
                is_async,
                space(),
                text("Function"),
                space(),
                name,
                space(),
                text("{"),
                &group(&soft_block_indent(&format_args![
                    text("accepts:"),
                    space(),
                    text("{"),
                    &group(&block_indent(&format_args![
                        text("params:"),
                        space(),
                        FmtFunctionParameters(&self.parameters),
                        hard_line_break(),
                        text("type_args:"),
                        space(),
                        FmtTypeReferences(&self.type_parameters),
                    ])),
                    text("}"),
                    hard_line_break(),
                    text("returns:"),
                    space(),
                    &self.return_type,
                    space(),
                ])),
                text("}"),
            ]]
        )
    }
}

impl Format<FormatTypeContext> for ReturnType {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            Self::Type(ty) => {
                write!(f, [&ty])
            }
            Self::Predicate(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::Asserts(ty) => write!(f, [FmtVerbatim(&ty)]),
        }
    }
}

impl Format<FormatTypeContext> for FunctionParameter {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let bindings = format_with(|f| {
            if !self.bindings.is_empty() {
                write!(
                    f,
                    [&format_args![
                        space(),
                        text("(bindings:"),
                        space(),
                        FmtFunctionParameterBindings(&self.bindings),
                        text(")")
                    ]]
                )
            } else {
                Ok(())
            }
        });
        if self.is_rest {
            write!(
                f,
                [&group(&format_args![
                    text("..."),
                    self.name.as_ref().unwrap_or(&Text::Static("(unnamed)")),
                    text(":"),
                    space(),
                    &self.ty,
                    bindings
                ])]
            )
        } else {
            let optional = format_with(|f| {
                if self.is_optional {
                    write!(f, [&format_args![text("optional")]])
                } else {
                    write!(f, [&format_args![text("required")]])
                }
            });
            write!(
                f,
                [&group(&block_indent(&format_args![
                    optional,
                    space(),
                    self.name.as_ref().unwrap_or(&Text::Static("(unnamed)")),
                    text(":"),
                    space(),
                    &self.ty,
                    bindings
                ]))]
            )
        }
    }
}

impl Format<FormatTypeContext> for TypeMember {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                &self.kind,
                text(":"),
                space(),
                &group(&soft_block_indent(&self.ty)),
            ]]
        )
    }
}

impl Format<FormatTypeContext> for TypeMemberKind {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            Self::CallSignature => write!(f, [text("()")]),
            Self::Constructor => write!(f, [text("constructor")]),
            Self::Getter(name) => {
                let quoted = std::format!("get \"{name}\"");
                write!(f, [dynamic_text(&quoted, TextSize::default())])
            }
            Self::IndexSignature(ty) => {
                write!(f, [text("["), ty, text("]")])
            }
            Self::Named(name) => {
                let quoted = std::format!("\"{name}\"");
                write!(f, [dynamic_text(&quoted, TextSize::default())])
            }
            Self::NamedStatic(name) => {
                let quoted = std::format!("static \"{name}\"");
                write!(f, [dynamic_text(&quoted, TextSize::default())])
            }
        }
    }
}

impl Format<FormatTypeContext> for TypeofAwaitExpression {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![group(&soft_block_indent(&self.argument))]]
        )
    }
}

impl Format<FormatTypeContext> for TypeofExpression {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        match self {
            Self::Addition(addition) => {
                write!(
                    f,
                    [&group(&format_args![
                        &addition.left,
                        soft_line_break_or_space(),
                        text("+"),
                        soft_line_break_or_space(),
                        &addition.right,
                    ])]
                )
            }
            Self::Await(await_expression) => {
                write!(
                    f,
                    [&format_args![
                        text("Await"),
                        text("("),
                        group(&soft_block_indent(await_expression)),
                        text(")")
                    ]]
                )
            }
            Self::BitwiseNot(expr) => {
                write!(f, [&format_args![text("~"), &expr.argument]])
            }
            Self::Call(call) => {
                write!(
                    f,
                    [&format_args![
                        text("Call"),
                        space(),
                        call.callee,
                        text("("),
                        group(&soft_block_indent(&FmtCallArgumentType(&call.arguments))),
                        text(")")
                    ]]
                )
            }
            Self::Conditional(conditional) => {
                write!(
                    f,
                    [&group(&format_args![
                        &conditional.test,
                        soft_line_break_or_space(),
                        text("?"),
                        soft_line_break_or_space(),
                        &conditional.consequent,
                        soft_line_break_or_space(),
                        text(":"),
                        soft_line_break_or_space(),
                        &conditional.alternate
                    ])]
                )
            }
            Self::Destructure(destructure) => match &destructure.destructure_field {
                DestructureField::Index(index) => {
                    write!(
                        f,
                        [&format_args![
                            destructure.ty,
                            text("["),
                            dynamic_text(&index.to_string(), TextSize::default()),
                            text("]")
                        ]]
                    )
                }
                DestructureField::Name(name) => {
                    write!(f, [&format_args![destructure.ty, text("."), name]])
                }
                DestructureField::RestExcept(names) => {
                    write!(
                        f,
                        [&format_args![
                            text("{"),
                            FmtNames(names),
                            text("..."),
                            destructure.ty,
                            text("}")
                        ]]
                    )
                }
                DestructureField::RestFrom(index) => {
                    write!(
                        f,
                        [&format_args![
                            text("["),
                            dynamic_text(&std::format!("({index} others)"), TextSize::default()),
                            text("..."),
                            destructure.ty,
                            text("]")
                        ]]
                    )
                }
            },
            Self::LogicalAnd(expr) => {
                write!(
                    f,
                    [&format_args![&group(&format_args![
                        &expr.left,
                        soft_line_break_or_space(),
                        text("&&"),
                        soft_line_break_or_space(),
                        &expr.right
                    ])]]
                )
            }
            Self::LogicalOr(expr) => {
                write!(
                    f,
                    [&format_args![&group(&format_args![
                        &expr.left,
                        soft_line_break_or_space(),
                        text("||"),
                        soft_line_break_or_space(),
                        &expr.right
                    ])]]
                )
            }
            Self::New(expr) => {
                write!(f, [&format_args![text("new"), space(), &expr.callee]])
            }
            Self::NullishCoalescing(expr) => {
                write!(
                    f,
                    [&format_args![&group(&format_args![
                        &expr.left,
                        soft_line_break_or_space(),
                        text("??"),
                        soft_line_break_or_space(),
                        &expr.right
                    ])]]
                )
            }
            Self::StaticMember(expr) => {
                write!(f, [&format_args![&expr.object, text("."), &expr.member]])
            }
            Self::Super(_) => write!(f, [&format_args![text("super")]]),
            Self::This(_) => write!(f, [&format_args![text("this")]]),
            Self::Typeof(expr) => {
                write!(f, [&format_args![text("typeof"), space(), &expr.argument]])
            }
            Self::UnaryMinus(expr) => {
                write!(f, [&format_args![text("-"), &expr.argument]])
            }
        }
    }
}

impl Format<FormatTypeContext> for GenericTypeParameter {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let constraint = format_with(|f| {
            if self.constraint.is_known() {
                write!(f, [space(), text("extends"), space(), &self.constraint])
            } else {
                Ok(())
            }
        });

        let default = format_with(|f| {
            if self.default.is_known() {
                write!(f, [space(), text("="), space(), &self.constraint])
            } else {
                Ok(())
            }
        });

        write!(
            f,
            [&format_args![
                dynamic_text(&self.name, TextSize::default()),
                constraint, default
            ]]
        )
    }
}

impl Format<FormatTypeContext> for TypeReference {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            Self::Qualifier(qualifier) => {
                write!(
                    f,
                    [&format_args![
                        text("unresolved reference"),
                        space(),
                        qualifier.as_ref()
                    ]]
                )
            }
            Self::Resolved(resolved) => {
                let level = resolved.level();
                let id = resolved.id();
                if level == TypeResolverLevel::Global {
                    if resolved.index() < NUM_PREDEFINED_TYPES {
                        write!(f, [text(global_type_name(id))])
                    } else {
                        // Start counting from `NUM_PREDEFINED_TYPES` so
                        // snapshots remain stable even if we add new predefined
                        // types.
                        let id = TypeId::new(id.index() - NUM_PREDEFINED_TYPES);
                        write!(
                            f,
                            [&format_args![
                                text("Global"),
                                space(),
                                dynamic_text(&std::format!("{id:?}"), TextSize::default()),
                            ]]
                        )
                    }
                } else if level == TypeResolverLevel::Thin {
                    let module_id = resolved.module_id().index();
                    write!(
                        f,
                        [&format_args![
                            dynamic_text(&std::format!("Module({module_id})"), TextSize::default()),
                            space(),
                            dynamic_text(&std::format!("{id:?}"), TextSize::default()),
                        ]]
                    )
                } else {
                    write!(
                        f,
                        [&format_args![
                            dynamic_text(&std::format!("{level:?}"), TextSize::default()),
                            space(),
                            dynamic_text(&std::format!("{id:?}"), TextSize::default()),
                        ]]
                    )
                }
            }
            Self::Import(import) => write!(f, [import.as_ref()]),
            Self::Unknown => write!(f, [text("unknown reference")]),
        }
    }
}

impl Format<FormatTypeContext> for TypeReferenceQualifier {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let type_args = format_with(|f| {
            if self.type_parameters.is_empty() {
                Ok(())
            } else {
                write!(f, [text("<")])?;
                for (index, param) in self.type_parameters.iter().enumerate() {
                    write!(f, [param])?;
                    if index != self.type_parameters.len() - 1 {
                        write!(f, [text(","), space()])?;
                    }
                }
                write!(f, [text(">")])
            }
        });

        let scope_id = format_with(|f| {
            write!(
                f,
                [&format_args![
                    space(),
                    dynamic_text(
                        &std::format!("(scope ID: {})", self.scope_id.index()),
                        TextSize::default()
                    )
                ]]
            )
        });

        write!(f, [text("\"")])?;
        for (index, part) in self.path.iter().enumerate() {
            write!(f, [dynamic_text(part, TextSize::default())])?;
            if index != self.path.len() - 1 {
                write!(f, [text(".")])?;
            }
        }
        write!(f, [text("\""), type_args, scope_id])?;
        Ok(())
    }
}

impl Format<FormatTypeContext> for TypeImportQualifier {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [
                self.symbol,
                space(),
                text("from"),
                space(),
                self.resolved_path
            ]
        )
    }
}

impl Format<FormatTypeContext> for MergedReference {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("("),
                text("type:"),
                space(),
                &self.ty,
                text(","),
                space(),
                text("value:"),
                space(),
                &self.value_ty,
                text(","),
                space(),
                text("namespace:"),
                space(),
                &self.namespace_ty,
                text(")")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Class {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let name = format_with(|f| {
            if let Some(name) = &self.name {
                write!(
                    f,
                    [dynamic_text(
                        &std::format!("\"{}\"", name),
                        TextSize::default()
                    )]
                )
            } else {
                Ok(())
            }
        });
        let extends = format_with(|f| {
            if let Some(extends) = &self.extends {
                write!(f, [extends])
            } else {
                write!(f, [text("none")])
            }
        });

        write!(
            f,
            [&format_args![
                text("class"),
                space(),
                name,
                space(),
                text("{"),
                &group(&block_indent(&format_args![
                    text("extends:"),
                    space(),
                    extends,
                    hard_line_break(),
                    text("implements:"),
                    space(),
                    FmtTypeReferences(self.implements.as_ref()),
                    hard_line_break(),
                    text("type_args:"),
                    space(),
                    FmtTypeReferences(&self.type_parameters),
                ])),
                text("}")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Interface {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("interface"),
                space(),
                dynamic_text(&std::format!("\"{}\"", self.name), TextSize::default()),
                space(),
                text("{"),
                &group(&block_indent(&format_args![
                    text("extends:"),
                    space(),
                    FmtTypeReferences(self.extends.as_ref()),
                    hard_line_break(),
                    text("type_args:"),
                    space(),
                    FmtTypeReferences(&self.type_parameters),
                    hard_line_break(),
                    text("members:"),
                    space(),
                    FmtTypeMembers(self.members.as_ref()),
                ])),
                text("}")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Literal {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [&format_args![text("value:"), space()]])?;
        match self {
            Self::BigInt(text) => write!(f, [dynamic_text(text, TextSize::default())]),
            Self::Boolean(lit) => write!(
                f,
                [dynamic_text(
                    lit.as_bool().to_string().as_str(),
                    TextSize::default()
                )]
            ),
            Self::Number(lit) => {
                write!(f, [dynamic_text(lit.as_str(), TextSize::default())])
            }
            Self::Object(obj) => write!(f, [&obj]),
            Self::RegExp(text) => write!(f, [dynamic_text(text, TextSize::default())]),
            Self::String(lit) => write!(f, [dynamic_text(lit.as_str(), TextSize::default())]),
            Self::Template(text) => write!(f, [dynamic_text(text, TextSize::default())]),
        }
    }
}

impl Format<FormatTypeContext> for ObjectLiteral {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("ObjectLiteral"),
                space(),
                text("{"),
                &group(&soft_block_indent(&format_args![
                    text("members:"),
                    space(),
                    FmtTypeMembers(self.0.as_ref())
                ])),
                text("}")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for TypeInstance {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let type_args = format_with(|f| {
            if self.type_parameters.is_empty() {
                Ok(())
            } else {
                write!(f, [text("<")])?;
                for (index, param) in self.type_parameters.iter().enumerate() {
                    write!(f, [param])?;
                    if index != self.type_parameters.len() - 1 {
                        write!(f, [text(","), space()])?;
                    }
                }
                write!(f, [text(">")])
            }
        });

        write!(f, [&format_args![self.ty, type_args]])
    }
}

impl Format<FormatTypeContext> for Union {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let references = format_with(|f| {
            for (index, reference) in self.0.iter().enumerate() {
                write!(f, [&format_args![reference]])?;
                if index != self.0.len() - 1 {
                    write!(f, [space(), text("|"), space()])?;
                }
            }
            Ok(())
        });

        write!(f, [&format_args![references]])
    }
}

impl Format<FormatTypeContext> for ResolvedPath {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let value = self.deref();
        if let Ok(value) = value {
            let quoted = std::format!("\"{}\"", value.as_str().replace('\\', "/"));
            write!(f, [dynamic_text(&quoted, TextSize::default())])?;
        }

        Ok(())
    }
}

impl Format<FormatTypeContext> for ImportSymbol {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let import = format_with(|f| match self {
            Self::Default => write!(f, [text("Default")]),
            Self::Named(name) => {
                write!(f, [dynamic_text(name, TextSize::default())])
            }
            Self::All => write!(f, [text("All")]),
        });
        write!(f, [&format_args![text("Import Symbol:"), space(), &import]])
    }
}

// #region Format utilities

struct FmtFunctionParameters<'a>(&'a [FunctionParameter]);
impl Format<FormatTypeContext> for FmtFunctionParameters<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [text("[]")]);
        }

        let function_parameters = format_with(|f| {
            let separator = format_with(|f| write!(f, [&format_args![soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![text("["), &function_parameters, text("]")]]
        )
    }
}

struct FmtFunctionParameterBindings<'a>(&'a [FunctionParameterBinding]);
impl Format<FormatTypeContext> for FmtFunctionParameterBindings<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }

        let function_parameters = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![text(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for part in self.0 {
                joiner.entry(&format_args![&part.name, text(":"), &part.ty]);
            }
            joiner.finish()
        });
        write!(f, [&function_parameters])
    }
}

struct FmtTypeMembers<'a>(&'a [TypeMember]);

impl Format<FormatTypeContext> for FmtTypeMembers<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [text("[")])?;

        let types = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![text(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![group(&soft_block_indent(&types)), text("]")]]
        )
    }
}

struct FmtTypeReferences<'a>(&'a [TypeReference]);

impl Format<FormatTypeContext> for FmtTypeReferences<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [text("[")])?;

        let references = format_with(|f| {
            let separator =
                format_with(|f| write!(f, [&format_args![text(","), soft_line_break_or_space()]]));
            let mut joiner = f.join_with(separator);
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![
                group(&soft_block_indent(&references)),
                text("]")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Text {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [&format_args![dynamic_text(self, TextSize::default())]])
    }
}

struct FmtCallArgumentType<'a>(&'a [CallArgumentType]);

impl Format<FormatTypeContext> for FmtCallArgumentType<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [text("No parameters")]);
        }

        let type_parameters = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for part in self.0 {
                match part {
                    CallArgumentType::Argument(ty) => joiner.entry(&format_args![ty]),
                    CallArgumentType::Spread(ty) => joiner.entry(&format_args![text("..."), ty]),
                };
            }
            joiner.finish()
        });
        write!(f, [&format_args![&type_parameters]])
    }
}

struct FmtNames<'a>(&'a [Text]);

impl Format<FormatTypeContext> for FmtNames<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }

        let types = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(f, [&format_args![&types]])
    }
}

struct FmtVerbatim<'a, T>(&'a T);

impl<T> Format<FormatTypeContext> for FmtVerbatim<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let text = std::format!("{:#?}", self.0);
        write!(
            f,
            [&format_args![dynamic_text(&text, TextSize::default()),]]
        )
    }
}

// #endregion
