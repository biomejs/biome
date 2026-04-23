use biome_html_syntax::{HtmlFileSource, element_ext::AnyHtmlTagElement};

// In HTML files: case-insensitive (TAG, Tag, tag all match)
// In component frameworks (Vue, Svelte, Astro): case-sensitive (only "tag" matches)
// This means <Tag> in Vue/Svelte is treated as a component and ignored
pub fn is_html_tag(element: &AnyHtmlTagElement, source_type: &HtmlFileSource, name: &str) -> bool {
    let Some(element_name) = element.tag_name() else {
        return false;
    };

    if source_type.is_html() {
        element_name.eq_ignore_ascii_case(name)
    } else {
        element_name.text() == name
    }
}
