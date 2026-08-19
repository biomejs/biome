use crate::keywords::{LENGTH_UNITS, RESOLUTION_UNITS};
use crate::{AnyCssFunction, AnyCssValue, CssRegularDimension, decode_css_identifier};
use biome_rowan::TokenText;
use biome_string_case::StrLikeExtension;

impl AnyCssValue {
    /// Returns the regular dimension represented by this value.
    pub fn as_css_regular_dimension(&self) -> Option<&CssRegularDimension> {
        self.as_any_css_dimension()?.as_css_regular_dimension()
    }

    /// Returns the token text when this value is an identifier, custom identifier, or dashed
    /// identifier.
    pub fn identifier_text(&self) -> Option<TokenText> {
        match self {
            Self::CssIdentifier(identifier) => {
                Some(identifier.value_token().ok()?.token_text_trimmed())
            }
            Self::CssCustomIdentifier(identifier) => {
                Some(identifier.value_token().ok()?.token_text_trimmed())
            }
            Self::AnyCssDashedIdentifier(identifier) => Some(
                identifier
                    .as_css_dashed_identifier()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            _ => None,
        }
    }

    /// Returns `true` when this value is a CSS function whose decoded name is present in the
    /// sorted, lowercase `names` list.
    pub fn matches_function(&self, names: &[&str]) -> bool {
        let Self::AnyCssFunction(AnyCssFunction::CssFunction(function)) = self else {
            return false;
        };
        function
            .name()
            .ok()
            .and_then(|name| name.as_css_identifier().cloned())
            .and_then(|identifier| identifier.value_token().ok())
            .is_some_and(|token| {
                let name = decode_css_identifier(token.text_trimmed());
                names
                    .binary_search(&name.to_ascii_lowercase_cow().as_ref())
                    .is_ok()
            })
    }

    /// Returns `true` for a length dimension or a unitless zero.
    pub fn matches_length(&self) -> bool {
        self.as_css_regular_dimension()
            .is_some_and(|dimension| dimension.matches_unit(LENGTH_UNITS))
            || self.as_css_number().is_some_and(|number| {
                number.value_token().is_ok_and(|token| {
                    token
                        .text_trimmed()
                        .parse::<f64>()
                        .is_ok_and(|value| value == 0.0)
                })
            })
    }

    /// Returns `true` for a percentage dimension.
    pub fn matches_percentage(&self) -> bool {
        self.as_any_css_dimension()
            .is_some_and(|dimension| dimension.as_css_percentage().is_some())
    }

    /// Returns `true` for a positive resolution dimension.
    pub fn matches_resolution(&self) -> bool {
        self.as_css_regular_dimension().is_some_and(|dimension| {
            dimension.matches_unit(RESOLUTION_UNITS)
                && dimension.value_token().is_ok_and(|token| {
                    token
                        .text_trimmed()
                        .parse::<f64>()
                        .is_ok_and(|value| value > 0.0)
                })
        })
    }

    /// Returns `true` for a URL value or a `src()` function.
    pub fn matches_url(&self) -> bool {
        matches!(
            self,
            Self::AnyCssFunction(AnyCssFunction::CssUrlFunction(_))
        ) || self.matches_function(&["src"])
    }
}
