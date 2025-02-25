//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::GritSyntaxNode::kind(&node) {
                $crate::GritSyntaxKind::GRIT_ADD_OPERATION => {
                    let $pattern = unsafe { $crate::GritAddOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_ANNOTATION => {
                    let $pattern = unsafe { $crate::GritAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_ASSIGNMENT_AS_PATTERN => {
                    let $pattern = unsafe { $crate::GritAssignmentAsPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BACKTICK_SNIPPET_LITERAL => {
                    let $pattern =
                        unsafe { $crate::GritBacktickSnippetLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOOLEAN_LITERAL => {
                    let $pattern = unsafe { $crate::GritBooleanLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BRACKETED_PATTERN => {
                    let $pattern = unsafe { $crate::GritBracketedPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BRACKETED_PREDICATE => {
                    let $pattern = unsafe { $crate::GritBracketedPredicate::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BUBBLE => {
                    let $pattern = unsafe { $crate::GritBubble::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BUBBLE_SCOPE => {
                    let $pattern = unsafe { $crate::GritBubbleScope::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_CODE_SNIPPET => {
                    let $pattern = unsafe { $crate::GritCodeSnippet::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_CURLY_PATTERN => {
                    let $pattern = unsafe { $crate::GritCurlyPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_DIV_OPERATION => {
                    let $pattern = unsafe { $crate::GritDivOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_DOT => {
                    let $pattern = unsafe { $crate::GritDot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_DOTDOTDOT => {
                    let $pattern = unsafe { $crate::GritDotdotdot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_DOUBLE_LITERAL => {
                    let $pattern = unsafe { $crate::GritDoubleLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_ENGINE_NAME => {
                    let $pattern = unsafe { $crate::GritEngineName::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_EVERY => {
                    let $pattern = unsafe { $crate::GritEvery::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_FILES => {
                    let $pattern = unsafe { $crate::GritFiles::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_FUNCTION_DEFINITION => {
                    let $pattern = unsafe { $crate::GritFunctionDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_INT_LITERAL => {
                    let $pattern = unsafe { $crate::GritIntLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_JAVASCRIPT_BODY_WRAPPER => {
                    let $pattern =
                        unsafe { $crate::GritJavascriptBodyWrapper::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_JAVASCRIPT_FUNCTION_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GritJavascriptFunctionDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_DECLARATION => {
                    let $pattern = unsafe { $crate::GritLanguageDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_FLAVOR => {
                    let $pattern = unsafe { $crate::GritLanguageFlavor::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_FLAVOR_KIND => {
                    let $pattern = unsafe { $crate::GritLanguageFlavorKind::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_NAME => {
                    let $pattern = unsafe { $crate::GritLanguageName::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_SPECIFIC_SNIPPET => {
                    let $pattern =
                        unsafe { $crate::GritLanguageSpecificSnippet::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LIKE => {
                    let $pattern = unsafe { $crate::GritLike::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LIKE_THRESHOLD => {
                    let $pattern = unsafe { $crate::GritLikeThreshold::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LIST => {
                    let $pattern = unsafe { $crate::GritList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LIST_ACCESSOR => {
                    let $pattern = unsafe { $crate::GritListAccessor::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MAP => {
                    let $pattern = unsafe { $crate::GritMap::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MAP_ACCESSOR => {
                    let $pattern = unsafe { $crate::GritMapAccessor::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MAP_ELEMENT => {
                    let $pattern = unsafe { $crate::GritMapElement::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MOD_OPERATION => {
                    let $pattern = unsafe { $crate::GritModOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MUL_OPERATION => {
                    let $pattern = unsafe { $crate::GritMulOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NAME => {
                    let $pattern = unsafe { $crate::GritName::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NAMED_ARG => {
                    let $pattern = unsafe { $crate::GritNamedArg::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NEGATIVE_INT_LITERAL => {
                    let $pattern = unsafe { $crate::GritNegativeIntLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NODE_LIKE => {
                    let $pattern = unsafe { $crate::GritNodeLike::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NOT => {
                    let $pattern = unsafe { $crate::GritNot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_ACCUMULATE => {
                    let $pattern = unsafe { $crate::GritPatternAccumulate::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_AFTER => {
                    let $pattern = unsafe { $crate::GritPatternAfter::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_AND => {
                    let $pattern = unsafe { $crate::GritPatternAnd::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_ANY => {
                    let $pattern = unsafe { $crate::GritPatternAny::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_AS => {
                    let $pattern = unsafe { $crate::GritPatternAs::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_BEFORE => {
                    let $pattern = unsafe { $crate::GritPatternBefore::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_CONTAINS => {
                    let $pattern = unsafe { $crate::GritPatternContains::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_DEFINITION => {
                    let $pattern = unsafe { $crate::GritPatternDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_DEFINITION_BODY => {
                    let $pattern =
                        unsafe { $crate::GritPatternDefinitionBody::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::GritPatternElseClause::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_IF_ELSE => {
                    let $pattern = unsafe { $crate::GritPatternIfElse::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_INCLUDES => {
                    let $pattern = unsafe { $crate::GritPatternIncludes::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_LIMIT => {
                    let $pattern = unsafe { $crate::GritPatternLimit::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_MAYBE => {
                    let $pattern = unsafe { $crate::GritPatternMaybe::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_NOT => {
                    let $pattern = unsafe { $crate::GritPatternNot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_OR => {
                    let $pattern = unsafe { $crate::GritPatternOr::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_OR_ELSE => {
                    let $pattern = unsafe { $crate::GritPatternOrElse::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_UNTIL_CLAUSE => {
                    let $pattern = unsafe { $crate::GritPatternUntilClause::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_WHERE => {
                    let $pattern = unsafe { $crate::GritPatternWhere::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_ACCUMULATE => {
                    let $pattern = unsafe { $crate::GritPredicateAccumulate::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_AND => {
                    let $pattern = unsafe { $crate::GritPredicateAnd::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_ANY => {
                    let $pattern = unsafe { $crate::GritPredicateAny::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::GritPredicateAssignment::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_CALL => {
                    let $pattern = unsafe { $crate::GritPredicateCall::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_CURLY => {
                    let $pattern = unsafe { $crate::GritPredicateCurly::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_DEFINITION => {
                    let $pattern = unsafe { $crate::GritPredicateDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::GritPredicateElseClause::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_EQUAL => {
                    let $pattern = unsafe { $crate::GritPredicateEqual::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_GREATER => {
                    let $pattern = unsafe { $crate::GritPredicateGreater::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_GREATER_EQUAL => {
                    let $pattern =
                        unsafe { $crate::GritPredicateGreaterEqual::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_IF_ELSE => {
                    let $pattern = unsafe { $crate::GritPredicateIfElse::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_LESS => {
                    let $pattern = unsafe { $crate::GritPredicateLess::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_LESS_EQUAL => {
                    let $pattern = unsafe { $crate::GritPredicateLessEqual::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_MATCH => {
                    let $pattern = unsafe { $crate::GritPredicateMatch::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_MAYBE => {
                    let $pattern = unsafe { $crate::GritPredicateMaybe::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_NOT => {
                    let $pattern = unsafe { $crate::GritPredicateNot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_NOT_EQUAL => {
                    let $pattern = unsafe { $crate::GritPredicateNotEqual::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_OR => {
                    let $pattern = unsafe { $crate::GritPredicateOr::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_RETURN => {
                    let $pattern = unsafe { $crate::GritPredicateReturn::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_REWRITE => {
                    let $pattern = unsafe { $crate::GritPredicateRewrite::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_RAW_BACKTICK_SNIPPET_LITERAL => {
                    let $pattern =
                        unsafe { $crate::GritRawBacktickSnippetLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_REGEX_LITERAL => {
                    let $pattern = unsafe { $crate::GritRegexLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_REGEX_PATTERN => {
                    let $pattern = unsafe { $crate::GritRegexPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_REGEX_PATTERN_VARIABLES => {
                    let $pattern =
                        unsafe { $crate::GritRegexPatternVariables::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_REWRITE => {
                    let $pattern = unsafe { $crate::GritRewrite::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_ROOT => {
                    let $pattern = unsafe { $crate::GritRoot::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SEQUENTIAL => {
                    let $pattern = unsafe { $crate::GritSequential::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SNIPPET_REGEX_LITERAL => {
                    let $pattern = unsafe { $crate::GritSnippetRegexLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SOME => {
                    let $pattern = unsafe { $crate::GritSome::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_STRING_LITERAL => {
                    let $pattern = unsafe { $crate::GritStringLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SUB_OPERATION => {
                    let $pattern = unsafe { $crate::GritSubOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_UNDEFINED_LITERAL => {
                    let $pattern = unsafe { $crate::GritUndefinedLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_UNDERSCORE => {
                    let $pattern = unsafe { $crate::GritUnderscore::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_VARIABLE => {
                    let $pattern = unsafe { $crate::GritVariable::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_VERSION => {
                    let $pattern = unsafe { $crate::GritVersion::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_WITHIN => {
                    let $pattern = unsafe { $crate::GritWithin::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS => {
                    let $pattern = unsafe { $crate::GritBogus::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_CONTAINER => {
                    let $pattern = unsafe { $crate::GritBogusContainer::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_DEFINITION => {
                    let $pattern = unsafe { $crate::GritBogusDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_LANGUAGE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::GritBogusLanguageDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_LANGUAGE_FLAVOR_KIND => {
                    let $pattern =
                        unsafe { $crate::GritBogusLanguageFlavorKind::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_LANGUAGE_NAME => {
                    let $pattern = unsafe { $crate::GritBogusLanguageName::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_LITERAL => {
                    let $pattern = unsafe { $crate::GritBogusLiteral::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_MAP_ELEMENT => {
                    let $pattern = unsafe { $crate::GritBogusMapElement::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_NAMED_ARG => {
                    let $pattern = unsafe { $crate::GritBogusNamedArg::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_PATTERN => {
                    let $pattern = unsafe { $crate::GritBogusPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_PREDICATE => {
                    let $pattern = unsafe { $crate::GritBogusPredicate::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_VERSION => {
                    let $pattern = unsafe { $crate::GritBogusVersion::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::GritDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LANGUAGE_FLAVOR_LIST => {
                    let $pattern = unsafe { $crate::GritLanguageFlavorList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_LIST_PATTERN_LIST => {
                    let $pattern = unsafe { $crate::GritListPatternList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_MAP_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::GritMapElementList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NAMED_ARG_LIST => {
                    let $pattern = unsafe { $crate::GritNamedArgList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PATTERN_LIST => {
                    let $pattern = unsafe { $crate::GritPatternList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_PREDICATE_LIST => {
                    let $pattern = unsafe { $crate::GritPredicateList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_VARIABLE_LIST => {
                    let $pattern = unsafe { $crate::GritVariableList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
