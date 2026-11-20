use biome_db::Source;
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot};
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::AnyParse;
use biome_parser::db::SourceExt;
use biome_rowan::{AstNode, TextRange, TextSize};
use camino::Utf8Path;

pub(crate) fn collect_static_imports<'a>(
    snippets: impl Iterator<Item = (&'a AnyParse, DocumentFileSource, TextSize)>,
) -> Vec<TextRange> {
    let mut ranges = Vec::new();
    for (parse, source, content_offset) in snippets {
        if source.to_js_file_source().is_none() {
            continue;
        }
        let root = parse.tree::<AnyJsRoot>();
        let Some(module) = root.as_js_module() else {
            continue;
        };
        // Typed nodes use snippet-local ranges; the parsed root owns the host offset.
        let offset = match parse {
            AnyParse::EmbeddedNode(parse) => parse.root.offset(),
            AnyParse::Node(_) => content_offset,
        };
        ranges.extend(module.items().into_iter().filter_map(|item| match item {
            AnyJsModuleItem::JsImport(import) => Some(import.range() + offset),
            _ => None,
        }));
    }
    ranges.sort_unstable_by_key(|range| range.start());
    ranges
}

pub(crate) fn imports_in_range(imports: &[TextRange], range: TextRange) -> &[TextRange] {
    let start = imports.partition_point(|import| import.start() < range.start());
    let imports = &imports[start..];
    let end = imports.partition_point(|import| import.end() <= range.end());
    &imports[..end]
}

#[salsa::tracked(returns(ref))]
fn static_imports_from_source(db: &dyn LanguageDb, file: Source) -> Vec<TextRange> {
    collect_static_imports(file.snippets(db).iter().filter_map(|snippet| {
        Some((
            snippet.parsed(db),
            db.source_from_index(snippet.document_source_index(db))?,
            snippet.content_offset(db),
        ))
    }))
}

/// Returns static import declarations contained in `range`, in host-document offsets.
/// Returns an empty slice when the document is not present in the workspace.
pub fn static_imports<'db>(
    db: &'db dyn LanguageDb,
    path: &Utf8Path,
    range: TextRange,
) -> &'db [TextRange] {
    let Some(file) = db.parsed_source_for_path(path) else {
        return &[];
    };
    imports_in_range(static_imports_from_source(db, file), range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{TestDb, TestSnippet};
    use crate::{EmbeddedSnippet, collect_embedded_data};
    use biome_db::testing::assert_function_query_was_not_run;
    use biome_html_parser::parse_html;
    use biome_js_parser::{JsParserOptions, parse_js_with_offset};
    use biome_languages::{HtmlFileSource, JsFileSource};
    use camino::Utf8PathBuf;

    #[test]
    fn static_imports_preserve_declarations_and_host_ranges() {
        let db = TestDb::new();
        let first = "\nimport first from './shared.js';\nimport { second } from './shared.js';";
        let second = "\nimport type { Shape } from './types.js';\nimport './setup.js';\nimport('./dynamic.js');\nconst text = 'import fake from \"fake\"';\n// import './comment.js';\n";
        let host =
            format!("<p>日本語</p>\r\n<script>{first}</script>\r\n<script>{second}</script>");
        let path = Utf8PathBuf::from("src/file.astro");
        let host_parse: AnyParse = parse_html(&host, (&HtmlFileSource::astro()).into()).into();
        let ranges: Vec<_> = [first, second]
            .into_iter()
            .map(|text| {
                let start = TextSize::from(host.find(text).unwrap() as u32);
                TextRange::at(start, TextSize::of(text))
            })
            .collect();
        db.insert_document(
            &path,
            &host,
            2,
            [first, second]
                .into_iter()
                .zip(&ranges)
                .map(|(text, range)| TestSnippet {
                    text: text.to_string(),
                    element_range: *range,
                    content_range: *range,
                    content_offset: range.start(),
                    source_index: 5,
                })
                .collect(),
        );
        let whole = TextRange::up_to(TextSize::of(&host));
        let expected: Vec<_> = [
            "import first from './shared.js';",
            "import { second } from './shared.js';",
            "import type { Shape } from './types.js';",
            "import './setup.js';",
        ]
        .into_iter()
        .map(|text| {
            TextRange::at(
                TextSize::from(host.find(text).unwrap() as u32),
                TextSize::of(text),
            )
        })
        .collect();
        assert_eq!(static_imports(&db, &path, whole), expected);
        assert_eq!(static_imports(&db, &path, ranges[0]), &expected[..2]);
        assert_eq!(static_imports(&db, &path, ranges[1]), &expected[2..]);
        assert!(static_imports(&db, Utf8Path::new("missing.astro"), whole).is_empty());

        let parses: Vec<AnyParse> = [first, second]
            .into_iter()
            .zip(&ranges)
            .map(|(text, range)| {
                parse_js_with_offset(
                    text,
                    range.start(),
                    JsFileSource::ts(),
                    JsParserOptions::default(),
                )
                .into()
            })
            .collect();
        let data = collect_embedded_data(
            HtmlFileSource::astro().into(),
            &host_parse,
            parses
                .iter()
                .zip(&ranges)
                .map(|(parse, range)| {
                    EmbeddedSnippet::new(parse, *range, JsFileSource::ts().into())
                })
                .collect(),
        );
        assert_eq!(data.static_imports(whole), expected);
        assert_eq!(data.static_imports(ranges[1]), &expected[2..]);

        let local_parse =
            biome_js_parser::parse(first, JsFileSource::ts(), JsParserOptions::default()).into();
        let data = collect_embedded_data(
            HtmlFileSource::astro().into(),
            &host_parse,
            vec![EmbeddedSnippet::new(
                &local_parse,
                ranges[0],
                JsFileSource::ts().into(),
            )],
        );
        assert_eq!(data.static_imports(whole), &expected[..2]);

        let file = biome_db::Db::parsed_source_for_path(&db, &path).unwrap();
        db.clear_salsa_events();
        assert_eq!(static_imports(&db, &path, whole), expected);
        assert_function_query_was_not_run(
            &db,
            static_imports_from_source,
            file,
            &db.take_salsa_events(),
        );
    }
}
