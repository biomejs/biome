use crate::format_element::tag::TagKind;
use crate::format_element::PrintMode;
use crate::printer::stack::{Stack, StackedStack};
use crate::printer::Indention;
use crate::{IndentStyle, InvalidDocumentError, PrintError, PrintResult};
use std::fmt::Debug;
use std::num::NonZeroU8;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum StackFrameKind {
    Root,
    Tag(TagKind),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct StackFrame {
    kind: StackFrameKind,
    args: PrintElementArgs,
}

/// Stores arguments passed to `print_element` call, holding the state specific to printing an element.
/// E.g. the `indent` depends on the token the Printer's currently processing. That's why
/// it must be stored outside of the [PrinterState] that stores the state common to all elements.
///
/// The state is passed by value, which is why it's important that it isn't storing any heavy
/// data structures. Such structures should be stored on the [PrinterState] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct PrintElementArgs {
    mode: PrintMode,
}

impl PrintElementArgs {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub(super) fn mode(&self) -> PrintMode {
        self.mode
    }

    pub fn with_print_mode(mut self, mode: PrintMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for PrintElementArgs {
    fn default() -> Self {
        Self {
            mode: PrintMode::Expanded,
        }
    }
}

/// Call stack that stores the [PrintElementCallArgs].
///
/// New [PrintElementCallArgs] are pushed onto the stack for every [`start`](Tag::is_start) [`Tag`](FormatElement::Tag)
/// and popped when reaching the corresponding [`end`](Tag::is_end) [`Tag`](FormatElement::Tag).
pub(super) trait CallStack {
    type Stack: Stack<StackFrame> + Debug;

    fn stack(&self) -> &Self::Stack;

    fn stack_mut(&mut self) -> &mut Self::Stack;

    /// Pops the call arguments at the top and asserts that they correspond to a start tag of `kind`.
    ///
    /// Returns `Ok` with the arguments if the kind of the top stack frame matches `kind`, otherwise
    /// returns `Err`.
    fn pop(&mut self, kind: TagKind) -> PrintResult<PrintElementArgs> {
        let last = self.stack_mut().pop();

        match last {
            Some(StackFrame {
                kind: StackFrameKind::Tag(actual_kind),
                args,
            }) if actual_kind == kind => Ok(args),
            // Start / End kind don't match
            Some(StackFrame {
                kind: StackFrameKind::Tag(expected_kind),
                ..
            }) => Err(PrintError::InvalidDocument(Self::invalid_document_error(
                kind,
                Some(expected_kind),
            ))),
            // Tried to pop the outer most stack frame, which is not valid
            Some(
                frame @ StackFrame {
                    kind: StackFrameKind::Root,
                    ..
                },
            ) => {
                // Put it back in to guarantee that the stack is never empty
                self.stack_mut().push(frame);
                Err(PrintError::InvalidDocument(Self::invalid_document_error(
                    kind, None,
                )))
            }

            // This should be unreachable but having it for completeness. Happens if the stack is empty.
            None => Err(PrintError::InvalidDocument(Self::invalid_document_error(
                kind, None,
            ))),
        }
    }

    #[cold]
    fn invalid_document_error(
        end_kind: TagKind,
        start_kind: Option<TagKind>,
    ) -> InvalidDocumentError {
        match start_kind {
            None => InvalidDocumentError::StartTagMissing { kind: end_kind },
            Some(start_kind) => InvalidDocumentError::StartEndTagMismatch {
                start_kind,
                end_kind,
            },
        }
    }

    /// Returns the [PrintElementArgs] for the current stack frame.
    fn top(&self) -> PrintElementArgs {
        self.stack()
            .top()
            .expect("Expected `stack` to never be empty.")
            .args
    }

    /// Returns the [TagKind] of the current stack frame or [None] if this is the root stack frame.
    fn top_kind(&self) -> Option<TagKind> {
        match self
            .stack()
            .top()
            .expect("Expected `stack` to never be empty.")
            .kind
        {
            StackFrameKind::Root => None,
            StackFrameKind::Tag(kind) => Some(kind),
        }
    }

    /// Creates a new stack frame for a [FormatElement::Tag] of `kind` with `args` as the call arguments.
    fn push(&mut self, kind: TagKind, args: PrintElementArgs) {
        self.stack_mut().push(StackFrame {
            kind: StackFrameKind::Tag(kind),
            args,
        })
    }
}

/// Call stack used for printing the [FormatElement]s
#[derive(Debug, Clone)]
pub(super) struct PrintCallStack(Vec<StackFrame>);

impl PrintCallStack {
    pub(super) fn new(args: PrintElementArgs) -> Self {
        Self(vec![StackFrame {
            kind: StackFrameKind::Root,
            args,
        }])
    }
}

impl CallStack for PrintCallStack {
    type Stack = Vec<StackFrame>;

    fn stack(&self) -> &Self::Stack {
        &self.0
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.0
    }
}

/// Call stack used for measuring if some content fits on the line.
///
/// The stack is a view on top of the [PrintCallStack] because the stack frames are still necessary for printing.
#[must_use]
pub(super) struct FitsCallStack<'print> {
    stack: StackedStack<'print, StackFrame>,
}

impl<'print> FitsCallStack<'print> {
    pub(super) fn new(print: &'print PrintCallStack, saved: Vec<StackFrame>) -> Self {
        let stack = StackedStack::with_vec(&print.0, saved);

        Self { stack }
    }

    pub(super) fn finish(self) -> Vec<StackFrame> {
        self.stack.into_vec()
    }
}

impl<'a> CallStack for FitsCallStack<'a> {
    type Stack = StackedStack<'a, StackFrame>;

    fn stack(&self) -> &Self::Stack {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.stack
    }
}

/// Suffix stack that stores the indention.
///
/// When ElementKind is [suffix], push the current indention onto the SuffixStack.
pub(super) trait SuffixStack {
    type SuffixStack: Stack<Indention> + Debug;
    fn suffix_stack_mut(&mut self) -> &mut Self::SuffixStack;
    fn push_suffix(&mut self, indention: Indention) {
        self.suffix_stack_mut().push(indention);
    }
}

/// Indent stack that stores the history of indention.
///
/// When the element kind is [indent] or [align], push the current indentation onto the stack of indentations.
/// When the element kind is [dedent], pop the last item from the indentations stack and push it onto the temp_indentations stack.
/// When the element kind is [end_dedent], pop the last item from the temp_indentations stack and push it onto the indentations stack.
pub(super) trait IndentStack {
    type Stack: Stack<Indention> + Debug;
    type HistoryStack: Stack<Indention> + Debug;

    fn current_stack(&self) -> &Self::Stack;

    fn current_stack_mut(&mut self) -> &mut Self::Stack;
    fn history_stack_mut(&mut self) -> &mut Self::HistoryStack;

    fn start_dedent(&mut self) {
        if let Some(indent) = self.current_stack_mut().pop() {
            self.history_stack_mut().push(indent);
        }
    }
    fn end_dedent(&mut self) {
        if let Some(indent) = self.history_stack_mut().pop() {
            self.current_stack_mut().push(indent);
        }
    }
    fn pop(&mut self) {
        self.current_stack_mut().pop();
    }
    fn indention(&self) -> Indention {
        self.current_stack().top().copied().unwrap_or_default()
    }
    fn reset_indent(&mut self) {
        self.current_stack_mut().push(Indention::default());
    }
    fn indent(&mut self, indent_style: IndentStyle) {
        let next_indent = self.indention().increment_level(indent_style);
        self.current_stack_mut().push(next_indent);
    }
    fn align(&mut self, count: NonZeroU8) {
        let next_indent = self.indention().set_align(count);
        self.current_stack_mut().push(next_indent);
    }
}

/// Indent stack used for storing indetion history when printing the [FormatElement]s
#[derive(Debug, Clone)]
pub(super) struct PrintIndentStack {
    indentions: Vec<Indention>,
    history_indentions: Vec<Indention>,
    suffix_indentions: Vec<Indention>,
}

impl PrintIndentStack {
    pub(super) fn new(indention: Indention) -> Self {
        Self {
            indentions: vec![indention],
            history_indentions: Vec::new(),
            suffix_indentions: Vec::new(),
        }
    }
    pub fn flush_suffixes(&mut self) {
        self.indentions
            .extend(self.suffix_indentions.drain(..).rev());
    }
}
impl IndentStack for PrintIndentStack {
    type Stack = Vec<Indention>;
    type HistoryStack = Vec<Indention>;

    fn current_stack(&self) -> &Self::Stack {
        &self.indentions
    }

    fn current_stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.indentions
    }
    fn history_stack_mut(&mut self) -> &mut Self::HistoryStack {
        &mut self.history_indentions
    }
}
impl SuffixStack for PrintIndentStack {
    type SuffixStack = Vec<Indention>;
    fn suffix_stack_mut(&mut self) -> &mut Self::SuffixStack {
        &mut self.suffix_indentions
    }
}
/// Indent stack used for storing the history of indention when measuring fits on the line.
///
/// The stack is a view on top of the [PrintIndentStack] because the stack frames are still necessary when printing.
pub(super) struct FitsIndentStack<'print> {
    indentions: StackedStack<'print, Indention>,
    history_indentions: StackedStack<'print, Indention>,
}

impl<'print> FitsIndentStack<'print> {
    pub(super) fn new(
        print: &'print PrintIndentStack,
        saved_indent_stack: Vec<Indention>,
        saved_history_indent_stack: Vec<Indention>,
    ) -> Self {
        let indentions = StackedStack::with_vec(&print.indentions, saved_indent_stack);
        let history_indentions =
            StackedStack::with_vec(&print.history_indentions, saved_history_indent_stack);

        Self {
            indentions,
            history_indentions,
        }
    }
}

impl<'a> IndentStack for FitsIndentStack<'a> {
    type Stack = StackedStack<'a, Indention>;
    type HistoryStack = StackedStack<'a, Indention>;

    fn current_stack(&self) -> &Self::Stack {
        &self.indentions
    }

    fn current_stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.indentions
    }
    fn history_stack_mut(&mut self) -> &mut Self::HistoryStack {
        &mut self.history_indentions
    }
}
