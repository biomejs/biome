use crate::model::{Selector, SemanticModel, Specificity};
use biome_css_syntax::AnyCssRoot;
use biome_formatter::prelude::*;
use biome_formatter::write;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    SourceMapGeneration, TrailingNewline, TransformSourceMap,
};
use biome_rowan::{AstNode, TextSize};

#[derive(Debug, Default)]
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

    fn trailing_newline(&self) -> TrailingNewline {
        TrailingNewline::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            indent_width: self.indent_width(),
            print_width: self.line_width().into(),
            line_ending: self.line_ending(),
            indent_style: self.indent_style(),
            source_map_generation: SourceMapGeneration::default(),
        }
    }
}

#[derive(Debug, Default)]
struct FormatSemanticModelContext {
    options: FormatSemanticModelOptions,
}

impl FormatContext for FormatSemanticModelContext {
    type Options = FormatSemanticModelOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl std::fmt::Display for SemanticModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = biome_formatter::format!(FormatSemanticModelContext::default(), [&self])
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
        let mut selectors: Vec<&Selector> = self
            .data
            .rules_by_id
            .values()
            .flat_map(|rule| rule.selectors())
            .collect();
        selectors.sort_by_key(|sel| sel.range(&self.root()).start());

        let mut builder = f.join_nodes_with_hardline();
        for selector in selectors {
            builder.entry(
                selector.node(&self.root()).syntax(),
                &SelectorWithRoot(selector, &self.root()),
            );
        }
        builder.finish()
    }
}

struct SelectorWithRoot<'a>(&'a Selector, &'a AnyCssRoot);

impl<'a> Format<FormatSemanticModelContext> for SelectorWithRoot<'a> {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let selector = self.0;
        let root = self.1;
        let range = std::format!("{:?}", selector.range(root));
        write!(
            f,
            [
                text(
                    selector.text(root).into_text().text(),
                    selector.range(root).start()
                ),
                token(":"),
                space(),
                &selector.specificity(),
                space(),
                token(" @ "),
                text(range.as_str(), TextSize::default()),
            ]
        )
    }
}

impl Format<FormatSemanticModelContext> for Specificity {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        write!(
            f,
            [
                token("("),
                text(self.0.to_string().as_str(), TextSize::default()),
                token(","),
                space(),
                text(self.1.to_string().as_str(), TextSize::default()),
                token(","),
                space(),
                text(self.2.to_string().as_str(), TextSize::default()),
                token(")")
            ]
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::semantic_model;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::CssFileSource;

    #[ignore]
    #[test]
    fn print_semantic_model() {
        let source = r#"div {


  & > p {

  }

  @media (orientation: portrait) {
    & > p {

    }
  }
}"#;

        let parsed = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let model = semantic_model(&parsed.tree());
        eprintln!("{}", model);
    }
}
