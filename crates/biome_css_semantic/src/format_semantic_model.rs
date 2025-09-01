use crate::model::{Rule, Selector, SemanticModel, Specificity};
use biome_formatter::prelude::*;
use biome_formatter::write;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    TransformSourceMap,
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

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            indent_width: self.indent_width(),
            print_width: self.line_width().into(),
            line_ending: self.line_ending(),
            indent_style: self.indent_style(),
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
        selectors.sort_by_key(|sel| sel.range().start());

        let mut builder = f.join_nodes_with_hardline();
        for selector in selectors {
            builder.entry(selector.node().syntax(), selector);
        }
        builder.finish()
    }
}

impl Format<FormatSemanticModelContext> for Rule {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        write!(
            f,
            [
                dynamic_text(
                    self.node().syntax().text_trimmed().into_text().text(),
                    self.node().syntax().text_trimmed_range().start()
                ),
                text(":"),
                space(),
                &self.specificity(),
            ]
        )
    }
}

impl Format<FormatSemanticModelContext> for Selector {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        let range = std::format!("{:?}", self.range());
        write!(
            f,
            [
                dynamic_text(self.text().into_text().text(), self.range().start()),
                text(":"),
                space(),
                &self.specificity(),
                space(),
                text(" @ "),
                dynamic_text(range.as_str(), TextSize::default()),
            ]
        )
    }
}

impl Format<FormatSemanticModelContext> for Specificity {
    fn fmt(&self, f: &mut Formatter<FormatSemanticModelContext>) -> FormatResult<()> {
        write!(
            f,
            [
                text("("),
                dynamic_text(self.0.to_string().as_str(), TextSize::default()),
                text(","),
                space(),
                dynamic_text(self.1.to_string().as_str(), TextSize::default()),
                text(","),
                space(),
                dynamic_text(self.2.to_string().as_str(), TextSize::default()),
                text(")")
            ]
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::semantic_model;
    use biome_css_parser::{CssParserOptions, parse_css};

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

        let parsed = parse_css(source, CssParserOptions::default());
        let model = semantic_model(&parsed.tree());
        eprintln!("{}", model);
    }
}
