use biome_html_factory::make;
use biome_html_syntax::{HtmlAttribute, HtmlLanguage, HtmlString, HtmlSyntaxKind, HtmlSyntaxToken};
use biome_rowan::{BatchMutation, TextRange, TextSize, TokenText};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;

pub(crate) trait TailwindClassStringOptions {
    fn has_attribute(&self, name: &str) -> bool;
}

impl TailwindClassStringOptions for UseTailwindShorthandClassesOptions {
    fn has_attribute(&self, name: &str) -> bool {
        self.has_attribute(name)
    }
}

pub(crate) fn class_string(
    attribute: &HtmlAttribute,
    options: &impl TailwindClassStringOptions,
) -> Option<TokenText> {
    if !options.has_attribute(attribute_name(attribute)?.text_trimmed()) {
        return None;
    }

    html_string(attribute)?.inner_string_text().ok()
}

pub(crate) fn html_string(attribute: &HtmlAttribute) -> Option<HtmlString> {
    attribute
        .initializer()?
        .value()
        .ok()?
        .as_html_string()
        .cloned()
}

pub(crate) fn host_range(attribute: &HtmlAttribute, range: TextRange) -> Option<TextRange> {
    let start = html_string(attribute)?
        .value_token()
        .ok()?
        .text_trimmed_range()
        .start()
        + TextSize::from(1);
    Some(TextRange::new(start + range.start(), start + range.end()))
}

pub(crate) fn apply_fixed_class_string(
    mutation: &mut BatchMutation<HtmlLanguage>,
    attribute: &HtmlAttribute,
    fixed: &str,
) -> Option<()> {
    let html_string = html_string(attribute)?;
    let value_token = html_string.value_token().ok()?;
    let new_token = if value_token.text_trimmed().starts_with('\'') {
        html_string_literal_single_quotes(fixed)
    } else {
        make::html_string_literal(fixed)
    };
    mutation.replace_node(html_string, make::html_string(new_token));
    Some(())
}

fn attribute_name(attribute: &HtmlAttribute) -> Option<biome_html_syntax::HtmlSyntaxToken> {
    attribute.name().ok()?.value_token().ok()
}

fn html_string_literal_single_quotes(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(
        HtmlSyntaxKind::HTML_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}
