//! Generated file, do not edit by hand, see `xtask/codegen`

//! Maps GritQL pattern names to Biome's internal syntax kinds.
use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_rowan::AstNode;

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<CssSyntaxKind> {
    use CssSyntaxKind::*;
    use biome_css_syntax::*;
    match node_name {
        // Native Biome AST patterns
        "CssAtRule" => CssAtRule::KIND_SET.iter().next(),
        "CssAttributeMatcher" => CssAttributeMatcher::KIND_SET.iter().next(),
        "CssAttributeMatcherValue" => CssAttributeMatcherValue::KIND_SET.iter().next(),
        "CssAttributeName" => CssAttributeName::KIND_SET.iter().next(),
        "CssAttributeSelector" => CssAttributeSelector::KIND_SET.iter().next(),
        "CssBinaryExpression" => CssBinaryExpression::KIND_SET.iter().next(),
        "CssBracketedValue" => CssBracketedValue::KIND_SET.iter().next(),
        "CssCharsetAtRule" => CssCharsetAtRule::KIND_SET.iter().next(),
        "CssClassSelector" => CssClassSelector::KIND_SET.iter().next(),
        "CssColor" => CssColor::KIND_SET.iter().next(),
        "CssColorProfileAtRule" => CssColorProfileAtRule::KIND_SET.iter().next(),
        "CssComplexSelector" => CssComplexSelector::KIND_SET.iter().next(),
        "CssComposesImportSpecifier" => CssComposesImportSpecifier::KIND_SET.iter().next(),
        "CssComposesProperty" => CssComposesProperty::KIND_SET.iter().next(),
        "CssComposesPropertyValue" => CssComposesPropertyValue::KIND_SET.iter().next(),
        "CssCompoundSelector" => CssCompoundSelector::KIND_SET.iter().next(),
        "CssContainerAndQuery" => CssContainerAndQuery::KIND_SET.iter().next(),
        "CssContainerAtRule" => CssContainerAtRule::KIND_SET.iter().next(),
        "CssContainerNotQuery" => CssContainerNotQuery::KIND_SET.iter().next(),
        "CssContainerOrQuery" => CssContainerOrQuery::KIND_SET.iter().next(),
        "CssContainerQueryInParens" => CssContainerQueryInParens::KIND_SET.iter().next(),
        "CssContainerSizeFeatureInParens" => {
            CssContainerSizeFeatureInParens::KIND_SET.iter().next()
        }
        "CssContainerStyleAndQuery" => CssContainerStyleAndQuery::KIND_SET.iter().next(),
        "CssContainerStyleInParens" => CssContainerStyleInParens::KIND_SET.iter().next(),
        "CssContainerStyleNotQuery" => CssContainerStyleNotQuery::KIND_SET.iter().next(),
        "CssContainerStyleOrQuery" => CssContainerStyleOrQuery::KIND_SET.iter().next(),
        "CssContainerStyleQueryInParens" => CssContainerStyleQueryInParens::KIND_SET.iter().next(),
        "CssCounterStyleAtRule" => CssCounterStyleAtRule::KIND_SET.iter().next(),
        "CssCustomIdentifier" => CssCustomIdentifier::KIND_SET.iter().next(),
        "CssDashedIdentifier" => CssDashedIdentifier::KIND_SET.iter().next(),
        "CssDeclaration" => CssDeclaration::KIND_SET.iter().next(),
        "CssDeclarationBlock" => CssDeclarationBlock::KIND_SET.iter().next(),
        "CssDeclarationImportant" => CssDeclarationImportant::KIND_SET.iter().next(),
        "CssDeclarationOrAtRuleBlock" => CssDeclarationOrAtRuleBlock::KIND_SET.iter().next(),
        "CssDeclarationOrRuleBlock" => CssDeclarationOrRuleBlock::KIND_SET.iter().next(),
        "CssDeclarationWithSemicolon" => CssDeclarationWithSemicolon::KIND_SET.iter().next(),
        "CssDocumentAtRule" => CssDocumentAtRule::KIND_SET.iter().next(),
        "CssDocumentCustomMatcher" => CssDocumentCustomMatcher::KIND_SET.iter().next(),
        "CssEmptyDeclaration" => CssEmptyDeclaration::KIND_SET.iter().next(),
        "CssFontFaceAtRule" => CssFontFaceAtRule::KIND_SET.iter().next(),
        "CssFontFamilyName" => CssFontFamilyName::KIND_SET.iter().next(),
        "CssFontFeatureValuesAtRule" => CssFontFeatureValuesAtRule::KIND_SET.iter().next(),
        "CssFontFeatureValuesBlock" => CssFontFeatureValuesBlock::KIND_SET.iter().next(),
        "CssFontFeatureValuesItem" => CssFontFeatureValuesItem::KIND_SET.iter().next(),
        "CssFontPaletteValuesAtRule" => CssFontPaletteValuesAtRule::KIND_SET.iter().next(),
        "CssFunction" => CssFunction::KIND_SET.iter().next(),
        "CssGenericDelimiter" => CssGenericDelimiter::KIND_SET.iter().next(),
        "CssGenericProperty" => CssGenericProperty::KIND_SET.iter().next(),
        "CssIdSelector" => CssIdSelector::KIND_SET.iter().next(),
        "CssIdentifier" => CssIdentifier::KIND_SET.iter().next(),
        "CssImportAnonymousLayer" => CssImportAnonymousLayer::KIND_SET.iter().next(),
        "CssImportAtRule" => CssImportAtRule::KIND_SET.iter().next(),
        "CssImportNamedLayer" => CssImportNamedLayer::KIND_SET.iter().next(),
        "CssImportSupports" => CssImportSupports::KIND_SET.iter().next(),
        "CssKeyframesAtRule" => CssKeyframesAtRule::KIND_SET.iter().next(),
        "CssKeyframesBlock" => CssKeyframesBlock::KIND_SET.iter().next(),
        "CssKeyframesIdentSelector" => CssKeyframesIdentSelector::KIND_SET.iter().next(),
        "CssKeyframesItem" => CssKeyframesItem::KIND_SET.iter().next(),
        "CssKeyframesPercentageSelector" => CssKeyframesPercentageSelector::KIND_SET.iter().next(),
        "CssKeyframesScopeFunction" => CssKeyframesScopeFunction::KIND_SET.iter().next(),
        "CssKeyframesScopePrefix" => CssKeyframesScopePrefix::KIND_SET.iter().next(),
        "CssKeyframesScopedName" => CssKeyframesScopedName::KIND_SET.iter().next(),
        "CssLayerAtRule" => CssLayerAtRule::KIND_SET.iter().next(),
        "CssLayerDeclaration" => CssLayerDeclaration::KIND_SET.iter().next(),
        "CssLayerReference" => CssLayerReference::KIND_SET.iter().next(),
        "CssMarginAtRule" => CssMarginAtRule::KIND_SET.iter().next(),
        "CssMediaAndCondition" => CssMediaAndCondition::KIND_SET.iter().next(),
        "CssMediaAndTypeQuery" => CssMediaAndTypeQuery::KIND_SET.iter().next(),
        "CssMediaAtRule" => CssMediaAtRule::KIND_SET.iter().next(),
        "CssMediaConditionInParens" => CssMediaConditionInParens::KIND_SET.iter().next(),
        "CssMediaConditionQuery" => CssMediaConditionQuery::KIND_SET.iter().next(),
        "CssMediaFeatureInParens" => CssMediaFeatureInParens::KIND_SET.iter().next(),
        "CssMediaNotCondition" => CssMediaNotCondition::KIND_SET.iter().next(),
        "CssMediaOrCondition" => CssMediaOrCondition::KIND_SET.iter().next(),
        "CssMediaType" => CssMediaType::KIND_SET.iter().next(),
        "CssMediaTypeQuery" => CssMediaTypeQuery::KIND_SET.iter().next(),
        "CssMetavariable" => CssMetavariable::KIND_SET.iter().next(),
        "CssNamedNamespacePrefix" => CssNamedNamespacePrefix::KIND_SET.iter().next(),
        "CssNamespace" => CssNamespace::KIND_SET.iter().next(),
        "CssNamespaceAtRule" => CssNamespaceAtRule::KIND_SET.iter().next(),
        "CssNestedQualifiedRule" => CssNestedQualifiedRule::KIND_SET.iter().next(),
        "CssNestedSelector" => CssNestedSelector::KIND_SET.iter().next(),
        "CssNthOffset" => CssNthOffset::KIND_SET.iter().next(),
        "CssNumber" => CssNumber::KIND_SET.iter().next(),
        "CssPageAtRule" => CssPageAtRule::KIND_SET.iter().next(),
        "CssPageAtRuleBlock" => CssPageAtRuleBlock::KIND_SET.iter().next(),
        "CssPageSelector" => CssPageSelector::KIND_SET.iter().next(),
        "CssPageSelectorPseudo" => CssPageSelectorPseudo::KIND_SET.iter().next(),
        "CssParameter" => CssParameter::KIND_SET.iter().next(),
        "CssParenthesizedExpression" => CssParenthesizedExpression::KIND_SET.iter().next(),
        "CssPercentage" => CssPercentage::KIND_SET.iter().next(),
        "CssPositionTryAtRule" => CssPositionTryAtRule::KIND_SET.iter().next(),
        "CssPropertyAtRule" => CssPropertyAtRule::KIND_SET.iter().next(),
        "CssPseudoClassFunctionCompoundSelector" => {
            CssPseudoClassFunctionCompoundSelector::KIND_SET
                .iter()
                .next()
        }
        "CssPseudoClassFunctionIdentifier" => {
            CssPseudoClassFunctionIdentifier::KIND_SET.iter().next()
        }
        "CssPseudoClassFunctionNth" => CssPseudoClassFunctionNth::KIND_SET.iter().next(),
        "CssPseudoClassFunctionSelector" => CssPseudoClassFunctionSelector::KIND_SET.iter().next(),
        "CssPseudoClassIdentifier" => CssPseudoClassIdentifier::KIND_SET.iter().next(),
        "CssPseudoClassNth" => CssPseudoClassNth::KIND_SET.iter().next(),
        "CssPseudoClassNthIdentifier" => CssPseudoClassNthIdentifier::KIND_SET.iter().next(),
        "CssPseudoClassNthNumber" => CssPseudoClassNthNumber::KIND_SET.iter().next(),
        "CssPseudoClassNthSelector" => CssPseudoClassNthSelector::KIND_SET.iter().next(),
        "CssPseudoClassOfNthSelector" => CssPseudoClassOfNthSelector::KIND_SET.iter().next(),
        "CssPseudoClassSelector" => CssPseudoClassSelector::KIND_SET.iter().next(),
        "CssPseudoElementFunction" => CssPseudoElementFunction::KIND_SET.iter().next(),
        "CssPseudoElementFunctionCustomIdentifier" => {
            CssPseudoElementFunctionCustomIdentifier::KIND_SET
                .iter()
                .next()
        }
        "CssPseudoElementFunctionSelector" => {
            CssPseudoElementFunctionSelector::KIND_SET.iter().next()
        }
        "CssPseudoElementIdentifier" => CssPseudoElementIdentifier::KIND_SET.iter().next(),
        "CssPseudoElementSelector" => CssPseudoElementSelector::KIND_SET.iter().next(),
        "CssQualifiedRule" => CssQualifiedRule::KIND_SET.iter().next(),
        "CssQueryFeatureBoolean" => CssQueryFeatureBoolean::KIND_SET.iter().next(),
        "CssQueryFeaturePlain" => CssQueryFeaturePlain::KIND_SET.iter().next(),
        "CssQueryFeatureRange" => CssQueryFeatureRange::KIND_SET.iter().next(),
        "CssQueryFeatureRangeComparison" => CssQueryFeatureRangeComparison::KIND_SET.iter().next(),
        "CssQueryFeatureRangeInterval" => CssQueryFeatureRangeInterval::KIND_SET.iter().next(),
        "CssQueryFeatureReverseRange" => CssQueryFeatureReverseRange::KIND_SET.iter().next(),
        "CssRatio" => CssRatio::KIND_SET.iter().next(),
        "CssRegularDimension" => CssRegularDimension::KIND_SET.iter().next(),
        "CssRelativeSelector" => CssRelativeSelector::KIND_SET.iter().next(),
        "CssRoot" => CssRoot::KIND_SET.iter().next(),
        "CssRuleBlock" => CssRuleBlock::KIND_SET.iter().next(),
        "CssScopeAtRule" => CssScopeAtRule::KIND_SET.iter().next(),
        "CssScopeEdge" => CssScopeEdge::KIND_SET.iter().next(),
        "CssScopeRangeEnd" => CssScopeRangeEnd::KIND_SET.iter().next(),
        "CssScopeRangeInterval" => CssScopeRangeInterval::KIND_SET.iter().next(),
        "CssScopeRangeStart" => CssScopeRangeStart::KIND_SET.iter().next(),
        "CssStartingStyleAtRule" => CssStartingStyleAtRule::KIND_SET.iter().next(),
        "CssString" => CssString::KIND_SET.iter().next(),
        "CssSupportsAndCondition" => CssSupportsAndCondition::KIND_SET.iter().next(),
        "CssSupportsAtRule" => CssSupportsAtRule::KIND_SET.iter().next(),
        "CssSupportsConditionInParens" => CssSupportsConditionInParens::KIND_SET.iter().next(),
        "CssSupportsFeatureDeclaration" => CssSupportsFeatureDeclaration::KIND_SET.iter().next(),
        "CssSupportsFeatureSelector" => CssSupportsFeatureSelector::KIND_SET.iter().next(),
        "CssSupportsNotCondition" => CssSupportsNotCondition::KIND_SET.iter().next(),
        "CssSupportsOrCondition" => CssSupportsOrCondition::KIND_SET.iter().next(),
        "CssTypeSelector" => CssTypeSelector::KIND_SET.iter().next(),
        "CssUnicodeCodepoint" => CssUnicodeCodepoint::KIND_SET.iter().next(),
        "CssUnicodeRange" => CssUnicodeRange::KIND_SET.iter().next(),
        "CssUnicodeRangeInterval" => CssUnicodeRangeInterval::KIND_SET.iter().next(),
        "CssUnicodeRangeWildcard" => CssUnicodeRangeWildcard::KIND_SET.iter().next(),
        "CssUniversalNamespacePrefix" => CssUniversalNamespacePrefix::KIND_SET.iter().next(),
        "CssUniversalSelector" => CssUniversalSelector::KIND_SET.iter().next(),
        "CssUnknownBlockAtRule" => CssUnknownBlockAtRule::KIND_SET.iter().next(),
        "CssUnknownDimension" => CssUnknownDimension::KIND_SET.iter().next(),
        "CssUnknownValueAtRule" => CssUnknownValueAtRule::KIND_SET.iter().next(),
        "CssUrlFunction" => CssUrlFunction::KIND_SET.iter().next(),
        "CssUrlValueRaw" => CssUrlValueRaw::KIND_SET.iter().next(),
        "CssValueAtRule" => CssValueAtRule::KIND_SET.iter().next(),
        "CssValueAtRuleDeclarationClause" => {
            CssValueAtRuleDeclarationClause::KIND_SET.iter().next()
        }
        "CssValueAtRuleGenericProperty" => CssValueAtRuleGenericProperty::KIND_SET.iter().next(),
        "CssValueAtRuleImportClause" => CssValueAtRuleImportClause::KIND_SET.iter().next(),
        "CssValueAtRuleImportSpecifier" => CssValueAtRuleImportSpecifier::KIND_SET.iter().next(),
        "CssValueAtRuleNamedImportSpecifier" => {
            CssValueAtRuleNamedImportSpecifier::KIND_SET.iter().next()
        }
        "CssViewTransitionAtRule" => CssViewTransitionAtRule::KIND_SET.iter().next(),
        _ => None,
    }
}
