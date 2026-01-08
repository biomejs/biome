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
        children::{HtmlChild, html_split_children},
        css_display::{CssDisplay, get_css_display},
    },
    verbatim::format_html_verbatim_node,
};
use biome_formatter::{FormatRuleWithOptions, GroupId, best_fitting, prelude::*};
use biome_formatter::{VecBuffer, format_args, write};
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlClosingElement, HtmlClosingElementFields, HtmlElement,
    HtmlElementList, HtmlRoot, HtmlSyntaxToken,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElementList {
    layout: HtmlChildListLayout,
    /// Whether the parent element that encapsulates this element list is whitespace sensitive.
    is_element_whitespace_sensitive: bool,

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
    /// Whether or not the parent element that encapsulates this element list is whitespace sensitive.
    ///
    /// This should always be false for the root element.
    pub is_element_whitespace_sensitive: bool,
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
        self.is_element_whitespace_sensitive = options.is_element_whitespace_sensitive;
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

impl FormatHtmlElementList {
    pub(crate) fn fmt_children(
        &self,
        list: &HtmlElementList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<FormatChildrenResult> {
        let is_root = list.parent::<HtmlRoot>().is_some();

        // This intentionally considers the actual items in the element list, not the split children.
        //
        // For example:
        // ```html
        // <div>foo bar</div>
        // <div>foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar</div>
        // ```
        // Should get formatted as:
        // ```html
        // <div>foo bar</div>
        // <div>
        //     foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo
        //     bar foo bar foo bar foo bar foo bar foo bar
        // </div>
        let has_multiple_true_children = list.len() > 1;

        // Split children into HtmlChild variants
        let children = html_split_children(
            list.iter(),
            list.parent::<HtmlElement>()
                .and_then(|p| p.closing_element().ok())
                .as_ref(),
            f,
        )?;

        let sensitivity = f.options().whitespace_sensitivity();

        // Determine if we should force multiline
        let force_multiline =
            matches!(self.layout, HtmlChildListLayout::Multiline) || classified.has_block_element();

        // Create builders
        let mut flat_builder = FlatBuilder::new();
        let mut multiline_builder = MultilineBuilder::new();

        if force_multiline {
            flat_builder.disable();
        }

        // Print borrowed opening `>` token
        if let Some(ref r_angle_token) = self.borrowed_tokens.borrowed_opening_r_angle {
            let r_angle_format = format_with(|f| r_angle_token.format().fmt(f));
            flat_builder.write(&r_angle_format, f);
            multiline_builder.write(&r_angle_format, f);
        }

        // TODO: actually format the children here.

        // Print borrowed closing tag
        if let Some(ref closing_tag) = self.borrowed_tokens.borrowed_closing_tag {
            let closing_tag_format = format_with(|f| format_partial_closing_tag(f, closing_tag));
            flat_builder.write(&closing_tag_format, f);
            multiline_builder.write(&closing_tag_format, f);
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

// /// Helper function to format individual `HtmlChild` variants.
fn format_child(child: &HtmlChild) -> impl Format<HtmlFormatContext> + '_ {
    format_with(move |f| match child {
        HtmlChild::Word(word) => write!(f, [word]),
        HtmlChild::Comment(comment) => write!(f, [comment]),
        HtmlChild::NonText(element) => write!(f, [element.format()]),
        HtmlChild::Verbatim(element) => write!(f, [format_html_verbatim_node(element.syntax())]),
        // Whitespace variants are handled by separator logic, not here
        HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine => Ok(()),
    })
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

/// Wrapper for formatting children in multiline/expanded mode.
#[derive(Debug)]
pub(crate) struct FormatMultilineChildren {
    elements: RefCell<Vec<FormatElement>>,
}

impl Format<HtmlFormatContext> for FormatMultilineChildren {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        if let Some(elements) = f.intern_vec(self.elements.take()) {
            f.write_element(elements)?;
        }
        Ok(())
    }
}

/// Builder for multiline/expanded output.
///
/// Similar to `FlatBuilder`, but for the expanded (multiline) variant.
#[derive(Debug)]
struct MultilineBuilder {
    result: FormatResult<Vec<FormatElement>>,
}

impl MultilineBuilder {
    fn new() -> Self {
        Self {
            result: Ok(Vec::new()),
        }
    }

    fn write(&mut self, content: &dyn Format<HtmlFormatContext>, f: &mut HtmlFormatter) {
        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));
        self.result = result.and_then(|elements| {
            let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
            write!(buffer, [content])?;
            Ok(buffer.into_vec())
        })
    }

    fn finish(self) -> FormatResult<FormatMultilineChildren> {
        Ok(FormatMultilineChildren {
            elements: RefCell::new(self.result?),
        })
    }
}

/// Determines if an element should force line breaks between all its children.
///
/// Prettier source: src/language-html/utilities/index.js:271-278
#[allow(dead_code)]
fn should_force_break_children(tag_name: Option<&str>) -> bool {
    let Some(tag) = tag_name else {
        return false;
    };

    let tag_lower = tag.to_ascii_lowercase();

    // These elements always break children
    if matches!(tag_lower.as_str(), "html" | "head" | "ul" | "ol" | "select") {
        return true;
    }

    // Table-related elements (except table-cell) break children
    let display = get_css_display(&tag_lower);
    display.is_table_like() && !matches!(display, CssDisplay::TableCell)
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
