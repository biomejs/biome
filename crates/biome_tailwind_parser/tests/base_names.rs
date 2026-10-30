use biome_rowan::NodeCache;
use biome_tailwind_parser::{
    BaseNameStore, TailwindParserOptions, parse_tailwind, parse_tailwind_with_options,
};
use biome_tailwind_syntax::TailwindSyntaxKind;
use biome_tailwind_syntax::metadata::BASENAMES_WITH_DASHES;

#[test]
fn appearance_category_bases() {
    for (class, expected_base) in [
        ("bg-blend-normal", "bg-blend"),
        ("bg-clip-border", "bg-clip"),
        ("bg-gradient-to-t", "bg-gradient-to"),
        ("bg-origin-border", "bg-origin"),
        ("border-be-4", "border-be"),
        ("border-bs-4", "border-bs"),
        ("box-decoration-slice", "box-decoration"),
        ("break-inside-auto", "break-inside"),
        ("drop-shadow!", "drop-shadow"),
        ("field-sizing-fixed", "field-sizing"),
        ("flex-grow", "flex-grow"),
        ("flex-grow!", "flex-grow"),
        ("flex-grow-0", "flex-grow"),
        ("flex-shrink-0", "flex-shrink"),
        ("font-features-['ss01']", "font-features"),
        ("forced-color-adjust-auto", "forced-color-adjust"),
        ("grid-flow-row", "grid-flow"),
        ("inset-be-full", "inset-be"),
        ("inset-bs-full", "inset-bs"),
        ("inset-e-full", "inset-e"),
        ("inset-s-full", "inset-s"),
        ("justify-items-start", "justify-items"),
        ("justify-self-auto", "justify-self"),
        ("mask-clip-border", "mask-clip"),
        ("mask-origin-border", "mask-origin"),
        ("mask-type-alpha", "mask-type"),
        ("max-block-none", "max-block"),
        ("max-inline-none", "max-inline"),
        ("max-w-screen", "max-w-screen"),
        ("max-w-screen-lg", "max-w-screen"),
        ("min-block-auto", "min-block"),
        ("min-inline-auto", "min-inline"),
        ("overflow-x-auto", "overflow-x"),
        ("overflow-y-auto", "overflow-y"),
        ("overscroll-x-auto", "overscroll-x"),
        ("overscroll-y-auto", "overscroll-y"),
        ("place-content-start", "place-content"),
        ("place-items-start", "place-items"),
        ("place-self-auto", "place-self"),
        ("scroll-mbe-4", "scroll-mbe"),
        ("scroll-mbs-4", "scroll-mbs"),
        ("scroll-pbe-4", "scroll-pbe"),
        ("scroll-pbs-4", "scroll-pbs"),
        ("scrollbar-gutter-auto", "scrollbar-gutter"),
        ("scrollbar-thumb-red-500", "scrollbar-thumb"),
        ("scrollbar-track-red-500", "scrollbar-track"),
        ("touch-pan-x", "touch-pan"),
    ] {
        let parsed = parse_tailwind(class);
        assert!(
            parsed.diagnostics().is_empty(),
            "{class}: {:?}",
            parsed.diagnostics()
        );
        let bases: Vec<_> = parsed
            .syntax()
            .descendants_tokens(biome_rowan::Direction::Next)
            .filter(|token| token.kind() == TailwindSyntaxKind::TW_BASE)
            .map(|token| token.text_trimmed().to_string())
            .collect();
        assert_eq!(bases, [expected_base], "{class}");
    }
}

#[test]
fn custom_base_names_split_functional_roots() {
    let mut names = vec!["slide-in-from-top", "zoom-in", "fill-mode"];
    names.extend_from_slice(BASENAMES_WITH_DASHES);
    let store = BaseNameStore::new(&names);
    let options = TailwindParserOptions::with_base_names(&store);
    for (class, expected_base) in [
        ("slide-in-from-top-[48%]", "slide-in-from-top"),
        ("slide-in-from-top-2", "slide-in-from-top"),
        ("slide-in-from-top", "slide-in-from-top"),
        ("-zoom-in-50", "zoom-in"),
        ("fill-mode-both", "fill-mode"),
        ("data-[open]:slide-in-from-top-[48%]", "slide-in-from-top"),
        ("bg-red-500", "bg"),
        ("border-t-2", "border-t"),
    ] {
        let parsed = parse_tailwind_with_options(class, &mut NodeCache::default(), options);
        assert!(
            parsed.diagnostics().is_empty(),
            "{class}: {:?}",
            parsed.diagnostics()
        );
        let bases: Vec<_> = parsed
            .syntax()
            .descendants_tokens(biome_rowan::Direction::Next)
            .filter(|token| token.kind() == TailwindSyntaxKind::TW_BASE)
            .map(|token| token.text_trimmed().to_string())
            .collect();
        assert_eq!(bases, [expected_base], "{class}");
    }
}
