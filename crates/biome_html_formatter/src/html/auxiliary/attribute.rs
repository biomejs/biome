use crate::html::auxiliary::attribute_initializer_clause::CompactKind;
use crate::{
    html::auxiliary::{
        attribute_initializer_clause::FormatHtmlAttributeInitializerClauseOptions,
        attribute_name::FormatHtmlAttributeNameOptions,
    },
    prelude::*,
};
use biome_formatter::{FormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlTagName, HtmlAttribute, HtmlAttributeFields,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttribute {
    /// Whether or not this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}

pub(crate) struct FormatHtmlAttributeOptions {
    /// Whether this attribute belongs to a canonical tag.
    pub is_canonical_html_element: bool,

    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<AnyHtmlTagName>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}

impl FormatRuleWithOptions<HtmlAttribute> for FormatHtmlAttribute {
    type Options = FormatHtmlAttributeOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.is_canonical_html_element = options.is_canonical_html_element;
        self.tag_name = options.tag_name;
        self.compact = options.compact;
        self
    }
}

/// Whether the formatter can write `x={x}` in `{x}`
/// The initializer must be a text expression, and the two values must match.
fn can_compact(node: &HtmlAttribute, f: &mut HtmlFormatter) -> bool {
    if f.context().options().file_source().is_svelte() {
        let name = node.name().ok().and_then(|name| name.token_text_trimmed());

        let initializer_value = node.initializer().and_then(|init| init.value().ok());

        let Some(AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(initializer_value)) =
            initializer_value
        else {
            return false;
        };

        let initializer_value = initializer_value
            .expression()
            .ok()
            .and_then(|expression| expression.string_value());

        if let (Some(name), Some(initializer_value)) = (name, initializer_value) {
            return initializer_value.text() == name.text();
        }
    }

    false
}

impl FormatNodeRule<HtmlAttribute> for FormatHtmlAttribute {
    fn fmt_fields(&self, node: &HtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeFields { name, initializer } = node.as_fields();

        if self.compact {
            let name = name.clone()?;
            write!(
                f,
                [name.format().with_options(FormatHtmlAttributeNameOptions {
                    compact: true,
                    tag_name: None,
                    is_canonical_html_element: self.is_canonical_html_element,
                })]
            )?;
            if let Some(initializer) = initializer {
                write!(
                    f,
                    [initializer.format().with_options(
                        FormatHtmlAttributeInitializerClauseOptions {
                            compact: CompactKind::Remove,
                            tag_name: None,
                            attribute_name: None,
                        }
                    )]
                )?;
            }
            Ok(())
        } else if can_compact(node, f) {
            let name = name.clone()?;
            write!(
                f,
                [name.format().with_options(FormatHtmlAttributeNameOptions {
                    compact: true,
                    tag_name: None,
                    is_canonical_html_element: self.is_canonical_html_element,
                })]
            )?;
            if let Some(initializer) = initializer.as_ref() {
                write!(
                    f,
                    [initializer.format().with_options(
                        FormatHtmlAttributeInitializerClauseOptions {
                            tag_name: self
                                .tag_name
                                .as_ref()
                                .and_then(|name| name.token_text_trimmed()),
                            attribute_name: name.token_text_trimmed(),
                            compact: CompactKind::Curly
                        }
                    )]
                )?;
            }
            Ok(())
        } else {
            write!(
                f,
                [name.format()?.with_options(FormatHtmlAttributeNameOptions {
                    is_canonical_html_element: self.is_canonical_html_element,
                    tag_name: self.tag_name.clone(),
                    compact: false
                }),]
            )?;

            if let Some(initializer) = initializer.as_ref() {
                write!(
                    f,
                    [initializer.format().with_options(
                        FormatHtmlAttributeInitializerClauseOptions {
                            tag_name: self
                                .tag_name
                                .as_ref()
                                .and_then(|name| name.token_text_trimmed()),
                            attribute_name: name.ok().and_then(|name| name.token_text_trimmed()),
                            compact: CompactKind::None
                        }
                    )]
                )?;
            }

            Ok(())
        }
    }
}
