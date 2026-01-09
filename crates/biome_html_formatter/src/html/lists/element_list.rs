//! # Caution
//!
//! Although it might look a lot like `JsxChildList` formatter, the formatting logic for JSX and HTML
//! differ in several key ways, primarily due to HTML's whitespace sensitivity. It may be tempting to
//! reuse the JSX logic here, but doing so would leads to incorrect formatting in many scenarios.
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
//! <i>wierd</i>.
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
//! Biome acomplishes this by having the element list "borrow" the tokens from the container
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

use std::cell::RefCell;

use crate::{
    prelude::*,
    utils::{
        children::{HtmlChild, HtmlChildrenIterator, html_split_children},
        css_display::{CssDisplay, get_css_display},
    },
    verbatim::format_html_verbatim_node,
};
use biome_formatter::format_element::tag::{GroupMode, Tag};
use biome_formatter::{FormatRuleWithOptions, GroupId, best_fitting, prelude::*};
use biome_formatter::{VecBuffer, format_args, write};
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlClosingElement, HtmlClosingElementFields, HtmlElement,
    HtmlElementList, HtmlRoot, HtmlSyntaxToken,
};
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElementList {
    layout: HtmlChildListLayout,
    /// Whether the parent element that encapsulates this element list is internally whitespace sensitive.
    is_container_whitespace_sensitive: bool,

    borrowed_tokens: BorrowedTokens,

    group: Option<GroupId>,

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

        let result = self.fmt_children(node, f)?;
        match result {
            FormatChildrenResult::ForceMultiline(format_multiline) => {
                write!(f, [format_multiline])?;
            }
            FormatChildrenResult::BestFitting {
                flat_children,
                expanded_children,
                group_id: _,
            } => {
                write!(f, [best_fitting![flat_children, expanded_children]])?;
            }
        }

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

/// The result of formatting the children of an [HtmlElementList]. This is ultimately determined by [FormatHtmlElementList::layout].
#[derive(Debug)]
pub(crate) enum FormatChildrenResult {
    /// Force the children to be formatted over multiple lines.
    ///
    /// For example:
    /// ```html
    /// <div>
    ///     <div>1</div>
    ///     <div>2</div>
    /// </div>
    /// ```
    ///
    /// This usually occurs when the children are already formatted over multiple lines, or when the children contains another tag.
    ForceMultiline(FormatMultilineChildren),

    /// Let the formatter determine whether the children should be formatted over multiple lines or if they can be kept on a single line.
    BestFitting {
        flat_children: FormatFlatChildren,
        expanded_children: FormatMultilineChildren,
        group_id: Option<GroupId>,
    },
}

impl Format<HtmlFormatContext> for FormatChildrenResult {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match self {
            Self::ForceMultiline(multiline) => {
                write!(f, [multiline])
            }
            Self::BestFitting {
                flat_children,
                expanded_children,
                group_id,
            } => {
                let group_id = group_id.unwrap_or(f.group_id("html-element-list-children-default"));

                let expanded_children = expanded_children.memoized();
                write!(
                    f,
                    [
                        // If the attribute group breaks, prettier always breaks the children as well.
                        &if_group_breaks(&expanded_children).with_group_id(Some(group_id)),
                        // If the attribute group does NOT break, print whatever fits best for the children.
                        &if_group_fits_on_line(&best_fitting![
                            format_args![flat_children],
                            format_args![expanded_children],
                        ])
                        .with_group_id(Some(group_id)),
                    ]
                )
            }
        }
    }
}

/// Metadata about the children used to determine layout decisions.
#[derive(Copy, Clone, Debug, Default)]
struct ChildrenMeta {
    /// `true` if children contains a block-like element
    has_block_element: bool,

    /// `true` if any child contains meaningful text content
    meaningful_text: bool,

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
                HtmlChild::Word(_) | HtmlChild::Comment(_) => {
                    meta.meaningful_text = true;
                }
                HtmlChild::Verbatim(_) => {
                    block_element_count += 1;
                }
                HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine => {}
            }
        }

        meta.multiple_block_elements = block_element_count > 1;
        meta
    }

    pub(crate) fn fmt_children(
        &self,
        list: &HtmlElementList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<FormatChildrenResult> {
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

        // Determine multiline layout based on whether there's meaningful text
        let multiline_layout = if children_meta.meaningful_text {
            MultilineLayout::Fill
        } else {
            MultilineLayout::NoFill
        };

        // Determine if we should force multiline
        // Force multiline if:
        // - Layout is explicitly set to Multiline
        // - Children contain block elements
        // - There are multiple block elements
        let mut force_multiline = matches!(self.layout, HtmlChildListLayout::Multiline)
            || children_meta.has_block_element
            || children_meta.multiple_block_elements;

        // Create builders
        let mut flat_builder = FlatBuilder::new();
        let mut multiline_builder = MultilineBuilder::new(multiline_layout, is_root);

        if force_multiline {
            flat_builder.disable();
        }

        // Print borrowed opening `>` token
        if let Some(ref r_angle_token) = self.borrowed_tokens.borrowed_opening_r_angle {
            let r_angle_format = format_with(|f| r_angle_token.format().fmt(f)).memoized();
            flat_builder.write(&r_angle_format, f);
            multiline_builder.write_content(&r_angle_format, f);
        }

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

        let mut children_iter = HtmlChildrenIterator::new(children.iter());

        // Trim leading new lines
        if !self.is_container_whitespace_sensitive {
            while matches!(
                children_iter.peek(),
                Some(HtmlChild::Newline | HtmlChild::EmptyLine)
            ) {
                children_iter.next();
            }
        }

        let mut last: Option<&HtmlChild> = None;

        let mut is_first_child = true;
        while let Some(child) = children_iter.next() {
            let mut child_breaks = false;

            let is_last_child = children_iter.peek().is_none();

            // For whitespace-sensitive containers (like <span>), leading/trailing
            // whitespace (including newlines) should be converted to a space in flat mode.
            // This matches Prettier's behavior where `<span>\n123</span>` becomes `<span> 123</span>`.
            if self.is_container_whitespace_sensitive && child.is_whitespace() {
                if is_first_child {
                    // Leading whitespace: becomes a space in flat mode
                    flat_builder.write(&space(), f);
                    multiline_builder.write_with_separator(&space(), &hard_line_break(), f);
                    is_first_child = false;
                    last = Some(child);
                    continue;
                } else if is_last_child {
                    // Trailing whitespace: becomes a space in flat mode
                    flat_builder.write(&space(), f);
                    multiline_builder.write_separator(&hard_line_break(), f);
                    last = Some(child);
                    continue;
                }
            }

            match child {
                // A single word in text content
                HtmlChild::Word(word) => {
                    let separator = match children_iter.peek() {
                        Some(HtmlChild::Word(_)) => {
                            // Separate words by a space or line break in extended mode
                            Some(WordSeparator::BetweenWords)
                        }

                        // Last word or word before an element without any whitespace in between
                        Some(HtmlChild::NonText(next_child)) => {
                            // <br> elements handle their own line breaking (after themselves),
                            // so we don't need to force a break before them.
                            // Other self-closing elements may need a hard break before them.
                            let is_br = is_br_element(next_child);
                            Some(WordSeparator::EndOfText {
                                is_soft_line_break: is_br
                                    || !is_self_closing_or_br(next_child)
                                    || word.is_single_character(),
                            })
                        }

                        Some(HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine) => {
                            None
                        }

                        Some(HtmlChild::Comment(_) | HtmlChild::Verbatim(_)) => {
                            Some(WordSeparator::BetweenWords)
                        }

                        None => None,
                    };

                    child_breaks = separator.is_some_and(|sep| sep.will_break());

                    flat_builder.write(&format_args![word, separator], f);

                    if let Some(separator) = separator {
                        multiline_builder.write_with_separator(word, &separator, f);
                    } else {
                        multiline_builder.write_content(word, f);
                    }
                }

                // HTML comment
                HtmlChild::Comment(comment) => {
                    let separator = match children_iter.peek() {
                        Some(HtmlChild::Word(_)) => Some(WordSeparator::BetweenWords),
                        Some(HtmlChild::NonText(_)) => Some(WordSeparator::EndOfText {
                            is_soft_line_break: true,
                        }),
                        _ => None,
                    };

                    flat_builder.write(&format_args![comment, separator], f);

                    if let Some(separator) = separator {
                        multiline_builder.write_with_separator(comment, &separator, f);
                    } else {
                        multiline_builder.write_content(comment, f);
                    }
                }

                // Whitespace between children
                HtmlChild::Whitespace => {
                    flat_builder.write(&space(), f);

                    let is_after_line_break = last.as_ref().is_some_and(|l| l.is_any_line());
                    let is_trailing_or_only_whitespace = children_iter.peek().is_none();

                    if is_trailing_or_only_whitespace || is_after_line_break {
                        multiline_builder.write_separator(&space(), f);
                    } else if last.is_none() {
                        // Leading whitespace
                        multiline_builder.write_with_separator(&space(), &hard_line_break(), f);
                    } else {
                        multiline_builder.write_separator(&space(), f);
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
                        multiline_builder.write_separator(&soft_line_break(), f);
                    } else {
                        child_breaks = true;
                        multiline_builder.write_separator(&hard_line_break(), f);
                    }
                }

                // An empty line between children
                HtmlChild::EmptyLine => {
                    child_breaks = true;

                    // Preserve empty lines between children
                    multiline_builder.write_separator(&empty_line(), f);
                }

                // Any non-text child (elements)
                HtmlChild::NonText(non_text) => {
                    let is_br = is_br_element(non_text);
                    let line_mode = match children_iter.peek() {
                        Some(HtmlChild::Word(word)) => {
                            // <br /> always forces a hard line break after it
                            if is_br {
                                Some(LineMode::Hard)
                            } else if is_self_closing_or_br(non_text) && !word.is_single_character()
                            {
                                Some(LineMode::Hard)
                            } else {
                                Some(LineMode::Soft)
                            }
                        }

                        Some(HtmlChild::NonText(_)) => Some(LineMode::Hard),

                        Some(HtmlChild::Comment(_)) => {
                            if is_br {
                                Some(LineMode::Hard)
                            } else {
                                Some(LineMode::Soft)
                            }
                        }

                        Some(HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine) => {
                            // <br /> forces hard break even with trailing whitespace
                            if is_br { Some(LineMode::Hard) } else { None }
                        }

                        Some(HtmlChild::Verbatim(_)) => Some(LineMode::Hard),

                        None => {
                            // <br /> at the end still forces a break
                            if is_br { Some(LineMode::Hard) } else { None }
                        }
                    };

                    child_breaks = line_mode.is_some_and(|mode| mode.is_hard());

                    let format_separator = line_mode.map(|mode| {
                        format_with(move |f| f.write_element(FormatElement::Line(mode)))
                    });

                    if force_multiline {
                        if let Some(format_separator) = format_separator {
                            multiline_builder.write_with_separator(
                                &non_text.format(),
                                &format_separator,
                                f,
                            );
                        } else {
                            multiline_builder.write_content(&non_text.format(), f);
                        }
                    } else {
                        let mut memoized = non_text.format().memoized();

                        // Only propagate breaks from block-like elements.
                        // Inline elements can break internally without forcing the parent to multiline.
                        let is_block_like = get_element_css_display(non_text).is_block_like();
                        if is_block_like && memoized.inspect(f)?.will_break() {
                            force_multiline = true;
                        }
                        flat_builder.write(&format_args![memoized, format_separator], f);

                        if let Some(format_separator) = format_separator {
                            multiline_builder.write_with_separator(&memoized, &format_separator, f);
                        } else {
                            multiline_builder.write_content(&memoized, f);
                        }
                    }
                }

                // Verbatim content (suppressed formatting)
                HtmlChild::Verbatim(element) => {
                    let format_verbatim = format_html_verbatim_node(element.syntax());

                    let line_mode = match children_iter.peek() {
                        Some(HtmlChild::NonText(_) | HtmlChild::Verbatim(_)) => {
                            Some(LineMode::Hard)
                        }
                        Some(HtmlChild::Word(_)) => Some(LineMode::Soft),
                        _ => None,
                    };

                    child_breaks = line_mode.is_some_and(|mode| mode.is_hard());

                    let format_separator = line_mode.map(|mode| {
                        format_with(move |f| f.write_element(FormatElement::Line(mode)))
                    });

                    flat_builder.write(&format_args![format_verbatim, format_separator], f);

                    if let Some(format_separator) = format_separator {
                        multiline_builder.write_with_separator(
                            &format_verbatim,
                            &format_separator,
                            f,
                        );
                    } else {
                        multiline_builder.write_content(&format_verbatim, f);
                    }
                }
            }

            if child_breaks {
                flat_builder.disable();
                force_multiline = true;
            }

            last = Some(child);
            is_first_child = false;
        }

        // Print borrowed closing tag
        if let Some(ref closing_tag) = self.borrowed_tokens.borrowed_closing_tag {
            let closing_tag_format =
                format_with(|f| format_partial_closing_tag(f, closing_tag)).memoized();
            flat_builder.write(&closing_tag_format, f);
            multiline_builder.write_content(&closing_tag_format, f);
        }

        // Return result
        if force_multiline {
            Ok(FormatChildrenResult::ForceMultiline(
                multiline_builder.finish()?,
            ))
        } else {
            Ok(FormatChildrenResult::BestFitting {
                flat_children: flat_builder.finish()?,
                expanded_children: multiline_builder.finish()?,
                group_id: self.group,
            })
        }
    }
}

/// Gets the CSS display value for an HTML element.
fn get_element_css_display(element: &AnyHtmlElement) -> CssDisplay {
    if let Some(tag_name) = element.name() {
        get_css_display(&tag_name)
    } else {
        CssDisplay::Inline
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

/// Separator between words in text content.
#[derive(Copy, Clone, Debug)]
enum WordSeparator {
    /// Separator between two words. Creates a soft line break or space.
    BetweenWords,

    /// A separator at the end of text content, before an element.
    EndOfText { is_soft_line_break: bool },
}

impl WordSeparator {
    fn will_break(&self) -> bool {
        matches!(
            self,
            Self::EndOfText {
                is_soft_line_break: false,
            }
        )
    }
}

impl Format<HtmlFormatContext> for WordSeparator {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match self {
            Self::BetweenWords => soft_line_break_or_space().fmt(f),
            Self::EndOfText { is_soft_line_break } => {
                if *is_soft_line_break {
                    soft_line_break().fmt(f)
                } else {
                    hard_line_break().fmt(f)
                }
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum HtmlChildListLayout {
    /// Prefers to format the children on a single line if possible.
    #[default]
    BestFitting,

    /// Forces the children to be formatted over multiple lines
    Multiline,
}

#[derive(Debug)]
struct FlatBuilder {
    result: FormatResult<Vec<FormatElement>>,
    disabled: bool,
}

impl FlatBuilder {
    fn new() -> Self {
        Self {
            result: Ok(Vec::new()),
            disabled: false,
        }
    }

    fn write(&mut self, content: &dyn Format<HtmlFormatContext>, f: &mut HtmlFormatter) {
        if self.disabled {
            return;
        }

        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));

        self.result = result.and_then(|elements| {
            let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);

            write!(buffer, [content])?;

            Ok(buffer.into_vec())
        })
    }

    fn disable(&mut self) {
        self.disabled = true;
    }

    fn finish(self) -> FormatResult<FormatFlatChildren> {
        assert!(
            !self.disabled,
            "The flat builder has been disabled and thus, does no longer store any elements. Make sure you don't call disable if you later intend to format the flat content."
        );

        Ok(FormatFlatChildren {
            elements: RefCell::new(self.result?),
        })
    }
}

#[derive(Debug)]
pub(crate) struct FormatFlatChildren {
    elements: RefCell<Vec<FormatElement>>,
}

impl Format<HtmlFormatContext> for FormatFlatChildren {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        if let Some(elements) = f.intern_vec(self.elements.take()) {
            f.write_element(elements)?;
        }
        Ok(())
    }
}

/// Layout mode for multiline content
#[derive(Copy, Clone, Debug, Default)]
enum MultilineLayout {
    /// Use fill layout for text content wrapping
    Fill,
    /// Don't use fill, just regular line breaks
    #[default]
    NoFill,
}

/// Builder for multiline/expanded output.
///
/// Supports both Fill and NoFill layouts depending on whether there's text content.
#[derive(Debug)]
struct MultilineBuilder {
    layout: MultilineLayout,
    result: FormatResult<Vec<FormatElement>>,
    is_root: bool,
}

impl MultilineBuilder {
    fn new(layout: MultilineLayout, is_root: bool) -> Self {
        Self {
            layout,
            result: Ok(Vec::new()),
            is_root,
        }
    }

    /// Formats content without a separator
    fn write_content(&mut self, content: &dyn Format<HtmlFormatContext>, f: &mut HtmlFormatter) {
        self.write(content, None, f);
    }

    /// Formats just a separator
    fn write_separator(
        &mut self,
        separator: &dyn Format<HtmlFormatContext>,
        f: &mut HtmlFormatter,
    ) {
        self.write(separator, None, f);
    }

    /// Formats content with a separator
    fn write_with_separator(
        &mut self,
        content: &dyn Format<HtmlFormatContext>,
        separator: &dyn Format<HtmlFormatContext>,
        f: &mut HtmlFormatter,
    ) {
        self.write(content, Some(separator), f);
    }

    fn write(
        &mut self,
        content: &dyn Format<HtmlFormatContext>,
        separator: Option<&dyn Format<HtmlFormatContext>>,
        f: &mut HtmlFormatter,
    ) {
        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));

        self.result = result.and_then(|elements| {
            let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);

            match self.layout {
                MultilineLayout::Fill => {
                    // Wrap in entry tags for fill layout
                    buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;
                    write!(buffer, [content])?;
                    buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

                    if let Some(separator) = separator {
                        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;
                        write!(buffer, [separator])?;
                        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;
                    }
                }
                MultilineLayout::NoFill => {
                    write!(buffer, [content])?;

                    if let Some(separator) = separator {
                        write!(buffer, [separator])?;
                    }
                }
            }

            Ok(buffer.into_vec())
        });
    }

    fn finish(self) -> FormatResult<FormatMultilineChildren> {
        Ok(FormatMultilineChildren {
            layout: self.layout,
            elements: RefCell::new(self.result?),
            is_root: self.is_root,
        })
    }
}

/// Wrapper for formatting children in multiline/expanded mode.
#[derive(Debug)]
pub(crate) struct FormatMultilineChildren {
    layout: MultilineLayout,
    elements: RefCell<Vec<FormatElement>>,
    is_root: bool,
}

impl Format<HtmlFormatContext> for FormatMultilineChildren {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        let format_inner = format_once(|f| {
            if let Some(elements) = f.intern_vec(self.elements.take()) {
                match self.layout {
                    MultilineLayout::Fill => f.write_elements([
                        FormatElement::Tag(Tag::StartFill),
                        elements,
                        FormatElement::Tag(Tag::EndFill),
                    ])?,
                    MultilineLayout::NoFill => f.write_elements([
                        FormatElement::Tag(Tag::StartGroup(
                            tag::Group::new().with_mode(GroupMode::Expand),
                        )),
                        elements,
                        FormatElement::Tag(Tag::EndGroup),
                    ])?,
                }
            }

            Ok(())
        });

        if self.is_root {
            write!(f, [format_inner])
        } else {
            write!(f, [group(&block_indent(&format_inner))])
        }
    }
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
