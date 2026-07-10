use biome_string_case::StrLikeExtension;

/// Shared classification for CSS grid-template property names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssGridTemplateProperty {
    /// `grid`
    Grid,
    /// `grid-template`
    GridTemplate,
    /// `grid-template-areas`
    GridTemplateAreas,
    /// `grid-template-columns`
    GridTemplateColumns,
    /// `grid-template-rows`
    GridTemplateRows,
    /// Other `grid-template-*` properties.
    OtherGridTemplateSubProperty,
}

impl CssGridTemplateProperty {
    /// Returns `true` for declarations that can contain named grid-area strings.
    ///
    /// ```css
    /// a { grid: "a a" auto / 1fr 1fr; }
    /// a { grid-template: "a a" auto / 1fr 1fr; }
    /// a { grid-template-areas: "a a"; }
    /// ```
    pub const fn is_grid_area_property(self) -> bool {
        matches!(
            self,
            Self::Grid | Self::GridTemplate | Self::GridTemplateAreas
        )
    }
}

/// Classifies `grid`, `grid-template`, and hyphenated `grid-template-*` names.
///
/// ```css
/// a { grid: "a" auto / 1fr; }
/// a { grid-template-rows: auto; }
/// a { grid-templatefoo: auto; }
/// ```
///
/// `grid-templatefoo` is not classified because it is not a `grid-template-*`
/// subproperty.
pub fn css_grid_template_property(name: &str) -> Option<CssGridTemplateProperty> {
    let name = name.to_ascii_lowercase_cow();

    match name.as_ref() {
        "grid" => Some(CssGridTemplateProperty::Grid),
        "grid-template" => Some(CssGridTemplateProperty::GridTemplate),
        "grid-template-areas" => Some(CssGridTemplateProperty::GridTemplateAreas),
        "grid-template-columns" => Some(CssGridTemplateProperty::GridTemplateColumns),
        "grid-template-rows" => Some(CssGridTemplateProperty::GridTemplateRows),
        name if name.starts_with("grid-template-") => {
            Some(CssGridTemplateProperty::OtherGridTemplateSubProperty)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{CssGridTemplateProperty, css_grid_template_property};

    #[test]
    fn classifies_grid_template_properties() {
        assert_eq!(
            css_grid_template_property("grid"),
            Some(CssGridTemplateProperty::Grid)
        );
        assert_eq!(
            css_grid_template_property("grid-template"),
            Some(CssGridTemplateProperty::GridTemplate)
        );
        assert_eq!(
            css_grid_template_property("grid-template-areas"),
            Some(CssGridTemplateProperty::GridTemplateAreas)
        );
        assert_eq!(
            css_grid_template_property("grid-template-rows"),
            Some(CssGridTemplateProperty::GridTemplateRows)
        );
        assert_eq!(
            css_grid_template_property("grid-template-columns"),
            Some(CssGridTemplateProperty::GridTemplateColumns)
        );
        assert_eq!(
            css_grid_template_property("grid-template-custom"),
            Some(CssGridTemplateProperty::OtherGridTemplateSubProperty)
        );
        assert_eq!(
            css_grid_template_property("Grid-Template-Custom"),
            Some(CssGridTemplateProperty::OtherGridTemplateSubProperty)
        );

        assert_eq!(css_grid_template_property("grid-templatefoo"), None);
    }
}
