use crate::{
    prelude::*,
    utils::{formatters::FormatTokenAsLowercase, metadata::is_canonical_html_attribute},
};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{AnyHtmlTagName, HtmlAttributeName, HtmlAttributeNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeName {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}

pub(crate) struct FormatHtmlAttributeNameOptions {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}

impl FormatRuleWithOptions<HtmlAttributeName> for FormatHtmlAttributeName {
    type Options = FormatHtmlAttributeNameOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.is_canonical_html_element = options.is_canonical_html_element;
        self.tag_name = options.tag_name;
        self.compact = options.compact;
        self
    }
}
impl FormatNodeRule<HtmlAttributeName> for FormatHtmlAttributeName {
    fn fmt_fields(&self, node: &HtmlAttributeName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeNameFields { value_token } = node.as_fields();

        if self.compact {
            let value_token = value_token?;
            format_removed(&value_token).fmt(f)
        } else {
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
}
