//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_css_syntax::{
    CssSyntaxElement as SyntaxElement, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
    *,
};
use biome_rowan::AstNode;
pub fn css_at_rule(at_token: SyntaxToken, rule: AnyCssAtRule) -> CssAtRule {
    CssAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Node(rule.into_syntax())),
        ],
    ))
}
pub fn css_at_rule_declarator(
    at_token: SyntaxToken,
    declarator: AnyCssAtRuleDeclarator,
) -> CssAtRuleDeclarator {
    CssAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Node(declarator.into_syntax())),
        ],
    ))
}
pub fn css_attr_fallback_value(
    comma_token: SyntaxToken,
    value: CssGenericComponentValueList,
) -> CssAttrFallbackValue {
    CssAttrFallbackValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTR_FALLBACK_VALUE,
        [
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_attr_function(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    attr_name: CssAttrNameList,
    r_paren_token: SyntaxToken,
) -> CssAttrFunctionBuilder {
    CssAttrFunctionBuilder {
        name_token,
        l_paren_token,
        attr_name,
        r_paren_token,
        attr_type: None,
        fallback_value: None,
    }
}
pub struct CssAttrFunctionBuilder {
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    attr_name: CssAttrNameList,
    r_paren_token: SyntaxToken,
    attr_type: Option<AnyCssAttrType>,
    fallback_value: Option<CssAttrFallbackValue>,
}
impl CssAttrFunctionBuilder {
    pub fn with_attr_type(mut self, attr_type: AnyCssAttrType) -> Self {
        self.attr_type = Some(attr_type);
        self
    }
    pub fn with_fallback_value(mut self, fallback_value: CssAttrFallbackValue) -> Self {
        self.fallback_value = Some(fallback_value);
        self
    }
    pub fn build(self) -> CssAttrFunction {
        CssAttrFunction::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTR_FUNCTION,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.attr_name.into_syntax())),
                self.attr_type
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.fallback_value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_attribute_matcher(
    operator_token: SyntaxToken,
    value: CssAttributeMatcherValue,
) -> CssAttributeMatcherBuilder {
    CssAttributeMatcherBuilder {
        operator_token,
        value,
        modifier_token: None,
    }
}
pub struct CssAttributeMatcherBuilder {
    operator_token: SyntaxToken,
    value: CssAttributeMatcherValue,
    modifier_token: Option<SyntaxToken>,
}
impl CssAttributeMatcherBuilder {
    pub fn with_modifier_token(mut self, modifier_token: SyntaxToken) -> Self {
        self.modifier_token = Some(modifier_token);
        self
    }
    pub fn build(self) -> CssAttributeMatcher {
        CssAttributeMatcher::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_MATCHER,
            [
                Some(SyntaxElement::Token(self.operator_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                self.modifier_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn css_attribute_matcher_value(name: AnyCssAttributeMatcherValue) -> CssAttributeMatcherValue {
    CssAttributeMatcherValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_MATCHER_VALUE,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_attribute_name(name: CssIdentifier) -> CssAttributeNameBuilder {
    CssAttributeNameBuilder {
        name,
        namespace: None,
    }
}
pub struct CssAttributeNameBuilder {
    name: CssIdentifier,
    namespace: Option<CssNamespace>,
}
impl CssAttributeNameBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssAttributeName {
        CssAttributeName::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_NAME,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn css_attribute_selector(
    l_brack_token: SyntaxToken,
    name: CssAttributeName,
    r_brack_token: SyntaxToken,
) -> CssAttributeSelectorBuilder {
    CssAttributeSelectorBuilder {
        l_brack_token,
        name,
        r_brack_token,
        matcher: None,
    }
}
pub struct CssAttributeSelectorBuilder {
    l_brack_token: SyntaxToken,
    name: CssAttributeName,
    r_brack_token: SyntaxToken,
    matcher: Option<CssAttributeMatcher>,
}
impl CssAttributeSelectorBuilder {
    pub fn with_matcher(mut self, matcher: CssAttributeMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }
    pub fn build(self) -> CssAttributeSelector {
        CssAttributeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.matcher
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn css_binary_expression(
    left: AnyCssExpression,
    operator_token_token: SyntaxToken,
    right: AnyCssExpression,
) -> CssBinaryExpression {
    CssBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_bracketed_value(
    l_brack_token: SyntaxToken,
    items: CssBracketedValueList,
    r_brack_token: SyntaxToken,
) -> CssBracketedValue {
    CssBracketedValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BRACKETED_VALUE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn css_charset_at_rule(
    charset_token: SyntaxToken,
    encoding: CssString,
    semicolon_token: SyntaxToken,
) -> CssCharsetAtRule {
    CssCharsetAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CHARSET_AT_RULE,
        [
            Some(SyntaxElement::Token(charset_token)),
            Some(SyntaxElement::Node(encoding.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn css_class_selector(dot_token: SyntaxToken, name: CssCustomIdentifier) -> CssClassSelector {
    CssClassSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CLASS_SELECTOR,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_color(hash_token: SyntaxToken, value_token: SyntaxToken) -> CssColor {
    CssColor::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COLOR,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn css_color_profile_at_rule(
    declarator: CssColorProfileAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssColorProfileAtRule {
    CssColorProfileAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COLOR_PROFILE_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_color_profile_at_rule_declarator(
    color_profile_token: SyntaxToken,
    name: CssCustomIdentifier,
) -> CssColorProfileAtRuleDeclarator {
    CssColorProfileAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COLOR_PROFILE_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(color_profile_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_comma_separated_value(
    l_curly_token: SyntaxToken,
    items: CssGenericComponentValueList,
    r_curly_token: SyntaxToken,
) -> CssCommaSeparatedValue {
    CssCommaSeparatedValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMMA_SEPARATED_VALUE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_complex_selector(
    left: AnyCssSelector,
    combinator_token: SyntaxToken,
    right: AnyCssSelector,
) -> CssComplexSelector {
    CssComplexSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPLEX_SELECTOR,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(combinator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_composes_import_specifier(
    from_token: SyntaxToken,
    source: AnyCssComposesImportSource,
) -> CssComposesImportSpecifier {
    CssComposesImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPOSES_IMPORT_SPECIFIER,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
        ],
    ))
}
pub fn css_composes_property(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: CssComposesPropertyValue,
) -> CssComposesProperty {
    CssComposesProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPOSES_PROPERTY,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_composes_property_value(
    classes: CssComposesClassList,
) -> CssComposesPropertyValueBuilder {
    CssComposesPropertyValueBuilder {
        classes,
        specifier: None,
    }
}
pub struct CssComposesPropertyValueBuilder {
    classes: CssComposesClassList,
    specifier: Option<CssComposesImportSpecifier>,
}
impl CssComposesPropertyValueBuilder {
    pub fn with_specifier(mut self, specifier: CssComposesImportSpecifier) -> Self {
        self.specifier = Some(specifier);
        self
    }
    pub fn build(self) -> CssComposesPropertyValue {
        CssComposesPropertyValue::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_COMPOSES_PROPERTY_VALUE,
            [
                Some(SyntaxElement::Node(self.classes.into_syntax())),
                self.specifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_compound_selector(
    nesting_selectors: CssNestedSelectorList,
    sub_selectors: CssSubSelectorList,
) -> CssCompoundSelectorBuilder {
    CssCompoundSelectorBuilder {
        nesting_selectors,
        sub_selectors,
        simple_selector: None,
    }
}
pub struct CssCompoundSelectorBuilder {
    nesting_selectors: CssNestedSelectorList,
    sub_selectors: CssSubSelectorList,
    simple_selector: Option<AnyCssSimpleSelector>,
}
impl CssCompoundSelectorBuilder {
    pub fn with_simple_selector(mut self, simple_selector: AnyCssSimpleSelector) -> Self {
        self.simple_selector = Some(simple_selector);
        self
    }
    pub fn build(self) -> CssCompoundSelector {
        CssCompoundSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_COMPOUND_SELECTOR,
            [
                Some(SyntaxElement::Node(self.nesting_selectors.into_syntax())),
                self.simple_selector
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.sub_selectors.into_syntax())),
            ],
        ))
    }
}
pub fn css_container_and_query(
    left: AnyCssContainerQueryInParens,
    and_token: SyntaxToken,
    right: AnyCssContainerAndCombinableQuery,
) -> CssContainerAndQuery {
    CssContainerAndQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_AND_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_at_rule(
    declarator: CssContainerAtRuleDeclarator,
    block: AnyCssConditionalBlock,
) -> CssContainerAtRule {
    CssContainerAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_container_at_rule_declarator(
    container_token: SyntaxToken,
    query: AnyCssContainerQuery,
) -> CssContainerAtRuleDeclaratorBuilder {
    CssContainerAtRuleDeclaratorBuilder {
        container_token,
        query,
        name: None,
    }
}
pub struct CssContainerAtRuleDeclaratorBuilder {
    container_token: SyntaxToken,
    query: AnyCssContainerQuery,
    name: Option<CssCustomIdentifier>,
}
impl CssContainerAtRuleDeclaratorBuilder {
    pub fn with_name(mut self, name: CssCustomIdentifier) -> Self {
        self.name = Some(name);
        self
    }
    pub fn build(self) -> CssContainerAtRuleDeclarator {
        CssContainerAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_CONTAINER_AT_RULE_DECLARATOR,
            [
                Some(SyntaxElement::Token(self.container_token)),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.query.into_syntax())),
            ],
        ))
    }
}
pub fn css_container_not_query(
    not_token: SyntaxToken,
    query: AnyCssContainerQueryInParens,
) -> CssContainerNotQuery {
    CssContainerNotQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_NOT_QUERY,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
        ],
    ))
}
pub fn css_container_or_query(
    left: AnyCssContainerQueryInParens,
    or_token: SyntaxToken,
    right: AnyCssContainerOrCombinableQuery,
) -> CssContainerOrQuery {
    CssContainerOrQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_OR_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_query_in_parens(
    l_paren_token: SyntaxToken,
    query: AnyCssContainerQuery,
    r_paren_token: SyntaxToken,
) -> CssContainerQueryInParens {
    CssContainerQueryInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_QUERY_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_size_feature_in_parens(
    l_paren_token: SyntaxToken,
    feature: AnyCssQueryFeature,
    r_paren_token: SyntaxToken,
) -> CssContainerSizeFeatureInParens {
    CssContainerSizeFeatureInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_SIZE_FEATURE_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(feature.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_style_and_query(
    left: CssContainerStyleInParens,
    and_token: SyntaxToken,
    right: AnyCssContainerStyleAndCombinableQuery,
) -> CssContainerStyleAndQuery {
    CssContainerStyleAndQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_AND_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_style_in_parens(
    l_paren_token: SyntaxToken,
    query: AnyCssContainerStyleInParens,
    r_paren_token: SyntaxToken,
) -> CssContainerStyleInParens {
    CssContainerStyleInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_style_not_query(
    not_token: SyntaxToken,
    query: CssContainerStyleInParens,
) -> CssContainerStyleNotQuery {
    CssContainerStyleNotQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_NOT_QUERY,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
        ],
    ))
}
pub fn css_container_style_or_query(
    left: CssContainerStyleInParens,
    or_token: SyntaxToken,
    right: AnyCssContainerStyleOrCombinableQuery,
) -> CssContainerStyleOrQuery {
    CssContainerStyleOrQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_OR_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_style_query_in_parens(
    style_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    query: AnyCssContainerStyleQuery,
    r_paren_token: SyntaxToken,
) -> CssContainerStyleQueryInParens {
    CssContainerStyleQueryInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_QUERY_IN_PARENS,
        [
            Some(SyntaxElement::Token(style_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_counter_style_at_rule(
    declarator: CssCounterStyleAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssCounterStyleAtRule {
    CssCounterStyleAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COUNTER_STYLE_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_counter_style_at_rule_declarator(
    counter_style_token: SyntaxToken,
    name: CssCustomIdentifier,
) -> CssCounterStyleAtRuleDeclarator {
    CssCounterStyleAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COUNTER_STYLE_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(counter_style_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_custom_identifier(value_token: SyntaxToken) -> CssCustomIdentifier {
    CssCustomIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CUSTOM_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_dashed_identifier(value_token: SyntaxToken) -> CssDashedIdentifier {
    CssDashedIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DASHED_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_declaration(property: AnyCssProperty) -> CssDeclarationBuilder {
    CssDeclarationBuilder {
        property,
        important: None,
    }
}
pub struct CssDeclarationBuilder {
    property: AnyCssProperty,
    important: Option<CssDeclarationImportant>,
}
impl CssDeclarationBuilder {
    pub fn with_important(mut self, important: CssDeclarationImportant) -> Self {
        self.important = Some(important);
        self
    }
    pub fn build(self) -> CssDeclaration {
        CssDeclaration::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DECLARATION,
            [
                Some(SyntaxElement::Node(self.property.into_syntax())),
                self.important
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_declaration_block(
    l_curly_token: SyntaxToken,
    declarations: CssDeclarationList,
    r_curly_token: SyntaxToken,
) -> CssDeclarationBlock {
    CssDeclarationBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(declarations.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_declaration_important(
    excl_token: SyntaxToken,
    important_token: SyntaxToken,
) -> CssDeclarationImportant {
    CssDeclarationImportant::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_IMPORTANT,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Token(important_token)),
        ],
    ))
}
pub fn css_declaration_or_at_rule_block(
    l_curly_token: SyntaxToken,
    items: CssDeclarationOrAtRuleList,
    r_curly_token: SyntaxToken,
) -> CssDeclarationOrAtRuleBlock {
    CssDeclarationOrAtRuleBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_declaration_or_rule_block(
    l_curly_token: SyntaxToken,
    items: CssDeclarationOrRuleList,
    r_curly_token: SyntaxToken,
) -> CssDeclarationOrRuleBlock {
    CssDeclarationOrRuleBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_declaration_with_semicolon(
    declaration: CssDeclaration,
) -> CssDeclarationWithSemicolonBuilder {
    CssDeclarationWithSemicolonBuilder {
        declaration,
        semicolon_token: None,
    }
}
pub struct CssDeclarationWithSemicolonBuilder {
    declaration: CssDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl CssDeclarationWithSemicolonBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> CssDeclarationWithSemicolon {
        CssDeclarationWithSemicolon::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DECLARATION_WITH_SEMICOLON,
            [
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn css_document_at_rule(
    document_token: SyntaxToken,
    matchers: CssDocumentMatcherList,
    block: AnyCssRuleBlock,
) -> CssDocumentAtRule {
    CssDocumentAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DOCUMENT_AT_RULE,
        [
            Some(SyntaxElement::Token(document_token)),
            Some(SyntaxElement::Node(matchers.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_document_custom_matcher(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> CssDocumentCustomMatcherBuilder {
    CssDocumentCustomMatcherBuilder {
        name_token,
        l_paren_token,
        r_paren_token,
        value: None,
    }
}
pub struct CssDocumentCustomMatcherBuilder {
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    value: Option<AnyCssUrlValue>,
}
impl CssDocumentCustomMatcherBuilder {
    pub fn with_value(mut self, value: AnyCssUrlValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> CssDocumentCustomMatcher {
        CssDocumentCustomMatcher::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DOCUMENT_CUSTOM_MATCHER,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_else_keyword(else_token: SyntaxToken) -> CssElseKeyword {
    CssElseKeyword::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ELSE_KEYWORD,
        [Some(SyntaxElement::Token(else_token))],
    ))
}
pub fn css_empty_declaration(semicolon_token: SyntaxToken) -> CssEmptyDeclaration {
    CssEmptyDeclaration::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_EMPTY_DECLARATION,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn css_font_face_at_rule(
    declarator: CssFontFaceAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssFontFaceAtRule {
    CssFontFaceAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FACE_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_font_face_at_rule_declarator(
    font_face_token: SyntaxToken,
) -> CssFontFaceAtRuleDeclarator {
    CssFontFaceAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FACE_AT_RULE_DECLARATOR,
        [Some(SyntaxElement::Token(font_face_token))],
    ))
}
pub fn css_font_family_name(names: CssCustomIdentifierSpaceSeparatedList) -> CssFontFamilyName {
    CssFontFamilyName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FAMILY_NAME,
        [Some(SyntaxElement::Node(names.into_syntax()))],
    ))
}
pub fn css_font_feature_values_at_rule(
    font_feature_values_token: SyntaxToken,
    names: CssFontFamilyNameList,
    block: AnyCssFontFeatureValuesBlock,
) -> CssFontFeatureValuesAtRule {
    CssFontFeatureValuesAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_AT_RULE,
        [
            Some(SyntaxElement::Token(font_feature_values_token)),
            Some(SyntaxElement::Node(names.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_font_feature_values_block(
    l_curly_token: SyntaxToken,
    items: CssFontFeatureValuesItemList,
    r_curly_token: SyntaxToken,
) -> CssFontFeatureValuesBlock {
    CssFontFeatureValuesBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_font_feature_values_item(
    at_token: SyntaxToken,
    name_token: SyntaxToken,
    block: AnyCssDeclarationBlock,
) -> CssFontFeatureValuesItem {
    CssFontFeatureValuesItem::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_font_palette_values_at_rule(
    declarator: CssFontPaletteValuesAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssFontPaletteValuesAtRule {
    CssFontPaletteValuesAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_PALETTE_VALUES_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_font_palette_values_at_rule_declarator(
    font_palette_values_token: SyntaxToken,
    name: CssDashedIdentifier,
) -> CssFontPaletteValuesAtRuleDeclarator {
    CssFontPaletteValuesAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_PALETTE_VALUES_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(font_palette_values_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_function(
    name: AnyCssFunctionName,
    l_paren_token: SyntaxToken,
    items: CssParameterList,
    r_paren_token: SyntaxToken,
) -> CssFunction {
    CssFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_function_at_rule(
    declarator: CssFunctionAtRuleDeclarator,
    block: CssDeclarationOrAtRuleBlock,
) -> CssFunctionAtRule {
    CssFunctionAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_function_at_rule_declarator(
    function_token: SyntaxToken,
    name: CssDashedIdentifier,
    l_paren_token: SyntaxToken,
    parameters: CssFunctionParameterList,
    r_paren_token: SyntaxToken,
) -> CssFunctionAtRuleDeclaratorBuilder {
    CssFunctionAtRuleDeclaratorBuilder {
        function_token,
        name,
        l_paren_token,
        parameters,
        r_paren_token,
        returns: None,
    }
}
pub struct CssFunctionAtRuleDeclaratorBuilder {
    function_token: SyntaxToken,
    name: CssDashedIdentifier,
    l_paren_token: SyntaxToken,
    parameters: CssFunctionParameterList,
    r_paren_token: SyntaxToken,
    returns: Option<CssReturnsStatement>,
}
impl CssFunctionAtRuleDeclaratorBuilder {
    pub fn with_returns(mut self, returns: CssReturnsStatement) -> Self {
        self.returns = Some(returns);
        self
    }
    pub fn build(self) -> CssFunctionAtRuleDeclarator {
        CssFunctionAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_FUNCTION_AT_RULE_DECLARATOR,
            [
                Some(SyntaxElement::Token(self.function_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.returns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_function_parameter(name: CssDashedIdentifier) -> CssFunctionParameterBuilder {
    CssFunctionParameterBuilder {
        name,
        ty: None,
        default_value: None,
    }
}
pub struct CssFunctionParameterBuilder {
    name: CssDashedIdentifier,
    ty: Option<AnyCssType>,
    default_value: Option<CssFunctionParameterDefaultValue>,
}
impl CssFunctionParameterBuilder {
    pub fn with_ty(mut self, ty: AnyCssType) -> Self {
        self.ty = Some(ty);
        self
    }
    pub fn with_default_value(mut self, default_value: CssFunctionParameterDefaultValue) -> Self {
        self.default_value = Some(default_value);
        self
    }
    pub fn build(self) -> CssFunctionParameter {
        CssFunctionParameter::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_FUNCTION_PARAMETER,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.ty
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.default_value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_function_parameter_default_value(
    colon_token: SyntaxToken,
    value: AnyCssValue,
) -> CssFunctionParameterDefaultValue {
    CssFunctionParameterDefaultValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION_PARAMETER_DEFAULT_VALUE,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_generic_delimiter(value_token: SyntaxToken) -> CssGenericDelimiter {
    CssGenericDelimiter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_GENERIC_DELIMITER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_generic_property(
    name: AnyCssDeclarationName,
    colon_token: SyntaxToken,
    value: CssGenericComponentValueList,
) -> CssGenericProperty {
    CssGenericProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_GENERIC_PROPERTY,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_id_selector(hash_token: SyntaxToken, name: CssCustomIdentifier) -> CssIdSelector {
    CssIdSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ID_SELECTOR,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_identifier(value_token: SyntaxToken) -> CssIdentifier {
    CssIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_if_branch(
    condition: AnyCssIfCondition,
    colon_token: SyntaxToken,
    value: CssGenericComponentValueList,
) -> CssIfBranch {
    CssIfBranch::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_BRANCH,
        [
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_if_function(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    css_if_branch_list: CssIfBranchList,
    r_paren_token: SyntaxToken,
) -> CssIfFunction {
    CssIfFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_FUNCTION,
        [
            Some(SyntaxElement::Token(if_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(css_if_branch_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_if_media_test(
    media_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyCssIfMediaTestQuery,
    r_paren_token: SyntaxToken,
) -> CssIfMediaTest {
    CssIfMediaTest::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_MEDIA_TEST,
        [
            Some(SyntaxElement::Token(media_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_if_style_test(
    style_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyCssContainerStyleQuery,
    r_paren_token: SyntaxToken,
) -> CssIfStyleTest {
    CssIfStyleTest::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_STYLE_TEST,
        [
            Some(SyntaxElement::Token(style_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_if_supports_identifier_test(
    ident: CssIdentifier,
    colon_token: SyntaxToken,
    value: AnyCssGenericComponentValue,
) -> CssIfSupportsIdentifierTest {
    CssIfSupportsIdentifierTest::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_SUPPORTS_IDENTIFIER_TEST,
        [
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_if_supports_test(
    supports_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyCssIfSupportsTestCondition,
    r_paren_token: SyntaxToken,
) -> CssIfSupportsTest {
    CssIfSupportsTest::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_SUPPORTS_TEST,
        [
            Some(SyntaxElement::Token(supports_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_if_test_boolean_and_expr(
    left: AnyCssIfTestBooleanExprGroup,
    and_token: SyntaxToken,
    right: AnyCssIfTestBooleanAndCombinableExpr,
) -> CssIfTestBooleanAndExpr {
    CssIfTestBooleanAndExpr::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_TEST_BOOLEAN_AND_EXPR,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_if_test_boolean_expr_in_parens(
    l_paren_token: SyntaxToken,
    expression: AnyCssIfTestBooleanExpr,
    r_paren_token: SyntaxToken,
) -> CssIfTestBooleanExprInParens {
    CssIfTestBooleanExprInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_TEST_BOOLEAN_EXPR_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_if_test_boolean_not_expr(
    not_token: SyntaxToken,
    expression: AnyCssIfTestBooleanExprGroup,
) -> CssIfTestBooleanNotExpr {
    CssIfTestBooleanNotExpr::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_TEST_BOOLEAN_NOT_EXPR,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn css_if_test_boolean_or_expr(
    left: AnyCssIfTestBooleanExprGroup,
    or_token: SyntaxToken,
    right: AnyCssIfTestBooleanOrCombinableExpr,
) -> CssIfTestBooleanOrExpr {
    CssIfTestBooleanOrExpr::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_TEST_BOOLEAN_OR_EXPR,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_import_anonymous_layer(layer_token: SyntaxToken) -> CssImportAnonymousLayer {
    CssImportAnonymousLayer::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IMPORT_ANONYMOUS_LAYER,
        [Some(SyntaxElement::Token(layer_token))],
    ))
}
pub fn css_import_at_rule(
    import_token: SyntaxToken,
    url: AnyCssImportUrl,
    media: CssMediaQueryList,
    semicolon_token: SyntaxToken,
) -> CssImportAtRuleBuilder {
    CssImportAtRuleBuilder {
        import_token,
        url,
        media,
        semicolon_token,
        layer: None,
        supports: None,
    }
}
pub struct CssImportAtRuleBuilder {
    import_token: SyntaxToken,
    url: AnyCssImportUrl,
    media: CssMediaQueryList,
    semicolon_token: SyntaxToken,
    layer: Option<AnyCssImportLayer>,
    supports: Option<CssImportSupports>,
}
impl CssImportAtRuleBuilder {
    pub fn with_layer(mut self, layer: AnyCssImportLayer) -> Self {
        self.layer = Some(layer);
        self
    }
    pub fn with_supports(mut self, supports: CssImportSupports) -> Self {
        self.supports = Some(supports);
        self
    }
    pub fn build(self) -> CssImportAtRule {
        CssImportAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_IMPORT_AT_RULE,
            [
                Some(SyntaxElement::Token(self.import_token)),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                self.layer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.supports
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.media.into_syntax())),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn css_import_named_layer(
    layer_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    name: CssLayerNameList,
    r_paren_token: SyntaxToken,
) -> CssImportNamedLayer {
    CssImportNamedLayer::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IMPORT_NAMED_LAYER,
        [
            Some(SyntaxElement::Token(layer_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_import_supports(
    supports_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyCssImportSupportsCondition,
    r_paren_token: SyntaxToken,
) -> CssImportSupports {
    CssImportSupports::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IMPORT_SUPPORTS,
        [
            Some(SyntaxElement::Token(supports_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_keyframes_at_rule(
    keyframes_token: SyntaxToken,
    name: AnyCssKeyframesName,
    block: AnyCssKeyframesBlock,
) -> CssKeyframesAtRule {
    CssKeyframesAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_AT_RULE,
        [
            Some(SyntaxElement::Token(keyframes_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_keyframes_block(
    l_curly_token: SyntaxToken,
    items: CssKeyframesItemList,
    r_curly_token: SyntaxToken,
) -> CssKeyframesBlock {
    CssKeyframesBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_keyframes_ident_selector(selector_token: SyntaxToken) -> CssKeyframesIdentSelector {
    CssKeyframesIdentSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_IDENT_SELECTOR,
        [Some(SyntaxElement::Token(selector_token))],
    ))
}
pub fn css_keyframes_item(
    selectors: CssKeyframesSelectorList,
    block: AnyCssDeclarationBlock,
) -> CssKeyframesItem {
    CssKeyframesItem::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_ITEM,
        [
            Some(SyntaxElement::Node(selectors.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_keyframes_percentage_selector(
    selector: CssPercentage,
) -> CssKeyframesPercentageSelector {
    CssKeyframesPercentageSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_PERCENTAGE_SELECTOR,
        [Some(SyntaxElement::Node(selector.into_syntax()))],
    ))
}
pub fn css_keyframes_scope_function(
    scope_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    name: AnyCssKeyframesIdentifier,
    r_paren_token: SyntaxToken,
) -> CssKeyframesScopeFunction {
    CssKeyframesScopeFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SCOPE_FUNCTION,
        [
            Some(SyntaxElement::Token(scope_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_keyframes_scope_prefix(
    scope_token: SyntaxToken,
    name: AnyCssKeyframesIdentifier,
) -> CssKeyframesScopePrefix {
    CssKeyframesScopePrefix::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SCOPE_PREFIX,
        [
            Some(SyntaxElement::Token(scope_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_keyframes_scoped_name(
    colon_token: SyntaxToken,
    scope: AnyCssKeyframesScope,
) -> CssKeyframesScopedName {
    CssKeyframesScopedName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SCOPED_NAME,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(scope.into_syntax())),
        ],
    ))
}
pub fn css_layer_at_rule(layer_token: SyntaxToken, layer: AnyCssLayer) -> CssLayerAtRule {
    CssLayerAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LAYER_AT_RULE,
        [
            Some(SyntaxElement::Token(layer_token)),
            Some(SyntaxElement::Node(layer.into_syntax())),
        ],
    ))
}
pub fn css_layer_declaration(
    references: CssLayerReferenceList,
    block: AnyCssConditionalBlock,
) -> CssLayerDeclaration {
    CssLayerDeclaration::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LAYER_DECLARATION,
        [
            Some(SyntaxElement::Node(references.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_layer_reference(
    references: CssLayerReferenceList,
    semicolon_token: SyntaxToken,
) -> CssLayerReference {
    CssLayerReference::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LAYER_REFERENCE,
        [
            Some(SyntaxElement::Node(references.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn css_list_of_component_values_expression(
    css_component_value_list: CssComponentValueList,
) -> CssListOfComponentValuesExpression {
    CssListOfComponentValuesExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION,
        [Some(SyntaxElement::Node(
            css_component_value_list.into_syntax(),
        ))],
    ))
}
pub fn css_margin_at_rule(
    at_token: SyntaxToken,
    name_token: SyntaxToken,
    block: AnyCssDeclarationOrAtRuleBlock,
) -> CssMarginAtRule {
    CssMarginAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MARGIN_AT_RULE,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_media_and_condition(
    left: AnyCssMediaInParens,
    and_token: SyntaxToken,
    right: AnyCssMediaAndCombinableCondition,
) -> CssMediaAndCondition {
    CssMediaAndCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_AND_CONDITION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_media_and_type_query(
    left: CssMediaTypeQuery,
    and_token: SyntaxToken,
    right: AnyCssMediaTypeCondition,
) -> CssMediaAndTypeQuery {
    CssMediaAndTypeQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_AND_TYPE_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_media_at_rule(
    declarator: CssMediaAtRuleDeclarator,
    block: AnyCssConditionalBlock,
) -> CssMediaAtRule {
    CssMediaAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_media_at_rule_declarator(
    media_token: SyntaxToken,
    queries: CssMediaQueryList,
) -> CssMediaAtRuleDeclarator {
    CssMediaAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(media_token)),
            Some(SyntaxElement::Node(queries.into_syntax())),
        ],
    ))
}
pub fn css_media_condition_in_parens(
    l_paren_token: SyntaxToken,
    condition: AnyCssMediaCondition,
    r_paren_token: SyntaxToken,
) -> CssMediaConditionInParens {
    CssMediaConditionInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_CONDITION_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_media_condition_query(condition: AnyCssMediaCondition) -> CssMediaConditionQuery {
    CssMediaConditionQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_CONDITION_QUERY,
        [Some(SyntaxElement::Node(condition.into_syntax()))],
    ))
}
pub fn css_media_feature_in_parens(
    l_paren_token: SyntaxToken,
    feature: AnyCssQueryFeature,
    r_paren_token: SyntaxToken,
) -> CssMediaFeatureInParens {
    CssMediaFeatureInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_FEATURE_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(feature.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_media_not_condition(
    not_token: SyntaxToken,
    condition: AnyCssMediaInParens,
) -> CssMediaNotCondition {
    CssMediaNotCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_NOT_CONDITION,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn css_media_or_condition(
    left: AnyCssMediaInParens,
    or_token: SyntaxToken,
    right: AnyCssMediaOrCombinableCondition,
) -> CssMediaOrCondition {
    CssMediaOrCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_OR_CONDITION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_media_type(value: CssIdentifier) -> CssMediaType {
    CssMediaType::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_TYPE,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn css_media_type_query(ty: CssMediaType) -> CssMediaTypeQueryBuilder {
    CssMediaTypeQueryBuilder {
        ty,
        modifier_token: None,
    }
}
pub struct CssMediaTypeQueryBuilder {
    ty: CssMediaType,
    modifier_token: Option<SyntaxToken>,
}
impl CssMediaTypeQueryBuilder {
    pub fn with_modifier_token(mut self, modifier_token: SyntaxToken) -> Self {
        self.modifier_token = Some(modifier_token);
        self
    }
    pub fn build(self) -> CssMediaTypeQuery {
        CssMediaTypeQuery::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_MEDIA_TYPE_QUERY,
            [
                self.modifier_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
            ],
        ))
    }
}
pub fn css_metavariable(value_token: SyntaxToken) -> CssMetavariable {
    CssMetavariable::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_METAVARIABLE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_named_namespace_prefix(name: CssIdentifier) -> CssNamedNamespacePrefix {
    CssNamedNamespacePrefix::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NAMED_NAMESPACE_PREFIX,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_namespace(bitwise_or_token: SyntaxToken) -> CssNamespaceBuilder {
    CssNamespaceBuilder {
        bitwise_or_token,
        prefix: None,
    }
}
pub struct CssNamespaceBuilder {
    bitwise_or_token: SyntaxToken,
    prefix: Option<AnyCssNamespacePrefix>,
}
impl CssNamespaceBuilder {
    pub fn with_prefix(mut self, prefix: AnyCssNamespacePrefix) -> Self {
        self.prefix = Some(prefix);
        self
    }
    pub fn build(self) -> CssNamespace {
        CssNamespace::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_NAMESPACE,
            [
                self.prefix
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.bitwise_or_token)),
            ],
        ))
    }
}
pub fn css_namespace_at_rule(
    namespace_token: SyntaxToken,
    url: AnyCssNamespaceUrl,
    semicolon_token: SyntaxToken,
) -> CssNamespaceAtRuleBuilder {
    CssNamespaceAtRuleBuilder {
        namespace_token,
        url,
        semicolon_token,
        prefix: None,
    }
}
pub struct CssNamespaceAtRuleBuilder {
    namespace_token: SyntaxToken,
    url: AnyCssNamespaceUrl,
    semicolon_token: SyntaxToken,
    prefix: Option<CssIdentifier>,
}
impl CssNamespaceAtRuleBuilder {
    pub fn with_prefix(mut self, prefix: CssIdentifier) -> Self {
        self.prefix = Some(prefix);
        self
    }
    pub fn build(self) -> CssNamespaceAtRule {
        CssNamespaceAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_NAMESPACE_AT_RULE,
            [
                Some(SyntaxElement::Token(self.namespace_token)),
                self.prefix
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn css_nested_qualified_rule(
    prelude: CssRelativeSelectorList,
    block: AnyCssDeclarationOrRuleBlock,
) -> CssNestedQualifiedRule {
    CssNestedQualifiedRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE,
        [
            Some(SyntaxElement::Node(prelude.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_nested_selector(amp_token: SyntaxToken) -> CssNestedSelector {
    CssNestedSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NESTED_SELECTOR,
        [Some(SyntaxElement::Token(amp_token))],
    ))
}
pub fn css_nth_offset(sign_token: SyntaxToken, value: CssNumber) -> CssNthOffset {
    CssNthOffset::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NTH_OFFSET,
        [
            Some(SyntaxElement::Token(sign_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_number(value_token: SyntaxToken) -> CssNumber {
    CssNumber::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_number_declarator(number_token: SyntaxToken) -> CssNumberDeclarator {
    CssNumberDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NUMBER_DECLARATOR,
        [Some(SyntaxElement::Token(number_token))],
    ))
}
pub fn css_page_at_rule(
    page_token: SyntaxToken,
    selectors: CssPageSelectorList,
    block: AnyCssPageAtRuleBlock,
) -> CssPageAtRule {
    CssPageAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_AT_RULE,
        [
            Some(SyntaxElement::Token(page_token)),
            Some(SyntaxElement::Node(selectors.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_page_at_rule_block(
    l_curly_token: SyntaxToken,
    items: CssPageAtRuleItemList,
    r_curly_token: SyntaxToken,
) -> CssPageAtRuleBlock {
    CssPageAtRuleBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_AT_RULE_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_page_selector(pseudos: CssPageSelectorPseudoList) -> CssPageSelectorBuilder {
    CssPageSelectorBuilder { pseudos, ty: None }
}
pub struct CssPageSelectorBuilder {
    pseudos: CssPageSelectorPseudoList,
    ty: Option<CssCustomIdentifier>,
}
impl CssPageSelectorBuilder {
    pub fn with_ty(mut self, ty: CssCustomIdentifier) -> Self {
        self.ty = Some(ty);
        self
    }
    pub fn build(self) -> CssPageSelector {
        CssPageSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PAGE_SELECTOR,
            [
                self.ty
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.pseudos.into_syntax())),
            ],
        ))
    }
}
pub fn css_page_selector_pseudo(
    colon_token: SyntaxToken,
    selector: CssIdentifier,
) -> CssPageSelectorPseudo {
    CssPageSelectorPseudo::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
        ],
    ))
}
pub fn css_parameter(any_css_expression: AnyCssExpression) -> CssParameter {
    CssParameter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER,
        [Some(SyntaxElement::Node(any_css_expression.into_syntax()))],
    ))
}
pub fn css_parenthesized_expression(
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> CssParenthesizedExpressionBuilder {
    CssParenthesizedExpressionBuilder {
        l_paren_token,
        r_paren_token,
        expression: None,
    }
}
pub struct CssParenthesizedExpressionBuilder {
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    expression: Option<AnyCssExpression>,
}
impl CssParenthesizedExpressionBuilder {
    pub fn with_expression(mut self, expression: AnyCssExpression) -> Self {
        self.expression = Some(expression);
        self
    }
    pub fn build(self) -> CssParenthesizedExpression {
        CssParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PARENTHESIZED_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.expression
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_percentage(value_token: SyntaxToken, percent_token: SyntaxToken) -> CssPercentage {
    CssPercentage::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PERCENTAGE,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(percent_token)),
        ],
    ))
}
pub fn css_position_try_at_rule(
    declarator: CssPositionTryAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssPositionTryAtRule {
    CssPositionTryAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_POSITION_TRY_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_position_try_at_rule_declarator(
    position_try_token: SyntaxToken,
    name: CssDashedIdentifier,
) -> CssPositionTryAtRuleDeclarator {
    CssPositionTryAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_POSITION_TRY_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(position_try_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_property_at_rule(
    declarator: CssPropertyAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssPropertyAtRule {
    CssPropertyAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PROPERTY_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_property_at_rule_declarator(
    property_token: SyntaxToken,
    name: CssDashedIdentifier,
) -> CssPropertyAtRuleDeclarator {
    CssPropertyAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PROPERTY_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(property_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_pseudo_class_function_compound_selector(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selector: AnyCssCompoundSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCompoundSelector {
    CssPseudoClassFunctionCompoundSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_compound_selector_list(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    compound_selectors: CssCompoundSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCompoundSelectorList {
    CssPseudoClassFunctionCompoundSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(compound_selectors.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_custom_identifier(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    ident: CssCustomIdentifier,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCustomIdentifier {
    CssPseudoClassFunctionCustomIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_custom_identifier_list(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    items: CssCustomIdentifierCommaSeparatedList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCustomIdentifierList {
    CssPseudoClassFunctionCustomIdentifierList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_identifier(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    ident: CssIdentifier,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionIdentifier {
    CssPseudoClassFunctionIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_nth(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selector: AnyCssPseudoClassNthSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionNth {
    CssPseudoClassFunctionNth::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_NTH,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_relative_selector_list(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    relative_selectors: CssRelativeSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionRelativeSelectorList {
    CssPseudoClassFunctionRelativeSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(relative_selectors.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_selector(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selector: AnyCssSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionSelector {
    CssPseudoClassFunctionSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_selector_list(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selectors: CssSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionSelectorList {
    CssPseudoClassFunctionSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selectors.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_value_list(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    values: CssPseudoValueList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionValueList {
    CssPseudoClassFunctionValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(values.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_identifier(name: CssIdentifier) -> CssPseudoClassIdentifier {
    CssPseudoClassIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_IDENTIFIER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_pseudo_class_nth(symbol_token: SyntaxToken) -> CssPseudoClassNthBuilder {
    CssPseudoClassNthBuilder {
        symbol_token,
        sign_token: None,
        value: None,
        offset: None,
    }
}
pub struct CssPseudoClassNthBuilder {
    symbol_token: SyntaxToken,
    sign_token: Option<SyntaxToken>,
    value: Option<CssNumber>,
    offset: Option<CssNthOffset>,
}
impl CssPseudoClassNthBuilder {
    pub fn with_sign_token(mut self, sign_token: SyntaxToken) -> Self {
        self.sign_token = Some(sign_token);
        self
    }
    pub fn with_value(mut self, value: CssNumber) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_offset(mut self, offset: CssNthOffset) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn build(self) -> CssPseudoClassNth {
        CssPseudoClassNth::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH,
            [
                self.sign_token.map(|token| SyntaxElement::Token(token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.symbol_token)),
                self.offset
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_nth_identifier(value_token: SyntaxToken) -> CssPseudoClassNthIdentifier {
    CssPseudoClassNthIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_pseudo_class_nth_number(value: CssNumber) -> CssPseudoClassNthNumberBuilder {
    CssPseudoClassNthNumberBuilder {
        value,
        sign_token: None,
    }
}
pub struct CssPseudoClassNthNumberBuilder {
    value: CssNumber,
    sign_token: Option<SyntaxToken>,
}
impl CssPseudoClassNthNumberBuilder {
    pub fn with_sign_token(mut self, sign_token: SyntaxToken) -> Self {
        self.sign_token = Some(sign_token);
        self
    }
    pub fn build(self) -> CssPseudoClassNthNumber {
        CssPseudoClassNthNumber::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_NUMBER,
            [
                self.sign_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_nth_selector(
    nth: AnyCssPseudoClassNth,
) -> CssPseudoClassNthSelectorBuilder {
    CssPseudoClassNthSelectorBuilder {
        nth,
        of_selector: None,
    }
}
pub struct CssPseudoClassNthSelectorBuilder {
    nth: AnyCssPseudoClassNth,
    of_selector: Option<CssPseudoClassOfNthSelector>,
}
impl CssPseudoClassNthSelectorBuilder {
    pub fn with_of_selector(mut self, of_selector: CssPseudoClassOfNthSelector) -> Self {
        self.of_selector = Some(of_selector);
        self
    }
    pub fn build(self) -> CssPseudoClassNthSelector {
        CssPseudoClassNthSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_SELECTOR,
            [
                Some(SyntaxElement::Node(self.nth.into_syntax())),
                self.of_selector
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_of_nth_selector(
    of_token: SyntaxToken,
    selectors: CssSelectorList,
) -> CssPseudoClassOfNthSelector {
    CssPseudoClassOfNthSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_OF_NTH_SELECTOR,
        [
            Some(SyntaxElement::Token(of_token)),
            Some(SyntaxElement::Node(selectors.into_syntax())),
        ],
    ))
}
pub fn css_pseudo_class_selector(
    colon_token: SyntaxToken,
    class: AnyCssPseudoClass,
) -> CssPseudoClassSelector {
    CssPseudoClassSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(class.into_syntax())),
        ],
    ))
}
pub fn css_pseudo_element_function(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    items: CssPseudoElementFunctionParameterList,
    r_paren_token: SyntaxToken,
) -> CssPseudoElementFunction {
    CssPseudoElementFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_element_function_custom_identifier(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    ident: CssCustomIdentifier,
    r_paren_token: SyntaxToken,
) -> CssPseudoElementFunctionCustomIdentifier {
    CssPseudoElementFunctionCustomIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_element_function_selector(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selector: AnyCssSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoElementFunctionSelector {
    CssPseudoElementFunctionSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_element_identifier(name: CssIdentifier) -> CssPseudoElementIdentifier {
    CssPseudoElementIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_IDENTIFIER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_pseudo_element_selector(
    double_colon_token: SyntaxToken,
    element: AnyCssPseudoElement,
) -> CssPseudoElementSelector {
    CssPseudoElementSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_SELECTOR,
        [
            Some(SyntaxElement::Token(double_colon_token)),
            Some(SyntaxElement::Node(element.into_syntax())),
        ],
    ))
}
pub fn css_qualified_rule(
    prelude: CssSelectorList,
    block: AnyCssDeclarationOrRuleBlock,
) -> CssQualifiedRule {
    CssQualifiedRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUALIFIED_RULE,
        [
            Some(SyntaxElement::Node(prelude.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_query_feature_boolean(name: CssIdentifier) -> CssQueryFeatureBoolean {
    CssQueryFeatureBoolean::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_BOOLEAN,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_query_feature_plain(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: AnyCssQueryFeatureValue,
) -> CssQueryFeaturePlain {
    CssQueryFeaturePlain::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_PLAIN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_query_feature_range(
    left: CssIdentifier,
    comparison: CssQueryFeatureRangeComparison,
    right: AnyCssQueryFeatureValue,
) -> CssQueryFeatureRange {
    CssQueryFeatureRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Node(comparison.into_syntax())),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_query_feature_range_comparison(
    operator_token: SyntaxToken,
) -> CssQueryFeatureRangeComparison {
    CssQueryFeatureRangeComparison::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_COMPARISON,
        [Some(SyntaxElement::Token(operator_token))],
    ))
}
pub fn css_query_feature_range_interval(
    left: AnyCssQueryFeatureValue,
    left_comparison: CssQueryFeatureRangeComparison,
    name: CssIdentifier,
    right_comparison: CssQueryFeatureRangeComparison,
    right: AnyCssQueryFeatureValue,
) -> CssQueryFeatureRangeInterval {
    CssQueryFeatureRangeInterval::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_INTERVAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Node(left_comparison.into_syntax())),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(right_comparison.into_syntax())),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_query_feature_reverse_range(
    left: AnyCssQueryFeatureValue,
    comparison: CssQueryFeatureRangeComparison,
    right: CssIdentifier,
) -> CssQueryFeatureReverseRange {
    CssQueryFeatureReverseRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_QUERY_FEATURE_REVERSE_RANGE,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Node(comparison.into_syntax())),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_ratio(
    numerator: CssNumber,
    slash_token: SyntaxToken,
    denominator: CssNumber,
) -> CssRatio {
    CssRatio::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RATIO,
        [
            Some(SyntaxElement::Node(numerator.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(denominator.into_syntax())),
        ],
    ))
}
pub fn css_raw_string_declarator(raw_string_token: SyntaxToken) -> CssRawStringDeclarator {
    CssRawStringDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RAW_STRING_DECLARATOR,
        [Some(SyntaxElement::Token(raw_string_token))],
    ))
}
pub fn css_regular_attr_unit(unit_token: SyntaxToken) -> CssRegularAttrUnit {
    CssRegularAttrUnit::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_REGULAR_ATTR_UNIT,
        [Some(SyntaxElement::Token(unit_token))],
    ))
}
pub fn css_regular_dimension(
    value_token: SyntaxToken,
    unit_token: SyntaxToken,
) -> CssRegularDimension {
    CssRegularDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_REGULAR_DIMENSION,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(unit_token)),
        ],
    ))
}
pub fn css_regular_syntax_type_name(name_token: SyntaxToken) -> CssRegularSyntaxTypeName {
    CssRegularSyntaxTypeName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_REGULAR_SYNTAX_TYPE_NAME,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn css_relative_selector(selector: AnyCssSelector) -> CssRelativeSelectorBuilder {
    CssRelativeSelectorBuilder {
        selector,
        combinator_token: None,
    }
}
pub struct CssRelativeSelectorBuilder {
    selector: AnyCssSelector,
    combinator_token: Option<SyntaxToken>,
}
impl CssRelativeSelectorBuilder {
    pub fn with_combinator_token(mut self, combinator_token: SyntaxToken) -> Self {
        self.combinator_token = Some(combinator_token);
        self
    }
    pub fn build(self) -> CssRelativeSelector {
        CssRelativeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_RELATIVE_SELECTOR,
            [
                self.combinator_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.selector.into_syntax())),
            ],
        ))
    }
}
pub fn css_returns_statement(returns_token: SyntaxToken, ty: AnyCssType) -> CssReturnsStatement {
    CssReturnsStatement::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RETURNS_STATEMENT,
        [
            Some(SyntaxElement::Token(returns_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn css_root(items: CssRootItemList, eof_token: SyntaxToken) -> CssRootBuilder {
    CssRootBuilder {
        items,
        eof_token,
        bom_token: None,
    }
}
pub struct CssRootBuilder {
    items: CssRootItemList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl CssRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> CssRoot {
        CssRoot::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.items.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn css_rule_block(
    l_curly_token: SyntaxToken,
    rules: CssRuleList,
    r_curly_token: SyntaxToken,
) -> CssRuleBlock {
    CssRuleBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RULE_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(rules.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_scope_at_rule(
    declarator: CssScopeAtRuleDeclarator,
    block: AnyCssConditionalBlock,
) -> CssScopeAtRule {
    CssScopeAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SCOPE_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_scope_at_rule_declarator(scope_token: SyntaxToken) -> CssScopeAtRuleDeclaratorBuilder {
    CssScopeAtRuleDeclaratorBuilder {
        scope_token,
        range: None,
    }
}
pub struct CssScopeAtRuleDeclaratorBuilder {
    scope_token: SyntaxToken,
    range: Option<AnyCssScopeRange>,
}
impl CssScopeAtRuleDeclaratorBuilder {
    pub fn with_range(mut self, range: AnyCssScopeRange) -> Self {
        self.range = Some(range);
        self
    }
    pub fn build(self) -> CssScopeAtRuleDeclarator {
        CssScopeAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_SCOPE_AT_RULE_DECLARATOR,
            [
                Some(SyntaxElement::Token(self.scope_token)),
                self.range
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_scope_edge(
    l_paren_token: SyntaxToken,
    selectors: CssSelectorList,
    r_paren_token: SyntaxToken,
) -> CssScopeEdge {
    CssScopeEdge::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SCOPE_EDGE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selectors.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_scope_range_end(to_token: SyntaxToken, end: CssScopeEdge) -> CssScopeRangeEnd {
    CssScopeRangeEnd::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SCOPE_RANGE_END,
        [
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(end.into_syntax())),
        ],
    ))
}
pub fn css_scope_range_interval(
    start: CssScopeEdge,
    to_token: SyntaxToken,
    end: CssScopeEdge,
) -> CssScopeRangeInterval {
    CssScopeRangeInterval::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SCOPE_RANGE_INTERVAL,
        [
            Some(SyntaxElement::Node(start.into_syntax())),
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(end.into_syntax())),
        ],
    ))
}
pub fn css_scope_range_start(start: CssScopeEdge) -> CssScopeRangeStart {
    CssScopeRangeStart::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SCOPE_RANGE_START,
        [Some(SyntaxElement::Node(start.into_syntax()))],
    ))
}
pub fn css_snippet_root(items: CssDeclarationOrRuleList, eof_token: SyntaxToken) -> CssSnippetRoot {
    CssSnippetRoot::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SNIPPET_ROOT,
        [
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn css_starting_style_at_rule(
    declarator: CssStartingStyleAtRuleDeclarator,
    block: AnyCssConditionalBlock,
) -> CssStartingStyleAtRule {
    CssStartingStyleAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STARTING_STYLE_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_starting_style_at_rule_declarator(
    starting_style_token: SyntaxToken,
) -> CssStartingStyleAtRuleDeclarator {
    CssStartingStyleAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STARTING_STYLE_AT_RULE_DECLARATOR,
        [Some(SyntaxElement::Token(starting_style_token))],
    ))
}
pub fn css_string(value_token: SyntaxToken) -> CssString {
    CssString::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_supports_and_condition(
    left: AnyCssSupportsInParens,
    and_token: SyntaxToken,
    right: AnyCssSupportsAndCombinableCondition,
) -> CssSupportsAndCondition {
    CssSupportsAndCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_AND_CONDITION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_supports_at_rule(
    declarator: CssSupportsAtRuleDeclarator,
    block: AnyCssConditionalBlock,
) -> CssSupportsAtRule {
    CssSupportsAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_supports_at_rule_declarator(
    supports_token: SyntaxToken,
    condition: AnyCssSupportsCondition,
) -> CssSupportsAtRuleDeclarator {
    CssSupportsAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_AT_RULE_DECLARATOR,
        [
            Some(SyntaxElement::Token(supports_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn css_supports_condition_in_parens(
    l_paren_token: SyntaxToken,
    condition: AnyCssSupportsCondition,
    r_paren_token: SyntaxToken,
) -> CssSupportsConditionInParens {
    CssSupportsConditionInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_CONDITION_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_supports_feature_declaration(
    l_paren_token: SyntaxToken,
    declaration: CssDeclaration,
    r_paren_token: SyntaxToken,
) -> CssSupportsFeatureDeclaration {
    CssSupportsFeatureDeclaration::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_FEATURE_DECLARATION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(declaration.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_supports_feature_selector(
    selector_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    selector: AnyCssSelector,
    r_paren_token: SyntaxToken,
) -> CssSupportsFeatureSelector {
    CssSupportsFeatureSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_FEATURE_SELECTOR,
        [
            Some(SyntaxElement::Token(selector_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_supports_not_condition(
    not_token: SyntaxToken,
    query: AnyCssSupportsInParens,
) -> CssSupportsNotCondition {
    CssSupportsNotCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_NOT_CONDITION,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
        ],
    ))
}
pub fn css_supports_or_condition(
    left: AnyCssSupportsInParens,
    or_token: SyntaxToken,
    right: AnyCssSupportsOrCombinableCondition,
) -> CssSupportsOrCondition {
    CssSupportsOrCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUPPORTS_OR_CONDITION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_syntax_component(component: AnyCssSyntaxSingleComponent) -> CssSyntaxComponentBuilder {
    CssSyntaxComponentBuilder {
        component,
        multiplier: None,
    }
}
pub struct CssSyntaxComponentBuilder {
    component: AnyCssSyntaxSingleComponent,
    multiplier: Option<CssSyntaxMultiplier>,
}
impl CssSyntaxComponentBuilder {
    pub fn with_multiplier(mut self, multiplier: CssSyntaxMultiplier) -> Self {
        self.multiplier = Some(multiplier);
        self
    }
    pub fn build(self) -> CssSyntaxComponent {
        CssSyntaxComponent::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_SYNTAX_COMPONENT,
            [
                Some(SyntaxElement::Node(self.component.into_syntax())),
                self.multiplier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_syntax_component_without_multiplier(
    l_angle_token: SyntaxToken,
    type_name_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> CssSyntaxComponentWithoutMultiplier {
    CssSyntaxComponentWithoutMultiplier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SYNTAX_COMPONENT_WITHOUT_MULTIPLIER,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(type_name_token)),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn css_syntax_multiplier(multiplier_token: SyntaxToken) -> CssSyntaxMultiplier {
    CssSyntaxMultiplier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SYNTAX_MULTIPLIER,
        [Some(SyntaxElement::Token(multiplier_token))],
    ))
}
pub fn css_syntax_type(
    l_angle_token: SyntaxToken,
    type_name: AnyCssSyntaxTypeName,
    r_angle_token: SyntaxToken,
) -> CssSyntaxType {
    CssSyntaxType::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SYNTAX_TYPE,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(type_name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn css_type_function(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    ty: AnyCssSyntax,
    r_paren_token: SyntaxToken,
) -> CssTypeFunction {
    CssTypeFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_TYPE_FUNCTION,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_type_selector(ident: CssIdentifier) -> CssTypeSelectorBuilder {
    CssTypeSelectorBuilder {
        ident,
        namespace: None,
    }
}
pub struct CssTypeSelectorBuilder {
    ident: CssIdentifier,
    namespace: Option<CssNamespace>,
}
impl CssTypeSelectorBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssTypeSelector {
        CssTypeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_TYPE_SELECTOR,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.ident.into_syntax())),
            ],
        ))
    }
}
pub fn css_unary_expression(
    operator_token: SyntaxToken,
    expression: AnyCssExpression,
) -> CssUnaryExpression {
    CssUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn css_unicode_codepoint(value_token: SyntaxToken) -> CssUnicodeCodepoint {
    CssUnicodeCodepoint::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNICODE_CODEPOINT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_unicode_range(prefix_token: SyntaxToken, value: AnyCssUnicodeValue) -> CssUnicodeRange {
    CssUnicodeRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNICODE_RANGE,
        [
            Some(SyntaxElement::Token(prefix_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_unicode_range_interval(
    start: CssUnicodeCodepoint,
    minus_token: SyntaxToken,
    end: CssUnicodeCodepoint,
) -> CssUnicodeRangeInterval {
    CssUnicodeRangeInterval::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNICODE_RANGE_INTERVAL,
        [
            Some(SyntaxElement::Node(start.into_syntax())),
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(end.into_syntax())),
        ],
    ))
}
pub fn css_unicode_range_wildcard(value_token: SyntaxToken) -> CssUnicodeRangeWildcard {
    CssUnicodeRangeWildcard::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNICODE_RANGE_WILDCARD,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_universal_namespace_prefix(star_token: SyntaxToken) -> CssUniversalNamespacePrefix {
    CssUniversalNamespacePrefix::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNIVERSAL_NAMESPACE_PREFIX,
        [Some(SyntaxElement::Token(star_token))],
    ))
}
pub fn css_universal_selector(star_token: SyntaxToken) -> CssUniversalSelectorBuilder {
    CssUniversalSelectorBuilder {
        star_token,
        namespace: None,
    }
}
pub struct CssUniversalSelectorBuilder {
    star_token: SyntaxToken,
    namespace: Option<CssNamespace>,
}
impl CssUniversalSelectorBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssUniversalSelector {
        CssUniversalSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_UNIVERSAL_SELECTOR,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.star_token)),
            ],
        ))
    }
}
pub fn css_universal_syntax(star_token: SyntaxToken) -> CssUniversalSyntax {
    CssUniversalSyntax::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNIVERSAL_SYNTAX,
        [Some(SyntaxElement::Token(star_token))],
    ))
}
pub fn css_unknown_attr_unit(unit_token: SyntaxToken) -> CssUnknownAttrUnit {
    CssUnknownAttrUnit::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_ATTR_UNIT,
        [Some(SyntaxElement::Token(unit_token))],
    ))
}
pub fn css_unknown_block_at_rule(
    name: CssIdentifier,
    components: CssUnknownAtRuleComponentList,
    block: AnyCssDeclarationOrRuleBlock,
) -> CssUnknownBlockAtRule {
    CssUnknownBlockAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_BLOCK_AT_RULE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(components.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_unknown_dimension(
    value_token: SyntaxToken,
    unit_token: SyntaxToken,
) -> CssUnknownDimension {
    CssUnknownDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_DIMENSION,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Token(unit_token)),
        ],
    ))
}
pub fn css_unknown_syntax_type_name(name_token: SyntaxToken) -> CssUnknownSyntaxTypeName {
    CssUnknownSyntaxTypeName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_SYNTAX_TYPE_NAME,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn css_unknown_value_at_rule(
    name: CssIdentifier,
    components: CssUnknownAtRuleComponentList,
    semicolon_token: SyntaxToken,
) -> CssUnknownValueAtRule {
    CssUnknownValueAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_VALUE_AT_RULE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(components.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn css_url_function(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    modifiers: CssUrlModifierList,
    r_paren_token: SyntaxToken,
) -> CssUrlFunctionBuilder {
    CssUrlFunctionBuilder {
        name_token,
        l_paren_token,
        modifiers,
        r_paren_token,
        value: None,
    }
}
pub struct CssUrlFunctionBuilder {
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    modifiers: CssUrlModifierList,
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
            CssSyntaxKind::CSS_URL_FUNCTION,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_url_value_raw(value_token: SyntaxToken) -> CssUrlValueRaw {
    CssUrlValueRaw::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_URL_VALUE_RAW,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_value_at_rule(
    value_token: SyntaxToken,
    clause: AnyCssValueAtRuleClause,
    semicolon_token: SyntaxToken,
) -> CssValueAtRule {
    CssValueAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE,
        [
            Some(SyntaxElement::Token(value_token)),
            Some(SyntaxElement::Node(clause.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn css_value_at_rule_declaration_clause(
    properties: CssValueAtRulePropertyList,
) -> CssValueAtRuleDeclarationClause {
    CssValueAtRuleDeclarationClause::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_DECLARATION_CLAUSE,
        [Some(SyntaxElement::Node(properties.into_syntax()))],
    ))
}
pub fn css_value_at_rule_generic_property(
    name: AnyCssDeclarationName,
    colon_token: SyntaxToken,
    value: CssValueAtRuleGenericValue,
) -> CssValueAtRuleGenericProperty {
    CssValueAtRuleGenericProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_GENERIC_PROPERTY,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_value_at_rule_import_clause(
    specifiers: CssValueAtRuleImportSpecifierList,
    from_token: SyntaxToken,
    source: AnyCssValueAtRuleImportSource,
) -> CssValueAtRuleImportClause {
    CssValueAtRuleImportClause::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_CLAUSE,
        [
            Some(SyntaxElement::Node(specifiers.into_syntax())),
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
        ],
    ))
}
pub fn css_value_at_rule_import_specifier(name: CssIdentifier) -> CssValueAtRuleImportSpecifier {
    CssValueAtRuleImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_SPECIFIER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_value_at_rule_named_import_specifier(
    name: CssIdentifier,
    as_token: SyntaxToken,
    local_name: CssIdentifier,
) -> CssValueAtRuleNamedImportSpecifier {
    CssValueAtRuleNamedImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(local_name.into_syntax())),
        ],
    ))
}
pub fn css_view_transition_at_rule(
    declarator: CssViewTransitionAtRuleDeclarator,
    block: AnyCssDeclarationBlock,
) -> CssViewTransitionAtRule {
    CssViewTransitionAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VIEW_TRANSITION_AT_RULE,
        [
            Some(SyntaxElement::Node(declarator.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_view_transition_at_rule_declarator(
    view_transition_token: SyntaxToken,
) -> CssViewTransitionAtRuleDeclarator {
    CssViewTransitionAtRuleDeclarator::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VIEW_TRANSITION_AT_RULE_DECLARATOR,
        [Some(SyntaxElement::Token(view_transition_token))],
    ))
}
pub fn scss_arbitrary_argument(
    value: AnyScssExpression,
    dotdotdot_token: SyntaxToken,
) -> ScssArbitraryArgument {
    ScssArbitraryArgument::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_ARBITRARY_ARGUMENT,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(dotdotdot_token)),
        ],
    ))
}
pub fn scss_binary_expression(
    left: AnyScssExpression,
    operator_token: SyntaxToken,
    right: AnyScssExpression,
) -> ScssBinaryExpression {
    ScssBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn scss_declaration(
    name: AnyScssDeclarationName,
    colon_token: SyntaxToken,
    value: ScssExpression,
    modifiers: ScssVariableModifierList,
) -> ScssDeclarationBuilder {
    ScssDeclarationBuilder {
        name,
        colon_token,
        value,
        modifiers,
        semicolon_token: None,
    }
}
pub struct ScssDeclarationBuilder {
    name: AnyScssDeclarationName,
    colon_token: SyntaxToken,
    value: ScssExpression,
    modifiers: ScssVariableModifierList,
    semicolon_token: Option<SyntaxToken>,
}
impl ScssDeclarationBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> ScssDeclaration {
        ScssDeclaration::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::SCSS_DECLARATION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn scss_expression(items: ScssExpressionItemList) -> ScssExpression {
    ScssExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_EXPRESSION,
        [Some(SyntaxElement::Node(items.into_syntax()))],
    ))
}
pub fn scss_identifier(dollar_token: SyntaxToken, name: CssIdentifier) -> ScssIdentifier {
    ScssIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_IDENTIFIER,
        [
            Some(SyntaxElement::Token(dollar_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn scss_keyword_argument(
    name: ScssIdentifier,
    colon_token: SyntaxToken,
    value: AnyScssExpression,
) -> ScssKeywordArgument {
    ScssKeywordArgument::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_KEYWORD_ARGUMENT,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn scss_list_expression(elements: ScssListExpressionElementList) -> ScssListExpression {
    ScssListExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_LIST_EXPRESSION,
        [Some(SyntaxElement::Node(elements.into_syntax()))],
    ))
}
pub fn scss_list_expression_element(value: AnyScssExpression) -> ScssListExpressionElement {
    ScssListExpressionElement::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_LIST_EXPRESSION_ELEMENT,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn scss_map_expression(
    l_paren_token: SyntaxToken,
    pairs: ScssMapExpressionPairList,
    r_paren_token: SyntaxToken,
) -> ScssMapExpression {
    ScssMapExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_MAP_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(pairs.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn scss_map_expression_pair(
    key: AnyScssExpression,
    colon_token: SyntaxToken,
    value: AnyScssExpression,
) -> ScssMapExpressionPair {
    ScssMapExpressionPair::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_MAP_EXPRESSION_PAIR,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn scss_namespaced_identifier(
    namespace: CssIdentifier,
    dot_token: SyntaxToken,
    name: ScssIdentifier,
) -> ScssNamespacedIdentifier {
    ScssNamespacedIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_NAMESPACED_IDENTIFIER,
        [
            Some(SyntaxElement::Node(namespace.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn scss_nesting_declaration(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: CssGenericComponentValueList,
    block: AnyCssDeclarationOrRuleBlock,
) -> ScssNestingDeclaration {
    ScssNestingDeclaration::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_NESTING_DECLARATION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn scss_parent_selector_value(amp_token: SyntaxToken) -> ScssParentSelectorValue {
    ScssParentSelectorValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_PARENT_SELECTOR_VALUE,
        [Some(SyntaxElement::Token(amp_token))],
    ))
}
pub fn scss_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: AnyScssExpression,
    r_paren_token: SyntaxToken,
) -> ScssParenthesizedExpression {
    ScssParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn scss_qualified_name(
    module: CssIdentifier,
    dot_token: SyntaxToken,
    member: AnyScssModuleMember,
) -> ScssQualifiedName {
    ScssQualifiedName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_QUALIFIED_NAME,
        [
            Some(SyntaxElement::Node(module.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn scss_unary_expression(
    operator_token: SyntaxToken,
    expression: AnyScssExpression,
) -> ScssUnaryExpression {
    ScssUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn scss_variable_modifier(
    excl_token: SyntaxToken,
    value_token: SyntaxToken,
) -> ScssVariableModifier {
    ScssVariableModifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_VARIABLE_MODIFIER,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn tw_apply_at_rule(
    apply_token: SyntaxToken,
    classes: TwApplyClassList,
    semicolon_token: SyntaxToken,
) -> TwApplyAtRule {
    TwApplyAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_APPLY_AT_RULE,
        [
            Some(SyntaxElement::Token(apply_token)),
            Some(SyntaxElement::Node(classes.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn tw_config_at_rule(
    config_token: SyntaxToken,
    path: CssString,
    semicolon_token: SyntaxToken,
) -> TwConfigAtRule {
    TwConfigAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_CONFIG_AT_RULE,
        [
            Some(SyntaxElement::Token(config_token)),
            Some(SyntaxElement::Node(path.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn tw_custom_variant_at_rule(
    custom_variant_token: SyntaxToken,
    name: CssIdentifier,
    selector: AnyTwCustomVariantSelector,
) -> TwCustomVariantAtRule {
    TwCustomVariantAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_CUSTOM_VARIANT_AT_RULE,
        [
            Some(SyntaxElement::Token(custom_variant_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(selector.into_syntax())),
        ],
    ))
}
pub fn tw_custom_variant_shorthand(
    l_paren_token: SyntaxToken,
    selector: AnyTwCustomVariantShorthand,
    r_paren_token: SyntaxToken,
    semicolon_token: SyntaxToken,
) -> TwCustomVariantShorthand {
    TwCustomVariantShorthand::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_CUSTOM_VARIANT_SHORTHAND,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn tw_functional_utility_name(
    identifier: CssIdentifier,
    minus_token: SyntaxToken,
    star_token: SyntaxToken,
) -> TwFunctionalUtilityName {
    TwFunctionalUtilityName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_FUNCTIONAL_UTILITY_NAME,
        [
            Some(SyntaxElement::Node(identifier.into_syntax())),
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Token(star_token)),
        ],
    ))
}
pub fn tw_plugin_at_rule(plugin_token: SyntaxToken, name: CssString) -> TwPluginAtRuleBuilder {
    TwPluginAtRuleBuilder {
        plugin_token,
        name,
        block: None,
        semicolon_token: None,
    }
}
pub struct TwPluginAtRuleBuilder {
    plugin_token: SyntaxToken,
    name: CssString,
    block: Option<AnyCssDeclarationBlock>,
    semicolon_token: Option<SyntaxToken>,
}
impl TwPluginAtRuleBuilder {
    pub fn with_block(mut self, block: AnyCssDeclarationBlock) -> Self {
        self.block = Some(block);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TwPluginAtRule {
        TwPluginAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::TW_PLUGIN_AT_RULE,
            [
                Some(SyntaxElement::Token(self.plugin_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.block
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn tw_reference_at_rule(
    reference_token: SyntaxToken,
    path: CssString,
    semicolon_token: SyntaxToken,
) -> TwReferenceAtRule {
    TwReferenceAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_REFERENCE_AT_RULE,
        [
            Some(SyntaxElement::Token(reference_token)),
            Some(SyntaxElement::Node(path.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn tw_slot_at_rule(slot_token: SyntaxToken, semicolon_token: SyntaxToken) -> TwSlotAtRule {
    TwSlotAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_SLOT_AT_RULE,
        [
            Some(SyntaxElement::Token(slot_token)),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn tw_source_at_rule(
    source_token: SyntaxToken,
    source: AnyTwSource,
    semicolon_token: SyntaxToken,
) -> TwSourceAtRuleBuilder {
    TwSourceAtRuleBuilder {
        source_token,
        source,
        semicolon_token,
        not_token: None,
    }
}
pub struct TwSourceAtRuleBuilder {
    source_token: SyntaxToken,
    source: AnyTwSource,
    semicolon_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl TwSourceAtRuleBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> TwSourceAtRule {
        TwSourceAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::TW_SOURCE_AT_RULE,
            [
                Some(SyntaxElement::Token(self.source_token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn tw_source_inline(
    inline_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    content: CssString,
    r_paren_token: SyntaxToken,
) -> TwSourceInline {
    TwSourceInline::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_SOURCE_INLINE,
        [
            Some(SyntaxElement::Token(inline_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn tw_theme_at_rule(
    theme_token: SyntaxToken,
    block: AnyCssDeclarationOrRuleBlock,
) -> TwThemeAtRuleBuilder {
    TwThemeAtRuleBuilder {
        theme_token,
        block,
        name: None,
    }
}
pub struct TwThemeAtRuleBuilder {
    theme_token: SyntaxToken,
    block: AnyCssDeclarationOrRuleBlock,
    name: Option<CssIdentifier>,
}
impl TwThemeAtRuleBuilder {
    pub fn with_name(mut self, name: CssIdentifier) -> Self {
        self.name = Some(name);
        self
    }
    pub fn build(self) -> TwThemeAtRule {
        TwThemeAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::TW_THEME_AT_RULE,
            [
                Some(SyntaxElement::Token(self.theme_token)),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.block.into_syntax())),
            ],
        ))
    }
}
pub fn tw_utility_at_rule(
    utility_token: SyntaxToken,
    name: AnyTwUtilityName,
    block: AnyCssDeclarationOrRuleBlock,
) -> TwUtilityAtRule {
    TwUtilityAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_UTILITY_AT_RULE,
        [
            Some(SyntaxElement::Token(utility_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn tw_value_theme_reference(
    reference: CssDashedIdentifier,
    star_token: SyntaxToken,
) -> TwValueThemeReferenceBuilder {
    TwValueThemeReferenceBuilder {
        reference,
        star_token,
        minus_token: None,
    }
}
pub struct TwValueThemeReferenceBuilder {
    reference: CssDashedIdentifier,
    star_token: SyntaxToken,
    minus_token: Option<SyntaxToken>,
}
impl TwValueThemeReferenceBuilder {
    pub fn with_minus_token(mut self, minus_token: SyntaxToken) -> Self {
        self.minus_token = Some(minus_token);
        self
    }
    pub fn build(self) -> TwValueThemeReference {
        TwValueThemeReference::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::TW_VALUE_THEME_REFERENCE,
            [
                Some(SyntaxElement::Node(self.reference.into_syntax())),
                self.minus_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.star_token)),
            ],
        ))
    }
}
pub fn tw_variant_at_rule(
    variant_token: SyntaxToken,
    name: CssIdentifier,
    block: AnyCssDeclarationOrRuleBlock,
) -> TwVariantAtRule {
    TwVariantAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_VARIANT_AT_RULE,
        [
            Some(SyntaxElement::Token(variant_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_attr_name_list<I, S>(items: I, separators: S) -> CssAttrNameList
where
    I: IntoIterator<Item = AnyCssAttrName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssAttrNameList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTR_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_bracketed_value_list<I>(items: I) -> CssBracketedValueList
where
    I: IntoIterator<Item = AnyCssBracketedValueItem>,
    I::IntoIter: ExactSizeIterator,
{
    CssBracketedValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BRACKETED_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_component_value_list<I>(items: I) -> CssComponentValueList
where
    I: IntoIterator<Item = AnyCssValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_composes_class_list<I>(items: I) -> CssComposesClassList
where
    I: IntoIterator<Item = CssCustomIdentifier>,
    I::IntoIter: ExactSizeIterator,
{
    CssComposesClassList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPOSES_CLASS_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_compound_selector_list<I, S>(items: I, separators: S) -> CssCompoundSelectorList
where
    I: IntoIterator<Item = AnyCssCompoundSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssCompoundSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPOUND_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_custom_identifier_comma_separated_list<I, S>(
    items: I,
    separators: S,
) -> CssCustomIdentifierCommaSeparatedList
where
    I: IntoIterator<Item = AnyCssCustomIdentifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssCustomIdentifierCommaSeparatedList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CUSTOM_IDENTIFIER_COMMA_SEPARATED_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_custom_identifier_space_separated_list<I>(
    items: I,
) -> CssCustomIdentifierSpaceSeparatedList
where
    I: IntoIterator<Item = AnyCssCustomIdentifier>,
    I::IntoIter: ExactSizeIterator,
{
    CssCustomIdentifierSpaceSeparatedList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CUSTOM_IDENTIFIER_SPACE_SEPARATED_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_declaration_list<I>(items: I) -> CssDeclarationList
where
    I: IntoIterator<Item = AnyCssDeclaration>,
    I::IntoIter: ExactSizeIterator,
{
    CssDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_declaration_or_at_rule_list<I>(items: I) -> CssDeclarationOrAtRuleList
where
    I: IntoIterator<Item = AnyCssDeclarationOrAtRule>,
    I::IntoIter: ExactSizeIterator,
{
    CssDeclarationOrAtRuleList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_declaration_or_rule_list<I>(items: I) -> CssDeclarationOrRuleList
where
    I: IntoIterator<Item = AnyCssDeclarationOrRule>,
    I::IntoIter: ExactSizeIterator,
{
    CssDeclarationOrRuleList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_document_matcher_list<I, S>(items: I, separators: S) -> CssDocumentMatcherList
where
    I: IntoIterator<Item = AnyCssDocumentMatcher>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssDocumentMatcherList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DOCUMENT_MATCHER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_font_family_name_list<I, S>(items: I, separators: S) -> CssFontFamilyNameList
where
    I: IntoIterator<Item = AnyCssFontFamilyName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssFontFamilyNameList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FAMILY_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_font_feature_values_item_list<I>(items: I) -> CssFontFeatureValuesItemList
where
    I: IntoIterator<Item = AnyCssFontFeatureValuesItem>,
    I::IntoIter: ExactSizeIterator,
{
    CssFontFeatureValuesItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_function_parameter_list<I, S>(items: I, separators: S) -> CssFunctionParameterList
where
    I: IntoIterator<Item = AnyCssFunctionParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssFunctionParameterList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FUNCTION_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_generic_component_value_list<I>(items: I) -> CssGenericComponentValueList
where
    I: IntoIterator<Item = AnyCssGenericComponentValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssGenericComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_if_branch_list<I, S>(items: I, separators: S) -> CssIfBranchList
where
    I: IntoIterator<Item = AnyCssIfBranch>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssIfBranchList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IF_BRANCH_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_keyframes_item_list<I>(items: I) -> CssKeyframesItemList
where
    I: IntoIterator<Item = AnyCssKeyframesItem>,
    I::IntoIter: ExactSizeIterator,
{
    CssKeyframesItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_keyframes_selector_list<I, S>(items: I, separators: S) -> CssKeyframesSelectorList
where
    I: IntoIterator<Item = AnyCssKeyframesSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssKeyframesSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_layer_name_list<I, S>(items: I, separators: S) -> CssLayerNameList
where
    I: IntoIterator<Item = CssIdentifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssLayerNameList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LAYER_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_layer_reference_list<I, S>(items: I, separators: S) -> CssLayerReferenceList
where
    I: IntoIterator<Item = CssLayerNameList>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssLayerReferenceList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_LAYER_REFERENCE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_media_query_list<I, S>(items: I, separators: S) -> CssMediaQueryList
where
    I: IntoIterator<Item = AnyCssMediaQuery>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssMediaQueryList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_nested_selector_list<I>(items: I) -> CssNestedSelectorList
where
    I: IntoIterator<Item = CssNestedSelector>,
    I::IntoIter: ExactSizeIterator,
{
    CssNestedSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NESTED_SELECTOR_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_page_at_rule_item_list<I>(items: I) -> CssPageAtRuleItemList
where
    I: IntoIterator<Item = AnyCssPageAtRuleItem>,
    I::IntoIter: ExactSizeIterator,
{
    CssPageAtRuleItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_AT_RULE_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_page_selector_list<I, S>(items: I, separators: S) -> CssPageSelectorList
where
    I: IntoIterator<Item = AnyCssPageSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssPageSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_page_selector_pseudo_list<I>(items: I) -> CssPageSelectorPseudoList
where
    I: IntoIterator<Item = AnyCssPageSelectorPseudo>,
    I::IntoIter: ExactSizeIterator,
{
    CssPageSelectorPseudoList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_parameter_list<I, S>(items: I, separators: S) -> CssParameterList
where
    I: IntoIterator<Item = CssParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssParameterList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_pseudo_element_function_parameter_list<I>(
    items: I,
) -> CssPseudoElementFunctionParameterList
where
    I: IntoIterator<Item = CssIdentifier>,
    I::IntoIter: ExactSizeIterator,
{
    CssPseudoElementFunctionParameterList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_PARAMETER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_pseudo_value_list<I, S>(items: I, separators: S) -> CssPseudoValueList
where
    I: IntoIterator<Item = AnyCssPseudoValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssPseudoValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_VALUE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_relative_selector_list<I, S>(items: I, separators: S) -> CssRelativeSelectorList
where
    I: IntoIterator<Item = AnyCssRelativeSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssRelativeSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RELATIVE_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_root_item_list<I>(items: I) -> CssRootItemList
where
    I: IntoIterator<Item = AnyCssRootItem>,
    I::IntoIter: ExactSizeIterator,
{
    CssRootItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ROOT_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_rule_list<I>(items: I) -> CssRuleList
where
    I: IntoIterator<Item = AnyCssRule>,
    I::IntoIter: ExactSizeIterator,
{
    CssRuleList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RULE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_selector_list<I, S>(items: I, separators: S) -> CssSelectorList
where
    I: IntoIterator<Item = AnyCssSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_sub_selector_list<I>(items: I) -> CssSubSelectorList
where
    I: IntoIterator<Item = AnyCssSubSelector>,
    I::IntoIter: ExactSizeIterator,
{
    CssSubSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUB_SELECTOR_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_syntax_component_list<I, S>(items: I, separators: S) -> CssSyntaxComponentList
where
    I: IntoIterator<Item = AnyCssSyntaxComponent>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssSyntaxComponentList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SYNTAX_COMPONENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_url_modifier_list<I>(items: I) -> CssUrlModifierList
where
    I: IntoIterator<Item = AnyCssUrlModifier>,
    I::IntoIter: ExactSizeIterator,
{
    CssUrlModifierList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_URL_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_value_at_rule_import_specifier_list<I, S>(
    items: I,
    separators: S,
) -> CssValueAtRuleImportSpecifierList
where
    I: IntoIterator<Item = AnyCssValueAtRuleImportSpecifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssValueAtRuleImportSpecifierList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_SPECIFIER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_value_at_rule_property_list<I, S>(items: I, separators: S) -> CssValueAtRulePropertyList
where
    I: IntoIterator<Item = AnyCssValueAtRuleProperty>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssValueAtRulePropertyList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_PROPERTY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn scss_expression_item_list<I>(items: I) -> ScssExpressionItemList
where
    I: IntoIterator<Item = AnyScssExpressionItem>,
    I::IntoIter: ExactSizeIterator,
{
    ScssExpressionItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_EXPRESSION_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn scss_list_expression_element_list<I, S>(
    items: I,
    separators: S,
) -> ScssListExpressionElementList
where
    I: IntoIterator<Item = ScssListExpressionElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    ScssListExpressionElementList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_LIST_EXPRESSION_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn scss_map_expression_pair_list<I, S>(items: I, separators: S) -> ScssMapExpressionPairList
where
    I: IntoIterator<Item = ScssMapExpressionPair>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    ScssMapExpressionPairList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_MAP_EXPRESSION_PAIR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn scss_variable_modifier_list<I>(items: I) -> ScssVariableModifierList
where
    I: IntoIterator<Item = ScssVariableModifier>,
    I::IntoIter: ExactSizeIterator,
{
    ScssVariableModifierList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::SCSS_VARIABLE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn tw_apply_class_list<I>(items: I) -> TwApplyClassList
where
    I: IntoIterator<Item = CssIdentifier>,
    I::IntoIter: ExactSizeIterator,
{
    TwApplyClassList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::TW_APPLY_CLASS_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_bogus<I>(slots: I) -> CssBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogus::unwrap_cast(SyntaxNode::new_detached(CssSyntaxKind::CSS_BOGUS, slots))
}
pub fn css_bogus_at_rule<I>(slots: I) -> CssBogusAtRule
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_AT_RULE,
        slots,
    ))
}
pub fn css_bogus_attr_name<I>(slots: I) -> CssBogusAttrName
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusAttrName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_ATTR_NAME,
        slots,
    ))
}
pub fn css_bogus_block<I>(slots: I) -> CssBogusBlock
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_BLOCK,
        slots,
    ))
}
pub fn css_bogus_custom_identifier<I>(slots: I) -> CssBogusCustomIdentifier
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusCustomIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_CUSTOM_IDENTIFIER,
        slots,
    ))
}
pub fn css_bogus_declaration_item<I>(slots: I) -> CssBogusDeclarationItem
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusDeclarationItem::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_DECLARATION_ITEM,
        slots,
    ))
}
pub fn css_bogus_document_matcher<I>(slots: I) -> CssBogusDocumentMatcher
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusDocumentMatcher::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_DOCUMENT_MATCHER,
        slots,
    ))
}
pub fn css_bogus_font_family_name<I>(slots: I) -> CssBogusFontFamilyName
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusFontFamilyName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_FONT_FAMILY_NAME,
        slots,
    ))
}
pub fn css_bogus_font_feature_values_item<I>(slots: I) -> CssBogusFontFeatureValuesItem
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusFontFeatureValuesItem::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_FONT_FEATURE_VALUES_ITEM,
        slots,
    ))
}
pub fn css_bogus_function_parameter<I>(slots: I) -> CssBogusFunctionParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusFunctionParameter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_FUNCTION_PARAMETER,
        slots,
    ))
}
pub fn css_bogus_if_branch<I>(slots: I) -> CssBogusIfBranch
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusIfBranch::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_IF_BRANCH,
        slots,
    ))
}
pub fn css_bogus_if_test<I>(slots: I) -> CssBogusIfTest
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusIfTest::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_IF_TEST,
        slots,
    ))
}
pub fn css_bogus_if_test_boolean_expr<I>(slots: I) -> CssBogusIfTestBooleanExpr
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusIfTestBooleanExpr::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_IF_TEST_BOOLEAN_EXPR,
        slots,
    ))
}
pub fn css_bogus_keyframes_item<I>(slots: I) -> CssBogusKeyframesItem
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusKeyframesItem::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_KEYFRAMES_ITEM,
        slots,
    ))
}
pub fn css_bogus_keyframes_name<I>(slots: I) -> CssBogusKeyframesName
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusKeyframesName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_KEYFRAMES_NAME,
        slots,
    ))
}
pub fn css_bogus_layer<I>(slots: I) -> CssBogusLayer
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusLayer::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_LAYER,
        slots,
    ))
}
pub fn css_bogus_media_query<I>(slots: I) -> CssBogusMediaQuery
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusMediaQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_MEDIA_QUERY,
        slots,
    ))
}
pub fn css_bogus_page_selector_pseudo<I>(slots: I) -> CssBogusPageSelectorPseudo
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPageSelectorPseudo::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PAGE_SELECTOR_PSEUDO,
        slots,
    ))
}
pub fn css_bogus_parameter<I>(slots: I) -> CssBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn css_bogus_property<I>(slots: I) -> CssBogusProperty
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PROPERTY,
        slots,
    ))
}
pub fn css_bogus_property_value<I>(slots: I) -> CssBogusPropertyValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPropertyValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PROPERTY_VALUE,
        slots,
    ))
}
pub fn css_bogus_pseudo_class<I>(slots: I) -> CssBogusPseudoClass
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPseudoClass::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PSEUDO_CLASS,
        slots,
    ))
}
pub fn css_bogus_pseudo_element<I>(slots: I) -> CssBogusPseudoElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPseudoElement::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PSEUDO_ELEMENT,
        slots,
    ))
}
pub fn css_bogus_rule<I>(slots: I) -> CssBogusRule
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_RULE,
        slots,
    ))
}
pub fn css_bogus_scope_range<I>(slots: I) -> CssBogusScopeRange
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusScopeRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SCOPE_RANGE,
        slots,
    ))
}
pub fn css_bogus_selector<I>(slots: I) -> CssBogusSelector
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SELECTOR,
        slots,
    ))
}
pub fn css_bogus_sub_selector<I>(slots: I) -> CssBogusSubSelector
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSubSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SUB_SELECTOR,
        slots,
    ))
}
pub fn css_bogus_supports_condition<I>(slots: I) -> CssBogusSupportsCondition
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSupportsCondition::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SUPPORTS_CONDITION,
        slots,
    ))
}
pub fn css_bogus_syntax<I>(slots: I) -> CssBogusSyntax
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSyntax::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SYNTAX,
        slots,
    ))
}
pub fn css_bogus_syntax_single_component<I>(slots: I) -> CssBogusSyntaxSingleComponent
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSyntaxSingleComponent::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SYNTAX_SINGLE_COMPONENT,
        slots,
    ))
}
pub fn css_bogus_type<I>(slots: I) -> CssBogusType
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusType::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_TYPE,
        slots,
    ))
}
pub fn css_bogus_unicode_range_value<I>(slots: I) -> CssBogusUnicodeRangeValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusUnicodeRangeValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_UNICODE_RANGE_VALUE,
        slots,
    ))
}
pub fn css_bogus_url_modifier<I>(slots: I) -> CssBogusUrlModifier
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusUrlModifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_URL_MODIFIER,
        slots,
    ))
}
pub fn css_unknown_at_rule_component_list<I>(slots: I) -> CssUnknownAtRuleComponentList
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssUnknownAtRuleComponentList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNKNOWN_AT_RULE_COMPONENT_LIST,
        slots,
    ))
}
pub fn css_value_at_rule_generic_value<I>(slots: I) -> CssValueAtRuleGenericValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssValueAtRuleGenericValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VALUE_AT_RULE_GENERIC_VALUE,
        slots,
    ))
}
