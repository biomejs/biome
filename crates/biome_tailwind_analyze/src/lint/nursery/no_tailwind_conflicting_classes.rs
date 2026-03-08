use std::hash::{Hash, Hasher};

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_tailwind_conflicting_classes::NoTailwindConflictingClassesOptions;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwValue, TwCandidateList, TwFullCandidate, TwVariantList,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::io;

use crate::utils::{hash_node, is_node_equal};

declare_lint_rule! {
    /// Disallow conflicting Tailwind classes in the same variant context.
    ///
    /// Tailwind classes conflict when they generate the same CSS property on the same element.
    /// This makes class lists harder to read and can make it unclear which style is meant to win.
    ///
    /// Biome compares classes within the same variant chain, so `hover:flex hover:block` is
    /// reported but `flex hover:block` is not.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```tailwind,expect_diagnostic
    /// flex block
    /// ```
    ///
    /// ```tailwind,expect_diagnostic
    /// outline outline-1
    /// ```
    ///
    /// ### Valid
    ///
    /// ```tailwind
    /// flex w-full
    /// ```
    ///
    /// ```tailwind
    /// flex hover:block
    /// ```
    ///
    pub NoTailwindConflictingClasses {
        version: "next",
        name: "noTailwindConflictingClasses",
        language: "tailwind",
        recommended: false,
        sources: &[RuleSource::EslintBetterTailwindcss("no-conflicting-classes").inspired()],
        domains: &[RuleDomain::Tailwind],
    }
}

impl Rule for NoTailwindConflictingClasses {
    type Query = Ast<TwCandidateList>;
    type State = ConflictingClassState;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindConflictingClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        collect_conflicts(ctx.query()).into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let css_noun = if state.properties.len() == 1 {
            "property"
        } else {
            "properties"
        };
        let properties = PropertiesDisplay(&state.properties);

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.current.range(),
            markup! {
                "This Tailwind class conflicts with another class because both utilities set the same CSS "
                {css_noun} ": "
                <Emphasis>{properties}</Emphasis>
                "."
            },
        );

        for conflicting in &state.conflicting {
            diagnostic = diagnostic.detail(
                conflicting.range(),
                markup! {
                    "The conflicting class is here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "When two Tailwind classes set the same CSS " {css_noun} " in the same variant context, it can be unclear which style should apply. Remove one of the classes or move one behind a different variant so they no longer apply together."
        }))
    }
}

#[derive(Debug, Clone)]
pub struct ConflictingClassState {
    current: TwFullCandidate,
    conflicting: Vec<TwFullCandidate>,
    properties: Vec<CssProperty>,
}

#[derive(Debug, Clone)]
struct NormalizedCandidate {
    node: TwFullCandidate,
    variants: TwVariantList,
    properties: &'static [CssProperty],
}

#[derive(Debug, Clone)]
struct GroupKey {
    variants: TwVariantList,
    property: CssProperty,
}

impl PartialEq for GroupKey {
    fn eq(&self, other: &Self) -> bool {
        self.property == other.property
            && is_node_equal(self.variants.syntax(), other.variants.syntax())
    }
}

impl Eq for GroupKey {}

impl Hash for GroupKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_node(self.variants.syntax(), state);
        self.property.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CssProperty {
    AlignItems,
    AlignSelf,
    Bottom,
    ColumnGap,
    Display,
    FlexDirection,
    FlexWrap,
    Height,
    InsetInlineEnd,
    InsetInlineStart,
    JustifyContent,
    Left,
    MaxHeight,
    MaxWidth,
    MinHeight,
    MinWidth,
    OutlineStyle,
    OutlineWidth,
    Overflow,
    OverflowX,
    OverflowY,
    Position,
    Right,
    Rotate,
    RotateX,
    RotateY,
    RotateZ,
    RowGap,
    ScaleX,
    ScaleY,
    ScaleZ,
    SkewX,
    SkewY,
    Top,
    TranslateX,
    TranslateY,
    Visibility,
    Width,
}

impl CssProperty {
    const fn css_name(self) -> &'static str {
        match self {
            Self::AlignItems => "align-items",
            Self::AlignSelf => "align-self",
            Self::Bottom => "bottom",
            Self::ColumnGap => "column-gap",
            Self::Display => "display",
            Self::FlexDirection => "flex-direction",
            Self::FlexWrap => "flex-wrap",
            Self::Height => "height",
            Self::InsetInlineEnd => "inset-inline-end",
            Self::InsetInlineStart => "inset-inline-start",
            Self::JustifyContent => "justify-content",
            Self::Left => "left",
            Self::MaxHeight => "max-height",
            Self::MaxWidth => "max-width",
            Self::MinHeight => "min-height",
            Self::MinWidth => "min-width",
            Self::OutlineStyle => "outline-style",
            Self::OutlineWidth => "outline-width",
            Self::Overflow => "overflow",
            Self::OverflowX => "overflow-x",
            Self::OverflowY => "overflow-y",
            Self::Position => "position",
            Self::Right => "right",
            Self::Rotate => "rotate",
            Self::RotateX => "rotate-x",
            Self::RotateY => "rotate-y",
            Self::RotateZ => "rotate-z",
            Self::RowGap => "row-gap",
            Self::ScaleX => "scale-x",
            Self::ScaleY => "scale-y",
            Self::ScaleZ => "scale-z",
            Self::SkewX => "skew-x",
            Self::SkewY => "skew-y",
            Self::Top => "top",
            Self::TranslateX => "translate-x",
            Self::TranslateY => "translate-y",
            Self::Visibility => "visibility",
            Self::Width => "width",
        }
    }
}

fn collect_conflicts(candidates: &TwCandidateList) -> Vec<ConflictingClassState> {
    let normalized: Vec<_> = candidates
        .iter()
        .filter_map(|candidate| candidate.as_tw_full_candidate().cloned())
        .filter_map(normalize_candidate)
        .collect();

    let mut groups: FxHashMap<GroupKey, Vec<usize>> = FxHashMap::default();
    for (index, candidate) in normalized.iter().enumerate() {
        for property in candidate.properties {
            groups
                .entry(GroupKey {
                    variants: candidate.variants.clone(),
                    property: *property,
                })
                .or_default()
                .push(index);
        }
    }

    let mut conflicts_by_candidate: FxHashMap<usize, (FxHashSet<usize>, FxHashSet<CssProperty>)> =
        FxHashMap::default();

    for (group, indexes) in groups {
        if indexes.len() < 2 {
            continue;
        }

        for &index in &indexes {
            let (conflicting_candidates, properties) =
                conflicts_by_candidate.entry(index).or_default();
            properties.insert(group.property);

            for &other_index in &indexes {
                if index != other_index {
                    conflicting_candidates.insert(other_index);
                }
            }
        }
    }

    let mut states = Vec::new();
    for (index, (conflicting_indexes, properties)) in conflicts_by_candidate {
        let mut properties: Vec<_> = properties.into_iter().collect();
        properties.sort_unstable();

        states.push(ConflictingClassState {
            current: normalized[index].node.clone(),
            conflicting: conflicting_indexes
                .into_iter()
                .map(|other_index| normalized[other_index].node.clone())
                .collect(),
            properties,
        });
    }

    states.sort_by_key(|state| state.current.range().start());
    states
}

fn normalize_candidate(candidate: TwFullCandidate) -> Option<NormalizedCandidate> {
    Some(NormalizedCandidate {
        variants: candidate.variants(),
        properties: classify_candidate(&candidate)?,
        node: candidate,
    })
}

fn classify_candidate(candidate: &TwFullCandidate) -> Option<&'static [CssProperty]> {
    match candidate.candidate().ok()? {
        AnyTwCandidate::TwStaticCandidate(static_candidate) => {
            classify_static_candidate(static_candidate.base_token().ok()?.text_trimmed())
        }
        AnyTwCandidate::TwFunctionalCandidate(functional_candidate) => {
            let base = functional_candidate.base_token().ok()?;
            let value = functional_candidate.value().ok()?;
            classify_functional_candidate(base.text_trimmed(), &value)
        }
        _ => None,
    }
}

fn classify_static_candidate(base: &str) -> Option<&'static [CssProperty]> {
    match base {
        "block" | "inline-block" | "inline" | "hidden" | "inline-flex" | "table"
        | "inline-table" | "table-caption" | "table-cell" | "table-column"
        | "table-column-group" | "table-footer-group" | "table-header-group"
        | "table-row-group" | "table-row" | "flow-root" | "flex" | "grid" | "inline-grid"
        | "contents" | "list-item" => Some(PROPERTY_DISPLAY),

        "static" | "fixed" | "absolute" | "relative" | "sticky" => Some(PROPERTY_POSITION),

        "visible" | "invisible" | "collapse" => Some(PROPERTY_VISIBILITY),

        "flex-row" | "flex-row-reverse" | "flex-col" | "flex-col-reverse" => {
            Some(PROPERTY_FLEX_DIRECTION)
        }

        "flex-wrap" | "flex-nowrap" | "flex-wrap-reverse" => Some(PROPERTY_FLEX_WRAP),

        "overflow-auto" | "overflow-hidden" | "overflow-clip" | "overflow-visible"
        | "overflow-scroll" => Some(PROPERTY_OVERFLOW),

        "outline-none" | "outline-hidden" | "outline-solid" | "outline-dashed"
        | "outline-dotted" | "outline-double" => Some(PROPERTY_OUTLINE_STYLE),

        "outline" => Some(PROPERTY_OUTLINE),

        _ => None,
    }
}

fn classify_functional_candidate(base: &str, value: &AnyTwValue) -> Option<&'static [CssProperty]> {
    match base {
        "overflow" => classify_overflow_value(value),
        "overflow-x" => Some(PROPERTY_OVERFLOW_X),
        "overflow-y" => Some(PROPERTY_OVERFLOW_Y),
        "flex" => classify_flex_value(value),
        "justify" => Some(PROPERTY_JUSTIFY_CONTENT),
        "items" => Some(PROPERTY_ALIGN_ITEMS),
        "self" => Some(PROPERTY_ALIGN_SELF),
        "size" => Some(PROPERTY_SIZE),
        "w" => Some(PROPERTY_WIDTH),
        "h" => Some(PROPERTY_HEIGHT),
        "min-w" => Some(PROPERTY_MIN_WIDTH),
        "max-w" => Some(PROPERTY_MAX_WIDTH),
        "min-h" => Some(PROPERTY_MIN_HEIGHT),
        "max-h" => Some(PROPERTY_MAX_HEIGHT),
        "gap" => Some(PROPERTY_GAP),
        "gap-x" => Some(PROPERTY_COLUMN_GAP),
        "gap-y" => Some(PROPERTY_ROW_GAP),
        "inset" => Some(PROPERTY_INSET),
        "inset-x" => Some(PROPERTY_INSET_X),
        "inset-y" => Some(PROPERTY_INSET_Y),
        "top" => Some(PROPERTY_TOP),
        "right" => Some(PROPERTY_RIGHT),
        "bottom" => Some(PROPERTY_BOTTOM),
        "left" => Some(PROPERTY_LEFT),
        "start" => Some(PROPERTY_INSET_INLINE_START),
        "end" => Some(PROPERTY_INSET_INLINE_END),
        "translate" => Some(PROPERTY_TRANSLATE),
        "translate-x" => Some(PROPERTY_TRANSLATE_X),
        "translate-y" => Some(PROPERTY_TRANSLATE_Y),
        "rotate" => Some(PROPERTY_ROTATE),
        "rotate-x" => Some(PROPERTY_ROTATE_X),
        "rotate-y" => Some(PROPERTY_ROTATE_Y),
        "rotate-z" => Some(PROPERTY_ROTATE_Z),
        "scale" => Some(PROPERTY_SCALE),
        "scale-x" => Some(PROPERTY_SCALE_X),
        "scale-y" => Some(PROPERTY_SCALE_Y),
        "scale-z" => Some(PROPERTY_SCALE_Z),
        "skew" => Some(PROPERTY_SKEW),
        "skew-x" => Some(PROPERTY_SKEW_X),
        "skew-y" => Some(PROPERTY_SKEW_Y),
        "outline" => classify_outline_value(value),
        _ => None,
    }
}

fn classify_flex_value(value: &AnyTwValue) -> Option<&'static [CssProperty]> {
    let named = value.as_tw_named_value()?;
    let token = named.value_token().ok()?;

    match token.text_trimmed() {
        "row" | "row-reverse" | "col" | "col-reverse" => Some(PROPERTY_FLEX_DIRECTION),
        "wrap" | "nowrap" | "wrap-reverse" => Some(PROPERTY_FLEX_WRAP),
        _ => None,
    }
}

fn classify_overflow_value(value: &AnyTwValue) -> Option<&'static [CssProperty]> {
    let named = value.as_tw_named_value()?;
    let token = named.value_token().ok()?;
    let value = token.text_trimmed();

    if let Some(axis_value) = value.strip_prefix("x-") {
        return (!axis_value.is_empty()).then_some(PROPERTY_OVERFLOW_X);
    }

    if let Some(axis_value) = value.strip_prefix("y-") {
        return (!axis_value.is_empty()).then_some(PROPERTY_OVERFLOW_Y);
    }

    Some(PROPERTY_OVERFLOW)
}

fn classify_outline_value(value: &AnyTwValue) -> Option<&'static [CssProperty]> {
    match value {
        AnyTwValue::TwArbitraryValue(_) | AnyTwValue::TwCssVariableValue(_) => {
            Some(PROPERTY_OUTLINE_WIDTH)
        }
        AnyTwValue::TwNamedValue(named) => match named.value_token().ok()?.text_trimmed() {
            "none" | "hidden" | "solid" | "dashed" | "dotted" | "double" => {
                Some(PROPERTY_OUTLINE_STYLE)
            }
            value if value.chars().all(|char| char.is_ascii_digit()) => {
                Some(PROPERTY_OUTLINE_WIDTH)
            }
            _ => None,
        },
        _ => None,
    }
}

const PROPERTY_ALIGN_ITEMS: &[CssProperty] = &[CssProperty::AlignItems];
const PROPERTY_ALIGN_SELF: &[CssProperty] = &[CssProperty::AlignSelf];
const PROPERTY_BOTTOM: &[CssProperty] = &[CssProperty::Bottom];
const PROPERTY_COLUMN_GAP: &[CssProperty] = &[CssProperty::ColumnGap];
const PROPERTY_DISPLAY: &[CssProperty] = &[CssProperty::Display];
const PROPERTY_FLEX_DIRECTION: &[CssProperty] = &[CssProperty::FlexDirection];
const PROPERTY_FLEX_WRAP: &[CssProperty] = &[CssProperty::FlexWrap];
const PROPERTY_GAP: &[CssProperty] = &[CssProperty::ColumnGap, CssProperty::RowGap];
const PROPERTY_HEIGHT: &[CssProperty] = &[CssProperty::Height];
const PROPERTY_INSET: &[CssProperty] = &[
    CssProperty::Top,
    CssProperty::Right,
    CssProperty::Bottom,
    CssProperty::Left,
];
const PROPERTY_INSET_INLINE_END: &[CssProperty] = &[CssProperty::InsetInlineEnd];
const PROPERTY_INSET_INLINE_START: &[CssProperty] = &[CssProperty::InsetInlineStart];
const PROPERTY_INSET_X: &[CssProperty] = &[CssProperty::Left, CssProperty::Right];
const PROPERTY_INSET_Y: &[CssProperty] = &[CssProperty::Top, CssProperty::Bottom];
const PROPERTY_JUSTIFY_CONTENT: &[CssProperty] = &[CssProperty::JustifyContent];
const PROPERTY_LEFT: &[CssProperty] = &[CssProperty::Left];
const PROPERTY_MAX_HEIGHT: &[CssProperty] = &[CssProperty::MaxHeight];
const PROPERTY_MAX_WIDTH: &[CssProperty] = &[CssProperty::MaxWidth];
const PROPERTY_MIN_HEIGHT: &[CssProperty] = &[CssProperty::MinHeight];
const PROPERTY_MIN_WIDTH: &[CssProperty] = &[CssProperty::MinWidth];
const PROPERTY_OUTLINE: &[CssProperty] = &[CssProperty::OutlineStyle, CssProperty::OutlineWidth];
const PROPERTY_OUTLINE_STYLE: &[CssProperty] = &[CssProperty::OutlineStyle];
const PROPERTY_OUTLINE_WIDTH: &[CssProperty] = &[CssProperty::OutlineWidth];
const PROPERTY_OVERFLOW: &[CssProperty] = &[CssProperty::Overflow];
const PROPERTY_OVERFLOW_X: &[CssProperty] = &[CssProperty::OverflowX];
const PROPERTY_OVERFLOW_Y: &[CssProperty] = &[CssProperty::OverflowY];
const PROPERTY_POSITION: &[CssProperty] = &[CssProperty::Position];
const PROPERTY_RIGHT: &[CssProperty] = &[CssProperty::Right];
const PROPERTY_ROTATE: &[CssProperty] = &[CssProperty::Rotate];
const PROPERTY_ROTATE_X: &[CssProperty] = &[CssProperty::RotateX];
const PROPERTY_ROTATE_Y: &[CssProperty] = &[CssProperty::RotateY];
const PROPERTY_ROTATE_Z: &[CssProperty] = &[CssProperty::RotateZ];
const PROPERTY_ROW_GAP: &[CssProperty] = &[CssProperty::RowGap];
const PROPERTY_SCALE: &[CssProperty] = &[CssProperty::ScaleX, CssProperty::ScaleY];
const PROPERTY_SCALE_X: &[CssProperty] = &[CssProperty::ScaleX];
const PROPERTY_SCALE_Y: &[CssProperty] = &[CssProperty::ScaleY];
const PROPERTY_SCALE_Z: &[CssProperty] = &[CssProperty::ScaleZ];
const PROPERTY_SIZE: &[CssProperty] = &[CssProperty::Width, CssProperty::Height];
const PROPERTY_SKEW: &[CssProperty] = &[CssProperty::SkewX, CssProperty::SkewY];
const PROPERTY_SKEW_X: &[CssProperty] = &[CssProperty::SkewX];
const PROPERTY_SKEW_Y: &[CssProperty] = &[CssProperty::SkewY];
const PROPERTY_TOP: &[CssProperty] = &[CssProperty::Top];
const PROPERTY_TRANSLATE: &[CssProperty] = &[CssProperty::TranslateX, CssProperty::TranslateY];
const PROPERTY_TRANSLATE_X: &[CssProperty] = &[CssProperty::TranslateX];
const PROPERTY_TRANSLATE_Y: &[CssProperty] = &[CssProperty::TranslateY];
const PROPERTY_VISIBILITY: &[CssProperty] = &[CssProperty::Visibility];
const PROPERTY_WIDTH: &[CssProperty] = &[CssProperty::Width];

struct PropertiesDisplay<'a>(&'a [CssProperty]);

impl Display for PropertiesDisplay<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        for (index, property) in self.0.iter().enumerate() {
            if index > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_str(property.css_name())?;
        }

        Ok(())
    }
}
