use biome_analyze::utils::{is_separated_list_sorted_by, sorted_separated_list_by};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBinding, AnyJsImportAssertionEntry, JsExportNamedFromSpecifierList, JsImportAssertion,
    JsNamedImportSpecifiers, T, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedElement, AstSeparatedList, TriviaPieceKind};
use biome_string_case::comparable_token::ComparableToken;

pub enum JsNamedSpecifiers {
    JsNamedImportSpecifiers(JsNamedImportSpecifiers),
    JsExportNamedFromSpecifierList(JsExportNamedFromSpecifierList),
}
impl JsNamedSpecifiers {
    pub fn are_sorted(&self) -> bool {
        match self {
            Self::JsNamedImportSpecifiers(specifeirs) => are_import_specifiers_sorted(specifeirs),
            Self::JsExportNamedFromSpecifierList(specifeirs) => {
                are_export_specifiers_sorted(specifeirs)
            }
        }
        // Assume the import is already sorted if there are any bogus nodes, otherwise the `--write`
        // flag will cause infinite loop.
        .unwrap_or(true)
    }
}

pub fn are_import_specifiers_sorted(named_specifiers: &JsNamedImportSpecifiers) -> Option<bool> {
    is_separated_list_sorted_by(&named_specifiers.specifiers(), |node| {
        let AnyJsBinding::JsIdentifierBinding(name) = node.local_name()? else {
            return None;
        };
        Some(ComparableToken::new(
            name.name_token().ok()?.token_text_trimmed(),
        ))
    })
    .ok()
}

pub fn sort_import_specifiers(
    named_specifiers: JsNamedImportSpecifiers,
) -> Option<JsNamedImportSpecifiers> {
    let new_list = sorted_separated_list_by(
        &named_specifiers.specifiers(),
        |node| {
            let AnyJsBinding::JsIdentifierBinding(name) = node.local_name()? else {
                return None;
            };
            Some(ComparableToken::new(
                name.name_token().ok()?.token_text_trimmed(),
            ))
        },
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .ok()?;
    Some(named_specifiers.with_specifiers(new_list))
}

pub fn merge_import_specifiers(
    named_specifiers1: JsNamedImportSpecifiers,
    named_specifiers2: &JsNamedImportSpecifiers,
) -> Option<JsNamedImportSpecifiers> {
    let specifiers1 = named_specifiers1.specifiers();
    let specifiers2 = named_specifiers2.specifiers();
    let mut nodes = Vec::with_capacity(specifiers1.len() + specifiers2.len());
    let mut separators = Vec::with_capacity(specifiers1.len() + specifiers2.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in specifiers1.elements()
    {
        let separator = trailing_separator.ok()?;
        let mut node = node.ok()?;
        if separator.is_none() {
            node = node.trim_trailing_trivia()?;
        }
        let separator = separator.unwrap_or_else(|| {
            make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
        });
        nodes.push(node);
        separators.push(separator);
    }
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in specifiers2.elements()
    {
        nodes.push(node.ok()?);
        if let Some(separator) = trailing_separator.ok()? {
            separators.push(separator);
        }
    }
    let new_list = make::js_named_import_specifier_list(nodes, separators);
    sort_import_specifiers(named_specifiers1.with_specifiers(new_list))
}

pub fn are_export_specifiers_sorted(specifiers: &JsExportNamedFromSpecifierList) -> Option<bool> {
    is_separated_list_sorted_by(specifiers, |node| {
        node.source_name()
            .ok()?
            .inner_string_text()
            .ok()
            .map(ComparableToken::new)
    })
    .ok()
}

pub fn sort_export_specifiers(
    named_specifiers: &JsExportNamedFromSpecifierList,
) -> Option<JsExportNamedFromSpecifierList> {
    let new_list = sorted_separated_list_by(
        named_specifiers,
        |node| {
            node.source_name()
                .ok()?
                .inner_string_text()
                .ok()
                .map(ComparableToken::new)
        },
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .ok()?;
    Some(new_list)
}

pub fn merge_export_specifiers(
    specifiers1: &JsExportNamedFromSpecifierList,
    specifiers2: &JsExportNamedFromSpecifierList,
) -> Option<JsExportNamedFromSpecifierList> {
    let mut nodes = Vec::with_capacity(specifiers1.len() + specifiers2.len());
    let mut separators = Vec::with_capacity(specifiers1.len() + specifiers2.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in specifiers1.elements()
    {
        let separator = trailing_separator.ok()?;
        let mut node = node.ok()?;
        if separator.is_none() {
            node = node.trim_trailing_trivia()?;
        }
        let separator = separator.unwrap_or_else(|| {
            make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
        });
        nodes.push(node);
        separators.push(separator);
    }
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in specifiers2.elements()
    {
        nodes.push(node.ok()?);
        if let Some(separator) = trailing_separator.ok()? {
            separators.push(separator);
        }
    }
    sort_export_specifiers(&make::js_export_named_from_specifier_list(
        nodes, separators,
    ))
}

pub fn are_import_attributes_sorted(attributes: &JsImportAssertion) -> Option<bool> {
    is_separated_list_sorted_by(&attributes.assertions(), |node| {
        let AnyJsImportAssertionEntry::JsImportAssertionEntry(node) = node else {
            return None;
        };
        Some(ComparableToken::new(inner_string_text(&node.key().ok()?)))
    })
    .ok()
}

pub fn sort_attributes(attributes: JsImportAssertion) -> Option<JsImportAssertion> {
    let new_list = sorted_separated_list_by(
        &attributes.assertions(),
        |node| {
            let AnyJsImportAssertionEntry::JsImportAssertionEntry(node) = node else {
                return None;
            };
            Some(ComparableToken::new(inner_string_text(&node.key().ok()?)))
        },
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .ok()?;
    Some(attributes.with_assertions(new_list))
}
