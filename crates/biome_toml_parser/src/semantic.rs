//! Detects keys and tables that a TOML document defines more than once.
//!
//! The specification forbids:
//! - defining a key twice;
//! - defining a table twice with `[table]` headers;
//! - defining a table with a `[table]` header after dotted keys defined it,
//!   and extending a table defined by a `[table]` header with dotted keys;
//! - adding keys to an inline table or appending to a static array;
//! - mixing a table and an array of tables under the same name.
//!
//! Tables created implicitly, by a header such as `[a.b.c]` for `a` and
//! `a.b`, can still be defined once by a header or extended by dotted keys.
//!
//! The parser feeds keys to [`KeyTracker`] as it parses them, so key names
//! borrow the source text and no syntax tree traversal is needed. All entries
//! live in a single map keyed by the containing table and the key name.

use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::collections::hash_map::Entry as MapEntry;

/// A decoded part of a key, such as `b` in `a."b".c`.
#[derive(Debug, Clone)]
pub(crate) struct KeyPart<'src> {
    pub name: Cow<'src, str>,
    pub range: TextRange,
}

type TableId = u32;

/// The table that contains the whole document.
const DOCUMENT: TableId = 0;

/// How a table came into existence.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TableKind {
    /// Created as the parent of a header, such as `a` for `[a.b]`. A header
    /// can still define it.
    Implicit,
    /// Defined by a `[table]` or `[[table]]` header.
    Header,
    /// Defined by dotted keys, such as `a` for `a.b = 1`, or an inline table.
    Dotted,
}

#[derive(Debug)]
struct Table {
    kind: TableKind,
    /// The key that defined the table.
    range: TextRange,
}

#[derive(Debug, Copy, Clone)]
enum Entry {
    /// A value that can't be extended: a scalar, an array, or an inline table.
    Value {
        range: TextRange,
    },
    Table(TableId),
    /// Tables defined by `[[array]]` headers. Only the last table can be
    /// extended by later headers.
    ArrayOfTables {
        range: TextRange,
        last: TableId,
    },
}

pub(crate) struct KeyTracker<'src> {
    tables: Vec<Table>,
    entries: FxHashMap<(TableId, Cow<'src, str>), Entry>,
    /// The tables that receive key/value pairs: the current table section,
    /// followed by the inline tables being parsed.
    scopes: Vec<TableId>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl Default for KeyTracker<'_> {
    fn default() -> Self {
        Self {
            tables: vec![Table {
                kind: TableKind::Header,
                range: TextRange::default(),
            }],
            entries: FxHashMap::default(),
            scopes: vec![DOCUMENT],
            diagnostics: Vec::new(),
        }
    }
}

impl<'src> KeyTracker<'src> {
    pub fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Starts the section of a `[table]` header with the given key parts.
    ///
    /// Pass `None` if the key is invalid. The key/value pairs of a rejected
    /// section are still checked against each other.
    pub fn start_table(&mut self, parts: Option<&[KeyPart<'src>]>, range: TextRange) {
        let section = parts
            .and_then(|parts| self.define_table(parts, range))
            .unwrap_or_else(|| self.new_table(TableKind::Header, range));
        self.scopes.clear();
        self.scopes.push(section);
    }

    /// Starts the section of a `[[table]]` header with the given key parts.
    pub fn start_array_table(&mut self, parts: Option<&[KeyPart<'src>]>, range: TextRange) {
        let section = parts
            .and_then(|parts| self.append_array_table(parts, range))
            .unwrap_or_else(|| self.new_table(TableKind::Header, range));
        self.scopes.clear();
        self.scopes.push(section);
    }

    /// Starts an inline table. Its key/value pairs go to a new table until
    /// [Self::end_inline_table].
    pub fn start_inline_table(&mut self, range: TextRange) {
        let table = self.new_table(TableKind::Dotted, range);
        self.scopes.push(table);
    }

    pub fn end_inline_table(&mut self) {
        debug_assert!(self.scopes.len() > 1);
        self.scopes.pop();
    }

    /// Defines the key of a key/value pair in the current scope.
    pub fn define_key_value(&mut self, parts: &[KeyPart<'src>]) {
        let Some((last, parents)) = parts.split_last() else {
            return;
        };

        let mut table = self.scopes.last().copied().unwrap_or(DOCUMENT);
        for (index, part) in parents.iter().enumerate() {
            let entry = match self.entries.entry((table, part.name.clone())) {
                MapEntry::Vacant(vacant) => {
                    let id = self.tables.len() as TableId;
                    self.tables.push(Table {
                        kind: TableKind::Dotted,
                        range: part.range,
                    });
                    vacant.insert(Entry::Table(id));
                    table = id;
                    continue;
                }
                MapEntry::Occupied(occupied) => *occupied.get(),
            };

            table = match entry {
                Entry::Table(id) if self.tables[id as usize].kind == TableKind::Header => {
                    let range = self.tables[id as usize].range;
                    self.report(
                        part.range,
                        format!(
                            "The table `{}` is defined by a header and can't be extended with dotted keys.",
                            key_text(&parts[..=index])
                        ),
                        range,
                        "The table is defined here.",
                        None,
                    );
                    return;
                }
                Entry::Table(id) => {
                    self.tables[id as usize].kind = TableKind::Dotted;
                    id
                }
                Entry::ArrayOfTables { range, .. } => {
                    self.report(
                        part.range,
                        format!(
                            "The key `{}` is an array of tables and can't be extended with dotted keys.",
                            key_text(&parts[..=index])
                        ),
                        range,
                        "The array of tables is defined here.",
                        None,
                    );
                    return;
                }
                Entry::Value { range } => {
                    self.report_value_conflict(part.range, &parts[..=index], range);
                    return;
                }
            };
        }

        let existing = match self.entries.entry((table, last.name.clone())) {
            MapEntry::Vacant(vacant) => {
                vacant.insert(Entry::Value { range: last.range });
                None
            }
            MapEntry::Occupied(occupied) => Some(*occupied.get()),
        };

        if let Some(existing) = existing {
            let range = self.entry_range(existing);
            self.report(
                last.range,
                format!("The key `{}` is defined more than once.", key_text(parts)),
                range,
                "The key is first defined here.",
                None,
            );
        }
    }

    fn new_table(&mut self, kind: TableKind, range: TextRange) -> TableId {
        let id = self.tables.len() as TableId;
        self.tables.push(Table { kind, range });
        id
    }

    fn entry_range(&self, entry: Entry) -> TextRange {
        match entry {
            Entry::Value { range } | Entry::ArrayOfTables { range, .. } => range,
            Entry::Table(id) => self.tables[id as usize].range,
        }
    }

    /// Defines the table named by a `[table]` header and returns it.
    fn define_table(&mut self, parts: &[KeyPart<'src>], range: TextRange) -> Option<TableId> {
        let (last, parents) = parts.split_last()?;
        let parent = self.header_parent(parts, parents.len())?;

        let entry = match self.entries.entry((parent, last.name.clone())) {
            MapEntry::Vacant(vacant) => {
                let id = self.tables.len() as TableId;
                self.tables.push(Table {
                    kind: TableKind::Header,
                    range,
                });
                vacant.insert(Entry::Table(id));
                return Some(id);
            }
            MapEntry::Occupied(occupied) => *occupied.get(),
        };

        match entry {
            Entry::Table(id) => {
                let table = &mut self.tables[id as usize];
                match table.kind {
                    TableKind::Implicit => {
                        table.kind = TableKind::Header;
                        table.range = range;
                        Some(id)
                    }
                    TableKind::Header => {
                        let first = table.range;
                        self.report(
                            range,
                            format!("The table `{}` is defined more than once.", key_text(parts)),
                            first,
                            "The table is first defined here.",
                            None,
                        );
                        None
                    }
                    TableKind::Dotted => {
                        let first = table.range;
                        self.report(
                            range,
                            format!(
                                "The table `{}` is already defined by dotted keys.",
                                key_text(parts)
                            ),
                            first,
                            "The dotted keys define the table here.",
                            None,
                        );
                        None
                    }
                }
            }
            Entry::ArrayOfTables { range: first, .. } => {
                self.report(
                    range,
                    format!(
                        "The table `{}` is already defined as an array of tables.",
                        key_text(parts)
                    ),
                    first,
                    "The array of tables is first defined here.",
                    None,
                );
                None
            }
            Entry::Value { range: first } => {
                self.report_value_conflict(last.range, parts, first);
                None
            }
        }
    }

    /// Appends a table to the array of tables named by a `[[table]]` header
    /// and returns the new table.
    fn append_array_table(&mut self, parts: &[KeyPart<'src>], range: TextRange) -> Option<TableId> {
        let (last, parents) = parts.split_last()?;
        let parent = self.header_parent(parts, parents.len())?;
        let id = self.tables.len() as TableId;

        match self.entries.entry((parent, last.name.clone())) {
            MapEntry::Vacant(vacant) => {
                vacant.insert(Entry::ArrayOfTables { range, last: id });
            }
            MapEntry::Occupied(mut occupied) => match *occupied.get() {
                Entry::ArrayOfTables { range: first, .. } => {
                    occupied.insert(Entry::ArrayOfTables {
                        range: first,
                        last: id,
                    });
                }
                Entry::Table(table) => {
                    let first = self.tables[table as usize].range;
                    self.report(
                        range,
                        format!(
                            "The table `{}` can't be redefined as an array of tables.",
                            key_text(parts)
                        ),
                        first,
                        "The table is first defined here.",
                        None,
                    );
                    return None;
                }
                Entry::Value { range: first } => {
                    self.report(
                        range,
                        format!(
                            "The key `{}` is already defined as a value.",
                            key_text(parts)
                        ),
                        first,
                        "The value is defined here.",
                        Some("Tables can't be appended to a static array."),
                    );
                    return None;
                }
            },
        }

        self.tables.push(Table {
            kind: TableKind::Header,
            range,
        });
        Some(id)
    }

    /// Returns the table that contains the last part of a header key,
    /// creating implicit tables for the first `count` parts.
    fn header_parent(&mut self, parts: &[KeyPart<'src>], count: usize) -> Option<TableId> {
        let mut table = DOCUMENT;
        for (index, part) in parts[..count].iter().enumerate() {
            let entry = match self.entries.entry((table, part.name.clone())) {
                MapEntry::Vacant(vacant) => {
                    let id = self.tables.len() as TableId;
                    self.tables.push(Table {
                        kind: TableKind::Implicit,
                        range: part.range,
                    });
                    vacant.insert(Entry::Table(id));
                    table = id;
                    continue;
                }
                MapEntry::Occupied(occupied) => *occupied.get(),
            };

            table = match entry {
                Entry::Table(id) => id,
                Entry::ArrayOfTables { last, .. } => last,
                Entry::Value { range } => {
                    self.report_value_conflict(part.range, &parts[..=index], range);
                    return None;
                }
            };
        }
        Some(table)
    }

    fn report_value_conflict(&mut self, range: TextRange, parts: &[KeyPart], first: TextRange) {
        self.report(
            range,
            format!(
                "The key `{}` is already defined as a value and can't be used as a table.",
                key_text(parts)
            ),
            first,
            "The value is defined here.",
            Some("Inline tables and arrays can't be extended after they are defined."),
        );
    }

    /// Reports a conflict at `range` with the definition at `first`.
    fn report(
        &mut self,
        range: TextRange,
        message: String,
        first: TextRange,
        first_message: &str,
        hint: Option<&str>,
    ) {
        let mut diagnostic = ParseDiagnostic::new(message, range).with_detail(first, first_message);
        if let Some(hint) = hint {
            diagnostic = diagnostic.with_hint(hint);
        }
        self.diagnostics.push(diagnostic);
    }
}

fn key_text(parts: &[KeyPart]) -> String {
    let mut text = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push('.');
        }
        text.push_str(&part.name);
    }
    text
}
