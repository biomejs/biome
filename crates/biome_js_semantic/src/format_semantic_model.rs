use biome_formatter::prelude::*;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TransformSourceMap,
};
use biome_formatter::{format_args, write};
use biome_js_syntax::TextSize;

use crate::{Binding, BindingId, Scope, ScopeId, SemanticModel};

struct FormatSemanticModelOptions;

impl FormatOptions for FormatSemanticModelOptions {
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

struct FormatSemanticModelContext;

impl FormatContext for FormatSemanticModelContext {
    type Options = FormatSemanticModelOptions;

    fn options(&self) -> &Self::Options {
        &FormatSemanticModelOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl std::fmt::Display for SemanticModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatSemanticModelContext, [&self])
            .expect("Formatting not to throw any FormatErrors");
        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

impl Format<FormatSemanticModelContext> for SemanticModel {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        write!(f, [self.global_scope()])
    }
}

enum ScopeOrBinding {
    Scope(Scope),
    Binding(Binding),
}

impl Format<FormatSemanticModelContext> for ScopeOrBinding {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        match self {
            Self::Scope(scope) => scope.fmt(f),
            Self::Binding(binding) => binding.fmt(f),
        }
    }
}

impl Format<FormatSemanticModelContext> for Scope {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let mut children = self
            .children()
            .map(ScopeOrBinding::Scope)
            .chain(self.bindings().map(ScopeOrBinding::Binding))
            .collect::<Vec<_>>();
        // Yes it would be more efficient to just build the vector in the right order, but
        // this is easier to read and maintain.
        children.sort_by_key(|item| match item {
            ScopeOrBinding::Scope(scope) => scope.range().start(),
            ScopeOrBinding::Binding(binding) => binding.syntax().text_range_with_trivia().start(),
        });

        let formatted_scope_info = format_with(|f| {
            let range = std::format!("{:?}", self.range());
            write!(
                f,
                [
                    text("id: "),
                    self.id,
                    text(" @ "),
                    dynamic_text(range.as_str(), TextSize::default()),
                    hard_line_break()
                ]
            )?;

            Ok(())
        });

        let formatted_children = format_with(|f| f.join().entries(&children).finish());

        write!(
            f,
            [
                text("Scope {"),
                group(&block_indent(&format_args![
                    formatted_scope_info,
                    text("children: ["),
                    group(&block_indent(&formatted_children)),
                    text("]"),
                ])),
                text("}"),
                hard_line_break(),
            ]
        )?;
        Ok(())
    }
}

impl Format<FormatSemanticModelContext> for Binding {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let formatted_binding_info = format_with(|f| {
            let range = std::format!("{:?}", self.syntax().text_trimmed_range());
            write!(
                f,
                [
                    text("id: "),
                    self.id,
                    text(" @ "),
                    dynamic_text(range.as_str(), TextSize::default()),
                    hard_line_break()
                ]
            )?;
            write!(f, [text("scope: "), self.scope().id, hard_line_break()])?;
            let full_text = self.syntax().text_trimmed().into_text();
            write!(
                f,
                [
                    text("ident: "),
                    dynamic_text(&full_text, TextSize::default()),
                    hard_line_break()
                ]
            )?;
            Ok(())
        });
        write!(
            f,
            [
                text("Binding {"),
                group(&block_indent(&formatted_binding_info)),
                text("}"),
                hard_line_break(),
            ]
        )?;

        Ok(())
    }
}

impl Format<FormatSemanticModelContext> for ScopeId {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let text = std::format!("ScopeId({})", self.0);
        write!(f, [dynamic_text(text.as_str(), TextSize::default())])
    }
}

impl Format<FormatSemanticModelContext> for BindingId {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let text = std::format!("BindingId({})", self.0);
        write!(f, [dynamic_text(text.as_str(), TextSize::default())])
    }
}
