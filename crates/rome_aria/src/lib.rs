use std::str::FromStr;

pub mod iso;
mod macros;
pub mod properties;
pub mod roles;

pub use biome_aria_metadata::{AriaPropertiesEnum, AriaPropertyTypeEnum};
pub use properties::AriaProperties;
pub(crate) use roles::AriaRoleDefinition;
pub use roles::AriaRoles;

/// It checks if an ARIA property is valid
///
/// ## Examples
///
/// ```
/// use rome_aria::is_aria_property_valid;
///
/// assert!(!is_aria_property_valid("aria-blabla"));
/// assert!(is_aria_property_valid("aria-checked"));
/// ```
pub fn is_aria_property_valid(property: &str) -> bool {
    AriaPropertiesEnum::from_str(property).is_ok()
}

/// It checks if an ARIA property type is valid
///
/// ## Examples
///
/// ```
/// use rome_aria::is_aria_property_type_valid;
///
/// assert!(is_aria_property_type_valid("string"));
/// assert!(!is_aria_property_type_valid("bogus"));
/// ```
pub fn is_aria_property_type_valid(property_type: &str) -> bool {
    AriaPropertyTypeEnum::from_str(property_type).is_ok()
}

#[cfg(test)]
mod test {
    use crate::roles::AriaRoles;

    #[test]
    fn property_is_required() {
        let roles = AriaRoles;

        let role = roles.get_role("checkbox");

        assert!(role.is_some());

        let role = role.unwrap();

        assert!(role.is_property_required("aria-checked"));
        assert!(!role.is_property_required("aria-sort"));
        assert!(!role.is_property_required("aria-bnlabla"));
    }
}
