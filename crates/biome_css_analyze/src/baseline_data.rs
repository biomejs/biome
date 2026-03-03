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
#[doc = r" The table must be sorted by name for binary search."]
pub fn find_baseline(table: &[(&str, BaselineStatus)], name: &str) -> Option<BaselineStatus> {
    table
        .binary_search_by_key(&name, |&(n, _)| n)
        .ok()
        .map(|i| table[i].1)
}
#[doc = r" Look up the Baseline status of a specific CSS property value."]
pub fn find_property_value_baseline(property: &str, value: &str) -> Option<BaselineStatus> {
    BASELINE_PROPERTY_VALUES
        .binary_search_by_key(&(property, value), |&(p, v, _)| (p, v))
        .ok()
        .map(|i| BASELINE_PROPERTY_VALUES[i].2)
}
#[doc = r" Baseline status for CSS properties. Sorted by name."]
pub static BASELINE_PROPERTIES: &[(&str, BaselineStatus)] = &[
    (
        "-webkit-text-fill-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "-webkit-text-stroke",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "-webkit-text-stroke-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "-webkit-text-stroke-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "accent-color",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "align-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "align-items",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "align-self",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "alignment-baseline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "anchor-name",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "anchor-scope",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-composition",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "animation-delay",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-direction",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-duration",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-fill-mode",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-iteration-count",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-name",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-play-state",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-range",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-range-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-range-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-timeline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-timing-function",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "appearance",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "aspect-ratio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "backdrop-filter",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "backface-visibility",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "background",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-attachment",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-blend-mode",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "background-clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-origin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "baseline-shift",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-source",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "block-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-end-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-end-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-end-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-start-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-start-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-start-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-block-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-left-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-right-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-collapse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-end-end-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-end-start-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-slice",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-source",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-end-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-end-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-end-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-start-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-start-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-start-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-inline-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-spacing",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-start-end-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-start-start-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-left-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-right-radius",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-decoration-break",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "box-shadow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-sizing",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "break-after",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-inside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "caption-side",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "caret-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "caret-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "clear",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "clip",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "clip-path",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "clip-rule",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-adjust",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "color-interpolation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color-interpolation-filters",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-scheme",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "column-count",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-fill",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-gap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-span",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "column-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "columns",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain-intrinsic-block-size",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-height",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-inline-size",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-size",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-width",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-name",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content-visibility",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "corner-block-end-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-block-start-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-bottom-left-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-bottom-right-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-bottom-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-end-end-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-end-start-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-inline-end-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-inline-start-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-left-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-right-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-start-end-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-start-start-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-top-left-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-top-right-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "corner-top-shape",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "counter-increment",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-reset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-set",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "custom-property",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "cx",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "cy",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "direction",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "display",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "dominant-baseline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "empty-cells",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "field-sizing",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "fill",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "fill-opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "fill-rule",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "filter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "flex",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-direction",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-flow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-grow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-shrink",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-wrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "float",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flood-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flood-opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-feature-settings",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "font-kerning",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-language-override",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-optical-sizing",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-palette",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-size-adjust",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "font-stretch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-synthesis",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-synthesis-position",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-synthesis-small-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-weight",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant-alternates",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-emoji",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-position",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variation-settings",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "font-weight",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-width",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "forced-color-adjust",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "gap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "glyph-orientation-vertical",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "grid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-area",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-columns",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-flow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-rows",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-column",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-column-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-column-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-row",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-row-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-row-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-areas",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "hanging-punctuation",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "height",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "hyphenate-character",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "hyphenate-limit-chars",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hyphens",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "image-orientation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "image-rendering",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "ime-mode",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "initial-letter",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inline-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "interactivity",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "interpolate-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "isolation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "justify-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-items",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-self",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "letter-spacing",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "lighting-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "line-break",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-clamp",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "line-height",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "margin-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-trim",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "marker",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-mid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "mask",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-border",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-border-outset",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-border-repeat",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-border-slice",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-border-source",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-border-width",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mask-clip",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-composite",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-image",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-mode",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-position",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-repeat",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-size",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "math-depth",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "math-shift",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "math-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "max-block-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-height",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "max-inline-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "min-block-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-height",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "min-inline-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "mix-blend-mode",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-view-box",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "offset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-distance",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-rotate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "order",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "orphans",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "outline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-offset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "overflow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "overflow-clip-margin",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "overflow-wrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "overflow-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overlay",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overscroll-behavior",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "padding",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "padding-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "padding-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "padding-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "padding-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "padding-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "page",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "page-break-after",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-inside",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "paint-order",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "perspective",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "place-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "place-items",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "place-self",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "pointer-events",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position-anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-visibility",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "print-color-adjust",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "quotes",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "r",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "reading-flow",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-order",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "resize",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "rotate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "row-gap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "ruby-align",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-overhang",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "ruby-position",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "rx",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "ry",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scale",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scroll-behavior",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scroll-initial-target",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-margin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-margin-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-marker-group",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-padding",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-align",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-stop",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-target-group",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-axis",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-name",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scrollbar-color",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "scrollbar-gutter",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scrollbar-width",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "shape-image-threshold",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-margin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-rendering",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "speak",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "speak-as",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "stop-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stop-opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-dasharray",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-dashoffset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linecap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linejoin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-miterlimit",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "tab-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "table-layout",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align-last",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-anchor",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-autospace",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-edge",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-trim",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-combine-upright",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-decoration",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip-ink",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-thickness",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-emphasis",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-indent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-justify",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-orientation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-overflow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-rendering",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-shadow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-size-adjust",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-spacing-trim",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-transform",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-underline-offset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-wrap",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-mode",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-style",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "timeline-scope",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "touch-action",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "transform",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-origin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "transition",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-behavior",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transition-delay",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-duration",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-property",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "translate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "unicode-bidi",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "user-select",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "vector-effect",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "vertical-align",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "view-timeline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-axis",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-inset",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-name",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-transition-class",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-name",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "visibility",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space-collapse",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "widows",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "will-change",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "word-break",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "word-spacing",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "writing-mode",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "z-index",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "zoom",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
];
#[doc = r" Baseline status for CSS property values (identifier keywords only). Sorted by (property, value)."]
pub static BASELINE_PROPERTY_VALUES: &[(&str, &str, BaselineStatus)] = &[
    (
        "accent-color",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "align-content",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "align-items",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "align-self",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "align-self",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "align-self",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "align-self",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "alignment-baseline",
        "alphabetic",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "baseline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "central",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "ideographic",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "mathematical",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "middle",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "text-after-edge",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "alignment-baseline",
        "text-before-edge",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "anchor-name",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "anchor-scope",
        "all",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "anchor-scope",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation",
        "alternate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "alternate-reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "backwards",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "both",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "ease",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "ease-in",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "ease-in-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "ease-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "forwards",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "infinite",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "linear",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "step-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation",
        "step-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-direction",
        "alternate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-direction",
        "alternate-reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-direction",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-direction",
        "reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-duration",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-fill-mode",
        "backwards",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-fill-mode",
        "both",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-fill-mode",
        "forwards",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-fill-mode",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-iteration-count",
        "infinite",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-name",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-play-state",
        "paused",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-play-state",
        "running",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-range-end",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-range-start",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-timeline",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-timeline",
        "view",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "animation-timing-function",
        "ease",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "ease-in",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "ease-in-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "ease-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "jump",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "linear",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "step-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "animation-timing-function",
        "step-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "appearance",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "base-select",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "appearance",
        "button",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "checkbox",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "listbox",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "menulist",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "menulist-button",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "meter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "progress-bar",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "radio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "searchfield",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "textarea",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "appearance",
        "textfield",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "aspect-ratio",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "background",
        "background-clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "background-origin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "background-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "local",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "no-repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "repeat-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "repeat-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "space",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-attachment",
        "fixed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-attachment",
        "local",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-attachment",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-clip",
        "border-area",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-clip",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-clip",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-clip",
        "padding-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-clip",
        "text",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-image",
        "element",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "background-image",
        "gradients",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-image",
        "image-set",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "background-image",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-origin",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-origin",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-origin",
        "padding-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-position",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-repeat",
        "2-value",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "no-repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "repeat-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "repeat-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-repeat",
        "space",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "background-size",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-size",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "background-size",
        "cover",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "baseline-shift",
        "baseline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-shift",
        "sub",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-shift",
        "super",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-source",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-source",
        "first",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "baseline-source",
        "last",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "block-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "block-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "block-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "block-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "border",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-left-radius",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-right-radius",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-bottom-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-collapse",
        "collapse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-collapse",
        "separate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-image",
        "fill",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "space",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-repeat",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-repeat",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-repeat",
        "space",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-repeat",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-image-width",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "border-left",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-left-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-radius",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-right-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-left-radius",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-right-radius",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-top-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "border-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "bottom",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "bottom",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "bottom",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-decoration-break",
        "clone",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "box-decoration-break",
        "slice",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "box-shadow",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-shadow",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-sizing",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "box-sizing",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "break-after",
        "always",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "avoid-column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "break-after",
        "avoid-page",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "break-after",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "page",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "recto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-after",
        "verso",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "always",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "avoid-column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "break-before",
        "avoid-page",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "break-before",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "page",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "recto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-before",
        "verso",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-inside",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-inside",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "break-inside",
        "avoid-column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "break-inside",
        "avoid-page",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "caption-side",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "caption-side",
        "bottom-outside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "caption-side",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "caption-side",
        "top-outside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "caret-shape",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "caret-shape",
        "bar",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "caret-shape",
        "block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "caret-shape",
        "underscore",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "clear",
        "both",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "clear",
        "inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "clear",
        "inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "clear",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "clear",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "clear",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "clip",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "clip-path",
        "fill-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "clip-path",
        "path",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "clip-path",
        "stroke-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "clip-path",
        "view-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "clip-rule",
        "evenodd",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "clip-rule",
        "nonzero",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-interpolation",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color-interpolation",
        "linearGradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color-interpolation",
        "sRGB",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color-interpolation-filters",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-interpolation-filters",
        "linearRGB",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-interpolation-filters",
        "sRGB",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-scheme",
        "dark",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "color-scheme",
        "light",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "color-scheme",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "color-scheme",
        "only",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "column-count",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-fill",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-fill",
        "balance",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-gap",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-rule-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "column-span",
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "column-span",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "column-width",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "contain",
        "content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "inline-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "layout",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "paint",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "strict",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain",
        "style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "contain-intrinsic-block-size",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-height",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-inline-size",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-size",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "contain-intrinsic-width",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-name",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-type",
        "anchored",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "container-type",
        "inline-size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-type",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "container-type",
        "scroll-state",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "container-type",
        "size",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "content",
        "close-quote",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "image-set",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "content",
        "no-close-quote",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "no-open-quote",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "open-quote",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content",
        "url",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "content-visibility",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "content-visibility",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "content-visibility",
        "visible",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "counter-increment",
        "list-item",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-increment",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-reset",
        "list-item",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-reset",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counter-reset",
        "reversed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "counter-set",
        "list-item",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "counter-set",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "direction",
        "ltr",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "direction",
        "rtl",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "display",
        "block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "contents",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "flex",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "flow-root",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "display",
        "grid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "inline-block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "inline-flex",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "inline-grid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "inline-table",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "list-item",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "math",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "display",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "display",
        "ruby",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "ruby-base",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "ruby-base-container",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "ruby-text",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "ruby-text-container",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display",
        "table",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-caption",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-cell",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-column",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-column-group",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-footer-group",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-header-group",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-row",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "display",
        "table-row-group",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "dominant-baseline",
        "alphabetic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "central",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "hanging",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "ideographic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "mathematical",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "dominant-baseline",
        "middle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "empty-cells",
        "hide",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "empty-cells",
        "show",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "field-sizing",
        "content",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "field-sizing",
        "fixed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "fill",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "fill-rule",
        "evenodd",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "fill-rule",
        "nonzero",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "filter",
        "blur",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "brightness",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "contrast",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "drop-shadow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "grayscale",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "hue-rotate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "invert",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "saturate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "filter",
        "sepia",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "flex",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        "content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-basis",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-direction",
        "column",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-direction",
        "column-reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-direction",
        "row",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-direction",
        "row-reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-wrap",
        "nowrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-wrap",
        "wrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flex-wrap",
        "wrap-reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "float",
        "inline-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "float",
        "inline-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "float",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "float",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "float",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flood-color",
        "currentColor",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "flood-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "caption",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "icon",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "menu",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "message-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "small-caption",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font",
        "status-bar",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "cursive",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "fangsong",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "fantasy",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "kai",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "khmer-mul",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "math",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "font-family",
        "monospace",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "nastaliq",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "sans-serif",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "serif",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-family",
        "system-ui",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "font-family",
        "ui-monospace",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-family",
        "ui-rounded",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-family",
        "ui-sans-serif",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-family",
        "ui-serif",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-feature-settings",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "font-kerning",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-kerning",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-kerning",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-optical-sizing",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-optical-sizing",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-palette",
        "dark",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-palette",
        "light",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-palette",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-size",
        "math",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-size",
        "xxx-large",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-size-adjust",
        "from-font",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "font-size-adjust",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "font-size-adjust",
        "two-values",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "font-stretch",
        "condensed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "expanded",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "extra-condensed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "extra-expanded",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "percentage",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "semi-condensed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "semi-expanded",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "ultra-condensed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-stretch",
        "ultra-expanded",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-style",
        "italic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-style",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-style",
        "oblique",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-style",
        "oblique-angle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-synthesis",
        "position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-synthesis",
        "small-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-synthesis",
        "style",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-synthesis",
        "weight",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "font-synthesis-position",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-synthesis-position",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-synthesis-small-caps",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-small-caps",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-style",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-weight",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-synthesis-weight",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant",
        "historical-forms",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant",
        "sub",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant",
        "super",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-variant-alternates",
        "annotation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "historical-forms",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "ornaments",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "styleset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "stylistic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-alternates",
        "swash",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-variant-caps",
        "all-petite-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "all-small-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "petite-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "small-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "titling-caps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-caps",
        "unicase",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "full-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "jis04",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "jis78",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "jis83",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "jis90",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "proportional-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "ruby",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "simplified",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-east-asian",
        "traditional",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-emoji",
        "emoji",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-emoji",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-emoji",
        "text",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-emoji",
        "unicode",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-ligatures",
        "common-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "contextual",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "discretionary-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "historical-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "no-common-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "no-contextual",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "no-discretionary-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "no-historical-ligatures",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-ligatures",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "diagonal-fractions",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "lining-nums",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "oldstyle-nums",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "ordinal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "proportional-nums",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "slashed-zero",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "stacked-fractions",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-numeric",
        "tabular-nums",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "font-variant-position",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-position",
        "sub",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-variant-position",
        "super",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-weight",
        "bold",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-weight",
        "bolder",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-weight",
        "lighter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-weight",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-weight",
        "number",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "font-width",
        "condensed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "expanded",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "extra-condensed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "extra-expanded",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "semi-condensed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "semi-expanded",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "ultra-condensed",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-width",
        "ultra-expanded",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "forced-color-adjust",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "forced-color-adjust",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "forced-color-adjust",
        "preserve-parent-color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "gap",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-flow",
        "column",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-flow",
        "dense",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-auto-flow",
        "row",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-areas",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "animation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "grid-template-columns",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "masonry",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "grid-template-columns",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "minmax",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-columns",
        "subgrid",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "grid-template-rows",
        "animation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "grid-template-rows",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "masonry",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "grid-template-rows",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "minmax",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "repeat",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "grid-template-rows",
        "subgrid",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "hanging-punctuation",
        "allow-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hanging-punctuation",
        "first",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hanging-punctuation",
        "last",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hanging-punctuation",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "height",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "height",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "height",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "height",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "height",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "height",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hyphenate-character",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "hyphenate-limit-chars",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hyphens",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "image-orientation",
        "from-image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "image-orientation",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "image-rendering",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "image-rendering",
        "crisp-edges",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "image-rendering",
        "pixelated",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "image-rendering",
        "smooth",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "initial-letter",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inline-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inline-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inline-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inline-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block-end",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block-end",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block-end",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-block-start",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block-start",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-block-start",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline-end",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline-end",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline-end",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "inset-inline-start",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline-start",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "inset-inline-start",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "interactivity",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "interactivity",
        "inert",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "interpolate-size",
        "allow-keywords",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "interpolate-size",
        "numeric-only",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "justify-content",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-content",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-content",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-items",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "justify-items",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-items",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-items",
        "legacy",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-items",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "justify-self",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "justify-self",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "justify-self",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "justify-self",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "justify-self",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "justify-self",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "left",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "left",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "left",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "letter-spacing",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "lighting-color",
        "currentColor",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "lighting-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "line-break",
        "anywhere",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-break",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-break",
        "loose",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-break",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-break",
        "strict",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "line-clamp",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "line-height",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style",
        "inside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style",
        "outside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style",
        "symbols",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-image",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-position",
        "inside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-position",
        "outside",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "arabic-indic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "armenian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "bengali",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "cambodian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "circle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "cjk-decimal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "cjk-earthly-branch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "cjk-heavenly-stem",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "cjk-ideographic",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "decimal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "decimal-leading-zero",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "devanagari",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "disc",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "disclosure-closed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "disclosure-open",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "ethiopic-numeric",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "georgian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "gujarati",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "gurmukhi",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "hebrew",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "hiragana",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "hiragana-iroha",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "japanese-formal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "japanese-informal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "kannada",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "katakana",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "katakana-iroha",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "khmer",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "korean-hangul-formal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "korean-hanja-formal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "korean-hanja-informal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lao",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lower-alpha",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lower-armenian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lower-greek",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lower-latin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "lower-roman",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "malayalam",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "mongolian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "myanmar",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "oriya",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "persian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "simp-chinese-formal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "simp-chinese-informal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "square",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "string",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "symbols",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "tamil",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "telugu",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "thai",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "tibetan",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "trad-chinese-formal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "trad-chinese-informal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "upper-alpha",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "upper-armenian",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "upper-latin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "list-style-type",
        "upper-roman",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-block",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-block-end",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-block-start",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-bottom",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-bottom",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-inline",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-inline-end",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-inline-start",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-left",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-left",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-right",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-right",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-top",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-top",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "margin-trim",
        "block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "block-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "block-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "inline-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "inline-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "margin-trim",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "marker",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-end",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-mid",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "marker-start",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "mask-clip",
        "border",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-clip",
        "content",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-clip",
        "padding",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-clip",
        "text",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-composite",
        "add",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-composite",
        "exclude",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-composite",
        "intersect",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-composite",
        "subtract",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-mode",
        "alpha",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-mode",
        "luminance",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-mode",
        "match-source",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "border",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "content",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "fill-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "padding",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "stroke-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-origin",
        "view-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "mask-type",
        "alpha",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "mask-type",
        "luminance",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "max-block-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "max-block-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-block-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-block-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-height",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "max-height",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-height",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "max-height",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "max-height",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "max-height",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "max-inline-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "max-inline-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-inline-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-inline-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-width",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "max-width",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "max-width",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "max-width",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "max-width",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "max-width",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-block-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-block-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-block-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-block-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-height",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-height",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "min-height",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-height",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "min-height",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "min-height",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-inline-size",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-inline-size",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-inline-size",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-inline-size",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-width",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "min-width",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "min-width",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "min-width",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "min-width",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "min-width",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "mix-blend-mode",
        "plus-darker",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "mix-blend-mode",
        "plus-lighter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        "cover",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        "fill",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-fit",
        "scale-down",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "object-view-box",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "offset-anchor",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-anchor",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "fill-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "margin-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "padding-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "path",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "ray",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "stroke-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "url",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-path",
        "view-box",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-position",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-rotate",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "offset-rotate",
        "reverse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "opacity",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "outline",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "outline-color",
        "transparent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "dashed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "dotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "double",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "groove",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "outset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "ridge",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-style",
        "solid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-width",
        "medium",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-width",
        "thick",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "outline-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "overflow",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow",
        "clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overflow",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow",
        "visible",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-anchor",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-anchor",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-block",
        "overlay",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "overflow-clip-margin",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-clip-margin",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-clip-margin",
        "padding-box",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overflow-inline",
        "overlay",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "overflow-wrap",
        "anywhere",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "overflow-wrap",
        "break-word",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "overflow-wrap",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "overflow-x",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-x",
        "clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overflow-x",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-x",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-x",
        "visible",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-y",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-y",
        "clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overflow-y",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-y",
        "scroll",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overflow-y",
        "visible",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "overlay",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overlay",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "overscroll-behavior",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-block",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-block",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-block",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-inline",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-inline",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-inline",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-x",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-x",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-x",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-y",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-y",
        "contain",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "overscroll-behavior-y",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "page-break-after",
        "always",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-after",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-after",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-after",
        "left",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-after",
        "right",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        "always",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        "left",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-before",
        "right",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-inside",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "page-break-inside",
        "avoid",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "perspective",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "perspective-origin",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "place-items",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "place-self",
        "anchor-center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position",
        "absolute",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position",
        "fixed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position",
        "relative",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position",
        "static",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "position",
        "sticky",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "position-anchor",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-anchor",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "block-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "block-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "center",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "inline-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "inline-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "left",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "right",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-block-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-block-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-inline-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-inline-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-all",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-block-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-block-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-bottom",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-inline-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-inline-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-left",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-right",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-block-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-block-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-inline-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-inline-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-top",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "span-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "top",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-area",
        "y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "span-self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "span-self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "span-self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try",
        "span-self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "flip-block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "flip-inline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "flip-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "position-area",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "span-self-x-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "span-self-x-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "span-self-y-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-fallbacks",
        "span-self-y-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        "most-block-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        "most-height",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        "most-inline-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        "most-width",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-try-order",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-visibility",
        "always",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-visibility",
        "anchors-visible",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "position-visibility",
        "no-overflow",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "print-color-adjust",
        "economy",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "print-color-adjust",
        "exact",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "quotes",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "quotes",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "reading-flow",
        "flex-flow",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "flex-visual",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "grid-columns",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "grid-order",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "grid-rows",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "reading-flow",
        "source-order",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "resize",
        "block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "resize",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "right",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "right",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "right",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "rotate",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "row-gap",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "ruby-align",
        "center",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-align",
        "space-around",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-align",
        "space-between",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-align",
        "start",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-overhang",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "ruby-overhang",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "ruby-position",
        "alternate",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "ruby-position",
        "inter-character",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-position",
        "over",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "ruby-position",
        "under",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scale",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scroll-behavior",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scroll-behavior",
        "smooth",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scroll-initial-target",
        "nearest",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-initial-target",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-marker-group",
        "after",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-marker-group",
        "before",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-marker-group",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-padding",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block-end",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-block-start",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline-end",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-padding-inline-start",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-align",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-align",
        "end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-align",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-align",
        "start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-stop",
        "always",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-stop",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "block",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "both",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-snap-type",
        "y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-target-group",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-target-group",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-axis",
        "block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-axis",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-axis",
        "x",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-timeline-axis",
        "y",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scrollbar-color",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "scrollbar-gutter",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scrollbar-gutter",
        "stable",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scrollbar-width",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scrollbar-width",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scrollbar-width",
        "thin",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "shape-image-threshold",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "circle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "path",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-outside",
        "polygon",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "shape-rendering",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "speak-as",
        "digits",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "speak-as",
        "literal-punctuation",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "speak-as",
        "no-punctuation",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "speak-as",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "speak-as",
        "spell-out",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "stroke",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-dasharray",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linecap",
        "butt",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linecap",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linecap",
        "square",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linejoin",
        "bevel",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linejoin",
        "miter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "stroke-linejoin",
        "round",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "tab-size",
        "length",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "table-layout",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "table-layout",
        "fixed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "justify",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "match-parent",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align",
        "start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-align-last",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-anchor",
        "end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-anchor",
        "middle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-anchor",
        "start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-autospace",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "ideograph-alpha",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "ideograph-numeric",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "insert",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "no-autospace",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "punctuation",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-autospace",
        "replace",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-edge",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-trim",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-trim",
        "trim-both",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-trim",
        "trim-end",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-box-trim",
        "trim-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-combine-upright",
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-combine-upright",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-decoration-line",
        "blink",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-decoration-line",
        "grammar-error",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        "line-through",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        "overline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        "spelling-error",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-line",
        "underline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip-ink",
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip-ink",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-skip-ink",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-style",
        "wavy",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-thickness",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-thickness",
        "from-font",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-decoration-thickness",
        "percentage",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-emphasis-position",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-position",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-position",
        "over",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-position",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-position",
        "under",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "circle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "dot",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "double-circle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "filled",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "sesame",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-emphasis-style",
        "triangle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "text-indent",
        "each-line",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-indent",
        "hanging",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-justify",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-justify",
        "inter-character",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-justify",
        "inter-word",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-justify",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-orientation",
        "mixed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-orientation",
        "sideways",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-orientation",
        "upright",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-overflow",
        "clip",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-overflow",
        "ellipsis",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-overflow",
        "string",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-rendering",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-rendering",
        "geometricPrecision",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-size-adjust",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-size-adjust",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-size-adjust",
        "percentages",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-spacing-trim",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-spacing-trim",
        "space-all",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-spacing-trim",
        "space-first",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-spacing-trim",
        "trim-start",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-transform",
        "capitalize",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-transform",
        "full-size-kana",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-transform",
        "full-width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-transform",
        "lowercase",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-transform",
        "math-auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "text-transform",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-transform",
        "uppercase",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "text-underline-offset",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-offset",
        "percentage",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        "from-font",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-underline-position",
        "under",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "text-wrap",
        "balance",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap",
        "nowrap",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap",
        "pretty",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-wrap",
        "stable",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap",
        "wrap",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-mode",
        "nowrap",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-mode",
        "wrap",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-style",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-style",
        "balance",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "text-wrap-style",
        "pretty",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "text-wrap-style",
        "stable",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "timeline-scope",
        "all",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "timeline-scope",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "top",
        "anchor",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "top",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "top",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "touch-action",
        "manipulation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-down",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-up",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-x",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pan-y",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "touch-action",
        "pinch-zoom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2019),
        },
    ),
    (
        "transform",
        "3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "transform",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-box",
        "border-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-box",
        "content-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-box",
        "fill-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-box",
        "stroke-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-box",
        "view-box",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transform-origin",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-origin",
        "center",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-origin",
        "left",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-origin",
        "right",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transform-origin",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "ease",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "ease-in",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "ease-in-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "ease-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "linear",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "step-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "step-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition",
        "transition-behavior",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "transition-property",
        "all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-property",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "ease",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "ease-in",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "ease-in-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "ease-out",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "jump",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "linear",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "step-end",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "transition-timing-function",
        "step-start",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "translate",
        "none",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "unicode-bidi",
        "bidi-override",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "unicode-bidi",
        "embed",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "unicode-bidi",
        "isolate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "unicode-bidi",
        "isolate-override",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "unicode-bidi",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "unicode-bidi",
        "plaintext",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "user-select",
        "all",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "user-select",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "user-select",
        "none",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "user-select",
        "text",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "vertical-align",
        "baseline",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "middle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "sub",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "super",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "text-bottom",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "text-top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "vertical-align",
        "top",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "view-timeline-axis",
        "block",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-axis",
        "inline",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-axis",
        "x",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-axis",
        "y",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-timeline-inset",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "view-transition-class",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-name",
        "match-element",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-name",
        "none",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "visibility",
        "collapse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "visibility",
        "hidden",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "visibility",
        "visible",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "break-spaces",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "nowrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "pre",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "pre-line",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space",
        "pre-wrap",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "white-space-collapse",
        "break-spaces",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "white-space-collapse",
        "collapse",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "white-space-collapse",
        "preserve",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "white-space-collapse",
        "preserve-breaks",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "white-space-collapse",
        "preserve-spaces",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "width",
        "anchor-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "width",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "width",
        "fit-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "width",
        "max-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "width",
        "min-content",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "width",
        "stretch",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "will-change",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "will-change",
        "contents",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "will-change",
        "scroll-position",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "word-break",
        "auto-phrase",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "word-break",
        "break-all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "word-break",
        "break-word",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "word-break",
        "keep-all",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "word-break",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "word-spacing",
        "normal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "writing-mode",
        "horizontal-tb",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "writing-mode",
        "lr",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "lr-tb",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "rl",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "rl-tb",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "sideways-lr",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "writing-mode",
        "sideways-rl",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "writing-mode",
        "tb",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "tb-rl",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "writing-mode",
        "vertical-lr",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "writing-mode",
        "vertical-rl",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2017),
        },
    ),
    (
        "z-index",
        "auto",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
];
#[doc = r" Baseline status for CSS at-rules. Sorted by name."]
pub static BASELINE_AT_RULES: &[(&str, BaselineStatus)] = &[
    (
        "charset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "container",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "counter-style",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "custom-media",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "font-face",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "font-feature-values",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "font-palette-values",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "function",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "import",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "keyframes",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "layer",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "media",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "namespace",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "page",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "position-try",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "property",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "scope",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "starting-style",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "supports",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "view-transition",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
];
#[doc = r" Baseline status for CSS media query conditions. Sorted by name."]
pub static BASELINE_MEDIA_CONDITIONS: &[(&str, BaselineStatus)] = &[
    (
        "-webkit-device-pixel-ratio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "-webkit-max-device-pixel-ratio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "-webkit-min-device-pixel-ratio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "-webkit-transform-3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "any-hover",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "any-pointer",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "aspect-ratio",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "calc",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-gamut",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "color-index",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "device-aspect-ratio",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "device-height",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "device-posture",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "device-width",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "display-mode",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "dynamic-range",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "forced-colors",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "grid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "height",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "horizontal-viewport-segments",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hover",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "inverted-colors",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "media_features",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "media_query_values",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "monochrome",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "nested-queries",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "or_syntax",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "orientation",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "overflow-block",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "overflow-inline",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "pointer",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2018),
        },
    ),
    (
        "prefers-color-scheme",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "prefers-contrast",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "prefers-reduced-data",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "prefers-reduced-motion",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "prefers-reduced-transparency",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "range_syntax",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "resolution",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scripting",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "update",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "vertical-viewport-segments",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "video-dynamic-range",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "width",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
];
#[doc = r" Baseline status for CSS value functions. Sorted by name."]
pub static BASELINE_FUNCTIONS: &[(&str, BaselineStatus)] = &[
    (
        "abs",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "acos",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "asin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "atan",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "atan2",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "attr",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "blur",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "brightness",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "calc",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "calc-size",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "circle",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "clamp",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "color",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "color-mix",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "conic-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "contrast",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "cos",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "counter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "counters",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "cross-fade",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "cubic-bezier",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "drop-shadow",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "element",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "ellipse",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "env",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "exp",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "grayscale",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "hsl",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "hue-rotate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "hwb",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "hypot",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "image",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "image-set",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "inset",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "invert",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "lab",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "lch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "light-dark",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "linear-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "log",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "matrix",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "matrix3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "max",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "min",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "mod",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "oklab",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "oklch",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "opacity",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "paint",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "path",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "perspective",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "polygon",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "pow",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "radial-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "ray",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "rect",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "rem",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "repeating-conic-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "repeating-linear-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "repeating-radial-gradient",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "rgb",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "rotate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "rotate3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "rotateX",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "rotateY",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "rotateZ",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "round",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "saturate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "scale",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "scale3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "scaleX",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "scaleY",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "scaleZ",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "sepia",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
    (
        "sign",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "sin",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "skew",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "skewX",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "skewY",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "sqrt",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "steps",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "string",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "tan",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "translate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "translate3d",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "translateX",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "translateY",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "translateZ",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "url",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2016),
        },
    ),
];
#[doc = r" Baseline status for CSS pseudo-class and pseudo-element selectors. Sorted by name."]
pub static BASELINE_SELECTORS: &[(&str, BaselineStatus)] = &[
    (
        "active",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "active-view-transition",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    ),
    (
        "active-view-transition-type",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2026),
        },
    ),
    (
        "after",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "any-link",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "attribute",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "autofill",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "backdrop",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "before",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "buffering",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "checked",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "checkmark",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "class",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "column",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "cue",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "default",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "defined",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "descendant",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "details-content",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "dir",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "disabled",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "empty",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "enabled",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "file-selector-button",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "first",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "first-child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "first-letter",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "first-line",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "first-of-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "focus",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "focus-visible",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "focus-within",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "fullscreen",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "future",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "grammar-error",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "has",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "has-slotted",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "heading",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "headingfunction",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "highlight",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "host",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "host-context",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "hostfunction",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "hover",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "id",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "in-range",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "indeterminate",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "interest-source",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "interest-target",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "invalid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "is",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "lang",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "last-child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "last-of-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "left",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "link",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "list",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "marker",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "modal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2022),
        },
    ),
    (
        "muted",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "namespace",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "nesting",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "next-sibling",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "not",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "nth-child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "nth-last-child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "nth-last-of-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "nth-of-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "only-child",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "only-of-type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "open",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "optional",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "out-of-range",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "part",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "past",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "paused",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "picker",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "picker-icon",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "picture-in-picture",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "placeholder",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "placeholder-shown",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "playing",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "popover-open",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "read-only",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "read-write",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "required",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "right",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "root",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "scope",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "scroll-button",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-marker",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "scroll-marker-group",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "seeking",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "selection",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "slotted",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "spelling-error",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "stalled",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "state",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "subsequent-sibling",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "target",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "target-after",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "target-before",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "target-current",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "target-text",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2024),
        },
    ),
    (
        "type",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "universal",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "user-invalid",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "user-valid",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2023),
        },
    ),
    (
        "valid",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2015),
        },
    ),
    (
        "view-transition",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-group",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-image-pair",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-new",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "view-transition-old",
        BaselineStatus {
            tier: BaselineTier::Newly,
            year: BaselineYear::Known(2025),
        },
    ),
    (
        "visited",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2020),
        },
    ),
    (
        "volume-locked",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
    (
        "where",
        BaselineStatus {
            tier: BaselineTier::Widely,
            year: BaselineYear::Known(2021),
        },
    ),
    (
        "xr-overlay",
        BaselineStatus {
            tier: BaselineTier::Limited,
            year: BaselineYear::Unknown,
        },
    ),
];
