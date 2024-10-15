/// Configuration related to the
/// [JSX A11y Eslint plugin](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::a11y::use_valid_aria_role;

#[derive(Debug, Default, Deserializable)]
pub(crate) struct AriaRoleOptions {
    allowed_invalid_roles: Box<[Box<str>]>,
    #[deserializable(rename = "ignoreNonDOM")]
    ignore_non_dom: bool,
}
impl From<AriaRoleOptions> for use_valid_aria_role::ValidAriaRoleOptions {
    fn from(val: AriaRoleOptions) -> Self {
        use_valid_aria_role::ValidAriaRoleOptions {
            allow_invalid_roles: val.allowed_invalid_roles,
            ignore_non_dom: val.ignore_non_dom,
        }
    }
}
