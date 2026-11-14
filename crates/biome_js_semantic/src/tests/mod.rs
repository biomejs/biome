mod assertions;
mod db;
pub mod declarations;
mod format;
mod functions;
mod infer;
mod references;
mod scopes;

#[macro_export]
macro_rules! assert_semantics {
    ($(#[$attr:meta])* $($name:ident, $code:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::tests::assertions::assert($code, stringify!($name));
            }
        )*
    };
}

#[test]
fn a_single_jsx_tag_root_has_a_global_scope() {
    use crate::{SemanticModelOptions, semantic_model};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    use biome_languages::javascript::{JsEmbeddingKind, MdxEmbeddingKind};

    let source_type = JsFileSource::jsx().with_embedding_kind(JsEmbeddingKind::Mdx {
        kind: MdxEmbeddingKind::Tag,
    });
    let parsed = parse(
        "<Chart data={values} />",
        source_type,
        JsParserOptions::default(),
    );
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());

    let mut unresolved: Vec<_> = model
        .all_unresolved_references()
        .map(|reference| reference.syntax().text_trimmed().to_string())
        .collect();
    unresolved.sort();
    assert_eq!(unresolved, ["Chart", "values"]);
}
