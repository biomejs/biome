use crate::prelude::*;
use crate::utils::jsx::{
    is_meaningful_jsx_text, is_whitespace_jsx_expression, jsx_split_children, JsxChild,
    JsxChildrenIterator, JsxRawSpace, JsxSpace,
};
use crate::JsFormatter;
use biome_formatter::format_element::tag::{GroupMode, Tag};
use biome_formatter::{format_args, write, CstFormatContext, FormatRuleWithOptions, VecBuffer};
use biome_js_syntax::{AnyJsxChild, JsxChildList};
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxChildList {
    layout: JsxChildListLayout,
}

impl FormatRuleWithOptions<JsxChildList> for FormatJsxChildList {
    type Options = JsxChildListLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn fmt(&self, list: &JsxChildList, f: &mut JsFormatter) -> FormatResult<()> {
        let result = self.fmt_children(list, f)?;

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

/// The result of formatting the children of an [JsxChildList]. This is ultimately determined by [FormatJsxChildList::layout].
#[derive(Debug)]
pub(crate) enum FormatChildrenResult {
    /// Force the children to be formatted over multiple lines.
    ///
    /// For example:
    /// ```jsx
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

impl FormatJsxChildList {
    pub(crate) fn fmt_children(
        &self,
        list: &JsxChildList,
        f: &mut JsFormatter,
    ) -> FormatResult<FormatChildrenResult> {
        self.disarm_debug_assertions(list, f);

        let children_meta = self.children_meta(list, f.context().comments());
        let layout = self.layout(children_meta);

        let multiline_layout = if children_meta.meaningful_text {
            MultilineLayout::Fill
        } else {
            MultilineLayout::NoFill
        };

        let mut flat = FlatBuilder::new();
        let mut multiline = MultilineBuilder::new(multiline_layout);

        let mut force_multiline = layout.is_multiline();

        let mut children = jsx_split_children(list, f.context().comments())?;

        // Trim trailing new lines
        if let Some(JsxChild::EmptyLine | JsxChild::Newline) = children.last() {
            children.pop();
        }

        let mut last: Option<&JsxChild> = None;
        let mut children_iter = JsxChildrenIterator::new(children.iter());

        // Trim leading new lines
        if let Some(JsxChild::Newline | JsxChild::EmptyLine) = children_iter.peek() {
            children_iter.next();
        }

        while let Some(child) = children_iter.next() {
            let mut child_breaks = false;

            match &child {
                // A single word: Both `a` and `b` are a word in `a b` because they're separated by JSX Whitespace.
                JsxChild::Word(word) => {
                    let separator = match children_iter.peek() {
                        Some(JsxChild::Word(_)) => {
                            // Separate words by a space or line break in extended mode
                            Some(WordSeparator::BetweenWords)
                        }

                        // Last word or last word before an element without any whitespace in between
                        Some(JsxChild::NonText(next_child)) => Some(WordSeparator::EndOfText {
                            is_soft_line_break: !matches!(
                                next_child,
                                AnyJsxChild::JsxSelfClosingElement(_)
                            ) || word.is_single_character(),
                        }),

                        Some(JsxChild::Newline | JsxChild::Whitespace | JsxChild::EmptyLine) => {
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
                JsxChild::Whitespace => {
                    flat.write(&JsxSpace, f);

                    // ```javascript
                    // <div>a
                    // {' '}</div>
                    // ```
                    let is_after_line_break = last.as_ref().is_some_and(|last| last.is_any_line());

                    // `<div>aaa </div>` or `<div> </div>`
                    let is_trailing_or_only_whitespace = children_iter.peek().is_none();

                    if is_trailing_or_only_whitespace || is_after_line_break {
                        multiline.write_separator(&JsxRawSpace, f);
                    }
                    // Leading whitespace. Only possible if used together with a expression child
                    //
                    // ```
                    // <div>
                    //
                    //   {' '}
                    //   <b />
                    // </div>
                    // ```
                    else if last.is_none() {
                        multiline.write_with_separator(&JsxRawSpace, &hard_line_break(), f);
                    } else {
                        multiline.write_separator(&JsxSpace, f);
                    }
                }

                // A new line between some JSX text and an element
                JsxChild::Newline => {
                    let is_soft_break = {
                        // Here we handle the case when we have a newline between a single-character word and a jsx element
                        // We need to use the previous and the next element
                        // [JsxChild::Word, JsxChild::Newline, JsxChild::NonText]
                        // ```
                        // <div>
                        //   <div>First</div>,
                        //   <div>Second</div>
                        // </div>
                        // ```
                        if let Some(JsxChild::Word(word)) = last {
                            let is_next_element_self_closing = matches!(
                                children_iter.peek(),
                                Some(JsxChild::NonText(AnyJsxChild::JsxSelfClosingElement(_)))
                            );
                            !is_next_element_self_closing && word.is_single_character()
                        }
                        // Here we handle the case when we have a single-character word between a new line and a jsx element
                        // Here we need to look ahead two elements
                        // [JsxChild::Newline, JsxChild::Word, JsxChild::NonText]
                        // ```
                        // <div>
                        //   <div>First</div>
                        //   ,<div>Second</div>
                        // </div>
                        // ```
                        else if let Some(JsxChild::Word(next_word)) = children_iter.peek() {
                            let next_next_element = children_iter.peek_next();
                            let is_next_next_element_new_line =
                                matches!(next_next_element, Some(JsxChild::Newline));
                            let is_next_next_element_self_closing = matches!(
                                next_next_element,
                                Some(JsxChild::NonText(AnyJsxChild::JsxSelfClosingElement(_)))
                            );
                            let has_new_line_and_self_closing = is_next_next_element_new_line
                                && matches!(
                                    children_iter.peek_next_next(),
                                    Some(JsxChild::NonText(AnyJsxChild::JsxSelfClosingElement(_)))
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
                JsxChild::EmptyLine => {
                    child_breaks = true;

                    // Additional empty lines are not preserved when any of
                    // the children are a meaningful text node.
                    //
                    // <>
                    //   <div>First</div>
                    //
                    //   <div>Second</div>
                    //
                    //   Third
                    // </>
                    //
                    // Becomes:
                    //
                    // <>
                    //   <div>First</div>
                    //   <div>Second</div>
                    //   Third
                    // </>
                    if children_meta.meaningful_text {
                        multiline.write_separator(&hard_line_break(), f);
                    } else {
                        multiline.write_separator(&empty_line(), f);
                    }
                }

                // Any child that isn't text
                JsxChild::NonText(non_text) => {
                    let line_mode = match children_iter.peek() {
                        Some(JsxChild::Word(word)) => {
                            // Break if the current or next element is a self closing element
                            // ```javascript
                            // <pre className="h-screen overflow-y-scroll" />adefg
                            // ```
                            // Becomes
                            // ```javascript
                            // <pre className="h-screen overflow-y-scroll" />
                            // adefg
                            // ```
                            if matches!(non_text, AnyJsxChild::JsxSelfClosingElement(_))
                                && !word.is_single_character()
                            {
                                Some(LineMode::Hard)
                            } else {
                                Some(LineMode::Soft)
                            }
                        }

                        // Add a hard line break if what comes after the element is not a text or is all whitespace
                        Some(JsxChild::NonText(_)) => Some(LineMode::Hard),

                        Some(JsxChild::Newline | JsxChild::Whitespace | JsxChild::EmptyLine) => {
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

        if force_multiline {
            Ok(FormatChildrenResult::ForceMultiline(multiline.finish()?))
        } else {
            Ok(FormatChildrenResult::BestFitting {
                flat_children: flat.finish()?,
                expanded_children: multiline.finish()?,
            })
        }
    }

    /// Tracks the tokens of [JsxText] and [JsxExpressionChild] nodes to be formatted and
    /// asserts that the suppression comments are checked (they get ignored).
    ///
    /// This is necessary because the formatting of [JsxChildList] bypasses the node formatting for
    /// [JsxText] and [JsxExpressionChild] and instead, formats the nodes itself.
    #[cfg(debug_assertions)]
    fn disarm_debug_assertions(&self, node: &JsxChildList, f: &mut JsFormatter) {
        use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression};
        use AnyJsxChild::*;

        for child in node {
            match child {
                JsxExpressionChild(expression)
                    if is_whitespace_jsx_expression(&expression, f.context().comments()) =>
                {
                    f.context()
                        .comments()
                        .mark_suppression_checked(expression.syntax());

                    match expression.expression().unwrap() {
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
                        ) => {
                            f.context()
                                .comments()
                                .mark_suppression_checked(string_literal.syntax());

                            f.state_mut()
                                .track_token(&string_literal.value_token().unwrap());

                            f.state_mut()
                                .track_token(&expression.l_curly_token().unwrap());
                            f.state_mut()
                                .track_token(&expression.r_curly_token().unwrap());
                        }
                        _ => unreachable!(),
                    }
                }
                JsxText(text) => {
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
    fn disarm_debug_assertions(&self, _: &JsxChildList, _: &mut JsFormatter) {}

    fn layout(&self, meta: ChildrenMeta) -> JsxChildListLayout {
        match self.layout {
            JsxChildListLayout::BestFitting => {
                if meta.any_tag || meta.multiple_expressions {
                    JsxChildListLayout::Multiline
                } else {
                    JsxChildListLayout::BestFitting
                }
            }
            JsxChildListLayout::Multiline => JsxChildListLayout::Multiline,
        }
    }

    /// Computes additional meta data about the children by iterating once over all children.
    fn children_meta(&self, list: &JsxChildList, comments: &JsComments) -> ChildrenMeta {
        let mut has_expression = false;

        let mut meta = ChildrenMeta::default();

        for child in list {
            use AnyJsxChild::*;

            match child {
                JsxElement(_) | JsxFragment(_) | JsxSelfClosingElement(_) => meta.any_tag = true,
                JsxExpressionChild(expression) => {
                    if is_whitespace_jsx_expression(&expression, comments) {
                        meta.meaningful_text = true;
                    } else {
                        meta.multiple_expressions = has_expression;
                        has_expression = true;
                    }
                }
                JsxText(text) => {
                    meta.meaningful_text = meta.meaningful_text
                        || text
                            .value_token()
                            .is_ok_and(|token| is_meaningful_jsx_text(token.text()));
                }
                _ => {}
            }
        }

        meta
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum JsxChildListLayout {
    /// Prefers to format the children on a single line if possible.
    #[default]
    BestFitting,

    /// Forces the children to be formatted over multiple lines
    Multiline,
}

impl JsxChildListLayout {
    const fn is_multiline(&self) -> bool {
        matches!(self, JsxChildListLayout::Multiline)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct ChildrenMeta {
    /// `true` if children contains a [JsxElement] or [JsxFragment]
    any_tag: bool,

    /// `true` if children contains more than one [JsxExpressionChild]
    multiple_expressions: bool,

    /// `true` if any child contains meaningful a [JsxText] with meaningful text.
    meaningful_text: bool,
}

#[derive(Copy, Clone, Debug)]
enum WordSeparator {
    /// Separator between two words. Creates a soft line break or space.
    ///
    /// `a b`
    BetweenWords,

    /// A separator of a word at the end of a [JsxText] element. Either because it is the last
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
    EndOfText { is_soft_line_break: bool },
}

impl WordSeparator {
    /// Returns if formatting this separator will result in a child that expands
    fn will_break(&self) -> bool {
        matches!(
            self,
            WordSeparator::EndOfText {
                is_soft_line_break: false,
            }
        )
    }
}

impl Format<JsFormatContext> for WordSeparator {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            WordSeparator::BetweenWords => soft_line_break_or_space().fmt(f),
            WordSeparator::EndOfText { is_soft_line_break } => {
                if *is_soft_line_break {
                    soft_line_break().fmt(f)
                }
                // ```javascript
                // <div>ab<br/></div>
                // ```
                // Becomes
                //
                // ```javascript
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
    result: FormatResult<Vec<FormatElement>>,
}

impl MultilineBuilder {
    fn new(layout: MultilineLayout) -> Self {
        Self {
            layout,
            result: Ok(Vec::new()),
        }
    }

    /// Formats an element that does not require a separator
    /// It is safe to omit the separator because at the call side we must guarantee that we have reached the end of the iterator
    /// or the next element is a space/newline that should be written into the separator "slot".
    fn write_content(&mut self, content: &dyn Format<JsFormatContext>, f: &mut JsFormatter) {
        self.write(content, None, f);
    }

    /// Formatting a separator does not require any element in the separator slot
    fn write_separator(&mut self, separator: &dyn Format<JsFormatContext>, f: &mut JsFormatter) {
        self.write(separator, None, f);
    }

    fn write_with_separator(
        &mut self,
        content: &dyn Format<JsFormatContext>,
        separator: &dyn Format<JsFormatContext>,
        f: &mut JsFormatter,
    ) {
        self.write(content, Some(separator), f);
    }

    fn write(
        &mut self,
        content: &dyn Format<JsFormatContext>,
        separator: Option<&dyn Format<JsFormatContext>>,
        f: &mut JsFormatter,
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

    fn finish(self) -> FormatResult<FormatMultilineChildren> {
        Ok(FormatMultilineChildren {
            layout: self.layout,
            elements: RefCell::new(self.result?),
        })
    }
}

#[derive(Debug)]
pub(crate) struct FormatMultilineChildren {
    layout: MultilineLayout,
    elements: RefCell<Vec<FormatElement>>,
}

impl Format<JsFormatContext> for FormatMultilineChildren {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
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
                };
            }

            Ok(())
        });

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

    fn write(&mut self, content: &dyn Format<JsFormatContext>, f: &mut JsFormatter) {
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

impl Format<JsFormatContext> for FormatFlatChildren {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if let Some(elements) = f.intern_vec(self.elements.take()) {
            f.write_element(elements)?;
        }
        Ok(())
    }
}
