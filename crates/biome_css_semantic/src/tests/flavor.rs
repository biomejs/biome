use crate::{SemanticFlavor, SemanticModelOptions, semantic_model_with_options};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;
use biome_languages::css::{CssEmbeddingKind, EmbeddingHtmlKind, EmbeddingStyleApplicability};

#[test]
fn derives_semantic_flavor_from_file_source() {
    let source = CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(
        EmbeddingHtmlKind::Astro {
            applicability: EmbeddingStyleApplicability::Local,
        },
    ));
    let options = SemanticModelOptions::from(&source);
    assert_eq!(options.flavor, SemanticFlavor::Astro);

    let parsed = parse_css(":root {}", source, CssParserOptions::default());
    let root = parsed.tree();
    let model = semantic_model_with_options(&root, options);
    assert_eq!(model.flavor(), SemanticFlavor::Astro);
    assert_ne!(
        model,
        semantic_model_with_options(&root, SemanticModelOptions::default())
    );

    assert_eq!(
        SemanticModelOptions::from(&CssFileSource::css()).flavor,
        SemanticFlavor::Vanilla
    );
}
