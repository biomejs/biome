//! Generated file, do not edit by hand, see `xtask/codegen`

//! Maps GritQL pattern names to Biome's internal syntax kinds.
use biome_css_syntax as lang;
use biome_rowan::AstNode;
use lang::CssSyntaxKind;

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<CssSyntaxKind> {
    match node_name {
        // Native Biome AST patterns
        "CssAtRule" => lang::CssAtRule::KIND_SET.iter().next(),
        "CssAttributeMatcher" => lang::CssAttributeMatcher::KIND_SET.iter().next(),
        "CssAttributeMatcherValue" => lang::CssAttributeMatcherValue::KIND_SET.iter().next(),
        "CssAttributeName" => lang::CssAttributeName::KIND_SET.iter().next(),
        "CssAttributeSelector" => lang::CssAttributeSelector::KIND_SET.iter().next(),
        "CssBinaryExpression" => lang::CssBinaryExpression::KIND_SET.iter().next(),
        "CssBracketedValue" => lang::CssBracketedValue::KIND_SET.iter().next(),
        "CssCharsetAtRule" => lang::CssCharsetAtRule::KIND_SET.iter().next(),
        "CssClassSelector" => lang::CssClassSelector::KIND_SET.iter().next(),
        "CssColor" => lang::CssColor::KIND_SET.iter().next(),
        "CssColorProfileAtRule" => lang::CssColorProfileAtRule::KIND_SET.iter().next(),
        "CssComplexSelector" => lang::CssComplexSelector::KIND_SET.iter().next(),
        "CssComposesImportSpecifier" => lang::CssComposesImportSpecifier::KIND_SET.iter().next(),
        "CssComposesProperty" => lang::CssComposesProperty::KIND_SET.iter().next(),
        "CssComposesPropertyValue" => lang::CssComposesPropertyValue::KIND_SET.iter().next(),
        "CssCompoundSelector" => lang::CssCompoundSelector::KIND_SET.iter().next(),
        "CssContainerAndQuery" => lang::CssContainerAndQuery::KIND_SET.iter().next(),
        "CssContainerAtRule" => lang::CssContainerAtRule::KIND_SET.iter().next(),
        "CssContainerNotQuery" => lang::CssContainerNotQuery::KIND_SET.iter().next(),
        "CssContainerOrQuery" => lang::CssContainerOrQuery::KIND_SET.iter().next(),
        "CssContainerQueryInParens" => lang::CssContainerQueryInParens::KIND_SET.iter().next(),
        "CssContainerSizeFeatureInParens" => lang::CssContainerSizeFeatureInParens::KIND_SET
            .iter()
            .next(),
        "CssContainerStyleAndQuery" => lang::CssContainerStyleAndQuery::KIND_SET.iter().next(),
        "CssContainerStyleInParens" => lang::CssContainerStyleInParens::KIND_SET.iter().next(),
        "CssContainerStyleNotQuery" => lang::CssContainerStyleNotQuery::KIND_SET.iter().next(),
        "CssContainerStyleOrQuery" => lang::CssContainerStyleOrQuery::KIND_SET.iter().next(),
        "CssContainerStyleQueryInParens" => {
            lang::CssContainerStyleQueryInParens::KIND_SET.iter().next()
        }
        "CssCounterStyleAtRule" => lang::CssCounterStyleAtRule::KIND_SET.iter().next(),
        "CssCustomIdentifier" => lang::CssCustomIdentifier::KIND_SET.iter().next(),
        "CssDashedIdentifier" => lang::CssDashedIdentifier::KIND_SET.iter().next(),
        "CssDeclaration" => lang::CssDeclaration::KIND_SET.iter().next(),
        "CssDeclarationBlock" => lang::CssDeclarationBlock::KIND_SET.iter().next(),
        "CssDeclarationImportant" => lang::CssDeclarationImportant::KIND_SET.iter().next(),
        "CssDeclarationOrAtRuleBlock" => lang::CssDeclarationOrAtRuleBlock::KIND_SET.iter().next(),
        "CssDeclarationOrRuleBlock" => lang::CssDeclarationOrRuleBlock::KIND_SET.iter().next(),
        "CssDeclarationWithSemicolon" => lang::CssDeclarationWithSemicolon::KIND_SET.iter().next(),
        "CssDocumentAtRule" => lang::CssDocumentAtRule::KIND_SET.iter().next(),
        "CssDocumentCustomMatcher" => lang::CssDocumentCustomMatcher::KIND_SET.iter().next(),
        "CssEmptyDeclaration" => lang::CssEmptyDeclaration::KIND_SET.iter().next(),
        "CssFontFaceAtRule" => lang::CssFontFaceAtRule::KIND_SET.iter().next(),
        "CssFontFamilyName" => lang::CssFontFamilyName::KIND_SET.iter().next(),
        "CssFontFeatureValuesAtRule" => lang::CssFontFeatureValuesAtRule::KIND_SET.iter().next(),
        "CssFontFeatureValuesBlock" => lang::CssFontFeatureValuesBlock::KIND_SET.iter().next(),
        "CssFontFeatureValuesItem" => lang::CssFontFeatureValuesItem::KIND_SET.iter().next(),
        "CssFontPaletteValuesAtRule" => lang::CssFontPaletteValuesAtRule::KIND_SET.iter().next(),
        "CssFunction" => lang::CssFunction::KIND_SET.iter().next(),
        "CssGenericDelimiter" => lang::CssGenericDelimiter::KIND_SET.iter().next(),
        "CssGenericProperty" => lang::CssGenericProperty::KIND_SET.iter().next(),
        "CssIdSelector" => lang::CssIdSelector::KIND_SET.iter().next(),
        "CssIdentifier" => lang::CssIdentifier::KIND_SET.iter().next(),
        "CssImportAnonymousLayer" => lang::CssImportAnonymousLayer::KIND_SET.iter().next(),
        "CssImportAtRule" => lang::CssImportAtRule::KIND_SET.iter().next(),
        "CssImportNamedLayer" => lang::CssImportNamedLayer::KIND_SET.iter().next(),
        "CssImportSupports" => lang::CssImportSupports::KIND_SET.iter().next(),
        "CssKeyframesAtRule" => lang::CssKeyframesAtRule::KIND_SET.iter().next(),
        "CssKeyframesBlock" => lang::CssKeyframesBlock::KIND_SET.iter().next(),
        "CssKeyframesIdentSelector" => lang::CssKeyframesIdentSelector::KIND_SET.iter().next(),
        "CssKeyframesItem" => lang::CssKeyframesItem::KIND_SET.iter().next(),
        "CssKeyframesPercentageSelector" => {
            lang::CssKeyframesPercentageSelector::KIND_SET.iter().next()
        }
        "CssKeyframesScopeFunction" => lang::CssKeyframesScopeFunction::KIND_SET.iter().next(),
        "CssKeyframesScopePrefix" => lang::CssKeyframesScopePrefix::KIND_SET.iter().next(),
        "CssKeyframesScopedName" => lang::CssKeyframesScopedName::KIND_SET.iter().next(),
        "CssLayerAtRule" => lang::CssLayerAtRule::KIND_SET.iter().next(),
        "CssLayerDeclaration" => lang::CssLayerDeclaration::KIND_SET.iter().next(),
        "CssLayerReference" => lang::CssLayerReference::KIND_SET.iter().next(),
        "CssMarginAtRule" => lang::CssMarginAtRule::KIND_SET.iter().next(),
        "CssMediaAndCondition" => lang::CssMediaAndCondition::KIND_SET.iter().next(),
        "CssMediaAndTypeQuery" => lang::CssMediaAndTypeQuery::KIND_SET.iter().next(),
        "CssMediaAtRule" => lang::CssMediaAtRule::KIND_SET.iter().next(),
        "CssMediaConditionInParens" => lang::CssMediaConditionInParens::KIND_SET.iter().next(),
        "CssMediaConditionQuery" => lang::CssMediaConditionQuery::KIND_SET.iter().next(),
        "CssMediaFeatureInParens" => lang::CssMediaFeatureInParens::KIND_SET.iter().next(),
        "CssMediaNotCondition" => lang::CssMediaNotCondition::KIND_SET.iter().next(),
        "CssMediaOrCondition" => lang::CssMediaOrCondition::KIND_SET.iter().next(),
        "CssMediaType" => lang::CssMediaType::KIND_SET.iter().next(),
        "CssMediaTypeQuery" => lang::CssMediaTypeQuery::KIND_SET.iter().next(),
        "CssMetavariable" => lang::CssMetavariable::KIND_SET.iter().next(),
        "CssNamedNamespacePrefix" => lang::CssNamedNamespacePrefix::KIND_SET.iter().next(),
        "CssNamespace" => lang::CssNamespace::KIND_SET.iter().next(),
        "CssNamespaceAtRule" => lang::CssNamespaceAtRule::KIND_SET.iter().next(),
        "CssNestedQualifiedRule" => lang::CssNestedQualifiedRule::KIND_SET.iter().next(),
        "CssNestedSelector" => lang::CssNestedSelector::KIND_SET.iter().next(),
        "CssNthOffset" => lang::CssNthOffset::KIND_SET.iter().next(),
        "CssNumber" => lang::CssNumber::KIND_SET.iter().next(),
        "CssPageAtRule" => lang::CssPageAtRule::KIND_SET.iter().next(),
        "CssPageAtRuleBlock" => lang::CssPageAtRuleBlock::KIND_SET.iter().next(),
        "CssPageSelector" => lang::CssPageSelector::KIND_SET.iter().next(),
        "CssPageSelectorPseudo" => lang::CssPageSelectorPseudo::KIND_SET.iter().next(),
        "CssParameter" => lang::CssParameter::KIND_SET.iter().next(),
        "CssParenthesizedExpression" => lang::CssParenthesizedExpression::KIND_SET.iter().next(),
        "CssPercentage" => lang::CssPercentage::KIND_SET.iter().next(),
        "CssPositionTryAtRule" => lang::CssPositionTryAtRule::KIND_SET.iter().next(),
        "CssPropertyAtRule" => lang::CssPropertyAtRule::KIND_SET.iter().next(),
        "CssPseudoClassFunctionCompoundSelector" => {
            lang::CssPseudoClassFunctionCompoundSelector::KIND_SET
                .iter()
                .next()
        }
        "CssPseudoClassFunctionCustomIdentifier" => {
            lang::CssPseudoClassFunctionCustomIdentifier::KIND_SET
                .iter()
                .next()
        }
        "CssPseudoClassFunctionIdentifier" => lang::CssPseudoClassFunctionIdentifier::KIND_SET
            .iter()
            .next(),
        "CssPseudoClassFunctionNth" => lang::CssPseudoClassFunctionNth::KIND_SET.iter().next(),
        "CssPseudoClassFunctionSelector" => {
            lang::CssPseudoClassFunctionSelector::KIND_SET.iter().next()
        }
        "CssPseudoClassIdentifier" => lang::CssPseudoClassIdentifier::KIND_SET.iter().next(),
        "CssPseudoClassNth" => lang::CssPseudoClassNth::KIND_SET.iter().next(),
        "CssPseudoClassNthIdentifier" => lang::CssPseudoClassNthIdentifier::KIND_SET.iter().next(),
        "CssPseudoClassNthNumber" => lang::CssPseudoClassNthNumber::KIND_SET.iter().next(),
        "CssPseudoClassNthSelector" => lang::CssPseudoClassNthSelector::KIND_SET.iter().next(),
        "CssPseudoClassOfNthSelector" => lang::CssPseudoClassOfNthSelector::KIND_SET.iter().next(),
        "CssPseudoClassSelector" => lang::CssPseudoClassSelector::KIND_SET.iter().next(),
        "CssPseudoElementFunction" => lang::CssPseudoElementFunction::KIND_SET.iter().next(),
        "CssPseudoElementFunctionCustomIdentifier" => {
            lang::CssPseudoElementFunctionCustomIdentifier::KIND_SET
                .iter()
                .next()
        }
        "CssPseudoElementFunctionSelector" => lang::CssPseudoElementFunctionSelector::KIND_SET
            .iter()
            .next(),
        "CssPseudoElementIdentifier" => lang::CssPseudoElementIdentifier::KIND_SET.iter().next(),
        "CssPseudoElementSelector" => lang::CssPseudoElementSelector::KIND_SET.iter().next(),
        "CssQualifiedRule" => lang::CssQualifiedRule::KIND_SET.iter().next(),
        "CssQueryFeatureBoolean" => lang::CssQueryFeatureBoolean::KIND_SET.iter().next(),
        "CssQueryFeaturePlain" => lang::CssQueryFeaturePlain::KIND_SET.iter().next(),
        "CssQueryFeatureRange" => lang::CssQueryFeatureRange::KIND_SET.iter().next(),
        "CssQueryFeatureRangeComparison" => {
            lang::CssQueryFeatureRangeComparison::KIND_SET.iter().next()
        }
        "CssQueryFeatureRangeInterval" => {
            lang::CssQueryFeatureRangeInterval::KIND_SET.iter().next()
        }
        "CssQueryFeatureReverseRange" => lang::CssQueryFeatureReverseRange::KIND_SET.iter().next(),
        "CssRatio" => lang::CssRatio::KIND_SET.iter().next(),
        "CssRegularDimension" => lang::CssRegularDimension::KIND_SET.iter().next(),
        "CssRelativeSelector" => lang::CssRelativeSelector::KIND_SET.iter().next(),
        "CssRoot" => lang::CssRoot::KIND_SET.iter().next(),
        "CssRuleBlock" => lang::CssRuleBlock::KIND_SET.iter().next(),
        "CssScopeAtRule" => lang::CssScopeAtRule::KIND_SET.iter().next(),
        "CssScopeEdge" => lang::CssScopeEdge::KIND_SET.iter().next(),
        "CssScopeRangeEnd" => lang::CssScopeRangeEnd::KIND_SET.iter().next(),
        "CssScopeRangeInterval" => lang::CssScopeRangeInterval::KIND_SET.iter().next(),
        "CssScopeRangeStart" => lang::CssScopeRangeStart::KIND_SET.iter().next(),
        "CssStartingStyleAtRule" => lang::CssStartingStyleAtRule::KIND_SET.iter().next(),
        "CssString" => lang::CssString::KIND_SET.iter().next(),
        "CssSupportsAndCondition" => lang::CssSupportsAndCondition::KIND_SET.iter().next(),
        "CssSupportsAtRule" => lang::CssSupportsAtRule::KIND_SET.iter().next(),
        "CssSupportsConditionInParens" => {
            lang::CssSupportsConditionInParens::KIND_SET.iter().next()
        }
        "CssSupportsFeatureDeclaration" => {
            lang::CssSupportsFeatureDeclaration::KIND_SET.iter().next()
        }
        "CssSupportsFeatureSelector" => lang::CssSupportsFeatureSelector::KIND_SET.iter().next(),
        "CssSupportsNotCondition" => lang::CssSupportsNotCondition::KIND_SET.iter().next(),
        "CssSupportsOrCondition" => lang::CssSupportsOrCondition::KIND_SET.iter().next(),
        "CssTypeSelector" => lang::CssTypeSelector::KIND_SET.iter().next(),
        "CssUnicodeCodepoint" => lang::CssUnicodeCodepoint::KIND_SET.iter().next(),
        "CssUnicodeRange" => lang::CssUnicodeRange::KIND_SET.iter().next(),
        "CssUnicodeRangeInterval" => lang::CssUnicodeRangeInterval::KIND_SET.iter().next(),
        "CssUnicodeRangeWildcard" => lang::CssUnicodeRangeWildcard::KIND_SET.iter().next(),
        "CssUniversalNamespacePrefix" => lang::CssUniversalNamespacePrefix::KIND_SET.iter().next(),
        "CssUniversalSelector" => lang::CssUniversalSelector::KIND_SET.iter().next(),
        "CssUnknownBlockAtRule" => lang::CssUnknownBlockAtRule::KIND_SET.iter().next(),
        "CssUnknownDimension" => lang::CssUnknownDimension::KIND_SET.iter().next(),
        "CssUnknownValueAtRule" => lang::CssUnknownValueAtRule::KIND_SET.iter().next(),
        "CssUrlFunction" => lang::CssUrlFunction::KIND_SET.iter().next(),
        "CssUrlValueRaw" => lang::CssUrlValueRaw::KIND_SET.iter().next(),
        "CssValueAtRule" => lang::CssValueAtRule::KIND_SET.iter().next(),
        "CssValueAtRuleDeclarationClause" => lang::CssValueAtRuleDeclarationClause::KIND_SET
            .iter()
            .next(),
        "CssValueAtRuleGenericProperty" => {
            lang::CssValueAtRuleGenericProperty::KIND_SET.iter().next()
        }
        "CssValueAtRuleImportClause" => lang::CssValueAtRuleImportClause::KIND_SET.iter().next(),
        "CssValueAtRuleImportSpecifier" => {
            lang::CssValueAtRuleImportSpecifier::KIND_SET.iter().next()
        }
        "CssValueAtRuleNamedImportSpecifier" => lang::CssValueAtRuleNamedImportSpecifier::KIND_SET
            .iter()
            .next(),
        "CssViewTransitionAtRule" => lang::CssViewTransitionAtRule::KIND_SET.iter().next(),
        "TwApplyAtRule" => lang::TwApplyAtRule::KIND_SET.iter().next(),
        "TwConfigAtRule" => lang::TwConfigAtRule::KIND_SET.iter().next(),
        "TwCustomVariantAtRule" => lang::TwCustomVariantAtRule::KIND_SET.iter().next(),
        "TwCustomVariantShorthand" => lang::TwCustomVariantShorthand::KIND_SET.iter().next(),
        "TwFunctionalUtilityName" => lang::TwFunctionalUtilityName::KIND_SET.iter().next(),
        "TwPluginAtRule" => lang::TwPluginAtRule::KIND_SET.iter().next(),
        "TwReferenceAtRule" => lang::TwReferenceAtRule::KIND_SET.iter().next(),
        "TwSourceAtRule" => lang::TwSourceAtRule::KIND_SET.iter().next(),
        "TwThemeAtRule" => lang::TwThemeAtRule::KIND_SET.iter().next(),
        "TwUtilityAtRule" => lang::TwUtilityAtRule::KIND_SET.iter().next(),
        "TwValueThemeReference" => lang::TwValueThemeReference::KIND_SET.iter().next(),
        "TwVariantAtRule" => lang::TwVariantAtRule::KIND_SET.iter().next(),
        _ => None,
    }
}
