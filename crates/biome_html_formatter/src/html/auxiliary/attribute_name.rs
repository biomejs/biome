use crate::{
    prelude::*,
    utils::{formatters::FormatTokenAsLowercase, metadata::is_canonical_html_attribute},
};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlAttributeName, HtmlAttributeNameFields, HtmlTagName};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeName {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<HtmlTagName>,
}

pub(crate) struct FormatHtmlAttributeNameOptions {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<HtmlTagName>,
}

impl FormatRuleWithOptions<HtmlAttributeName> for FormatHtmlAttributeName {
    type Options = FormatHtmlAttributeNameOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.is_canonical_html_element = options.is_canonical_html_element;
        self.tag_name = options.tag_name;
        self
    }
}
impl FormatNodeRule<HtmlAttributeName> for FormatHtmlAttributeName {
    fn fmt_fields(&self, node: &HtmlAttributeName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeNameFields { value_token } = node.as_fields();

        let should_lowercase = self.is_canonical_html_element
            && if let Some(tag_name) = &self.tag_name {
                is_canonical_html_attribute(tag_name, node)
            } else {
                false
            };

        if should_lowercase {
            write!(f, [FormatTokenAsLowercase::from(value_token?)])
        } else {
            write!(f, [value_token.format()])
        }
    }
}
