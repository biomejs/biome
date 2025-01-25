use biome_analyze::QueryMatch;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsImportClause, AnyJsModuleItem, AnyJsNamedImportSpecifier, JsImport, JsLanguage, JsModule,
    JsSyntaxToken, JsSyntaxTrivia, T,
};
use biome_rowan::{
    chain_trivia_pieces, AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutation,
    SyntaxTriviaPiece, TextRange, TokenText, TriviaPiece, TriviaPieceKind,
};
use std::{cell::Cell, cmp::Ordering, collections::BTreeMap, iter};

pub(crate) fn run(root: &JsModule) -> Option<ImportGroups> {
    let mut groups = Vec::new();
    let mut first_node = None;
    let mut nodes = BTreeMap::new();

    for item in root.items() {
        let AnyJsModuleItem::JsImport(import) = item else {
            // If we have pending nodes and encounter a non-import node, append the nodes to a new group
            if let Some(first_node) = first_node.take() {
                groups.push(ImportGroup {
                    first_node,
                    nodes: std::mem::take(&mut nodes),
                });
            }
            continue;
        };

        let is_side_effect_import = matches!(
            import.import_clause(),
            Ok(AnyJsImportClause::JsImportBareClause(_))
        );
        if is_side_effect_import {
            if let Some(first_node) = first_node.take() {
                groups.push(ImportGroup {
                    first_node,
                    nodes: std::mem::take(&mut nodes),
                });
            }
            // A side effect import creates its own import group
            let mut nodes = BTreeMap::new();
            nodes.insert(
                ImportKey(import.source_text().ok()?),
                vec![ImportNode::from(import.clone())],
            );
            groups.push(ImportGroup {
                first_node: import.clone(),
                nodes,
            });
            continue;
        }

        // If this is not the first import in the group, check for a group break
        if has_empty_line(&import.import_token().ok()?.leading_trivia()) {
            if let Some(first_node) = first_node.take() {
                groups.push(ImportGroup {
                    first_node,
                    nodes: std::mem::take(&mut nodes),
                });
            }
        }

        // If this is the first import in the group save the leading trivia
        // and slot index
        if first_node.is_none() {
            first_node = Some(import.clone());
        }

        nodes
            .entry(ImportKey(import.source_text().ok()?))
            .or_default()
            .push(ImportNode::from(import));
    }

    // Flush the remaining nodes
    if let Some(first_node) = first_node.take() {
        groups.push(ImportGroup { first_node, nodes });
    }

    groups
        .iter()
        .any(|group| !group.is_sorted())
        .then_some(ImportGroups { groups })
}

pub(crate) fn action(
    root: &JsModule,
    groups: &ImportGroups,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let mut groups_iter = groups.groups.iter();
    let mut next_group = groups_iter.next().expect("state is empty");

    let old_list = root.items();
    let mut new_list = Vec::new();

    let mut items_iter = old_list.iter();
    let mut iter = (&mut items_iter).enumerate();

    // Iterate other the nodes of the old list
    while let Some((item_slot, item)) = iter.next() {
        // If the current position in the old list is lower than the start
        // of the new group, append the old node to the new list
        if item_slot < next_group.first_node.syntax().index() {
            new_list.push(item);
            continue;
        }

        // Extract the leading trivia for the whole group from the leading
        // trivia for the import token of the first node in the group. If
        // the trivia contains empty lines the leading trivia for the group
        // comprise all trivia pieces coming before the empty line that's
        // closest to the token. Otherwise the group leading trivia is
        // created from all the newline and whitespace pieces on the first
        // token before the first comment or skipped piece.
        let group_first_token = next_group.first_node.import_token().ok()?;
        let group_leading_trivia = group_first_token.leading_trivia();

        let mut prev_newline = None;
        let mut group_leading_trivia: Vec<_> = group_leading_trivia
            .pieces()
            .enumerate()
            .rev()
            .find_map(|(index, piece)| {
                if piece.is_whitespace() {
                    return None;
                }

                let is_newline = piece.is_newline();
                if let Some(first_newline) = prev_newline.filter(|_| is_newline) {
                    return Some(first_newline + 1);
                }

                prev_newline = is_newline.then_some(index);
                None
            })
            .map_or_else(
                || {
                    group_leading_trivia
                        .pieces()
                        .take_while(is_ascii_whitespace)
                        .collect()
                },
                |length| group_leading_trivia.pieces().take(length).collect(),
            );

        let mut saved_leading_trivia = Vec::new();
        let group_leading_pieces = group_leading_trivia.len();

        let nodes_iter = next_group
            .nodes
            .values()
            // TODO: Try to merge nodes from the same source
            .flat_map(|nodes| nodes.iter())
            .enumerate();

        for (node_index, import_node) in nodes_iter {
            // For each node in the group, pop an item from the old list
            // iterator (ignoring `item` itself) and discard it
            if node_index > 0 {
                iter.next()
                    .unwrap_or_else(|| panic!("missing node {item_slot} {node_index}"));
            }

            let first_token = import_node.node.import_token().ok()?;
            let mut node = import_node.build_sorted_node();

            if node_index == 0 && group_first_token != first_token {
                // If this node was not previously in the leading position
                // but is being moved there, replace its leading whitespace
                // with the group's leading trivia
                let group_leading_trivia = group_leading_trivia.drain(..);
                let mut token_leading_trivia = first_token.leading_trivia().pieces().peekable();

                // Save off the leading whitespace of the token to be
                // reused by the import take the place of this node in the list
                while let Some(piece) = token_leading_trivia.next_if(is_ascii_whitespace) {
                    saved_leading_trivia.push(piece);
                }

                node = node.with_import_token(first_token.with_leading_trivia_pieces(
                    chain_trivia_pieces(group_leading_trivia, token_leading_trivia),
                ));
            } else if node_index > 0 && group_first_token == first_token {
                // If this node used to be in the leading position but
                // got moved, remove the group leading trivia from its
                // first token
                let saved_leading_trivia = saved_leading_trivia.drain(..);
                let token_leading_trivia = first_token
                    .leading_trivia()
                    .pieces()
                    .skip(group_leading_pieces);

                node = node.with_import_token(first_token.with_leading_trivia_pieces(
                    chain_trivia_pieces(saved_leading_trivia, token_leading_trivia),
                ));
            }

            new_list.push(AnyJsModuleItem::JsImport(node));
        }

        // Load the next group before moving on to the next item in the old
        // list, breaking the loop if there a no remaining groups to insert
        next_group = match groups_iter.next() {
            Some(entry) => entry,
            None => break,
        };
    }

    // Append all remaining nodes to the new list if the loop performed an
    // early exit after reaching the last group
    new_list.extend(items_iter);

    let new_list = make::js_module_item_list(new_list);

    mutation.replace_node_discard_trivia(old_list, new_list);

    Some(())
}

#[derive(Debug)]
pub struct ImportGroups {
    /// The list of all the import groups in the file
    groups: Vec<ImportGroup>,
}

#[derive(Debug)]
struct ImportGroup {
    /// The import that was at the start of the group before sorting
    first_node: JsImport,
    /// Multimap storing all the imports for each import source in the group,
    /// sorted in natural order
    nodes: BTreeMap<ImportKey, Vec<ImportNode>>,
}

impl ImportGroup {
    /// Returns true if the nodes in the group are already sorted in the file
    fn is_sorted(&self) -> bool {
        // The imports are sorted if the text position of each node in the `BTreeMap`
        // (sorted in natural order) is higher than the previous item in
        // the sequence
        let mut iter = self.nodes.values().flat_map(|nodes| nodes.iter());
        let Some(import_node) = iter.next() else {
            return true;
        };
        let mut previous_start = import_node.node.syntax().text_range().end();
        import_node.is_sorted()
            && iter.all(|import_node| {
                let start = import_node.node.syntax().text_range().end();
                let is_sorted = previous_start < start && import_node.is_sorted();
                previous_start = start;
                is_sorted
            })
    }
}

#[derive(Debug)]
struct ImportNode {
    /// The original `JsImport` node this import node was created from
    node: JsImport,
    /// The number of separators present in the named specifiers list of this node if it has one
    separator_count: usize,
    /// Map storing all the named import specifiers and their associated trailing separator,
    /// sorted in natural order
    specifiers: BTreeMap<ImportKey, (AnyJsNamedImportSpecifier, Option<JsSyntaxToken>)>,
}

impl From<JsImport> for ImportNode {
    fn from(node: JsImport) -> Self {
        let import_clause = node.import_clause().ok();

        let mut separator_count = 0;
        let specifiers = import_clause.and_then(|import_clause| {
            let AnyJsImportClause::JsImportNamedClause(import_named_clause) = import_clause else {
                return None;
            };
            let named_import_specifiers = import_named_clause.named_specifiers().ok()?;
            let mut result = BTreeMap::new();

            for element in named_import_specifiers.specifiers().elements() {
                let node = element.node.ok()?;
                let key = node.imported_name()?.token_text_trimmed();

                let trailing_separator = element.trailing_separator.ok()?;
                separator_count += usize::from(trailing_separator.is_some());

                result.insert(ImportKey(key), (node, trailing_separator));
            }

            Some(result)
        });

        Self {
            node,
            separator_count,
            specifiers: specifiers.unwrap_or_default(),
        }
    }
}

impl ImportNode {
    /// Returns `true` if the named import specifiers of this import node are sorted
    fn is_sorted(&self) -> bool {
        let mut iter = self
            .specifiers
            .values()
            .map(|(node, _)| node.syntax().text_range().start());
        let mut previous_start = iter.next().unwrap_or_default();
        iter.all(|start| {
            let is_sorted = previous_start < start;
            previous_start = start;
            is_sorted
        })
    }

    /// Build a clone of the original node this import node was created from with its import specifiers sorted
    fn build_sorted_node(&self) -> JsImport {
        let import = self.node.clone().detach();

        let import_clause = import.import_clause();
        let Ok(AnyJsImportClause::JsImportNamedClause(import_named_clause)) = import_clause else {
            return import;
        };
        let Ok(old_specifiers) = import_named_clause.named_specifiers() else {
            return import;
        };

        let element_count = self.specifiers.len();
        let last_element = element_count.saturating_sub(1);
        let separator_count = self.separator_count.max(last_element);
        let needs_newline: Cell<Option<Option<JsSyntaxToken>>> = Cell::new(None);

        let items = self
            .specifiers
            .values()
            .enumerate()
            .map(|(index, (node, sep))| {
                let is_last = index == last_element;

                let mut node = node.clone().detach();
                let Some(prev_token) = node.syntax().last_token() else {
                    return node;
                };

                if let Some(sep) = sep {
                    if is_last && separator_count == last_element {
                        // If this is the last item and we are removing its trailing separator,
                        // move the trailing trivia from the separator to the node
                        let next_token =
                            prev_token.append_trivia_pieces(sep.trailing_trivia().pieces());

                        node = node
                            .replace_token_discard_trivia(prev_token, next_token)
                            .expect("prev_token should be a child of node");
                    }
                } else if !is_last {
                    // If the node has no separator and this is not the last item,
                    // remove the trailing trivia since it will get cloned on the inserted separator
                    let next_token = prev_token.with_trailing_trivia([]);
                    node = node
                        .replace_token_discard_trivia(prev_token, next_token)
                        .expect("prev_token should be a child of node");
                }

                // Check if the last separator we emitted ended with a single-line comment
                if let Some(newline_source) = needs_newline.take() {
                    if let Some(first_token) = node.syntax().first_token() {
                        if let Some(new_token) =
                            prepend_leading_newline(&first_token, newline_source)
                        {
                            node = node
                                .replace_token_discard_trivia(first_token, new_token)
                                .expect("first_token should be a child of node");
                        }
                    }
                }

                node
            });

        let separators = self
            .specifiers
            .values()
            .take(separator_count)
            .map(|(node, sep)| {
                // If this entry has an associated separator, reuse it
                let (token, will_need_newline) = if let Some(sep) = sep {
                    // If the last trivia piece for the separator token is a single-line comment,
                    // signal to the items iterator it will need to prepend a newline to the leading
                    // trivia of the next node
                    let will_need_newline = sep
                        .trailing_trivia()
                        .last()
                        .is_some_and(|piece| piece.kind().is_single_line_comment());

                    (sep.clone(), will_need_newline)
                } else {
                    // If the node we're attaching this separator to has no trailing trivia, just create a simple comma token
                    let last_trailing_trivia = match node.syntax().last_trailing_trivia() {
                        Some(trivia) if !trivia.is_empty() => trivia,
                        _ => {
                            let sep = make::token(T![,]);
                            return if node.syntax().has_leading_newline() {
                                sep
                            } else {
                                sep.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                            };
                        }
                    };

                    // Otherwise we need to clone the trailing trivia from the node to the separator
                    // (the items iterator should have already filtered this trivia when it previously
                    // emitted the node)
                    let mut text = String::from(",");
                    let mut trailing = Vec::with_capacity(last_trailing_trivia.pieces().len());

                    let mut will_need_newline = false;
                    for piece in last_trailing_trivia.pieces() {
                        text.push_str(piece.text());
                        trailing.push(TriviaPiece::new(piece.kind(), piece.text_len()));
                        will_need_newline =
                            matches!(piece.kind(), TriviaPieceKind::SingleLineComment);
                    }

                    let token = JsSyntaxToken::new_detached(T![,], &text, [], trailing);
                    (token, will_need_newline)
                };

                // If the last trivia piece was a single-line comment, signal to the items iterator
                // it will need to prepend a newline to the leading trivia of the next node, and provide
                // it the token that followed this separator in the original source so the newline trivia
                // can be cloned from there
                let newline_source =
                    will_need_newline.then(|| sep.as_ref().and_then(|token| token.next_token()));

                needs_newline.set(newline_source);

                token
            });

        let mut new_specifiers = old_specifiers
            .clone()
            .detach()
            .with_specifiers(make::js_named_import_specifier_list(items, separators));

        // If the separators iterator has a pending newline, prepend it to closing curly token
        if let Some(newline_source) = needs_newline.into_inner() {
            let new_token = new_specifiers
                .r_curly_token()
                .ok()
                .and_then(|token| prepend_leading_newline(&token, newline_source));

            if let Some(new_token) = new_token {
                new_specifiers = new_specifiers.with_r_curly_token(new_token);
            }
        }

        import
            .replace_node_discard_trivia(old_specifiers, new_specifiers)
            .expect("old_specifiers should be a child of import")
    }
}

/// Return a clone of `prev_token` with a newline trivia piece prepended to its
/// leading trivia if it didn't have one already. This function will try to copy
/// the newline trivia piece from the leading trivia of `newline_source` if its set
fn prepend_leading_newline(
    prev_token: &JsSyntaxToken,
    newline_source: Option<JsSyntaxToken>,
) -> Option<JsSyntaxToken> {
    // Check if this node already starts with a newline,
    // if it does we don't need to prepend anything
    let leading_trivia = prev_token.leading_trivia();
    let has_leading_newline = leading_trivia
        .first()
        .is_some_and(|piece| piece.is_newline());

    if has_leading_newline {
        return None;
    }

    // Extract the leading newline from the `newline_source` token
    let leading_newline = newline_source.and_then(|newline_source| {
        let leading_piece = newline_source.leading_trivia().first()?;
        if !leading_piece.is_newline() {
            return None;
        }
        Some(leading_piece)
    });

    // Prepend a newline trivia piece to the node, either by copying the leading newline
    // and whitespace from `newline_source`, or falling back to the "\n" character
    let leading_newline = if let Some(leading_newline) = &leading_newline {
        (leading_newline.kind(), leading_newline.text())
    } else {
        (TriviaPieceKind::Newline, "\n")
    };

    let piece_count = 1 + leading_trivia.pieces().len();
    let mut iter = iter::once(leading_newline).chain(leading_trivia_iter(prev_token));

    Some(prev_token.with_leading_trivia((0..piece_count).map(|_| iter.next().unwrap())))
}

/// Builds an iterator over the leading trivia pieces of a token
///
/// The items of the iterator inherit their lifetime from the token,
/// rather than the trivia pieces themselves
fn leading_trivia_iter(
    token: &JsSyntaxToken,
) -> impl ExactSizeIterator<Item = (TriviaPieceKind, &str)> {
    let token_text = token.text();
    let token_range = token.text_range();
    let trivia = token.leading_trivia();
    trivia.pieces().map(move |piece| {
        let piece_range = piece.text_range();
        let range = TextRange::at(piece_range.start() - token_range.start(), piece_range.len());

        let text = &token_text[range];
        assert_eq!(text, piece.text());

        (piece.kind(), text)
    })
}

#[derive(Debug)]
struct ImportKey(TokenText);

impl Ord for ImportKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let own_category = ImportCategory::from(self.0.text());
        let other_category = ImportCategory::from(other.0.text());
        if own_category != other_category {
            return own_category.cmp(&other_category);
        }

        // Sort imports using natural ordering
        natord::compare(&self.0, &other.0)
    }
}

impl PartialOrd for ImportKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ImportKey {}

impl PartialEq for ImportKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Imports get sorted by categories before being sorted on natural order.
///
/// The rationale for this is that imports "further away" from the source file
/// are listed before imports closer to the source file.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum ImportCategory {
    /// Anything with an explicit `bun:` prefix.
    Bun,
    /// Anything with an explicit `node:` prefix, or one of the recognized
    /// Node built-ins, such `"fs"`, `"child_process"`, etc..
    NodeBuiltin,
    /// NPM dependencies with an explicit `npm:` prefix, such as supported by
    /// Deno.
    Npm,
    /// Modules that contains the column `:` are usually considered "virtual modules". E.g. `astro:middleware`
    ///
    /// This modules are usually injected by the environment of the application, and usually present before any relative module.
    VirtualModule,
    /// Imports from an absolute URL such as supported by browsers.
    Url,
    /// Anything without explicit protocol specifier is assumed to be a library
    /// import. Because we currently do not have configuration for this, this
    /// may (incorrectly) include source imports through custom import mappings
    /// as well.
    Library,
    /// Absolute file imports `/<path>`.
    Absolute,
    /// Node allows specifying an import map with name prefixed with `#`.
    /// See https://nodejs.org/api/packages.html#subpath-imports
    SharpImport,
    /// Relative file imports `./<path>`.
    Relative,
}

impl From<&str> for ImportCategory {
    fn from(value: &str) -> Self {
        if value.starts_with('.') {
            Self::Relative
        } else if let Some((protocol, _)) = value.split_once(':') {
            match protocol {
                "bun" => Self::Bun,
                "http" | "https" => Self::Url,
                "node" => Self::NodeBuiltin,
                "npm" => Self::Npm,
                _ => Self::VirtualModule,
            }
        } else if value.starts_with('#') {
            Self::SharpImport
        } else if value.starts_with('/') {
            Self::Absolute
        } else if NODE_BUILTINS.binary_search(&value).is_ok() {
            Self::NodeBuiltin
        } else {
            Self::Library
        }
    }
}

/// Returns true is this trivia piece is "ASCII whitespace" (newline or whitespace)
fn is_ascii_whitespace(piece: &SyntaxTriviaPiece<JsLanguage>) -> bool {
    piece.is_newline() || piece.is_whitespace()
}

/// Returns true if the provided trivia contains an empty line (two consecutive newline pieces, ignoring whitespace)
fn has_empty_line(trivia: &JsSyntaxTrivia) -> bool {
    let mut was_newline = false;
    trivia
        .pieces()
        .filter(|piece| !piece.is_whitespace())
        .any(|piece| {
            let prev_newline = was_newline;
            was_newline = piece.is_newline();
            prev_newline && was_newline
        })
}

/// Sorted array of Node builtin
const NODE_BUILTINS: &[&str] = &[
    "assert",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "https",
    "module",
    "net",
    "os",
    "path",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "tty",
    "url",
    "util",
    "vm",
    "zlib",
];

#[test]
fn test_order() {
    for items in NODE_BUILTINS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
