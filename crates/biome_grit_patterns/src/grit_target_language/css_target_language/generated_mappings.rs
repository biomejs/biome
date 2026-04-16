//! Generated file, do not edit by hand, see `xtask/codegen`

//! Maps GritQL pattern names to Biome's internal syntax kinds.
use biome_css_syntax as lang;
use biome_rowan::AstNode;
use lang::CssSyntaxKind;

/// Returns the syntax kind for a legacy TreeSitter node name.
pub fn legacy_kind_by_name(_node_name: &str) -> Option<CssSyntaxKind> {
    None
}

/// Returns the syntax kind for a native Biome node name.
pub fn native_kind_by_name(node_name: &str) -> Option<CssSyntaxKind> {
    match node_name {
        // Native Biome AST patterns
        "CssAtRule" => lang::CssAtRule::KIND_SET.iter().next(),
        "CssAtRuleDeclarator" => lang::CssAtRuleDeclarator::KIND_SET.iter().next(),
        "CssAttrFallbackValue" => lang::CssAttrFallbackValue::KIND_SET.iter().next(),
        "CssAttrFunction" => lang::CssAttrFunction::KIND_SET.iter().next(),
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
        "CssColorProfileAtRuleDeclarator" => lang::CssColorProfileAtRuleDeclarator::KIND_SET
            .iter()
            .next(),
        "CssCommaSeparatedValue" => lang::CssCommaSeparatedValue::KIND_SET.iter().next(),
        "CssComplexSelector" => lang::CssComplexSelector::KIND_SET.iter().next(),
        "CssComposesImportSpecifier" => lang::CssComposesImportSpecifier::KIND_SET.iter().next(),
        "CssComposesProperty" => lang::CssComposesProperty::KIND_SET.iter().next(),
        "CssComposesPropertyValue" => lang::CssComposesPropertyValue::KIND_SET.iter().next(),
        "CssCompoundSelector" => lang::CssCompoundSelector::KIND_SET.iter().next(),
        "CssContainerAndQuery" => lang::CssContainerAndQuery::KIND_SET.iter().next(),
        "CssContainerAtRule" => lang::CssContainerAtRule::KIND_SET.iter().next(),
        "CssContainerAtRuleDeclarator" => {
            lang::CssContainerAtRuleDeclarator::KIND_SET.iter().next()
        }
        "CssContainerNotQuery" => lang::CssContainerNotQuery::KIND_SET.iter().next(),
        "CssContainerOrQuery" => lang::CssContainerOrQuery::KIND_SET.iter().next(),
        "CssContainerQueryInParens" => lang::CssContainerQueryInParens::KIND_SET.iter().next(),
        "CssContainerScrollStateAndQuery" => lang::CssContainerScrollStateAndQuery::KIND_SET
            .iter()
            .next(),
        "CssContainerScrollStateInParens" => lang::CssContainerScrollStateInParens::KIND_SET
            .iter()
            .next(),
        "CssContainerScrollStateNotQuery" => lang::CssContainerScrollStateNotQuery::KIND_SET
            .iter()
            .next(),
        "CssContainerScrollStateOrQuery" => {
            lang::CssContainerScrollStateOrQuery::KIND_SET.iter().next()
        }
        "CssContainerScrollStateQueryInParens" => {
            lang::CssContainerScrollStateQueryInParens::KIND_SET
                .iter()
                .next()
        }
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
        "CssCounterStyleAtRuleDeclarator" => lang::CssCounterStyleAtRuleDeclarator::KIND_SET
            .iter()
            .next(),
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
        "CssElseKeyword" => lang::CssElseKeyword::KIND_SET.iter().next(),
        "CssEmptyDeclaration" => lang::CssEmptyDeclaration::KIND_SET.iter().next(),
        "CssFontFaceAtRule" => lang::CssFontFaceAtRule::KIND_SET.iter().next(),
        "CssFontFaceAtRuleDeclarator" => lang::CssFontFaceAtRuleDeclarator::KIND_SET.iter().next(),
        "CssFontFamilyName" => lang::CssFontFamilyName::KIND_SET.iter().next(),
        "CssFontFeatureValuesAtRule" => lang::CssFontFeatureValuesAtRule::KIND_SET.iter().next(),
        "CssFontFeatureValuesBlock" => lang::CssFontFeatureValuesBlock::KIND_SET.iter().next(),
        "CssFontFeatureValuesItem" => lang::CssFontFeatureValuesItem::KIND_SET.iter().next(),
        "CssFontPaletteValuesAtRule" => lang::CssFontPaletteValuesAtRule::KIND_SET.iter().next(),
        "CssFontPaletteValuesAtRuleDeclarator" => {
            lang::CssFontPaletteValuesAtRuleDeclarator::KIND_SET
                .iter()
                .next()
        }
        "CssFunction" => lang::CssFunction::KIND_SET.iter().next(),
        "CssFunctionAtRule" => lang::CssFunctionAtRule::KIND_SET.iter().next(),
        "CssFunctionAtRuleDeclarator" => lang::CssFunctionAtRuleDeclarator::KIND_SET.iter().next(),
        "CssFunctionParameter" => lang::CssFunctionParameter::KIND_SET.iter().next(),
        "CssFunctionParameterDefaultValue" => lang::CssFunctionParameterDefaultValue::KIND_SET
            .iter()
            .next(),
        "CssGenericDelimiter" => lang::CssGenericDelimiter::KIND_SET.iter().next(),
        "CssGenericProperty" => lang::CssGenericProperty::KIND_SET.iter().next(),
        "CssIdSelector" => lang::CssIdSelector::KIND_SET.iter().next(),
        "CssIdentifier" => lang::CssIdentifier::KIND_SET.iter().next(),
        "CssIfBranch" => lang::CssIfBranch::KIND_SET.iter().next(),
        "CssIfFunction" => lang::CssIfFunction::KIND_SET.iter().next(),
        "CssIfMediaTest" => lang::CssIfMediaTest::KIND_SET.iter().next(),
        "CssIfSassTest" => lang::CssIfSassTest::KIND_SET.iter().next(),
        "CssIfStyleTest" => lang::CssIfStyleTest::KIND_SET.iter().next(),
        "CssIfSupportsIdentifierTest" => lang::CssIfSupportsIdentifierTest::KIND_SET.iter().next(),
        "CssIfSupportsTest" => lang::CssIfSupportsTest::KIND_SET.iter().next(),
        "CssIfTestBooleanAndExpr" => lang::CssIfTestBooleanAndExpr::KIND_SET.iter().next(),
        "CssIfTestBooleanExprInParens" => {
            lang::CssIfTestBooleanExprInParens::KIND_SET.iter().next()
        }
        "CssIfTestBooleanNotExpr" => lang::CssIfTestBooleanNotExpr::KIND_SET.iter().next(),
        "CssIfTestBooleanOrExpr" => lang::CssIfTestBooleanOrExpr::KIND_SET.iter().next(),
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
        "CssKeyframesRangeSelector" => lang::CssKeyframesRangeSelector::KIND_SET.iter().next(),
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
        "CssMediaAtRuleDeclarator" => lang::CssMediaAtRuleDeclarator::KIND_SET.iter().next(),
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
        "CssNumberDeclarator" => lang::CssNumberDeclarator::KIND_SET.iter().next(),
        "CssPageAtRule" => lang::CssPageAtRule::KIND_SET.iter().next(),
        "CssPageAtRuleBlock" => lang::CssPageAtRuleBlock::KIND_SET.iter().next(),
        "CssPageSelector" => lang::CssPageSelector::KIND_SET.iter().next(),
        "CssPageSelectorPseudo" => lang::CssPageSelectorPseudo::KIND_SET.iter().next(),
        "CssParenthesizedExpression" => lang::CssParenthesizedExpression::KIND_SET.iter().next(),
        "CssPercentage" => lang::CssPercentage::KIND_SET.iter().next(),
        "CssPositionTryAtRule" => lang::CssPositionTryAtRule::KIND_SET.iter().next(),
        "CssPositionTryAtRuleDeclarator" => {
            lang::CssPositionTryAtRuleDeclarator::KIND_SET.iter().next()
        }
        "CssPropertyAtRule" => lang::CssPropertyAtRule::KIND_SET.iter().next(),
        "CssPropertyAtRuleDeclarator" => lang::CssPropertyAtRuleDeclarator::KIND_SET.iter().next(),
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
        "CssRawStringDeclarator" => lang::CssRawStringDeclarator::KIND_SET.iter().next(),
        "CssRegularAttrUnit" => lang::CssRegularAttrUnit::KIND_SET.iter().next(),
        "CssRegularDimension" => lang::CssRegularDimension::KIND_SET.iter().next(),
        "CssRegularSyntaxTypeName" => lang::CssRegularSyntaxTypeName::KIND_SET.iter().next(),
        "CssRelativeSelector" => lang::CssRelativeSelector::KIND_SET.iter().next(),
        "CssReturnsStatement" => lang::CssReturnsStatement::KIND_SET.iter().next(),
        "CssRoot" => lang::CssRoot::KIND_SET.iter().next(),
        "CssRuleBlock" => lang::CssRuleBlock::KIND_SET.iter().next(),
        "CssScopeAtRule" => lang::CssScopeAtRule::KIND_SET.iter().next(),
        "CssScopeAtRuleDeclarator" => lang::CssScopeAtRuleDeclarator::KIND_SET.iter().next(),
        "CssScopeEdge" => lang::CssScopeEdge::KIND_SET.iter().next(),
        "CssScopeRangeEnd" => lang::CssScopeRangeEnd::KIND_SET.iter().next(),
        "CssScopeRangeInterval" => lang::CssScopeRangeInterval::KIND_SET.iter().next(),
        "CssScopeRangeStart" => lang::CssScopeRangeStart::KIND_SET.iter().next(),
        "CssSnippetRoot" => lang::CssSnippetRoot::KIND_SET.iter().next(),
        "CssStartingStyleAtRule" => lang::CssStartingStyleAtRule::KIND_SET.iter().next(),
        "CssStartingStyleAtRuleDeclarator" => lang::CssStartingStyleAtRuleDeclarator::KIND_SET
            .iter()
            .next(),
        "CssString" => lang::CssString::KIND_SET.iter().next(),
        "CssSupportsAndCondition" => lang::CssSupportsAndCondition::KIND_SET.iter().next(),
        "CssSupportsAtRule" => lang::CssSupportsAtRule::KIND_SET.iter().next(),
        "CssSupportsAtRuleDeclarator" => lang::CssSupportsAtRuleDeclarator::KIND_SET.iter().next(),
        "CssSupportsConditionInParens" => {
            lang::CssSupportsConditionInParens::KIND_SET.iter().next()
        }
        "CssSupportsFeatureDeclaration" => {
            lang::CssSupportsFeatureDeclaration::KIND_SET.iter().next()
        }
        "CssSupportsFeatureSelector" => lang::CssSupportsFeatureSelector::KIND_SET.iter().next(),
        "CssSupportsNotCondition" => lang::CssSupportsNotCondition::KIND_SET.iter().next(),
        "CssSupportsOrCondition" => lang::CssSupportsOrCondition::KIND_SET.iter().next(),
        "CssSyntaxComponent" => lang::CssSyntaxComponent::KIND_SET.iter().next(),
        "CssSyntaxComponentWithoutMultiplier" => {
            lang::CssSyntaxComponentWithoutMultiplier::KIND_SET
                .iter()
                .next()
        }
        "CssSyntaxMultiplier" => lang::CssSyntaxMultiplier::KIND_SET.iter().next(),
        "CssSyntaxType" => lang::CssSyntaxType::KIND_SET.iter().next(),
        "CssTypeFunction" => lang::CssTypeFunction::KIND_SET.iter().next(),
        "CssTypeSelector" => lang::CssTypeSelector::KIND_SET.iter().next(),
        "CssUnaryExpression" => lang::CssUnaryExpression::KIND_SET.iter().next(),
        "CssUnicodeCodepoint" => lang::CssUnicodeCodepoint::KIND_SET.iter().next(),
        "CssUnicodeRange" => lang::CssUnicodeRange::KIND_SET.iter().next(),
        "CssUnicodeRangeInterval" => lang::CssUnicodeRangeInterval::KIND_SET.iter().next(),
        "CssUnicodeRangeWildcard" => lang::CssUnicodeRangeWildcard::KIND_SET.iter().next(),
        "CssUniversalNamespacePrefix" => lang::CssUniversalNamespacePrefix::KIND_SET.iter().next(),
        "CssUniversalSelector" => lang::CssUniversalSelector::KIND_SET.iter().next(),
        "CssUniversalSyntax" => lang::CssUniversalSyntax::KIND_SET.iter().next(),
        "CssUnknownAttrUnit" => lang::CssUnknownAttrUnit::KIND_SET.iter().next(),
        "CssUnknownBlockAtRule" => lang::CssUnknownBlockAtRule::KIND_SET.iter().next(),
        "CssUnknownDimension" => lang::CssUnknownDimension::KIND_SET.iter().next(),
        "CssUnknownSyntaxTypeName" => lang::CssUnknownSyntaxTypeName::KIND_SET.iter().next(),
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
        "CssViewTransitionAtRuleDeclarator" => lang::CssViewTransitionAtRuleDeclarator::KIND_SET
            .iter()
            .next(),
        "ScssArbitraryArgument" => lang::ScssArbitraryArgument::KIND_SET.iter().next(),
        "ScssAtRootAtRule" => lang::ScssAtRootAtRule::KIND_SET.iter().next(),
        "ScssAtRootQuery" => lang::ScssAtRootQuery::KIND_SET.iter().next(),
        "ScssAtRootSelector" => lang::ScssAtRootSelector::KIND_SET.iter().next(),
        "ScssBinaryExpression" => lang::ScssBinaryExpression::KIND_SET.iter().next(),
        "ScssContentAtRule" => lang::ScssContentAtRule::KIND_SET.iter().next(),
        "ScssDebugAtRule" => lang::ScssDebugAtRule::KIND_SET.iter().next(),
        "ScssDeclaration" => lang::ScssDeclaration::KIND_SET.iter().next(),
        "ScssEachAtRule" => lang::ScssEachAtRule::KIND_SET.iter().next(),
        "ScssElseClause" => lang::ScssElseClause::KIND_SET.iter().next(),
        "ScssErrorAtRule" => lang::ScssErrorAtRule::KIND_SET.iter().next(),
        "ScssExpression" => lang::ScssExpression::KIND_SET.iter().next(),
        "ScssExtendAtRule" => lang::ScssExtendAtRule::KIND_SET.iter().next(),
        "ScssExtendOptionalModifier" => lang::ScssExtendOptionalModifier::KIND_SET.iter().next(),
        "ScssForAtRule" => lang::ScssForAtRule::KIND_SET.iter().next(),
        "ScssForwardAsClause" => lang::ScssForwardAsClause::KIND_SET.iter().next(),
        "ScssForwardAtRule" => lang::ScssForwardAtRule::KIND_SET.iter().next(),
        "ScssFunctionAtRule" => lang::ScssFunctionAtRule::KIND_SET.iter().next(),
        "ScssHideClause" => lang::ScssHideClause::KIND_SET.iter().next(),
        "ScssIdentifier" => lang::ScssIdentifier::KIND_SET.iter().next(),
        "ScssIfAtRule" => lang::ScssIfAtRule::KIND_SET.iter().next(),
        "ScssImportAtRule" => lang::ScssImportAtRule::KIND_SET.iter().next(),
        "ScssIncludeAtRule" => lang::ScssIncludeAtRule::KIND_SET.iter().next(),
        "ScssInterpolatedIdentifier" => lang::ScssInterpolatedIdentifier::KIND_SET.iter().next(),
        "ScssInterpolatedIdentifierHyphen" => lang::ScssInterpolatedIdentifierHyphen::KIND_SET
            .iter()
            .next(),
        "ScssInterpolatedString" => lang::ScssInterpolatedString::KIND_SET.iter().next(),
        "ScssInterpolation" => lang::ScssInterpolation::KIND_SET.iter().next(),
        "ScssKeywordArgument" => lang::ScssKeywordArgument::KIND_SET.iter().next(),
        "ScssMapExpression" => lang::ScssMapExpression::KIND_SET.iter().next(),
        "ScssMapExpressionPair" => lang::ScssMapExpressionPair::KIND_SET.iter().next(),
        "ScssMixinAtRule" => lang::ScssMixinAtRule::KIND_SET.iter().next(),
        "ScssModuleConfiguration" => lang::ScssModuleConfiguration::KIND_SET.iter().next(),
        "ScssNamespacedIdentifier" => lang::ScssNamespacedIdentifier::KIND_SET.iter().next(),
        "ScssNestingDeclaration" => lang::ScssNestingDeclaration::KIND_SET.iter().next(),
        "ScssParameter" => lang::ScssParameter::KIND_SET.iter().next(),
        "ScssParameterDefaultValue" => lang::ScssParameterDefaultValue::KIND_SET.iter().next(),
        "ScssParentSelectorValue" => lang::ScssParentSelectorValue::KIND_SET.iter().next(),
        "ScssParenthesizedExpression" => lang::ScssParenthesizedExpression::KIND_SET.iter().next(),
        "ScssPlaceholderSelector" => lang::ScssPlaceholderSelector::KIND_SET.iter().next(),
        "ScssPlainImport" => lang::ScssPlainImport::KIND_SET.iter().next(),
        "ScssQualifiedName" => lang::ScssQualifiedName::KIND_SET.iter().next(),
        "ScssReturnAtRule" => lang::ScssReturnAtRule::KIND_SET.iter().next(),
        "ScssShowClause" => lang::ScssShowClause::KIND_SET.iter().next(),
        "ScssStringText" => lang::ScssStringText::KIND_SET.iter().next(),
        "ScssUnaryExpression" => lang::ScssUnaryExpression::KIND_SET.iter().next(),
        "ScssUseAllNamespace" => lang::ScssUseAllNamespace::KIND_SET.iter().next(),
        "ScssUseAsClause" => lang::ScssUseAsClause::KIND_SET.iter().next(),
        "ScssUseAtRule" => lang::ScssUseAtRule::KIND_SET.iter().next(),
        "ScssVariableModifier" => lang::ScssVariableModifier::KIND_SET.iter().next(),
        "ScssWarnAtRule" => lang::ScssWarnAtRule::KIND_SET.iter().next(),
        "ScssWhileAtRule" => lang::ScssWhileAtRule::KIND_SET.iter().next(),
        "ScssWithClause" => lang::ScssWithClause::KIND_SET.iter().next(),
        "TwApplyAtRule" => lang::TwApplyAtRule::KIND_SET.iter().next(),
        "TwConfigAtRule" => lang::TwConfigAtRule::KIND_SET.iter().next(),
        "TwCustomVariantAtRule" => lang::TwCustomVariantAtRule::KIND_SET.iter().next(),
        "TwCustomVariantShorthand" => lang::TwCustomVariantShorthand::KIND_SET.iter().next(),
        "TwFunctionalUtilityName" => lang::TwFunctionalUtilityName::KIND_SET.iter().next(),
        "TwPluginAtRule" => lang::TwPluginAtRule::KIND_SET.iter().next(),
        "TwReferenceAtRule" => lang::TwReferenceAtRule::KIND_SET.iter().next(),
        "TwSlotAtRule" => lang::TwSlotAtRule::KIND_SET.iter().next(),
        "TwSourceAtRule" => lang::TwSourceAtRule::KIND_SET.iter().next(),
        "TwSourceInline" => lang::TwSourceInline::KIND_SET.iter().next(),
        "TwThemeAtRule" => lang::TwThemeAtRule::KIND_SET.iter().next(),
        "TwUtilityAtRule" => lang::TwUtilityAtRule::KIND_SET.iter().next(),
        "TwValueThemeReference" => lang::TwValueThemeReference::KIND_SET.iter().next(),
        "TwVariantAtRule" => lang::TwVariantAtRule::KIND_SET.iter().next(),
        _ => None,
    }
}

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<CssSyntaxKind> {
    legacy_kind_by_name(node_name).or_else(|| native_kind_by_name(node_name))
}

/// Returns the native Biome slot mappings for a node name.
pub fn native_slots_for_name(node_name: &str) -> &'static [(&'static str, u32)] {
    match node_name {
        "CssAtRule" => &[("rule", 1)],
        "CssAtRuleDeclarator" => &[("declarator", 1)],
        "CssAttrFallbackValue" => &[("value", 1)],
        "CssAttrFunction" => &[("attr_name", 2), ("attr_type", 3), ("fallback_value", 4)],
        "CssAttributeMatcher" => &[("value", 1)],
        "CssAttributeMatcherValue" => &[("name", 0)],
        "CssAttributeName" => &[("namespace", 0), ("name", 1)],
        "CssAttributeSelector" => &[("name", 1), ("matcher", 2)],
        "CssBinaryExpression" => &[("left", 0), ("right", 2)],
        "CssBracketedValue" => &[("items", 1)],
        "CssCharsetAtRule" => &[("encoding", 1)],
        "CssClassSelector" => &[("name", 1)],
        "CssColorProfileAtRule" => &[("declarator", 0), ("block", 1)],
        "CssColorProfileAtRuleDeclarator" => &[("name", 1)],
        "CssCommaSeparatedValue" => &[("items", 1)],
        "CssComplexSelector" => &[("left", 0), ("right", 2)],
        "CssComposesImportSpecifier" => &[("source", 1)],
        "CssComposesProperty" => &[("name", 0), ("value", 2)],
        "CssComposesPropertyValue" => &[("classes", 0), ("specifier", 1)],
        "CssCompoundSelector" => &[
            ("nesting_selectors", 0),
            ("simple_selector", 1),
            ("sub_selectors", 2),
        ],
        "CssContainerAndQuery" => &[("left", 0), ("right", 2)],
        "CssContainerAtRule" => &[("declarator", 0), ("block", 1)],
        "CssContainerAtRuleDeclarator" => &[("name", 1), ("query", 2)],
        "CssContainerNotQuery" => &[("query", 1)],
        "CssContainerOrQuery" => &[("left", 0), ("right", 2)],
        "CssContainerQueryInParens" => &[("query", 1)],
        "CssContainerScrollStateAndQuery" => &[("left", 0), ("right", 2)],
        "CssContainerScrollStateInParens" => &[("query", 1)],
        "CssContainerScrollStateNotQuery" => &[("query", 1)],
        "CssContainerScrollStateOrQuery" => &[("left", 0), ("right", 2)],
        "CssContainerScrollStateQueryInParens" => &[("name", 0), ("query", 2)],
        "CssContainerSizeFeatureInParens" => &[("feature", 1)],
        "CssContainerStyleAndQuery" => &[("left", 0), ("right", 2)],
        "CssContainerStyleInParens" => &[("query", 1)],
        "CssContainerStyleNotQuery" => &[("query", 1)],
        "CssContainerStyleOrQuery" => &[("left", 0), ("right", 2)],
        "CssContainerStyleQueryInParens" => &[("query", 2)],
        "CssCounterStyleAtRule" => &[("declarator", 0), ("block", 1)],
        "CssCounterStyleAtRuleDeclarator" => &[("name", 1)],
        "CssDeclaration" => &[("property", 0), ("important", 1)],
        "CssDeclarationBlock" => &[("declarations", 1)],
        "CssDeclarationOrAtRuleBlock" => &[("items", 1)],
        "CssDeclarationOrRuleBlock" => &[("items", 1)],
        "CssDeclarationWithSemicolon" => &[("declaration", 0)],
        "CssDocumentAtRule" => &[("matchers", 1), ("block", 2)],
        "CssDocumentCustomMatcher" => &[("value", 2)],
        "CssFontFaceAtRule" => &[("declarator", 0), ("block", 1)],
        "CssFontFamilyName" => &[("names", 0)],
        "CssFontFeatureValuesAtRule" => &[("names", 1), ("block", 2)],
        "CssFontFeatureValuesBlock" => &[("items", 1)],
        "CssFontFeatureValuesItem" => &[("block", 2)],
        "CssFontPaletteValuesAtRule" => &[("declarator", 0), ("block", 1)],
        "CssFontPaletteValuesAtRuleDeclarator" => &[("name", 1)],
        "CssFunction" => &[("name", 0), ("items", 2)],
        "CssFunctionAtRule" => &[("declarator", 0), ("block", 1)],
        "CssFunctionAtRuleDeclarator" => &[("name", 1), ("parameters", 3), ("returns", 5)],
        "CssFunctionParameter" => &[("name", 0), ("ty", 1), ("default_value", 2)],
        "CssFunctionParameterDefaultValue" => &[("value", 1)],
        "CssGenericProperty" => &[("name", 0), ("value", 2)],
        "CssIdSelector" => &[("name", 1)],
        "CssIfBranch" => &[("condition", 0), ("value", 2)],
        "CssIfFunction" => &[("css_if_branch_list", 2)],
        "CssIfMediaTest" => &[("test", 2)],
        "CssIfSassTest" => &[("test", 2)],
        "CssIfStyleTest" => &[("test", 2)],
        "CssIfSupportsIdentifierTest" => &[("ident", 0), ("value", 2)],
        "CssIfSupportsTest" => &[("test", 2)],
        "CssIfTestBooleanAndExpr" => &[("left", 0), ("right", 2)],
        "CssIfTestBooleanExprInParens" => &[("expression", 1)],
        "CssIfTestBooleanNotExpr" => &[("expression", 1)],
        "CssIfTestBooleanOrExpr" => &[("left", 0), ("right", 2)],
        "CssImportAtRule" => &[("url", 1), ("layer", 2), ("supports", 3), ("media", 4)],
        "CssImportNamedLayer" => &[("name", 2)],
        "CssImportSupports" => &[("condition", 2)],
        "CssKeyframesAtRule" => &[("name", 1), ("block", 2)],
        "CssKeyframesBlock" => &[("items", 1)],
        "CssKeyframesItem" => &[("selectors", 0), ("block", 1)],
        "CssKeyframesPercentageSelector" => &[("selector", 0)],
        "CssKeyframesRangeSelector" => &[("percentage", 1)],
        "CssKeyframesScopeFunction" => &[("name", 2)],
        "CssKeyframesScopePrefix" => &[("name", 1)],
        "CssKeyframesScopedName" => &[("scope", 1)],
        "CssLayerAtRule" => &[("layer", 1)],
        "CssLayerDeclaration" => &[("references", 0), ("block", 1)],
        "CssLayerReference" => &[("references", 0)],
        "CssMarginAtRule" => &[("block", 2)],
        "CssMediaAndCondition" => &[("left", 0), ("right", 2)],
        "CssMediaAndTypeQuery" => &[("left", 0), ("right", 2)],
        "CssMediaAtRule" => &[("declarator", 0), ("block", 1)],
        "CssMediaAtRuleDeclarator" => &[("queries", 1)],
        "CssMediaConditionInParens" => &[("condition", 1)],
        "CssMediaConditionQuery" => &[("condition", 0)],
        "CssMediaFeatureInParens" => &[("feature", 1)],
        "CssMediaNotCondition" => &[("condition", 1)],
        "CssMediaOrCondition" => &[("left", 0), ("right", 2)],
        "CssMediaType" => &[("value", 0)],
        "CssMediaTypeQuery" => &[("ty", 1)],
        "CssNamedNamespacePrefix" => &[("name", 0)],
        "CssNamespace" => &[("prefix", 0)],
        "CssNamespaceAtRule" => &[("prefix", 1), ("url", 2)],
        "CssNestedQualifiedRule" => &[("prelude", 0), ("block", 1)],
        "CssNthOffset" => &[("value", 1)],
        "CssPageAtRule" => &[("selectors", 1), ("block", 2)],
        "CssPageAtRuleBlock" => &[("items", 1)],
        "CssPageSelector" => &[("ty", 0), ("pseudos", 1)],
        "CssPageSelectorPseudo" => &[("selector", 1)],
        "CssParenthesizedExpression" => &[("expression", 1)],
        "CssPositionTryAtRule" => &[("declarator", 0), ("block", 1)],
        "CssPositionTryAtRuleDeclarator" => &[("name", 1)],
        "CssPropertyAtRule" => &[("declarator", 0), ("block", 1)],
        "CssPropertyAtRuleDeclarator" => &[("name", 1)],
        "CssPseudoClassFunctionCompoundSelector" => &[("name", 0), ("selector", 2)],
        "CssPseudoClassFunctionCustomIdentifier" => &[("name", 0), ("ident", 2)],
        "CssPseudoClassFunctionIdentifier" => &[("name", 0), ("ident", 2)],
        "CssPseudoClassFunctionNth" => &[("name", 0), ("selector", 2)],
        "CssPseudoClassFunctionSelector" => &[("name", 0), ("selector", 2)],
        "CssPseudoClassIdentifier" => &[("name", 0)],
        "CssPseudoClassNth" => &[("value", 1), ("offset", 3)],
        "CssPseudoClassNthNumber" => &[("value", 1)],
        "CssPseudoClassNthSelector" => &[("nth", 0), ("of_selector", 1)],
        "CssPseudoClassOfNthSelector" => &[("selectors", 1)],
        "CssPseudoClassSelector" => &[("class", 1)],
        "CssPseudoElementFunction" => &[("name", 0), ("items", 2)],
        "CssPseudoElementFunctionCustomIdentifier" => &[("name", 0), ("ident", 2)],
        "CssPseudoElementFunctionSelector" => &[("name", 0), ("selector", 2)],
        "CssPseudoElementIdentifier" => &[("name", 0)],
        "CssPseudoElementSelector" => &[("element", 1)],
        "CssQualifiedRule" => &[("prelude", 0), ("block", 1)],
        "CssQueryFeatureBoolean" => &[("name", 0)],
        "CssQueryFeaturePlain" => &[("name", 0), ("value", 2)],
        "CssQueryFeatureRange" => &[("left", 0), ("comparison", 1), ("right", 2)],
        "CssQueryFeatureRangeInterval" => &[
            ("left", 0),
            ("left_comparison", 1),
            ("name", 2),
            ("right_comparison", 3),
            ("right", 4),
        ],
        "CssQueryFeatureReverseRange" => &[("left", 0), ("comparison", 1), ("right", 2)],
        "CssRatio" => &[("numerator", 0), ("denominator", 2)],
        "CssRelativeSelector" => &[("selector", 1)],
        "CssReturnsStatement" => &[("ty", 1)],
        "CssRoot" => &[("items", 1)],
        "CssRuleBlock" => &[("rules", 1)],
        "CssScopeAtRule" => &[("declarator", 0), ("block", 1)],
        "CssScopeAtRuleDeclarator" => &[("range", 1)],
        "CssScopeEdge" => &[("selectors", 1)],
        "CssScopeRangeEnd" => &[("end", 1)],
        "CssScopeRangeInterval" => &[("start", 0), ("end", 2)],
        "CssScopeRangeStart" => &[("start", 0)],
        "CssSnippetRoot" => &[("items", 0)],
        "CssStartingStyleAtRule" => &[("declarator", 0), ("block", 1)],
        "CssSupportsAndCondition" => &[("left", 0), ("right", 2)],
        "CssSupportsAtRule" => &[("declarator", 0), ("block", 1)],
        "CssSupportsAtRuleDeclarator" => &[("condition", 1)],
        "CssSupportsConditionInParens" => &[("condition", 1)],
        "CssSupportsFeatureDeclaration" => &[("declaration", 1)],
        "CssSupportsFeatureSelector" => &[("selector", 2)],
        "CssSupportsNotCondition" => &[("query", 1)],
        "CssSupportsOrCondition" => &[("left", 0), ("right", 2)],
        "CssSyntaxComponent" => &[("component", 0), ("multiplier", 1)],
        "CssSyntaxType" => &[("type_name", 1)],
        "CssTypeFunction" => &[("ty", 2)],
        "CssTypeSelector" => &[("namespace", 0), ("ident", 1)],
        "CssUnaryExpression" => &[("expression", 1)],
        "CssUnicodeRange" => &[("value", 1)],
        "CssUnicodeRangeInterval" => &[("start", 0), ("end", 2)],
        "CssUniversalSelector" => &[("namespace", 0)],
        "CssUnknownBlockAtRule" => &[("name", 0), ("components", 1), ("block", 2)],
        "CssUnknownValueAtRule" => &[("name", 0), ("components", 1)],
        "CssUrlFunction" => &[("value", 2), ("modifiers", 3)],
        "CssValueAtRule" => &[("clause", 1)],
        "CssValueAtRuleDeclarationClause" => &[("properties", 0)],
        "CssValueAtRuleGenericProperty" => &[("name", 0), ("value", 2)],
        "CssValueAtRuleImportClause" => &[("specifiers", 0), ("source", 2)],
        "CssValueAtRuleImportSpecifier" => &[("name", 0)],
        "CssValueAtRuleNamedImportSpecifier" => &[("name", 0), ("local_name", 2)],
        "CssViewTransitionAtRule" => &[("declarator", 0), ("block", 1)],
        "ScssArbitraryArgument" => &[("value", 0)],
        "ScssAtRootAtRule" => &[("query", 1), ("selector", 2), ("block", 3)],
        "ScssAtRootQuery" => &[("queries", 3)],
        "ScssAtRootSelector" => &[("selector", 0)],
        "ScssBinaryExpression" => &[("left", 0), ("right", 2)],
        "ScssContentAtRule" => &[("arguments", 1)],
        "ScssDebugAtRule" => &[("value", 1)],
        "ScssDeclaration" => &[("name", 0), ("value", 2), ("modifiers", 3)],
        "ScssEachAtRule" => &[("bindings", 1), ("iterable", 3), ("block", 4)],
        "ScssElseClause" => &[("body", 2)],
        "ScssErrorAtRule" => &[("value", 1)],
        "ScssExpression" => &[("items", 0)],
        "ScssExtendAtRule" => &[("css_selector_list", 1), ("optional_modifier", 2)],
        "ScssForAtRule" => &[
            ("variable", 1),
            ("lower_bound", 3),
            ("upper_bound", 5),
            ("block", 6),
        ],
        "ScssForwardAsClause" => &[("prefix", 1)],
        "ScssForwardAtRule" => &[
            ("url", 1),
            ("as_clause", 2),
            ("visibility_clause", 3),
            ("with_clause", 4),
        ],
        "ScssFunctionAtRule" => &[("name", 1), ("parameters", 2), ("block", 3)],
        "ScssHideClause" => &[("members", 1)],
        "ScssIdentifier" => &[("name", 1)],
        "ScssIfAtRule" => &[("condition", 1), ("block", 2), ("else_clause", 3)],
        "ScssImportAtRule" => &[("imports", 1)],
        "ScssIncludeAtRule" => &[("name", 1), ("arguments", 2), ("block", 3)],
        "ScssInterpolatedIdentifier" => &[("items", 0)],
        "ScssInterpolatedString" => &[("parts", 1)],
        "ScssInterpolation" => &[("value", 2)],
        "ScssKeywordArgument" => &[("name", 0), ("value", 2)],
        "ScssMapExpression" => &[("pairs", 1)],
        "ScssMapExpressionPair" => &[("key", 0), ("value", 2)],
        "ScssMixinAtRule" => &[("name", 1), ("parameters", 2), ("block", 3)],
        "ScssModuleConfiguration" => &[("name", 0), ("value", 2), ("modifier", 3)],
        "ScssNamespacedIdentifier" => &[("namespace", 0), ("name", 2)],
        "ScssNestingDeclaration" => &[("name", 0), ("value", 2), ("block", 3)],
        "ScssParameter" => &[("name", 0), ("default_value", 1)],
        "ScssParameterDefaultValue" => &[("value", 1)],
        "ScssParenthesizedExpression" => &[("expression", 1)],
        "ScssPlaceholderSelector" => &[("name", 1)],
        "ScssPlainImport" => &[("url", 0), ("layer", 1), ("supports", 2), ("media", 3)],
        "ScssQualifiedName" => &[("module", 0), ("member", 2)],
        "ScssReturnAtRule" => &[("value", 1)],
        "ScssShowClause" => &[("members", 1)],
        "ScssUnaryExpression" => &[("expression", 1)],
        "ScssUseAsClause" => &[("namespace", 1)],
        "ScssUseAtRule" => &[("url", 1), ("as_clause", 2), ("with_clause", 3)],
        "ScssWarnAtRule" => &[("value", 1)],
        "ScssWhileAtRule" => &[("condition", 1), ("block", 2)],
        "ScssWithClause" => &[("configurations", 1)],
        "TwApplyAtRule" => &[("classes", 1)],
        "TwConfigAtRule" => &[("path", 1)],
        "TwCustomVariantAtRule" => &[("name", 1), ("selector", 2)],
        "TwCustomVariantShorthand" => &[("selector", 1)],
        "TwFunctionalUtilityName" => &[("identifier", 0)],
        "TwPluginAtRule" => &[("name", 1), ("block", 2)],
        "TwReferenceAtRule" => &[("path", 1)],
        "TwSourceAtRule" => &[("source", 2)],
        "TwSourceInline" => &[("content", 2)],
        "TwThemeAtRule" => &[("name", 1), ("block", 2)],
        "TwUtilityAtRule" => &[("name", 1), ("block", 2)],
        "TwValueThemeReference" => &[("reference", 0)],
        "TwVariantAtRule" => &[("name", 1), ("block", 2)],
        _ => &[],
    }
}
