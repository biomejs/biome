//! Additions to the default Tailwind CSS configuration, read from the user's
//! stylesheet (the `tailwind.stylesheet` option).
//!
//! The sorter consults a [TailwindDesignSystem] before its generated defaults:
//! theme values make classes like `bg-brand` known, `@utility` rules add
//! static utilities, `@custom-variant` rules add variants, and breakpoint and
//! container values change how size variants compare.

use std::sync::LazyLock;

use biome_analyze::options::TailwindOptions;
use biome_module_graph::{ModuleDb, TailwindStylesheet, TailwindThemeEntry, tailwind_stylesheet};
use rustc_hash::FxHashMap;

use super::tailwind_preset_v4::PROPERTY_INDEX;
use super::tailwind_preset_v4_types::ThemeNamespace;

/// The parts of a user's Tailwind CSS configuration that change how classes
/// sort. The default value describes the default configuration.
#[derive(Debug, Default)]
pub struct TailwindDesignSystem {
    theme: FxHashMap<ThemeNamespace, ThemeValues>,
    utilities: FxHashMap<Box<str>, CustomUtility>,
    /// Custom variant names, mapped to their declaration index. Tailwind
    /// registers them after its own variants, in declaration order.
    variants: FxHashMap<Box<str>, u16>,
}

#[derive(Debug, Default)]
struct ThemeValues {
    /// Whether `--<namespace>-*: initial` removed the default values.
    reset: bool,
    /// Keys defined by the stylesheet, mapped to their values.
    values: FxHashMap<Box<str>, Box<str>>,
}

/// A static utility declared with `@utility`.
#[derive(Clone, Debug)]
pub(super) struct CustomUtility {
    /// Ascending indices into Tailwind's property order of the properties the
    /// utility sets.
    pub(super) signature: Box<[u16]>,
    /// The number of declarations, Tailwind's tie-break after the signature.
    pub(super) count: u8,
}

/// What the stylesheet says about a theme key.
pub(super) enum ThemeLookup<'a> {
    /// The stylesheet defines the key with this value.
    Defined(&'a str),
    /// The stylesheet removed the namespace's default values and didn't
    /// define the key again.
    Removed,
    /// The default configuration decides.
    Default,
}

static DEFAULT: LazyLock<TailwindDesignSystem> = LazyLock::new(TailwindDesignSystem::default);

impl TailwindDesignSystem {
    /// The default configuration, for analyses without a stylesheet.
    pub fn default_ref() -> &'static Self {
        &DEFAULT
    }

    /// Reads the stylesheet configured in `options` from the module graph. The
    /// result describes the default configuration when no stylesheet is
    /// configured or the module graph doesn't know it.
    pub fn from_module_graph(module_db: &dyn ModuleDb, options: &TailwindOptions) -> Self {
        options
            .stylesheet()
            .and_then(|path| module_db.module_for_path(path))
            .map(|module| Self::from(tailwind_stylesheet(module_db, module)))
            .unwrap_or_default()
    }

    /// Whether the design system has nothing beyond the default configuration.
    pub fn is_default(&self) -> bool {
        self.theme.is_empty() && self.utilities.is_empty() && self.variants.is_empty()
    }

    /// Records a theme variable such as `--color-brand: #00f`.
    pub fn add_theme_variable(&mut self, name: &str, value: &str) {
        let Some((namespace, key)) = theme_key(name) else {
            return;
        };
        self.theme
            .entry(namespace)
            .or_default()
            .values
            .insert(key.into(), value.trim().into());
    }

    /// Records a reset such as `--color-*: initial`, where `reference` is the
    /// part before `-*` (`--color`). `--*: initial` (reference `--`) resets
    /// every namespace.
    pub fn reset_theme(&mut self, reference: &str) {
        let Some(name) = reference.strip_prefix("--") else {
            return;
        };
        if name.is_empty() {
            for namespace in ThemeNamespace::ALL {
                self.reset_namespace(namespace);
            }
        } else if let Some(namespace) = ThemeNamespace::from_css_name(name) {
            self.reset_namespace(namespace);
        }
    }

    fn reset_namespace(&mut self, namespace: ThemeNamespace) {
        let values = self.theme.entry(namespace).or_default();
        values.reset = true;
        values.values.clear();
    }

    /// Records a static utility declared with `@utility`, given the property
    /// names of its declarations and their count.
    pub fn add_utility<'a>(
        &mut self,
        name: &str,
        properties: impl IntoIterator<Item = &'a str>,
        declaration_count: usize,
    ) {
        let mut signature: Vec<u16> = properties
            .into_iter()
            .filter_map(|property| PROPERTY_INDEX.get(property).copied())
            .collect();
        signature.sort_unstable();
        signature.dedup();
        self.utilities.insert(
            name.into(),
            CustomUtility {
                signature: signature.into_boxed_slice(),
                count: u8::try_from(declaration_count).unwrap_or(u8::MAX),
            },
        );
    }

    /// Records a variant declared with `@custom-variant`.
    pub fn add_variant(&mut self, name: &str) {
        let index = u16::try_from(self.variants.len()).unwrap_or(u16::MAX);
        self.variants.entry(name.into()).or_insert(index);
    }

    /// Whether `key` is a value of the theme `namespace`.
    pub(super) fn has_theme_key(&self, namespace: ThemeNamespace, key: &str) -> bool {
        match self.theme_lookup(namespace, key) {
            ThemeLookup::Defined(_) => true,
            ThemeLookup::Removed => false,
            ThemeLookup::Default => namespace.keys().contains(key),
        }
    }

    pub(super) fn theme_lookup(&self, namespace: ThemeNamespace, key: &str) -> ThemeLookup<'_> {
        let Some(values) = self.theme.get(&namespace) else {
            return ThemeLookup::Default;
        };
        match values.values.get(key) {
            Some(value) => ThemeLookup::Defined(value),
            None if values.reset => ThemeLookup::Removed,
            None => ThemeLookup::Default,
        }
    }

    pub(super) fn utility(&self, name: &str) -> Option<&CustomUtility> {
        self.utilities.get(name)
    }

    pub(super) fn has_utilities(&self) -> bool {
        !self.utilities.is_empty()
    }

    /// The declaration index of a custom variant.
    pub(super) fn variant(&self, name: &str) -> Option<u16> {
        self.variants.get(name).copied()
    }
}

impl From<&TailwindStylesheet> for TailwindDesignSystem {
    fn from(stylesheet: &TailwindStylesheet) -> Self {
        let mut design = Self::default();
        for entry in &stylesheet.theme {
            match entry {
                TailwindThemeEntry::Variable { name, value } => {
                    design.add_theme_variable(name.text(), value.text());
                }
                TailwindThemeEntry::Reset { reference } => design.reset_theme(reference.text()),
            }
        }
        for utility in &stylesheet.utilities {
            design.add_utility(
                utility.name.text(),
                utility.properties.iter().map(|property| property.text()),
                utility.declaration_count,
            );
        }
        for variant in &stylesheet.custom_variants {
            design.add_variant(variant.text());
        }
        design
    }
}

/// Splits a theme variable name into its namespace and key:
/// `--color-brand-500` is key `brand-500` of the color namespace. Returns
/// `None` for variables outside the known namespaces and for sub-properties
/// such as `--text-lg--line-height`.
fn theme_key(name: &str) -> Option<(ThemeNamespace, &str)> {
    let name = name.strip_prefix("--")?;
    // Namespaces such as `text` and `text-shadow` share a prefix, so the
    // longest match wins.
    let (namespace, key) = ThemeNamespace::ALL
        .iter()
        .filter_map(|&namespace| {
            let key = name.strip_prefix(namespace.css_name())?.strip_prefix('-')?;
            Some((namespace, key))
        })
        .max_by_key(|(namespace, _)| namespace.css_name().len())?;
    (!key.is_empty() && !key.contains("--")).then_some((namespace, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_keys_use_the_longest_namespace() {
        assert!(matches!(
            theme_key("--text-shadow-glow"),
            Some((ThemeNamespace::TextShadow, "glow"))
        ));
        assert!(matches!(
            theme_key("--text-huge"),
            Some((ThemeNamespace::Text, "huge"))
        ));
        assert!(theme_key("--text-lg--line-height").is_none());
        assert!(theme_key("--my-variable").is_none());
    }

    #[test]
    fn reset_removes_defaults_but_keeps_later_values() {
        let mut design = TailwindDesignSystem::default();
        design.reset_theme("--color");
        design.add_theme_variable("--color-brand", "#00f");
        assert!(design.has_theme_key(ThemeNamespace::Color, "brand"));
        assert!(!design.has_theme_key(ThemeNamespace::Color, "red-500"));
        assert!(design.has_theme_key(ThemeNamespace::Radius, "lg"));
    }

    #[test]
    fn reset_all_namespaces() {
        let mut design = TailwindDesignSystem::default();
        design.add_theme_variable("--color-brand", "#00f");
        design.reset_theme("--");
        assert!(!design.has_theme_key(ThemeNamespace::Color, "brand"));
        assert!(!design.has_theme_key(ThemeNamespace::Radius, "lg"));
    }

    #[test]
    fn variants_keep_declaration_order() {
        let mut design = TailwindDesignSystem::default();
        design.add_variant("theme-midnight");
        design.add_variant("pointer-fine");
        design.add_variant("theme-midnight");
        assert_eq!(design.variant("theme-midnight"), Some(0));
        assert_eq!(design.variant("pointer-fine"), Some(1));
    }
}
