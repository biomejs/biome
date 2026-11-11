use biome_db::ParsedSource;
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot};
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::AnyParse;
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
fn static_imports_from_source(db: &dyn LanguageDb, file: ParsedSource) -> Vec<TextRange> {
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
    use crate::testing::TestDb;
    use crate::{EmbeddedSnippet, collect_embedded_data};
    use biome_db::ParsedSnippet;
    use biome_db::testing::assert_function_query_was_not_run;
    use biome_html_parser::parse_html;
    use biome_js_parser::{JsParserOptions, parse_js_with_offset};
    use biome_languages::{HtmlFileSource, JsFileSource};
    use camino::Utf8PathBuf;
    use salsa::Setter;

    #[test]
    fn static_imports_preserve_declarations_and_host_ranges() {
        let mut db = TestDb::new();
        let first = "\nimport first from './shared.js';\nimport { second } from './shared.js';";
        let second = "\nimport type { Shape } from './types.js';\nimport './setup.js';\nimport('./dynamic.js');\nconst text = 'import fake from \"fake\"';\n// import './comment.js';\n";
        let host =
            format!("<p>日本語</p>\r\n<script>{first}</script>\r\n<script>{second}</script>");
        let path = Utf8PathBuf::from("src/file.astro");
        let host_parse: AnyParse = parse_html(&host, (&HtmlFileSource::astro()).into()).into();
        let snippets: Vec<_> = [first, second]
            .into_iter()
            .map(|text| {
                let start = TextSize::from(host.find(text).unwrap() as u32);
                let range = TextRange::at(start, TextSize::of(text));
                let parse = parse_js_with_offset(
                    text,
                    start,
                    JsFileSource::ts(),
                    JsParserOptions::default(),
                );
                assert!(!parse.has_errors());
                ParsedSnippet::new(&db, parse.into(), range, range, start, 5)
            })
            .collect();
        let file = ParsedSource::new(&db, path.clone(), host_parse.clone(), 2, snippets.clone());
        db.insert_file(path.clone(), file);
        let whole = TextRange::up_to(TextSize::of(&host));
        let imports = static_imports(&db, &path, whole);
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
        assert_eq!(imports, expected);
        assert_eq!(
            static_imports(&db, &path, snippets[0].content_range(&db)),
            &expected[..2]
        );
        assert_eq!(
            static_imports(&db, &path, snippets[1].content_range(&db)),
            &expected[2..]
        );
        assert!(static_imports(&db, Utf8Path::new("missing.astro"), whole).is_empty());

        let data = collect_embedded_data(
            HtmlFileSource::astro().into(),
            &host_parse,
            snippets
                .iter()
                .map(|snippet| {
                    EmbeddedSnippet::new(
                        snippet.parsed(&db),
                        snippet.content_range(&db),
                        JsFileSource::ts().into(),
                    )
                })
                .collect(),
        );
        assert_eq!(data.static_imports(whole), expected);
        assert_eq!(
            data.static_imports(snippets[1].content_range(&db)),
            &expected[2..]
        );

        let local_parse =
            biome_js_parser::parse(first, JsFileSource::ts(), JsParserOptions::default()).into();
        let data = collect_embedded_data(
            HtmlFileSource::astro().into(),
            &host_parse,
            vec![EmbeddedSnippet::new(
                &local_parse,
                snippets[0].content_range(&db),
                JsFileSource::ts().into(),
            )],
        );
        assert_eq!(data.static_imports(whole), &expected[..2]);

        db.clear_salsa_events();
        assert_eq!(static_imports(&db, &path, whole), expected);
        assert_function_query_was_not_run(
            &db,
            static_imports_from_source,
            file,
            &db.take_salsa_events(),
        );

        let replacement = parse_js_with_offset(
            "import('./dynamic.js');",
            snippets[0].content_offset(&db),
            JsFileSource::ts(),
            JsParserOptions::default(),
        )
        .into();
        snippets[0].set_parsed(&mut db).to(replacement);
        assert_eq!(static_imports(&db, &path, whole), &expected[2..]);
        file.set_snippets(&mut db).to(Vec::new());
        assert!(static_imports(&db, &path, whole).is_empty());
    }
}
