use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::AnyJsFunctionBody;

#[derive(Copy, Clone, Debug, Default)]
pub enum FunctionBodyCacheMode {
    /// Format the body without caching it or retrieving it from the cache.
    #[default]
    NoCache,

    /// The body has been cached before, try to retrieve the body from the cache.
    Cached,

    /// Cache the body during the next [formatting](Format::fmt).
    Cache,
}

/// Formats a [function body](AnyJsFunctionBody) with additional caching depending on [`mode`](Self::mode).
pub(crate) struct FormatMaybeCachedFunctionBody<'a> {
    /// The body to format.
    pub body: &'a AnyJsFunctionBody,

    /// If the body should be cached or if the formatter should try to retrieve it from the cache.
    pub mode: FunctionBodyCacheMode,
}

impl Format<JsFormatContext> for FormatMaybeCachedFunctionBody<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.mode {
            FunctionBodyCacheMode::NoCache => {
                write!(f, [self.body.format()])
            }
            FunctionBodyCacheMode::Cached => {
                match f.context().get_cached_function_body(self.body) {
                    Some(cached) => f.write_element(cached),
                    None => {
                        // This can happen in the unlikely event where a function has a parameter with
                        // an initializer that contains a call expression with a first or last function/arrow
                        // ```javascript
                        // test((
                        //   problematic = test(() => body)
                        // ) => {});
                        // ```
                        // This case should be rare as it requires very specific syntax (and is rather messy to write)
                        // which is why it's fine to just fallback to formatting the body again in this case.
                        write!(f, [self.body.format()])
                    }
                }
            }
            FunctionBodyCacheMode::Cache => match f.intern(&self.body.format())? {
                Some(interned) => {
                    f.context_mut()
                        .set_cached_function_body(self.body, interned.clone());
                    f.write_element(interned)
                }
                None => Ok(()),
            },
        }
    }
}
