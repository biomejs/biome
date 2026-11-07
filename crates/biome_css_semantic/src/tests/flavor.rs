use crate::{
    SemanticFlavor, SemanticModelOptions, SyntheticCustomProperty, semantic_model_with_options,
};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;
use biome_languages::css::{CssEmbeddingKind, EmbeddingHtmlKind, EmbeddingStyleApplicability};
use biome_rowan::{TextRange, TextSize};

#[test]
fn derives_semantic_flavor_from_file_source() {
    let source = CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(
        EmbeddingHtmlKind::Astro {
            applicability: EmbeddingStyleApplicability::Local,
        },
    ));
    let mut options = SemanticModelOptions::from(&source);
    let host_range = TextRange::new(TextSize::from(10), TextSize::from(22));
    options.synthetic_custom_properties =
        vec![SyntheticCustomProperty::new("--foreground").with_host_range(host_range)];
    options.custom_properties_open = true;
    assert_eq!(options.flavor, SemanticFlavor::Astro);

    let parsed = parse_css(":root {}", source, CssParserOptions::default());
    let root = parsed.tree();
    let model = semantic_model_with_options(&root, options);
    assert_eq!(model.flavor(), SemanticFlavor::Astro);
    assert_eq!(
        model.synthetic_custom_properties()[0].name(),
        "--foreground"
    );
    assert!(model.custom_properties_are_open());
    assert!(model.global_custom_variables().contains_key("--foreground"));
    assert_eq!(
        model
            .global_custom_variables()
            .get("--foreground")
            .and_then(|property| property.synthetic_range()),
        Some(host_range)
    );
    // Semantic equality ignores source locations; range-sensitive queries track parses separately.
    assert_eq!(
        model,
        semantic_model_with_options(
            &root,
            SemanticModelOptions {
                flavor: SemanticFlavor::Astro,
                synthetic_custom_properties: vec![
                    SyntheticCustomProperty::new("--foreground")
                        .with_host_range(TextRange::new(TextSize::from(30), TextSize::from(42)),),
                ],
                custom_properties_open: true
            },
        )
    );
    assert_ne!(
        model,
        semantic_model_with_options(&root, SemanticModelOptions::default())
    );

    assert_eq!(
        SemanticModelOptions::from(&CssFileSource::css()).flavor,
        SemanticFlavor::Vanilla
    );
}
