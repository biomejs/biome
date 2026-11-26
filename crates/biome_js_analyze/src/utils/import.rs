use biome_js_factory::make;
use biome_js_syntax::{AnyJsNamedImportSpecifier, JsNamedImportSpecifierList, T};
use biome_rowan::{AstNode, AstSeparatedList};

/// Returns a copy of `specifiers` with `specifier` added after the last
/// specifier.
///
/// The new specifier takes over the trailing trivia of the specifier it
/// follows, and a trailing comma stays at the end of the list.
pub(crate) fn append_specifier(
    specifiers: &JsNamedImportSpecifierList,
    specifier: AnyJsNamedImportSpecifier,
) -> Option<JsNamedImportSpecifierList> {
    let Some(last) = specifiers.last() else {
        return Some(make::js_named_import_specifier_list([specifier], []));
    };
    let last = last.ok()?;
    let index = last.syntax().index();
    let specifier =
        specifier.with_trailing_trivia_pieces(last.syntax().last_trailing_trivia()?.pieces())?;
    let last = last.with_trailing_trivia_pieces([])?;
    JsNamedImportSpecifierList::cast(specifiers.syntax().clone().splice_slots(
        index..=index,
        [
            Some(last.into_syntax().into()),
            Some(make::token(T![,]).with_trailing_space().into()),
            Some(specifier.into_syntax().into()),
        ],
    ))
}
