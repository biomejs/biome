use crate::{html::auxiliary::attribute::FormatHtmlAttributeOptions, prelude::*};
use biome_formatter::{AttributePosition, FormatRuleWithOptions, write};
use biome_html_syntax::{AnyHtmlAttribute, AnyHtmlTagName, HtmlAttributeList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeList {
    /// Whether or not this attribute list belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute list belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether the attributes are printed on a single line, whatever its width.
    pub is_single_line: bool,
}

pub(crate) struct FormatHtmlAttributeListOptions {
    /// Whether this attribute list belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute list belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether the attributes are printed on a single line, whatever its width.
    pub is_single_line: bool,
}

impl FormatRuleWithOptions<HtmlAttributeList> for FormatHtmlAttributeList {
    type Options = FormatHtmlAttributeListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.is_canonical_html_element = options.is_canonical_html_element;
        self.tag_name = options.tag_name;
        self.is_single_line = options.is_single_line;
        self
    }
}

impl FormatRule<HtmlAttributeList> for FormatHtmlAttributeList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlAttributeList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let attribute_count = node.len();
        let is_multiline =
            f.options().attribute_position() == AttributePosition::Multiline && attribute_count > 1;
        let attribute_separator = format_with(|f| {
            if self.is_single_line {
                write!(f, [space()])
            } else if is_multiline {
                write!(f, [hard_line_break()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        if attribute_count > 0 {
            let attributes = format_with(|f| {
                f.join_with(&attribute_separator)
                    .entries(node.iter().map(|attribute| {
                        // Prettier normalizes the casing for attributes, but only for elements that are known to be canonical HTML elements, as in they are defined in any version of the HTML specification.
                        format_with(move |f| match &attribute {
                            AnyHtmlAttribute::HtmlAttribute(attr) => attr
                                .format()
                                .with_options(FormatHtmlAttributeOptions {
                                    is_canonical_html_element: self.is_canonical_html_element,
                                    tag_name: self.tag_name.clone(),
                                    compact: false,
                                })
                                .fmt(f),
                            AnyHtmlAttribute::HtmlAttributeDoubleTextExpression(attr) => {
                                attr.format().fmt(f)
                            }
                            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(attr) => {
                                attr.format().fmt(f)
                            }
                            AnyHtmlAttribute::HtmlBogusAttribute(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::SvelteAttachAttribute(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::AnyVueDirective(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::AnySvelteDirective(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::HtmlSpreadAttribute(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::AnyAstroDirective(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::AnyAngularBinding(attr) => attr.format().fmt(f),
                            AnyHtmlAttribute::AngularStructuralDirective(attr) => {
                                attr.format().fmt(f)
                            }
                            AnyHtmlAttribute::AngularTemplateRefVariable(attr) => {
                                attr.format().fmt(f)
                            }
                        })
                    }))
                    .finish()
            });
            if self.is_single_line {
                write!(f, [space(), attributes])?;
            } else {
                write!(f, [space(), soft_line_indent_or_space(&attributes)])?;
            }
        }

        Ok(())
    }
}
