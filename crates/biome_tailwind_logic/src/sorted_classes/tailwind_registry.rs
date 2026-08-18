//! Project overlay on top of the generated Tailwind v4 preset.
//!
//! The preset (`tailwind_preset_v4`) is `&'static` `phf` data. A project's
//! stylesheet adds to it through `@utility`, `@custom-variant`, and
//! `@theme`; those registrations live here as owned entries, and every
//! lookup the sorter makes goes through this type. Tailwind keeps a
//! custom utility *alongside* a builtin of the same name (both compile,
//! the one that sorts first places the candidate), so utilities are
//! exposed as custom-only lookups the sorter merges with the preset;
//! variants and theme keys replace, so those lookups fall through. An
//! empty registry answers every lookup from the preset alone, and
//! sorting without a stylesheet is unchanged.

use std::sync::{Arc, LazyLock};

use rustc_hash::{FxHashMap, FxHashSet};

use super::tailwind_preset_v4::{BREAKPOINT_VALUES, CONTAINER_VALUES, VARIANTS};
use super::tailwind_preset_v4_types::{
    CssDataType, ThemeNamespace, VariantCompare, VariantEntry, VariantKind,
};

/// Number of [ThemeNamespace] variants; the per-namespace overlay sets
/// and the cleared-namespace bitmask are sized by it.
const NUM_NAMESPACES: usize = 20;

/// Order of the first `@custom-variant` that does not override a
/// builtin. Tailwind's `Variants.set` hands out `lastOrder + 1` for a
/// new name, so custom variants sort after every builtin, in
/// registration order.
static FIRST_CUSTOM_VARIANT_ORDER: LazyLock<u16> = LazyLock::new(|| {
    VARIANTS
        .values()
        .map(|entry| entry.order)
        .max()
        .map_or(0, |max| max + 1)
});

/// The `sm`/`md`/… entry every breakpoint variant shares: Tailwind
/// registers breakpoints as one variant group, so they take one `order`
/// and compare by their length values.
static BREAKPOINT_VARIANT: LazyLock<VariantEntry> = LazyLock::new(|| {
    VARIANTS.get("sm").copied().unwrap_or(VariantEntry {
        kind: VariantKind::Static,
        order: 0,
        compare: VariantCompare::BreakpointAsc,
        compounds: 1,
        compounds_with: 0,
    })
});

/// A registry with nothing registered, for sorting without a stylesheet.
pub static EMPTY_REGISTRY: LazyLock<TailwindRegistry> = LazyLock::new(TailwindRegistry::default);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TailwindRegistry {
    static_utilities: FxHashMap<Box<str>, RegisteredUtility>,
    functional_utilities: FxHashMap<Box<str>, RegisteredFunctional>,
    variants: FxHashMap<Box<str>, VariantEntry>,
    /// Custom keys per builtin namespace (`--color-brand` → `Color`,
    /// `brand`), the fast path for the preset's `NamedBranch::Theme`
    /// lookups.
    theme_keys: [FxHashSet<Box<str>>; NUM_NAMESPACES],
    /// Every `@theme` key by its full custom-property name
    /// (`--tab-size-github`), for `--value(--tab-size-*)` probes against
    /// namespaces the preset does not know.
    theme_vars: FxHashSet<Box<str>>,
    /// Bit `n` set: `--<namespace n>-*: initial` (or `--*: initial`)
    /// dropped the preset's keys for that namespace.
    cleared_namespaces: u32,
    breakpoints: FxHashMap<Box<str>, Box<str>>,
    containers: FxHashMap<Box<str>, Box<str>>,
    /// Number of `@custom-variant`s registered under a new name; the
    /// next one gets `FIRST_CUSTOM_VARIANT_ORDER + this`.
    custom_variant_count: u16,
    /// Longest custom static-utility name in bytes; the sorter's joined
    /// `base-value` probe stops at `max(preset cap, this)`.
    pub(super) longest_static_name: usize,
    /// Longest custom functional root in bytes, for the same probe on
    /// dashed roots the Tailwind lexer split (`my-thing-4` → `my` +
    /// `thing-4`).
    pub(super) longest_functional_name: usize,
    /// Longest custom variant name in bytes, capping the variant-root
    /// probe.
    pub(super) longest_variant_name: usize,
}

/// A custom `@utility name { … }`: the ascending property-order indices
/// its declarations set, and the declaration count Tailwind breaks
/// signature ties with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RegisteredUtility {
    pub sig: Arc<[u16]>,
    pub count: u8,
}

/// A custom `@utility name-* { … }`. Which declarations survive depends
/// on the candidate: a declaration whose `--value(…)` / `--modifier(…)`
/// does not resolve is dropped, so the signature is computed per
/// candidate from [FunctionalDecl]s rather than stored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RegisteredFunctional {
    /// Every declaration in the block, nested rules flattened, in
    /// Tailwind's `getPropertySort` (breadth-first) order.
    pub decls: Box<[FunctionalDecl]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionalDecl {
    /// Position of the declared property in Tailwind's property order,
    /// `None` for a property Tailwind does not order (custom properties,
    /// unknown names) — it still counts.
    pub property: Option<u16>,
    /// A `--tw-sort: <property>` hint: the position to sort by instead of
    /// the property. Tailwind stops collecting positions after the hint.
    pub sort_hint: Option<u16>,
    /// The `--value(…)` argument shapes this declaration accepts, if it
    /// uses `--value(…)`.
    pub value: Option<Box<[ValueShape]>>,
    /// The `--modifier(…)` argument shapes, if it uses `--modifier(…)`.
    pub modifier: Option<Box<[ValueShape]>>,
}

/// One argument of `--value(…)` / `--modifier(…)`, matched against a
/// candidate's value the way Tailwind's `resolveValueFunction` does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueShape {
    /// `--color-*`: a key in a theme namespace, kept as the full prefix
    /// (`--color-`).
    Theme(Box<str>),
    /// A bare data type: `integer`, `number`, `percentage`, `ratio`.
    Bare(BareValueType),
    /// `[length]` (a typed arbitrary value) or `[*]` (`None`, any).
    Arbitrary(Option<CssDataType>),
    /// `"inherit"`: the exact literal.
    Literal(Box<str>),
}

/// The bare data types `--value(…)` accepts by name.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BareValueType {
    Integer,
    Number,
    Percentage,
    Ratio,
}

impl BareValueType {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "integer" => Some(Self::Integer),
            "number" => Some(Self::Number),
            "percentage" => Some(Self::Percentage),
            "ratio" => Some(Self::Ratio),
            _ => None,
        }
    }
}

/// `--<name>-`, the custom-property prefix of a builtin theme namespace,
/// longest prefixes first so `--text-shadow-` wins over `--text-`.
pub(super) const NAMESPACE_PREFIXES: &[(&str, ThemeNamespace)] = &[
    ("--background-image-", ThemeNamespace::BackgroundImage),
    ("--inset-shadow-", ThemeNamespace::InsetShadow),
    ("--text-shadow-", ThemeNamespace::TextShadow),
    ("--drop-shadow-", ThemeNamespace::DropShadow),
    ("--font-weight-", ThemeNamespace::FontWeight),
    ("--perspective-", ThemeNamespace::Perspective),
    ("--breakpoint-", ThemeNamespace::Breakpoint),
    ("--container-", ThemeNamespace::Container),
    ("--tracking-", ThemeNamespace::Tracking),
    ("--spacing-", ThemeNamespace::Spacing),
    ("--leading-", ThemeNamespace::Leading),
    ("--animate-", ThemeNamespace::Animate),
    ("--shadow-", ThemeNamespace::Shadow),
    ("--radius-", ThemeNamespace::Radius),
    ("--aspect-", ThemeNamespace::Aspect),
    ("--color-", ThemeNamespace::Color),
    ("--font-", ThemeNamespace::Font),
    ("--text-", ThemeNamespace::Text),
    ("--blur-", ThemeNamespace::Blur),
    ("--ease-", ThemeNamespace::Ease),
];

impl ThemeNamespace {
    /// The builtin namespace a `--<prefix>` names, if any.
    pub(super) fn from_prefix(prefix: &str) -> Option<Self> {
        NAMESPACE_PREFIXES
            .iter()
            .find_map(|&(candidate, namespace)| (candidate == prefix).then_some(namespace))
    }
}

impl TailwindRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether anything is registered; an empty registry sorts exactly
    /// like the preset.
    pub fn is_empty(&self) -> bool {
        self.static_utilities.is_empty()
            && self.functional_utilities.is_empty()
            && self.variants.is_empty()
            && self.theme_vars.is_empty()
            && self.cleared_namespaces == 0
            && self.breakpoints.is_empty()
            && self.containers.is_empty()
    }

    // -- Lookups --

    /// A custom `@utility name { … }`; the sorter merges it with the
    /// preset's entry of the same name.
    #[inline]
    pub(super) fn custom_static(&self, name: &str) -> Option<&RegisteredUtility> {
        if self.static_utilities.is_empty() {
            return None;
        }
        self.static_utilities.get(name)
    }

    /// A custom `@utility name-* { … }`, by root.
    #[inline]
    pub(super) fn custom_functional(&self, name: &str) -> Option<&RegisteredFunctional> {
        if self.functional_utilities.is_empty() {
            return None;
        }
        self.functional_utilities.get(name)
    }

    #[inline]
    pub(super) fn has_custom_functional(&self) -> bool {
        !self.functional_utilities.is_empty()
    }

    /// The variant `name` and its entry, borrowing the name from the
    /// overlay or the preset so keys built from it carry no allocation.
    #[inline]
    pub(super) fn get_variant<'a>(&'a self, name: &str) -> Option<(&'a str, &'a VariantEntry)> {
        // Fast path for the common empty overlay: a bare `phf` lookup,
        // identical to the pre-overlay code, so sorting without a
        // stylesheet pays nothing for the registry indirection.
        if self.variants_untouched() {
            return VARIANTS.get_entry(name).map(|(&key, entry)| (key, entry));
        }
        if !self.variants.is_empty()
            && let Some((key, entry)) = self.variants.get_key_value(name)
        {
            return Some((key, entry));
        }
        if self.is_cleared_breakpoint_variant(name) {
            return None;
        }
        VARIANTS.get_entry(name).map(|(&key, entry)| (key, entry))
    }

    /// The hottest registry lookup — called for both sides of every
    /// variant comparison during the sort. Skip the tuple and the
    /// overlay entirely when nothing is registered.
    #[inline]
    pub(super) fn get_variant_entry(&self, name: &str) -> Option<&VariantEntry> {
        if self.variants_untouched() {
            return VARIANTS.get(name);
        }
        self.get_variant(name).map(|(_, entry)| entry)
    }

    /// No custom variants and no cleared namespace, so variant lookups
    /// resolve exactly against the builtin `phf` tables.
    #[inline]
    fn variants_untouched(&self) -> bool {
        self.variants.is_empty() && self.cleared_namespaces == 0
    }

    /// `--breakpoint-*: initial` removes the preset's breakpoints, and
    /// with them their `sm:`-style variants, unless the stylesheet
    /// registered the name again.
    #[inline]
    fn is_cleared_breakpoint_variant(&self, name: &str) -> bool {
        self.is_cleared(ThemeNamespace::Breakpoint) && BREAKPOINT_VALUES.contains_key(name)
    }

    #[inline]
    fn is_cleared(&self, namespace: ThemeNamespace) -> bool {
        self.cleared_namespaces & (1 << namespace as u32) != 0
    }

    #[inline]
    pub(super) fn theme_contains(&self, namespace: ThemeNamespace, key: &str) -> bool {
        let overlay = &self.theme_keys[namespace as usize];
        if !overlay.is_empty() && overlay.contains(key) {
            return true;
        }
        !self.is_cleared(namespace) && namespace.keys().contains(key)
    }

    /// Whether `<prefix><key>` is a theme key, for `--value(--prefix-*)`
    /// probes: builtin namespaces answer through [Self::theme_contains];
    /// any other prefix is matched against the stylesheet's own keys.
    #[inline]
    pub(super) fn theme_var_contains(&self, prefix: &str, key: &str) -> bool {
        if let Some(namespace) = ThemeNamespace::from_prefix(prefix) {
            return self.theme_contains(namespace, key);
        }
        if self.theme_vars.is_empty() {
            return false;
        }
        let mut buf = [0u8; 64];
        join_into(&mut buf, prefix, key).is_some_and(|name| self.theme_vars.contains(name))
    }

    #[inline]
    pub(super) fn get_breakpoint_value(&self, name: &str) -> Option<&str> {
        if !self.breakpoints.is_empty()
            && let Some(value) = self.breakpoints.get(name)
        {
            return Some(value);
        }
        if self.is_cleared(ThemeNamespace::Breakpoint) {
            return None;
        }
        BREAKPOINT_VALUES.get(name).copied()
    }

    #[inline]
    pub(super) fn breakpoint_contains(&self, name: &str) -> bool {
        self.get_breakpoint_value(name).is_some()
    }

    #[inline]
    pub(super) fn get_container_value(&self, name: &str) -> Option<&str> {
        if !self.containers.is_empty()
            && let Some(value) = self.containers.get(name)
        {
            return Some(value);
        }
        if self.is_cleared(ThemeNamespace::Container) {
            return None;
        }
        CONTAINER_VALUES.get(name).copied()
    }

    #[inline]
    pub(super) fn container_contains(&self, name: &str) -> bool {
        self.get_container_value(name).is_some()
    }

    // -- Registration (from the project's stylesheet) --

    /// `@utility name { … }` with the property positions its declarations
    /// set (`sort_indices`, ascending, deduplicated) and the declaration
    /// count. A name the preset already has is overridden, as Tailwind
    /// does.
    pub fn register_static_utility(&mut self, name: &str, sort_indices: &[u16], count: u8) {
        self.longest_static_name = self.longest_static_name.max(name.len());
        self.static_utilities.insert(
            name.into(),
            RegisteredUtility {
                sig: Arc::from(sort_indices),
                count,
            },
        );
    }

    /// `@utility name-* { … }`; `name` is the root without `-*`.
    pub fn register_functional_utility(&mut self, name: &str, decls: Vec<FunctionalDecl>) {
        self.longest_functional_name = self.longest_functional_name.max(name.len());
        self.functional_utilities.insert(
            name.into(),
            RegisteredFunctional {
                decls: decls.into_boxed_slice(),
            },
        );
    }

    /// `@custom-variant name …`. Tailwind's `Variants.set` keeps the
    /// order of a name it already knows (`@custom-variant dark (…)`
    /// still sorts where the builtin `dark:` does) and appends a new
    /// name after every builtin. `compounds` follows Tailwind's
    /// `Compounds` flags: `1` for at-rule variants, `2` for selector
    /// variants.
    pub fn register_custom_variant(&mut self, name: &str, compounds: u8) {
        let entry = match VARIANTS.get(name) {
            Some(builtin) => VariantEntry {
                kind: VariantKind::Static,
                compounds,
                ..*builtin
            },
            None => {
                let order = FIRST_CUSTOM_VARIANT_ORDER.saturating_add(self.custom_variant_count);
                self.custom_variant_count = self.custom_variant_count.saturating_add(1);
                VariantEntry {
                    kind: VariantKind::Static,
                    order,
                    compare: VariantCompare::Default,
                    compounds,
                    compounds_with: 0,
                }
            }
        };
        self.longest_variant_name = self.longest_variant_name.max(name.len());
        self.variants.insert(name.into(), entry);
    }

    /// A `@theme` key by its full name (`--color-brand`). Keys in a
    /// builtin namespace also join that namespace's overlay; breakpoints
    /// and container sizes additionally keep their value (for length
    /// comparison) and, for breakpoints, register the `name:` variant.
    pub fn register_theme_key(&mut self, name: &str, value: &str) {
        let Some((prefix, namespace)) = NAMESPACE_PREFIXES
            .iter()
            .find(|(prefix, _)| name.starts_with(prefix))
            .copied()
        else {
            self.theme_vars.insert(name.into());
            return;
        };
        let key = &name[prefix.len()..];
        // `--text-xs--line-height` is a sub-key of `xs`, not a key.
        if key.is_empty() || key.contains("--") {
            return;
        }
        self.theme_vars.insert(name.into());
        self.theme_keys[namespace as usize].insert(key.into());
        match namespace {
            ThemeNamespace::Breakpoint => {
                self.breakpoints.insert(key.into(), value.into());
                self.longest_variant_name = self.longest_variant_name.max(key.len());
                self.variants.insert(key.into(), *BREAKPOINT_VARIANT);
            }
            ThemeNamespace::Container => {
                self.containers.insert(key.into(), value.into());
            }
            _ => {}
        }
    }

    /// `--<prefix>*: initial`: forget the preset's keys for the namespace
    /// and any keys the stylesheet registered before it. An unknown
    /// prefix (`--tab-size-*: initial`) forgets only the stylesheet's own
    /// keys under it.
    pub fn clear_theme_namespace(&mut self, prefix: &str) {
        self.theme_vars.retain(|name| !name.starts_with(prefix));
        let Some(namespace) = ThemeNamespace::from_prefix(prefix) else {
            return;
        };
        self.cleared_namespaces |= 1 << namespace as u32;
        self.theme_keys[namespace as usize].clear();
        match namespace {
            ThemeNamespace::Breakpoint => {
                for name in self.breakpoints.keys() {
                    self.variants.remove(name);
                }
                self.breakpoints.clear();
            }
            ThemeNamespace::Container => self.containers.clear(),
            _ => {}
        }
    }

    /// `--*: initial`: forget every theme key.
    pub fn clear_all_theme_namespaces(&mut self) {
        for (prefix, _) in NAMESPACE_PREFIXES {
            self.clear_theme_namespace(prefix);
        }
        self.theme_vars.clear();
    }
}

/// Write `left`, then `right`, into `buf` and view the result as a
/// `str`. `None` when the pair does not fit. Both halves are `str`s and
/// the buffer holds them back to back, so the view is valid UTF-8.
#[inline]
pub(super) fn join_into<'b>(buf: &'b mut [u8], left: &str, right: &str) -> Option<&'b str> {
    let len = left.len() + right.len();
    if len > buf.len() {
        return None;
    }
    buf[..left.len()].copy_from_slice(left.as_bytes());
    buf[left.len()..len].copy_from_slice(right.as_bytes());
    str::from_utf8(&buf[..len]).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_delegates_to_builtin() {
        let reg = TailwindRegistry::new();
        assert!(reg.is_empty());
        assert!(reg.custom_static("flex").is_none());
        assert!(reg.custom_functional("p").is_none());
        assert!(reg.get_variant("hover").is_some());
        assert!(reg.theme_contains(ThemeNamespace::Color, "red-500"));
        assert!(reg.theme_var_contains("--color-", "red-500"));
        assert!(!reg.theme_var_contains("--tab-size-", "github"));
    }

    #[test]
    fn custom_static_utility_is_registered_beside_the_builtin() {
        let mut reg = TailwindRegistry::new();
        reg.register_static_utility("flex", &[0, 1], 2);
        let entry = reg.custom_static("flex").unwrap();
        assert_eq!(entry.sig.as_ref(), &[0, 1]);
        assert_eq!(entry.count, 2);
        assert_eq!(reg.longest_static_name, 4);
    }

    #[test]
    fn custom_variant_under_a_new_name_sorts_after_every_builtin() {
        let mut reg = TailwindRegistry::new();
        reg.register_custom_variant("hocus", 2);
        reg.register_custom_variant("theme-midnight", 1);
        let hocus = reg.get_variant_entry("hocus").unwrap();
        let pointer = reg.get_variant_entry("theme-midnight").unwrap();
        assert_eq!(hocus.order, *FIRST_CUSTOM_VARIANT_ORDER);
        assert_eq!(pointer.order, *FIRST_CUSTOM_VARIANT_ORDER + 1);
        assert!(hocus.order > VARIANTS.get("dark").unwrap().order);
    }

    #[test]
    fn custom_variant_overriding_a_builtin_keeps_its_order() {
        let mut reg = TailwindRegistry::new();
        reg.register_custom_variant("dark", 2);
        let custom = reg.get_variant_entry("dark").unwrap();
        let builtin = VARIANTS.get("dark").unwrap();
        assert_eq!(custom.order, builtin.order);
        assert_eq!(custom.compounds, 2);
        // The next new name still takes the first free order.
        reg.register_custom_variant("hocus", 2);
        assert_eq!(
            reg.get_variant_entry("hocus").unwrap().order,
            *FIRST_CUSTOM_VARIANT_ORDER
        );
    }

    #[test]
    fn theme_key_joins_its_namespace_and_the_full_name_set() {
        let mut reg = TailwindRegistry::new();
        reg.register_theme_key("--color-brand", "#abc");
        reg.register_theme_key("--tab-size-github", "8");
        reg.register_theme_key("--text-xs--line-height", "1rem");
        assert!(reg.theme_contains(ThemeNamespace::Color, "brand"));
        assert!(reg.theme_var_contains("--color-", "brand"));
        assert!(reg.theme_var_contains("--tab-size-", "github"));
        assert!(!reg.theme_contains(ThemeNamespace::Text, "xs--line-height"));
        assert!(reg.theme_contains(ThemeNamespace::Color, "red-500"));
    }

    #[test]
    fn breakpoint_registers_value_and_variant() {
        let mut reg = TailwindRegistry::new();
        reg.register_theme_key("--breakpoint-3xl", "120rem");
        assert!(reg.breakpoint_contains("3xl"));
        assert_eq!(reg.get_breakpoint_value("3xl"), Some("120rem"));
        assert!(reg.theme_contains(ThemeNamespace::Breakpoint, "3xl"));
        let (name, entry) = reg.get_variant("3xl").unwrap();
        assert_eq!(name, "3xl");
        assert_eq!(entry.order, VARIANTS.get("sm").unwrap().order);
        assert_eq!(entry.compare, VariantCompare::BreakpointAsc);
    }

    #[test]
    fn clearing_a_namespace_drops_builtin_and_earlier_custom_keys() {
        let mut reg = TailwindRegistry::new();
        reg.register_theme_key("--color-old", "#000");
        reg.clear_theme_namespace("--color-");
        reg.register_theme_key("--color-new", "#fff");
        assert!(!reg.theme_contains(ThemeNamespace::Color, "red-500"));
        assert!(!reg.theme_contains(ThemeNamespace::Color, "old"));
        assert!(reg.theme_contains(ThemeNamespace::Color, "new"));
        assert!(!reg.theme_var_contains("--color-", "old"));
        // Other namespaces are untouched.
        assert!(reg.theme_contains(ThemeNamespace::Text, "lg"));
    }

    #[test]
    fn clearing_breakpoints_removes_their_variants() {
        let mut reg = TailwindRegistry::new();
        reg.register_theme_key("--breakpoint-3xl", "120rem");
        reg.clear_theme_namespace("--breakpoint-");
        assert!(reg.get_variant("sm").is_none());
        assert!(reg.get_variant("3xl").is_none());
        assert!(!reg.breakpoint_contains("md"));
        reg.register_theme_key("--breakpoint-tablet", "48rem");
        assert!(reg.get_variant("tablet").is_some());
        assert!(reg.get_variant("sm").is_none());
        // Non-breakpoint variants survive.
        assert!(reg.get_variant("hover").is_some());
    }

    #[test]
    fn clearing_everything() {
        let mut reg = TailwindRegistry::new();
        reg.register_theme_key("--tab-size-github", "8");
        reg.clear_all_theme_namespaces();
        assert!(!reg.theme_contains(ThemeNamespace::Color, "red-500"));
        assert!(!reg.container_contains("sm"));
        assert!(!reg.theme_var_contains("--tab-size-", "github"));
        assert!(!reg.is_empty());
    }

    #[test]
    fn join_into_bounds() {
        let mut buf = [0u8; 8];
        assert_eq!(join_into(&mut buf, "abc", "-def"), Some("abc-def"));
        assert_eq!(join_into(&mut buf, "abcdefgh", "i"), None);
    }
}
