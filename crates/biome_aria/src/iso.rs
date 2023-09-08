use biome_aria_metadata::{IsoCountries, IsoLanguages, ISO_COUNTRIES, ISO_LANGUAGES};
use std::str::FromStr;

/// Returns a list of valid ISO countries
pub fn is_valid_country(country: &str) -> bool {
    IsoCountries::from_str(country).is_ok()
}

/// Returns a list of valid ISO languages
pub fn is_valid_language(language: &str) -> bool {
    IsoLanguages::from_str(language).is_ok()
}

/// An array of all available countries
pub fn countries() -> &'static [&'static str] {
    &ISO_COUNTRIES
}

/// An array of all available languages
pub fn languages() -> &'static [&'static str] {
    &ISO_LANGUAGES
}
