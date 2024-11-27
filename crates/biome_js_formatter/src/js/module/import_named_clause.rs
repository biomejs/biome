use crate::prelude::*;

use biome_formatter::{write, CstFormatContext};
use biome_js_syntax::JsImportNamedClause;
use biome_js_syntax::JsImportNamedClauseFields;
use biome_js_syntax::JsNamedImportSpecifiersFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportNamedClause;

impl FormatNodeRule<JsImportNamedClause> for FormatJsImportNamedClause {
    fn fmt_fields(&self, node: &JsImportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportNamedClauseFields {
            type_token,
            named_specifiers,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        let named_specifiers = named_specifiers?;
        let specifiers = named_specifiers.specifiers();

        // can_break implementation, return `format_element` instead of boolean to reduce enum conversion overhead.
        // if `can_break` is true we just use the previous format strategy, otherwise we use the new format strategy.
        // reference https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L173

        // https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L184-L209v,
        // `standaloneSpecifiers` corresponding our `JsDefaultImportSpecifier` + part of `JsNamespaceImportSpecifier`,
        // `groupedSpecifiers` corresponding our `JsNamedImportSpecifiers`

        //  Here we use an opposite way of thinking, we only thinking about the way that can not break
        // That's to say
        // 2. length of `JsNamedImportSpecifiers` at least is one
        // 3. Surrounding of the only `JsNamedImportSpecifiers` should not have any comments
        if specifiers.len() == 1
            && !f
                .context()
                .comments()
                .is_suppressed(named_specifiers.syntax())
        {
            // SAFETY: we know that the `specifiers.specifiers().len() == 1`, so unwrap `iter().next()` is safe.
            let first_specifier = specifiers.elements().next().unwrap();
            if let (Ok(specifier), Ok(separator)) =
                (first_specifier.node(), first_specifier.trailing_separator())
            {
                if f.comments().has_comments(specifier.syntax()) {
                    write!(f, [named_specifiers.format()])
                } else {
                    let JsNamedImportSpecifiersFields {
                        l_curly_token,
                        specifiers: _,
                        r_curly_token,
                    } = named_specifiers.as_fields();
                    let should_insert_space_around_brackets = f.options().bracket_spacing().value();

                    write!(
                        f,
                        [
                            l_curly_token.format(),
                            maybe_space(should_insert_space_around_brackets),
                            specifier.format(),
                        ]
                    )?;

                    if let Some(separator) = separator {
                        format_removed(separator).fmt(f)?;
                    }

                    write!(
                        f,
                        [
                            maybe_space(should_insert_space_around_brackets),
                            r_curly_token.format()
                        ]
                    )
                }
            } else {
                write![f, [named_specifiers.format()]]
            }
        } else {
            write![f, [named_specifiers.format()]]
        }?;

        write![f, [space(), from_token.format(), space(), source.format(),]]?;

        if let Some(assertion) = assertion {
            write!(f, [assertion.format()])?;
        }

        Ok(())
    }
}
