//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_grit_syntax::{
    GritSyntaxElement as SyntaxElement, GritSyntaxNode as SyntaxNode,
    GritSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
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
pub fn grit_annotation(value_token: SyntaxToken) -> GritAnnotation {
    GritAnnotation::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_ANNOTATION,
        [Some(SyntaxElement::Token(value_token))],
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
pub fn grit_backtick_snippet_literal(value_token: SyntaxToken) -> GritBacktickSnippetLiteral {
    GritBacktickSnippetLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BACKTICK_SNIPPET_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_boolean_literal(value_token: SyntaxToken) -> GritBooleanLiteral {
    GritBooleanLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOOLEAN_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_bracketed_pattern(
    l_paren_token: SyntaxToken,
    pattern: AnyGritPattern,
    r_paren_token: SyntaxToken,
) -> GritBracketedPattern {
    GritBracketedPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BRACKETED_PATTERN,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_bracketed_predicate(
    l_paren_token: SyntaxToken,
    predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
) -> GritBracketedPredicate {
    GritBracketedPredicate::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BRACKETED_PREDICATE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(predicate.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_bubble(
    bubble_token: SyntaxToken,
    pattern: AnyGritMaybeCurlyPattern,
) -> GritBubbleBuilder {
    GritBubbleBuilder {
        bubble_token,
        pattern,
        scope: None,
    }
}
pub struct GritBubbleBuilder {
    bubble_token: SyntaxToken,
    pattern: AnyGritMaybeCurlyPattern,
    scope: Option<GritBubbleScope>,
}
impl GritBubbleBuilder {
    pub fn with_scope(mut self, scope: GritBubbleScope) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn build(self) -> GritBubble {
        GritBubble::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_BUBBLE,
            [
                Some(SyntaxElement::Token(self.bubble_token)),
                self.scope
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
            ],
        ))
    }
}
pub fn grit_bubble_scope(
    l_paren_token: SyntaxToken,
    variables: GritVariableList,
    r_paren_token: SyntaxToken,
) -> GritBubbleScope {
    GritBubbleScope::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BUBBLE_SCOPE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(variables.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_code_snippet(source: AnyGritCodeSnippetSource) -> GritCodeSnippet {
    GritCodeSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_CODE_SNIPPET,
        [Some(SyntaxElement::Node(source.into_syntax()))],
    ))
}
pub fn grit_curly_pattern(
    l_curly_token: SyntaxToken,
    pattern: AnyGritPattern,
    r_curly_token: SyntaxToken,
) -> GritCurlyPattern {
    GritCurlyPattern::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_CURLY_PATTERN,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
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
pub fn grit_dotdotdot(dotdotdot_token: SyntaxToken) -> GritDotdotdotBuilder {
    GritDotdotdotBuilder {
        dotdotdot_token,
        pattern: None,
    }
}
pub struct GritDotdotdotBuilder {
    dotdotdot_token: SyntaxToken,
    pattern: Option<AnyGritMaybeCurlyPattern>,
}
impl GritDotdotdotBuilder {
    pub fn with_pattern(mut self, pattern: AnyGritMaybeCurlyPattern) -> Self {
        self.pattern = Some(pattern);
        self
    }
    pub fn build(self) -> GritDotdotdot {
        GritDotdotdot::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_DOTDOTDOT,
            [
                Some(SyntaxElement::Token(self.dotdotdot_token)),
                self.pattern
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
pub fn grit_engine_name(engine_kind_token: SyntaxToken) -> GritEngineName {
    GritEngineName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_ENGINE_NAME,
        [Some(SyntaxElement::Token(engine_kind_token))],
    ))
}
pub fn grit_every(every_token: SyntaxToken, pattern: AnyGritMaybeCurlyPattern) -> GritEvery {
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
    files: GritPatternList,
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
    args: GritVariableList,
    r_paren_token: SyntaxToken,
    body: GritPredicateCurly,
) -> GritFunctionDefinition {
    GritFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_FUNCTION_DEFINITION,
        [
            Some(SyntaxElement::Token(function_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn grit_int_literal(value_token: SyntaxToken) -> GritIntLiteral {
    GritIntLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_INT_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_javascript_body_wrapper(value_token: SyntaxToken) -> GritJavascriptBodyWrapper {
    GritJavascriptBodyWrapper::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_JAVASCRIPT_BODY_WRAPPER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_javascript_function_definition(
    function_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    args: GritVariableList,
    r_paren_token: SyntaxToken,
    js_token: SyntaxToken,
    grit_javascript_body_wrapper: GritJavascriptBodyWrapper,
) -> GritJavascriptFunctionDefinition {
    GritJavascriptFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_JAVASCRIPT_FUNCTION_DEFINITION,
        [
            Some(SyntaxElement::Token(function_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Token(js_token)),
            Some(SyntaxElement::Node(
                grit_javascript_body_wrapper.into_syntax(),
            )),
        ],
    ))
}
pub fn grit_language_declaration(
    language_token: SyntaxToken,
    name: AnyGritLanguageName,
) -> GritLanguageDeclarationBuilder {
    GritLanguageDeclarationBuilder {
        language_token,
        name,
        flavor: None,
        semicolon_token: None,
    }
}
pub struct GritLanguageDeclarationBuilder {
    language_token: SyntaxToken,
    name: AnyGritLanguageName,
    flavor: Option<GritLanguageFlavor>,
    semicolon_token: Option<SyntaxToken>,
}
impl GritLanguageDeclarationBuilder {
    pub fn with_flavor(mut self, flavor: GritLanguageFlavor) -> Self {
        self.flavor = Some(flavor);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
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
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn grit_language_flavor(
    l_paren_token: SyntaxToken,
    flavors: GritLanguageFlavorList,
    r_paren_token: SyntaxToken,
) -> GritLanguageFlavor {
    GritLanguageFlavor::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_FLAVOR,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(flavors.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_language_flavor_kind(flavor_kind_token: SyntaxToken) -> GritLanguageFlavorKind {
    GritLanguageFlavorKind::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_FLAVOR_KIND,
        [Some(SyntaxElement::Token(flavor_kind_token))],
    ))
}
pub fn grit_language_name(language_kind_token: SyntaxToken) -> GritLanguageName {
    GritLanguageName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_NAME,
        [Some(SyntaxElement::Token(language_kind_token))],
    ))
}
pub fn grit_language_specific_snippet(
    language: AnyGritLanguageName,
    snippet_token: SyntaxToken,
) -> GritLanguageSpecificSnippet {
    GritLanguageSpecificSnippet::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_LANGUAGE_SPECIFIC_SNIPPET,
        [
            Some(SyntaxElement::Node(language.into_syntax())),
            Some(SyntaxElement::Token(snippet_token)),
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
        threshold: None,
    }
}
pub struct GritLikeBuilder {
    like_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    example: AnyGritPattern,
    r_curly_token: SyntaxToken,
    threshold: Option<GritLikeThreshold>,
}
impl GritLikeBuilder {
    pub fn with_threshold(mut self, threshold: GritLikeThreshold) -> Self {
        self.threshold = Some(threshold);
        self
    }
    pub fn build(self) -> GritLike {
        GritLike::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LIKE,
            [
                Some(SyntaxElement::Token(self.like_token)),
                self.threshold
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
pub fn grit_list(
    l_brack_token: SyntaxToken,
    patterns: GritListPatternList,
    r_brack_token: SyntaxToken,
) -> GritListBuilder {
    GritListBuilder {
        l_brack_token,
        patterns,
        r_brack_token,
        name: None,
    }
}
pub struct GritListBuilder {
    l_brack_token: SyntaxToken,
    patterns: GritListPatternList,
    r_brack_token: SyntaxToken,
    name: Option<GritName>,
}
impl GritListBuilder {
    pub fn with_name(mut self, name: GritName) -> Self {
        self.name = Some(name);
        self
    }
    pub fn build(self) -> GritList {
        GritList::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_LIST,
            [
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.patterns.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn grit_list_accessor(
    list: AnyGritListAccessorSubject,
    l_brack_token: SyntaxToken,
    index: AnyGritListIndex,
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
pub fn grit_map(
    l_curly_token: SyntaxToken,
    elements: GritMapElementList,
    r_curly_token: SyntaxToken,
) -> GritMap {
    GritMap::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_MAP,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_map_accessor(
    map: AnyGritMapAccessorSubject,
    dot_token: SyntaxToken,
    key: AnyGritMapKey,
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
pub fn grit_name(value_token: SyntaxToken) -> GritName {
    GritName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_named_arg(
    name: GritName,
    eq_token: SyntaxToken,
    pattern: AnyGritPattern,
) -> GritNamedArg {
    GritNamedArg::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NAMED_ARG,
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
    named_args: GritNamedArgList,
    r_paren_token: SyntaxToken,
) -> GritNodeLike {
    GritNodeLike::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NODE_LIKE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(named_args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_not(token_token: SyntaxToken) -> GritNot {
    GritNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_NOT,
        [Some(SyntaxElement::Token(token_token))],
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
    patterns: GritPatternList,
    r_curly_token: SyntaxToken,
) -> GritPatternAnd {
    GritPatternAnd::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_AND,
        [
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(patterns.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_pattern_any(
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    patterns: GritPatternList,
    r_curly_token: SyntaxToken,
) -> GritPatternAny {
    GritPatternAny::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_ANY,
        [
            Some(SyntaxElement::Token(any_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(patterns.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
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
    contains: AnyGritMaybeCurlyPattern,
) -> GritPatternContainsBuilder {
    GritPatternContainsBuilder {
        contains_token,
        contains,
        until_clause: None,
    }
}
pub struct GritPatternContainsBuilder {
    contains_token: SyntaxToken,
    contains: AnyGritMaybeCurlyPattern,
    until_clause: Option<GritPatternUntilClause>,
}
impl GritPatternContainsBuilder {
    pub fn with_until_clause(mut self, until_clause: GritPatternUntilClause) -> Self {
        self.until_clause = Some(until_clause);
        self
    }
    pub fn build(self) -> GritPatternContains {
        GritPatternContains::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PATTERN_CONTAINS,
            [
                Some(SyntaxElement::Token(self.contains_token)),
                Some(SyntaxElement::Node(self.contains.into_syntax())),
                self.until_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_pattern_definition(
    pattern_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    args: GritVariableList,
    r_paren_token: SyntaxToken,
    body: GritPatternDefinitionBody,
) -> GritPatternDefinitionBuilder {
    GritPatternDefinitionBuilder {
        pattern_token,
        name,
        l_paren_token,
        args,
        r_paren_token,
        body,
        visibility_token: None,
        language: None,
    }
}
pub struct GritPatternDefinitionBuilder {
    pattern_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    args: GritVariableList,
    r_paren_token: SyntaxToken,
    body: GritPatternDefinitionBody,
    visibility_token: Option<SyntaxToken>,
    language: Option<GritLanguageDeclaration>,
}
impl GritPatternDefinitionBuilder {
    pub fn with_visibility_token(mut self, visibility_token: SyntaxToken) -> Self {
        self.visibility_token = Some(visibility_token);
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
                Some(SyntaxElement::Node(self.args.into_syntax())),
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
    patterns: GritPatternList,
    r_curly_token: SyntaxToken,
) -> GritPatternDefinitionBody {
    GritPatternDefinitionBody::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_DEFINITION_BODY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(patterns.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_pattern_else_clause(
    else_token: SyntaxToken,
    else_pattern: AnyGritMaybeCurlyPattern,
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
    then_pattern: AnyGritMaybeCurlyPattern,
) -> GritPatternIfElseBuilder {
    GritPatternIfElseBuilder {
        if_token,
        l_paren_token,
        if_predicate,
        r_paren_token,
        then_pattern,
        else_clause: None,
    }
}
pub struct GritPatternIfElseBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_pattern: AnyGritMaybeCurlyPattern,
    else_clause: Option<GritPatternElseClause>,
}
impl GritPatternIfElseBuilder {
    pub fn with_else_clause(mut self, else_clause: GritPatternElseClause) -> Self {
        self.else_clause = Some(else_clause);
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
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn grit_pattern_includes(
    includes_token: SyntaxToken,
    includes: AnyGritMaybeCurlyPattern,
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
    pattern: AnyGritMaybeCurlyPattern,
) -> GritPatternMaybe {
    GritPatternMaybe::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_MAYBE,
        [
            Some(SyntaxElement::Token(maybe_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn grit_pattern_not(not: GritNot, pattern: AnyGritPattern) -> GritPatternNot {
    GritPatternNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_NOT,
        [
            Some(SyntaxElement::Node(not.into_syntax())),
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
    patterns: GritPatternList,
    r_curly_token: SyntaxToken,
) -> GritPatternOrElse {
    GritPatternOrElse::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_OR_ELSE,
        [
            Some(SyntaxElement::Token(orelse_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(patterns.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_pattern_until_clause(
    until_token: SyntaxToken,
    until: AnyGritPattern,
) -> GritPatternUntilClause {
    GritPatternUntilClause::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PATTERN_UNTIL_CLAUSE,
        [
            Some(SyntaxElement::Token(until_token)),
            Some(SyntaxElement::Node(until.into_syntax())),
        ],
    ))
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
    predicates: GritPredicateList,
    r_curly_token: SyntaxToken,
) -> GritPredicateAndBuilder {
    GritPredicateAndBuilder {
        l_curly_token,
        predicates,
        r_curly_token,
        and_token: None,
    }
}
pub struct GritPredicateAndBuilder {
    l_curly_token: SyntaxToken,
    predicates: GritPredicateList,
    r_curly_token: SyntaxToken,
    and_token: Option<SyntaxToken>,
}
impl GritPredicateAndBuilder {
    pub fn with_and_token(mut self, and_token: SyntaxToken) -> Self {
        self.and_token = Some(and_token);
        self
    }
    pub fn build(self) -> GritPredicateAnd {
        GritPredicateAnd::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_PREDICATE_AND,
            [
                self.and_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.predicates.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn grit_predicate_any(
    any_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    predicates: GritPredicateList,
    r_curly_token: SyntaxToken,
) -> GritPredicateAny {
    GritPredicateAny::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_ANY,
        [
            Some(SyntaxElement::Token(any_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(predicates.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
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
    named_args: GritNamedArgList,
    r_paren_token: SyntaxToken,
) -> GritPredicateCall {
    GritPredicateCall::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_CALL,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(named_args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_predicate_curly(
    l_curly_token: SyntaxToken,
    predicates: GritPredicateList,
    r_curly_token: SyntaxToken,
) -> GritPredicateCurly {
    GritPredicateCurly::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_CURLY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(predicates.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn grit_predicate_definition(
    predicate_token: SyntaxToken,
    name: GritName,
    l_paren_token: SyntaxToken,
    args: GritVariableList,
    r_paren_token: SyntaxToken,
    body: GritPredicateCurly,
) -> GritPredicateDefinition {
    GritPredicateDefinition::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_DEFINITION,
        [
            Some(SyntaxElement::Token(predicate_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
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
        else_clause: None,
    }
}
pub struct GritPredicateIfElseBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    if_predicate: AnyGritPredicate,
    r_paren_token: SyntaxToken,
    then_predicate: AnyGritPredicate,
    else_clause: Option<GritPredicateElseClause>,
}
impl GritPredicateIfElseBuilder {
    pub fn with_else_clause(mut self, else_clause: GritPredicateElseClause) -> Self {
        self.else_clause = Some(else_clause);
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
                self.else_clause
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
    left: AnyGritPredicateMatchSubject,
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
pub fn grit_predicate_not(not: GritNot, predicate: AnyGritPredicate) -> GritPredicateNot {
    GritPredicateNot::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_NOT,
        [
            Some(SyntaxElement::Node(not.into_syntax())),
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
    predicates: GritPredicateList,
    r_curly_token: SyntaxToken,
) -> GritPredicateOr {
    GritPredicateOr::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_PREDICATE_OR,
        [
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(predicates.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
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
pub fn grit_raw_backtick_snippet_literal(
    value_token: SyntaxToken,
) -> GritRawBacktickSnippetLiteral {
    GritRawBacktickSnippetLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_RAW_BACKTICK_SNIPPET_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_regex_literal(value_token: SyntaxToken) -> GritRegexLiteral {
    GritRegexLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_REGEX_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_regex_pattern(regex: AnyGritRegex) -> GritRegexPatternBuilder {
    GritRegexPatternBuilder {
        regex,
        variables: None,
    }
}
pub struct GritRegexPatternBuilder {
    regex: AnyGritRegex,
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
    args: GritVariableList,
    r_paren_token: SyntaxToken,
) -> GritRegexPatternVariables {
    GritRegexPatternVariables::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_REGEX_PATTERN_VARIABLES,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
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
pub fn grit_root(definitions: GritDefinitionList, eof_token: SyntaxToken) -> GritRootBuilder {
    GritRootBuilder {
        definitions,
        eof_token,
        bom_token: None,
        version: None,
        language: None,
    }
}
pub struct GritRootBuilder {
    definitions: GritDefinitionList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
    version: Option<AnyGritVersion>,
    language: Option<AnyGritLanguageDeclaration>,
}
impl GritRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn with_version(mut self, version: AnyGritVersion) -> Self {
        self.version = Some(version);
        self
    }
    pub fn with_language(mut self, language: AnyGritLanguageDeclaration) -> Self {
        self.language = Some(language);
        self
    }
    pub fn build(self) -> GritRoot {
        GritRoot::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                self.version
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.language
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.definitions.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn grit_sequential(
    sequential_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    sequential: GritPatternList,
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
pub fn grit_snippet_regex_literal(value_token: SyntaxToken) -> GritSnippetRegexLiteral {
    GritSnippetRegexLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_SNIPPET_REGEX_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_some(some_token: SyntaxToken, pattern: AnyGritMaybeCurlyPattern) -> GritSome {
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
pub fn grit_undefined_literal(token_token: SyntaxToken) -> GritUndefinedLiteral {
    GritUndefinedLiteral::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_UNDEFINED_LITERAL,
        [Some(SyntaxElement::Token(token_token))],
    ))
}
pub fn grit_underscore(token_token: SyntaxToken) -> GritUnderscore {
    GritUnderscore::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_UNDERSCORE,
        [Some(SyntaxElement::Token(token_token))],
    ))
}
pub fn grit_variable(value_token: SyntaxToken) -> GritVariable {
    GritVariable::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_VARIABLE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn grit_version(
    engine_token: SyntaxToken,
    engine_name: GritEngineName,
    l_paren_token: SyntaxToken,
    version: GritDoubleLiteral,
    r_paren_token: SyntaxToken,
) -> GritVersion {
    GritVersion::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_VERSION,
        [
            Some(SyntaxElement::Token(engine_token)),
            Some(SyntaxElement::Node(engine_name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(version.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn grit_within(
    within_token: SyntaxToken,
    pattern: AnyGritMaybeCurlyPattern,
) -> GritWithinBuilder {
    GritWithinBuilder {
        within_token,
        pattern,
        until_clause: None,
    }
}
pub struct GritWithinBuilder {
    within_token: SyntaxToken,
    pattern: AnyGritMaybeCurlyPattern,
    until_clause: Option<GritPatternUntilClause>,
}
impl GritWithinBuilder {
    pub fn with_until_clause(mut self, until_clause: GritPatternUntilClause) -> Self {
        self.until_clause = Some(until_clause);
        self
    }
    pub fn build(self) -> GritWithin {
        GritWithin::unwrap_cast(SyntaxNode::new_detached(
            GritSyntaxKind::GRIT_WITHIN,
            [
                Some(SyntaxElement::Token(self.within_token)),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
                self.until_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
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
pub fn grit_language_flavor_list<I, S>(items: I, separators: S) -> GritLanguageFlavorList
where
    I: IntoIterator<Item = AnyGritLanguageFlavorKind>,
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
    I: IntoIterator<Item = AnyGritMapElement>,
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
    I: IntoIterator<Item = AnyGritMaybeNamedArg>,
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
pub fn grit_bogus_container<I>(slots: I) -> GritBogusContainer
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusContainer::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_CONTAINER,
        slots,
    ))
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
pub fn grit_bogus_language_declaration<I>(slots: I) -> GritBogusLanguageDeclaration
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusLanguageDeclaration::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_LANGUAGE_DECLARATION,
        slots,
    ))
}
pub fn grit_bogus_language_flavor_kind<I>(slots: I) -> GritBogusLanguageFlavorKind
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusLanguageFlavorKind::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_LANGUAGE_FLAVOR_KIND,
        slots,
    ))
}
pub fn grit_bogus_language_name<I>(slots: I) -> GritBogusLanguageName
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusLanguageName::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_LANGUAGE_NAME,
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
pub fn grit_bogus_map_element<I>(slots: I) -> GritBogusMapElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusMapElement::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_MAP_ELEMENT,
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
pub fn grit_bogus_version<I>(slots: I) -> GritBogusVersion
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GritBogusVersion::unwrap_cast(SyntaxNode::new_detached(
        GritSyntaxKind::GRIT_BOGUS_VERSION,
        slots,
    ))
}
