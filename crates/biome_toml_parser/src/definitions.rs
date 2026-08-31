use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::{AstNode, TextRange};
use biome_toml_syntax::{
    AnyTomlItem, AnyTomlValue, TomlInlineTable, TomlKey, TomlKeyValue, TomlRoot, TomlSyntaxKind,
};
use std::collections::HashMap;

const ROOT_TABLE: TableId = 0;

type TableId = usize;

pub(crate) fn validate_definitions(root: &TomlRoot) -> Vec<ParseDiagnostic> {
    let mut validator = DefinitionValidator::new();
    let mut current_table = ROOT_TABLE;

    for item in root.items() {
        match item {
            AnyTomlItem::TomlKeyValue(key_value) => {
                validator.define_key_value(current_table, &key_value);
            }
            AnyTomlItem::TomlTable(table) => {
                if table.l_brack_token().is_ok()
                    && table.r_brack_token().is_ok()
                    && let Ok(key) = table.name()
                    && let Some(table_id) = validator.define_table(&key, false)
                {
                    current_table = table_id;
                }
            }
            AnyTomlItem::TomlArrayTable(table) => {
                if table.opening_outer_token().is_ok()
                    && table.opening_inner_token().is_ok()
                    && table.closing_inner_token().is_ok()
                    && table.closing_outer_token().is_ok()
                    && let Ok(key) = table.name()
                    && let Some(table_id) = validator.define_table(&key, true)
                {
                    current_table = table_id;
                }
            }
            AnyTomlItem::TomlBogus(_) => {}
        }
    }

    validator.diagnostics.sort_by_key(ParseDiagnostic::span);
    validator.diagnostics
}

struct DefinitionValidator {
    tables: Vec<Table>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl DefinitionValidator {
    fn new() -> Self {
        Self {
            tables: vec![Table::new(TableOrigin::Root)],
            diagnostics: Vec::new(),
        }
    }

    fn define_key_value(&mut self, table: TableId, key_value: &TomlKeyValue) {
        let Some(value) = self.record_key_value(table, key_value) else {
            return;
        };
        self.validate_value(value);
    }

    fn record_key_value(
        &mut self,
        table: TableId,
        key_value: &TomlKeyValue,
    ) -> Option<AnyTomlValue> {
        let (Ok(key), Ok(value)) = (key_value.key(), key_value.value()) else {
            return None;
        };
        let path = key_path(&key)?;
        let range = key.syntax().text_trimmed_range();

        self.define_key(table, &path, range);
        Some(value)
    }

    fn define_key(&mut self, mut table: TableId, path: &[String], range: TextRange) {
        let Some((key, parents)) = path.split_last() else {
            return;
        };

        for parent in parents {
            let Some(next_table) =
                self.resolve_parent(table, parent, range, TableOrigin::Dotted(range))
            else {
                return;
            };
            table = next_table;
        }

        match self.entry(table, key) {
            None => {
                self.tables[table]
                    .entries
                    .insert(key.clone(), Entry::Value(range));
            }
            Some(EntryRef::Value(previous)) => {
                self.error("This TOML key is already defined.", range, previous);
            }
            Some(EntryRef::Table(_, previous) | EntryRef::ArrayTable(_, previous)) => {
                self.error(
                    "This TOML key conflicts with a previous definition.",
                    range,
                    previous,
                );
            }
        }
    }

    fn define_table(&mut self, key: &TomlKey, array: bool) -> Option<TableId> {
        let path = key_path(key)?;
        let (name, parents) = path.split_last()?;
        let range = key.syntax().text_trimmed_range();
        let mut table = ROOT_TABLE;

        for parent in parents {
            table = self.resolve_parent(table, parent, range, TableOrigin::Implicit(range))?;
        }

        if array {
            self.define_array_table(table, name, range)
        } else {
            self.define_standard_table(table, name, range)
        }
    }

    fn define_standard_table(
        &mut self,
        table: TableId,
        name: &str,
        range: TextRange,
    ) -> Option<TableId> {
        match self.entry(table, name) {
            None => {
                let new_table = self.push_table(TableOrigin::Explicit(range));
                self.tables[table]
                    .entries
                    .insert(name.to_string(), Entry::Table(new_table));
                Some(new_table)
            }
            Some(EntryRef::Table(existing, previous)) => {
                if matches!(self.tables[existing].origin, TableOrigin::Implicit(_)) {
                    self.tables[existing].origin = TableOrigin::Explicit(range);
                } else {
                    self.error("This TOML table is already defined.", range, previous);
                }
                Some(existing)
            }
            Some(EntryRef::ArrayTable(latest, previous)) => {
                self.error(
                    "This TOML table conflicts with a previous definition.",
                    range,
                    previous,
                );
                Some(latest)
            }
            Some(EntryRef::Value(previous)) => {
                self.error(
                    "This TOML table conflicts with a previous definition.",
                    range,
                    previous,
                );
                None
            }
        }
    }

    fn define_array_table(
        &mut self,
        table: TableId,
        name: &str,
        range: TextRange,
    ) -> Option<TableId> {
        match self.entry(table, name) {
            None => {
                let element = self.push_table(TableOrigin::ArrayElement);
                self.tables[table].entries.insert(
                    name.to_string(),
                    Entry::ArrayTable {
                        range,
                        latest: element,
                    },
                );
                Some(element)
            }
            Some(EntryRef::ArrayTable(_, _)) => {
                let element = self.push_table(TableOrigin::ArrayElement);
                let Some(Entry::ArrayTable { latest, .. }) =
                    self.tables[table].entries.get_mut(name)
                else {
                    unreachable!();
                };
                *latest = element;
                Some(element)
            }
            Some(EntryRef::Table(existing, previous)) => {
                self.error(
                    "This TOML array table conflicts with a previous definition.",
                    range,
                    previous,
                );
                Some(existing)
            }
            Some(EntryRef::Value(previous)) => {
                self.error(
                    "This TOML array table conflicts with a previous definition.",
                    range,
                    previous,
                );
                None
            }
        }
    }

    fn resolve_parent(
        &mut self,
        table: TableId,
        name: &str,
        range: TextRange,
        missing_origin: TableOrigin,
    ) -> Option<TableId> {
        match self.entry(table, name) {
            None => {
                let parent = self.push_table(missing_origin);
                self.tables[table]
                    .entries
                    .insert(name.to_string(), Entry::Table(parent));
                Some(parent)
            }
            Some(EntryRef::Table(parent, previous)) => {
                if matches!(missing_origin, TableOrigin::Dotted(_)) {
                    match self.tables[parent].origin {
                        TableOrigin::Implicit(_) => {
                            self.tables[parent].origin = TableOrigin::Dotted(range);
                        }
                        TableOrigin::Explicit(_) => {
                            self.error(
                                "This TOML key conflicts with a previous table definition.",
                                range,
                                previous,
                            );
                            return None;
                        }
                        _ => {}
                    }
                }
                Some(parent)
            }
            Some(EntryRef::ArrayTable(latest, previous)) => {
                if matches!(missing_origin, TableOrigin::Dotted(_)) {
                    self.error(
                        "This TOML key conflicts with a previous array table definition.",
                        range,
                        previous,
                    );
                    None
                } else {
                    Some(latest)
                }
            }
            Some(EntryRef::Value(previous)) => {
                self.error(
                    "This TOML key conflicts with a previous definition.",
                    range,
                    previous,
                );
                None
            }
        }
    }

    fn validate_value(&mut self, value: AnyTomlValue) {
        let mut values = vec![value];
        while let Some(value) = values.pop() {
            match value {
                AnyTomlValue::TomlArray(array) => {
                    values.extend(array.elements().into_iter().flatten());
                }
                AnyTomlValue::TomlInlineTable(table) => {
                    self.validate_inline_table(&table, &mut values);
                }
                _ => {}
            }
        }
    }

    fn validate_inline_table(&mut self, table: &TomlInlineTable, values: &mut Vec<AnyTomlValue>) {
        let mut validator = Self::new();
        for element in table.elements().into_iter().flatten() {
            if let Some(key_value) = element.as_toml_key_value()
                && let Some(value) = validator.record_key_value(ROOT_TABLE, key_value)
            {
                values.push(value);
            }
        }
        self.diagnostics.extend(validator.diagnostics);
    }

    fn entry(&self, table: TableId, name: &str) -> Option<EntryRef> {
        match self.tables[table].entries.get(name)? {
            Entry::Value(range) => Some(EntryRef::Value(*range)),
            Entry::Table(table) => {
                let range = self.tables[*table].origin.range()?;
                Some(EntryRef::Table(*table, range))
            }
            Entry::ArrayTable { range, latest } => Some(EntryRef::ArrayTable(*latest, *range)),
        }
    }

    fn push_table(&mut self, origin: TableOrigin) -> TableId {
        let id = self.tables.len();
        self.tables.push(Table::new(origin));
        id
    }

    fn error(&mut self, message: &'static str, range: TextRange, previous: TextRange) {
        self.diagnostics.push(
            ParseDiagnostic::new(message, range)
                .with_detail(previous, "The first definition is here."),
        );
    }
}

struct Table {
    origin: TableOrigin,
    entries: HashMap<String, Entry>,
}

impl Table {
    fn new(origin: TableOrigin) -> Self {
        Self {
            origin,
            entries: HashMap::new(),
        }
    }
}

enum Entry {
    Value(TextRange),
    Table(TableId),
    ArrayTable { range: TextRange, latest: TableId },
}

#[derive(Clone, Copy)]
enum EntryRef {
    Value(TextRange),
    Table(TableId, TextRange),
    ArrayTable(TableId, TextRange),
}

#[derive(Clone, Copy)]
enum TableOrigin {
    Root,
    Implicit(TextRange),
    Dotted(TextRange),
    Explicit(TextRange),
    ArrayElement,
}

impl TableOrigin {
    fn range(self) -> Option<TextRange> {
        match self {
            Self::Implicit(range) | Self::Dotted(range) | Self::Explicit(range) => Some(range),
            Self::Root | Self::ArrayElement => None,
        }
    }
}

fn key_path(key: &TomlKey) -> Option<Vec<String>> {
    let path: Option<Vec<_>> = key
        .segments()
        .into_iter()
        .map(|segment| decode_key_segment(&segment.ok()?.value().ok()?))
        .collect();
    path.filter(|path| !path.is_empty())
}

fn decode_key_segment(token: &biome_toml_syntax::TomlSyntaxToken) -> Option<String> {
    let text = token.text_trimmed();
    match token.kind() {
        TomlSyntaxKind::TOML_BARE_KEY => Some(text.to_string()),
        TomlSyntaxKind::TOML_LITERAL_STRING => text
            .strip_prefix('\'')?
            .strip_suffix('\'')
            .map(str::to_string),
        TomlSyntaxKind::TOML_BASIC_STRING => decode_basic_string(text),
        _ => None,
    }
}

fn decode_basic_string(text: &str) -> Option<String> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    if inner.starts_with("\"\"") || inner.ends_with("\"\"") {
        return None;
    }

    let mut decoded = String::with_capacity(inner.len());
    let mut characters = inner.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            decoded.push(character);
            continue;
        }

        let escaped = match characters.next()? {
            'b' => '\u{0008}',
            'e' => '\u{001b}',
            't' => '\t',
            'n' => '\n',
            'f' => '\u{000c}',
            'r' => '\r',
            '"' => '"',
            '\\' => '\\',
            'x' => decode_unicode_escape(&mut characters, 2)?,
            'u' => decode_unicode_escape(&mut characters, 4)?,
            'U' => decode_unicode_escape(&mut characters, 8)?,
            _ => return None,
        };
        decoded.push(escaped);
    }

    Some(decoded)
}

fn decode_unicode_escape(characters: &mut std::str::Chars, digits: usize) -> Option<char> {
    let mut value = 0;
    for _ in 0..digits {
        value = value * 16 + characters.next()?.to_digit(16)?;
    }
    char::from_u32(value)
}
