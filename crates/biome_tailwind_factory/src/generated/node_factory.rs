//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use biome_tailwind_syntax::{
    TailwindSyntaxElement as SyntaxElement, TailwindSyntaxNode as SyntaxNode,
    TailwindSyntaxToken as SyntaxToken, *,
};
pub fn tw_arbitrary_candidate(
    l_brack_token: SyntaxToken,
    property_token: SyntaxToken,
    colon_token: SyntaxToken,
    value_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> TwArbitraryCandidateBuilder {
    TwArbitraryCandidateBuilder {
        l_brack_token,
        property_token,
        colon_token,
        value_token,
        r_brack_token,
        modifier: None,
    }
}
pub struct TwArbitraryCandidateBuilder {
    l_brack_token: SyntaxToken,
    property_token: SyntaxToken,
    colon_token: SyntaxToken,
    value_token: SyntaxToken,
    r_brack_token: SyntaxToken,
    modifier: Option<AnyTwModifier>,
}
impl TwArbitraryCandidateBuilder {
    pub fn with_modifier(mut self, modifier: AnyTwModifier) -> Self {
        self.modifier = Some(modifier);
        self
    }
    pub fn build(self) -> TwArbitraryCandidate {
        TwArbitraryCandidate::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_ARBITRARY_CANDIDATE,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Token(self.property_token)),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Token(self.value_token)),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn tw_arbitrary_modifier(
    slash_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    value_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> TwArbitraryModifier {
    TwArbitraryModifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_ARBITRARY_MODIFIER,
        [
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn tw_arbitrary_value(
    l_brack_token: SyntaxToken,
    value_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> TwArbitraryValue {
    TwArbitraryValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_ARBITRARY_VALUE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn tw_arbitrary_variant(
    l_brack_token: SyntaxToken,
    selector_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> TwArbitraryVariant {
    TwArbitraryVariant::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_ARBITRARY_VARIANT,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(selector_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn tw_candidate(variants: TwVariantList, candidate: AnyTwCandidate) -> TwCandidateBuilder {
    TwCandidateBuilder {
        variants,
        candidate,
        excl_token: None,
    }
}
pub struct TwCandidateBuilder {
    variants: TwVariantList,
    candidate: AnyTwCandidate,
    excl_token: Option<SyntaxToken>,
}
impl TwCandidateBuilder {
    pub fn with_excl_token(mut self, excl_token: SyntaxToken) -> Self {
        self.excl_token = Some(excl_token);
        self
    }
    pub fn build(self) -> TwCandidate {
        TwCandidate::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_CANDIDATE,
            [
                Some(SyntaxElement::Node(self.variants.into_syntax())),
                Some(SyntaxElement::Node(self.candidate.into_syntax())),
                self.excl_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn tw_css_variable_value(
    l_paren_token: SyntaxToken,
    value_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> TwCssVariableValue {
    TwCssVariableValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_CSS_VARIABLE_VALUE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn tw_functional_candidate(
    base_token: SyntaxToken,
    minus_token: SyntaxToken,
    value: AnyTwValue,
) -> TwFunctionalCandidateBuilder {
    TwFunctionalCandidateBuilder {
        base_token,
        minus_token,
        value,
        modifier: None,
    }
}
pub struct TwFunctionalCandidateBuilder {
    base_token: SyntaxToken,
    minus_token: SyntaxToken,
    value: AnyTwValue,
    modifier: Option<AnyTwModifier>,
}
impl TwFunctionalCandidateBuilder {
    pub fn with_modifier(mut self, modifier: AnyTwModifier) -> Self {
        self.modifier = Some(modifier);
        self
    }
    pub fn build(self) -> TwFunctionalCandidate {
        TwFunctionalCandidate::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_FUNCTIONAL_CANDIDATE,
            [
                Some(SyntaxElement::Token(self.base_token)),
                Some(SyntaxElement::Token(self.minus_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                self.modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn tw_functional_variant(selector_token: SyntaxToken) -> TwFunctionalVariantBuilder {
    TwFunctionalVariantBuilder {
        selector_token,
        value: None,
    }
}
pub struct TwFunctionalVariantBuilder {
    selector_token: SyntaxToken,
    value: Option<TwFunctionalVariantValue>,
}
impl TwFunctionalVariantBuilder {
    pub fn with_value(mut self, value: TwFunctionalVariantValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> TwFunctionalVariant {
        TwFunctionalVariant::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_FUNCTIONAL_VARIANT,
            [
                Some(SyntaxElement::Token(self.selector_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn tw_functional_variant_value(
    minus_token: SyntaxToken,
    value: AnyTwValue,
) -> TwFunctionalVariantValue {
    TwFunctionalVariantValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_FUNCTIONAL_VARIANT_VALUE,
        [
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn tw_named_modifier(slash_token: SyntaxToken, value_token: SyntaxToken) -> TwNamedModifier {
    TwNamedModifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_NAMED_MODIFIER,
        [
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn tw_named_value(value_token: SyntaxToken) -> TwNamedValue {
    TwNamedValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_NAMED_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn tw_root(tw_candidate_list: TwCandidateList) -> TwRoot {
    TwRoot::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_ROOT,
        [Some(SyntaxElement::Node(tw_candidate_list.into_syntax()))],
    ))
}
pub fn tw_static_candidate(base_token: SyntaxToken) -> TwStaticCandidate {
    TwStaticCandidate::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_STATIC_CANDIDATE,
        [Some(SyntaxElement::Token(base_token))],
    ))
}
pub fn tw_static_variant(selector_token: SyntaxToken) -> TwStaticVariant {
    TwStaticVariant::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_STATIC_VARIANT,
        [Some(SyntaxElement::Token(selector_token))],
    ))
}
pub fn tw_candidate_list<I>(items: I) -> TwCandidateList
where
    I: IntoIterator<Item = AnyTwCandidate>,
    I::IntoIter: ExactSizeIterator,
{
    TwCandidateList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_CANDIDATE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn tw_variant_list<I>(items: I) -> TwVariantList
where
    I: IntoIterator<Item = AnyTwVariant>,
    I::IntoIter: ExactSizeIterator,
{
    TwVariantList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_VARIANT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn tw_bogus<I>(slots: I) -> TwBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TwBogus::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_BOGUS,
        slots,
    ))
}
pub fn tw_bogus_candidate<I>(slots: I) -> TwBogusCandidate
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TwBogusCandidate::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_BOGUS_CANDIDATE,
        slots,
    ))
}
pub fn tw_bogus_modifier<I>(slots: I) -> TwBogusModifier
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TwBogusModifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_BOGUS_MODIFIER,
        slots,
    ))
}
pub fn tw_bogus_variant<I>(slots: I) -> TwBogusVariant
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TwBogusVariant::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_BOGUS_VARIANT,
        slots,
    ))
}
