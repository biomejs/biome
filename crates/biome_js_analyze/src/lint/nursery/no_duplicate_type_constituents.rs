use crate::JsRuleAction;
use crate::utils::{is_node_equal, node_hash};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyTsType, JsFormalParameter, JsLanguage, TsIntersectionType, TsIntersectionTypeElementList,
    TsParenthesizedType, TsPropertyParameter, TsTypeAnnotation, TsUnionType,
    TsUnionTypeVariantList,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::no_duplicate_type_constituents::NoDuplicateTypeConstituentsOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow duplicate types in union and intersection types.
    ///
    /// A union type (`A | B`) accepts a value of any of the listed types.
    /// An intersection type (`A & B`) requires a value to match all of the listed types.
    /// Listing the same type more than once has no effect in either case.
    /// A repeated type is usually a copy-paste mistake, and it makes the type harder to read.
    ///
    /// Two types are duplicates when they are written the same way.
    /// The rule ignores spaces, comments, quote styles, and parentheses around a listed type, so `"A" | ('A')` is reported.
    /// The rule also looks inside parenthesized groups of the same kind, so `A | (B | A)` is reported.
    /// The rule doesn't look up what a type name refers to, so `A | B` isn't reported even when `B` is another name for `A`.
    ///
    /// The rule also reports `undefined` in the type of an optional parameter (a parameter marked with `?`),
    /// because an optional parameter already accepts `undefined`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type T = string | number | string;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type T = "a" | "b" | "a";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type T = Foo & Bar & Foo;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type T = (A | B) | (A | B);
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a?: string | undefined) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type T1 = string | number;
    /// type T2 = "a" | "b";
    /// type T3 = Foo & Bar;
    /// type T4 = (A | B) | (A & B);
    /// type T5 = Set<string> | Set<number>;
    /// function f(a: string | undefined) {}
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreIntersections`
    ///
    /// Default: `false`
    ///
    /// When set to `true`, the rule doesn't check intersection types.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreIntersections": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```ts,use_options
    /// type T = Foo & Foo;
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,use_options,expect_diagnostic
    /// type T = Foo | Foo;
    /// ```
    ///
    /// ### `ignoreUnions`
    ///
    /// Default: `false`
    ///
    /// When set to `true`, the rule doesn't check union types.
    /// This also turns off the check for `undefined` in the type of an optional parameter.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreUnions": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```ts,use_options
    /// type T = Foo | Foo;
    /// function f(a?: string | undefined) {}
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,use_options,expect_diagnostic
    /// type T = Foo & Foo;
    /// ```
    ///
    pub NoDuplicateTypeConstituents {
        version: "2.6.7",
        name: "noDuplicateTypeConstituents",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-duplicate-type-constituents").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyTsUnionOrIntersectionType = TsUnionType | TsIntersectionType
}

/// A problem found in a union or intersection type.
///
/// `constituent` is always the type as written in its list, including any parentheses around it,
/// so that the fix can remove it from that list.
pub enum NoDuplicateTypeConstituentsState {
    /// `constituent` is written the same way as `previous`, which comes earlier.
    Duplicate {
        constituent: AnyTsType,
        previous: AnyTsType,
    },
    /// `constituent` is `undefined` in the type of an optional parameter.
    UnnecessaryUndefined { constituent: AnyTsType },
}

impl Rule for NoDuplicateTypeConstituents {
    type Query = Ast<AnyTsUnionOrIntersectionType>;
    type State = NoDuplicateTypeConstituentsState;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateTypeConstituentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        if is_nested_in_same_kind(node) {
            return Box::default();
        }
        let mut collector = ConstituentCollector::default();
        match node {
            AnyTsUnionOrIntersectionType::TsUnionType(union) => {
                if options.ignore_unions() {
                    return Box::default();
                }
                collector.check_undefined = is_optional_parameter_type(union);
                collector.collect_union(union);
            }
            AnyTsUnionOrIntersectionType::TsIntersectionType(intersection) => {
                if options.ignore_intersections() {
                    return Box::default();
                }
                collector.collect_intersection(intersection);
            }
        }
        collector.signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = TypeListKind::from(ctx.query());
        Some(match state {
            NoDuplicateTypeConstituentsState::Duplicate {
                constituent,
                previous,
            } => RuleDiagnostic::new(
                rule_category!(),
                constituent.range(),
                markup! {
                    "This type is already listed in the "{kind}" type."
                },
            )
            .detail(previous.range(), markup! { "It is first listed here." })
            .note(markup! {
                "Listing the same type more than once has no effect and makes the type harder to read."
            })
            .note(markup! {
                "Remove the duplicate type, or replace it with the type you meant."
            }),
            NoDuplicateTypeConstituentsState::UnnecessaryUndefined { constituent } => {
                RuleDiagnostic::new(
                    rule_category!(),
                    constituent.range(),
                    markup! {
                        "This "<Emphasis>"undefined"</Emphasis>" is unnecessary because the parameter is optional."
                    },
                )
                .note(markup! {
                    "A parameter marked with "<Emphasis>"?"</Emphasis>" already accepts "<Emphasis>"undefined"</Emphasis>"."
                })
                .note(markup! {
                    "Remove "<Emphasis>"undefined"</Emphasis>" from the parameter type."
                })
            }
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let constituent = match state {
            NoDuplicateTypeConstituentsState::Duplicate { constituent, .. }
            | NoDuplicateTypeConstituentsState::UnnecessaryUndefined { constituent } => constituent,
        };
        let list = constituent.syntax().parent()?;
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            AnyTsUnionOrIntersectionType::TsUnionType(_) => remove_constituent(
                &mut mutation,
                &TsUnionTypeVariantList::cast(list)?,
                constituent,
            )?,
            AnyTsUnionOrIntersectionType::TsIntersectionType(_) => remove_constituent(
                &mut mutation,
                &TsIntersectionTypeElementList::cast(list)?,
                constituent,
            )?,
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                NoDuplicateTypeConstituentsState::Duplicate { .. } => {
                    markup! { "Remove the duplicate type." }
                }
                NoDuplicateTypeConstituentsState::UnnecessaryUndefined { .. } => {
                    markup! { "Remove "<Emphasis>"undefined"</Emphasis>"." }
                }
            },
            mutation,
        ))
    }
}

#[derive(Clone, Copy)]
enum TypeListKind {
    Union,
    Intersection,
}

impl From<&AnyTsUnionOrIntersectionType> for TypeListKind {
    fn from(node: &AnyTsUnionOrIntersectionType) -> Self {
        match node {
            AnyTsUnionOrIntersectionType::TsUnionType(_) => Self::Union,
            AnyTsUnionOrIntersectionType::TsIntersectionType(_) => Self::Intersection,
        }
    }
}

impl biome_console::fmt::Display for TypeListKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Union => "union",
            Self::Intersection => "intersection",
        })
    }
}

/// Walks the types listed in a union or intersection, including the types of nested
/// parenthesized groups of the same kind, and records a signal for every problem found.
#[derive(Default)]
struct ConstituentCollector {
    /// The types seen so far without their parentheses, grouped by [node_hash].
    seen: FxHashMap<u64, Vec<AnyTsType>>,
    /// Whether `undefined` must be reported, because the type is the type of an optional parameter.
    check_undefined: bool,
    signals: Vec<NoDuplicateTypeConstituentsState>,
}

impl ConstituentCollector {
    fn collect_union(&mut self, union: &TsUnionType) {
        for constituent in union.types().iter().flatten() {
            if let Some(AnyTsType::TsUnionType(nested)) = self.visit(constituent) {
                self.collect_union(&nested);
            }
        }
    }

    fn collect_intersection(&mut self, intersection: &TsIntersectionType) {
        for constituent in intersection.types().iter().flatten() {
            if let Some(AnyTsType::TsIntersectionType(nested)) = self.visit(constituent) {
                self.collect_intersection(&nested);
            }
        }
    }

    /// Records a signal if `constituent` is a problem.
    /// Returns `constituent` without its parentheses, or `None` if it is a duplicate, whose
    /// contents were already checked through the earlier copy.
    fn visit(&mut self, constituent: AnyTsType) -> Option<AnyTsType> {
        let unwrapped = constituent.clone().omit_parentheses();
        let same_hash = self.seen.entry(node_hash(unwrapped.syntax())).or_default();
        if let Some(previous) = same_hash
            .iter()
            .find(|previous| is_node_equal(previous.syntax(), unwrapped.syntax()))
        {
            self.signals
                .push(NoDuplicateTypeConstituentsState::Duplicate {
                    previous: previous.clone(),
                    constituent,
                });
            return None;
        }
        same_hash.push(unwrapped.clone());
        if self.check_undefined && matches!(unwrapped, AnyTsType::TsUndefinedType(_)) {
            self.signals
                .push(NoDuplicateTypeConstituentsState::UnnecessaryUndefined { constituent });
        }
        Some(unwrapped)
    }
}

/// Returns `true` if `node` is a parenthesized group inside a list of the same kind, such as
/// `(B | C)` in `A | (B | C)`. The outermost union or intersection checks those types.
fn is_nested_in_same_kind(node: &AnyTsUnionOrIntersectionType) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !TsParenthesizedType::can_cast(ancestor.kind()))
        .is_some_and(|ancestor| match node {
            AnyTsUnionOrIntersectionType::TsUnionType(_) => {
                TsUnionTypeVariantList::can_cast(ancestor.kind())
            }
            AnyTsUnionOrIntersectionType::TsIntersectionType(_) => {
                TsIntersectionTypeElementList::can_cast(ancestor.kind())
            }
        })
}

/// Returns `true` if `union` is the type of an optional function parameter, such as
/// `a?: string | undefined`.
///
/// Parameter properties, such as `constructor(private a?: string | undefined)`, are excluded
/// because they also declare a class property, whose type can need the explicit `undefined`.
fn is_optional_parameter_type(union: &TsUnionType) -> bool {
    let Some(annotation) = union
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !TsParenthesizedType::can_cast(ancestor.kind()))
        .and_then(TsTypeAnnotation::cast)
    else {
        return false;
    };
    annotation
        .parent::<JsFormalParameter>()
        .is_some_and(|parameter| {
            parameter.question_mark_token().is_some()
                && !parameter
                    .syntax()
                    .parent()
                    .is_some_and(|parent| TsPropertyParameter::can_cast(parent.kind()))
        })
}

/// Removes `constituent` from `list` together with one separator next to it: the separator
/// before it, or the separator after it when it comes first.
///
/// Returns `None`, without changing anything, when the list would become empty or when a
/// comment would be removed.
fn remove_constituent(
    mutation: &mut biome_rowan::BatchMutation<JsLanguage>,
    list: &impl AstSeparatedList<Language = JsLanguage, Node = AnyTsType>,
    constituent: &AnyTsType,
) -> Option<()> {
    if list.len() < 2 {
        return None;
    }
    let mut previous_separator = None;
    for element in list.elements() {
        let trailing_separator = element.trailing_separator().ok()?.cloned();
        if element.node().ok()? == constituent {
            let separator = previous_separator.or(trailing_separator)?;
            if constituent.syntax().has_comments_descendants()
                || separator.has_leading_comments()
                || separator.has_trailing_comments()
            {
                return None;
            }
            mutation.remove_token(separator);
            mutation.remove_node(constituent.clone());
            return Some(());
        }
        previous_separator = trailing_separator;
    }
    None
}
