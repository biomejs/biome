use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::TailwindSyntaxKind;

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
        ("field-sizing-fixed", "field-sizing"),
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
