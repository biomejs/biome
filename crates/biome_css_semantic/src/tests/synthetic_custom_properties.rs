use crate::{SemanticModelOptions, SyntheticCustomProperty, semantic_model_with_options};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;
use biome_rowan::{TextRange, TextSize};

#[test]
fn host_custom_properties_are_global_custom_variables() {
    let host_range = TextRange::new(TextSize::from(10), TextSize::from(22));
    let options = SemanticModelOptions {
        synthetic_custom_properties: vec![
            SyntheticCustomProperty::new("foreground".into()).with_host_range(host_range),
        ],
        custom_properties_open: true,
    };

    let parsed = parse_css(
        ":root {}",
        CssFileSource::css(),
        CssParserOptions::default(),
    );
    let root = parsed.tree();
    let model = semantic_model_with_options(&root, options);
    let property = &model.synthetic_custom_properties()[0];
    assert_eq!(property.name().text(), "foreground");
    assert_eq!(property.host_range(), Some(host_range));
    assert!(model.custom_properties_are_open());
    assert!(model.global_custom_variables().contains_key("--foreground"));
    assert!(!model.global_custom_variables().contains_key("foreground"));
    assert!(
        model
            .global_custom_variables()
            .get("--foreground")
            .is_none()
    );
    // Semantic equality ignores source locations; range-sensitive queries track parses separately.
    assert_eq!(
        model,
        semantic_model_with_options(
            &root,
            SemanticModelOptions {
                synthetic_custom_properties: vec![
                    SyntheticCustomProperty::new("foreground".into())
                        .with_host_range(TextRange::new(TextSize::from(30), TextSize::from(42))),
                ],
                custom_properties_open: true,
            },
        )
    );
    assert_ne!(
        model,
        semantic_model_with_options(&root, SemanticModelOptions::default())
    );
}
