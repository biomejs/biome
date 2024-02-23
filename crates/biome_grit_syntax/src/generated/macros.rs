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
                $crate::GritSyntaxKind::ANY_GRIT_PATTERN => {
                    let $pattern = unsafe { $crate::AnyGritPattern::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::ANY_GRIT_PREDICATE => {
                    let $pattern = unsafe { $crate::AnyGritPredicate::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::CURLY_GRIT_PATTERN => {
                    let $pattern = unsafe { $crate::CurlyGritPattern::new_unchecked(node) };
                    $body
                }
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
                $crate::GritSyntaxKind::GRIT_BACKTICK_SNIPPET => {
                    let $pattern = unsafe { $crate::GritBacktickSnippet::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOOLEAN_VALUE => {
                    let $pattern = unsafe { $crate::GritBooleanValue::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_CURLY_PREDICATE_LIST => {
                    let $pattern = unsafe { $crate::GritCurlyPredicateList::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_DOUBLE_VALUE => {
                    let $pattern = unsafe { $crate::GritDoubleValue::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_INT_VALUE => {
                    let $pattern = unsafe { $crate::GritIntValue::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_NAMED_ARG_WITH_DEFAULT => {
                    let $pattern = unsafe { $crate::GritNamedArgWithDefault::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_NEGATIVE_INT_VALUE => {
                    let $pattern = unsafe { $crate::GritNegativeIntValue::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_PATTERN_ARG_LIST => {
                    let $pattern = unsafe { $crate::GritPatternArgList::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::GritPatternContainsUntilClause::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_RAW_BACKTICK_SNIPPET => {
                    let $pattern = unsafe { $crate::GritRawBacktickSnippet::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_REGEX_VALUE => {
                    let $pattern = unsafe { $crate::GritRegexValue::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_SNIPPET_REGEX_VALUE => {
                    let $pattern = unsafe { $crate::GritSnippetRegexValue::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SOME => {
                    let $pattern = unsafe { $crate::GritSome::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_STRING_VALUE => {
                    let $pattern = unsafe { $crate::GritStringValue::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_SUB_OPERATION => {
                    let $pattern = unsafe { $crate::GritSubOperation::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_UNDEFINED => {
                    let $pattern = unsafe { $crate::GritUndefined::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_BOGUS_DEFINITION => {
                    let $pattern = unsafe { $crate::GritBogusDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_BOGUS_LITERAL => {
                    let $pattern = unsafe { $crate::GritBogusLiteral::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::GritDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::GritSyntaxKind::GRIT_FILES_LIST => {
                    let $pattern = unsafe { $crate::GritFilesList::new_unchecked(node) };
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
                $crate::GritSyntaxKind::GRIT_SEQUENTIAL_LIST => {
                    let $pattern = unsafe { $crate::GritSequentialList::new_unchecked(node) };
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
