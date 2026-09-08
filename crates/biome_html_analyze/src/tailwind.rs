use biome_html_factory::make;
use biome_html_syntax::{HtmlAttribute, HtmlLanguage};
use biome_rowan::{BatchMutation, TextRange};

/// Converts a range relative to the attribute string contents into a source range.
pub(crate) fn host_range(attribute: &HtmlAttribute, range: TextRange) -> Option<TextRange> {
    let start = attribute.html_string()?.inner_string_range().ok()?.start();
    Some(TextRange::new(start + range.start(), start + range.end()))
}

pub(crate) fn apply_fixed_class_string(
    mutation: &mut BatchMutation<HtmlLanguage>,
    attribute: &HtmlAttribute,
    fixed: &str,
) -> Option<()> {
    let html_string = attribute.html_string()?;
    let value_token = html_string.value_token().ok()?;
    let new_token = if value_token.text_trimmed().starts_with('\'') {
        make::html_string_literal_single_quotes(fixed)
    } else {
        make::html_string_literal(fixed)
    };
    mutation.replace_node(html_string, make::html_string(new_token));
    Some(())
}
