//! This implementation is very heavily inspired by the JSX formatter implementation for JsxChildList.

use std::cell::RefCell;

use crate::{
    comments::HtmlComments,
    prelude::*,
    utils::{
        children::{
            html_split_children, is_meaningful_html_text, HtmlChild, HtmlChildrenIterator,
            HtmlSpace,
        },
        metadata::is_element_whitespace_sensitive_from_element,
    },
};
use biome_formatter::{best_fitting, prelude::*, CstFormatContext, FormatRuleWithOptions};
use biome_formatter::{format_args, write, VecBuffer};
use biome_html_syntax::{
    AnyHtmlElement, HtmlClosingElement, HtmlClosingElementFields, HtmlElementList, HtmlRoot,
    HtmlSyntaxToken,
};
use tag::GroupMode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElementList {
    layout: HtmlChildListLayout,
    /// Whether or not the parent element that encapsulates this element list is whitespace sensitive.
    is_element_whitespace_sensitive: bool,

    borrowed_tokens: BorrowedTokens,
}

pub(crate) struct FormatHtmlElementListOptions {
    pub layout: HtmlChildListLayout,
    /// Whether or not the parent element that encapsulates this element list is whitespace sensitive.
    ///
    /// This should always be false for the root element.
    pub is_element_whitespace_sensitive: bool,
    pub borrowed_r_angle: Option<HtmlSyntaxToken>,
    pub borrowed_closing_tag: Option<HtmlClosingElement>,
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
        self
    }
}

impl FormatRule<HtmlElementList> for FormatHtmlElementList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlElementList, f: &mut HtmlFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }
        let result = self.fmt_children(node, f)?;
        match result {
            FormatChildrenResult::ForceMultiline(format_multiline) => {
                write!(f, [format_multiline])
            }
            FormatChildrenResult::BestFitting {
                flat_children,
                expanded_children,
            } => {
                write!(f, [best_fitting![flat_children, expanded_children]])
            }
        }
    }
}

/// Borrowed tokens from sibling opening and closing tags. Used to help deal with whitespace sensitivity.
#[derive(Debug, Clone, Default)]
pub(crate) struct BorrowedTokens {
    /// The opening tag's r_angle token. If present, it must be printed first before the children.
    borrowed_opening_r_angle: Option<HtmlSyntaxToken>,

    /// The closing tag. If present, it must be printed last after the children.
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
    },
}

impl FormatHtmlElementList {
    pub(crate) fn fmt_children(
        &self,
        list: &HtmlElementList,
        f: &mut HtmlFormatter,
    ) -> FormatResult<FormatChildrenResult> {
        self.disarm_debug_assertions(list, f);

        let borrowed_opening_r_angle = self
            .borrowed_tokens
            .borrowed_opening_r_angle
            .as_ref()
            .map(|token| format_with(|f| token.format().fmt(f)))
            .memoized();
        let borrowed_closing_tag = self
            .borrowed_tokens
            .borrowed_closing_tag
            .as_ref()
            .map(|tag| format_with(|f| format_partial_closing_tag(f, tag)))
            .memoized();

        let children_meta = self.children_meta(list, f.context().comments());
        let layout = self.layout(children_meta);

        let multiline_layout = if children_meta.meaningful_text {
            MultilineLayout::Fill
        } else {
            MultilineLayout::NoFill
        };

        let is_root_parent = list
            .syntax()
            .parent()
            .is_some_and(|parent| HtmlRoot::can_cast(parent.kind()));

        let mut flat = FlatBuilder::new();
        let mut multiline = MultilineBuilder::new(multiline_layout, is_root_parent);

        let mut force_multiline = layout.is_multiline();

        let mut children = html_split_children(list.iter(), f.context().comments())?;

        // Trim trailing new lines
        if let Some(HtmlChild::EmptyLine | HtmlChild::Newline) = children.last() {
            children.pop();
        }

        let mut last: Option<&HtmlChild> = None;
        let mut children_iter = HtmlChildrenIterator::new(children.iter());

        // Trim leading new lines
        if let Some(HtmlChild::Newline | HtmlChild::EmptyLine) = children_iter.peek() {
            children_iter.next();
            // since there is a leading newline, we need to preserve the fact that there is whitespace there if this element is whitespace sensitive.
            if self.is_element_whitespace_sensitive {
                flat.write(&space(), f);
                // don't need to add anything for multiline here because there will already be a newline.
            }
        }

        flat.write(&borrowed_opening_r_angle, f);
        multiline.write_prefix(&borrowed_opening_r_angle, f);

        while let Some(child) = children_iter.next() {
            let mut child_breaks = false;
            match &child {
                // A single word: Both `a` and `b` are a word in `a b` because they're separated by HTML Whitespace.
                HtmlChild::Word(word) => {
                    let separator = match children_iter.peek() {
                        Some(HtmlChild::Word(_)) => {
                            // Separate words by a space or line break in extended mode
                            Some(WordSeparator::BetweenWords)
                        }

                        // Last word or last word before an element without any whitespace in between
                        Some(HtmlChild::NonText(next_child)) => Some(WordSeparator::EndOfText {
                            is_soft_line_break: !matches!(
                                next_child,
                                AnyHtmlElement::HtmlSelfClosingElement(_)
                            ) || word.is_single_character(),
                            is_next_element_whitespace_sensitive:
                                is_element_whitespace_sensitive_from_element(f, next_child),
                        }),

                        Some(HtmlChild::Newline | HtmlChild::Whitespace | HtmlChild::EmptyLine) => {
                            None
                        }

                        None => None,
                    };

                    child_breaks = separator.is_some_and(|separator| separator.will_break());

                    flat.write(&format_args![word, separator], f);

                    if let Some(separator) = separator {
                        multiline.write_with_separator(word, &separator, f);
                    } else {
                        // it's safe to write without a separator because None means that next element is a separator or end of the iterator
                        multiline.write_content(word, f);
                    }
                }

                // * Whitespace after the opening tag and before a meaningful text: `<div> a`
                // * Whitespace before the closing tag: `a </div>`
                // * Whitespace before an opening tag: `a <div>`
                HtmlChild::Whitespace => {
                    flat.write(&HtmlSpace, f);
                    multiline.write_separator(&soft_line_break_or_space(), f);
                }

                // A new line between some JSX text and an element
                HtmlChild::Newline => {
                    let is_soft_break = {
                        // Here we handle the case when we have a newline between a single-character word and a jsx element
                        // We need to use the previous and the next element
                        // [HtmlChild::Word, HtmlChild::Newline, HtmlChild::NonText]
                        // ```
                        // <div>
                        //   <div>First</div>,
                        //   <div>Second</div>
                        // </div>
                        // ```
                        if let Some(HtmlChild::Word(word)) = last {
                            let is_next_element_self_closing = matches!(
                                children_iter.peek(),
                                Some(HtmlChild::NonText(AnyHtmlElement::HtmlSelfClosingElement(
                                    _
                                )))
                            );
                            !is_next_element_self_closing && word.is_single_character()
                        }
                        // Here we handle the case when we have a single-character word between a new line and a jsx element
                        // Here we need to look ahead two elements
                        // [HtmlChild::Newline, HtmlChild::Word, HtmlChild::NonText]
                        // ```
                        // <div>
                        //   <div>First</div>
                        //   ,<div>Second</div>
                        // </div>
                        // ```
                        else if let Some(HtmlChild::Word(next_word)) = children_iter.peek() {
                            let next_next_element = children_iter.peek_next();
                            let is_next_next_element_new_line =
                                matches!(next_next_element, Some(HtmlChild::Newline));
                            let is_next_next_element_self_closing = matches!(
                                next_next_element,
                                Some(HtmlChild::NonText(AnyHtmlElement::HtmlSelfClosingElement(
                                    _
                                )))
                            );
                            let has_new_line_and_self_closing = is_next_next_element_new_line
                                && matches!(
                                    children_iter.peek_next_next(),
                                    Some(HtmlChild::NonText(
                                        AnyHtmlElement::HtmlSelfClosingElement(_)
                                    ))
                                );

                            !has_new_line_and_self_closing
                                && !is_next_next_element_self_closing
                                && next_word.is_single_character()
                        } else {
                            false
                        }
                    };

                    if is_soft_break {
                        multiline.write_separator(&soft_line_break(), f);
                    } else {
                        child_breaks = true;
                        multiline.write_separator(&hard_line_break(), f);
                    }
                }

                // An empty line between some JSX text and an element
                HtmlChild::EmptyLine => {
                    child_breaks = true;
                    multiline.write_separator(&empty_line(), f);
                }

                // Any child that isn't text
                HtmlChild::NonText(non_text) => {
                    let line_mode = match children_iter.peek() {
                        Some(HtmlChild::Word(word)) => {
                            // Break if the current or next element is a self closing element
                            // ```html
                            // <pre className="h-screen overflow-y-scroll" />adefg
                            // ```
                            // Becomes
                            // ```html
                            // <pre className="h-screen overflow-y-scroll" />
                            // adefg
                            // ```
                            let is_current_whitespace_sensitive =
                                is_element_whitespace_sensitive_from_element(f, non_text);
                            if is_current_whitespace_sensitive {
                                // we can't add any whitespace if the element is whitespace sensitive
                                None
                            } else if matches!(non_text, AnyHtmlElement::HtmlSelfClosingElement(_))
                                && !word.is_single_character()
                            {
                                Some(LineMode::Hard)
                            } else {
                                Some(LineMode::Soft)
                            }
                        }

                        // Add a hard line break if what comes after the element is not a text or is all whitespace
                        Some(HtmlChild::NonText(next_non_text)) => {
                            // In the case of the formatter using the multiline layout, we want to treat inline elements like we do words.
                            //
                            // ```html
                            // <a>foo</a> <a>foo</a>
                            // ```
                            // Should effectively be treated the same as:
                            // ```html
                            // foo foo
                            // ```
                            //
                            // However, block elements should go on new lines. So this:
                            // ```html
                            // <a>foo</a> <a>foo</a> <div>bar</div> <a>foo</a> <a>foo</a>
                            // ```
                            //
                            // Should get formatted as:
                            // ```html
                            // <a>foo</a> <a>foo</a>
                            // <div>bar</div>
                            // <a>foo</a> <a>foo</a>
                            // ```

                            if !is_element_whitespace_sensitive_from_element(f, non_text) {
                                Some(LineMode::Hard)
                            } else if is_element_whitespace_sensitive_from_element(f, next_non_text)
                            {
                                // only add whitespace if there is already whitespace between these elements
                                let has_whitespace_between = non_text
                                    .syntax()
                                    .last_token()
                                    .is_some_and(|tok| tok.has_trailing_whitespace())
                                    || next_non_text
                                        .syntax()
                                        .first_token()
                                        .is_some_and(|tok| tok.has_leading_whitespace_or_newline());
                                if has_whitespace_between {
                                    Some(LineMode::SoftOrSpace)
                                } else {
                                    Some(LineMode::Soft)
                                }
                            } else {
                                Some(LineMode::Hard)
                            }
                        }

                        Some(HtmlChild::Newline | HtmlChild::Whitespace | HtmlChild::EmptyLine) => {
                            None
                        }
                        // Don't insert trailing line breaks
                        None => None,
                    };

                    child_breaks = line_mode.is_some_and(|mode| mode.is_hard());

                    let format_separator = line_mode.map(|mode| {
                        format_with(move |f| f.write_element(FormatElement::Line(mode)))
                    });

                    if force_multiline {
                        if let Some(format_separator) = format_separator {
                            multiline.write_with_separator(
                                &non_text.format(),
                                &format_separator,
                                f,
                            );
                        } else {
                            // it's safe to write without a separator because None means that next element is a separator or end of the iterator
                            multiline.write_content(&non_text.format(), f);
                        }
                    } else {
                        let mut memoized = non_text.format().memoized();

                        force_multiline = memoized.inspect(f)?.will_break();
                        flat.write(&format_args![memoized, format_separator], f);

                        if let Some(format_separator) = format_separator {
                            multiline.write_with_separator(&memoized, &format_separator, f);
                        } else {
                            // it's safe to write without a separator because None means that next element is a separator or end of the iterator
                            multiline.write_content(&memoized, f);
                        }
                    }
                }
            }

            if child_breaks {
                flat.disable();
                force_multiline = true;
            }

            last = Some(child);
        }

        flat.write(&borrowed_closing_tag, f);
        multiline.write_content(&borrowed_closing_tag, f);

        if force_multiline {
            Ok(FormatChildrenResult::ForceMultiline(multiline.finish()?))
        } else {
            Ok(FormatChildrenResult::BestFitting {
                flat_children: flat.finish()?,
                expanded_children: multiline.finish()?,
            })
        }
    }

    /// Tracks the tokens of [HtmlContent] nodes to be formatted and
    /// asserts that the suppression comments are checked (they get ignored).
    ///
    /// This is necessary because the formatting of [HtmlContentList] bypasses the node formatting for
    /// [HtmlContent] and instead, formats the nodes itself.
    #[cfg(debug_assertions)]
    fn disarm_debug_assertions(&self, node: &HtmlElementList, f: &mut HtmlFormatter) {
        use biome_formatter::CstFormatContext;
        use AnyHtmlElement::*;

        for child in node {
            match child {
                HtmlContent(text) => {
                    f.state_mut().track_token(&text.value_token().unwrap());

                    // You can't suppress a text node
                    f.context()
                        .comments()
                        .mark_suppression_checked(text.syntax());
                }
                _ => {
                    continue;
                }
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn disarm_debug_assertions(&self, _: &HtmlElementList, _: &mut HtmlFormatter) {}

    fn layout(&self, meta: ChildrenMeta) -> HtmlChildListLayout {
        match self.layout {
            HtmlChildListLayout::BestFitting => {
                if meta.any_tag || meta.multiple_expressions {
                    HtmlChildListLayout::Multiline
                } else {
                    HtmlChildListLayout::BestFitting
                }
            }
            HtmlChildListLayout::Multiline => HtmlChildListLayout::Multiline,
        }
    }

    /// Computes additional meta data about the children by iterating once over all children.
    fn children_meta(&self, list: &HtmlElementList, _comments: &HtmlComments) -> ChildrenMeta {
        let mut meta = ChildrenMeta::default();

        for child in list {
            use AnyHtmlElement::*;

            match child {
                HtmlElement(_) | HtmlSelfClosingElement(_) => meta.any_tag = true,
                HtmlContent(text) => {
                    meta.meaningful_text = meta.meaningful_text
                        || text
                            .value_token()
                            .is_ok_and(|token| is_meaningful_html_text(token.text()));
                }
                _ => {}
            }
        }

        meta
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

impl HtmlChildListLayout {
    const fn is_multiline(&self) -> bool {
        matches!(self, HtmlChildListLayout::Multiline)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct ChildrenMeta {
    /// `true` if children contains a [HtmlElement] or [HtmlFragment]
    any_tag: bool,

    /// `true` if children contains more than one [HtmlExpressionChild]
    multiple_expressions: bool,

    /// `true` if any child contains meaningful a [HtmlText] with meaningful text.
    meaningful_text: bool,
}

#[derive(Copy, Clone, Debug)]
enum WordSeparator {
    /// Separator between two words. Creates a soft line break or space.
    ///
    /// `a b`
    BetweenWords,

    /// A separator of a word at the end of a [HtmlText] element. Either because it is the last
    /// child in its parent OR it is right before the start of another child (element, expression, ...).
    ///
    /// ```javascript
    /// <div>a</div>; // last element of parent
    /// <div>a<other /></div> // last element before another element
    /// <div>a{expression}</div> // last element before expression
    /// ```
    ///
    /// Creates a soft line break EXCEPT if the next element is a self closing element
    /// or the previous word was an ascii punctuation, which results in a hard line break:
    ///
    /// ```javascript
    /// a = <div>ab<br/></div>;
    ///
    /// // becomes
    ///
    /// a = (
    ///     <div>
    ///         ab
    ///         <br />
    ///     </div>
    /// );
    /// ```
    EndOfText {
        is_soft_line_break: bool,
        is_next_element_whitespace_sensitive: bool,
    },
}

impl WordSeparator {
    /// Returns if formatting this separator will result in a child that expands
    fn will_break(&self) -> bool {
        matches!(
            self,
            WordSeparator::EndOfText {
                is_soft_line_break: false,
                is_next_element_whitespace_sensitive: _
            }
        )
    }
}

impl Format<HtmlFormatContext> for WordSeparator {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match self {
            WordSeparator::BetweenWords => soft_line_break_or_space().fmt(f),
            WordSeparator::EndOfText {
                is_soft_line_break,
                is_next_element_whitespace_sensitive,
            } => {
                // If the next element is whitespace sensitive, we can't insert any whitespace.
                if *is_next_element_whitespace_sensitive {
                    return Ok(());
                }
                if *is_soft_line_break {
                    soft_line_break().fmt(f)
                }
                // ```html
                // <div>ab<br/></div>
                // ```
                // Becomes
                //
                // ```html
                // <div>
                //  ab
                //  <br />
                // </div>
                // ```
                else {
                    hard_line_break().fmt(f)
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum MultilineLayout {
    Fill,
    #[default]
    NoFill,
}

/// Builder that helps to create the output for the multiline layout.
///
/// The multiline layout may use [FormatElement::Fill] element that requires that its children
/// are an alternating sequence of `[element, separator, element, separator, ...]`.
///
/// This requires that each element is wrapped inside of a list if it emits more than one element to uphold
/// the constraints of [FormatElement::Fill].
///
/// However, the wrapping is only necessary for [MultilineLayout::Fill] for when the [FormatElement::Fill] element is used.
///
/// This builder takes care of doing the least amount of work necessary for the chosen layout while also guaranteeing
/// that the written element is valid
#[derive(Debug, Clone)]
struct MultilineBuilder {
    layout: MultilineLayout,
    is_root: bool,
    /// The elements that should be written as the main content. An alternating sequence of `[element, separator, element, separator, ...]`.
    result: FormatResult<Vec<FormatElement>>,
    /// Elements to be written before the main content.
    prefix: FormatResult<Vec<FormatElement>>,
}

impl MultilineBuilder {
    fn new(layout: MultilineLayout, is_root: bool) -> Self {
        Self {
            layout,
            is_root,
            result: Ok(Vec::new()),
            prefix: Ok(Vec::new()),
        }
    }

    /// Formats an element that does not require a separator
    /// It is safe to omit the separator because at the call side we must guarantee that we have reached the end of the iterator
    /// or the next element is a space/newline that should be written into the separator "slot".
    fn write_content(&mut self, content: &dyn Format<HtmlFormatContext>, f: &mut HtmlFormatter) {
        self.write(content, None, f);
    }

    /// Formatting a separator does not require any element in the separator slot
    fn write_separator(
        &mut self,
        separator: &dyn Format<HtmlFormatContext>,
        f: &mut HtmlFormatter,
    ) {
        self.write(separator, None, f);
    }

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
            let elements = {
                let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
                match self.layout {
                    MultilineLayout::Fill => {
                        // Make sure that the separator and content only ever write a single element
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
                        write!(buffer, [content, separator])?;

                        if let Some(separator) = separator {
                            write!(buffer, [separator])?;
                        }
                    }
                };
                buffer.into_vec()
            };
            Ok(elements)
        })
    }

    /// Write elements that should be prepended before the main content.
    fn write_prefix(&mut self, content: &dyn Format<HtmlFormatContext>, f: &mut HtmlFormatter) {
        let prefix = std::mem::replace(&mut self.prefix, Ok(Vec::new()));

        self.prefix = prefix.and_then(|elements| {
            let elements = {
                let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
                write!(buffer, [content])?;
                buffer.into_vec()
            };
            Ok(elements)
        })
    }

    fn finish(self) -> FormatResult<FormatMultilineChildren> {
        Ok(FormatMultilineChildren {
            layout: self.layout,
            is_root: self.is_root,
            elements: RefCell::new(self.result?),
            elements_prefix: RefCell::new(self.prefix?),
        })
    }
}

#[derive(Debug)]
pub(crate) struct FormatMultilineChildren {
    layout: MultilineLayout,
    is_root: bool,
    elements: RefCell<Vec<FormatElement>>,
    elements_prefix: RefCell<Vec<FormatElement>>,
}

impl Format<HtmlFormatContext> for FormatMultilineChildren {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        let format_inner = format_once(|f| {
            let prefix = f.intern_vec(self.elements_prefix.take());

            if let Some(elements) = f.intern_vec(self.elements.take()) {
                match self.layout {
                    MultilineLayout::Fill => {
                        if let Some(prefix) = prefix {
                            f.write_elements([prefix])?;
                        }
                        f.write_elements([
                            FormatElement::Tag(Tag::StartFill),
                            elements,
                            FormatElement::Tag(Tag::EndFill),
                        ])?;
                    }
                    MultilineLayout::NoFill => {
                        f.write_elements([FormatElement::Tag(Tag::StartGroup(
                            tag::Group::new().with_mode(GroupMode::Expand),
                        ))])?;
                        if let Some(prefix) = prefix {
                            f.write_elements([prefix])?;
                        }
                        f.write_elements([elements, FormatElement::Tag(Tag::EndGroup)])?;
                    }
                };
            }

            Ok(())
        });
        // We do not need the block ident when the list node is at the html root node
        if self.is_root {
            return write!(f, [format_inner]);
        }

        // This indent is wrapped with a group to ensure that the print mode is
        // set to `Expanded` when the group prints and will guarantee that the
        // content _does not_ fit when printed as part of a `Fill`. Example:
        //   <div>
        //     <span a b>
        //       <Foo />
        //     </span>{" "}
        //     ({variable})
        //   </div>
        // The `<span>...</span>` is the element that gets wrapped in the group
        // by this line. Importantly, it contains a hard line break, and because
        // [FitsMeasurer::fits_element] considers all hard lines as `Fits::Yes`,
        // it will cause the element and the following separator to be printed
        // in flat mode due to the logic of `Fill`. But because the we know the
        // item breaks over multiple lines, we want it to _not_ fit and print
        // both the content and the separator in Expanded mode, keeping the
        // formatting as shown above.
        //
        // The `group` here allows us to opt-in to telling the `FitsMeasurer`
        // that content that breaks shouldn't be considered flat and should be
        // expanded. This is in contrast to something like a concise array fill,
        // which _does_ allow breaks to fit and preserves density.
        write!(f, [group(&block_indent(&format_inner))])
    }
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
        assert!(!self.disabled, "The flat builder has been disabled and thus, does no longer store any elements. Make sure you don't call disable if you later intend to format the flat content.");

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
