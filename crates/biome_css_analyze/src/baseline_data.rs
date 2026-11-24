//! This is a generated file. Don't modify it by hand! Run 'baseline' to re-generate the file.

#[doc = r" The Baseline availability tier of a CSS feature."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineTier {
    #[doc = r" Not yet supported in all core browsers."]
    Limited,
    #[doc = r" Supported in all core browsers for less than 30 months."]
    Newly,
    #[doc = r" Supported in all core browsers for at least 30 months."]
    Widely,
}
#[doc = r" The year a CSS feature became Baseline newly-available."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineYear {
    #[doc = r" The year the feature became newly available."]
    Known(u16),
    #[doc = r" No date is recorded for when this feature became available."]
    Unknown,
}
#[doc = r" The combined Baseline status of a CSS feature."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineStatus {
    pub tier: BaselineTier,
    pub year: BaselineYear,
}
#[doc = r" Look up the Baseline status of a feature by name."]
pub fn find_baseline(
    table: &phf::Map<&'static str, BaselineStatus>,
    name: &str,
) -> Option<BaselineStatus> {
    table.get(name).copied()
}
#[doc = r" Look up the Baseline status of a specific CSS property value."]
#[doc = r#" The key format is `"property:value"` (lowercase)."#]
pub fn find_property_value_baseline(property: &str, value: &str) -> Option<BaselineStatus> {
    let key = format!("{property}:{value}");
    BASELINE_PROPERTY_VALUES.get(key.as_str()).copied()
}
/// Baseline status for CSS properties.
pub static BASELINE_PROPERTIES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "-webkit-text-fill-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "-webkit-text-stroke" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "-webkit-text-stroke-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "-webkit-text-stroke-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "accent-color" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "align-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "alignment-baseline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "anchor-name" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "anchor-scope" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "animation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-composition" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "animation-delay" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-direction" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-duration" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-fill-mode" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-iteration-count" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-name" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-play-state" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-range" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-range-end" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-range-start" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-timeline" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-timing-function" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-trigger" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "appearance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "aspect-ratio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "backdrop-filter" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "backface-visibility" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "background" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-attachment" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-blend-mode" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-origin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "baseline-shift" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "baseline-source" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "block-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-left-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-right-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-collapse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-end-end-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-end-start-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-slice" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-source" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "border-spacing" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-start-end-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-start-start-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-left-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-right-radius" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-decoration-break" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "box-shadow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-sizing" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "break-after" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-inside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "caption-side" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "caret-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "caret-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "clear" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "clip" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "clip-path" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "clip-rule" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-adjust" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "color-interpolation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color-interpolation-filters" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-scheme" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "column-count" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-fill" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-gap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-height" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-break" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-visibility-items" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-span" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "column-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-wrap" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "columns" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "contain" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain-intrinsic-block-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-name" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content-visibility" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "corner-block-end-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "counter-increment" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-reset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-set" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "custom-property" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "cx" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "cy" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "direction" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "display" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "dominant-baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dynamic-range-limit" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "empty-cells" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "field-sizing" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "fill" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "fill-opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "fill-rule" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "filter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "fit-tolerance" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "flex" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-direction" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-grow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-shrink" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-wrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "float" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flood-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flood-opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-feature-settings" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "font-kerning" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-language-override" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-optical-sizing" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-palette" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size-adjust" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "font-stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-synthesis" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-synthesis-position" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-synthesis-small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-weight" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant-alternates" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-emoji" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-position" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variation-settings" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "font-weight" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "forced-color-adjust" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "frame-sizing" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "gap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "glyph-orientation-vertical" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "grid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-area" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-columns" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-flow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-rows" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-areas" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "hanging-punctuation" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "hyphenate-character" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "hyphenate-limit-chars" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hyphenate-limit-lines" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hyphens" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "image-orientation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "image-rendering" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "ime-mode" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "initial-letter" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "interactivity" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interpolate-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "isolation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "justify-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-self" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "letter-spacing" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "lighting-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "line-break" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-clamp" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "line-height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "link-parameters" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "list-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "margin-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-trim" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "marker" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-mid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "mask" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-border" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-outset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-repeat" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-slice" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-source" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-composite" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-mode" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "math-depth" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "math-shift" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "math-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "max-block-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "max-inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "min-block-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "min-inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "mix-blend-mode" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-view-box" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "offset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-distance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-rotate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "order" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "orphans" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "outline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-offset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "overflow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-anchor" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "overflow-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "overflow-clip-margin" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overflow-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "overflow-wrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "overflow-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overlay" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-inline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-x" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-y" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "padding" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "padding-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "padding-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "padding-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "padding-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "padding-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "page" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "page-break-after" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-inside" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "paint-order" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "perspective" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "place-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position-anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "print-color-adjust" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "quotes" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "r" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "reading-flow" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-order" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "resize" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "rotate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "row-gap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "row-rule" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-break" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-color" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-visibility-items" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-align" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-overhang" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-position" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "rule" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-break" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-color" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-cap" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-junction" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-overlap" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-visibility-items" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rx" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "ry" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scale" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scroll-axis-lock" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-behavior" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scroll-initial-target" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-margin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-margin-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-marker-group" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-padding" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-align" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-stop" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-target-group" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-timeline" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scroll-timeline-axis" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scroll-timeline-name" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scrollbar-color" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "scrollbar-gutter" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scrollbar-width" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "shape-image-threshold" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-margin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-rendering" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "speak" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "speak-as" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "stop-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stop-opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-dasharray" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-dashoffset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linecap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linejoin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-miterlimit" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "tab-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "table-layout" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align-last" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-anchor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-autospace" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-edge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-trim" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-combine-upright" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-decoration" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-line" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-skip-ink" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-decoration-skip-spaces" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-thickness" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-emphasis" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-fit" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-indent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-justify" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-orientation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-overflow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-rendering" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-shadow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-size-adjust" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-spacing-trim" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-transform" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-underline-offset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-wrap" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-mode" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-style" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "timeline-scope" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "timeline-trigger" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-name" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-source" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "touch-action" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "transform" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-origin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "transition" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-behavior" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "transition-delay" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-duration" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-property" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "translate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "trigger-scope" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "unicode-bidi" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "user-select" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "vector-effect" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "vertical-align" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "view-timeline" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-axis" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-inset" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-name" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-transition-class" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-name" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-scope" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "visibility" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space-collapse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-trim" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "widows" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "will-change" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "window-drag" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "word-break" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "word-spacing" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "wrap-inside" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "z-index" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "zoom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
};
/// Baseline status for CSS property values (identifier keywords only).
/// Keys are in `"property:value"` format (lowercase).
pub static BASELINE_PROPERTY_VALUES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "accent-color:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "accent-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "accent-color:transparent" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "align-content:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:safe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:space-around" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:space-between" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:space-evenly" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-content:unsafe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "align-items:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:safe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-items:unsafe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "align-self:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:safe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "align-self:unsafe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "alignment-baseline:alphabetic" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:baseline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:central" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:ideographic" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:mathematical" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:middle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:text-after-edge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "alignment-baseline:text-before-edge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "anchor-name:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "anchor-scope:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "anchor-scope:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "animation:alternate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:alternate-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:backwards" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:both" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:ease" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:ease-in" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:ease-in-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:ease-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:forwards" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:infinite" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:linear" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:paused" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:running" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:step-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation:step-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-direction:alternate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-direction:alternate-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-direction:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-direction:reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-duration:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-fill-mode:backwards" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-fill-mode:both" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-fill-mode:forwards" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-fill-mode:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-iteration-count:infinite" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-name:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-play-state:paused" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-play-state:running" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-range-end:normal" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-range-start:normal" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-timeline:scroll" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-timeline:view" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "animation-timing-function:ease" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:ease-in" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:ease-in-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:ease-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:jump" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:linear" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:step-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-timing-function:step-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "animation-trigger:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "appearance:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:base-select" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "appearance:button" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:checkbox" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:listbox" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:menulist" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:menulist-button" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:meter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:progress-bar" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:radio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:searchfield" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:textarea" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "appearance:textfield" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "aspect-ratio:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "backface-visibility:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "backface-visibility:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "background:background-clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:background-origin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:background-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:local" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:no-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:repeat-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:repeat-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:scroll" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background:text" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-attachment:fixed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-attachment:local" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-attachment:scroll" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-blend-mode:color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:color-burn" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:color-dodge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:darken" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:difference" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:exclusion" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:hard-light" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:hue" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:lighten" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:luminosity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:multiply" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:overlay" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:saturation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:screen" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-blend-mode:soft-light" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "background-clip:border-area" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-clip:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-clip:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-clip:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-clip:text" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-image:element" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "background-image:gradients" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-image:image-set" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "background-image:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-origin:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-origin:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-origin:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-x:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-x:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-x:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-y:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-y:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-position-y:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-repeat:2-value" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:no-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:repeat-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:repeat-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-repeat:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "background-size:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-size:contain" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "background-size:cover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "baseline-shift:sub" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "baseline-shift:super" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "baseline-source:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "baseline-source:first" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "baseline-source:last" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "block-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "block-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "block-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "block-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-block:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-end-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-start-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-block-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-bottom:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-left-radius:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-right-radius:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-bottom-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-collapse:collapse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-collapse:separate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-image:fill" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-repeat:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-repeat:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-repeat:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-repeat:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-source:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-image-width:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "border-inline:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-end-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-start-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-inline-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "border-left:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-left-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-radius:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-right-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-shape:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "border-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-left-radius:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-right-radius:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-top-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "border-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "bottom:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "bottom:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "bottom:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-decoration-break:clone" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "box-decoration-break:slice" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "box-shadow:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-shadow:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-sizing:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "box-sizing:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "break-after:always" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:avoid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:avoid-column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "break-after:avoid-page" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "break-after:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:page" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:recto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-after:verso" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:always" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:avoid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:avoid-column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "break-before:avoid-page" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "break-before:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:page" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:recto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-before:verso" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-inside:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-inside:avoid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "break-inside:avoid-column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "break-inside:avoid-page" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "caption-side:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "caption-side:bottom-outside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "caption-side:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "caption-side:top-outside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "caret-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "caret-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "caret-shape:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "caret-shape:bar" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "caret-shape:block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "caret-shape:underscore" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "clear:both" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "clear:inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "clear:inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "clear:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "clear:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "clear:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "clip:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "clip-path:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "clip-path:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "clip-path:path" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "clip-path:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "clip-path:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "clip-rule:evenodd" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "clip-rule:nonzero" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-interpolation:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color-interpolation:linearGradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color-interpolation:linearRGB" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color-interpolation:sRGB" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color-interpolation-filters:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-interpolation-filters:linearRGB" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-interpolation-filters:sRGB" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-scheme:dark" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "color-scheme:light" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "color-scheme:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "color-scheme:only" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "column-count:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-fill:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-fill:balance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-gap:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-break:intersection" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-break:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-break:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-color:repeat" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-inset:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-cap-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-junction-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-inset-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-visibility-items:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-visibility-items:around" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-visibility-items:between" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-visibility-items:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "column-rule-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-rule-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "column-span:all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "column-span:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "column-width:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "columns:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "columns:column-height" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "contain:content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:layout" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:paint" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:strict" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain:style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "contain-intrinsic-block-size:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-height:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-inline-size:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-size:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "contain-intrinsic-width:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-name:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-type:anchored" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "container-type:inline-size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-type:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "container-type:scroll-state" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "container-type:size" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "content:close-quote" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:image-set" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "content:no-close-quote" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:no-open-quote" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:open-quote" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content:url" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "content-visibility:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "content-visibility:hidden" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "content-visibility:visible" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "corner-block-end-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-end-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-end-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-end-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-end-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-end-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-block-start-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-left-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-right-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-bottom-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-end-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-end-start-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-end-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-inline-start-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-left-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-right-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-end-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-start-start-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-left-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-right-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:bevel" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:notch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:scoop" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:square" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "corner-top-shape:squircle" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "counter-increment:list-item" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-increment:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-reset:list-item" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-reset:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counter-reset:reversed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "counter-set:list-item" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "counter-set:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "direction:ltr" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "direction:rtl" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "display:block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:contents" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:flex" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:flow-root" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "display:grid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "display:grid-lanes" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:inline-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:inline-flex" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:inline-grid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "display:inline-grid-lanes" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:inline-table" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:list-item" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:math" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "display:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:ruby" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:ruby-base" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:ruby-base-container" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:ruby-text" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:ruby-text-container" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display:table" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-caption" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-cell" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-column" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-column-group" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-footer-group" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-header-group" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-row" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "display:table-row-group" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "dominant-baseline:alphabetic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:central" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:hanging" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:ideographic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:mathematical" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dominant-baseline:middle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "dynamic-range-limit:constrained" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "dynamic-range-limit:no-limit" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "dynamic-range-limit:standard" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "empty-cells:hide" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "empty-cells:show" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "field-sizing:content" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "field-sizing:fixed" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "fill:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "fill-rule:evenodd" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "fill-rule:nonzero" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "filter:blur" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:brightness" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:contrast" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:drop-shadow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:grayscale" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:hue-rotate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:invert" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:saturate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "filter:sepia" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "fit-tolerance:infinite" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "fit-tolerance:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "flex:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex:content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-basis:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-direction:column" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-direction:column-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-direction:row" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-direction:row-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:column" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:column-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:nowrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:row" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:row-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:wrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-flow:wrap-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-wrap:balance" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "flex-wrap:nowrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-wrap:wrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flex-wrap:wrap-reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "float:inline-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "float:inline-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "float:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "float:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "float:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flood-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "flood-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:caption" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:icon" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:menu" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:message-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:small-caption" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font:status-bar" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:cursive" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:fantasy" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:math" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "font-family:monospace" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:sans-serif" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:serif" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-family:system-ui" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "font-family:ui-monospace" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-family:ui-rounded" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-family:ui-sans-serif" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-family:ui-serif" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-feature-settings:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "font-kerning:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-kerning:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-kerning:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-optical-sizing:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-optical-sizing:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-palette:dark" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-palette:light" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-palette:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-size:large" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:larger" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:math" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-size:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:small" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:smaller" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:x-large" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:x-small" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:xx-large" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:xx-small" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size:xxx-large" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-size-adjust:from-font" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "font-size-adjust:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "font-size-adjust:two-values" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "font-stretch:condensed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:expanded" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:extra-condensed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:extra-expanded" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:percentage" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:semi-condensed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:semi-expanded" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:ultra-condensed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-stretch:ultra-expanded" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-style:italic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-style:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-style:oblique" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-style:oblique-angle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-synthesis:position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-synthesis:small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-synthesis:style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-synthesis:weight" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "font-synthesis-position:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-synthesis-position:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-synthesis-small-caps:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-small-caps:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-style:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-weight:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-synthesis-weight:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant:all-petite-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:all-small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:common-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:contextual" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:diagonal-fractions" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:discretionary-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:full-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:historical-forms" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant:historical-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:jis04" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:jis78" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:jis83" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:jis90" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:lining-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:no-common-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:no-contextual" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:no-discretionary-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:no-historical-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant:oldstyle-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:ordinal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:petite-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:proportional-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:proportional-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:ruby" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:simplified" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:slashed-zero" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:stacked-fractions" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:sub" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant:super" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-variant:tabular-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:titling-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:traditional" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant:unicase" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-alternates:annotation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:historical-forms" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:ornaments" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:styleset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:stylistic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-alternates:swash" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-variant-caps:all-petite-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:all-small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:petite-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:small-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:titling-caps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-caps:unicase" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:full-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:jis04" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:jis78" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:jis83" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:jis90" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:proportional-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:ruby" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:simplified" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-east-asian:traditional" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-emoji:emoji" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-emoji:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-emoji:text" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-emoji:unicode" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-ligatures:common-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:contextual" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:discretionary-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:historical-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:no-common-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:no-contextual" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:no-discretionary-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:no-historical-ligatures" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-ligatures:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:diagonal-fractions" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:lining-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:oldstyle-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:ordinal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:proportional-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:slashed-zero" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:stacked-fractions" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-numeric:tabular-nums" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "font-variant-position:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-position:sub" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-variant-position:super" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-weight:bold" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-weight:bolder" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-weight:lighter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-weight:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-weight:number" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "font-width:condensed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:expanded" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:extra-condensed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:extra-expanded" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:semi-condensed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:semi-expanded" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:ultra-condensed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-width:ultra-expanded" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "forced-color-adjust:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "forced-color-adjust:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "forced-color-adjust:preserve-parent-color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "frame-sizing:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "frame-sizing:content-block-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "frame-sizing:content-height" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "frame-sizing:content-inline-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "frame-sizing:content-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "gap:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-area:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-columns:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-columns:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-columns:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-flow:column" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-flow:dense" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-flow:row" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-rows:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-rows:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-auto-rows:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-column-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-row-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-areas:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:animation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "grid-template-columns:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:minmax" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-columns:subgrid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "grid-template-rows:animation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "grid-template-rows:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:minmax" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "grid-template-rows:subgrid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "hanging-punctuation:allow-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hanging-punctuation:first" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hanging-punctuation:force-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hanging-punctuation:last" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hanging-punctuation:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "height:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "height:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "height:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "height:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "height:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "height:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hyphenate-character:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "hyphenate-limit-chars:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hyphenate-limit-lines:no-limit" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hyphens:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "image-orientation:from-image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "image-orientation:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "image-rendering:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "image-rendering:crisp-edges" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "image-rendering:pixelated" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "image-rendering:smooth" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "initial-letter:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inline-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inline-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inline-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inline-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block-end:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block-end:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-block-start:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block-start:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-block-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline-end:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline-end:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "inset-inline-start:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline-start:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "inset-inline-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "interactivity:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interactivity:inert" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay-end:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-delay-start:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interpolate-size:allow-keywords" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interpolate-size:numeric-only" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "justify-content:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:safe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:space-around" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:space-between" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:space-evenly" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-content:unsafe" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "justify-items:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:legacy" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-items:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "justify-self:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "justify-self:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "justify-self:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "left:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "left:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "left:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "letter-spacing:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "lighting-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "lighting-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "line-break:anywhere" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-break:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-break:loose" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-break:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-break:strict" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "line-clamp:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "line-height:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "link-parameters:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "list-style:inside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style:outside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style:symbols" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "list-style-image:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-position:inside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-position:outside" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:arabic-indic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:armenian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:bengali" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:cambodian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:circle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:cjk-decimal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:cjk-earthly-branch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:cjk-heavenly-stem" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:cjk-ideographic" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:decimal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:decimal-leading-zero" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:devanagari" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:disc" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:disclosure-closed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:disclosure-open" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:ethiopic-numeric" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:georgian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:gujarati" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:gurmukhi" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:hebrew" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:hiragana" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:hiragana-iroha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:japanese-formal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:japanese-informal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:kannada" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:katakana" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:katakana-iroha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:khmer" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:korean-hangul-formal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:korean-hanja-formal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:korean-hanja-informal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lao" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lower-alpha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lower-armenian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lower-greek" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lower-latin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:lower-roman" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:malayalam" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:mongolian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:myanmar" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:oriya" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:persian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:simp-chinese-formal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:simp-chinese-informal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:square" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:string" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:symbols" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "list-style-type:tamil" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:telugu" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:thai" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:tibetan" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:trad-chinese-formal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:trad-chinese-informal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:upper-alpha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:upper-armenian" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:upper-latin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "list-style-type:upper-roman" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-block:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-block-end:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-block-start:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-bottom:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-bottom:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-inline:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-inline-end:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-inline-start:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-left:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-left:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-right:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-right:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-top:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-top:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "margin-trim:block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:inline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "margin-trim:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "marker:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-end:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-mid:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "marker-start:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "mask:add" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:alpha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:exclude" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:intersect" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:luminance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:match-source" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:no-clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:no-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:repeat-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:repeat-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:subtract" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-border:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border:repeat" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border:space" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-repeat:repeat" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-repeat:round" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-repeat:space" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-repeat:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-source:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-border-width:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mask-clip:border" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:no-clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:padding" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:text" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-clip:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-composite:add" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-composite:exclude" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-composite:intersect" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-composite:subtract" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-image:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-mode:alpha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-mode:luminance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-mode:match-source" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:border" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:padding" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-origin:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-position:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:no-repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:repeat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:repeat-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:repeat-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-repeat:space" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-size:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-size:contain" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-size:cover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "mask-type:alpha" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "mask-type:luminance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "max-block-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "max-block-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-block-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-block-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-height:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "max-height:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-height:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "max-height:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "max-height:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "max-height:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "max-inline-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "max-inline-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-inline-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-inline-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-width:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "max-width:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "max-width:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "max-width:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "max-width:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "max-width:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-block-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-block-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-block-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-block-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-height:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-height:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "min-height:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-height:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "min-height:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "min-height:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-inline-size:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-inline-size:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-inline-size:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-inline-size:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-width:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "min-width:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "min-width:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "min-width:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "min-width:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "min-width:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "mix-blend-mode:plus-darker" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "mix-blend-mode:plus-lighter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit:contain" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit:cover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit:fill" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-fit:scale-down" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "object-view-box:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "offset-anchor:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-anchor:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:margin-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:padding-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:path" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:ray" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:url" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-path:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-position:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-rotate:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "offset-rotate:reverse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "opacity:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "outline:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "outline-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "outline-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:dashed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:dotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:double" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:groove" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:outset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:ridge" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-style:solid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-width:medium" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-width:thick" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "outline-width:thin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "overflow:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow:clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "overflow:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow:scroll" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-anchor:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "overflow-anchor:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "overflow-clip-margin:border-box" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overflow-clip-margin:content-box" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overflow-clip-margin:padding-box" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overflow-wrap:anywhere" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "overflow-wrap:break-word" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "overflow-wrap:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "overflow-x:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-x:clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "overflow-x:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-x:scroll" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-x:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-y:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-y:clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "overflow-y:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-y:scroll" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overflow-y:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "overlay:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overlay:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior:contain" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-block:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-block:contain" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-block:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-inline:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-inline:contain" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-inline:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-x:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-x:contain" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-x:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-y:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-y:contain" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "overscroll-behavior-y:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-after:always" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-after:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-after:avoid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-after:left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-after:right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before:always" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before:avoid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before:left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-before:right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-inside:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "page-break-inside:avoid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "perspective:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "perspective-origin:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "place-content:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:space-around" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:space-between" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:space-evenly" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-content:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "place-items:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-items:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:anchor-center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "place-self:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:flex-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:flex-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:self-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:self-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "place-self:stretch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:fill" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:painted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:stroke" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "pointer-events:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position:absolute" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position:fixed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position:relative" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position:static" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "position:sticky" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "position-anchor:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-anchor:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-anchor:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:span-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-area:y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:flip-block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:flip-inline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:flip-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:flip-x" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:flip-y" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:span-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try:y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:center" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:flip-block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:flip-inline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:flip-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:flip-x" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:flip-y" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:position-area" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-bottom" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-block-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-block-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-inline-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-inline-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-self-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:span-y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:top" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:x-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:x-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:y-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-fallbacks:y-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order:most-block-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order:most-height" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order:most-inline-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order:most-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-try-order:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:always" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:anchor-valid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:anchor-visible" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:anchors-valid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:anchors-visible" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "position-visibility:no-overflow" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "print-color-adjust:economy" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "print-color-adjust:exact" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "quotes:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "quotes:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "reading-flow:flex-flow" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:flex-visual" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:grid-columns" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:grid-order" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:grid-rows" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "reading-flow:source-order" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "resize:block" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "resize:inline" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "right:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "right:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "right:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "rotate:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "row-gap:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "row-rule:currentColor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:dashed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:dotted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:double" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:groove" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:hidden" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:medium" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:outset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:ridge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:solid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:thick" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:thin" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule:transparent" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-break:intersection" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-break:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-break:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-color:transparent" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-cap-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-junction-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-inset-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:dashed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:dotted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:double" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:groove" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:hidden" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:outset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:ridge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-style:solid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-visibility-items:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-visibility-items:around" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-visibility-items:between" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-visibility-items:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-width:medium" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-width:thick" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "row-rule-width:thin" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-align:center" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-align:space-around" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-align:space-between" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-align:start" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-overhang:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-overhang:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-position:alternate" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ruby-position:inter-character" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-position:over" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "ruby-position:under" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "rule:currentColor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:dashed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:dotted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:double" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:groove" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:hidden" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:medium" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:outset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:ridge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:solid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:thick" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:thin" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule:transparent" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-break:intersection" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-break:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-break:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-color:transparent" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-cap:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-end:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-junction:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-inset-start:overlap-join" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-overlap:column-over-row" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-overlap:row-over-column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:dashed" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:dotted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:double" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:groove" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:hidden" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:inset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:outset" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:ridge" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-style:solid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-visibility-items:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-visibility-items:around" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-visibility-items:between" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-visibility-items:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-width:medium" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-width:thick" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "rule-width:thin" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scale:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scroll-axis-lock:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-axis-lock:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-behavior:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scroll-behavior:smooth" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scroll-initial-target:nearest" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-initial-target:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-marker-group:after" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-marker-group:before" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-marker-group:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-padding:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-block-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline-end:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-padding-inline-start:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-align:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-align:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-align:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-align:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-stop:always" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-stop:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:both" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:pair" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-snap-type:x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-snap-type:y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-target-group:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-target-group:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-timeline-axis:block" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scroll-timeline-axis:inline" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scroll-timeline-axis:x" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scroll-timeline-axis:y" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "scrollbar-color:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "scrollbar-gutter:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scrollbar-gutter:stable" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scrollbar-width:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scrollbar-width:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scrollbar-width:thin" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "shape-image-threshold:percentages" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:circle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:path" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-outside:polygon" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "shape-rendering:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "speak-as:digits" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "speak-as:literal-punctuation" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "speak-as:no-punctuation" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "speak-as:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "speak-as:spell-out" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "stop-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stop-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-dasharray:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linecap:butt" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linecap:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linecap:square" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linejoin:bevel" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linejoin:miter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "stroke-linejoin:round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "tab-size:length" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "table-layout:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "table-layout:fixed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:justify" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:match-parent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-align-last:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-anchor:end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-anchor:middle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-anchor:start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-autospace:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-autospace:ideograph-alpha" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-autospace:ideograph-numeric" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-autospace:insert" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-autospace:no-autospace" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-autospace:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-edge:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-trim:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-trim:trim-both" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-trim:trim-end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-box-trim:trim-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-combine-upright:all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-combine-upright:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-decoration:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration:line-through" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration:overline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration:underline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-inset:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-line:blink" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-line:grammar-error" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "text-decoration-line:line-through" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-line:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-line:overline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-line:spelling-error" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "text-decoration-line:underline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-skip-ink:all" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "text-decoration-skip-ink:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-decoration-skip-ink:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-decoration-skip-spaces:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-skip-spaces:end" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-skip-spaces:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-skip-spaces:start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-decoration-style:wavy" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-thickness:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-thickness:from-font" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-decoration-thickness:percentage" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-emphasis:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-color:currentColor" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-color:transparent" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position:over" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-position:under" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:circle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:dot" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:double-circle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:filled" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:sesame" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-emphasis-style:triangle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "text-fit:grow" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-fit:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-fit:shrink" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-indent:each-line" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "text-indent:hanging" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "text-justify:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-justify:inter-character" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-justify:inter-word" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-justify:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-orientation:mixed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-orientation:sideways" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-orientation:upright" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-overflow:clip" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-overflow:ellipsis" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-overflow:string" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-rendering:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-rendering:geometricPrecision" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-shadow:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-size-adjust:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-size-adjust:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-size-adjust:percentages" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-spacing-trim:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-spacing-trim:space-all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-spacing-trim:space-first" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-spacing-trim:trim-start" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-transform:capitalize" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-transform:full-size-kana" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-transform:full-width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-transform:lowercase" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-transform:math-auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "text-transform:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-transform:uppercase" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "text-underline-offset:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-offset:percentage" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position:from-font" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-underline-position:under" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "text-wrap:balance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "text-wrap:nowrap" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap:pretty" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-wrap:stable" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap:wrap" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-mode:nowrap" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-mode:wrap" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-style:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-style:balance" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "text-wrap-style:pretty" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "text-wrap-style:stable" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "timeline-scope:all" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "timeline-scope:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "timeline-trigger:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range-end:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-activation-range-start:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-end:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-end:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-start:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-active-range-start:normal" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-name:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-source:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "timeline-trigger-source:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "top:anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "top:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "top:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "touch-action:manipulation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-down" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-up" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-x" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pan-y" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "touch-action:pinch-zoom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    "transform:3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "transform:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-box:border-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-box:content-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-box:fill-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-box:stroke-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-box:view-box" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "transform-origin:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-origin:center" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-origin:left" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-origin:right" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-origin:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transform-style:flat" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "transform-style:preserve-3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "transition:all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:ease" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:ease-in" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:ease-in-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:ease-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:linear" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:step-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:step-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition:transition-behavior" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "transition-property:all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-property:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:ease" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:ease-in" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:ease-in-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:ease-out" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:jump" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:linear" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:step-end" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "transition-timing-function:step-start" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "translate:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "trigger-scope:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "trigger-scope:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "unicode-bidi:bidi-override" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "unicode-bidi:embed" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "unicode-bidi:isolate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "unicode-bidi:isolate-override" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "unicode-bidi:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "unicode-bidi:plaintext" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "user-select:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "user-select:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "user-select:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "user-select:text" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "vector-effect:non-scaling-stroke" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "vector-effect:none" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "vertical-align:baseline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:middle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:sub" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:super" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:text-bottom" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:text-top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "vertical-align:top" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "view-timeline-axis:block" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-axis:inline" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-axis:x" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-axis:y" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-timeline-inset:auto" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "view-transition-class:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-name:match-element" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-name:none" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-scope:all" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "view-transition-scope:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "visibility:collapse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "visibility:hidden" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "visibility:visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:break-spaces" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:nowrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:pre" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:pre-line" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space:pre-wrap" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "white-space-collapse:break-spaces" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-collapse:collapse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-collapse:preserve" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-collapse:preserve-breaks" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-collapse:preserve-spaces" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "white-space-trim:discard-after" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "white-space-trim:discard-before" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "white-space-trim:discard-inner" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "white-space-trim:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "width:anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "width:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "width:fit-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "width:max-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "width:min-content" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "width:stretch" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "will-change:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "will-change:contents" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "will-change:scroll-position" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "window-drag:move" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "window-drag:none" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "word-break:auto-phrase" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "word-break:break-all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "word-break:break-word" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "word-break:keep-all" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "word-break:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "word-spacing:normal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "wrap-inside:auto" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "wrap-inside:avoid" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:horizontal-tb" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "writing-mode:lr" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:lr-tb" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:rl" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:rl-tb" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:sideways-lr" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "writing-mode:sideways-rl" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "writing-mode:tb" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:tb-rl" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "writing-mode:vertical-lr" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "writing-mode:vertical-rl" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "z-index:auto" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
};
/// Baseline status for CSS at-rules.
pub static BASELINE_AT_RULES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "charset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "container" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "counter-style" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "custom-media" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "font-face" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "font-feature-values" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "font-palette-values" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "function" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "import" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "keyframes" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "layer" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "media" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "namespace" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "page" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "position-try" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "property" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "scope" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "starting-style" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "supports" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "view-transition" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
};
/// Baseline status for CSS media query conditions.
pub static BASELINE_MEDIA_CONDITIONS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "-webkit-device-pixel-ratio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "-webkit-max-device-pixel-ratio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "-webkit-min-device-pixel-ratio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "-webkit-transform-3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "any-hover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "any-pointer" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "aspect-ratio" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "calc" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-gamut" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "color-index" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "device-aspect-ratio" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "device-height" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "device-posture" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "device-width" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display-mode" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "display-state" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "dynamic-range" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "forced-colors" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "grid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "height" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "horizontal-viewport-segments" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "inverted-colors" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "media_features" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "media_query_values" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "monochrome" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "nested-queries" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "or_syntax" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "orientation" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "overflow-block" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "overflow-inline" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "pointer" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    "prefers-color-scheme" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "prefers-contrast" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "prefers-reduced-data" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "prefers-reduced-motion" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "prefers-reduced-transparency" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "range_syntax" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "resizable" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "resolution" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scripting" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "update" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "vertical-viewport-segments" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "video-dynamic-range" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "width" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
};
/// Baseline status for CSS value functions.
pub static BASELINE_FUNCTIONS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "abs" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "acos" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "anchor" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "anchor-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "asin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "atan" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "atan2" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "attr" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "blur" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "brightness" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "calc" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "calc-size" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "circle" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "clamp" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "color" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "color-mix" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "conic-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "contrast" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "cos" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "counter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "counters" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "cross-fade" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "cubic-bezier" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "drop-shadow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "element" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "ellipse" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "env" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "exp" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "grayscale" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "hsl" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "hsla" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "hue-rotate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "hwb" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "hypot" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "image" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "image-set" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "inset" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "invert" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "lab" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "lch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "light-dark" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "linear-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "log" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "matrix" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "matrix3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "max" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "min" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "mod" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "oklab" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "oklch" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "opacity" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "paint" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "path" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "perspective" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "polygon" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "pow" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "radial-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "ray" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "rect" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "rem" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "repeating-conic-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "repeating-linear-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "repeating-radial-gradient" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "rgb" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "rotate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "rotate3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "rotateX" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "rotateY" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "rotateZ" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "round" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "saturate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "scale" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "scale3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "scaleX" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "scaleY" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "scaleZ" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "sepia" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "sign" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "sin" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "skew" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "skewX" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "skewY" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "sqrt" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "steps" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "string" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "tan" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "translate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "translate3d" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "translateX" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "translateY" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "translateZ" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "url" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    "var" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    "xywh" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
};
/// Baseline status for CSS pseudo-class and pseudo-element selectors.
pub static BASELINE_SELECTORS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
    "active" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "active-view-transition" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "active-view-transition-type" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "after" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "any-link" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "attribute" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "autofill" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "backdrop" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "before" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "buffering" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "checked" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "checkmark" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "class" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "column" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "cue" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "default" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "defined" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "descendant" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "details-content" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "dir" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "disabled" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "empty" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "enabled" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "field-content" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "file-selector-button" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "first" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "first-child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "first-letter" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "first-line" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "first-of-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "focus" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "focus-visible" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "focus-within" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "fullscreen" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "future" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "grammar-error" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "has" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "has-slotted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "heading" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "headingfunction" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "highlight" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "host" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "host-context" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "hostfunction" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "hover" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "id" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "in-range" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "indeterminate" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "interest-button" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-source" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "interest-target" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "invalid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "is" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "lang" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "last-child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "last-of-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "left" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "link" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "list" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "marker" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "modal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    "muted" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "namespace" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "navigation-source" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "nesting" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "next-sibling" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "not" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "nth-child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "nth-last-child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "nth-last-of-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "nth-of-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "only-child" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "only-of-type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "open" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    "optional" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "out-of-range" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "part" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "past" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "paused" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "picker" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "picker-icon" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "picture-in-picture" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "placeholder" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "placeholder-shown" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "playing" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "popover-open" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "read-only" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "read-write" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "required" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "right" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "root" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "scope" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "scroll-button" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-marker" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "scroll-marker-group" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "seeking" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "selection" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "slotted" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "spelling-error" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "stalled" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "state" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2024),
        },
    "subsequent-sibling" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "target" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "target-after" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "target-before" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "target-current" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "target-text" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    "type" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "universal" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "user-invalid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "user-valid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    "valid" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    "view-transition" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-group" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-image-pair" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-new" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "view-transition-old" => BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    "visited" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    "volume-locked" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    "where" => BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    "xr-overlay" => BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
};
