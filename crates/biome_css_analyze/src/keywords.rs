pub const BASIC_KEYWORDS: [&str; 5] = ["initial", "inherit", "revert", "revert-layer", "unset"];

pub const SYSTEM_FONT_KEYWORDS: [&str; 6] = [
    "caption",
    "icon",
    "menu",
    "message-box",
    "small-caption",
    "status-bar",
];

pub const FONT_FAMILY_KEYWORDS: [&str; 10] = [
    "serif",
    "sans-serif",
    "cursive",
    "fantasy",
    "monospace",
    "system-ui",
    "ui-serif",
    "ui-sans-serif",
    "ui-monospace",
    "ui-rounded",
];

pub const FONT_WEIGHT_ABSOLUTE_KEYWORDS: [&str; 2] = ["normal", "bold"];
pub const FONT_WIGHT_NUMERIC_KEYWORDS: [&str; 9] = [
    "100", "200", "300", "400", "500", "600", "700", "800", "900",
];
pub const FONT_STYLE_KEYWORDS: [&str; 3] = ["normal", "italic", "oblique"];
pub const FONT_VARIANTS_KEYWORDS: [&str; 36] = [
    "normal",
    "none",
    "historical-forms",
    "none",
    "common-ligatures",
    "no-common-ligatures",
    "discretionary-ligatures",
    "no-discretionary-ligatures",
    "historical-ligatures",
    "no-historical-ligatures",
    "contextual",
    "no-contextual",
    "small-caps",
    "small-caps",
    "all-small-caps",
    "petite-caps",
    "all-petite-caps",
    "unicase",
    "titling-caps",
    "lining-nums",
    "oldstyle-nums",
    "proportional-nums",
    "tabular-nums",
    "diagonal-fractions",
    "stacked-fractions",
    "ordinal",
    "slashed-zero",
    "jis78",
    "jis83",
    "jis90",
    "jis04",
    "simplified",
    "traditional",
    "full-width",
    "proportional-width",
    "ruby",
];

pub const FONT_STRETCH_KEYWORDS: [&str; 8] = [
    "semi-condensed",
    "condensed",
    "extra-condensed",
    "ultra-condensed",
    "semi-expanded",
    "expanded",
    "extra-expanded",
    "ultra-expanded",
];

pub const FONT_SIZE_KEYWORDS: [&str; 9] = [
    "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large", "larger", "smaller",
];

pub const LINE_HEIGHT_KEYWORDS: [&str; 1] = ["normal"];
