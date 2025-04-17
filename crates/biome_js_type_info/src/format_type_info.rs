use crate::{
    CallSignatureTypeMember, Class, Function, FunctionParameter, GenericTypeParameter,
    MethodTypeMember, Object, PropertyTypeMember, ReturnType, Type, TypeInner, TypeMember,
    TypeReference, TypeReferenceQualifier,
};
use biome_formatter::prelude::*;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TransformSourceMap,
};
use biome_formatter::{format_args, write};
use biome_js_syntax::TextSize;
use biome_rowan::Text;
use std::ops::Deref;

struct FormatTypeOptions;

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

struct FormatTypeContext;

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

impl Format<FormatTypeContext> for Type {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [&self.deref()])
    }
}

impl Format<FormatTypeContext> for TypeInner {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            TypeInner::Unknown => write!(f, [text("unknown")]),
            TypeInner::BigInt => write!(f, [text("BigInt")]),
            TypeInner::Boolean => write!(f, [text("boolean")]),
            TypeInner::Null => write!(f, [text("null")]),
            TypeInner::Number => write!(f, [text("number")]),
            TypeInner::String => write!(f, [text("string")]),
            TypeInner::Symbol => write!(f, [text("symbol")]),
            TypeInner::Undefined => write!(f, [text("undefined")]),
            TypeInner::Class(class) => write!(f, [&class.as_ref()]),
            TypeInner::Constructor(_) => todo!(),
            TypeInner::Function(function) => write!(f, [&function.as_ref()]),
            TypeInner::Namespace(_) => todo!(),
            TypeInner::Object(object) => write!(f, [&object.as_ref()]),
            TypeInner::Tuple(_) => todo!(),
            TypeInner::Intersection(_) => todo!(),
            TypeInner::Union(_) => todo!(),
            TypeInner::TypeOperator(_) => todo!(),
            TypeInner::Alias(_) => todo!(),
            TypeInner::Literal(_) => todo!(),
            TypeInner::Reference(reference) => write!(f, [&reference.as_ref()]),
            TypeInner::TypeofExpression(_) => todo!(),
            TypeInner::TypeofType(_) => todo!(),
            TypeInner::TypeofValue(_) => todo!(),
            TypeInner::AnyKeyword => write!(f, [text("any")]),
            TypeInner::NeverKeyword => write!(f, [text("never")]),
            TypeInner::ObjectKeyword => write!(f, [text("object")]),
            TypeInner::ThisKeyword => write!(f, [text("this")]),
            TypeInner::UnknownKeyword => write!(f, [text("unknown")]),
            TypeInner::VoidKeyword => write!(f, [text("void")]),
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
                    text("{"),
                    FmtTypeMembers(self.members.as_ref()),
                    text("}"),
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
                        &std::format!("\"{}\"", name),
                        TextSize::default()
                    ),]
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
                        FmtGenericTypeParameters(&self.type_parameters),
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
            ReturnType::Type(ty) => {
                write!(f, [&ty])
            }
            ReturnType::Predicate(_) => todo!(),
            ReturnType::Asserts(_asset) => {
                todo!()
            }
        }
    }
}

impl Format<FormatTypeContext> for FunctionParameter {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let optional = format_with(|f| {
            if self.is_optional {
                write!(f, [&format_args![text("optional")]])
            } else {
                write!(f, [&format_args![text("required")]])
            }
        });
        let rest = format_with(|f| {
            if self.is_rest {
                write!(f, [&format_args![text("rest")]])
            } else {
                write!(f, [&format_args![text("not_rest")]])
            }
        });
        write!(
            f,
            [&group(&format_args![
                text("["),
                self.name,
                text(","),
                space(),
                optional,
                text(","),
                space(),
                rest,
                text("]")
            ])]
        )
    }
}

impl Format<FormatTypeContext> for TypeMember {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        match self {
            TypeMember::CallSignature(ty) => {
                write!(
                    f,
                    [&format_args![
                        text("CallSignature"),
                        text("("),
                        &group(&soft_block_indent(&ty)),
                        text(")")
                    ]]
                )
            }
            TypeMember::Constructor(_) => todo!(),
            TypeMember::Method(method) => {
                write!(f, [&format_args![&method]])
            }
            TypeMember::Property(property) => {
                write!(
                    f,
                    [&format_args![
                        text("Property"),
                        text("("),
                        &group(&soft_block_indent(&property)),
                        text(")")
                    ]]
                )
            }
        }
    }
}

impl Format<FormatTypeContext> for PropertyTypeMember {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let is_optional = format_with(|f| {
            if self.is_optional {
                write!(f, [&format_args![text("optional")]])
            } else {
                write!(f, [&format_args![text("required")]])
            }
        });
        write!(
            f,
            [&format_args![
                text("["),
                dynamic_text(&self.name, TextSize::default()),
                text(","),
                space(),
                is_optional,
                text("]"),
                hard_line_break(),
                text("Type"),
                text("("),
                group(&soft_block_indent(&self.ty)),
                text(")")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for MethodTypeMember {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let is_async = format_with(|f| {
            if self.is_async {
                write!(f, [&format_args![text("async")]])
            } else {
                write!(f, [&format_args![text("sync")]])
            }
        });

        let is_optional = format_with(|f| {
            if self.is_optional {
                write!(f, [&format_args![text("optional")]])
            } else {
                write!(f, [&format_args![text("required")]])
            }
        });
        write!(
            f,
            [&format_args![
                is_optional,
                space(),
                is_async,
                space(),
                text("Method"),
                space(),
                dynamic_text(&std::format!("\"{}\"", &self.name), TextSize::default()),
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
                        FmtGenericTypeParameters(&self.type_parameters),
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

impl Format<FormatTypeContext> for CallSignatureTypeMember {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                FmtGenericTypeParameters(&self.type_parameters),
                FmtFunctionParameters(&self.parameters),
                text("ReturnType"),
                text("("),
                group(&soft_block_indent(&self.return_type)),
                text(")")
            ]]
        )
    }
}

impl Format<FormatTypeContext> for GenericTypeParameter {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("Name:"),
                space(),
                dynamic_text(&self.name, TextSize::default()),
                text("Type:"),
                &self.ty
            ]]
        )
    }
}

impl Format<FormatTypeContext> for TypeReference {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                &self.qualifier,
                space(),
                text("{"),
                &group(&block_indent(&format_args![
                    text("params:"),
                    space(),
                    &self.ty,
                    hard_line_break(),
                    text("type_args:"),
                    space(),
                    FmtTypes(self.type_parameters.as_ref())
                ])),
                text("}"),
            ]]
        )
    }
}

impl Format<FormatTypeContext> for TypeReferenceQualifier {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        write!(f, [text("\"")])?;
        for (index, part) in self.parts().iter().enumerate() {
            write!(f, [&format_args![dynamic_text(part, TextSize::default())]])?;
            if index != self.parts().len() - 1 {
                write!(f, [text(".")])?;
            }
        }
        write!(f, [text("\"")])?;
        Ok(())
    }
}

impl Format<FormatTypeContext> for Class {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        let members = format_with(|f| {
            let types = format_with(|f| {
                let mut joiner = f.join_with(soft_line_break());
                for part in self.members.as_ref() {
                    joiner.entry(&format_args![part]);
                }
                joiner.finish()
            });
            write!(f, [&types])
        });
        let name = format_with(|f| {
            if let Some(name) = &self.name {
                write!(
                    f,
                    [dynamic_text(
                        &std::format!("\"{}\"", name),
                        TextSize::default()
                    ),]
                )
            } else {
                Ok(())
            }
        });
        write!(
            f,
            [&format_args![
                name,
                space(),
                text("members:"),
                space(),
                text("{"),
                &group(&block_indent(&members)),
                text("}")
            ]]
        )
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
            let mut joiner = f.join_with(soft_line_break());
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(f, [&function_parameters])
    }
}

struct FmtGenericTypeParameters<'a>(&'a [GenericTypeParameter]);

impl Format<FormatTypeContext> for FmtGenericTypeParameters<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [text("[]")]);
        }

        let type_parameters = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(f, [&format_args![&type_parameters]])
    }
}

struct FmtTypeMembers<'a>(&'a [TypeMember]);

impl Format<FormatTypeContext> for FmtTypeMembers<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, [&format_args![text("TypeMembers"), text("("),]])?;

        let types = format_with(|f| {
            let mut joiner = f.join_with(soft_line_break());
            for part in self.0 {
                joiner.entry(&format_args![part]);
            }
            joiner.finish()
        });
        write!(
            f,
            [&format_args![group(&soft_block_indent(&types)), text(")")]]
        )
    }
}

struct FmtTypes<'a>(&'a [Type]);

impl Format<FormatTypeContext> for FmtTypes<'_> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [text("No types")]);
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

impl Format<FormatTypeContext> for Option<Text> {
    fn fmt(&self, f: &mut Formatter<FormatTypeContext>) -> FormatResult<()> {
        if let Some(name) = self.as_ref() {
            write!(f, [&format_args![dynamic_text(name, TextSize::default())]])
        } else {
            write!(f, [&format_args![text("No name")]])
        }
    }
}

// #endregion
