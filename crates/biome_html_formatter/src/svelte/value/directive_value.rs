use crate::html::auxiliary::attribute_initializer_clause::{
    CompactKind, FormatHtmlAttributeInitializerClauseOptions,
};
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnySvelteBindingProperty, SvelteDirectiveValue, SvelteDirectiveValueFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveValue {
    compact: bool,
}
impl FormatNodeRule<SvelteDirectiveValue> for FormatSvelteDirectiveValue {
    fn fmt_fields(&self, node: &SvelteDirectiveValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        if !self.fmt_compact(node, f)? {
            let SvelteDirectiveValueFields {
                property,
                colon_token,
                modifiers,
                initializer,
            } = node.as_fields();

            write!(
                f,
                [colon_token.format(), property.format(), modifiers.format()]
            )?;

            if let Some(initializer) = initializer {
                write!(f, [initializer.format()])?;
            }
        }

        Ok(())
    }
}

impl FormatSvelteDirectiveValue {
    fn fmt_compact(
        &self,
        node: &SvelteDirectiveValue,
        f: &mut HtmlFormatter,
    ) -> FormatResult<bool> {
        let SvelteDirectiveValueFields {
            property,
            colon_token,
            modifiers,
            initializer,
        } = node.as_fields();
        if self.compact {
            let property = property.clone()?;
            let binding_value = match &property {
                AnySvelteBindingProperty::SvelteLiteral(literal) => literal.value_token(),
                AnySvelteBindingProperty::SvelteName(name) => name.ident_token(),
            }?;

            let Some(initializer) = initializer.clone() else {
                return Ok(false);
            };
            let Some(initializer_value) = initializer.value().ok().and_then(|v| v.string_value())
            else {
                return Ok(false);
            };

            if initializer_value.text() != binding_value.text_trimmed() {
                return Ok(false);
            }

            write!(
                f,
                [
                    colon_token.format(),
                    text(initializer_value.text(), initializer.range().start()),
                    initializer.format().with_options(
                        FormatHtmlAttributeInitializerClauseOptions {
                            compact: CompactKind::Remove,
                            attribute_name: None,
                            tag_name: None
                        }
                    ),
                    property.format().with_options(true),
                    modifiers.format().with_options(true)
                ]
            )?;

            return Ok(true);
        }

        Ok(false)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveValueOptions {
    pub(crate) compact: bool,
}

impl FormatRuleWithOptions<SvelteDirectiveValue> for FormatSvelteDirectiveValue {
    type Options = FormatSvelteDirectiveValueOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options.compact;
        self
    }
}
