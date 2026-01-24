//! # Caution
//!
//! Although it might look a lot like `JsxChildList` formatter, the formatting logic for JSX and HTML
//! differ in several key ways, primarily due to HTML's whitespace sensitivity. It may be tempting to
//! reuse the JSX logic here, but doing so would lead to incorrect formatting in many scenarios.
//!
//! # Formatting Rules
//!
//! At a very high level, these are the rules that the HTML element list formatter must take into account.
//!
//! Something to keep in mind is that generally, the HTML formatter **maintains presence of whitespace**, just
//! not the amount of whitespace in those spots. That can sound a little confusing at first, but it'll make
//! sense after reading this.
//!
//! ## Root vs Inside element
//!
//! The root level [`HtmlElementList`] is always formatted on multiple lines. However, inside an element, it
//! depends on the initial CSS display value as defined in browser user agent style sheets. Prettier loads this
//! info from `html-ua-styles`, and Biome's HTML metadata is manually derived from it.
//!
//! ## How CSS Display affects whitespace sensitivity
//!
//! There are 2 types of whitespace sensitivity that an element can be sensitive to:
//! - Inside of the element, specifically at the first and last child, e.g. `<span>here and here</span>`
//! - Outside of the element, when its adjacent to sibling elements/text, e.g. `here <span>not here</span> here`
//!
//! These sensitivities are **not** mutually exclusive.
//!
//! The whitespace sensitivity formatter option controls if sensitivity is always considered, sometimes
//! considered (based on css display value), or never considered.
//!
//! ### Block elements
//! Block elements are elements that take up the full width of the container (by default). Because of this, the first and last children don't care about the whitespace between the parent and those children.
//!
//! Whitespace Sensitivity:
//! - Inside (first/last child): NO
//! - Outside (sibling): NO
//!
//! ```html
//! <div>
//!     It's ok for me to be like this...
//! </div>
//! <div>... or like this.</div>
//! ```
//!
//! Block elements also don't care about whitespace around them on the outside.
//!
//! ```html
//! <div>
//!     <div>Block inside block.</div>
//! </div>
//! ```
//!
//! As previously mentioned, the existence of whitespace is preserved.
//!
//! In this one, the outer and inner div has no whitespace between them:
//! ```html
//! <div><div>Block inside block.</div></div>
//! ```
//!
//! However, if the outer div has a newline before the inner div, then it treats the group
//! as breaking.
//! ```diff
//! - <div>
//! - <div>Block inside block.</div>
//! - <div>Block inside block.</div></div>
//! + <div>
//! +   <div>Block inside block.</div>
//! +   <div>Block inside block.</div>
//! + </div>
//! ```
//!
//! Also if the block element has multiple block elements inside it, the outer block element always breaks.
//! ```html
//! <div>
//!     <div>Block inside block.</div>
//!     <div>Block inside block.</div>
//! </div>
//! ```
//!
//! Some elements may not necessarily have `display: block`, but always break their children onto multiple lines (if any children are present).
//! - elements with `display: table-*`, except for `display: table-cell`
//! - elements with `display: list-item`
//! - "html", "head", "ul", "ol", "select"
//!
//! (see `forceBreakChildren()`: prettier/src/language-html/utilities/index.js:271)
//!
//! ### Inline Elements
//! Inline elements, however, are sensitive to both inside and outside whitespace.
//!
//! Whitespace Sensitivity:
//! - Inside (first/last child): YES
//! - Outside (sibling): YES
//!
//! For example, the whitespace around the inline elements here makes it so that
//! when the page renders, it'll render spaces around them. Where there is a lack
//! of spaces, no space will be rendered.
//! ```html
//! Whitespace <b>must</b> be preserved around the inline elements, or it'll render
//! <i>weird</i>.
//! ```
//!
//! It's **very important** to note that whitespace doesn't just mean any number of space ` `
//! characters. It can also refer to any amount of newlines too.
//!
//! ```html
//! <!-- Input -->
//! <span>
//! 123</span>
//! <!-- Output - newline is replaced with a space -->
//! <span> 123</span>
//! ```
//!
//! Sometimes, an inline element is too long for the line width. When an inline element breaks,
//! it actually breaks inside the element to not introduce whitespace that would be rendered by the
//! browser.
//!
//! ```html
//! <span
//!     >pretend this is really long</span
//! >
//! ```
//!
//! Biome accomplishes this by having the element list "borrow" the tokens from the container
//! to include inside the element list's group.
//!
//! ### Inline-Block Elements
//!
//! Inline-Block elements care about whitespace on the outside, but not on the inside. `<button>` is a perfect example of an Inline-Block element.
//!
//! Whitespace Sensitivity:
//! - Inside (first/last child): NO
//! - Outside (sibling): YES
//!
//! In this example, the lack of whitespace before the `<button>` causes it to hug the text.
//! But, because of the line width restriction, it can't keep the button on the same line.
//! But also, because it doesn't care about whitespace on the inside, its
//! ```html
//! <div>
//!     Nulla id velit convallis, Integer sed enim id neque molestie mollis.<button>
//!         submit
//!     </button>
//! </div>
//! ```
//!
//! ## Text Content
//!
//! Text content is generally considered to be "inline".
//! Long text content wraps at the line width limit. The easiest way to do this is to use the `f.fill()` helper.
//!
//! Examples:
//!
//! ```html
//! foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar
//! foo bar foo bar foo bar foo bar foo bar
//! <p>
//!   Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long
//!   Long Long Long
//! </p>
//! ```
//!
//! The `<br>` element has special handling, in that when it occurs, its always followed by a literal line break.
//!
//! ```html
//! <p>
//!   Long Long Long Long Long Long Long Long<br />
//!   Long Long Long Long Long Long Long Long Long Long
//! </p>
//! ```
//!
//! Text content gets condensed if it spans multiple lines. Biome has fill helpers
//! to make this happen correctly.
//!
//! ```diff
//! - This      will     get
//! - condensed into
//! -
//! -
//! -
//! - a single line   because its just text.
//! + This will get condensed into a single line because its just text.
//! ```

use crate::{
    html::auxiliary::element::{FormatHtmlElement, FormatHtmlElementOptions},
    prelude::*,
    utils::{
        children::{HtmlChild, HtmlChildrenIterator, html_split_children},
        css_display::CssDisplay,
        metadata::get_element_css_display,
    },
};
use biome_formatter::{FormatRuleWithOptions, GroupId, prelude::*};
use biome_formatter::{format_args, write};
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlClosingElement, HtmlClosingElementFields, HtmlElement,
    HtmlElementList, HtmlRoot, HtmlSyntaxToken,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElementList {
    layout: HtmlChildListLayout,
    /// Whether the parent element that encapsulates this element list is internally whitespace sensitive.
    is_container_whitespace_sensitive: bool,

    borrowed_tokens: BorrowedTokens,

    opening_tag_group: Option<GroupId>,
}

impl FormatHtmlElementList {
    pub(crate) fn with_multiline(mut self) -> Self {
        self.layout = HtmlChildListLayout::Multiline;
        self
    }
}

pub(crate) struct FormatHtmlElementListOptions {
    pub layout: HtmlChildListLayout,
    /// Whether or not the parent element that encapsulates this element list is internally whitespace sensitive.
    ///
    /// This should always be false for the root element.
    pub is_container_whitespace_sensitive: bool,
    /// Borrowed token from the containing element's opening tag.
    ///
    /// The existence of this token is not guaranteed, even if the opening tag exists. It only `Some` if
    /// the container element is whitespace sensitive and we may need to print this token with no
    /// whitespace before the children.
    pub borrowed_r_angle: Option<HtmlSyntaxToken>,
    /// Borrowed closing tag from the containing element.
    ///
    /// The existence of this tag is not guaranteed, even if the closing tag exists. It only `Some` if
    /// the container element is whitespace sensitive and we may need to print this tag with no
    /// whitespace after the children.
    pub borrowed_closing_tag: Option<HtmlClosingElement>,

    /// The opening tag's group ID that surrounds the tag and the attribute list, if any.
    pub opening_tag_group: Option<GroupId>,
}

impl FormatRuleWithOptions<HtmlElementList> for FormatHtmlElementList {
    type Options = FormatHtmlElementListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options.layout;
        self.is_container_whitespace_sensitive = options.is_container_whitespace_sensitive;
        self.borrowed_tokens = BorrowedTokens {
            borrowed_opening_r_angle: options.borrowed_r_angle,
            borrowed_closing_tag: options.borrowed_closing_tag,
        };
        self.opening_tag_group = options.opening_tag_group;
        self
    }
}

impl FormatRule<HtmlElementList> for FormatHtmlElementList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlElementList, f: &mut HtmlFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        let should_delegate_fmt_embedded_nodes = f.context().should_delegate_fmt_embedded_nodes();
        // early exit - If it's just a single HtmlEmbeddedContent node as the only child,
        // we know the parser will only emit one of these. We can simply call its formatter and be done.
        // This is also necessary for how we implement embedded language formatting.
        if node.len() == 1
            && should_delegate_fmt_embedded_nodes
            && let Some(AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlEmbeddedContent(
                embedded_content,
            ))) = node.first()
        {
            return embedded_content.format().fmt(f);
        }

        self.fmt_children(node, f)?;

        Ok(())
    }
}

/// Borrowed tokens from sibling opening and closing tags. Used to help deal with whitespace sensitivity.
#[derive(Debug, Clone, Default)]
pub(crate) struct BorrowedTokens {
    /// The opening tag's r_angle token. If present, it must be printed first before the children.
    ///
    /// The existence of this token is not guaranteed, even if the opening tag exists. It only `Some` if
    /// the container element is whitespace sensitive and we may need to print this token with no
    /// whitespace before the children.
    borrowed_opening_r_angle: Option<HtmlSyntaxToken>,

    /// The closing tag. If present, it must be printed last after the children.
    ///
    /// The existence of this tag is not guaranteed, even if the closing tag exists. It only `Some` if
    /// the container element is whitespace sensitive and we may need to print this tag with no
    /// whitespace after the children.
    borrowed_closing_tag: Option<HtmlClosingElement>,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum HtmlChildListLayout {
    /// Let the formatter decide between flat and multiline based on content
    #[default]
    BestFitting,
    /// Always format children on multiple lines
    Multiline,
}

/// Metadata about the children used to determine layout decisions.
#[derive(Copy, Clone, Debug, Default)]
struct ChildrenMeta {
    /// `true` if children contains a block-like element
    has_block_element: bool,

    /// `true` if there are multiple non-text children
    multiple_block_elements: bool,
}

impl FormatHtmlElementList {
    /// Computes metadata about the children by iterating once over all children.
    fn children_meta(&self, children: &[HtmlChild]) -> ChildrenMeta {
        let mut meta = ChildrenMeta::default();
        let mut block_element_count = 0;

        for child in children {
            match child {
                HtmlChild::NonText(element) => {
                    // Check if this is a block element
                    let display = get_element_css_display(element);
                    if display.is_block_like() {
                        meta.has_block_element = true;
                        block_element_count += 1;
                    }
                }
                HtmlChild::Verbatim(_) => {
                    block_element_count += 1;
                }
                _ => {}
            }
        }

        meta.multiple_block_elements = block_element_count > 1;
        meta
    }

    pub(crate) fn fmt_children(
        &self,
        list: &HtmlElementList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let is_root = list.parent::<HtmlRoot>().is_some();

        // Split children into HtmlChild variants
        let children = html_split_children(
            list.iter(),
            list.parent::<HtmlElement>()
                .and_then(|p| p.closing_element().ok())
                .as_ref(),
            f,
        )?;

        let children_meta = self.children_meta(&children);

        // Trim trailing new lines from children
        let mut children = children;
        if !self.is_container_whitespace_sensitive {
            while matches!(
                children.last(),
                Some(HtmlChild::EmptyLine | HtmlChild::Newline)
            ) {
                children.pop();
            }
        }

        let formatted_children = format_with(|f| {
            // Print borrowed opening `>` token
            if let Some(ref r_angle_token) = self.borrowed_tokens.borrowed_opening_r_angle {
                write!(f, [r_angle_token.format()])?;
            }

            // Determine if we should force multiline
            // Force multiline if:
            // - Layout is explicitly set to Multiline
            // - Children contain block elements
            // - There are multiple block elements
            let mut force_multiline = matches!(self.layout, HtmlChildListLayout::Multiline)
                || children_meta.multiple_block_elements;
            let mut children_iter = HtmlChildrenIterator::new(children.iter());

            // Trim leading new lines, but track if there were any
            let mut had_leading_newline = false;
            if !self.is_container_whitespace_sensitive {
                while matches!(
                    children_iter.peek(),
                    Some(HtmlChild::Newline | HtmlChild::EmptyLine)
                ) {
                    had_leading_newline = true;
                    children_iter.next();
                }
            }

            // Force multiline if there was a leading newline AND there are block elements
            // This respects the user's intent to break when they have:
            // <div>
            //   <div>...</div>
            // </div>
            if had_leading_newline && children_meta.has_block_element {
                force_multiline = true;
            }

            if force_multiline {
                write!(f, [expand_parent()])?;
            }

            let mut last: Option<&HtmlChild> = None;
            // Tracks whether the last NonText element already had its trailing whitespace
            // handled by the outer group pattern (needs_trailing_line_in_inner).
            let mut last_nontext_had_trailing_line = false;
            // Tracks the group ID of the previous inline element when it's directly followed
            // by another inline element (no whitespace between them). This is used to implement
            // Prettier's `ifBreak("", softline)` pattern where we conditionally add a line break
            // before the current element only if the previous element's group fits on a single line.
            let mut prev_inline_group_id: Option<GroupId> = None;
            // Tracks the borrowed `>` token from the previous sibling's closing tag.
            // When two inline elements are adjacent with no whitespace (`</span><span`),
            // Prettier borrows the `>` from the first element's closing tag and prints it
            // as part of the next element's opening tag group. This ensures they stay "touching".
            let mut borrowed_sibling_r_angle: Option<HtmlSyntaxToken> = None;

            let mut is_first_child = true;

            // It is **critically important** in this loop to check external whitespace sensitivity for the
            // current and next item to ensure we don't accidentally add whitespace where none is allowed!
            while let Some(child) = children_iter.next() {
                let mut child_breaks = false;
                let is_last_child = children_iter.peek().is_none();

                // For whitespace-sensitive containers (like <span>), leading/trailing
                // whitespace (including newlines) should be converted to a space in flat mode.
                // This matches Prettier's behavior where `<span>\n123</span>` becomes `<span> 123</span>`.
                if self.is_container_whitespace_sensitive && child.is_any_whitespace() {
                    if is_first_child {
                        // Leading whitespace: becomes a space in flat mode
                        write!(f, [soft_line_break_or_space()])?;
                        is_first_child = false;
                        last = Some(child);
                        continue;
                    } else if is_last_child {
                        // Trailing whitespace: becomes a space in flat mode
                        write!(f, [soft_line_break_or_space()])?;
                        last = Some(child);
                        continue;
                    }
                }

                match child {
                    // A single word in text content
                    HtmlChild::Word(word) => {
                        // when we encounter a word, we need to collect all subsequent words
                        // so we can use fill to format them together.
                        let mut fill = f.fill();
                        fill.entry(&soft_line_break_or_space(), word);
                        loop {
                            match children_iter.peek() {
                                Some(HtmlChild::Word(next_word)) => {
                                    fill.entry(&soft_line_break_or_space(), next_word);
                                    children_iter.next();
                                }
                                Some(
                                    HtmlChild::Whitespace
                                    | HtmlChild::Newline
                                    | HtmlChild::EmptyLine,
                                ) => match children_iter.peek_next() {
                                    Some(HtmlChild::Word(_)) => {
                                        // consume the whitespace/newline/emptyline
                                        children_iter.next();
                                    }
                                    Some(
                                        HtmlChild::Whitespace
                                        | HtmlChild::Newline
                                        | HtmlChild::EmptyLine,
                                    ) => {
                                        break;
                                    }
                                    _ => {
                                        break;
                                    }
                                },
                                _ => {
                                    break;
                                }
                            }
                        }
                        fill.finish()?;

                        match children_iter.peek() {
                            Some(HtmlChild::NonText(non_text)) => {
                                let css_display = get_element_css_display(non_text);
                                // not allowed to add whitespace if the next one is externally whitespace sensitive
                                // ```html
                                // <a>link</a>more text
                                // ```
                                if !css_display.is_externally_whitespace_sensitive(f) {
                                    // For inline elements, we don't add a line break here.
                                    // Instead, we'll wrap the element in an outer group with a line
                                    // before it in the NonText handling code below. This matches
                                    // Prettier's behavior where the line break happens BEFORE the
                                    // element rather than after the text.
                                    if !css_display.is_inline_like() {
                                        if css_display.is_block_like()
                                            && css_display != CssDisplay::None
                                        {
                                            // A visible block-like element adjacent to text.
                                            //
                                            // Prettier always breaks in cases like:
                                            // `<div>a<div>b</div> c</div>`.
                                            child_breaks = true;
                                            force_multiline = true;
                                            write!(f, [hard_line_break()])?;
                                        } else if css_display == CssDisplay::None {
                                            // Avoid introducing whitespace around `display: none` elements if they are adjacent to whitespace sensitive children.
                                            //
                                            // ```html
                                            // <div>123<meta attr />456</div>
                                            // ```
                                        } else if force_multiline {
                                            write!(f, [hard_line_break()])?;
                                        } else {
                                            write!(f, [soft_line_break()])?;
                                        }
                                    }
                                }
                            }
                            Some(HtmlChild::Comment(_)) => {
                                // comments are whitespace sensitive
                            }
                            Some(HtmlChild::Word(_)) => {
                                unreachable!("should have already consumed all the words")
                            }
                            _ => {}
                        }
                    }

                    // HTML comment
                    HtmlChild::Comment(comment) => {
                        write!(f, [comment])?;
                    }

                    // Whitespace between children
                    HtmlChild::Whitespace => {
                        // `<br>` is explicitly special-cased in `preferHardlineAsTrailingSpaces()` to always prefer a hard line break after it.
                        //
                        // ```html
                        // <p>
                        //   Hello world!<br/>
                        //   This is HTML5 Boilerplate.
                        // </p>
                        // ```
                        if let Some(HtmlChild::NonText(last_elem)) = last
                            && is_br_element(last_elem)
                        {
                            write!(f, [hard_line_break()])?;
                        } else if let Some(HtmlChild::NonText(last_elem)) = last
                            && let Some(HtmlChild::NonText(next)) = children_iter.peek()
                        {
                            // Whitespace between two elements
                            let last_css_display = get_element_css_display(last_elem);
                            let next_css_display = get_element_css_display(next);
                            if last_css_display.is_externally_whitespace_sensitive(f)
                                && next_css_display.is_externally_whitespace_sensitive(f)
                            {
                                // Both are inline - the outer group pattern handles the space
                                // via the trailing line in the inner group from the previous element
                            } else {
                                write!(f, [soft_line_break_or_space()])?;
                            }
                        } else if let Some(HtmlChild::NonText(last_elem)) = last {
                            // Whitespace AFTER an element (followed by text or end)
                            let last_css_display = get_element_css_display(last_elem);
                            if last_nontext_had_trailing_line {
                                // The outer group pattern already added the trailing space
                                // Don't add another one
                            } else if force_multiline
                                && matches!(children_iter.peek(), Some(HtmlChild::Word(_)))
                                && last_css_display.is_block_like()
                                && last_css_display != CssDisplay::None
                            {
                                // If we're already multiline, keep words after block-like elements on their own line.
                                child_breaks = true;
                                write!(f, [hard_line_break()])?;
                            } else if last_css_display.is_externally_whitespace_sensitive(f) {
                                // For whitespace-sensitive elements, use soft_line_break_or_space
                                // so the space is preserved in flat mode
                                write!(f, [soft_line_break_or_space()])?;
                            } else {
                                write!(f, [space()])?;
                            }
                        } else if children_iter.peek().is_none()
                            && last.is_some()
                            && !self.is_container_whitespace_sensitive
                        {
                            // If the container is not whitespace sensitive, we trim trailing whitespace
                        } else {
                            write!(f, [space()])?;
                        }
                    }

                    // A new line between children
                    HtmlChild::Newline => {
                        let is_soft_break = {
                            // Handle case of newline between single-char word and element
                            if let Some(HtmlChild::Word(word)) = last {
                                let is_next_self_closing = matches!(
                                    children_iter.peek(),
                                    Some(HtmlChild::NonText(elem)) if is_self_closing_or_br(elem)
                                );
                                !is_next_self_closing && word.is_single_character()
                            } else if let Some(HtmlChild::Word(next_word)) = children_iter.peek() {
                                let next_next = children_iter.peek_next();
                                let is_next_next_self_closing = matches!(
                                    next_next,
                                    Some(HtmlChild::NonText(elem)) if is_self_closing_or_br(elem)
                                );
                                !is_next_next_self_closing && next_word.is_single_character()
                            } else {
                                false
                            }
                        };

                        if is_soft_break {
                            write!(f, [soft_line_break()])?;
                        } else {
                            child_breaks = true;
                            write!(f, [hard_line_break()])?;
                        }
                    }

                    // An empty line between children
                    HtmlChild::EmptyLine => {
                        child_breaks = true;

                        // Preserve empty lines between children
                        write!(f, [empty_line()])?;
                    }

                    // Any non-text child (elements)
                    HtmlChild::NonText(non_text) => {
                        let non_text_group_id = f.group_id("non text child");
                        let css_display = get_element_css_display(non_text);

                        // Check if this inline element was preceded by whitespace (text context).
                        // If so, we need to wrap it in an outer group with `line` before it.
                        // This matches Prettier's "outer/inner" group pattern where the line break
                        // happens BEFORE the element rather than inside it.
                        let needs_outer_group = css_display.is_externally_whitespace_sensitive(f)
                            && matches!(last, Some(HtmlChild::Whitespace));

                        // For the outer group pattern, check if we need a trailing line inside the inner group.
                        // This handles cases like `<a>link</a> more` where there's whitespace then text after.
                        let needs_trailing_line_in_inner = needs_outer_group
                            && matches!(
                                children_iter.peek(),
                                Some(HtmlChild::Whitespace | HtmlChild::Newline)
                            );

                        // Check if the next child is also an inline element with no whitespace between.
                        // If so, we'll set prev_inline_group_id so the next element can use it for
                        // conditional line breaks.
                        let next_is_adjacent_inline =
                            if let Some(HtmlChild::NonText(non_text_next)) = children_iter.peek() {
                                let next_css_display = get_element_css_display(non_text_next);
                                css_display.is_externally_whitespace_sensitive(f)
                                    && next_css_display.is_externally_whitespace_sensitive(f)
                            } else {
                                false
                            };

                        let line_mode = match children_iter.peek() {
                            Some(HtmlChild::Word(_)) => {
                                if css_display.is_externally_whitespace_sensitive(f) {
                                    // not allowed to add whitespace if the next one is externally whitespace sensitive
                                    // ```html
                                    // <a>link</a>more text
                                    // ```
                                    None
                                } else if css_display.is_block_like()
                                    && css_display != CssDisplay::None
                                {
                                    // Block-like elements followed by text should break before the text.
                                    // This matches Prettier for cases like:
                                    // `<div>a<div>b</div> c</div>` and `<details><summary>...</summary>details</details>`.
                                    Some(LineMode::Hard)
                                } else {
                                    Some(LineMode::Soft)
                                }
                            }

                            Some(HtmlChild::NonText(non_text_next)) => {
                                let next_css_display = get_element_css_display(non_text_next);
                                if css_display.is_externally_whitespace_sensitive(f)
                                    && next_css_display.is_externally_whitespace_sensitive(f)
                                {
                                    // not allowed to add whitespace if the next one is externally whitespace sensitive
                                    // ```html
                                    // <a>link</a><span>foo</span>
                                    // ```
                                    None
                                } else {
                                    Some(LineMode::Hard)
                                }
                            }

                            Some(HtmlChild::Comment(_)) => {
                                if css_display.is_externally_whitespace_sensitive(f) {
                                    // not allowed to add whitespace if the next one is externally whitespace sensitive
                                    // ```html
                                    // <a>link</a><!-- comment -->
                                    // ```
                                    None
                                } else {
                                    Some(LineMode::Soft)
                                }
                            }

                            Some(
                                HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine,
                            ) => None,

                            Some(HtmlChild::Verbatim(verbatim_element)) => {
                                let verbatim_css_display =
                                    get_element_css_display(verbatim_element);
                                if css_display.is_externally_whitespace_sensitive(f)
                                    && verbatim_css_display.is_externally_whitespace_sensitive(f)
                                {
                                    // not allowed to add whitespace if the next one is externally whitespace sensitive
                                    // ```html
                                    // <span>1</span><!--biome-ignore format: reason--><span>2</span>
                                    // ```
                                    None
                                } else {
                                    Some(LineMode::Hard)
                                }
                            }

                            None => None,
                        };

                        child_breaks = line_mode.is_some_and(|mode| mode.is_hard());

                        let format_separator = line_mode.map(|mode| {
                            format_with(move |f| f.write_element(FormatElement::Line(mode)))
                        });

                        // When the previous element was an inline element directly followed by this
                        // inline element (no whitespace), we need to conditionally add a line break.
                        // This implements Prettier's `ifBreak("", softline)` pattern:
                        // - If the previous element's group broke: print nothing (keep elements touching)
                        // - If the previous element's group fits: print a soft line break
                        // We store the group ID and use it when writing the element below.
                        let conditional_leading_break_group_id = prev_inline_group_id.take();

                        // Take any borrowed `>` from the previous sibling element
                        let current_borrowed_r_angle = borrowed_sibling_r_angle.take();

                        // Create the element formatter with borrowing options
                        let element_format = format_element_with_borrowing(
                            non_text,
                            current_borrowed_r_angle,
                            next_is_adjacent_inline,
                        );

                        if needs_outer_group {
                            // Wrap inline element in outer group with `line` before it.
                            // This makes the line break happen BEFORE the element when it doesn't fit.
                            // Pattern: group([line, group([element, line?])])
                            // The trailing line inside the inner group handles "element whitespace text" cases.
                            let memoized = element_format.memoized();
                            let inner_group_id = f.group_id("inner");

                            if needs_trailing_line_in_inner {
                                write!(
                                    f,
                                    [group(&format_args![
                                        soft_line_break_or_space(),
                                        group(&format_args![&memoized, soft_line_break_or_space()])
                                            .with_group_id(Some(inner_group_id)),
                                    ])
                                    .with_group_id(Some(non_text_group_id))]
                                )?;
                                last_nontext_had_trailing_line = true;
                            } else {
                                write!(
                                    f,
                                    [group(&format_args![
                                        soft_line_break_or_space(),
                                        group(&memoized).with_group_id(Some(inner_group_id)),
                                        format_separator
                                    ])
                                    .with_group_id(Some(non_text_group_id))]
                                )?;
                                last_nontext_had_trailing_line = false;
                            }
                        } else if force_multiline {
                            // When force_multiline is true, wrap conditional break with element in a group
                            // to match Prettier's pattern: group([ifBreak("", softline), element])
                            if let Some(prev_id) = conditional_leading_break_group_id {
                                write!(
                                    f,
                                    [group(&format_args![
                                        if_group_fits_on_line(&soft_line_break())
                                            .with_group_id(Some(prev_id)),
                                        group(&element_format)
                                            .with_group_id(Some(non_text_group_id)),
                                        format_separator
                                    ])]
                                )?;
                            } else {
                                write!(
                                    f,
                                    [
                                        group(&element_format)
                                            .with_group_id(Some(non_text_group_id)),
                                        format_separator
                                    ]
                                )?;
                            }
                            last_nontext_had_trailing_line = false;
                        } else {
                            let mut memoized = element_format.memoized();

                            // Only propagate breaks from block-like elements.
                            // Inline elements can break internally without forcing the parent to multiline.
                            let internally_sensitive = get_element_css_display(non_text)
                                .is_internally_whitespace_sensitive(f);
                            if !internally_sensitive && memoized.inspect(f)?.will_break() {
                                force_multiline = true;
                            }
                            // Wrap conditional break with element in a group
                            // to match Prettier's pattern: group([ifBreak("", softline), element])
                            if let Some(prev_id) = conditional_leading_break_group_id {
                                write!(
                                    f,
                                    [group(&format_args![
                                        if_group_fits_on_line(&soft_line_break())
                                            .with_group_id(Some(prev_id)),
                                        group(&memoized).with_group_id(Some(non_text_group_id)),
                                        format_separator
                                    ])]
                                )?;
                            } else {
                                write!(
                                    f,
                                    [
                                        group(&memoized).with_group_id(Some(non_text_group_id)),
                                        format_separator
                                    ]
                                )?;
                            }
                            last_nontext_had_trailing_line = false;
                        }

                        // Track this element's group ID if it's followed by another adjacent inline element
                        if next_is_adjacent_inline {
                            prev_inline_group_id = Some(non_text_group_id);
                            // Store the closing r_angle token from this element for the next sibling
                            borrowed_sibling_r_angle = non_text.closing_r_angle_token();
                        } else {
                            prev_inline_group_id = None;
                        }
                    }

                    // Verbatim content (suppressed formatting)
                    HtmlChild::Verbatim(element) => {
                        write!(f, [format_verbatim_skipped(element.syntax())])?;
                    }
                }

                if child_breaks {
                    force_multiline = true;
                }

                last = Some(child);
                is_first_child = false;
            }

            // Print borrowed closing tag
            if let Some(ref closing_tag) = self.borrowed_tokens.borrowed_closing_tag {
                let closing_tag_format =
                    format_with(|f| format_partial_closing_tag(f, closing_tag));
                write!(f, [closing_tag_format])?;
            }

            Ok(())
        });

        if is_root {
            write!(f, [&formatted_children])
        } else {
            write!(f, [&soft_block_indent(&formatted_children)])
        }
    }
}

/// Check if an element is a self-closing element or `<br>`
fn is_self_closing_or_br(element: &AnyHtmlElement) -> bool {
    matches!(element, AnyHtmlElement::HtmlSelfClosingElement(_)) || is_br_element(element)
}

/// Check if an element is a `<br>` element
fn is_br_element(element: &AnyHtmlElement) -> bool {
    element
        .name()
        .is_some_and(|name| name.text().eq_ignore_ascii_case("br"))
}

fn format_partial_closing_tag(
    f: &mut Formatter<HtmlFormatContext>,
    closing_tag: &HtmlClosingElement,
) -> FormatResult<()> {
    let HtmlClosingElementFields {
        l_angle_token,
        name,
        slash_token,
        r_angle_token: _,
    } = closing_tag.as_fields();

    write!(
        f,
        [l_angle_token.format(), slash_token.format(), name.format(),]
    )
}

/// Helper to format an AnyHtmlElement with sibling borrowing options.
///
/// This is used when formatting adjacent inline elements to implement Prettier's
/// `>` borrowing pattern where the closing `>` from one element is printed at the
/// start of the next element's opening tag group.
///
/// # Arguments
/// * `element` - The element to format
/// * `borrowed_sibling_r_angle` - The `>` token borrowed from the previous sibling's closing tag
/// * `closing_r_angle_borrowed` - Whether this element's closing `>` should be borrowed by the next sibling
fn format_element_with_borrowing(
    element: &AnyHtmlElement,
    borrowed_sibling_r_angle: Option<HtmlSyntaxToken>,
    closing_r_angle_borrowed: bool,
) -> impl Format<HtmlFormatContext> + '_ {
    format_with(move |f| {
        // Only HtmlElement supports the borrowing pattern (has opening/closing tags)
        if let AnyHtmlElement::HtmlElement(html_element) = element {
            FormatNodeRule::fmt(
                &FormatHtmlElement::default().with_options(FormatHtmlElementOptions {
                    closing_r_angle_borrowed,
                    borrowed_sibling_r_angle: borrowed_sibling_r_angle.clone(),
                }),
                html_element,
                f,
            )
        } else {
            // For other element types (self-closing, etc.), use default formatting
            write!(f, [element.format()])
        }
    })
}
