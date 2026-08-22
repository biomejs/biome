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
use crate::utils::metadata::MATHML_ALL_TAGS;
use biome_html_syntax::{
    AnyHtmlTagName,
    HtmlSyntaxKind::{self, *},
};
use biome_parser::{TokenSet, token_set};
use biome_string_case::StrLikeExtension;

const BLOCK_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    HTML_KW,
    BODY_KW,
    ADDRESS_KW,
    BLOCKQUOTE_KW,
    CENTER_KW,
    DIALOG_KW,
    DIV_KW,
    FIGURE_KW,
    FIGCAPTION_KW,
    FOOTER_KW,
    FORM_KW,
    HEADER_KW,
    HR_KW,
    LEGEND_KW,
    MAIN_KW,
    P_KW,
    PLAINTEXT_KW,
    PRE_KW,
    SEARCH_KW,
    XMP_KW,
    ARTICLE_KW,
    ASIDE_KW,
    H1_KW,
    H2_KW,
    H3_KW,
    H4_KW,
    H5_KW,
    H6_KW,
    HGROUP_KW,
    NAV_KW,
    SECTION_KW,
    DIR_KW,
    DD_KW,
    DL_KW,
    DT_KW,
    MENU_KW,
    OL_KW,
    UL_KW,
    DETAILS_KW,
    SUMMARY_KW,
    PARAM_KW,
    SOURCE_KW,
    TRACK_KW,
    FIELDSET_KW,
    OPTION_KW,
    OPTGROUP_KW,
    ANIMATE_KW,
    ANIMATE_MOTION_KW,
    ANIMATE_TRANSFORM_KW,
    CIRCLE_KW,
    CLIP_PATH_KW,
    DEFS_KW,
    DESC_KW,
    ELLIPSE_KW,
    FE_BLEND_KW,
    FE_COLOR_MATRIX_KW,
    FE_COMPONENT_TRANSFER_KW,
    FE_COMPOSITE_KW,
    FE_CONVOLVE_MATRIX_KW,
    FE_DIFFUSE_LIGHTING_KW,
    FE_DISPLACEMENT_MAP_KW,
    FE_DISTANT_LIGHT_KW,
    FE_DROP_SHADOW_KW,
    FE_FLOOD_KW,
    FE_FUNC_A_KW,
    FE_FUNC_B_KW,
    FE_FUNC_G_KW,
    FE_FUNC_R_KW,
    FE_GAUSSIAN_BLUR_KW,
    FE_IMAGE_KW,
    FE_MERGE_KW,
    FE_MERGE_NODE_KW,
    FE_MORPHOLOGY_KW,
    FE_OFFSET_KW,
    FE_POINT_LIGHT_KW,
    FE_SPECULAR_LIGHTING_KW,
    FE_SPOT_LIGHT_KW,
    FE_TILE_KW,
    FE_TURBULENCE_KW,
    FILTER_KW,
    FOREIGN_OBJECT_KW,
    G_KW,
    IMAGE_KW,
    LINE_KW,
    LINEAR_GRADIENT_KW,
    MARKER_KW,
    MASK_KW,
    METADATA_KW,
    MPATH_KW,
    PATH_KW,
    PATTERN_KW,
    POLYGON_KW,
    POLYLINE_KW,
    RADIAL_GRADIENT_KW,
    RECT_KW,
    SET_KW,
    STOP_KW,
    SWITCH_KW,
    SYMBOL_KW,
    TEXT_KW,
    TEXT_PATH_KW,
    TSPAN_KW,
    USE_KW,
    VIEW_KW
);

const INLINE_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    IMG_KW,
    EMBED_KW,
    IFRAME_KW,
    CANVAS_KW,
    TEMPLATE_KW,
    A_KW,
    ABBR_KW,
    ACRONYM_KW,
    B_KW,
    BDI_KW,
    BDO_KW,
    BIG_KW,
    BR_KW,
    CITE_KW,
    CODE_KW,
    DATA_KW,
    DEL_KW,
    DFN_KW,
    EM_KW,
    FONT_KW,
    I_KW,
    INS_KW,
    KBD_KW,
    LABEL_KW,
    MAP_KW,
    MARK_KW,
    NOBR_KW,
    OUTPUT_KW,
    PICTURE_KW,
    Q_KW,
    S_KW,
    SAMP_KW,
    SLOT_KW,
    SMALL_KW,
    SPAN_KW,
    STRIKE_KW,
    STRONG_KW,
    SUB_KW,
    SUP_KW,
    TIME_KW,
    TT_KW,
    U_KW,
    VAR_KW,
    WBR_KW,
    AUDIO_KW,
    BGSOUND_KW,
    BLINK_KW,
    COMPONENT_KW,
    FRAME_KW,
    FRAMESET_KW,
    KEYGEN_KW,
    MENUITEM_KW,
    NOSCRIPT_KW,
    OBJECT_KW,
    VIDEO_KW
);

const HIDDEN_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    RP_KW,
    AREA_KW,
    BASE_KW,
    BASEFONT_KW,
    DATALIST_KW,
    HEAD_KW,
    LINK_KW,
    META_KW,
    NOEMBED_KW,
    NOFRAMES_KW,
    SCRIPT_KW,
    STYLE_KW,
    TITLE_KW
);

const INLINE_BLOCK_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    SVG_KW,
    BUTTON_KW,
    TEXTAREA_KW,
    INPUT_KW,
    SELECT_KW,
    METER_KW,
    PROGRESS_KW,
    MARQUEE_KW
);

const TABLE_CELL_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(TD_KW, TH_KW);

const RUBY_TEXT_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(RT_KW, RTC_KW);

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

fn get_unknown_css_display(tag_name: &str) -> CssDisplay {
    let tag_name = tag_name.to_ascii_lowercase_cow();
    if MATHML_ALL_TAGS.binary_search(&tag_name.as_ref()).is_ok() {
        CssDisplay::Block
    } else {
        CssDisplay::Inline
    }
}

fn get_css_display(kind: HtmlSyntaxKind, tag_name: &str) -> CssDisplay {
    if INLINE_ELEMENTS.contains(kind) {
        CssDisplay::Inline
    } else if BLOCK_ELEMENTS.contains(kind) {
        CssDisplay::Block
    } else if HIDDEN_ELEMENTS.contains(kind) {
        CssDisplay::None
    } else if INLINE_BLOCK_ELEMENTS.contains(kind) {
        CssDisplay::InlineBlock
    } else if TABLE_CELL_ELEMENTS.contains(kind) {
        CssDisplay::TableCell
    } else if RUBY_TEXT_ELEMENTS.contains(kind) {
        CssDisplay::RubyText
    } else {
        match kind {
            LI_KW => CssDisplay::ListItem,
            TABLE_KW => CssDisplay::Table,
            CAPTION_KW => CssDisplay::TableCaption,
            COLGROUP_KW => CssDisplay::TableColumnGroup,
            COL_KW => CssDisplay::TableColumn,
            THEAD_KW => CssDisplay::TableHeaderGroup,
            TBODY_KW => CssDisplay::TableRowGroup,
            TFOOT_KW => CssDisplay::TableFooterGroup,
            TR_KW => CssDisplay::TableRow,
            RUBY_KW => CssDisplay::Ruby,
            RB_KW => CssDisplay::RubyBase,
            HTML_UNKNOWN_TAG => get_unknown_css_display(tag_name),
            _ => CssDisplay::Inline,
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
            let kind = token.kind();
            let tag_name = if kind == HTML_UNKNOWN_TAG {
                token.text_trimmed()
            } else {
                ""
            };
            get_css_display(kind, tag_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn display(kind: HtmlSyntaxKind) -> CssDisplay {
        get_css_display(kind, "")
    }

    #[test]
    fn test_block_elements() {
        let block_tags = [
            DIV_KW, P_KW, H1_KW, UL_KW, OL_KW, SECTION_KW, ARTICLE_KW, HEADER_KW, FOOTER_KW,
        ];
        for tag in block_tags {
            assert!(
                display(tag).is_block_like(),
                "Expected '{tag:?}' to be block-like"
            );
        }
    }

    #[test]
    fn test_inline_elements() {
        let inline_tags = [
            SPAN_KW, A_KW, STRONG_KW, EM_KW, B_KW, I_KW, CODE_KW, LABEL_KW,
        ];
        for tag in inline_tags {
            assert!(
                display(tag).is_inline_like(),
                "Expected '{tag:?}' to be inline-like"
            );
        }
    }

    #[test]
    fn test_tr_is_table_like() {
        assert!(display(TR_KW).is_table_like());
    }

    #[test]
    fn test_hidden_elements() {
        let hidden_tags = [HEAD_KW, SCRIPT_KW, STYLE_KW, META_KW, LINK_KW];
        for tag in hidden_tags {
            assert_eq!(
                display(tag),
                CssDisplay::None,
                "Expected '{tag:?}' to be display: none"
            );
        }
    }

    #[test]
    fn test_form_elements_are_inline_block() {
        let form_tags = [BUTTON_KW, INPUT_KW, SELECT_KW, TEXTAREA_KW];
        for tag in form_tags {
            assert_eq!(
                display(tag),
                CssDisplay::InlineBlock,
                "Expected '{tag:?}' to be inline-block"
            );
        }
    }

    #[test]
    fn test_whitespace_sensitive_display_classifications() {
        assert_eq!(display(MARQUEE_KW), CssDisplay::InlineBlock);

        for tag in [NOSCRIPT_KW, VIDEO_KW, AUDIO_KW, OBJECT_KW] {
            assert_eq!(
                display(tag),
                CssDisplay::Inline,
                "Expected '{tag:?}' to be inline"
            );
        }
    }

    #[test]
    fn test_unknown_elements_default_to_inline() {
        assert_eq!(
            get_css_display(HTML_UNKNOWN_TAG, "custom-element"),
            CssDisplay::Inline
        );
        assert_eq!(
            get_css_display(HTML_UNKNOWN_TAG, "my-component"),
            CssDisplay::Inline
        );
        assert_eq!(
            get_css_display(HTML_UNKNOWN_TAG, "foreignobject"),
            CssDisplay::Inline
        );
    }

    #[test]
    fn test_svg_element_is_block() {
        assert_eq!(display(IMAGE_KW), CssDisplay::Block);
    }

    #[test]
    fn test_mathml_elements_are_block() {
        assert_eq!(
            get_css_display(HTML_UNKNOWN_TAG, "mfrac"),
            CssDisplay::Block
        );
        assert_eq!(
            get_css_display(HTML_UNKNOWN_TAG, "ANNOTATION-XML"),
            CssDisplay::Block
        );
    }

    #[test]
    fn test_list_item() {
        assert_eq!(display(LI_KW), CssDisplay::ListItem);
        // ListItem is block-like
        assert!(CssDisplay::ListItem.is_block_like());
    }
}
