use std::cmp::Ordering;
use biome_analyze::utils::{is_separated_list_sorted_by, sorted_separated_list_by};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBinding, AnyJsImportAssertionEntry, JsExportNamedFromSpecifierList, JsImportAssertion,
    JsNamedImportSpecifiers, T, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedElement, AstSeparatedList, TriviaPieceKind};
use biome_string_case::comparable_token::ComparableToken;
use biome_rule_options::organize_imports::SortMode;

pub enum JsNamedSpecifiers {
    JsNamedImportSpecifiers(JsNamedImportSpecifiers),
    JsExportNamedFromSpecifierList(JsExportNamedFromSpecifierList),
}
impl JsNamedSpecifiers {
    pub fn are_sorted(&self, sort_mode: SortMode) -> bool {


        match self {
            Self::JsNamedImportSpecifiers(specifeirs) => are_import_specifiers_sorted(specifeirs, sort_mode),
            Self::JsExportNamedFromSpecifierList(specifeirs) => {
                are_export_specifiers_sorted(specifeirs, sort_mode)
            }
        }
        // Assume the import is already sorted if there are any bogus nodes, otherwise the `--write`
        // flag will cause infinite loop.
        .unwrap_or(true)
    }
}

pub fn are_import_specifiers_sorted(named_specifiers: &JsNamedImportSpecifiers, sort_mode: SortMode) -> Option<bool> {
    let comparator = match sort_mode {
        SortMode::Lexicographic => ComparableToken::lexicographic_cmp,
        SortMode::Natural => ComparableToken::ascii_nat_cmp,
    };

    is_separated_list_sorted_by(&named_specifiers.specifiers(), |node| {
        let AnyJsBinding::JsIdentifierBinding(name) = node.local_name()? else {
            return None;
        };
        Some(ComparableToken::new(
            name.name_token().ok()?.token_text_trimmed(),
        ))
    }, Some(comparator))
    .ok()
}

pub fn sort_import_specifiers(
    named_specifiers: JsNamedImportSpecifiers,
    sort_mode: SortMode,
) -> Option<JsNamedImportSpecifiers> {
    let comparator = get_comparator(sort_mode);
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
        Some(comparator)
    )
    .ok()?;
    Some(named_specifiers.with_specifiers(new_list))
}

pub fn merge_import_specifiers(
    named_specifiers1: JsNamedImportSpecifiers,
    named_specifiers2: &JsNamedImportSpecifiers,
    sort_mode: SortMode,
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
    sort_import_specifiers(named_specifiers1.with_specifiers(new_list), sort_mode)
}

pub fn are_export_specifiers_sorted(specifiers: &JsExportNamedFromSpecifierList, sort_mode: SortMode) -> Option<bool> {
    let comparator = get_comparator(sort_mode);

    is_separated_list_sorted_by(specifiers, |node| {
        node.source_name()
            .ok()?
            .inner_string_text()
            .ok()
            .map(ComparableToken::new)
    }, Some(comparator))
    .ok()
}

pub fn sort_export_specifiers(
    named_specifiers: &JsExportNamedFromSpecifierList,
    sort_mode: SortMode,
) -> Option<JsExportNamedFromSpecifierList> {
    let comparator = get_comparator(sort_mode);
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
    Some(comparator))
    .ok()?;
    Some(new_list)
}

pub fn merge_export_specifiers(
    specifiers1: &JsExportNamedFromSpecifierList,
    specifiers2: &JsExportNamedFromSpecifierList,
    sort_mode: SortMode,
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
        nodes, separators
    ), sort_mode)
}

pub fn are_import_attributes_sorted(attributes: &JsImportAssertion, sort_mode: SortMode) -> Option<bool> {
    let comparator = get_comparator(sort_mode);
    is_separated_list_sorted_by(&attributes.assertions(), |node| {
        let AnyJsImportAssertionEntry::JsImportAssertionEntry(node) = node else {
            return None;
        };
        Some(ComparableToken::new(inner_string_text(&node.key().ok()?)))
    }, Some(comparator))
    .ok()
}

pub fn sort_attributes(attributes: JsImportAssertion, sort_mode: SortMode) -> Option<JsImportAssertion> {
    let comparator = get_comparator(sort_mode);

    let new_list = sorted_separated_list_by(
        &attributes.assertions(),
        |node| {
            let AnyJsImportAssertionEntry::JsImportAssertionEntry(node) = node else {
                return None;
            };
            Some(ComparableToken::new(inner_string_text(&node.key().ok()?)))
        },
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        Some(comparator),
    )
    .ok()?;
    Some(attributes.with_assertions(new_list))
}

pub fn get_comparator(sort_mode: SortMode) -> fn(&ComparableToken, &ComparableToken) -> Ordering {
    let comparator = match sort_mode {
        SortMode::Lexicographic => ComparableToken::lexicographic_cmp,
        SortMode::Natural => ComparableToken::ascii_nat_cmp
    };
    comparator
}
