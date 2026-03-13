use crate::css_module_info::{CssClassDefinition, CssClassReference, CssImport, CssImports};
use crate::html_module_info::HtmlModuleInfoInner;
use crate::js_module_info::{Exports, Imports, JsBindingData};
use crate::{
    BindingTypeData, CssModuleInfo, HtmlModuleInfo, JsExport, JsImport, JsImportPath,
    JsImportPhase, JsModuleInfo, JsOwnExport, JsReexport, ModuleInfo,
};
use biome_formatter::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_type_info::FormatTypeContext;
use biome_rowan::{TextRange, TextSize};
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

impl Format<FormatTypeContext> for BindingTypeData {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let ranges: Vec<TypedRange> = self
            .export_ranges
            .iter()
            .map(|range| range.into())
            .collect();
        let export_ranges = format_with(|f| {
            let mut join = f.join();

            for range in ranges.clone() {
                join.entry(&range);
            }
            join.finish()
        });

        let jsdoc = format_with(|f| {
            if self.jsdoc.is_some() {
                write!(f, [&self.jsdoc, token(","), hard_line_break()])?;
            };
            Ok(())
        });
        write!(
            f,
            [
                token("BindingTypeData {"),
                &group(&block_indent(&format_args![
                    token("Types "),
                    &self.ty,
                    token(","),
                    hard_line_break(),
                    jsdoc,
                    token("Exported Ranges: "),
                    &export_ranges
                ])),
                token("}")
            ]
        )?;

        Ok(())
    }
}

impl Format<FormatTypeContext> for JsModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let exports = format_with(|f| {
            if self.exports.is_empty() {
                write!(f, [token("No exports")])
            } else {
                write!(f, [&self.exports])
            }
        });

        let static_imports = format_with(|f| {
            if self.static_imports.is_empty() {
                write!(f, [token("No imports")])
            } else {
                write!(f, [&self.static_imports])
            }
        });

        write!(
            f,
            [&format_args![
                token("Exports"),
                space(),
                token("{"),
                &group(&block_indent(&exports)),
                token("}"),
                hard_line_break(),
                token("Imports"),
                space(),
                token("{"),
                &group(&block_indent(&static_imports)),
                token("}"),
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
                    [text(
                        &std::format!("{:?}", export_name.text()),
                        TextSize::default()
                    ),]
                )
            });
            let arrow = format_with(|f| write!(f, [&format_args![space(), token("=>"), space()]]));

            let export = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        token("{"),
                        &group(&block_indent(&format_args![export]),),
                        token("}")
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
                    [text(
                        &std::format!("{:?}", import_name.text()),
                        TextSize::default()
                    ),]
                )
            });
            let arrow = format_with(|f| write!(f, [&format_args![space(), token("=>"), space()]]));

            let export = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        token("{"),
                        &group(&block_indent(&format_args![import]),),
                        token("}")
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
        write!(f, [token("Export")])?;
        match self {
            Self::Own(export) => {
                write!(
                    f,
                    [&format_args![
                        token("OwnExport"),
                        space(),
                        token("=>"),
                        space(),
                        &export
                    ]]
                )
            }
            Self::OwnType(own_type) => {
                write!(
                    f,
                    [&format_args![
                        token("OwnType"),
                        space(),
                        token("=>"),
                        space(),
                        &own_type
                    ]]
                )
            }
            Self::Reexport(reexport) => {
                write!(
                    f,
                    [&format_args![
                        token("Reexport"),
                        space(),
                        token("=>"),
                        space(),
                        &reexport
                    ]]
                )
            }
            Self::ReexportType(reexport_type) => {
                write!(
                    f,
                    [&format_args![
                        token("ReexportType"),
                        space(),
                        token("=>"),
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
                        token("JSDoc comment:"),
                        space(),
                        jsdoc,
                        token(","),
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
                    token("Name:"),
                    space(),
                    text(&self.name, TextSize::default()),
                    token(","),
                    hard_line_break(),
                    token("Type:"),
                    space(),
                    &self.ty,
                    token(","),
                    hard_line_break(),
                    jsdoc_comment,
                    token("Declaration kind:"),
                    space(),
                    text(
                        &std::format!("{:?}", self.declaration_kind),
                        TextSize::default()
                    ),
                ]]
            )
        });

        write!(
            f,
            [&format_args![
                token("JsBindingData {"),
                block_indent(&content),
                token("}")
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
                token("Reexport"),
                token("("),
                block_indent(&content),
                token(")")
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
            Self::Binding(binding_range) => {
                let range_str = std::format!("{:?}", binding_range);
                write!(
                    f,
                    [&format_args![
                        token("JsOwnExport::Binding("),
                        text(&range_str, TextSize::default()),
                        token(")")
                    ]]
                )
            }
            Self::Type(resolved_type_id) => write!(
                f,
                [&format_args![
                    token("JsOwnExport::Type("),
                    text(&std::format!("{resolved_type_id:?}"), TextSize::default()),
                    token(")")
                ]]
            ),
            Self::Namespace(reexport) => write!(
                f,
                [&format_args![
                    token("JsOwnExport::Namespace("),
                    reexport,
                    token(")")
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
                token("Specifier:"),
                space(),
                text(
                    &std::format!("{:?}", self.specifier.text()),
                    TextSize::default()
                ),
            ]]
        )?;
        write!(f, [hard_line_break()])?;

        write!(
            f,
            [&format_args![
                token("Resolved path:"),
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
#[derive(Clone)]
struct TypedRange(TextRange);

impl From<&TextRange> for TypedRange {
    fn from(value: &TextRange) -> Self {
        Self(*value)
    }
}

#[derive(Clone)]
struct TypedSize(TextSize);

impl From<TextSize> for TypedSize {
    fn from(value: TextSize) -> Self {
        Self(value)
    }
}

impl Format<FormatTypeContext> for TypedSize {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let value = std::format!("{}", self.0);
        write!(f, [text(&value, TextSize::default())])
    }
}

impl Format<FormatTypeContext> for TypedRange {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [
                token("("),
                TypedSize::from(self.0.start()),
                token(".."),
                TypedSize::from(self.0.end()),
                token(")")
            ]
        )
    }
}

// #region JsImportPhase

impl Format<FormatTypeContext> for JsImportPhase {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let label = match self {
            Self::Default => "Default",
            Self::Defer => "Defer",
            Self::Source => "Source",
            Self::Type => "Type",
        };
        write!(f, [token(label)])
    }
}

// #endregion

// #region JsImportPath

impl Format<FormatTypeContext> for JsImportPath {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let content = format_with(|f| {
            write!(
                f,
                [&format_args![
                    token("resolved_path:"),
                    space(),
                    self.resolved_path,
                    token(","),
                    hard_line_break(),
                    token("phase:"),
                    space(),
                    &self.phase,
                ]]
            )
        });
        write!(
            f,
            [&format_args![
                token("JsImportPath {"),
                block_indent(&content),
                token("}")
            ]]
        )
    }
}

// #endregion

// #region CssImport

impl Format<FormatTypeContext> for CssImport {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let content = format_with(|f| {
            write!(
                f,
                [&format_args![
                    token("specifier:"),
                    space(),
                    text(
                        &std::format!("{:?}", self.specifier.text()),
                        TextSize::default()
                    ),
                    token(","),
                    hard_line_break(),
                    token("resolved_path:"),
                    space(),
                    self.resolved_path,
                ]]
            )
        });
        write!(
            f,
            [&format_args![
                token("CssImport {"),
                block_indent(&content),
                token("}")
            ]]
        )
    }
}

// #endregion

// #region CssImports

impl Format<FormatTypeContext> for CssImports {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        if self.0.is_empty() {
            return write!(f, [token("No imports")]);
        }
        let mut joiner = f.join();
        for (specifier, import) in &self.0 {
            let entry = format_with(|f| {
                write!(
                    f,
                    [&format_args![
                        text(&std::format!("{:?}", specifier.text()), TextSize::default()),
                        space(),
                        token("=>"),
                        space(),
                        token("{"),
                        &group(&block_indent(&format_args![import])),
                        token("}"),
                        soft_line_break()
                    ]]
                )
            });
            joiner.entry(&group(&format_args![entry, soft_line_break_or_space()]));
        }
        joiner.finish()
    }
}

// #endregion

// #region CssClassDefinition

impl Format<FormatTypeContext> for CssClassDefinition {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text(self.name.text(), TextSize::default()),
                space(),
                token("("),
                text(
                    &std::format!("{:?}", self.applicability),
                    TextSize::default()
                ),
                token(")"),
            ]]
        )
    }
}

// #endregion

// #region CssClassReference

impl Format<FormatTypeContext> for CssClassReference {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(
            f,
            [&format_args![
                text(
                    &std::format!("{:?}", self.token.text()),
                    TextSize::default()
                ),
                space(),
                token("in"),
                space(),
                text(self.file_path.as_str(), TextSize::default()),
            ]]
        )
    }
}

// #endregion

// #region CssModuleInfo

impl std::fmt::Display for CssModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatTypeContext, [&**self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<FormatTypeContext> for CssModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(f, [&**self])
    }
}

impl Format<FormatTypeContext> for crate::css_module_info::CssModuleInfoInner {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let classes_section = format_with(|f| {
            let mut sorted: Vec<_> = self.classes.iter().map(|t| t.text().to_string()).collect();
            sorted.sort();
            if sorted.is_empty() {
                write!(f, [token("No classes")])
            } else {
                let separator = hard_line_break();
                let mut joiner = f.join_with(&separator);
                for class in &sorted {
                    let entry =
                        format_with(|f| write!(f, [text(class, TextSize::default()), token(",")]));
                    joiner.entry(&entry);
                }
                joiner.finish()
            }
        });

        let imports_section = format_with(|f| write!(f, [&self.imports]));

        write!(
            f,
            [&format_args![
                token("Classes"),
                space(),
                token("{"),
                &group(&block_indent(&classes_section)),
                token("}"),
                hard_line_break(),
                token("Imports"),
                space(),
                token("{"),
                &group(&block_indent(&imports_section)),
                token("}"),
            ]]
        )
    }
}

// #endregion

// #region HtmlModuleInfo

impl std::fmt::Display for HtmlModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatTypeContext, [&**self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<FormatTypeContext> for HtmlModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        write!(f, [&**self])
    }
}

impl Format<FormatTypeContext> for HtmlModuleInfoInner {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let style_classes_section = format_with(|f| {
            let mut sorted: Vec<_> = self
                .style_classes
                .iter()
                .map(|c| (c.name.text().to_string(), c.applicability))
                .collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            if sorted.is_empty() {
                write!(f, [token("No style classes")])
            } else {
                let separator = hard_line_break();
                let mut joiner = f.join_with(&separator);
                for (name, applicability) in &sorted {
                    let entry = format_with(|f| {
                        write!(
                            f,
                            [&format_args![
                                text(name, TextSize::default()),
                                space(),
                                token("("),
                                text(&std::format!("{applicability:?}"), TextSize::default()),
                                token(")"),
                                token(","),
                            ]]
                        )
                    });
                    joiner.entry(&entry);
                }
                joiner.finish()
            }
        });

        let ref_classes_section = format_with(|f| {
            let mut sorted: Vec<_> = self
                .referenced_classes
                .iter()
                .flat_map(|r| {
                    r.token
                        .text()
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                })
                .collect();
            sorted.sort();
            if sorted.is_empty() {
                write!(f, [token("No referenced classes")])
            } else {
                let separator = hard_line_break();
                let mut joiner = f.join_with(&separator);
                for class in &sorted {
                    let entry =
                        format_with(|f| write!(f, [text(class, TextSize::default()), token(",")]));
                    joiner.entry(&entry);
                }
                joiner.finish()
            }
        });

        let stylesheets_section = format_with(|f| {
            let mut sorted: Vec<_> = self
                .imported_stylesheets
                .iter()
                .map(|p| {
                    p.as_path().map_or("<unresolved>".to_string(), |p| {
                        p.as_str().replace('\\', "/")
                    })
                })
                .collect();
            sorted.sort();
            if sorted.is_empty() {
                write!(f, [token("No linked stylesheets")])
            } else {
                let separator = hard_line_break();
                let mut joiner = f.join_with(&separator);
                for path in &sorted {
                    let entry =
                        format_with(|f| write!(f, [text(path, TextSize::default()), token(",")]));
                    joiner.entry(&entry);
                }
                joiner.finish()
            }
        });

        let script_imports_section = format_with(|f| {
            let mut sorted: Vec<_> = self
                .static_import_paths
                .iter()
                .map(|(specifier, resolved)| {
                    let resolved_str = resolved.as_path().map_or("<unresolved>".to_string(), |p| {
                        p.as_str().replace('\\', "/")
                    });
                    (specifier.text().to_string(), resolved_str)
                })
                .collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            if sorted.is_empty() {
                write!(f, [token("No script imports")])
            } else {
                let separator = hard_line_break();
                let mut joiner = f.join_with(&separator);
                for (specifier, resolved) in &sorted {
                    let entry = format_with(|f| {
                        write!(
                            f,
                            [&format_args![
                                text(&std::format!("{specifier:?}"), TextSize::default()),
                                space(),
                                token("=>"),
                                space(),
                                text(resolved, TextSize::default()),
                                token(","),
                            ]]
                        )
                    });
                    joiner.entry(&entry);
                }
                joiner.finish()
            }
        });

        write!(
            f,
            [&format_args![
                token("StyleClasses"),
                space(),
                token("{"),
                &group(&block_indent(&style_classes_section)),
                token("}"),
                hard_line_break(),
                token("ReferencedClasses"),
                space(),
                token("{"),
                &group(&block_indent(&ref_classes_section)),
                token("}"),
                hard_line_break(),
                token("LinkedStylesheets"),
                space(),
                token("{"),
                &group(&block_indent(&stylesheets_section)),
                token("}"),
                hard_line_break(),
                token("ScriptImports"),
                space(),
                token("{"),
                &group(&block_indent(&script_imports_section)),
                token("}"),
            ]]
        )
    }
}

// #endregion

// #region ModuleInfo

impl std::fmt::Display for ModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatTypeContext, [self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<FormatTypeContext> for ModuleInfo {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        match self {
            Self::Js(js) => write!(f, [&format_args![token("Js"), token("("), js, token(")")]]),
            Self::Css(css) => write!(
                f,
                [&format_args![token("Css"), token("("), css, token(")")]]
            ),
            Self::Html(html) => write!(
                f,
                [&format_args![token("Html"), token("("), html, token(")")]]
            ),
        }
    }
}

// #endregion
