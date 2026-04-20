use crate::AsFormat;
use crate::context::HtmlFormatContext;
use biome_formatter::formatter::Formatter;
use biome_formatter::{Format, FormatResult};
use biome_html_syntax::{AnyHtmlAttributeInitializer, AnySvelteBindingProperty};

pub(crate) struct FmtAnyAttributeInitializer {
    pub(crate) node: AnyHtmlAttributeInitializer,
    pub(crate) compact: bool,
}

impl Format<HtmlFormatContext> for FmtAnyAttributeInitializer {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match &self.node {
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
            AnyHtmlAttributeInitializer::HtmlString(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
        }
    }
}

pub(crate) struct FmtAnySvelteBindingProperty {
    pub(crate) node: AnySvelteBindingProperty,
    pub(crate) compact: bool,
}

impl Format<HtmlFormatContext> for FmtAnySvelteBindingProperty {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match &self.node {
            AnySvelteBindingProperty::SvelteLiteral(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
            AnySvelteBindingProperty::SvelteMemberProperty(node) => node.format().fmt(f),
            AnySvelteBindingProperty::SvelteName(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
        }
    }
}
