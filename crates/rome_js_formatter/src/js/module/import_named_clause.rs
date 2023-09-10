use crate::prelude::*;

use biome_js_syntax::AnyJsNamedImport;
use biome_js_syntax::AnyJsNamedImportSpecifier;
use biome_js_syntax::JsImportNamedClause;
use biome_js_syntax::JsImportNamedClauseFields;
use biome_js_syntax::JsNamedImportSpecifiersFields;
use rome_formatter::{write, CstFormatContext};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportNamedClause;

impl FormatNodeRule<JsImportNamedClause> for FormatJsImportNamedClause {
    fn fmt_fields(&self, node: &JsImportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        let is_default_specifier_empty = default_specifier.is_none();

        if let Some(default_specifier) = default_specifier {
            write!(f, [default_specifier.format(), space()])?;
        }

        let named_import = named_import?;

        // can_break implementation, return `format_element` instead of boolean to reduce enum conversion overhead.
        // if `can_break` is true we just use the previous format strategy, otherwise we use the new format strategy.
        // reference https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L173

        // https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L184-L209v,
        // `standaloneSpecifiers` corresponding our `JsDefaultImportSpecifier` + part of `JsNamespaceImportSpecifier`,
        // `groupedSpecifiers` corresponding our `JsNamedImportSpecifiers`

        //  Here we use an opposite way of thinking, we only thinking about the way that can not break
        // That's to say
        // 1. `default_specifier` need to be none.
        // 2. length of `JsNamedImportSpecifiers` at least is one
        // 3. Surrounding of the only `JsNamedImportSpecifiers` should not have any comments
        if !is_default_specifier_empty {
            // `can_break` is true.
            write![f, [named_import.format()]]
        } else {
            match named_import {
                AnyJsNamedImport::JsNamedImportSpecifiers(ref specifiers)
                    if specifiers.specifiers().len() == 1
                        && !f.context().comments().is_suppressed(specifiers.syntax()) =>
                {
                    // SAFETY: we know that the `specifiers.specifiers().len() == 1`, so unwrap `iter().next()` is safe.
                    let first_specifier = specifiers.specifiers().elements().next().unwrap();
                    match (first_specifier.node(), first_specifier.trailing_separator()) {
                        (
                            Ok(AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                                specifier,
                            )),
                            Ok(separator),
                        ) => {
                            if f.comments().has_comments(specifier.syntax()) {
                                write!(f, [named_import.format()])
                            } else {
                                let JsNamedImportSpecifiersFields {
                                    l_curly_token,
                                    specifiers: _,
                                    r_curly_token,
                                } = specifiers.as_fields();
                                write!(f, [l_curly_token.format(), space(), specifier.format(),])?;

                                if let Some(separator) = separator {
                                    format_removed(separator).fmt(f)?;
                                }

                                write!(f, [space(), r_curly_token.format()])
                            }
                        }
                        (
                            Ok(AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier)),
                            Ok(separator),
                        ) => {
                            if f.comments().has_comments(specifier.syntax()) {
                                write!(f, [named_import.format()])
                            } else {
                                let JsNamedImportSpecifiersFields {
                                    l_curly_token,
                                    specifiers: _,
                                    r_curly_token,
                                } = specifiers.as_fields();
                                write!(f, [l_curly_token.format(), space(), specifier.format(),])?;

                                if let Some(separator) = separator {
                                    format_removed(separator).fmt(f)?;
                                }

                                write!(f, [space(), r_curly_token.format()])
                            }
                        }
                        _ => write![f, [named_import.format()]],
                    }
                }
                _ => write![f, [named_import.format()]],
            }
        }?;

        write![f, [space(), from_token.format(), space(), source.format(),]]?;

        if let Some(assertion) = assertion {
            write!(f, [space(), assertion.format()])?;
        }

        Ok(())
    }
}
