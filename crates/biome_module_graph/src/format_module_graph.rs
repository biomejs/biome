use crate::js_module_info::{Exports, Imports, JsBindingData};
use crate::{JsExport, JsImport, JsModuleInfo, JsOwnExport, JsReexport};
use biome_formatter::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_type_info::FormatTypeContext;
use biome_rowan::TextSize;
use std::fmt::Formatter;
use std::ops::Deref;

impl std::fmt::Display for JsModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Format<FormatTypeContext> for JsModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let exports = format_with(|f| {
            if self.exports.is_empty() {
                write!(f, [text("No exports")])
            } else {
                write!(f, [&self.exports])
            }
        });

        let static_imports = format_with(|f| {
            if self.static_imports.is_empty() {
                write!(f, [text("No imports")])
            } else {
                write!(f, [&self.static_imports])
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
                hard_line_break(),
                text("Imports"),
                space(),
                text("{"),
                &group(&block_indent(&static_imports)),
                text("}"),
            ]]
        )
    }
}

impl Format<FormatTypeContext> for Exports {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let mut joiner = f.join();
        for (export_name, export) in self.deref() {
            let name = format_with(|f| {
                write!(
                    f,
                    [dynamic_text(
                        &std::format!("{:?}", export_name.text()),
                        TextSize::default()
                    ),]
                )
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

impl Format<FormatTypeContext> for Imports {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let mut joiner = f.join();

        for (import_name, import) in &self.0 {
            let name = format_with(|f| {
                write!(
                    f,
                    [dynamic_text(
                        &std::format!("{:?}", import_name.text()),
                        TextSize::default()
                    ),]
                )
            });
            let arrow = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        space(),
                        biome_formatter::prelude::text("=>"),
                        space()
                    ]]
                )
            });

            let export = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        text("{"),
                        &group(&block_indent(&format_args![import]),),
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

        Ok(())
    }
}

impl Format<FormatTypeContext> for JsExport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
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
                        text("OwnType"),
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

impl std::fmt::Display for JsBindingData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Format<FormatTypeContext> for JsBindingData {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let jsdoc_comment = format_with(|f| {
            if let Some(jsdoc) = &self.jsdoc {
                write!(
                    f,
                    [&format_args![
                        text("JSDoc comment:"),
                        space(),
                        jsdoc,
                        text(","),
                        hard_line_break()
                    ]]
                )
            } else {
                Ok(())
            }
        });

        let content = format_with(|f| {
            write!(
                f,
                [&format_args![
                    text("Name:"),
                    space(),
                    dynamic_text(&self.name, TextSize::default()),
                    text(","),
                    hard_line_break(),
                    text("Type:"),
                    space(),
                    &self.ty,
                    text(","),
                    hard_line_break(),
                    jsdoc_comment,
                    text("Declaration kind:"),
                    space(),
                    dynamic_text(
                        &std::format!("{:?}", self.declaration_kind),
                        TextSize::default()
                    ),
                ]]
            )
        });

        write!(
            f,
            [&format_args![
                text("JsBindingData {"),
                block_indent(&content),
                text("}")
            ],]
        )?;

        Ok(())
    }
}

impl Format<FormatTypeContext> for JsReexport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
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

impl Format<FormatTypeContext> for JsOwnExport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        match self {
            Self::Binding(binding_id) => write!(
                f,
                [&format_args![
                    text("JsOwnExport::Binding("),
                    dynamic_text(&binding_id.index().to_string(), TextSize::default()),
                    text(")")
                ]]
            ),
            Self::Type(resolved_type_id) => write!(
                f,
                [&format_args![
                    text("JsOwnExport::Type("),
                    dynamic_text(&std::format!("{resolved_type_id:?}"), TextSize::default()),
                    text(")")
                ]]
            ),
        }
    }
}

impl Format<FormatTypeContext> for JsImport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
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
