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
    CallSignatureTypeMember, Class, ClassMember, ConstructorTypeMember, Function,
    FunctionParameter, GenericTypeParameter, Literal, MethodTypeMember, Object, ObjectLiteral,
    PropertyTypeMember, ReturnType, Type, TypeInner, TypeMember, TypeReference,
    TypeReferenceQualifier,
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
        let formatted = biome_formatter::format!(ModuleFormatContext, [&self.exports])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl std::fmt::Display for Exports {
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
                        [&format_args![
                            text("Borrowed("),
                            dynamic_text(&std::format!("{:?}", t.text()), TextSize::default()),
                            text(")")
                        ]]
                    )
                }
                Text::Owned(t) => {
                    write!(
                        f,
                        [&format_args![
                            text("Owned("),
                            dynamic_text(&std::format!("{:?}", t.as_str()), TextSize::default()),
                            text(")")
                        ]]
                    )
                }
                Text::Static(t) => {
                    write!(
                        f,
                        [&format_args![
                            text("Static("),
                            dynamic_text(&std::format!("{:?}", t), TextSize::default()),
                            text(")")
                        ]]
                    )
                }
            });
            let arrow = format_with(|f| write!(f, [&format_args![space(), text("=>"), space()]]));

            let export = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        text("["),
                        block_indent(&format_args![export]),
                        text("]")
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
            JsExport::Own(export) => {
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
            JsExport::OwnType(own_type) => {
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
            JsExport::Reexport(reexport) => {
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
            JsExport::ReexportType(reexport_type) => {
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
            TypeInner::Function(func) => write!(
                f,
                [&format_args![
                    text("Function"),
                    text("("),
                    group(&soft_block_indent(&func.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::Union(_) => todo!(),
            TypeInner::Intersection(_) => todo!(),
            TypeInner::Array(_) => todo!(),
            TypeInner::Tuple(_) => todo!(),
            TypeInner::Namespace(_) => todo!(),
            TypeInner::Class(class) => write!(
                f,
                [&format_args![
                    text("Class"),
                    text("("),
                    group(&soft_block_indent(&class.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::Constructor(_) => todo!(),
            TypeInner::Promise(promise) => {
                write!(
                    f,
                    [&format_args![
                        text("Promise"),
                        text("("),
                        promise.as_ref(),
                        text(")")
                    ]]
                )
            }
            TypeInner::TypeOperator(_) => todo!(),
            TypeInner::Alias(_) => todo!(),
            TypeInner::Literal(literal) => write!(
                f,
                [&format_args![
                    text("Literal"),
                    text("("),
                    group(&soft_block_indent(&literal.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::Reference(reference) => write!(
                f,
                [&format_args![
                    text("Reference"),
                    text("("),
                    group(&soft_block_indent(&reference.as_ref())),
                    text(")")
                ]]
            ),
            TypeInner::TypeofType(_) => todo!(),
            TypeInner::TypeofValue(_) => todo!(),
            TypeInner::AnyKeyword => write!(f, [&format_args![text("AnyKeyword")]]),
            TypeInner::NeverKeyword => write!(f, [&format_args![text("NeverKeyword")]]),
            TypeInner::ObjectKeyword => write!(f, [&format_args![text("ObjectKeyword")]]),
            TypeInner::ThisKeyword => write!(f, [&format_args![text("ThisKeyword")]]),
            TypeInner::UnknownKeyword => write!(f, [&format_args![text("UnknownKeyword")]]),
            TypeInner::VoidKeyword => write!(f, [&format_args![text("VoidKeyword")]]),
        }
    }
}

impl Format<ModuleFormatContext> for Literal {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            Literal::BigInt(value) => write!(
                f,
                [&format_args![
                    text("BigInt"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Literal::Boolean(value) => write!(
                f,
                [&format_args![
                    text("Boolean"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Literal::Null => write!(f, [&format_args![text("Null")]]),
            Literal::Number(value) => write!(
                f,
                [&format_args![
                    text("Number"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Literal::Object(value) => write!(
                f,
                [&format_args![
                    text("Object"),
                    text("("),
                    block_indent(&format_args![value]),
                    text(")")
                ]]
            ),
            Literal::RegExp(value) => write!(
                f,
                [&format_args![
                    text("RegExp"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Literal::String(value) => write!(
                f,
                [&format_args![
                    text("String"),
                    text(":"),
                    space(),
                    dynamic_text(value.text(), TextSize::default()),
                ]]
            ),
            Literal::Template(value) => write!(
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
        write!(f, [&format_args![text("TypeMembers")]])?;

        fmt_type_members(self.members(), f)
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
                    value.as_str().replace("\\", "/").as_str(),
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
            JsImportSymbol::Default => write!(f, [&format_args![text("Default")]]),
            JsImportSymbol::Named(name) => {
                write!(f, [&format_args![dynamic_text(name, TextSize::default())]])
            }
            JsImportSymbol::All => write!(f, [&format_args![text("All")]]),
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
                text("Qualifier"),
                text("("),
                group(&soft_block_indent(&self.qualifier)),
                text(")"),
                hard_line_break(),
                text("Type"),
                text("("),
                block_indent(&self.ty),
                text(")"),
                hard_line_break(),
            ]]
        )?;

        fmt_types(&self.type_parameters, f)
    }
}

impl Format<ModuleFormatContext> for TypeReferenceQualifier {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        for (index, part) in self.parts().iter().enumerate() {
            write!(f, [&format_args![dynamic_text(part, TextSize::default())]])?;
            if index != self.parts().len() - 1 {
                write!(f, [text(".")])?;
            }
        }

        Ok(())
    }
}

impl Format<ModuleFormatContext> for Object {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        let mut joiner = f.join_with(soft_line_break());
        for type_member in self.members() {
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
            TypeMember::Method(_) => todo!(),
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

impl Format<ModuleFormatContext> for CallSignatureTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        fmt_type_parameters(&self.type_parameters, f)?;
        fmt_function_parameters(&self.parameters, f)?;

        write!(
            f,
            [&format_args![
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
                text("Default Type:"),
                &self.default_ty
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
            ReturnType::Type(ty) => {
                write!(
                    f,
                    [&format_args![
                        text("Type"),
                        text("("),
                        &group(&soft_block_indent(&ty)),
                        text(")")
                    ]]
                )
            }
            ReturnType::Predicate(_) => todo!(),
            ReturnType::Asserts(_asset) => {
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

        write!(
            f,
            [&format_args![
                text("["),
                is_async,
                text(","),
                space(),
                text("Name"),
                text(":"),
                space(),
                self.name,
                text("]"),
                hard_line_break(),
            ]]
        )?;
        fmt_type_parameters(&self.type_parameters, f)?;
        write!(f, [hard_line_break()])?;
        fmt_function_parameters(&self.parameters, f)?;
        write!(
            f,
            [&format_args![
                hard_line_break(),
                text("ReturnType"),
                text("("),
                group(&soft_block_indent(&self.return_type)),
                text(")")
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

impl Format<ModuleFormatContext> for ClassMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        match self {
            ClassMember::Constructor(constructor) => {
                write!(
                    f,
                    [&format_args![
                        text("Constructor"),
                        text("("),
                        group(&soft_block_indent(&constructor)),
                        text(")")
                    ]]
                )
            }
            ClassMember::Method(method) => {
                write!(
                    f,
                    [&format_args![
                        text("Method"),
                        text("("),
                        group(&soft_block_indent(&method)),
                        text(")")
                    ]]
                )
            }
            ClassMember::Property(property) => {
                write!(
                    f,
                    [&format_args![
                        text("Property"),
                        text("("),
                        group(&soft_block_indent(&property)),
                        text(")")
                    ]]
                )
            }
        }
    }
}

impl Format<ModuleFormatContext> for ConstructorTypeMember {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        fmt_function_parameters(&self.parameters, f)?;
        fmt_type_parameters(&self.type_parameters, f)?;
        write!(
            f,
            [&format_args![
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
                is_async,
                text(","),
                space(),
                is_optional,
                text("]"),
                hard_line_break(),
            ]]
        )?;

        fmt_type_parameters(&self.type_parameters, f)?;
        fmt_function_parameters(&self.parameters, f)?;

        write!(
            f,
            [&format_args![
                hard_line_break(),
                text("ReturnType"),
                text("("),
                group(&soft_block_indent(&self.return_type)),
                text(")")
            ]]
        )
    }
}

// #region Formatting Helpers

fn fmt_function_parameters(
    function_parameters: &[FunctionParameter],
    f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
) -> FormatResult<()> {
    if function_parameters.is_empty() {
        return Ok(());
    }

    write!(f, [&format_args![text("FunctionParameters"), text("("),]])?;

    let function_parameters = format_with(|f| {
        let mut joiner = f.join_with(soft_line_break());
        for part in function_parameters {
            joiner.entry(&format_args![part]);
        }
        joiner.finish()
    });
    write!(
        f,
        [&format_args![
            group(&soft_block_indent(&function_parameters)),
            text(")")
        ]]
    )
}
fn fmt_type_parameters(
    type_parameters: &[GenericTypeParameter],
    f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
) -> FormatResult<()> {
    if type_parameters.is_empty() {
        return Ok(());
    }

    write!(f, [&format_args![text("TypeParameters"), text("("),]])?;

    let type_parameters = format_with(|f| {
        let mut joiner = f.join_with(soft_line_break());
        for part in type_parameters {
            joiner.entry(&format_args![part]);
        }
        joiner.finish()
    });
    write!(
        f,
        [&format_args![
            group(&soft_block_indent(&type_parameters)),
            text(")")
        ]]
    )
}

fn fmt_types(
    types: &[Type],
    f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
) -> FormatResult<()> {
    if types.is_empty() {
        return Ok(());
    }

    write!(f, [&format_args![text("Types"), text("("),]])?;

    let types = format_with(|f| {
        let mut joiner = f.join_with(soft_line_break());
        for part in types {
            joiner.entry(&format_args![part]);
        }
        joiner.finish()
    });
    write!(
        f,
        [&format_args![group(&soft_block_indent(&types)), text(")")]]
    )
}

fn fmt_type_members(
    type_members: &[TypeMember],
    f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
) -> FormatResult<()> {
    if type_members.is_empty() {
        return Ok(());
    }

    write!(f, [&format_args![text("TypeMembers"), text("("),]])?;

    let types = format_with(|f| {
        let mut joiner = f.join_with(soft_line_break());
        for part in type_members {
            joiner.entry(&format_args![part]);
        }
        joiner.finish()
    });
    write!(
        f,
        [&format_args![group(&soft_block_indent(&types)), text(")")]]
    )
}

impl Format<ModuleFormatContext> for Option<Text> {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<ModuleFormatContext>,
    ) -> FormatResult<()> {
        if let Some(name) = self.as_ref() {
            write!(f, [&format_args![dynamic_text(name, TextSize::default())]])
        } else {
            write!(f, [&format_args![text("No name")]])
        }
    }
}

// #endregion
