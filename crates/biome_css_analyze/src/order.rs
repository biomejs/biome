use std::{collections::HashMap, sync::LazyLock};

/// This is the desired order of CSS properties used by useSortedProperties.
///
/// It's the same ordering used by stylelint-config-recess-order, except in the following ways:
/// - Vendor prefixes removed
/// - IE filters (progid:DXImageTransform) removed
/// - Removed properties not known to biome:
///   - font-effect, font-emphasize, font-emphasize-position, font-emphasize-style,
///   - font-smooth, text-outline, text-overflow-ellipsis, text-overflow-mode, nav-index, color-profile
pub(crate) const PROPERTY_ORDER: [&str; 370] = [
    // the all property will always be first since it overrides everything
    "all",
    // position
    "position",
    "inset",
    "inset-block",
    "inset-block-start",
    "inset-block-end",
    "inset-inline",
    "inset-inline-start",
    "inset-inline-end",
    "top",
    "right",
    "bottom",
    "left",
    "z-index",
    // display
    "box-sizing",
    "display",
    // layout - flex & grid
    "flex",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "flex-flow",
    "flex-direction",
    "flex-wrap",
    "box-orient",
    "grid",
    "grid-area",
    "grid-template",
    "grid-template-areas",
    "grid-template-rows",
    "grid-template-columns",
    "grid-row",
    "grid-row-start",
    "grid-row-end",
    "grid-column",
    "grid-column-start",
    "grid-column-end",
    "grid-auto-rows",
    "grid-auto-columns",
    "grid-auto-flow",
    "grid-gap",
    "grid-row-gap",
    "grid-column-gap",
    "gap",
    "row-gap",
    "column-gap",
    "place-content",
    "place-items",
    "place-self",
    "align-content",
    "align-items",
    "align-self",
    "justify-content",
    "justify-items",
    "justify-self",
    "order",
    // layout - box model
    "float",
    "inline-size",
    "min-inline-size",
    "max-inline-size",
    "width",
    "min-width",
    "max-width",
    "block-size",
    "min-block-size",
    "max-block-size",
    "height",
    "min-height",
    "max-height",
    "aspect-ratio",
    "padding",
    "padding-block",
    "padding-block-start",
    "padding-block-end",
    "padding-inline",
    "padding-inline-start",
    "padding-inline-end",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "margin",
    "margin-block",
    "margin-block-start",
    "margin-block-end",
    "margin-inline",
    "margin-inline-start",
    "margin-inline-end",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "overflow",
    "overflow-block",
    "overflow-inline",
    "overflow-x",
    "overflow-y",
    "overscroll-behavior",
    "overscroll-behavior-inline",
    "overscroll-behavior-block",
    "overscroll-behavior-x",
    "overscroll-behavior-y",
    "clip",
    "clip-path",
    "clear",
    // typography
    "font",
    "font-family",
    "font-size",
    "font-variation-settings",
    "font-style",
    "font-weight",
    "font-feature-settings",
    "font-optical-sizing",
    "font-kerning",
    "font-variant",
    "font-variant-ligatures",
    "font-variant-caps",
    "font-variant-alternates",
    "font-variant-numeric",
    "font-variant-east-asian",
    "font-variant-position",
    "font-size-adjust",
    "font-stretch",
    "hyphens",
    "line-height",
    "color",
    "text-align",
    "text-align-last",
    "text-emphasis",
    "text-emphasis-color",
    "text-emphasis-style",
    "text-emphasis-position",
    "text-decoration",
    "text-decoration-line",
    "text-decoration-thickness",
    "text-decoration-style",
    "text-decoration-color",
    "text-underline-position",
    "text-underline-offset",
    "text-indent",
    "text-justify",
    "text-overflow",
    "line-clamp",
    "text-shadow",
    "text-transform",
    "text-wrap",
    "letter-spacing",
    "word-break",
    "word-spacing",
    "word-wrap",
    "overflow-wrap",
    "tab-size",
    "white-space",
    "vertical-align",
    "list-style",
    "list-style-position",
    "list-style-type",
    "list-style-image",
    "src",
    "font-display",
    "unicode-range",
    "size-adjust",
    "ascent-override",
    "descent-override",
    "line-gap-override",
    // a11y
    "appearance",
    "accent-color",
    "color-scheme",
    "pointer-events",
    "touch-action",
    "cursor",
    "caret-color",
    "visibility",
    "zoom",
    "table-layout",
    "empty-cells",
    "caption-side",
    "border-spacing",
    "border-collapse",
    "content",
    "quotes",
    "counter-reset",
    "counter-set",
    "counter-increment",
    "resize",
    "scroll-behavior",
    "scroll-snap-type",
    "scroll-snap-align",
    "scroll-snap-stop",
    "scroll-padding",
    "scroll-padding-inline",
    "scroll-padding-inline-start",
    "scroll-padding-inline-end",
    "scroll-padding-block",
    "scroll-padding-block-start",
    "scroll-padding-block-end",
    "scroll-padding-top",
    "scroll-padding-right",
    "scroll-padding-bottom",
    "scroll-padding-left",
    "scroll-margin",
    "scroll-margin-inline",
    "scroll-margin-inline-start",
    "scroll-margin-inline-end",
    "scroll-margin-block",
    "scroll-margin-block-start",
    "scroll-margin-block-end",
    "scroll-margin-top",
    "scroll-margin-right",
    "scroll-margin-bottom",
    "scroll-margin-left",
    "scrollbar-color",
    "scrollbar-gutter",
    "scrollbar-width",
    "user-select",
    "nav-up",
    "nav-right",
    "nav-down",
    "nav-left",
    // backgrounds
    "object-fit",
    "object-position",
    "image-orientation",
    "image-rendering",
    "image-resolution",
    "background",
    "background-color",
    "background-image",
    "filter",
    "background-repeat",
    "background-attachment",
    "background-position",
    "background-position-x",
    "background-position-y",
    "background-clip",
    "background-origin",
    "background-size",
    "background-blend-mode",
    "isolation",
    "backdrop-filter",
    // borders
    "border",
    "border-color",
    "border-style",
    "border-width",
    "border-block",
    "border-block-start",
    "border-block-start-color",
    "border-block-start-style",
    "border-block-start-width",
    "border-block-end",
    "border-block-end-color",
    "border-block-end-style",
    "border-block-end-width",
    "border-inline",
    "border-inline-start",
    "border-inline-start-color",
    "border-inline-start-style",
    "border-inline-start-width",
    "border-inline-end",
    "border-inline-end-color",
    "border-inline-end-style",
    "border-inline-end-width",
    "border-top",
    "border-top-color",
    "border-top-style",
    "border-top-width",
    "border-right",
    "border-right-color",
    "border-right-style",
    "border-right-width",
    "border-bottom",
    "border-bottom-color",
    "border-bottom-style",
    "border-bottom-width",
    "border-left",
    "border-left-color",
    "border-left-style",
    "border-left-width",
    "border-radius",
    "border-start-start-radius",
    "border-start-end-radius",
    "border-end-start-radius",
    "border-end-end-radius",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-bottom-right-radius",
    "border-bottom-left-radius",
    "border-image",
    "border-image-source",
    "border-image-slice",
    "border-image-width",
    "border-image-outset",
    "border-image-repeat",
    "outline",
    "outline-width",
    "outline-style",
    "outline-color",
    "outline-offset",
    "box-shadow",
    "mix-blend-mode",
    "opacity",
    // mask
    "mask-border",
    "mask-border-source",
    "mask-border-slice",
    "mask-border-width",
    "mask-border-outset",
    "mask-border-repeat",
    "mask-border-mode",
    "mask",
    "mask-image",
    "mask-mode",
    "mask-repeat",
    "mask-position",
    "mask-clip",
    "mask-origin",
    "mask-size",
    "mask-composite",
    // svg
    "alignment-baseline",
    "baseline-shift",
    "dominant-baseline",
    "text-anchor",
    "word-spacing",
    "writing-mode",
    "fill",
    "fill-opacity",
    "fill-rule",
    "stroke",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-opacity",
    "stroke-width",
    "color-interpolation",
    "color-interpolation-filters",
    "color-rendering",
    "flood-color",
    "flood-opacity",
    "lighting-color",
    "marker-start",
    "marker-mid",
    "marker-end",
    "shape-rendering",
    "stop-color",
    "stop-opacity",
    // animations
    "transition",
    "transition-delay",
    "transition-timing-function",
    "transition-duration",
    "transition-property",
    "transform",
    "transform-origin",
    "rotate",
    "scale",
    "translate",
    "perspective",
    "perspective-origin",
    "animation",
    "animation-name",
    "animation-duration",
    "animation-play-state",
    "animation-timing-function",
    "animation-delay",
    "animation-iteration-count",
    "animation-direction",
    "will-change",
    // pagination
    "break-before",
    "break-inside",
    "break-after",
    "orphans",
    "widows",
];

/// A map from CSS property names to the index at which they appear in the [PROPERTY_ORDER] array.
pub(crate) static PROPERTY_ORDER_MAP: LazyLock<HashMap<&'static str, usize>> =
    LazyLock::new(|| {
        let mut map = HashMap::with_capacity(PROPERTY_ORDER.len());
        for (i, p) in PROPERTY_ORDER.iter().enumerate() {
            map.insert(*p, i);
        }
        map
    });

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        keywords::VENDOR_PREFIXES,
        order::PROPERTY_ORDER_MAP,
        utils::{
            get_longhand_sub_properties, get_reset_to_initial_properties, is_known_properties,
            vendor_prefixed,
        },
    };

    use super::PROPERTY_ORDER;

    #[test]
    fn test_no_shorthand_after_longhand_in_order_list() {
        let mut disallowed = HashSet::<&str>::new();
        // backwards iteration allows this to make use of get_longhand_sub_properties
        for prop in PROPERTY_ORDER.iter().rev() {
            // assert this property is not in the set yet
            assert!(
                !disallowed.contains(prop),
                "{} must be ordered after any properties that may override it",
                prop
            );

            // record disallowed properties
            for longhand_child_property in get_longhand_sub_properties(prop) {
                disallowed.insert(longhand_child_property);
            }
            for longhand_child_property in get_reset_to_initial_properties(prop) {
                disallowed.insert(longhand_child_property);
            }
        }
    }

    #[test]
    fn test_no_vendor_prefixes_in_order_list() {
        for prop in PROPERTY_ORDER {
            assert!(
                !vendor_prefixed(prop),
                "{} should not have a vendor prefix",
                prop
            );
        }
    }

    #[test]
    fn test_no_unknown_properties_in_order_list() {
        'outer: for prop in PROPERTY_ORDER {
            if is_known_properties(prop) {
                continue;
            }
            for prefix in VENDOR_PREFIXES {
                if is_known_properties(&(prefix.to_owned() + prop)) {
                    continue 'outer;
                }
            }
            panic!("unknown property {}", prop)
        }
    }

    #[test]
    fn test_properties_ending_in_start_come_before_properties_ending_in_end() {
        for prop in PROPERTY_ORDER {
            if prop.contains("start") {
                let with_end = prop.replace("start", "end");
                assert!(
                    PROPERTY_ORDER_MAP.get(prop) < PROPERTY_ORDER_MAP.get(with_end.as_str()),
                    "{} should be before {}",
                    prop,
                    with_end
                );
            }
        }
    }
}
