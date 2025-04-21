use crate::js_module_info::Exports;
use crate::{
    JsExport, JsImport, JsImportSymbol, JsModuleInfo, JsOwnExport, JsReexport, JsResolvedPath,
    JsdocComment,
};
use biome_formatter::prelude::*;
use biome_formatter::{
    FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth, TransformSourceMap,
    format_args, write,
};
use biome_js_type_info::{
    CallArgumentType, CallSignatureTypeMember, Class, ConstructorTypeMember, DestructureField,
    Function, FunctionParameter, GenericTypeParameter, Literal, MethodTypeMember, Object,
    ObjectLiteral, PropertyTypeMember, ReturnType, Type, TypeInner, TypeMember, TypeReference,
    TypeReferenceQualifier, TypeofAwaitExpression, TypeofExpression,
};
use biome_rowan::{Text, TextSize};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Debug, Default)]
struct ModuleFormatContext;

#[derive(Debug, Clone, Default)]

struct ModuleFormatContextOptions;

impl FormatOptions for ModuleFormatContextOptions {
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
        LineEnding::default()
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

impl biome_formatter::FormatContext for ModuleFormatContext {
    type Options = ModuleFormatContextOptions;

    fn options(&self) -> &Self::Options {
        &ModuleFormatContextOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl std::fmt::Display for JsModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(ModuleFormatContext, [&self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<ModuleFormatContext> for JsModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let exports = format_with(|f| {
            if self.exports.is_empty() {
                write!(f, [text("No exports")])
            } else {
                write!(f, [&self.exports])
            }
        });

        write!(
            f,
            [&format_args![
                text("Exports"),
                space(),
                text("{"),
                &group(&block_indent(&exports)),
                text("}"),
            ]]
        )
    }
}

impl Format<ModuleFormatContext> for Exports {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let mut joiner = f.join();
        for (export_name, export) in self.deref() {
            let name = format_with(|f| match export_name {
                Text::Borrowed(t) => {
                    write!(
                        f,
                        [dynamic_text(
                            &std::format!("{:?}", t.text()),
                            TextSize::default()
                        ),]
                    )
                }
                Text::Owned(t) => {
                    write!(
                        f,
                        [dynamic_text(
                            &std::format!("{:?}", t.as_str()),
                            TextSize::default()
                        ),]
                    )
                }
                Text::Static(t) => {
                    write!(
                        f,
                        [dynamic_text(&std::format!("{:?}", t), TextSize::default()),]
                    )
                }
            });
            let arrow = format_with(|f| write!(f, [&format_args![space(), text("=>"), space()]]));

            let export = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        text("{"),
                        &group(&block_indent(&format_args![export]),),
                        text("}")
                    ]]
                )
            });
            let line = format_with(|f| {
                write!(
                    f,
                    [&format_args![&name, &arrow, &export, soft_line_break()]]
                )
            });
            joiner.entry(&group(&format_args![line, soft_line_break_or_space()]));
        }

        joiner.finish()
    }
}

impl Format<ModuleFormatContext> for JsExport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(f, [text("Export")])?;
        match self {
            Self::Own(export) => {
                write!(
                    f,
                    [&format_args![
                        text("OwnExport"),
                        space(),
                        text("=>"),
                        space(),
                        &export
                    ]]
                )
            }
            Self::OwnType(own_type) => {
                write!(
                    f,
                    [&format_args![
                        text("OnwType"),
                        space(),
                        text("=>"),
                        space(),
                        &own_type
                    ]]
                )
            }
            Self::Reexport(reexport) => {
                write!(
                    f,
                    [&format_args![
                        text("Reexport"),
                        space(),
                        text("=>"),
                        space(),
                        &reexport
                    ]]
                )
            }
            Self::ReexportType(reexport_type) => {
                write!(
                    f,
                    [&format_args![
                        text("ReexportType"),
                        space(),
                        text("=>"),
                        space(),
                        &reexport_type
                    ]]
                )
            }
        }
    }
}

impl Format<ModuleFormatContext> for JsReexport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let content = format_with(|f| {
            write!(f, [&format_args![&self.import]])?;

            if let Some(comment) = &self.jsdoc_comment {
                write!(f, [&format_args![&comment]])?;
            }

            Ok(())
        });

        write!(
            f,
            [&format_args![
                text("Reexport"),
                text("("),
                block_indent(&content),
                text(")")
            ],]
        )?;

        Ok(())
    }
}

impl Format<ModuleFormatContext> for JsOwnExport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let content = format_with(|f| {
            write!(f, [&self.ty])?;

            write!(f, [hard_line_break()])?;

            write!(f, [text("Local name: ")])?;
            if let Some(name) = &self.local_name {
                write!(f, [&format_args![dynamic_text(name, TextSize::default())]])?;
            } else {
                write!(f, [&format_args![text("None")]])?;
            }

            write!(f, [hard_line_break()])?;

            if let Some(comment) = &self.jsdoc_comment {
                write!(f, [&format_args![&comment]])?;
                write!(f, [&format_args![soft_line_break()]])?;
            }

            Ok(())
        });

        write!(
            f,
            [&format_args![
                text("JsOwnExport"),
                text("("),
                block_indent(&content),
                text(")")
            ],]
        )?;

        Ok(())
    }
}

impl Format<ModuleFormatContext> for JsdocComment {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let comment = self.deref();

        let comment = format_with(|f| {
            let mut joiner = f.join_with(hard_line_break());
            comment.lines().for_each(|line| {
                joiner.entry(&format_args![dynamic_text(
                    line.trim(),
                    TextSize::default()
                ),]);
            });
            joiner.finish()
        });

        write!(
            f,
            [&format_args![
                text("JsDoc"),
                text("("),
                block_indent(&comment),
                text(")")
            ]]
        )
    }
}

impl Format<ModuleFormatContext> for JsImport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("Specifier:"),
                space(),
                dynamic_text(
                    &std::format!("{:?}", self.specifier.text()),
                    TextSize::default()
                ),
            ]]
        )?;
        write!(f, [hard_line_break()])?;

        write!(
            f,
            [&format_args![
                text("Resolved path:"),
                space(),
                self.resolved_path
            ]]
        )?;

        write!(f, [hard_line_break()])?;

        write!(f, [&format_args![&self.symbol]])?;

        write!(f, [hard_line_break()])?;
        Ok(())
    }
}

impl Format<ModuleFormatContext> for Type {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self.deref() {
            TypeInner::Unknown => write!(f, [&format_args![text("Unknown")]]),
            TypeInner::Undefined => write!(f, [&format_args![text("Undefined")]]),
            TypeInner::Null => write!(f, [&format_args![text("Null")]]),
            TypeInner::Boolean => write!(f, [&format_args![text("Boolean")]]),
            TypeInner::Number => write!(f, [&format_args![text("Number")]]),
            TypeInner::BigInt => write!(f, [&format_args![text("BigInt")]]),
            TypeInner::String => write!(f, [&format_args![text("String")]]),
            TypeInner::Symbol => write!(f, [&format_args![text("Symbol")]]),
            TypeInner::Object(obj) => write!(
                f,
                [&format_args![
                    text("Object"),
                    text("("),
                    group(&soft_block_indent(&obj.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::Function(func) => write!(f, [&func.as_ref(),]),
            TypeInner::Union(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Intersection(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Tuple(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Namespace(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Class(class) => write!(f, [&class.as_ref()]),
            TypeInner::Constructor(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::TypeOperator(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Alias(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::Literal(literal) => write!(
                f,
                [&format_args![
                    text("Literal"),
                    text("("),
                    group(&soft_block_indent(&literal.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::InstanceOf(ty) => write!(f, [text("instanceof"), space(), &ty.as_ref()]),
            TypeInner::Reference(reference) => write!(f, [&reference.as_ref(),]),
            TypeInner::TypeofType(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::TypeofValue(ty) => write!(f, [FmtVerbatim(&ty.as_ref())]),
            TypeInner::AnyKeyword => write!(f, [&format_args![text("AnyKeyword")]]),
            TypeInner::NeverKeyword => write!(f, [&format_args![text("NeverKeyword")]]),
            TypeInner::ObjectKeyword => write!(f, [&format_args![text("ObjectKeyword")]]),
            TypeInner::ThisKeyword => write!(f, [&format_args![text("ThisKeyword")]]),
            TypeInner::UnknownKeyword => write!(f, [&format_args![text("UnknownKeyword")]]),
            TypeInner::VoidKeyword => write!(f, [&format_args![text("VoidKeyword")]]),
            TypeInner::TypeofExpression(typeof_expression) => {
                write!(
                    f,
                    [&format_args![
                        text("Reference"),
                        text("("),
                        group(&soft_block_indent(&typeof_expression.as_ref())),
                        text(")")
                    ]]
                )
            }
        }
    }
}

impl Format<ModuleFormatContext> for Literal {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            Self::BigInt(value) => write!(
                f,
                [&format_args![
                    text("BigInt"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Self::Boolean(value) => write!(
                f,
                [&format_args![
                    text("Boolean"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Self::Null => write!(f, [&format_args![text("Null")]]),
            Self::Number(value) => write!(
                f,
                [&format_args![
                    text("Number"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Self::Object(value) => write!(
                f,
                [&format_args![
                    text("Object"),
                    text("("),
                    block_indent(&format_args![value]),
                    text(")")
                ]]
            ),
            Self::RegExp(value) => write!(
                f,
                [&format_args![
                    text("RegExp"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Self::String(value) => write!(
                f,
                [&format_args![
                    text("String"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Self::Template(value) => write!(
                f,
                [&format_args![
                    text("Template"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
        }
    }
}

impl Format<ModuleFormatContext> for ObjectLiteral {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text("TypeMembers"),
                FmtTypeMembers(self.members())
            ]]
        )
    }
}
impl Format<ModuleFormatContext> for JsResolvedPath {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let value = self.deref();
        if let Ok(value) = value {
            write!(
                f,
                [format_args![dynamic_text(
                    value.as_str().replace('\\', "/").as_str(),
                    TextSize::default()
                )]]
            )?;
        }

        Ok(())
    }
}

impl Format<ModuleFormatContext> for JsImportSymbol {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let import = format_with(|f| match self {
            Self::Default => write!(f, [&format_args![text("Default")]]),
            Self::Named(name) => {
                write!(f, [&format_args![dynamic_text(name, TextSize::default())]])
            }
            Self::All => write!(f, [&format_args![text("All")]]),
        });
        write!(f, [&format_args![text("Import Symbol:"), space(), &import]])
    }
}

impl Format<ModuleFormatContext> for TypeReference {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for TypeReferenceQualifier {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for Object {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let mut joiner = f.join_with(soft_line_break());
        for type_member in self.all_members() {
            joiner.entry(&format_args![&type_member]);
        }
        joiner.finish()
    }
}

impl Format<ModuleFormatContext> for TypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            Self::CallSignature(ty) => {
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
            Self::Constructor(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::Method(method) => {
                write!(f, [&format_args![&method]])
            }
            Self::Property(property) => {
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

impl Format<ModuleFormatContext> for CallSignatureTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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
        )?;

        Ok(())
    }
}

impl Format<ModuleFormatContext> for GenericTypeParameter {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for FunctionParameter {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let optional = format_with(|f| {
            if self.is_optional {
                write!(f, [&format_args![text("The parameter is optional")]])
            } else {
                write!(f, [&format_args![text("The parameter is not optional")]])
            }
        });
        let rest = format_with(|f| {
            if self.is_rest {
                write!(f, [&format_args![text("The parameter is rest")]])
            } else {
                write!(f, [&format_args![text("The parameter is not rest")]])
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

impl Format<ModuleFormatContext> for ReturnType {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            Self::Type(ty) => {
                write!(f, [&ty])
            }
            Self::Predicate(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::Asserts(_asset) => {
                todo!()
            }
        }
    }
}

impl Format<ModuleFormatContext> for Function {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for PropertyTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let is_optional = format_with(|f| {
            if self.is_optional {
                write!(f, [&format_args![text("The parameter is optional")]])
            } else {
                write!(f, [&format_args![text("The parameter is not optional")]])
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

impl Format<ModuleFormatContext> for Class {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let members = format_with(|f| {
            write!(f, [&format_args![text("Members"), text("("),]])?;

            let types = format_with(|f| {
                let mut joiner = f.join_with(soft_line_break());
                for part in self.members.as_ref() {
                    joiner.entry(&format_args![part]);
                }
                joiner.finish()
            });
            write!(
                f,
                [&format_args![group(&soft_block_indent(&types)), text(")")]]
            )
        });
        write!(
            f,
            [&format_args![
                text("["),
                self.name,
                text("]"),
                hard_line_break(),
                members
            ]]
        )
    }
}

impl Format<ModuleFormatContext> for ConstructorTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                FmtGenericTypeParameters(&self.type_parameters),
                hard_line_break(),
                FmtFunctionParameters(&self.parameters),
                text("ReturnType"),
                text("("),
                group(&soft_block_indent(&self.return_type)),
                text(")")
            ]]
        )
    }
}

impl Format<ModuleFormatContext> for MethodTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for TypeofExpression {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            Self::Addition(ty) => write!(f, [FmtVerbatim(&ty)]),
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
            Self::Call(call) => {
                write!(
                    f,
                    [&format_args![
                        text("Call"),
                        space(),
                        call.callee,
                        space(),
                        text("("),
                        group(&soft_block_indent(&FmtCallArgumentType(&call.arguments))),
                        text(")")
                    ]]
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
            Self::New(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::StaticMember(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::Super(ty) => write!(f, [FmtVerbatim(&ty)]),
            Self::This(ty) => write!(f, [FmtVerbatim(&ty)]),
        }
    }
}

impl Format<ModuleFormatContext> for TypeofAwaitExpression {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![group(&soft_block_indent(&self.argument)),]]
        )
    }
}

// #region Formatting Helpers

struct FmtFunctionParameters<'a>(&'a [FunctionParameter]);
impl Format<ModuleFormatContext> for FmtFunctionParameters<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

struct FmtCallArgumentType<'a>(&'a [CallArgumentType]);

impl Format<ModuleFormatContext> for FmtCallArgumentType<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
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

struct FmtGenericTypeParameters<'a>(&'a [GenericTypeParameter]);

impl Format<ModuleFormatContext> for FmtGenericTypeParameters<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [text("No parameters")]);
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

impl Format<ModuleFormatContext> for FmtTypeMembers<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

impl Format<ModuleFormatContext> for FmtTypes<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
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

struct FmtNames<'a>(&'a [Text]);

impl Format<ModuleFormatContext> for FmtNames<'_> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
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

impl Format<ModuleFormatContext> for Text {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![dynamic_text(
                self.text(),
                TextSize::default()
            )]]
        )
    }
}
struct FmtVerbatim<'a, T>(&'a T);

impl<T> Format<ModuleFormatContext> for FmtVerbatim<'_, T>
where
    T: Debug,
{
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let text = std::format!("{:#?}", self.0);
        write!(
            f,
            [&format_args![dynamic_text(&text, TextSize::default()),]]
        )
    }
}

// #endregion
