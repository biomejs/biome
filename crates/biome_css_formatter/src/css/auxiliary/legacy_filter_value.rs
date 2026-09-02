use crate::prelude::*;
use crate::utils::custom_property::has_source_gap;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, CssCustomPropertyBracedBlock, CssCustomPropertyBracketedBlock,
    CssCustomPropertyComponentList, CssCustomPropertyFunction, CssCustomPropertyParenthesizedBlock,
    CssLegacyFilterValue, CssLegacyFilterValueFields,
};
use biome_formatter::write;
use biome_rowan::{AstNode, AstNodeList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLegacyFilterValue;
impl FormatNodeRule<CssLegacyFilterValue> for FormatCssLegacyFilterValue {
    fn fmt_fields(&self, node: &CssLegacyFilterValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLegacyFilterValueFields { components } = node.as_fields();
        FormatLegacyFilterComponents(&components).fmt(f)
    }
}

struct FormatLegacyFilterComponents<'a>(&'a CssCustomPropertyComponentList);

impl Format<CssFormatContext> for FormatLegacyFilterComponents<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.0.is_empty() {
            return format_dangling_comments(self.0.syntax()).fmt(f);
        }

        let mut previous = None;

        for component in self.0.iter() {
            let first = component.syntax().first_token();
            if let (Some(previous), Some(first)) = (previous.as_ref(), first.as_ref()) {
                if first.has_leading_newline() {
                    write!(f, [hard_line_break()])?;
                } else if has_source_gap(previous, first) {
                    write!(f, [space()])?;
                }
            }

            FormatLegacyFilterComponent(&component).fmt(f)?;
            previous = component.syntax().last_token();
        }

        Ok(())
    }
}

struct FormatLegacyFilterComponent<'a>(&'a AnyCssCustomPropertyComponent);

impl Format<CssFormatContext> for FormatLegacyFilterComponent<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if f.comments().is_suppressed(self.0.syntax()) {
            return self.0.format().fmt(f);
        }

        match self.0 {
            AnyCssCustomPropertyComponent::AnyCssDimension(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomPropertyBracedBlock(node) => {
                FormatLegacyFilterBracedBlock(node).fmt(f)
            }
            AnyCssCustomPropertyComponent::CssCustomPropertyBracketedBlock(node) => {
                FormatLegacyFilterBracketedBlock(node).fmt(f)
            }
            AnyCssCustomPropertyComponent::CssCustomPropertyDelimiter(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssCustomPropertyFunction(node) => {
                FormatLegacyFilterFunction(node).fmt(f)
            }
            AnyCssCustomPropertyComponent::CssCustomPropertyParenthesizedBlock(node) => {
                FormatLegacyFilterParenthesizedBlock(node).fmt(f)
            }
            AnyCssCustomPropertyComponent::CssNumber(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::CssString(node) => FormatLegacyFilterString(node).fmt(f),
            AnyCssCustomPropertyComponent::ScssInterpolatedString(node) => node.format().fmt(f),
            AnyCssCustomPropertyComponent::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}

struct FormatLegacyFilterFunction<'a>(&'a CssCustomPropertyFunction);

impl Format<CssFormatContext> for FormatLegacyFilterFunction<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let fields = self.0.as_fields();

        write!(
            f,
            [
                fields.name?.format(),
                fields.l_paren_token?.format(),
                FormatLegacyFilterComponents(&fields.components),
                fields.r_paren_token?.format(),
            ]
        )
    }
}

struct FormatLegacyFilterString<'a>(&'a biome_css_syntax::CssString);

impl Format<CssFormatContext> for FormatLegacyFilterString<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if f.comments().is_suppressed(self.0.syntax()) {
            return self.0.format().fmt(f);
        }

        write!(
            f,
            [
                format_leading_comments(self.0.syntax()),
                self.0.value_token()?.format(),
                format_dangling_comments(self.0.syntax()),
                format_trailing_comments(self.0.syntax()),
            ]
        )
    }
}

struct FormatLegacyFilterParenthesizedBlock<'a>(&'a CssCustomPropertyParenthesizedBlock);

impl Format<CssFormatContext> for FormatLegacyFilterParenthesizedBlock<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let fields = self.0.as_fields();

        write!(
            f,
            [
                fields.l_paren_token?.format(),
                FormatLegacyFilterComponents(&fields.components),
                fields.r_paren_token?.format(),
            ]
        )
    }
}

struct FormatLegacyFilterBracketedBlock<'a>(&'a CssCustomPropertyBracketedBlock);

impl Format<CssFormatContext> for FormatLegacyFilterBracketedBlock<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let fields = self.0.as_fields();

        write!(
            f,
            [
                fields.l_brack_token?.format(),
                FormatLegacyFilterComponents(&fields.components),
                fields.r_brack_token?.format(),
            ]
        )
    }
}

struct FormatLegacyFilterBracedBlock<'a>(&'a CssCustomPropertyBracedBlock);

impl Format<CssFormatContext> for FormatLegacyFilterBracedBlock<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let fields = self.0.as_fields();

        write!(
            f,
            [
                fields.l_curly_token?.format(),
                FormatLegacyFilterComponents(&fields.components),
                fields.r_curly_token?.format(),
            ]
        )
    }
}
