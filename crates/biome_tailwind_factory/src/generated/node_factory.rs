//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use biome_tailwind_syntax::{
    TailwindSyntaxElement as SyntaxElement, TailwindSyntaxNode as SyntaxNode,
    TailwindSyntaxToken as SyntaxToken, *,
};
pub fn css_binary_expression(
    left: AnyCssExpression,
    operator_token: SyntaxToken,
    right: AnyCssExpression,
) -> CssBinaryExpression {
    CssBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_color(value_token: SyntaxToken) -> CssColor {
    CssColor::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_COLOR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_dashed_identifier(ident_token: SyntaxToken) -> CssDashedIdentifier {
    CssDashedIdentifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_DASHED_IDENTIFIER,
        [Some(SyntaxElement::Token(ident_token))],
    ))
}
pub fn css_function(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    parameters: CssParameterList,
    r_paren_token: SyntaxToken,
) -> CssFunction {
    CssFunction::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_FUNCTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_generic_delimiter(value_token: SyntaxToken) -> CssGenericDelimiter {
    CssGenericDelimiter::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_GENERIC_DELIMITER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_identifier(ident_token: SyntaxToken) -> CssIdentifier {
    CssIdentifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_IDENTIFIER,
        [Some(SyntaxElement::Token(ident_token))],
    ))
}
pub fn css_list_of_component_values_expression(
    css_component_value_list: CssComponentValueList,
) -> CssListOfComponentValuesExpression {
    CssListOfComponentValuesExpression::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION,
        [Some(SyntaxElement::Node(
            css_component_value_list.into_syntax(),
        ))],
    ))
}
pub fn css_number(value_token: SyntaxToken) -> CssNumber {
    CssNumber::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: CssComponentValueList,
    r_paren_token: SyntaxToken,
) -> CssParenthesizedExpression {
    CssParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_percentage(value_token: SyntaxToken, remainder_token: SyntaxToken) -> CssPercentage {
    CssPercentage::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_PERCENTAGE,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(remainder_token)),
        ],
    ))
}
pub fn css_ratio(left: CssNumber, slash_token: SyntaxToken, right: CssNumber) -> CssRatio {
    CssRatio::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_RATIO,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_regular_dimension(
    value_token: SyntaxToken,
    unit_token: SyntaxToken,
) -> CssRegularDimension {
    CssRegularDimension::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_REGULAR_DIMENSION,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(unit_token)),
        ],
    ))
}
pub fn css_string(value_token: SyntaxToken) -> CssString {
    CssString::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_unary_expression(
    operator_token: SyntaxToken,
    argument: AnyCssValue,
) -> CssUnaryExpression {
    CssUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn css_unknown_dimension(
    value_token: SyntaxToken,
    unit_token: SyntaxToken,
) -> CssUnknownDimension {
    CssUnknownDimension::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_UNKNOWN_DIMENSION,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(unit_token)),
        ],
    ))
}
pub fn css_url_function(
    url_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> CssUrlFunctionBuilder {
    CssUrlFunctionBuilder {
        url_token,
        l_paren_token,
        r_paren_token,
        value: None,
    }
}
pub struct CssUrlFunctionBuilder {
    url_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    value: Option<AnyCssUrlValue>,
}
impl CssUrlFunctionBuilder {
    pub fn with_value(mut self, value: AnyCssUrlValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> CssUrlFunction {
        CssUrlFunction::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::CSS_URL_FUNCTION,
            [
                Some(SyntaxElement::Token(self.url_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_url_value_raw(value_token: SyntaxToken) -> CssUrlValueRaw {
    CssUrlValueRaw::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_URL_VALUE_RAW,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn tw_arbitrary_candidate(
    l_brack_token: SyntaxToken,
    property_token: SyntaxToken,
    colon_token: SyntaxToken,
    value: CssGenericComponentValueList,
    r_brack_token: SyntaxToken,
) -> TwArbitraryCandidateBuilder {
    TwArbitraryCandidateBuilder {
        l_brack_token,
        property_token,
        colon_token,
        value,
        r_brack_token,
        modifier: None,
    }
}
pub struct TwArbitraryCandidateBuilder {
    l_brack_token: SyntaxToken,
    property_token: SyntaxToken,
    colon_token: SyntaxToken,
    value: CssGenericComponentValueList,
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
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn tw_arbitrary_value(
    l_brack_token: SyntaxToken,
    value: CssGenericComponentValueList,
    r_brack_token: SyntaxToken,
) -> TwArbitraryValue {
    TwArbitraryValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_ARBITRARY_VALUE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
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
pub fn tw_data_attribute(
    data_token: SyntaxToken,
    minus_token: SyntaxToken,
    value: AnyTwDataAttributeValue,
) -> TwDataAttribute {
    TwDataAttribute::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_DATA_ATTRIBUTE,
        [
            Some(SyntaxElement::Token(data_token)),
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn tw_full_candidate(
    variants: TwVariantList,
    candidate: AnyTwCandidate,
) -> TwFullCandidateBuilder {
    TwFullCandidateBuilder {
        variants,
        candidate,
        negative_token: None,
        excl_token: None,
    }
}
pub struct TwFullCandidateBuilder {
    variants: TwVariantList,
    candidate: AnyTwCandidate,
    negative_token: Option<SyntaxToken>,
    excl_token: Option<SyntaxToken>,
}
impl TwFullCandidateBuilder {
    pub fn with_negative_token(mut self, negative_token: SyntaxToken) -> Self {
        self.negative_token = Some(negative_token);
        self
    }
    pub fn with_excl_token(mut self, excl_token: SyntaxToken) -> Self {
        self.excl_token = Some(excl_token);
        self
    }
    pub fn build(self) -> TwFullCandidate {
        TwFullCandidate::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_FULL_CANDIDATE,
            [
                Some(SyntaxElement::Node(self.variants.into_syntax())),
                self.negative_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.candidate.into_syntax())),
                self.excl_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn tw_functional_variant(
    base_token: SyntaxToken,
    minus_token: SyntaxToken,
    value: AnyTwValue,
) -> TwFunctionalVariant {
    TwFunctionalVariant::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_FUNCTIONAL_VARIANT,
        [
            Some(SyntaxElement::Token(base_token)),
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn tw_modifier(slash_token: SyntaxToken, value: AnyTwValue) -> TwModifier {
    TwModifier::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_MODIFIER,
        [
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn tw_named_value(value_token: SyntaxToken) -> TwNamedValue {
    TwNamedValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_NAMED_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn tw_root(candidates: TwCandidateList, eof_token: SyntaxToken) -> TwRootBuilder {
    TwRootBuilder {
        candidates,
        eof_token,
        bom_token: None,
    }
}
pub struct TwRootBuilder {
    candidates: TwCandidateList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl TwRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> TwRoot {
        TwRoot::unwrap_cast(SyntaxNode::new_detached(
            TailwindSyntaxKind::TW_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.candidates.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn tw_static_candidate(base_token: SyntaxToken) -> TwStaticCandidate {
    TwStaticCandidate::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_STATIC_CANDIDATE,
        [Some(SyntaxElement::Token(base_token))],
    ))
}
pub fn tw_static_variant(base_token: SyntaxToken) -> TwStaticVariant {
    TwStaticVariant::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_STATIC_VARIANT,
        [Some(SyntaxElement::Token(base_token))],
    ))
}
pub fn css_component_value_list<I>(items: I) -> CssComponentValueList
where
    I: IntoIterator<Item = AnyCssValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_generic_component_value_list<I>(items: I) -> CssGenericComponentValueList
where
    I: IntoIterator<Item = AnyCssGenericComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssGenericComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_parameter_list<I, S>(items: I, separators: S) -> CssParameterList
where
    I: IntoIterator<Item = AnyCssExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TailwindSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssParameterList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn tw_candidate_list<I>(items: I) -> TwCandidateList
where
    I: IntoIterator<Item = AnyTwFullCandidate>,
    I::IntoIter: ExactSizeIterator,
{
    TwCandidateList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_CANDIDATE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn tw_variant_list<I, S>(items: I, separators: S) -> TwVariantList
where
    I: IntoIterator<Item = AnyTwVariant>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = TailwindSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TwVariantList::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_VARIANT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_bogus_property_value<I>(slots: I) -> CssBogusPropertyValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPropertyValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::CSS_BOGUS_PROPERTY_VALUE,
        slots,
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
pub fn tw_bogus_value<I>(slots: I) -> TwBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    TwBogusValue::unwrap_cast(SyntaxNode::new_detached(
        TailwindSyntaxKind::TW_BOGUS_VALUE,
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
