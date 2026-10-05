use crate::prelude::*;
use std::cell::OnceCell;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::{Buffer, FormatWithRule};

/// Temporarily changes formatter context while preserving an item's formatting rule.
///
/// Implementations must return enough state from [`Self::enter`] for
/// [`Self::exit`] to restore the context, including when formatting returns an error.
///
/// Prefer [`crate::FormatRuleWithOptions`] when the selected rule directly owns
/// the option. Use scoped options when an option must pass through an existing
/// [`FormatWithRule`] wrapper, such as a generated node union, without replacing
/// its rule.
pub trait FormatScopedOptions<Context, Item> {
    /// State required to restore the context after formatting.
    type Restore;

    /// Updates the context before formatting `item`.
    fn enter(&self, item: &Item, context: &mut Context) -> Self::Restore;

    /// Restores the context after formatting `item`, including after a formatting error.
    fn exit(&self, restore: Self::Restore, context: &mut Context);
}

/// Formats an item with temporary context options while preserving its rule.
#[derive(Debug, Clone, Copy)]
pub struct FormatWithScopedOptions<Formatted, Options> {
    formatted: Formatted,
    options: Options,
}

impl<Formatted, Options, Context> Format<Context> for FormatWithScopedOptions<Formatted, Options>
where
    Formatted: FormatWithRule<Context>,
    Options: FormatScopedOptions<Context, Formatted::Item>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let restore = self.options.enter(self.formatted.item(), f.context_mut());
        let result = self.formatted.fmt(f);
        self.options.exit(restore, f.context_mut());
        result
    }
}

impl<Formatted, Options, Context> FormatWithRule<Context>
    for FormatWithScopedOptions<Formatted, Options>
where
    Formatted: FormatWithRule<Context>,
    Options: FormatScopedOptions<Context, Formatted::Item>,
{
    type Item = Formatted::Item;

    fn item(&self) -> &Self::Item {
        self.formatted.item()
    }
}

/// Adds temporary context options to an existing formatted item.
pub trait FormatScopedOptionsExt<Context>: FormatWithRule<Context> + Sized {
    /// Wraps this item without replacing its formatting rule.
    ///
    /// Apply rule-specific options before calling this method because the
    /// returned wrapper only preserves [`FormatWithRule`]. Chained scopes are
    /// nested: the first scope in the chain is closest to the formatted item
    /// and takes precedence when multiple scopes modify the same context state.
    ///
    /// ```rust,ignore
    /// node.format().with_scoped_options(options)
    /// ```
    fn with_scoped_options<Options>(
        self,
        options: Options,
    ) -> FormatWithScopedOptions<Self, Options>
    where
        Options: FormatScopedOptions<Context, Self::Item>,
    {
        FormatWithScopedOptions {
            formatted: self,
            options,
        }
    }
}

impl<Formatted, Context> FormatScopedOptionsExt<Context> for Formatted where
    Formatted: FormatWithRule<Context>
{
}

/// Utility trait that allows memorizing the output of a [Format].
/// Useful to avoid re-formatting the same object twice.
pub trait MemoizeFormat<Context> {
    /// Returns a formattable object that memoizes the result of `Format` by cloning.
    /// Mainly useful if the same sub-tree can appear twice in the formatted output because it's
    /// used inside of `if_group_breaks` or `if_group_fits_single_line`.
    ///
    /// ```
    /// use std::cell::Cell;
    /// use biome_formatter::{format, write};
    /// use biome_formatter::prelude::*;
    /// use biome_rowan::TextSize;
    ///
    /// struct MyFormat {
    ///   value: Cell<u64>
    /// }
    ///
    /// impl MyFormat {
    ///     pub fn new() -> Self {
    ///         Self { value: Cell::new(1) }
    ///     }
    /// }
    ///
    /// impl Format<SimpleFormatContext> for MyFormat {
    ///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
    ///         let value = self.value.get();
    ///         self.value.set(value + 1);
    ///
    ///         write!(f, [text(&std::format!("Formatted {value} times."), None)])
    ///     }
    /// }
    ///
    /// # fn main() -> FormatResult<()> {
    /// let normal = MyFormat::new();
    ///
    /// // Calls `format` for everytime the object gets formatted
    /// assert_eq!(
    ///     "Formatted 1 times. Formatted 2 times.",
    ///     format!(SimpleFormatContext::default(), [normal, space(), normal])?.print()?.as_code()
    /// );
    ///
    /// // Memoized memoizes the result and calls `format` only once.
    /// let memoized = normal.memoized();
    /// assert_eq!(
    ///     "Formatted 3 times. Formatted 3 times.",
    ///     format![SimpleFormatContext::default(), [memoized, space(), memoized]]?.print()?.as_code()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn memoized(self) -> Memoized<Self, Context>
    where
        Self: Sized + Format<Context>,
    {
        Memoized::new(self)
    }
}

impl<T, Context> MemoizeFormat<Context> for T where T: Format<Context> {}

/// Memoizes the output of its inner [Format] to avoid re-formatting a potential expensive object.
#[derive(Debug)]
pub struct Memoized<F, Context> {
    inner: F,
    memory: OnceCell<FormatResult<Option<FormatElement>>>,
    options: PhantomData<Context>,
}

impl<F, Context> Memoized<F, Context>
where
    F: Format<Context>,
{
    fn new(inner: F) -> Self {
        Self {
            inner,
            memory: OnceCell::new(),
            options: PhantomData,
        }
    }

    /// Gives access to the memoized content.
    ///
    /// Performs the formatting if the content hasn't been formatted at this point.
    ///
    /// # Example
    ///
    /// Inspect if some memoized content breaks.
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use biome_formatter::{format, write};
    /// use biome_formatter::prelude::*;
    /// use biome_rowan::TextSize;
    ///
    /// #[derive(Default)]
    /// struct Counter {
    ///   value: Cell<u64>
    /// }
    ///
    /// impl Format<SimpleFormatContext> for Counter {
    ///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
    ///         let current = self.value.get();
    ///
    ///         write!(f, [
    ///             token("Count:"),
    ///             space(),
    ///             text(&std::format!("{current}"), None),
    ///             hard_line_break()
    ///         ])?;
    ///
    ///         self.value.set(current + 1);
    ///         Ok(())
    ///     }
    /// }
    ///
    /// # fn main() -> FormatResult<()> {
    /// let content = format_with(|f| {
    ///     let mut counter = Counter::default().memoized();
    ///     let counter_content = counter.inspect(f)?;
    ///
    ///     if counter_content.will_break() {
    ///         write!(f, [token("Counter:"), block_indent(&counter)])
    ///     } else {
    ///         write!(f, [token("Counter:"), counter])
    ///     }?;
    ///
    ///     write!(f, [counter])
    /// });
    ///
    ///
    /// let formatted = format!(SimpleFormatContext::default(), [content])?;
    /// assert_eq!("Counter:\n\tCount: 0\nCount: 0\n", formatted.print()?.as_code());
    /// # Ok(())
    /// # }
    ///
    /// ```
    pub fn inspect(&mut self, f: &mut Formatter<Context>) -> FormatResult<&[FormatElement]> {
        let result = self.memory.get_or_init(|| f.intern(&self.inner));

        match result.as_ref() {
            Ok(Some(FormatElement::Interned(interned))) => Ok(interned.deref()),
            Ok(Some(other)) => Ok(std::slice::from_ref(other)),
            Ok(None) => Ok(&[]),
            Err(error) => Err(*error),
        }
    }
}

impl<F, Context> Format<Context> for Memoized<F, Context>
where
    F: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let result = self.memory.get_or_init(|| f.intern(&self.inner));

        match result {
            Ok(Some(elements)) => {
                f.write_element(elements.clone())?;

                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(*err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        FormatContext, FormatError, FormatRefWithRule, FormatRule, SimpleFormatOptions,
        TransformSourceMap, format,
    };
    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Mode {
        Outer,
        Middle,
        Inner,
    }

    impl Mode {
        const fn text(self) -> &'static str {
            match self {
                Self::Outer => "outer",
                Self::Middle => "middle",
                Self::Inner => "inner",
            }
        }
    }

    #[derive(Debug)]
    struct TestContext {
        options: SimpleFormatOptions,
        mode: Mode,
        observed: Rc<Cell<Mode>>,
    }

    impl TestContext {
        fn new(mode: Mode, observed: Rc<Cell<Mode>>) -> Self {
            Self {
                options: SimpleFormatOptions::default(),
                mode,
                observed,
            }
        }
    }

    impl FormatContext for TestContext {
        type Options = SimpleFormatOptions;

        fn options(&self) -> &Self::Options {
            &self.options
        }

        fn source_map(&self) -> Option<&TransformSourceMap> {
            None
        }
    }

    struct TestItem {
        calls: Cell<u8>,
    }

    #[derive(Debug, Clone, Copy)]
    struct TestRule {
        fail: bool,
    }

    impl FormatRule<TestItem> for TestRule {
        type Context = TestContext;

        fn fmt(&self, item: &TestItem, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
            item.calls.set(item.calls.get() + 1);
            f.context().observed.set(f.context().mode);

            if self.fail {
                Err(FormatError::SyntaxError)
            } else {
                token(f.context().mode.text()).fmt(f)
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct ScopedMode(Mode);

    impl<Item> FormatScopedOptions<TestContext, Item> for ScopedMode {
        type Restore = Mode;

        fn enter(&self, _item: &Item, context: &mut TestContext) -> Self::Restore {
            let previous = std::mem::replace(&mut context.mode, self.0);
            context.observed.set(context.mode);
            previous
        }

        fn exit(&self, restore: Self::Restore, context: &mut TestContext) {
            context.mode = restore;
            context.observed.set(context.mode);
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct CopyFormat;

    impl Format<TestContext> for CopyFormat {
        fn fmt(&self, _f: &mut Formatter<TestContext>) -> FormatResult<()> {
            Ok(())
        }
    }

    impl FormatWithRule<TestContext> for CopyFormat {
        type Item = ();

        fn item(&self) -> &Self::Item {
            &()
        }
    }

    fn assert_copy<T: Copy>(_: &T) {}

    #[test]
    fn scoped_options_copy_does_not_depend_on_context() {
        let formatted = CopyFormat.with_scoped_options(ScopedMode(Mode::Inner));
        assert_copy(&formatted);
    }

    #[test]
    fn scoped_options_delegate_to_the_existing_rule_and_restore_context() {
        let observed = Rc::new(Cell::new(Mode::Outer));
        let context = TestContext::new(Mode::Outer, Rc::clone(&observed));
        let item = TestItem {
            calls: Cell::new(0),
        };
        let formatted = FormatRefWithRule::new(&item, TestRule { fail: false })
            .with_scoped_options(ScopedMode(Mode::Inner));

        let formatted = format!(context, [formatted]).unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "inner");
        assert_eq!(formatted.context().mode, Mode::Outer);
        assert_eq!(observed.get(), Mode::Outer);
        assert_eq!(item.calls.get(), 1);
    }

    #[test]
    fn scoped_options_nest_in_lifo_order() {
        let observed = Rc::new(Cell::new(Mode::Outer));
        let context = TestContext::new(Mode::Outer, Rc::clone(&observed));
        let item = TestItem {
            calls: Cell::new(0),
        };
        let formatted = FormatRefWithRule::new(&item, TestRule { fail: false })
            .with_scoped_options(ScopedMode(Mode::Inner))
            .with_scoped_options(ScopedMode(Mode::Middle));

        assert!(std::ptr::eq(formatted.item(), &item));

        let formatted = format!(context, [formatted]).unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "inner");
        assert_eq!(formatted.context().mode, Mode::Outer);
        assert_eq!(observed.get(), Mode::Outer);
        assert_eq!(item.calls.get(), 1);
    }

    #[test]
    fn scoped_options_restore_context_after_an_error() {
        let observed = Rc::new(Cell::new(Mode::Outer));
        let context = TestContext::new(Mode::Outer, Rc::clone(&observed));
        let item = TestItem {
            calls: Cell::new(0),
        };
        let formatted = FormatRefWithRule::new(&item, TestRule { fail: true })
            .with_scoped_options(ScopedMode(Mode::Inner));

        assert!(matches!(
            format!(context, [formatted]),
            Err(FormatError::SyntaxError)
        ));
        assert_eq!(observed.get(), Mode::Outer);
        assert_eq!(item.calls.get(), 1);
    }
}
