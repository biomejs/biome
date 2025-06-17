use crate::{html::auxiliary::attribute_name::FormatHtmlAttributeNameOptions, prelude::*};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlAttribute, HtmlAttributeFields, HtmlTagName};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttribute {
    /// Whether or not this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<HtmlTagName>,
}

pub(crate) struct FormatHtmlAttributeOptions {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<HtmlTagName>,
}

impl FormatRuleWithOptions<HtmlAttribute> for FormatHtmlAttribute {
    type Options = FormatHtmlAttributeOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.is_canonical_html_element = options.is_canonical_html_element;
        self.tag_name = options.tag_name;
        self
    }
}

impl FormatNodeRule<HtmlAttribute> for FormatHtmlAttribute {
    fn fmt_fields(&self, node: &HtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeFields { name, initializer } = node.as_fields();

        write!(
            f,
            [
                name.format()?.with_options(FormatHtmlAttributeNameOptions {
                    is_canonical_html_element: self.is_canonical_html_element,
                    tag_name: self.tag_name.clone(),
                }),
                initializer.format()
            ]
        )
    }
}
