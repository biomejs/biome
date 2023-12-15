use crate::define_property;
use biome_aria_metadata::AriaPropertyTypeEnum;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

define_property! {
    AriaActivedescendant {
        PROPERTY_TYPE: "id",
        VALUES: [],
    }
}

define_property! {
    AriaAtomic {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaAutocomplete {
        PROPERTY_TYPE: "token",
        VALUES: ["inline", "list", "both", "none"],
    }
}

define_property! {
    AriaBusy {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaChecked {
        PROPERTY_TYPE: "tristate",
        VALUES: [],
    }
}

define_property! {
    AriaColcount {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaColindex {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaColspan {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaControls {
        PROPERTY_TYPE: "idlist",
        VALUES: [],
    }
}

define_property! {
    AriaCurrent {
        PROPERTY_TYPE: "token",
        VALUES: ["page", "step", "location", "date", "time", "true", "false"],
    }
}

define_property! {
    AriaDescribedby {
        PROPERTY_TYPE: "idlist",
        VALUES: [],
    }
}

define_property! {
    AriaDetails {
        PROPERTY_TYPE: "id",
        VALUES: [],
    }
}

define_property! {
    AriaDisabled {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaDropeffect {
        PROPERTY_TYPE: "tokenlist",
        VALUES: ["copy", "execute", "link", "move", "none", "popup"],
    }
}

define_property! {
    AriaErrormessage {
        PROPERTY_TYPE: "id",
        VALUES: [],
    }
}

define_property! {
    AriaExpanded {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaFlowto {
        PROPERTY_TYPE: "idlist",
        VALUES: [],
    }
}

define_property! {
    AriaGrabbed {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaHaspopup {
        PROPERTY_TYPE: "token",
        VALUES: ["false", "true", "menu", "listbox", "tree", "grid", "dialog"],
    }
}

define_property! {
    AriaHidden {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaInvalid {
        PROPERTY_TYPE: "token",
        VALUES: ["grammar", "false", "spelling", "true"],
    }
}

define_property! {
    AriaKeyshortcuts {
        PROPERTY_TYPE: "string",
        VALUES: [],
    }
}

define_property! {
    AriaLabel {
        PROPERTY_TYPE: "string",
        VALUES: [],
    }
}

define_property! {
    AriaLabelledby {
        PROPERTY_TYPE: "idlist",
        VALUES: [],
    }
}

define_property! {
    AriaLevel {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaLive {
        PROPERTY_TYPE: "token",
        VALUES: ["assertive", "off", "polite"],
    }
}

define_property! {
    AriaModal {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}
define_property! {
    AriaMultiline {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}
define_property! {
    AriaMultiselectable {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaOrientation {
        PROPERTY_TYPE: "token",
        VALUES: ["vertical", "undefined", "horizontal"],
    }
}

define_property! {
    AriaOwns {
        PROPERTY_TYPE: "idlist",
        VALUES: [],
    }
}

define_property! {
    AriaPlaceholder {
        PROPERTY_TYPE: "string",
        VALUES: [],
    }
}

define_property! {
    AriaPosinset {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaPressed {
        PROPERTY_TYPE: "tristate",
        VALUES: [],
    }
}

define_property! {
    AriaReadonly {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaRelevant {
        PROPERTY_TYPE: "tokenlist",
        VALUES: ["additions", "all", "removals", "text"],
    }
}

define_property! {
    AriaRequired {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaRoledescription {
        PROPERTY_TYPE: "string",
        VALUES: [],
    }
}

define_property! {
    AriaRowcount {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaRowindex {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaRowspan {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaSelected {
        PROPERTY_TYPE: "boolean",
        VALUES: [],
    }
}

define_property! {
    AriaSetsize {
        PROPERTY_TYPE: "integer",
        VALUES: [],
    }
}

define_property! {
    AriaSort {
        PROPERTY_TYPE: "token",
        VALUES: ["ascending", "descending", "none", "other"],
    }
}

define_property! {
    AriaValuemax {
        PROPERTY_TYPE: "number",
        VALUES: [],
    }
}

define_property! {
    AriaValuemin {
        PROPERTY_TYPE: "number",
        VALUES: [],
    }
}

define_property! {
    AriaValuenow {
        PROPERTY_TYPE: "number",
        VALUES: [],
    }
}

define_property! {
    AriaValuetext {
        PROPERTY_TYPE: "string",
        VALUES: [],
    }
}
/// A collection of ARIA properties with their metadata, necessary to perform various operations.
#[derive(Debug, Default)]
pub struct AriaProperties;

impl AriaProperties {
    pub fn get_property<'a>(&self, property_name: &str) -> Option<&'a dyn AriaPropertyDefinition> {
        Some(match property_name {
            "aria-activedescendant" => &AriaActivedescendant as &dyn AriaPropertyDefinition,
            "aria-autocomplete" => &AriaAutocomplete as &dyn AriaPropertyDefinition,
            "aria-busy" => &AriaBusy as &dyn AriaPropertyDefinition,
            "aria-checked" => &AriaChecked as &dyn AriaPropertyDefinition,
            "aria-colcount" => &AriaColcount as &dyn AriaPropertyDefinition,
            "aria-colindex" => &AriaColindex as &dyn AriaPropertyDefinition,
            "aria-colspan" => &AriaColspan as &dyn AriaPropertyDefinition,
            "aria-controls" => &AriaControls as &dyn AriaPropertyDefinition,
            "aria-current" => &AriaCurrent as &dyn AriaPropertyDefinition,
            "aria-describedby" => &AriaDescribedby as &dyn AriaPropertyDefinition,
            "aria-details" => &AriaDetails as &dyn AriaPropertyDefinition,
            "aria-disabled" => &AriaDisabled as &dyn AriaPropertyDefinition,
            "aria-dropeffect" => &AriaDropeffect as &dyn AriaPropertyDefinition,
            "aria-errormessage" => &AriaErrormessage as &dyn AriaPropertyDefinition,
            "aria-expanded" => &AriaExpanded as &dyn AriaPropertyDefinition,
            "aria-flowto" => &AriaFlowto as &dyn AriaPropertyDefinition,
            "aria-grabbed" => &AriaGrabbed as &dyn AriaPropertyDefinition,
            "aria-haspopup" => &AriaHaspopup as &dyn AriaPropertyDefinition,
            "aria-hidden" => &AriaHidden as &dyn AriaPropertyDefinition,
            "aria-invalid" => &AriaInvalid as &dyn AriaPropertyDefinition,
            "aria-keyshortcuts" => &AriaKeyshortcuts as &dyn AriaPropertyDefinition,
            "aria-label" => &AriaLabel as &dyn AriaPropertyDefinition,
            "aria-labelledby" => &AriaLabelledby as &dyn AriaPropertyDefinition,
            "aria-level" => &AriaLevel as &dyn AriaPropertyDefinition,
            "aria-live" => &AriaLive as &dyn AriaPropertyDefinition,
            "aria-modal" => &AriaModal as &dyn AriaPropertyDefinition,
            "aria-multiline" => &AriaMultiline as &dyn AriaPropertyDefinition,
            "aria-multiselectable" => &AriaMultiselectable as &dyn AriaPropertyDefinition,
            "aria-orientation" => &AriaOrientation as &dyn AriaPropertyDefinition,
            "aria-owns" => &AriaOwns as &dyn AriaPropertyDefinition,
            "aria-placeholder" => &AriaPlaceholder as &dyn AriaPropertyDefinition,
            "aria-posinset" => &AriaPosinset as &dyn AriaPropertyDefinition,
            "aria-pressed" => &AriaPressed as &dyn AriaPropertyDefinition,
            "aria-readonly" => &AriaReadonly as &dyn AriaPropertyDefinition,
            "aria-relevant" => &AriaRelevant as &dyn AriaPropertyDefinition,
            "aria-required" => &AriaRequired as &dyn AriaPropertyDefinition,
            "aria-roledescription" => &AriaRoledescription as &dyn AriaPropertyDefinition,
            "aria-rowcount" => &AriaRowcount as &dyn AriaPropertyDefinition,
            "aria-rowindex" => &AriaRowindex as &dyn AriaPropertyDefinition,
            "aria-rowspan" => &AriaRowspan as &dyn AriaPropertyDefinition,
            "aria-selected" => &AriaSelected as &dyn AriaPropertyDefinition,
            "aria-setsize" => &AriaSetsize as &dyn AriaPropertyDefinition,
            "aria-sort" => &AriaSort as &dyn AriaPropertyDefinition,
            "aria-valuemax" => &AriaValuemax as &dyn AriaPropertyDefinition,
            "aria-valuemin" => &AriaValuemin as &dyn AriaPropertyDefinition,
            "aria-valuenow" => &AriaValuenow as &dyn AriaPropertyDefinition,
            "aria-valuetext" => &AriaValuetext as &dyn AriaPropertyDefinition,
            "aria-atomic" => &AriaAtomic as &dyn AriaPropertyDefinition,
            _ => return None,
        })
    }
}

pub trait AriaPropertyDefinition: Debug {
    /// Returns the allowed values by this property
    fn values(&self) -> Iter<&str>;

    /// Returns the property type
    fn property_type(&self) -> AriaPropertyTypeEnum;

    /// It checks if a value is complaint for the current ARIA property.
    ///
    /// [Source](https://www.w3.org/TR/wai-aria-1.1/#propcharacteristic_value)
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_aria::AriaProperties;
    ///
    /// let aria_properties = AriaProperties::default();
    ///
    /// let aria_current = aria_properties.get_property("aria-current").unwrap();
    ///
    /// assert!(aria_current.contains_correct_value("true"));
    /// assert!(aria_current.contains_correct_value("false"));
    /// assert!(aria_current.contains_correct_value("step"));
    /// assert!(!aria_current.contains_correct_value("something_not_allowed"));
    /// ```
    fn contains_correct_value(&self, input_value: &str) -> bool {
        if input_value.is_empty() {
            return false;
        }
        match self.property_type() {
            AriaPropertyTypeEnum::String | AriaPropertyTypeEnum::Id => {
                input_value.parse::<f32>().is_err()
            }
            AriaPropertyTypeEnum::Idlist => input_value
                .split(' ')
                .any(|piece| piece.parse::<f32>().is_err()),
            // A numerical value without a fractional component.
            AriaPropertyTypeEnum::Integer => input_value.parse::<u32>().is_ok(),
            AriaPropertyTypeEnum::Number => input_value.parse::<f32>().is_ok(),
            AriaPropertyTypeEnum::Boolean => {
                matches!(input_value, "false" | "true")
            }
            AriaPropertyTypeEnum::Token => self
                .values()
                .any(|allowed_token| *allowed_token == input_value),
            AriaPropertyTypeEnum::Tokenlist => input_value.split(' ').all(|input_token| {
                self.values()
                    .any(|allowed_token| allowed_token.trim() == input_token)
            }),
            AriaPropertyTypeEnum::Tristate => {
                matches!(input_value, "false" | "true" | "mixed")
            }
        }
    }
}
