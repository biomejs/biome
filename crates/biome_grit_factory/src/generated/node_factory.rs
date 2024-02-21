//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_grit_syntax::{
    GritSyntaxElement as SyntaxElement, GritSyntaxNode as SyntaxNode,
    GritSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn any_grit_pattern(
    any_grit_literal: AnyGritLiteral,
    grit_pattern_not: GritPatternNot,
    grit_pattern_or: GritPatternOr,
    grit_pattern_or_else: GritPatternOrElse,
    grit_pattern_any: GritPatternAny,
    grit_pattern_and: GritPatternAnd,
    grit_pattern_maybe: GritPatternMaybe,
    grit_pattern_if_else: GritPatternIfElse,
    grit_pattern_contains: GritPatternContains,
    grit_pattern_includes: GritPatternIncludes,
    grit_pattern_after: GritPatternAfter,
    grit_pattern_before: GritPatternBefore,
    grit_within: GritWithin,
    grit_bubble: GritBubble,
    grit_node_like: GritNodeLike,
    grit_map_accessor: GritMapAccessor,
    grit_list_accessor: GritListAccessor,
    grit_dot: GritDot,
    grit_some: GritSome,
    grit_every: GritEvery,
    grit_underscore: GritUnderscore,
    grit_variable: GritVariable,
    grit_regex_pattern: GritRegexPattern,
    grit_pattern_as: GritPatternAs,
    grit_pattern_limit: GritPatternLimit,
    grit_assignment_as_pattern: GritAssignmentAsPattern,
    grit_pattern_accumulate: GritPatternAccumulate,
    grit_rewrite: GritRewrite,
    grit_like: GritLike,
    grit_pattern_where: GritPatternWhere,
    grit_mul_operation: GritMulOperation,
    grit_div_operation: GritDivOperation,
    grit_mod_operation: GritModOperation,
    grit_add_operation: GritAddOperation,
    grit_sub_operation: GritSubOperation,
    grit_sequential: GritSequential,
    grit_files: GritFiles,
    l_paren_token: SyntaxToken,
    any_grit_pattern: AnyGritPattern,
    r_paren_token: SyntaxToken,
    grit_bogus_pattern: GritBogusPattern,
) -> AnyGritPattern {
    AnyGritPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::ANY_GRIT_PATTERN,
        [
            Some(SyntaxElement::Node(any_grit_literal.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_not.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_or.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_or_else.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_any.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_and.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_maybe.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_if_else.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_contains.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_includes.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_after.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_before.into_syntax())),
            Some(SyntaxElement::Node(grit_within.into_syntax())),
            Some(SyntaxElement::Node(grit_bubble.into_syntax())),
            Some(SyntaxElement::Node(grit_node_like.into_syntax())),
            Some(SyntaxElement::Node(grit_map_accessor.into_syntax())),
            Some(SyntaxElement::Node(grit_list_accessor.into_syntax())),
            Some(SyntaxElement::Node(grit_dot.into_syntax())),
            Some(SyntaxElement::Node(grit_some.into_syntax())),
            Some(SyntaxElement::Node(grit_every.into_syntax())),
            Some(SyntaxElement::Node(grit_underscore.into_syntax())),
            Some(SyntaxElement::Node(grit_variable.into_syntax())),
            Some(SyntaxElement::Node(grit_regex_pattern.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_as.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_limit.into_syntax())),
            Some(SyntaxElement::Node(
                grit_assignment_as_pattern.into_syntax(),
            )),
            Some(SyntaxElement::Node(grit_pattern_accumulate.into_syntax())),
            Some(SyntaxElement::Node(grit_rewrite.into_syntax())),
            Some(SyntaxElement::Node(grit_like.into_syntax())),
            Some(SyntaxElement::Node(grit_pattern_where.into_syntax())),
            Some(SyntaxElement::Node(grit_mul_operation.into_syntax())),
            Some(SyntaxElement::Node(grit_div_operation.into_syntax())),
            Some(SyntaxElement::Node(grit_mod_operation.into_syntax())),
            Some(SyntaxElement::Node(grit_add_operation.into_syntax())),
            Some(SyntaxElement::Node(grit_sub_operation.into_syntax())),
            Some(SyntaxElement::Node(grit_sequential.into_syntax())),
            Some(SyntaxElement::Node(grit_files.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(any_grit_pattern.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(grit_bogus_pattern.into_syntax())),
        ],
    ))
}
pub fn any_grit_predicate(
    grit_predicate_not: GritPredicateNot,
    grit_predicate_maybe: GritPredicateMaybe,
    grit_predicate_and: GritPredicateAnd,
    grit_predicate_or: GritPredicateOr,
    grit_predicate_any: GritPredicateAny,
    grit_predicate_if_else: GritPredicateIfElse,
    grit_predicate_assignment: GritPredicateAssignment,
    grit_predicate_accumulate: GritPredicateAccumulate,
    grit_predicate_rewrite: GritPredicateRewrite,
    grit_predicate_greater: GritPredicateGreater,
    grit_predicate_less: GritPredicateLess,
    grit_predicate_greater_equal: GritPredicateGreaterEqual,
    grit_predicate_less_equal: GritPredicateLessEqual,
    grit_predicate_not_equal: GritPredicateNotEqual,
    grit_predicate_equal: GritPredicateEqual,
    grit_predicate_match: GritPredicateMatch,
    grit_predicate_call: GritPredicateCall,
    l_paren_token: SyntaxToken,
    any_grit_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    grit_boolean_literal: GritBooleanLiteral,
    grit_predicate_return: GritPredicateReturn,
    grit_bogus_predicate: GritBogusPredicate,
) -> AnyGritPredicate {
    AnyGritPredicate::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::ANY_GRIT_PREDICATE,
        [
            Some(SyntaxElement::Node(grit_predicate_not.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_maybe.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_and.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_or.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_any.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_if_else.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_assignment.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_accumulate.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_rewrite.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_greater.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_less.into_syntax())),
            Some(SyntaxElement::Node(
                grit_predicate_greater_equal.into_syntax(),
            )),
            Some(SyntaxElement::Node(grit_predicate_less_equal.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_not_equal.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_equal.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_match.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_call.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(any_grit_predicate.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(grit_boolean_literal.into_syntax())),
            Some(SyntaxElement::Node(grit_predicate_return.into_syntax())),
            Some(SyntaxElement::Node(grit_bogus_predicate.into_syntax())),
        ],
    ))
}
pub fn curly_grit_pattern(
    l_curly_token: SyntaxToken,
    any_grit_pattern: AnyGritPattern,
    r_curly_token: SyntaxToken,
) -> CurlyGritPattern {
    CurlyGritPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::CURLY_GRIT_PATTERN,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(any_grit_pattern.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_add_operation(
    left: AnyGritPattern,
    plus_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritAddOperation {
    GritAddOperation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_ADD_OPERATION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(plus_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_annotation(grit_annotation_token: SyntaxToken) -> GritAnnotation {
    GritAnnotation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_ANNOTATION,
        [Some(SyntaxElement::Token(grit_annotation_token))],
    ))
}
pub fn grit_assignment_as_pattern(
    container: AnyGritContainer,
    eq_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritAssignmentAsPattern {
    GritAssignmentAsPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_ASSIGNMENT_AS_PATTERN,
        [
            Some(SyntaxElement::Node(container.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_backtick_snippet(value_token: SyntaxToken) -> GritBacktickSnippet {
    GritBacktickSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BACKTICK_SNIPPET,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_boolean_literal(
    true_token: SyntaxToken,
    false_token: SyntaxToken,
) -> GritBooleanLiteral {
    GritBooleanLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOOLEAN_LITERAL,
        [
            Some(SyntaxElement::Token(true_token)),
            Some(SyntaxElement::Token(false_token)),
        ],
    ))
}
pub fn grit_bubble(bubble_token: SyntaxToken, pattern: MaybeCurlyGritPattern) -> GritBubbleBuilder {
    GritBubbleBuilder {
        bubble_token,
        pattern,
        variables: None,
    }
}
pub struct GritBubbleBuilder {
    bubble_token: SyntaxToken,
    pattern: MaybeCurlyGritPattern,
    variables: Option<GritBubbleScope>,
}
impl GritBubbleBuilder {
    pub fn with_variables(mut self, variables: GritBubbleScope) -> Self {
        self.variables = Some(variables);
        self
    }
    pub fn build(self) -> GritBubble {
        GritBubble::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_BUBBLE,
            [
                Some(SyntaxElement::Token(self.bubble_token)),
                self.variables
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
            ],
        ))
    }
}
pub fn grit_bubble_scope(
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> GritBubbleScopeBuilder {
    GritBubbleScopeBuilder {
        l_paren_token,
        r_paren_token,
        grit_variable_list: None,
    }
}
pub struct GritBubbleScopeBuilder {
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    grit_variable_list: Option<GritVariableList>,
}
impl GritBubbleScopeBuilder {
    pub fn with_grit_variable_list(mut self, grit_variable_list: GritVariableList) -> Self {
        self.grit_variable_list = Some(grit_variable_list);
        self
    }
    pub fn build(self) -> GritBubbleScope {
        GritBubbleScope::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_BUBBLE_SCOPE,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.grit_variable_list
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn grit_code_snippet(source: GritCodeSnippetSource) -> GritCodeSnippet {
    GritCodeSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_CODE_SNIPPET,
        [Some(SyntaxElement::Node(source.into_syntax()))],
    ))
}
pub fn grit_curly_predicate_list(
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritCurlyPredicateListBuilder {
    GritCurlyPredicateListBuilder {
        l_curly_token,
        r_curly_token,
        predicates: None,
    }
}
pub struct GritCurlyPredicateListBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    predicates: Option<GritPredicateList>,
}
impl GritCurlyPredicateListBuilder {
    pub fn with_predicates(mut self, predicates: GritPredicateList) -> Self {
        self.predicates = Some(predicates);
        self
    }
    pub fn build(self) -> GritCurlyPredicateList {
        GritCurlyPredicateList::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_CURLY_PREDICATE_LIST,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.predicates
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_div_operation(
    left: AnyGritPattern,
    slash_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritDivOperation {
    GritDivOperation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_DIV_OPERATION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_dot(dot_token: SyntaxToken) -> GritDot {
    GritDot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_DOT,
        [Some(SyntaxElement::Token(dot_token))],
    ))
}
pub fn grit_dotdotdot(dollar_dotdotdot_token: SyntaxToken) -> GritDotdotdotBuilder {
    GritDotdotdotBuilder {
        dollar_dotdotdot_token,
        maybe_curly_grit_pattern: None,
    }
}
pub struct GritDotdotdotBuilder {
    dollar_dotdotdot_token: SyntaxToken,
    maybe_curly_grit_pattern: Option<MaybeCurlyGritPattern>,
}
impl GritDotdotdotBuilder {
    pub fn with_maybe_curly_grit_pattern(
        mut self,
        maybe_curly_grit_pattern: MaybeCurlyGritPattern,
    ) -> Self {
        self.maybe_curly_grit_pattern = Some(maybe_curly_grit_pattern);
        self
    }
    pub fn build(self) -> GritDotdotdot {
        GritDotdotdot::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_DOTDOTDOT,
            [
                Some(SyntaxElement::Token(self.dollar_dotdotdot_token)),
                self.maybe_curly_grit_pattern
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_double_literal(value_token: SyntaxToken) -> GritDoubleLiteral {
    GritDoubleLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_DOUBLE_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_double_quote_snippet(value_token: SyntaxToken) -> GritDoubleQuoteSnippet {
    GritDoubleQuoteSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_DOUBLE_QUOTE_SNIPPET,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_every(every_token: SyntaxToken, pattern: MaybeCurlyGritPattern) -> GritEvery {
    GritEvery::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_EVERY,
        [
            Some(SyntaxElement::Token(every_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_files(
    multifile_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    files: GritFilesList,
    r_curly_token: SyntaxToken,
) -> GritFiles {
    GritFiles::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_FILES,
        [
            Some(SyntaxElement::Token(multifile_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(files.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_function_definition(
    function_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritCurlyPredicateList,
) -> GritFunctionDefinitionBuilder {
    GritFunctionDefinitionBuilder {
        function_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        args: None,
    }
}
pub struct GritFunctionDefinitionBuilder {
    function_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritCurlyPredicateList,
    args: Option<GritVariableList>,
}
impl GritFunctionDefinitionBuilder {
    pub fn with_args(mut self, args: GritVariableList) -> Self {
        self.args = Some(args);
        self
    }
    pub fn build(self) -> GritFunctionDefinition {
        GritFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_FUNCTION_DEFINITION,
            [
                Some(SyntaxElement::Token(self.function_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn grit_int_literal(value_token: SyntaxToken) -> GritIntLiteral {
    GritIntLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_INT_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_language_declaration(
    language_token: SyntaxToken,
    name: GritLanguageName,
) -> GritLanguageDeclarationBuilder {
    GritLanguageDeclarationBuilder {
        language_token,
        name,
        flavor: None,
    }
}
pub struct GritLanguageDeclarationBuilder {
    language_token: SyntaxToken,
    name: GritLanguageName,
    flavor: Option<GritLanguageFlavor>,
}
impl GritLanguageDeclarationBuilder {
    pub fn with_flavor(mut self, flavor: GritLanguageFlavor) -> Self {
        self.flavor = Some(flavor);
        self
    }
    pub fn build(self) -> GritLanguageDeclaration {
        GritLanguageDeclaration::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LANGUAGE_DECLARATION,
            [
                Some(SyntaxElement::Token(self.language_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.flavor
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_language_flavor(
    l_paren_token: SyntaxToken,
    grit_language_flavor_list: GritLanguageFlavorList,
    r_paren_token: SyntaxToken,
) -> GritLanguageFlavorBuilder {
    GritLanguageFlavorBuilder {
        l_paren_token,
        grit_language_flavor_list,
        r_paren_token,
        semicolon_token: None,
    }
}
pub struct GritLanguageFlavorBuilder {
    l_paren_token: SyntaxToken,
    grit_language_flavor_list: GritLanguageFlavorList,
    r_paren_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl GritLanguageFlavorBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> GritLanguageFlavor {
        GritLanguageFlavor::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LANGUAGE_FLAVOR,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(
                    self.grit_language_flavor_list.into_syntax(),
                )),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn grit_language_flavor_kind(flavor_kind_token: SyntaxToken) -> GritLanguageFlavorKind {
    GritLanguageFlavorKind::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_FLAVOR_KIND,
        [Some(SyntaxElement::Token(flavor_kind_token))],
    ))
}
pub fn grit_language_name(
    js_token: SyntaxToken,
    css_token: SyntaxToken,
    json_token: SyntaxToken,
    grit_token: SyntaxToken,
    html_token: SyntaxToken,
) -> GritLanguageName {
    GritLanguageName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_NAME,
        [
            Some(SyntaxElement::Token(js_token)),
            Some(SyntaxElement::Token(css_token)),
            Some(SyntaxElement::Token(json_token)),
            Some(SyntaxElement::Token(grit_token)),
            Some(SyntaxElement::Token(html_token)),
        ],
    ))
}
pub fn grit_language_specific_snippet(
    language: GritLanguageName,
    snippet: GritDoubleQuoteSnippet,
) -> GritLanguageSpecificSnippet {
    GritLanguageSpecificSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_SPECIFIC_SNIPPET,
        [
            Some(SyntaxElement::Node(language.into_syntax())),
            Some(SyntaxElement::Node(snippet.into_syntax())),
        ],
    ))
}
pub fn grit_like(
    like_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    example: AnyGritPattern,
    r_curly_token: SyntaxToken,
) -> GritLikeBuilder {
    GritLikeBuilder {
        like_token,
        l_curly_token,
        example,
        r_curly_token,
        grit_like_threshold: None,
    }
}
pub struct GritLikeBuilder {
    like_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    example: AnyGritPattern,
    r_curly_token: SyntaxToken,
    grit_like_threshold: Option<GritLikeThreshold>,
}
impl GritLikeBuilder {
    pub fn with_grit_like_threshold(mut self, grit_like_threshold: GritLikeThreshold) -> Self {
        self.grit_like_threshold = Some(grit_like_threshold);
        self
    }
    pub fn build(self) -> GritLike {
        GritLike::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LIKE,
            [
                Some(SyntaxElement::Token(self.like_token)),
                self.grit_like_threshold
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.example.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_like_threshold(
    l_paren_token: SyntaxToken,
    threshold: AnyGritPattern,
    r_paren_token: SyntaxToken,
) -> GritLikeThreshold {
    GritLikeThreshold::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LIKE_THRESHOLD,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(threshold.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_list(l_brack_token: SyntaxToken, r_brack_token: SyntaxToken) -> GritListBuilder {
    GritListBuilder {
        l_brack_token,
        r_brack_token,
        grit_name: None,
        patterns: None,
    }
}
pub struct GritListBuilder {
    l_brack_token: SyntaxToken,
    r_brack_token: SyntaxToken,
    grit_name: Option<GritName>,
    patterns: Option<GritListPatternList>,
}
impl GritListBuilder {
    pub fn with_grit_name(mut self, grit_name: GritName) -> Self {
        self.grit_name = Some(grit_name);
        self
    }
    pub fn with_patterns(mut self, patterns: GritListPatternList) -> Self {
        self.patterns = Some(patterns);
        self
    }
    pub fn build(self) -> GritList {
        GritList::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LIST,
            [
                self.grit_name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                self.patterns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn grit_list_accessor(
    list: GritListAccessorSubject,
    l_brack_token: SyntaxToken,
    index: GritListIndex,
    r_brack_token: SyntaxToken,
) -> GritListAccessor {
    GritListAccessor::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LIST_ACCESSOR,
        [
            Some(SyntaxElement::Node(list.into_syntax())),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(index.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn grit_map(l_curly_token: SyntaxToken, r_curly_token: SyntaxToken) -> GritMapBuilder {
    GritMapBuilder {
        l_curly_token,
        r_curly_token,
        elements: None,
    }
}
pub struct GritMapBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    elements: Option<GritMapElementList>,
}
impl GritMapBuilder {
    pub fn with_elements(mut self, elements: GritMapElementList) -> Self {
        self.elements = Some(elements);
        self
    }
    pub fn build(self) -> GritMap {
        GritMap::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_MAP,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.elements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_map_accessor(
    map: GritMapAccessorSubject,
    dot_token: SyntaxToken,
    key: GritMapKey,
) -> GritMapAccessor {
    GritMapAccessor::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MAP_ACCESSOR,
        [
            Some(SyntaxElement::Node(map.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(key.into_syntax())),
        ],
    ))
}
pub fn grit_map_element(
    key: GritName,
    colon_token: SyntaxToken,
    value: AnyGritPattern,
) -> GritMapElement {
    GritMapElement::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MAP_ELEMENT,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn grit_mod_operation(
    left: AnyGritPattern,
    remainder_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritModOperation {
    GritModOperation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MOD_OPERATION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(remainder_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_mul_operation(
    left: AnyGritPattern,
    star_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritMulOperation {
    GritMulOperation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MUL_OPERATION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(star_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_name(grit_name_token: SyntaxToken) -> GritName {
    GritName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAME,
        [Some(SyntaxElement::Token(grit_name_token))],
    ))
}
pub fn grit_named_arg(name: GritName) -> GritNamedArg {
    GritNamedArg::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAMED_ARG,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn grit_named_arg_with_default(
    name: GritName,
    eq_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritNamedArgWithDefault {
    GritNamedArgWithDefault::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAMED_ARG_WITH_DEFAULT,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_negative_int_literal(value_token: SyntaxToken) -> GritNegativeIntLiteral {
    GritNegativeIntLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NEGATIVE_INT_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_node_like(
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> GritNodeLikeBuilder {
    GritNodeLikeBuilder {
        name,
        l_paren_token,
        r_paren_token,
        named_args: None,
    }
}
pub struct GritNodeLikeBuilder {
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    named_args: Option<GritNamedArgList>,
}
impl GritNodeLikeBuilder {
    pub fn with_named_args(mut self, named_args: GritNamedArgList) -> Self {
        self.named_args = Some(named_args);
        self
    }
    pub fn build(self) -> GritNodeLike {
        GritNodeLike::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_NODE_LIKE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.named_args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn grit_not(not_token: SyntaxToken, excl_token: SyntaxToken) -> GritNot {
    GritNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NOT,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Token(excl_token)),
        ],
    ))
}
pub fn grit_pattern_accumulate(
    left: AnyGritPattern,
    add_assign_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPatternAccumulate {
    GritPatternAccumulate::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_ACCUMULATE,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(add_assign_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_after(after_token: SyntaxToken, pattern: AnyGritPattern) -> GritPatternAfter {
    GritPatternAfter::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_AFTER,
        [
            Some(SyntaxElement::Token(after_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_and(
    and_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPatternAndBuilder {
    GritPatternAndBuilder {
        and_token,
        l_curly_token,
        r_curly_token,
        patterns: None,
    }
}
pub struct GritPatternAndBuilder {
    and_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    patterns: Option<GritPatternList>,
}
impl GritPatternAndBuilder {
    pub fn with_patterns(mut self, patterns: GritPatternList) -> Self {
        self.patterns = Some(patterns);
        self
    }
    pub fn build(self) -> GritPatternAnd {
        GritPatternAnd::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_AND,
            [
                Some(SyntaxElement::Token(self.and_token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.patterns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_pattern_any(
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPatternAnyBuilder {
    GritPatternAnyBuilder {
        any_token,
        l_curly_token,
        r_curly_token,
        patterns: None,
    }
}
pub struct GritPatternAnyBuilder {
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    patterns: Option<GritPatternList>,
}
impl GritPatternAnyBuilder {
    pub fn with_patterns(mut self, patterns: GritPatternList) -> Self {
        self.patterns = Some(patterns);
        self
    }
    pub fn build(self) -> GritPatternAny {
        GritPatternAny::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_ANY,
            [
                Some(SyntaxElement::Token(self.any_token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.patterns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_pattern_arg_list(grit_variable_list: GritVariableList) -> GritPatternArgList {
    GritPatternArgList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_ARG_LIST,
        [Some(SyntaxElement::Node(grit_variable_list.into_syntax()))],
    ))
}
pub fn grit_pattern_as(
    pattern: AnyGritPattern,
    as_token: SyntaxToken,
    variable: GritVariable,
) -> GritPatternAs {
    GritPatternAs::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_AS,
        [
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(variable.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_before(
    before_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritPatternBefore {
    GritPatternBefore::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_BEFORE,
        [
            Some(SyntaxElement::Token(before_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_contains(
    contains_token: SyntaxToken,
    contains: MaybeCurlyGritPattern,
) -> GritPatternContainsBuilder {
    GritPatternContainsBuilder {
        contains_token,
        contains,
        grit_pattern_contains_until_clause: None,
    }
}
pub struct GritPatternContainsBuilder {
    contains_token: SyntaxToken,
    contains: MaybeCurlyGritPattern,
    grit_pattern_contains_until_clause: Option<GritPatternContainsUntilClause>,
}
impl GritPatternContainsBuilder {
    pub fn with_grit_pattern_contains_until_clause(
        mut self,
        grit_pattern_contains_until_clause: GritPatternContainsUntilClause,
    ) -> Self {
        self.grit_pattern_contains_until_clause = Some(grit_pattern_contains_until_clause);
        self
    }
    pub fn build(self) -> GritPatternContains {
        GritPatternContains::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_CONTAINS,
            [
                Some(SyntaxElement::Token(self.contains_token)),
                Some(SyntaxElement::Node(self.contains.into_syntax())),
                self.grit_pattern_contains_until_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_pattern_contains_until_clause(
    until_token: SyntaxToken,
    until: AnyGritPattern,
) -> GritPatternContainsUntilClause {
    GritPatternContainsUntilClause::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE,
        [
            Some(SyntaxElement::Token(until_token)),
            Some(SyntaxElement::Node(until.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_definition(
    pattern_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritPatternDefinitionBody,
) -> GritPatternDefinitionBuilder {
    GritPatternDefinitionBuilder {
        pattern_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        visibility_token: None,
        args: None,
        language: None,
    }
}
pub struct GritPatternDefinitionBuilder {
    pattern_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritPatternDefinitionBody,
    visibility_token: Option<SyntaxToken>,
    args: Option<GritPatternArgList>,
    language: Option<GritLanguageDeclaration>,
}
impl GritPatternDefinitionBuilder {
    pub fn with_visibility_token(mut self, visibility_token: SyntaxToken) -> Self {
        self.visibility_token = Some(visibility_token);
        self
    }
    pub fn with_args(mut self, args: GritPatternArgList) -> Self {
        self.args = Some(args);
        self
    }
    pub fn with_language(mut self, language: GritLanguageDeclaration) -> Self {
        self.language = Some(language);
        self
    }
    pub fn build(self) -> GritPatternDefinition {
        GritPatternDefinition::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_DEFINITION,
            [
                self.visibility_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.pattern_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.language
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn grit_pattern_definition_body(
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPatternDefinitionBodyBuilder {
    GritPatternDefinitionBodyBuilder {
        l_curly_token,
        r_curly_token,
        patterns: None,
    }
}
pub struct GritPatternDefinitionBodyBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    patterns: Option<GritPatternList>,
}
impl GritPatternDefinitionBodyBuilder {
    pub fn with_patterns(mut self, patterns: GritPatternList) -> Self {
        self.patterns = Some(patterns);
        self
    }
    pub fn build(self) -> GritPatternDefinitionBody {
        GritPatternDefinitionBody::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_DEFINITION_BODY,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.patterns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_pattern_else_clause(
    else_token: SyntaxToken,
    else_pattern: MaybeCurlyGritPattern,
) -> GritPatternElseClause {
    GritPatternElseClause::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(else_pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_if_else(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_pattern: MaybeCurlyGritPattern,
) -> GritPatternIfElseBuilder {
    GritPatternIfElseBuilder {
        if_token,
        l_paren_token,
        if_predicate,
        r_paren_token,
        then_pattern,
        grit_pattern_else_clause: None,
    }
}
pub struct GritPatternIfElseBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_pattern: MaybeCurlyGritPattern,
    grit_pattern_else_clause: Option<GritPatternElseClause>,
}
impl GritPatternIfElseBuilder {
    pub fn with_grit_pattern_else_clause(
        mut self,
        grit_pattern_else_clause: GritPatternElseClause,
    ) -> Self {
        self.grit_pattern_else_clause = Some(grit_pattern_else_clause);
        self
    }
    pub fn build(self) -> GritPatternIfElse {
        GritPatternIfElse::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_IF_ELSE,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.if_predicate.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.then_pattern.into_syntax())),
                self.grit_pattern_else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_pattern_includes(
    includes_token: SyntaxToken,
    includes: MaybeCurlyGritPattern,
) -> GritPatternIncludes {
    GritPatternIncludes::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_INCLUDES,
        [
            Some(SyntaxElement::Token(includes_token)),
            Some(SyntaxElement::Node(includes.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_limit(
    pattern: AnyGritPattern,
    limit_token: SyntaxToken,
    limit: GritIntLiteral,
) -> GritPatternLimit {
    GritPatternLimit::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_LIMIT,
        [
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(limit_token)),
            Some(SyntaxElement::Node(limit.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_maybe(
    maybe_token: SyntaxToken,
    pattern: MaybeCurlyGritPattern,
) -> GritPatternMaybe {
    GritPatternMaybe::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_MAYBE,
        [
            Some(SyntaxElement::Token(maybe_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_not(grit_not: GritNot, pattern: AnyGritPattern) -> GritPatternNot {
    GritPatternNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_NOT,
        [
            Some(SyntaxElement::Node(grit_not.into_syntax())),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_or(
    or_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    patterns: GritPatternList,
    r_curly_token: SyntaxToken,
) -> GritPatternOr {
    GritPatternOr::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_OR,
        [
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(patterns.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_pattern_or_else(
    orelse_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPatternOrElseBuilder {
    GritPatternOrElseBuilder {
        orelse_token,
        l_curly_token,
        r_curly_token,
        patterns: None,
    }
}
pub struct GritPatternOrElseBuilder {
    orelse_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    patterns: Option<GritPatternList>,
}
impl GritPatternOrElseBuilder {
    pub fn with_patterns(mut self, patterns: GritPatternList) -> Self {
        self.patterns = Some(patterns);
        self
    }
    pub fn build(self) -> GritPatternOrElse {
        GritPatternOrElse::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_OR_ELSE,
            [
                Some(SyntaxElement::Token(self.orelse_token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.patterns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_pattern_where(
    pattern: AnyGritPattern,
    where_token: SyntaxToken,
    side_condition: AnyGritPredicate,
) -> GritPatternWhere {
    GritPatternWhere::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_WHERE,
        [
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(where_token)),
            Some(SyntaxElement::Node(side_condition.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_accumulate(
    left: GritVariable,
    add_assign_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateAccumulate {
    GritPredicateAccumulate::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_ACCUMULATE,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(add_assign_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_and(
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPredicateAndBuilder {
    GritPredicateAndBuilder {
        l_curly_token,
        r_curly_token,
        and_token: None,
        predicates: None,
    }
}
pub struct GritPredicateAndBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    and_token: Option<SyntaxToken>,
    predicates: Option<GritPredicateList>,
}
impl GritPredicateAndBuilder {
    pub fn with_and_token(mut self, and_token: SyntaxToken) -> Self {
        self.and_token = Some(and_token);
        self
    }
    pub fn with_predicates(mut self, predicates: GritPredicateList) -> Self {
        self.predicates = Some(predicates);
        self
    }
    pub fn build(self) -> GritPredicateAnd {
        GritPredicateAnd::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_AND,
            [
                self.and_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.predicates
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_predicate_any(
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPredicateAnyBuilder {
    GritPredicateAnyBuilder {
        any_token,
        l_curly_token,
        r_curly_token,
        predicates: None,
    }
}
pub struct GritPredicateAnyBuilder {
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    predicates: Option<GritPredicateList>,
}
impl GritPredicateAnyBuilder {
    pub fn with_predicates(mut self, predicates: GritPredicateList) -> Self {
        self.predicates = Some(predicates);
        self
    }
    pub fn build(self) -> GritPredicateAny {
        GritPredicateAny::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_ANY,
            [
                Some(SyntaxElement::Token(self.any_token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.predicates
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_predicate_assignment(
    container: AnyGritContainer,
    eq_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritPredicateAssignment {
    GritPredicateAssignment::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(container.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_call(
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> GritPredicateCallBuilder {
    GritPredicateCallBuilder {
        name,
        l_paren_token,
        r_paren_token,
        named_args: None,
    }
}
pub struct GritPredicateCallBuilder {
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    named_args: Option<GritNamedArgList>,
}
impl GritPredicateCallBuilder {
    pub fn with_named_args(mut self, named_args: GritNamedArgList) -> Self {
        self.named_args = Some(named_args);
        self
    }
    pub fn build(self) -> GritPredicateCall {
        GritPredicateCall::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_CALL,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.named_args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn grit_predicate_definition(
    predicate_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritCurlyPredicateList,
) -> GritPredicateDefinitionBuilder {
    GritPredicateDefinitionBuilder {
        predicate_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        args: None,
    }
}
pub struct GritPredicateDefinitionBuilder {
    predicate_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: GritCurlyPredicateList,
    args: Option<GritPatternArgList>,
}
impl GritPredicateDefinitionBuilder {
    pub fn with_args(mut self, args: GritPatternArgList) -> Self {
        self.args = Some(args);
        self
    }
    pub fn build(self) -> GritPredicateDefinition {
        GritPredicateDefinition::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_DEFINITION,
            [
                Some(SyntaxElement::Token(self.predicate_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn grit_predicate_else_clause(
    else_token: SyntaxToken,
    else_predicate: AnyGritPredicate,
) -> GritPredicateElseClause {
    GritPredicateElseClause::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(else_predicate.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_equal(
    left: GritVariable,
    equality_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateEqual {
    GritPredicateEqual::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_EQUAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(equality_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_greater(
    left: GritVariable,
    r_angle_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateGreater {
    GritPredicateGreater::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_GREATER,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_greater_equal(
    left: GritVariable,
    greater_than_equal_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateGreaterEqual {
    GritPredicateGreaterEqual::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_GREATER_EQUAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(greater_than_equal_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_if_else(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_predicate: AnyGritPredicate,
) -> GritPredicateIfElseBuilder {
    GritPredicateIfElseBuilder {
        if_token,
        l_paren_token,
        if_predicate,
        r_paren_token,
        then_predicate,
        grit_predicate_else_clause: None,
    }
}
pub struct GritPredicateIfElseBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_predicate: AnyGritPredicate,
    grit_predicate_else_clause: Option<GritPredicateElseClause>,
}
impl GritPredicateIfElseBuilder {
    pub fn with_grit_predicate_else_clause(
        mut self,
        grit_predicate_else_clause: GritPredicateElseClause,
    ) -> Self {
        self.grit_predicate_else_clause = Some(grit_predicate_else_clause);
        self
    }
    pub fn build(self) -> GritPredicateIfElse {
        GritPredicateIfElse::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_IF_ELSE,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.if_predicate.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.then_predicate.into_syntax())),
                self.grit_predicate_else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_predicate_less(
    left: GritVariable,
    l_angle_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateLess {
    GritPredicateLess::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_LESS,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_less_equal(
    left: GritVariable,
    less_than_equal_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateLessEqual {
    GritPredicateLessEqual::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_LESS_EQUAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(less_than_equal_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_match(
    left: GritPredicateMatchSubject,
    match_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateMatch {
    GritPredicateMatch::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_MATCH,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(match_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_maybe(
    maybe_token: SyntaxToken,
    predicate: AnyGritPredicate,
) -> GritPredicateMaybe {
    GritPredicateMaybe::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_MAYBE,
        [
            Some(SyntaxElement::Token(maybe_token)),
            Some(SyntaxElement::Node(predicate.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_not(grit_not: GritNot, predicate: AnyGritPredicate) -> GritPredicateNot {
    GritPredicateNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_NOT,
        [
            Some(SyntaxElement::Node(grit_not.into_syntax())),
            Some(SyntaxElement::Node(predicate.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_not_equal(
    left: GritVariable,
    inequality_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateNotEqual {
    GritPredicateNotEqual::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_NOT_EQUAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(inequality_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_or(
    or_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> GritPredicateOrBuilder {
    GritPredicateOrBuilder {
        or_token,
        l_curly_token,
        r_curly_token,
        predicates: None,
    }
}
pub struct GritPredicateOrBuilder {
    or_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    predicates: Option<GritPredicateList>,
}
impl GritPredicateOrBuilder {
    pub fn with_predicates(mut self, predicates: GritPredicateList) -> Self {
        self.predicates = Some(predicates);
        self
    }
    pub fn build(self) -> GritPredicateOr {
        GritPredicateOr::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_OR,
            [
                Some(SyntaxElement::Token(self.or_token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.predicates
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_predicate_return(
    return_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritPredicateReturn {
    GritPredicateReturn::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_RETURN,
        [
            Some(SyntaxElement::Token(return_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_predicate_rewrite(
    left: GritVariable,
    fat_arrow_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritPredicateRewriteBuilder {
    GritPredicateRewriteBuilder {
        left,
        fat_arrow_token,
        right,
        annotation: None,
    }
}
pub struct GritPredicateRewriteBuilder {
    left: GritVariable,
    fat_arrow_token: SyntaxToken,
    right: AnyGritPattern,
    annotation: Option<GritAnnotation>,
}
impl GritPredicateRewriteBuilder {
    pub fn with_annotation(mut self, annotation: GritAnnotation) -> Self {
        self.annotation = Some(annotation);
        self
    }
    pub fn build(self) -> GritPredicateRewrite {
        GritPredicateRewrite::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_REWRITE,
            [
                Some(SyntaxElement::Node(self.left.into_syntax())),
                self.annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.fat_arrow_token)),
                Some(SyntaxElement::Node(self.right.into_syntax())),
            ],
        ))
    }
}
pub fn grit_raw_backtick_snippet(value_token: SyntaxToken) -> GritRawBacktickSnippet {
    GritRawBacktickSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_RAW_BACKTICK_SNIPPET,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_regex_literal(value_token: SyntaxToken) -> GritRegexLiteral {
    GritRegexLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_REGEX_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_regex_pattern(regex: GritRegex) -> GritRegexPatternBuilder {
    GritRegexPatternBuilder {
        regex,
        variables: None,
    }
}
pub struct GritRegexPatternBuilder {
    regex: GritRegex,
    variables: Option<GritRegexPatternVariables>,
}
impl GritRegexPatternBuilder {
    pub fn with_variables(mut self, variables: GritRegexPatternVariables) -> Self {
        self.variables = Some(variables);
        self
    }
    pub fn build(self) -> GritRegexPattern {
        GritRegexPattern::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_REGEX_PATTERN,
            [
                Some(SyntaxElement::Node(self.regex.into_syntax())),
                self.variables
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_regex_pattern_variables(
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> GritRegexPatternVariablesBuilder {
    GritRegexPatternVariablesBuilder {
        l_paren_token,
        r_paren_token,
        grit_pattern_arg_list: None,
    }
}
pub struct GritRegexPatternVariablesBuilder {
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    grit_pattern_arg_list: Option<GritPatternArgList>,
}
impl GritRegexPatternVariablesBuilder {
    pub fn with_grit_pattern_arg_list(mut self, grit_pattern_arg_list: GritPatternArgList) -> Self {
        self.grit_pattern_arg_list = Some(grit_pattern_arg_list);
        self
    }
    pub fn build(self) -> GritRegexPatternVariables {
        GritRegexPatternVariables::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_REGEX_PATTERN_VARIABLES,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.grit_pattern_arg_list
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn grit_rewrite(
    left: AnyGritPattern,
    fat_arrow_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritRewriteBuilder {
    GritRewriteBuilder {
        left,
        fat_arrow_token,
        right,
        annotation: None,
    }
}
pub struct GritRewriteBuilder {
    left: AnyGritPattern,
    fat_arrow_token: SyntaxToken,
    right: AnyGritPattern,
    annotation: Option<GritAnnotation>,
}
impl GritRewriteBuilder {
    pub fn with_annotation(mut self, annotation: GritAnnotation) -> Self {
        self.annotation = Some(annotation);
        self
    }
    pub fn build(self) -> GritRewrite {
        GritRewrite::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_REWRITE,
            [
                Some(SyntaxElement::Node(self.left.into_syntax())),
                self.annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.fat_arrow_token)),
                Some(SyntaxElement::Node(self.right.into_syntax())),
            ],
        ))
    }
}
pub fn grit_root() -> GritRootBuilder {
    GritRootBuilder {
        version: None,
        language: None,
        definitions: None,
        pattern: None,
        definitions_continued: None,
    }
}
pub struct GritRootBuilder {
    version: Option<GritVersion>,
    language: Option<GritLanguageDeclaration>,
    definitions: Option<GritDefinitionList>,
    pattern: Option<AnyGritPattern>,
    definitions_continued: Option<GritDefinitionList>,
}
impl GritRootBuilder {
    pub fn with_version(mut self, version: GritVersion) -> Self {
        self.version = Some(version);
        self
    }
    pub fn with_language(mut self, language: GritLanguageDeclaration) -> Self {
        self.language = Some(language);
        self
    }
    pub fn with_definitions(mut self, definitions: GritDefinitionList) -> Self {
        self.definitions = Some(definitions);
        self
    }
    pub fn with_pattern(mut self, pattern: AnyGritPattern) -> Self {
        self.pattern = Some(pattern);
        self
    }
    pub fn with_definitions_continued(mut self, definitions_continued: GritDefinitionList) -> Self {
        self.definitions_continued = Some(definitions_continued);
        self
    }
    pub fn build(self) -> GritRoot {
        GritRoot::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_ROOT,
            [
                self.version
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.language
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.definitions
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.pattern
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.definitions_continued
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_sequential(
    sequential_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    sequential: GritSequentialList,
    r_curly_token: SyntaxToken,
) -> GritSequential {
    GritSequential::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SEQUENTIAL,
        [
            Some(SyntaxElement::Token(sequential_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(sequential.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_snippet_regex(value_token: SyntaxToken) -> GritSnippetRegex {
    GritSnippetRegex::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SNIPPET_REGEX,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_some(some_token: SyntaxToken, pattern: MaybeCurlyGritPattern) -> GritSome {
    GritSome::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SOME,
        [
            Some(SyntaxElement::Token(some_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_string_literal(value_token: SyntaxToken) -> GritStringLiteral {
    GritStringLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_STRING_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_sub_operation(
    left: AnyGritPattern,
    minus_token: SyntaxToken,
    right: AnyGritPattern,
) -> GritSubOperation {
    GritSubOperation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SUB_OPERATION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(minus_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn grit_undefined(undefined_token: SyntaxToken) -> GritUndefined {
    GritUndefined::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_UNDEFINED,
        [Some(SyntaxElement::Token(undefined_token))],
    ))
}
pub fn grit_underscore(dollar_underscore_token: SyntaxToken) -> GritUnderscore {
    GritUnderscore::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_UNDERSCORE,
        [Some(SyntaxElement::Token(dollar_underscore_token))],
    ))
}
pub fn grit_variable(grit_variable_token: SyntaxToken) -> GritVariable {
    GritVariable::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_VARIABLE,
        [Some(SyntaxElement::Token(grit_variable_token))],
    ))
}
pub fn grit_version(
    engine_token: SyntaxToken,
    biome_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    grit_double_literal: GritDoubleLiteral,
    r_paren_token: SyntaxToken,
) -> GritVersion {
    GritVersion::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_VERSION,
        [
            Some(SyntaxElement::Token(engine_token)),
            Some(SyntaxElement::Token(biome_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(grit_double_literal.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_within(within_token: SyntaxToken, pattern: MaybeCurlyGritPattern) -> GritWithin {
    GritWithin::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_WITHIN,
        [
            Some(SyntaxElement::Token(within_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_definition_list<I, S>(items: I, separators: S) -> GritDefinitionList
where
    I: IntoIterator<Item = AnyGritDefinition>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_DEFINITION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_files_list<I, S>(items: I, separators: S) -> GritFilesList
where
    I: IntoIterator<Item = AnyGritPattern>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritFilesList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_FILES_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_language_flavor_list<I, S>(items: I, separators: S) -> GritLanguageFlavorList
where
    I: IntoIterator<Item = GritLanguageFlavorKind>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritLanguageFlavorList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_FLAVOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_list_pattern_list<I, S>(items: I, separators: S) -> GritListPatternList
where
    I: IntoIterator<Item = AnyGritListPattern>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritListPatternList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LIST_PATTERN_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_map_element_list<I, S>(items: I, separators: S) -> GritMapElementList
where
    I: IntoIterator<Item = GritMapElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritMapElementList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MAP_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_named_arg_list<I, S>(items: I, separators: S) -> GritNamedArgList
where
    I: IntoIterator<Item = GritNamedArg>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritNamedArgList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAMED_ARG_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_pattern_list<I, S>(items: I, separators: S) -> GritPatternList
where
    I: IntoIterator<Item = AnyGritPattern>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritPatternList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_predicate_list<I, S>(items: I, separators: S) -> GritPredicateList
where
    I: IntoIterator<Item = AnyGritPredicate>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritPredicateList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_sequential_list<I, S>(items: I, separators: S) -> GritSequentialList
where
    I: IntoIterator<Item = AnyGritPattern>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritSequentialList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SEQUENTIAL_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_variable_list<I, S>(items: I, separators: S) -> GritVariableList
where
    I: IntoIterator<Item = GritVariable>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GritSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GritVariableList::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_VARIABLE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn grit_bogus<I>(slots: I) -> GritBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogus::unwrap_cast(SyntaxNode::new_detached(GritSyntaxKind::GRIT_BOGUS, slots))
}
pub fn grit_bogus_definition<I>(slots: I) -> GritBogusDefinition
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusDefinition::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_DEFINITION,
        slots,
    ))
}
pub fn grit_bogus_literal<I>(slots: I) -> GritBogusLiteral
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_LITERAL,
        slots,
    ))
}
pub fn grit_bogus_named_arg<I>(slots: I) -> GritBogusNamedArg
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusNamedArg::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_NAMED_ARG,
        slots,
    ))
}
pub fn grit_bogus_pattern<I>(slots: I) -> GritBogusPattern
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_PATTERN,
        slots,
    ))
}
pub fn grit_bogus_predicate<I>(slots: I) -> GritBogusPredicate
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusPredicate::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_PREDICATE,
        slots,
    ))
}
