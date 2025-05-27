use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBinding, AnyJsImportAssertionEntry, JsExportNamedFromSpecifierList, JsImportAssertion,
    JsNamedImportSpecifiers, T, inner_string_text,
};
use biome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, Language, SyntaxToken, TriviaPieceKind,
    chain_trivia_pieces,
};

use super::comparable_token::ComparableToken;

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
    let mut is_sorted = true;
    let specifiers = named_specifiers.specifiers();
    if specifiers.len() > 1 {
        let mut prev_node_key = None;
        for AstSeparatedElement {
            node,
            trailing_separator,
        } in specifiers.elements()
        {
            // We have to check if the separator is not buggy.
            let _separator = trailing_separator.ok()?;
            let node = node.ok()?;
            let Some(AnyJsBinding::JsIdentifierBinding(name)) = node.local_name() else {
                return None;
            };
            let name = ComparableToken(name.name_token().ok()?.token_text_trimmed());
            if prev_node_key.is_some_and(|prev_node_key| prev_node_key > name) {
                // We don't return early because we want to return `None` if we met any error.
                is_sorted = false;
            }
            prev_node_key = Some(name);
        }
    }
    Some(is_sorted)
}

pub fn sort_import_specifiers(
    named_specifiers: JsNamedImportSpecifiers,
) -> Option<JsNamedImportSpecifiers> {
    let list = named_specifiers.specifiers();
    let mut last_has_separator = false;
    let mut sorted = Vec::with_capacity(list.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in list.elements()
    {
        let separator = trailing_separator.ok()?;
        let node = node.ok()?;
        let AnyJsBinding::JsIdentifierBinding(name) = node.local_name()? else {
            return None;
        };
        let node_key = ComparableToken(name.name_token().ok()?.token_text_trimmed());
        last_has_separator = separator.is_some();
        sorted.push((node_key, node, separator));
    }
    sorted.sort_unstable_by(|(key1, _, _), (key2, _, _)| key1.cmp(key2));
    handle_trvia(
        sorted.iter_mut().map(|(_, a, b)| (a, b)),
        last_has_separator,
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    );
    let separators: Vec<_> = sorted
        .iter_mut()
        .filter_map(|(_, _, sep)| sep.take())
        .collect();
    let nodes = sorted.into_iter().map(|(_, specifier, _)| specifier);
    let new_list = make::js_named_import_specifier_list(nodes, separators);
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
    let mut is_sorted = true;
    if specifiers.len() > 1 {
        let mut prev_node_key = None;
        for AstSeparatedElement {
            node,
            trailing_separator,
        } in specifiers.elements()
        {
            // We have to check if the separator is not buggy.
            let _separator = trailing_separator.ok()?;
            let node = node.ok()?;
            let node_key = ComparableToken(node.source_name().ok()?.inner_string_text().ok()?);
            if prev_node_key.is_some_and(|prev_node_key| prev_node_key > node_key) {
                // We don't return early because we want to return `None` if we met any error.
                is_sorted = false;
            }
            prev_node_key = Some(node_key);
        }
    }
    Some(is_sorted)
}

pub fn sort_export_specifiers(
    named_specifiers: &JsExportNamedFromSpecifierList,
) -> Option<JsExportNamedFromSpecifierList> {
    let mut last_has_separator = false;
    let mut sorted = Vec::with_capacity(named_specifiers.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in named_specifiers.elements()
    {
        let node = node.ok()?;
        let separator = trailing_separator.ok()?;
        let name = node.source_name().ok()?.inner_string_text().ok()?;
        let node_key = ComparableToken(name);
        last_has_separator = separator.is_some();
        sorted.push((node_key, node, separator));
    }
    sorted.sort_unstable_by(|(key1, _, _), (key2, _, _)| key1.cmp(key2));
    handle_trvia(
        sorted.iter_mut().map(|(_, a, b)| (a, b)),
        last_has_separator,
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    );
    let separators: Vec<_> = sorted
        .iter_mut()
        .filter_map(|(_, _, sep)| sep.take())
        .collect();
    let nodes = sorted.into_iter().map(|(_, node, _)| node);
    Some(make::js_export_named_from_specifier_list(nodes, separators))
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
    let mut is_sorted = true;
    let attributes_list = attributes.assertions();
    if attributes_list.len() > 1 {
        let mut prev_node_key: Option<ComparableToken> = None;
        for AstSeparatedElement {
            node,
            trailing_separator,
        } in attributes_list.elements()
        {
            let Ok(AnyJsImportAssertionEntry::JsImportAssertionEntry(node)) = node else {
                return None;
            };
            // We have to check if the separator is not buggy.
            let _separator = trailing_separator.ok()?;
            let node_key = ComparableToken(inner_string_text(&node.key().ok()?));
            if prev_node_key.is_some_and(|prev_node| prev_node > node_key) {
                // We don't return early because we want to return `None` if we met any error.
                is_sorted = false;
            }
            prev_node_key = Some(node_key);
        }
    }
    Some(is_sorted)
}

pub fn sort_attributes(attributes: JsImportAssertion) -> Option<JsImportAssertion> {
    let attributes_list = attributes.assertions();
    let mut last_has_separator = false;
    let mut sorted = Vec::new();
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in attributes_list.elements()
    {
        let separator = trailing_separator.ok()?;
        let Ok(AnyJsImportAssertionEntry::JsImportAssertionEntry(node)) = node else {
            return None;
        };
        let node_key = ComparableToken(inner_string_text(&node.key().ok()?));
        last_has_separator = separator.is_some();
        sorted.push((node_key, node, separator));
    }
    sorted.sort_unstable_by(|(key1, _, _), (key2, _, _)| key1.cmp(key2));
    handle_trvia(
        sorted.iter_mut().map(|(_, a, b)| (a, b)),
        last_has_separator,
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    );
    let separators: Vec<_> = sorted
        .iter_mut()
        .filter_map(|(_, _, sep)| sep.take())
        .collect();
    let nodes = sorted
        .into_iter()
        .map(|(_, node, _)| AnyJsImportAssertionEntry::JsImportAssertionEntry(node));
    Some(attributes.with_assertions(make::js_import_assertion_entry_list(nodes, separators)))
}

fn handle_trvia<'a, L: Language + 'a, N: AstNode<Language = L> + 'a>(
    // Mutable iterator of a list of nodes and their optional separators
    iter: impl std::iter::ExactSizeIterator<Item = (&'a mut N, &'a mut Option<SyntaxToken<L>>)>,
    needs_last_separator: bool,
    make_separator: fn() -> SyntaxToken<L>,
) {
    let last_index = iter.len().saturating_sub(1);
    for (i, (node, optional_separator)) in iter.enumerate() {
        if let Some(separator) = optional_separator {
            // Remove the last separator at the separator has no attached comments
            if i == last_index
                && !(needs_last_separator
                    || separator.has_leading_comments()
                    || separator.has_trailing_comments())
            {
                // Transfer the separator trivia
                if let Some(new_node) = node.clone().append_trivia_pieces(chain_trivia_pieces(
                    separator.leading_trivia().pieces(),
                    separator.trailing_trivia().pieces(),
                )) {
                    *node = new_node;
                }
                *optional_separator = None;
            }
        } else if i != last_index || needs_last_separator {
            // The last node is moved and has no trailing separator.
            // Thus we build a new separator and remove its trailing whitespaces.
            if let Some(new_node) = node.clone().trim_trailing_trivia() {
                *node = new_node;
            }
            *optional_separator = Some(make_separator());
        }
    }
}
