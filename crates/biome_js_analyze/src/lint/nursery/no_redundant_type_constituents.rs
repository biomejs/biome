use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyTsType, JsGetterClassMember, JsGetterObjectMember, JsLanguage, JsSyntaxKind, JsSyntaxNode,
    TsConstructSignatureTypeMember, TsConstructorType, TsFunctionType,
    TsGetterSignatureClassMember, TsGetterSignatureTypeMember, TsIntersectionType,
    TsParenthesizedType, TsReturnTypeAnnotation, TsTypeAnnotation, TsUnionType,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, declare_node_union};
use biome_rule_options::no_redundant_type_constituents::NoRedundantTypeConstituentsOptions;
use enumflags2::{BitFlags, bitflags};

declare_lint_rule! {
    /// Disallow members of union and intersection types that have no effect.
    ///
    /// A union type (`A | B`) accepts a value if it matches any of its members.
    /// An intersection type (`A & B`) accepts a value only if it matches all of its members.
    /// Some members make the others pointless, or are pointless themselves:
    ///
    /// - In a union, `any` and `unknown` accept every value, so the other members have no effect.
    /// - In a union, `never` accepts no value, so it has no effect.
    ///   It's allowed when the union is the return type of a function.
    /// - In a union, a literal type such as `"a"` or `1` has no effect when the union also includes
    ///   its primitive type, such as `string` or `number`.
    /// - In an intersection, `any` and `never` replace the whole type, so the other members have no effect.
    /// - In an intersection, `unknown` has no effect.
    /// - In an intersection, a primitive type such as `string` has no effect when the intersection
    ///   also includes one of its literal types, such as `"a"`.
    ///
    /// These members usually point to a mistake, such as a type that is wider than intended.
    ///
    /// This rule only looks at the types written directly in the union or intersection.
    /// It doesn't follow type aliases or other named types.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type UnionAny = any | "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type UnionUnknown = unknown | "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type UnionNever = never | "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type UnionBooleanLiteral = boolean | false;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type UnionNumberLiteral = number | 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type UnionStringLiteral = string | "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionAny = any & "foo";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionUnknown = string & unknown;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionNever = string & never;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionBooleanLiteral = boolean & false;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionNumberLiteral = number & 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type IntersectionStringLiteral = string & "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type UnionAny = any;
    /// type UnionUnknown = unknown;
    /// type UnionNever = never;
    /// type UnionBooleanLiteral = boolean;
    /// type UnionNumberLiteral = number;
    /// type UnionStringLiteral = string;
    /// type IntersectionAny = any;
    /// type IntersectionUnknown = string;
    /// type IntersectionNever = never;
    /// type IntersectionBooleanLiteral = false;
    /// type IntersectionNumberLiteral = 1;
    /// type IntersectionStringLiteral = "foo";
    /// declare function fail(): string | never;
    /// ```
    ///
    pub NoRedundantTypeConstituents {
        version: "2.6.7",
        name: "noRedundantTypeConstituents",
        language: "ts",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintTypeScript("no-redundant-type-constituents").inspired()],
    }
}

declare_node_union! {
    pub AnyTsUnionOrIntersectionType = TsUnionType | TsIntersectionType
}

/// A member of the queried union or intersection that has no effect on the
/// resulting type.
pub struct RedundantConstituent {
    member: AnyTsType,
    reason: RedundancyReason,
}

impl Rule for NoRedundantTypeConstituents {
    type Query = Ast<AnyTsUnionOrIntersectionType>;
    type State = RedundantConstituent;
    type Signals = Box<[Self::State]>;
    type Options = NoRedundantTypeConstituentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // A union nested in another union is checked as part of the outermost
        // one, so that each member is reported only once. The same goes for
        // intersections.
        if is_nested(node) {
            return Box::default();
        }
        match node {
            AnyTsUnionOrIntersectionType::TsUnionType(union) => check_union(union),
            AnyTsUnionOrIntersectionType::TsIntersectionType(intersection) => {
                check_intersection(intersection)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let container = match ctx.query() {
            AnyTsUnionOrIntersectionType::TsUnionType(_) => "union",
            AnyTsUnionOrIntersectionType::TsIntersectionType(_) => "intersection",
        };
        let range = state.member.range();
        Some(match state.reason {
            RedundancyReason::Overrides(keyword) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>{keyword}</Emphasis>" type makes every other member of this "{container}" redundant."
                },
            )
            .note(markup! {
                "This "{container}" is always "<Emphasis>{keyword}</Emphasis>", whatever its other members are."
            })
            .note(markup! {
                "If the type should be "<Emphasis>{keyword}</Emphasis>", replace the whole "{container}" with "<Emphasis>{keyword}</Emphasis>". Otherwise, remove "<Emphasis>{keyword}</Emphasis>"."
            }),
            RedundancyReason::Ignored(keyword) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>{keyword}</Emphasis>" type has no effect in this "{container}"."
                },
            )
            .note(markup! {
                "This "{container}" accepts the same values with or without "<Emphasis>{keyword}</Emphasis>"."
            })
            .note(markup! {
                "Remove "<Emphasis>{keyword}</Emphasis>"."
            }),
            RedundancyReason::LiteralInUnion(primitive) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This literal type is redundant because the union also includes "<Emphasis>{primitive}</Emphasis>"."
                },
            )
            .note(markup! {
                "Every value of this literal type is already a "<Emphasis>{primitive}</Emphasis>", so the union accepts the same values without it."
            })
            .note(markup! {
                "Remove the literal type, or remove "<Emphasis>{primitive}</Emphasis>" if only specific values should be accepted."
            }),
            RedundancyReason::PrimitiveInIntersection(primitive) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>{primitive}</Emphasis>" type is redundant because the intersection also includes a "<Emphasis>{primitive}</Emphasis>" literal type."
                },
            )
            .note(markup! {
                "The literal type already limits the values to specific "<Emphasis>{primitive}</Emphasis>" values, so "<Emphasis>{primitive}</Emphasis>" doesn't change the result."
            })
            .note(markup! {
                "Remove "<Emphasis>{primitive}</Emphasis>"."
            }),
        })
    }
}

fn check_union(union: &TsUnionType) -> Box<[RedundantConstituent]> {
    let mut member_count = 0;
    let mut has_any = false;
    let mut primitives = BitFlags::empty();
    for_each_union_member(union, &mut |member| {
        member_count += 1;
        if let AnyTsType::TsAnyType(_) = member {
            has_any = true;
        } else if let Some(primitive) = Primitive::from_keyword_type(&member) {
            primitives |= primitive;
        }
    });
    // A leading `|` alone, as in `type A = | any`, creates a union with a single member.
    if member_count < 2 {
        return Box::default();
    }

    let in_return_type = is_return_type(union);
    let mut signals = Vec::new();
    for_each_union_member(union, &mut |member| {
        let reason = match &member {
            AnyTsType::TsAnyType(_) => RedundancyReason::Overrides(Keyword::Any),
            // `unknown | any` is `any`.
            AnyTsType::TsUnknownType(_) if has_any => RedundancyReason::Ignored(Keyword::Unknown),
            AnyTsType::TsUnknownType(_) => RedundancyReason::Overrides(Keyword::Unknown),
            // `never` is a common way to document that a function can throw,
            // as in `(): string | never`.
            AnyTsType::TsNeverType(_) if !in_return_type => {
                RedundancyReason::Ignored(Keyword::Never)
            }
            _ => match Primitive::from_literal_type(&member) {
                Some(primitive) if primitives.contains(primitive) => {
                    RedundancyReason::LiteralInUnion(primitive)
                }
                _ => return,
            },
        };
        signals.push(RedundantConstituent { member, reason });
    });
    signals.into_boxed_slice()
}

fn check_intersection(intersection: &TsIntersectionType) -> Box<[RedundantConstituent]> {
    let mut member_count = 0;
    let mut has_never = false;
    let mut literal_primitives = BitFlags::empty();
    for_each_intersection_member(intersection, &mut |member| {
        member_count += 1;
        let primitive = match &member {
            AnyTsType::TsNeverType(_) => {
                has_never = true;
                None
            }
            AnyTsType::TsUnionType(union) => union_literal_primitive(union),
            _ => Primitive::from_literal_type(&member),
        };
        if let Some(primitive) = primitive {
            literal_primitives |= primitive;
        }
    });
    // A leading `&` alone, as in `type A = & any`, creates an intersection with a single member.
    if member_count < 2 {
        return Box::default();
    }

    let mut signals = Vec::new();
    for_each_intersection_member(intersection, &mut |member| {
        let reason = match &member {
            // `any & never` is `never`.
            AnyTsType::TsAnyType(_) if has_never => RedundancyReason::Ignored(Keyword::Any),
            AnyTsType::TsAnyType(_) => RedundancyReason::Overrides(Keyword::Any),
            AnyTsType::TsNeverType(_) => RedundancyReason::Overrides(Keyword::Never),
            AnyTsType::TsUnknownType(_) => RedundancyReason::Ignored(Keyword::Unknown),
            _ => match Primitive::from_keyword_type(&member) {
                Some(primitive) if literal_primitives.contains(primitive) => {
                    RedundancyReason::PrimitiveInIntersection(primitive)
                }
                _ => return,
            },
        };
        signals.push(RedundantConstituent { member, reason });
    });
    signals.into_boxed_slice()
}

/// Calls `f` with every member of `union`, without parentheses, including the
/// members of nested unions such as `(A | B)` in `(A | B) | C`.
fn for_each_union_member(union: &TsUnionType, f: &mut impl FnMut(AnyTsType)) {
    for member in union.types().iter().flatten() {
        match member.omit_parentheses() {
            AnyTsType::TsUnionType(nested) => for_each_union_member(&nested, f),
            member => f(member),
        }
    }
}

/// Calls `f` with every member of `intersection`, without parentheses,
/// including the members of nested intersections such as `(A & B)` in
/// `(A & B) & C`.
fn for_each_intersection_member(intersection: &TsIntersectionType, f: &mut impl FnMut(AnyTsType)) {
    for member in intersection.types().iter().flatten() {
        match member.omit_parentheses() {
            AnyTsType::TsIntersectionType(nested) => for_each_intersection_member(&nested, f),
            member => f(member),
        }
    }
}

/// Returns the primitive type of `union` if all of its members are literal
/// types of that primitive, as in `"a" | "b"`.
fn union_literal_primitive(union: &TsUnionType) -> Option<Primitive> {
    let mut result = Some(BitFlags::<Primitive>::empty());
    for_each_union_member(union, &mut |member| {
        result = result
            .zip(Primitive::from_literal_type(&member))
            .map(|(primitives, primitive)| primitives | primitive);
    });
    result?.exactly_one()
}

/// Returns `true` if `node` is a direct member of another union or
/// intersection of the same kind, ignoring parentheses around `node`.
fn is_nested(node: &AnyTsUnionOrIntersectionType) -> bool {
    let list_kind = match node {
        AnyTsUnionOrIntersectionType::TsUnionType(_) => JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST,
        AnyTsUnionOrIntersectionType::TsIntersectionType(_) => {
            JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST
        }
    };
    parent_skipping_parentheses(node).is_some_and(|parent| parent.kind() == list_kind)
}

/// Returns `true` if `union` is the return type of a function, method, getter,
/// construct signature, or function type, ignoring parentheses around `union`.
fn is_return_type(union: &TsUnionType) -> bool {
    let Some(parent) = parent_skipping_parentheses(union) else {
        return false;
    };
    RETURN_TYPE_PARENT_KINDS.matches(parent.kind())
        || (TsTypeAnnotation::can_cast(parent.kind())
            && parent.parent().is_some_and(|grand_parent| {
                ANNOTATED_RETURN_TYPE_KINDS.matches(grand_parent.kind())
            }))
}

fn parent_skipping_parentheses(node: &impl AstNode<Language = JsLanguage>) -> Option<JsSyntaxNode> {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !TsParenthesizedType::can_cast(ancestor.kind()))
}

/// The nodes whose direct child type is a return type.
const RETURN_TYPE_PARENT_KINDS: SyntaxKindSet<JsLanguage> = TsReturnTypeAnnotation::KIND_SET
    .union(TsFunctionType::KIND_SET)
    .union(TsConstructorType::KIND_SET);

/// The getters and construct signatures, whose type annotation is a return type.
const ANNOTATED_RETURN_TYPE_KINDS: SyntaxKindSet<JsLanguage> = JsGetterClassMember::KIND_SET
    .union(JsGetterObjectMember::KIND_SET)
    .union(TsGetterSignatureClassMember::KIND_SET)
    .union(TsGetterSignatureTypeMember::KIND_SET)
    .union(TsConstructSignatureTypeMember::KIND_SET);

pub enum RedundancyReason {
    /// The member replaces the whole type: `any` or `unknown` in a union, or
    /// `any` or `never` in an intersection.
    Overrides(Keyword),
    /// The member is ignored: `never` in a union, `unknown` in an intersection,
    /// or a keyword that another one replaces, such as `unknown` in `unknown | any`.
    Ignored(Keyword),
    /// A literal type in a union that also includes its primitive type.
    LiteralInUnion(Primitive),
    /// A primitive type in an intersection that also includes one of its literal types.
    PrimitiveInIntersection(Primitive),
}

#[derive(Clone, Copy)]
pub enum Keyword {
    Any,
    Never,
    Unknown,
}

impl biome_console::fmt::Display for Keyword {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Any => "any",
            Self::Never => "never",
            Self::Unknown => "unknown",
        })
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
    Bigint = 1 << 0,
    Boolean = 1 << 1,
    Number = 1 << 2,
    String = 1 << 3,
}

impl Primitive {
    /// Returns the primitive of a primitive type such as `string`.
    fn from_keyword_type(ty: &AnyTsType) -> Option<Self> {
        match ty {
            AnyTsType::TsBigintType(_) => Some(Self::Bigint),
            AnyTsType::TsBooleanType(_) => Some(Self::Boolean),
            AnyTsType::TsNumberType(_) => Some(Self::Number),
            AnyTsType::TsStringType(_) => Some(Self::String),
            _ => None,
        }
    }

    /// Returns the primitive of a literal type such as `"a"` or `` `a${string}` ``.
    fn from_literal_type(ty: &AnyTsType) -> Option<Self> {
        match ty {
            AnyTsType::TsBigintLiteralType(_) => Some(Self::Bigint),
            AnyTsType::TsBooleanLiteralType(_) => Some(Self::Boolean),
            AnyTsType::TsNumberLiteralType(_) => Some(Self::Number),
            AnyTsType::TsStringLiteralType(_) | AnyTsType::TsTemplateLiteralType(_) => {
                Some(Self::String)
            }
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for Primitive {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Bigint => "bigint",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
        })
    }
}
