//! CSS display value mappings for HTML elements.
//!
//! This module provides CSS display value lookup for HTML elements based on the
//! browser's default user-agent stylesheet. This is crucial for determining
//! whitespace sensitivity during HTML formatting.
//!
//! The data is derived from:
//! - The `html-ua-styles` npm package used by Prettier
//! - HTML WHATWG spec: <https://html.spec.whatwg.org/multipage/rendering.html#the-css-user-agent-style-sheet-and-presentational-hints>

use crate::HtmlFormatter;
use crate::utils::metadata::{MATHML_ALL_TAGS, SVG_ALL_TAGS};
use biome_html_syntax::AnyHtmlTagName;
use biome_string_case::StrLikeExtension;

/// CSS display values that are relevant for HTML formatting decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CssDisplay {
    /// `display: block` - Element generates a block-level box.
    Block,
    /// `display: inline` - Element generates inline-level boxes.
    #[default]
    Inline,
    /// `display: inline-block` - Element generates inline-level block container.
    InlineBlock,
    /// `display: list-item` - Element generates a block box and a list-item marker box.
    ListItem,
    /// `display: table` - Element behaves like a table element.
    Table,
    /// `display: table-cell` - Element behaves like a table cell.
    TableCell,
    /// `display: table-row` - Element behaves like a table row.
    TableRow,
    /// `display: table-row-group` - Element behaves like a table row group (tbody).
    TableRowGroup,
    /// `display: table-header-group` - Element behaves like thead.
    TableHeaderGroup,
    /// `display: table-footer-group` - Element behaves like tfoot.
    TableFooterGroup,
    /// `display: table-column` - Element behaves like a col element.
    TableColumn,
    /// `display: table-column-group` - Element behaves like a colgroup element.
    TableColumnGroup,
    /// `display: table-caption` - Element behaves like a caption element.
    TableCaption,
    /// `display: ruby` - Element behaves like a ruby element.
    Ruby,
    /// `display: ruby-base` - Element behaves like rb.
    RubyBase,
    /// `display: ruby-text` - Element behaves like rt.
    RubyText,
    /// `display: none` - Element generates no boxes.
    None,
}

impl CssDisplay {
    /// Returns true if this display value creates a block-like formatting context.
    ///
    /// Block-like elements are NOT whitespace-sensitive in the CSS whitespace mode.
    /// This includes `block`, `list-item`, `table`, `table-row`, etc.
    ///
    /// **Note: For formatting purposes, you MUST use [`Self::is_internally_whitespace_sensitive`] or
    /// [`Self::is_externally_whitespace_sensitive`] to determine if an element is whitespace-sensitive.**
    pub fn is_block_like(self) -> bool {
        // FIXME: Prettier treats `display: none` as whitespace sensitive. So technically, this should not be included here.
        // However, including it here simplifies some logic elsewhere.
        matches!(
            self,
            Self::Block
                | Self::ListItem
                | Self::Table
                | Self::TableRow
                | Self::TableRowGroup
                | Self::TableHeaderGroup
                | Self::TableFooterGroup
                | Self::TableColumn
                | Self::TableColumnGroup
                | Self::TableCaption
                | Self::TableCell
                | Self::None
        )
    }

    /// Returns true if this display value creates an inline-like formatting context.
    ///
    /// Inline-like elements ARE whitespace-sensitive in the CSS whitespace mode.
    /// This includes `inline`, `inline-block`, `ruby`, etc.
    ///
    /// **Note: For formatting purposes, you MUST use [`Self::is_internally_whitespace_sensitive`] or
    /// [`Self::is_externally_whitespace_sensitive`] to determine if an element is whitespace-sensitive.**
    pub fn is_inline_like(self) -> bool {
        // TableCell is intentionally not included here, even though prettier considers it inline-like for formatting. This is to get formatting for `<tr>` correct.
        matches!(
            self,
            Self::Inline | Self::InlineBlock | Self::Ruby | Self::RubyBase | Self::RubyText
        )
    }

    pub fn is_inline_block(self) -> bool {
        matches!(self, Self::InlineBlock)
    }

    /// Returns true if this is a table-related display value.
    ///
    /// This is used for determining if children should be forced to multiline.
    /// Prettier forces line breaks between children of table-like elements
    /// (except table-cell).
    ///
    /// **Note: For formatting purposes, you MUST use [`Self::is_internally_whitespace_sensitive`] or
    /// [`Self::is_externally_whitespace_sensitive`] to determine if an element is whitespace-sensitive.**
    pub fn is_table_like(self) -> bool {
        matches!(
            self,
            Self::Table
                | Self::TableCell
                | Self::TableRow
                | Self::TableRowGroup
                | Self::TableHeaderGroup
                | Self::TableFooterGroup
                | Self::TableColumn
                | Self::TableColumnGroup
                | Self::TableCaption
        )
    }

    /// Whether elements with this display value are considered
    /// whitespace-sensitive on the inside (i.e., their children), respecting
    /// the formatter's whitespace sensitivity setting.
    ///
    /// - `Css`: Only inline-like elements (not block-like and not inline-block) are whitespace-sensitive.
    /// - `Strict`: All elements are whitespace-sensitive.
    /// - `Ignore`: No elements are whitespace-sensitive.
    pub fn is_internally_whitespace_sensitive(&self, f: &HtmlFormatter) -> bool {
        let sensitivity = f.options().whitespace_sensitivity();
        sensitivity.is_css() && !self.is_block_like() && *self != Self::InlineBlock
            || sensitivity.is_strict()
    }

    /// Whether elements with this display value are considered
    /// whitespace-sensitive on the outside (i.e., around the element, to siblings),
    /// respecting the formatter's whitespace sensitivity setting.
    ///
    /// - `Css`: Only inline-like elements are whitespace-sensitive.
    /// - `Strict`: All elements are whitespace-sensitive.
    /// - `Ignore`: No elements are whitespace-sensitive.
    pub fn is_externally_whitespace_sensitive(&self, f: &HtmlFormatter) -> bool {
        let sensitivity = f.options().whitespace_sensitivity();
        sensitivity.is_css() && self.is_inline_like() || sensitivity.is_strict()
    }
}

/// Gets the CSS display value for a given HTML tag name.
///
/// This returns the default display value from the browser's user-agent stylesheet.
/// For unknown elements, returns `CssDisplay::Inline` (the CSS default).
///
/// Data source: `html-ua-styles` npm package and HTML WHATWG spec.
/// Includes Prettier-specific adjustments for formatting purposes.
pub fn get_css_display(tag_name: &str) -> CssDisplay {
    // Use case-insensitive matching
    let tag_lower = tag_name.to_ascii_lowercase_cow();

    match tag_lower.as_ref() {
        // Block elements
        "html" | "body" | "address" | "blockquote" | "center" | "dialog" | "div" | "figure"
        | "figcaption" | "footer" | "form" | "header" | "hr" | "legend" | "listing" | "main"
        | "p" | "plaintext" | "pre" | "search" | "xmp" => CssDisplay::Block,

        // Sections and headings (block)
        "article" | "aside" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "hgroup" | "nav"
        | "section" => CssDisplay::Block,

        // Lists (block)
        "dir" | "dd" | "dl" | "dt" | "menu" | "ol" | "ul" => CssDisplay::Block,

        // List items
        "li" => CssDisplay::ListItem,

        // Details/summary (block)
        "details" | "summary" => CssDisplay::Block,

        // Table elements
        "table" => CssDisplay::Table,
        "caption" => CssDisplay::TableCaption,
        "colgroup" => CssDisplay::TableColumnGroup,
        "col" => CssDisplay::TableColumn,
        "thead" => CssDisplay::TableHeaderGroup,
        "tbody" => CssDisplay::TableRowGroup,
        "tfoot" => CssDisplay::TableFooterGroup,
        "tr" => CssDisplay::TableRow,
        "td" | "th" => CssDisplay::TableCell,

        // Ruby elements
        "ruby" => CssDisplay::Ruby,
        "rb" => CssDisplay::RubyBase,
        "rt" | "rtc" => CssDisplay::RubyText,
        "rp" => CssDisplay::None,

        // Hidden elements (display: none)
        "area" | "base" | "basefont" | "datalist" | "head" | "link" | "meta" | "noembed"
        | "noframes" | "script" | "style" | "title" | "noscript" => CssDisplay::None,

        // Media elements - these have special handling but are essentially block-like
        // for formatting purposes when considering children
        "audio" | "video" | "object" | "svg" => CssDisplay::InlineBlock,
        "param" => CssDisplay::Block,

        // Form elements - inline-block
        "button" | "textarea" | "input" | "select" | "meter" | "progress" => {
            CssDisplay::InlineBlock
        }

        // Replaced/embedded content (inline or inline-block depending on context)
        "img" | "embed" | "iframe" | "canvas" | "template" => CssDisplay::Inline,

        // Other inline elements
        "a" | "abbr" | "acronym" | "b" | "bdi" | "bdo" | "big" | "br" | "cite" | "code"
        | "data" | "del" | "dfn" | "em" | "font" | "i" | "ins" | "kbd" | "label" | "map"
        | "mark" | "nobr" | "output" | "picture" | "q" | "s" | "samp" | "slot" | "small"
        | "span" | "strike" | "strong" | "sub" | "sup" | "time" | "tt" | "u" | "var" | "wbr" => {
            CssDisplay::Inline
        }

        // Source and track are hidden, but prettier treats them as block for formatting purposes
        "source" | "track" => CssDisplay::Block,

        // Fieldset is block
        "fieldset" => CssDisplay::Block,

        // Option/optgroup have special handling in selects
        "option" | "optgroup" => CssDisplay::Block,

        // Unknown elements default to inline (CSS default behavior)
        other => {
            if SVG_ALL_TAGS.binary_search(&other).is_ok()
                || MATHML_ALL_TAGS.binary_search(&other).is_ok()
            {
                CssDisplay::Block
            } else {
                CssDisplay::Inline
            }
        }
    }
}

/// Gets the CSS display value from an [AnyHtmlTagName] syntax node.
pub fn get_css_display_from_tag(tag_name: &AnyHtmlTagName) -> CssDisplay {
    match tag_name {
        AnyHtmlTagName::HtmlComponentName(_) => CssDisplay::Inline,
        AnyHtmlTagName::HtmlMemberName(_) => CssDisplay::Inline,
        AnyHtmlTagName::HtmlTagName(tag_name) => {
            let Ok(token) = tag_name.value_token() else {
                return CssDisplay::Inline;
            };
            get_css_display(token.text_trimmed())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_elements() {
        let block_tags = [
            "div", "p", "h1", "ul", "ol", "section", "article", "header", "footer",
        ];
        for tag in block_tags {
            assert!(
                get_css_display(tag).is_block_like(),
                "Expected '{tag}' to be block-like"
            );
        }
    }

    #[test]
    fn test_inline_elements() {
        let inline_tags = ["span", "a", "strong", "em", "b", "i", "code", "label"];
        for tag in inline_tags {
            assert!(
                get_css_display(tag).is_inline_like(),
                "Expected '{tag}' to be inline-like"
            );
        }
    }

    #[test]
    fn test_tr_is_table_like() {
        assert!(get_css_display("tr").is_table_like());
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(get_css_display("DIV"), CssDisplay::Block);
        assert_eq!(get_css_display("Span"), CssDisplay::Inline);
        assert_eq!(get_css_display("TD"), CssDisplay::TableCell);
    }

    #[test]
    fn test_hidden_elements() {
        let hidden_tags = ["head", "script", "style", "meta", "link"];
        for tag in hidden_tags {
            assert_eq!(
                get_css_display(tag),
                CssDisplay::None,
                "Expected '{tag}' to be display: none"
            );
        }
    }

    #[test]
    fn test_form_elements_are_inline_block() {
        let form_tags = ["button", "input", "select", "textarea"];
        for tag in form_tags {
            assert_eq!(
                get_css_display(tag),
                CssDisplay::InlineBlock,
                "Expected '{tag}' to be inline-block"
            );
        }
    }

    #[test]
    fn test_unknown_elements_default_to_inline() {
        assert_eq!(get_css_display("custom-element"), CssDisplay::Inline);
        assert_eq!(get_css_display("my-component"), CssDisplay::Inline);
    }

    #[test]
    fn test_list_item() {
        assert_eq!(get_css_display("li"), CssDisplay::ListItem);
        // ListItem is block-like
        assert!(CssDisplay::ListItem.is_block_like());
    }
}
