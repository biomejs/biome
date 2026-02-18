//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_html_syntax::{
    HtmlSyntaxElement as SyntaxElement, HtmlSyntaxNode as SyntaxNode,
    HtmlSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn astro_embedded_content() -> AstroEmbeddedContentBuilder {
    AstroEmbeddedContentBuilder {
        content_token: None,
    }
}
pub struct AstroEmbeddedContentBuilder {
    content_token: Option<SyntaxToken>,
}
impl AstroEmbeddedContentBuilder {
    pub fn with_content_token(mut self, content_token: SyntaxToken) -> Self {
        self.content_token = Some(content_token);
        self
    }
    pub fn build(self) -> AstroEmbeddedContent {
        AstroEmbeddedContent::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::ASTRO_EMBEDDED_CONTENT,
            [self.content_token.map(|token| SyntaxElement::Token(token))],
        ))
    }
}
pub fn astro_frontmatter_element(
    l_fence_token: SyntaxToken,
    content: AstroEmbeddedContent,
    r_fence_token: SyntaxToken,
) -> AstroFrontmatterElement {
    AstroFrontmatterElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::ASTRO_FRONTMATTER_ELEMENT,
        [
            Some(SyntaxElement::Token(l_fence_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_fence_token)),
        ],
    ))
}
pub fn html_attribute(name: HtmlAttributeName) -> HtmlAttributeBuilder {
    HtmlAttributeBuilder {
        name,
        initializer: None,
    }
}
pub struct HtmlAttributeBuilder {
    name: HtmlAttributeName,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl HtmlAttributeBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> HtmlAttribute {
        HtmlAttribute::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_ATTRIBUTE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn html_attribute_double_text_expression(
    l_double_curly_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_double_curly_token: SyntaxToken,
) -> HtmlAttributeDoubleTextExpression {
    HtmlAttributeDoubleTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_double_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_double_curly_token)),
        ],
    ))
}
pub fn html_attribute_initializer_clause(
    eq_token: SyntaxToken,
    value: AnyHtmlAttributeInitializer,
) -> HtmlAttributeInitializerClause {
    HtmlAttributeInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn html_attribute_name(value_token: SyntaxToken) -> HtmlAttributeName {
    HtmlAttributeName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_attribute_single_text_expression(
    l_curly_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> HtmlAttributeSingleTextExpression {
    HtmlAttributeSingleTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn html_cdata_section(
    cdata_start_token: SyntaxToken,
    content_token: SyntaxToken,
    cdata_end_token: SyntaxToken,
) -> HtmlCdataSection {
    HtmlCdataSection::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CDATA_SECTION,
        [
            Some(SyntaxElement::Token(cdata_start_token)),
            Some(SyntaxElement::Token(content_token)),
            Some(SyntaxElement::Token(cdata_end_token)),
        ],
    ))
}
pub fn html_closing_element(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    name: AnyHtmlTagName,
    r_angle_token: SyntaxToken,
) -> HtmlClosingElement {
    HtmlClosingElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_component_name(value_token: SyntaxToken) -> HtmlComponentName {
    HtmlComponentName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_COMPONENT_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_content(value_token: SyntaxToken) -> HtmlContent {
    HtmlContent::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_CONTENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_directive(
    l_angle_token: SyntaxToken,
    excl_token: SyntaxToken,
    doctype_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> HtmlDirectiveBuilder {
    HtmlDirectiveBuilder {
        l_angle_token,
        excl_token,
        doctype_token,
        r_angle_token,
        html_token: None,
        quirk_token: None,
        public_id_token: None,
        system_id_token: None,
    }
}
pub struct HtmlDirectiveBuilder {
    l_angle_token: SyntaxToken,
    excl_token: SyntaxToken,
    doctype_token: SyntaxToken,
    r_angle_token: SyntaxToken,
    html_token: Option<SyntaxToken>,
    quirk_token: Option<SyntaxToken>,
    public_id_token: Option<SyntaxToken>,
    system_id_token: Option<SyntaxToken>,
}
impl HtmlDirectiveBuilder {
    pub fn with_html_token(mut self, html_token: SyntaxToken) -> Self {
        self.html_token = Some(html_token);
        self
    }
    pub fn with_quirk_token(mut self, quirk_token: SyntaxToken) -> Self {
        self.quirk_token = Some(quirk_token);
        self
    }
    pub fn with_public_id_token(mut self, public_id_token: SyntaxToken) -> Self {
        self.public_id_token = Some(public_id_token);
        self
    }
    pub fn with_system_id_token(mut self, system_id_token: SyntaxToken) -> Self {
        self.system_id_token = Some(system_id_token);
        self
    }
    pub fn build(self) -> HtmlDirective {
        HtmlDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Token(self.excl_token)),
                Some(SyntaxElement::Token(self.doctype_token)),
                self.html_token.map(|token| SyntaxElement::Token(token)),
                self.quirk_token.map(|token| SyntaxElement::Token(token)),
                self.public_id_token
                    .map(|token| SyntaxElement::Token(token)),
                self.system_id_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn html_double_text_expression(
    l_double_curly_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_double_curly_token: SyntaxToken,
) -> HtmlDoubleTextExpression {
    HtmlDoubleTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_DOUBLE_TEXT_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_double_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_double_curly_token)),
        ],
    ))
}
pub fn html_element(
    opening_element: HtmlOpeningElement,
    children: HtmlElementList,
    closing_element: HtmlClosingElement,
) -> HtmlElement {
    HtmlElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ELEMENT,
        [
            Some(SyntaxElement::Node(opening_element.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing_element.into_syntax())),
        ],
    ))
}
pub fn html_embedded_content(value_token: SyntaxToken) -> HtmlEmbeddedContent {
    HtmlEmbeddedContent::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_EMBEDDED_CONTENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_member_name(
    object: AnyHtmlComponentObjectName,
    dot_token: SyntaxToken,
    member: HtmlTagName,
) -> HtmlMemberName {
    HtmlMemberName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_MEMBER_NAME,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn html_opening_element(
    l_angle_token: SyntaxToken,
    name: AnyHtmlTagName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
) -> HtmlOpeningElement {
    HtmlOpeningElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_OPENING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn html_root(html: HtmlElementList, eof_token: SyntaxToken) -> HtmlRootBuilder {
    HtmlRootBuilder {
        html,
        eof_token,
        bom_token: None,
        frontmatter: None,
        directive: None,
    }
}
pub struct HtmlRootBuilder {
    html: HtmlElementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
    frontmatter: Option<AnyAstroFrontmatterElement>,
    directive: Option<HtmlDirective>,
}
impl HtmlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn with_frontmatter(mut self, frontmatter: AnyAstroFrontmatterElement) -> Self {
        self.frontmatter = Some(frontmatter);
        self
    }
    pub fn with_directive(mut self, directive: HtmlDirective) -> Self {
        self.directive = Some(directive);
        self
    }
    pub fn build(self) -> HtmlRoot {
        HtmlRoot::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                self.frontmatter
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.directive
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.html.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn html_self_closing_element(
    l_angle_token: SyntaxToken,
    name: AnyHtmlTagName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
) -> HtmlSelfClosingElementBuilder {
    HtmlSelfClosingElementBuilder {
        l_angle_token,
        name,
        attributes,
        r_angle_token,
        slash_token: None,
    }
}
pub struct HtmlSelfClosingElementBuilder {
    l_angle_token: SyntaxToken,
    name: AnyHtmlTagName,
    attributes: HtmlAttributeList,
    r_angle_token: SyntaxToken,
    slash_token: Option<SyntaxToken>,
}
impl HtmlSelfClosingElementBuilder {
    pub fn with_slash_token(mut self, slash_token: SyntaxToken) -> Self {
        self.slash_token = Some(slash_token);
        self
    }
    pub fn build(self) -> HtmlSelfClosingElement {
        HtmlSelfClosingElement::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.attributes.into_syntax())),
                self.slash_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn html_single_text_expression(
    l_curly_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> HtmlSingleTextExpression {
    HtmlSingleTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_SINGLE_TEXT_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn html_spread_attribute(
    l_curly_token: SyntaxToken,
    dotdotdot_token: SyntaxToken,
    argument: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> HtmlSpreadAttribute {
    HtmlSpreadAttribute::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_SPREAD_ATTRIBUTE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn html_string(value_token: SyntaxToken) -> HtmlString {
    HtmlString::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_tag_name(value_token: SyntaxToken) -> HtmlTagName {
    HtmlTagName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_TAG_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn html_text_expression(html_literal_token: SyntaxToken) -> HtmlTextExpression {
    HtmlTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_TEXT_EXPRESSION,
        [Some(SyntaxElement::Token(html_literal_token))],
    ))
}
pub fn svelte_animate_directive(
    animate_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteAnimateDirective {
    SvelteAnimateDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_ANIMATE_DIRECTIVE,
        [
            Some(SyntaxElement::Token(animate_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_attach_attribute(
    sv_curly_at_token: SyntaxToken,
    attach_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteAttachAttribute {
    SvelteAttachAttribute::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_ATTACH_ATTRIBUTE,
        [
            Some(SyntaxElement::Token(sv_curly_at_token)),
            Some(SyntaxElement::Token(attach_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_await_block(
    opening_block: SvelteAwaitOpeningBlock,
    clauses: SvelteAwaitClausesList,
    closing_block: SvelteAwaitClosingBlock,
) -> SvelteAwaitBlock {
    SvelteAwaitBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_BLOCK,
        [
            Some(SyntaxElement::Node(opening_block.into_syntax())),
            Some(SyntaxElement::Node(clauses.into_syntax())),
            Some(SyntaxElement::Node(closing_block.into_syntax())),
        ],
    ))
}
pub fn svelte_await_catch_block(
    sv_curly_colon_token: SyntaxToken,
    catch_token: SyntaxToken,
    name: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteAwaitCatchBlock {
    SvelteAwaitCatchBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_CATCH_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_colon_token)),
            Some(SyntaxElement::Token(catch_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_await_catch_clause(
    catch_token: SyntaxToken,
    name: HtmlTextExpression,
) -> SvelteAwaitCatchClause {
    SvelteAwaitCatchClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_CATCH_CLAUSE,
        [
            Some(SyntaxElement::Token(catch_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn svelte_await_closing_block(
    sv_curly_slash_token: SyntaxToken,
    await_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> SvelteAwaitClosingBlock {
    SvelteAwaitClosingBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_CLOSING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_slash_token)),
            Some(SyntaxElement::Token(await_token)),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_await_opening_block(
    sv_curly_hash_token: SyntaxToken,
    await_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteAwaitOpeningBlockBuilder {
    SvelteAwaitOpeningBlockBuilder {
        sv_curly_hash_token,
        await_token,
        expression,
        r_curly_token,
        children,
        then_clause: None,
        catch_clause: None,
    }
}
pub struct SvelteAwaitOpeningBlockBuilder {
    sv_curly_hash_token: SyntaxToken,
    await_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
    then_clause: Option<SvelteAwaitThenClause>,
    catch_clause: Option<SvelteAwaitCatchClause>,
}
impl SvelteAwaitOpeningBlockBuilder {
    pub fn with_then_clause(mut self, then_clause: SvelteAwaitThenClause) -> Self {
        self.then_clause = Some(then_clause);
        self
    }
    pub fn with_catch_clause(mut self, catch_clause: SvelteAwaitCatchClause) -> Self {
        self.catch_clause = Some(catch_clause);
        self
    }
    pub fn build(self) -> SvelteAwaitOpeningBlock {
        SvelteAwaitOpeningBlock::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_AWAIT_OPENING_BLOCK,
            [
                Some(SyntaxElement::Token(self.sv_curly_hash_token)),
                Some(SyntaxElement::Token(self.await_token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.then_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.catch_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
                Some(SyntaxElement::Node(self.children.into_syntax())),
            ],
        ))
    }
}
pub fn svelte_await_then_block(
    sv_curly_colon_token: SyntaxToken,
    then_token: SyntaxToken,
    name: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteAwaitThenBlock {
    SvelteAwaitThenBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_THEN_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_colon_token)),
            Some(SyntaxElement::Token(then_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_await_then_clause(
    then_token: SyntaxToken,
    name: HtmlTextExpression,
) -> SvelteAwaitThenClause {
    SvelteAwaitThenClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_THEN_CLAUSE,
        [
            Some(SyntaxElement::Token(then_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn svelte_bind_directive(
    bind_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteBindDirective {
    SvelteBindDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_BIND_DIRECTIVE,
        [
            Some(SyntaxElement::Token(bind_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_class_directive(
    class_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteClassDirective {
    SvelteClassDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_CLASS_DIRECTIVE,
        [
            Some(SyntaxElement::Token(class_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_const_block(
    sv_curly_at_token: SyntaxToken,
    const_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteConstBlock {
    SvelteConstBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_CONST_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_at_token)),
            Some(SyntaxElement::Token(const_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_curly_destructured_name(
    l_curly_token: SyntaxToken,
    names: SvelteBindingAssignmentBindingList,
    r_curly_token: SyntaxToken,
) -> SvelteCurlyDestructuredName {
    SvelteCurlyDestructuredName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_CURLY_DESTRUCTURED_NAME,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(names.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_debug_block(
    sv_curly_at_token: SyntaxToken,
    debug_token: SyntaxToken,
    bindings: SvelteBindingList,
    r_curly_token: SyntaxToken,
) -> SvelteDebugBlock {
    SvelteDebugBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_DEBUG_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_at_token)),
            Some(SyntaxElement::Token(debug_token)),
            Some(SyntaxElement::Node(bindings.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_directive_modifier(
    bitwise_or_token: SyntaxToken,
    name: SvelteName,
) -> SvelteDirectiveModifier {
    SvelteDirectiveModifier::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_DIRECTIVE_MODIFIER,
        [
            Some(SyntaxElement::Token(bitwise_or_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn svelte_directive_value(
    colon_token: SyntaxToken,
    property: AnySvelteBindingProperty,
    modifiers: SvelteDirectiveModifierList,
) -> SvelteDirectiveValueBuilder {
    SvelteDirectiveValueBuilder {
        colon_token,
        property,
        modifiers,
        initializer: None,
    }
}
pub struct SvelteDirectiveValueBuilder {
    colon_token: SyntaxToken,
    property: AnySvelteBindingProperty,
    modifiers: SvelteDirectiveModifierList,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl SvelteDirectiveValueBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> SvelteDirectiveValue {
        SvelteDirectiveValue::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_DIRECTIVE_VALUE,
            [
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.property.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn svelte_each_as_keyed_item(
    as_token: SyntaxToken,
    name: AnySvelteEachName,
) -> SvelteEachAsKeyedItemBuilder {
    SvelteEachAsKeyedItemBuilder {
        as_token,
        name,
        index: None,
        key: None,
    }
}
pub struct SvelteEachAsKeyedItemBuilder {
    as_token: SyntaxToken,
    name: AnySvelteEachName,
    index: Option<SvelteEachIndex>,
    key: Option<SvelteEachKey>,
}
impl SvelteEachAsKeyedItemBuilder {
    pub fn with_index(mut self, index: SvelteEachIndex) -> Self {
        self.index = Some(index);
        self
    }
    pub fn with_key(mut self, key: SvelteEachKey) -> Self {
        self.key = Some(key);
        self
    }
    pub fn build(self) -> SvelteEachAsKeyedItem {
        SvelteEachAsKeyedItem::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_EACH_AS_KEYED_ITEM,
            [
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.index
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.key
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn svelte_each_block(
    opening_block: SvelteEachOpeningBlock,
    children: HtmlElementList,
    closing_block: SvelteEachClosingBlock,
) -> SvelteEachBlockBuilder {
    SvelteEachBlockBuilder {
        opening_block,
        children,
        closing_block,
        else_clause: None,
    }
}
pub struct SvelteEachBlockBuilder {
    opening_block: SvelteEachOpeningBlock,
    children: HtmlElementList,
    closing_block: SvelteEachClosingBlock,
    else_clause: Option<SvelteElseClause>,
}
impl SvelteEachBlockBuilder {
    pub fn with_else_clause(mut self, else_clause: SvelteElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> SvelteEachBlock {
        SvelteEachBlock::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_EACH_BLOCK,
            [
                Some(SyntaxElement::Node(self.opening_block.into_syntax())),
                Some(SyntaxElement::Node(self.children.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.closing_block.into_syntax())),
            ],
        ))
    }
}
pub fn svelte_each_closing_block(
    sv_curly_slash_token: SyntaxToken,
    each_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> SvelteEachClosingBlock {
    SvelteEachClosingBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_EACH_CLOSING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_slash_token)),
            Some(SyntaxElement::Token(each_token)),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_each_index(comma_token: SyntaxToken, value: SvelteName) -> SvelteEachIndex {
    SvelteEachIndex::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_EACH_INDEX,
        [
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_each_key(
    l_paren_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_paren_token: SyntaxToken,
) -> SvelteEachKey {
    SvelteEachKey::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_EACH_KEY,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn svelte_each_keyed_item() -> SvelteEachKeyedItemBuilder {
    SvelteEachKeyedItemBuilder { index: None }
}
pub struct SvelteEachKeyedItemBuilder {
    index: Option<SvelteEachIndex>,
}
impl SvelteEachKeyedItemBuilder {
    pub fn with_index(mut self, index: SvelteEachIndex) -> Self {
        self.index = Some(index);
        self
    }
    pub fn build(self) -> SvelteEachKeyedItem {
        SvelteEachKeyedItem::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_EACH_KEYED_ITEM,
            [self
                .index
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn svelte_each_opening_block(
    sv_curly_hash_token: SyntaxToken,
    each_token: SyntaxToken,
    list: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteEachOpeningBlockBuilder {
    SvelteEachOpeningBlockBuilder {
        sv_curly_hash_token,
        each_token,
        list,
        r_curly_token,
        item: None,
    }
}
pub struct SvelteEachOpeningBlockBuilder {
    sv_curly_hash_token: SyntaxToken,
    each_token: SyntaxToken,
    list: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    item: Option<AnySvelteBlockItem>,
}
impl SvelteEachOpeningBlockBuilder {
    pub fn with_item(mut self, item: AnySvelteBlockItem) -> Self {
        self.item = Some(item);
        self
    }
    pub fn build(self) -> SvelteEachOpeningBlock {
        SvelteEachOpeningBlock::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_EACH_OPENING_BLOCK,
            [
                Some(SyntaxElement::Token(self.sv_curly_hash_token)),
                Some(SyntaxElement::Token(self.each_token)),
                Some(SyntaxElement::Node(self.list.into_syntax())),
                self.item
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn svelte_else_clause(
    sv_curly_colon_token: SyntaxToken,
    else_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteElseClause {
    SvelteElseClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(sv_curly_colon_token)),
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_else_if_clause(
    sv_curly_colon_token: SyntaxToken,
    else_token: SyntaxToken,
    if_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteElseIfClause {
    SvelteElseIfClause::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE,
        [
            Some(SyntaxElement::Token(sv_curly_colon_token)),
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Token(if_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_html_block(
    sv_curly_at_token: SyntaxToken,
    html_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteHtmlBlock {
    SvelteHtmlBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_HTML_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_at_token)),
            Some(SyntaxElement::Token(html_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_if_block(
    opening_block: SvelteIfOpeningBlock,
    else_if_clauses: SvelteElseIfClauseList,
    closing_block: SvelteIfClosingBlock,
) -> SvelteIfBlockBuilder {
    SvelteIfBlockBuilder {
        opening_block,
        else_if_clauses,
        closing_block,
        else_clause: None,
    }
}
pub struct SvelteIfBlockBuilder {
    opening_block: SvelteIfOpeningBlock,
    else_if_clauses: SvelteElseIfClauseList,
    closing_block: SvelteIfClosingBlock,
    else_clause: Option<SvelteElseClause>,
}
impl SvelteIfBlockBuilder {
    pub fn with_else_clause(mut self, else_clause: SvelteElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> SvelteIfBlock {
        SvelteIfBlock::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::SVELTE_IF_BLOCK,
            [
                Some(SyntaxElement::Node(self.opening_block.into_syntax())),
                Some(SyntaxElement::Node(self.else_if_clauses.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.closing_block.into_syntax())),
            ],
        ))
    }
}
pub fn svelte_if_closing_block(
    sv_curly_slash_token: SyntaxToken,
    if_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> SvelteIfClosingBlock {
    SvelteIfClosingBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_IF_CLOSING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_slash_token)),
            Some(SyntaxElement::Token(if_token)),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_if_opening_block(
    sv_curly_hash_token: SyntaxToken,
    if_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteIfOpeningBlock {
    SvelteIfOpeningBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_IF_OPENING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_hash_token)),
            Some(SyntaxElement::Token(if_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_in_directive(
    in_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteInDirective {
    SvelteInDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_IN_DIRECTIVE,
        [
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_key_block(
    opening_block: SvelteKeyOpeningBlock,
    children: HtmlElementList,
    closing_block: SvelteKeyClosingBlock,
) -> SvelteKeyBlock {
    SvelteKeyBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_KEY_BLOCK,
        [
            Some(SyntaxElement::Node(opening_block.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing_block.into_syntax())),
        ],
    ))
}
pub fn svelte_key_closing_block(
    sv_curly_slash_token: SyntaxToken,
    key_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> SvelteKeyClosingBlock {
    SvelteKeyClosingBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_KEY_CLOSING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_slash_token)),
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_key_opening_block(
    sv_curly_hash_token: SyntaxToken,
    key_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteKeyOpeningBlock {
    SvelteKeyOpeningBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_KEY_OPENING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_hash_token)),
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_literal(value_token: SyntaxToken) -> SvelteLiteral {
    SvelteLiteral::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_LITERAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn svelte_name(ident_token: SyntaxToken) -> SvelteName {
    SvelteName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_NAME,
        [Some(SyntaxElement::Token(ident_token))],
    ))
}
pub fn svelte_out_directive(
    out_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteOutDirective {
    SvelteOutDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_OUT_DIRECTIVE,
        [
            Some(SyntaxElement::Token(out_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_render_block(
    sv_curly_at_token: SyntaxToken,
    render_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
) -> SvelteRenderBlock {
    SvelteRenderBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_RENDER_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_at_token)),
            Some(SyntaxElement::Token(render_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_rest_binding(dotdotdot_token: SyntaxToken, name: SvelteName) -> SvelteRestBinding {
    SvelteRestBinding::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_REST_BINDING,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn svelte_snippet_block(
    opening_block: SvelteSnippetOpeningBlock,
    closing_block: SvelteSnippetClosingBlock,
) -> SvelteSnippetBlock {
    SvelteSnippetBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_SNIPPET_BLOCK,
        [
            Some(SyntaxElement::Node(opening_block.into_syntax())),
            Some(SyntaxElement::Node(closing_block.into_syntax())),
        ],
    ))
}
pub fn svelte_snippet_closing_block(
    sv_curly_slash_token: SyntaxToken,
    snippet_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> SvelteSnippetClosingBlock {
    SvelteSnippetClosingBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_SNIPPET_CLOSING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_slash_token)),
            Some(SyntaxElement::Token(snippet_token)),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn svelte_snippet_opening_block(
    sv_curly_hash_token: SyntaxToken,
    snippet_token: SyntaxToken,
    expression: HtmlTextExpression,
    r_curly_token: SyntaxToken,
    children: HtmlElementList,
) -> SvelteSnippetOpeningBlock {
    SvelteSnippetOpeningBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_SNIPPET_OPENING_BLOCK,
        [
            Some(SyntaxElement::Token(sv_curly_hash_token)),
            Some(SyntaxElement::Token(snippet_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
            Some(SyntaxElement::Node(children.into_syntax())),
        ],
    ))
}
pub fn svelte_square_destructured_name(
    l_brack_token: SyntaxToken,
    names: SvelteBindingAssignmentBindingList,
    r_brack_token: SyntaxToken,
) -> SvelteSquareDestructuredName {
    SvelteSquareDestructuredName::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_SQUARE_DESTRUCTURED_NAME,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(names.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn svelte_style_directive(
    style_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteStyleDirective {
    SvelteStyleDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_STYLE_DIRECTIVE,
        [
            Some(SyntaxElement::Token(style_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_transition_directive(
    transition_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteTransitionDirective {
    SvelteTransitionDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_TRANSITION_DIRECTIVE,
        [
            Some(SyntaxElement::Token(transition_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn svelte_use_directive(
    use_token: SyntaxToken,
    value: SvelteDirectiveValue,
) -> SvelteUseDirective {
    SvelteUseDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_USE_DIRECTIVE,
        [
            Some(SyntaxElement::Token(use_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn vue_directive(name_token: SyntaxToken, modifiers: VueModifierList) -> VueDirectiveBuilder {
    VueDirectiveBuilder {
        name_token,
        modifiers,
        arg: None,
        initializer: None,
    }
}
pub struct VueDirectiveBuilder {
    name_token: SyntaxToken,
    modifiers: VueModifierList,
    arg: Option<VueDirectiveArgument>,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl VueDirectiveBuilder {
    pub fn with_arg(mut self, arg: VueDirectiveArgument) -> Self {
        self.arg = Some(arg);
        self
    }
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> VueDirective {
        VueDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::VUE_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.name_token)),
                self.arg
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn vue_directive_argument(
    colon_token: SyntaxToken,
    arg: AnyVueDirectiveArgument,
) -> VueDirectiveArgument {
    VueDirectiveArgument::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_DIRECTIVE_ARGUMENT,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(arg.into_syntax())),
        ],
    ))
}
pub fn vue_dynamic_argument(
    l_brack_token: SyntaxToken,
    name_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> VueDynamicArgument {
    VueDynamicArgument::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_DYNAMIC_ARGUMENT,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn vue_modifier(dot_token: SyntaxToken, modifier_token: SyntaxToken) -> VueModifier {
    VueModifier::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_MODIFIER,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Token(modifier_token)),
        ],
    ))
}
pub fn vue_static_argument(name_token: SyntaxToken) -> VueStaticArgument {
    VueStaticArgument::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_STATIC_ARGUMENT,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn vue_v_bind_shorthand_directive(
    arg: VueDirectiveArgument,
    modifiers: VueModifierList,
) -> VueVBindShorthandDirectiveBuilder {
    VueVBindShorthandDirectiveBuilder {
        arg,
        modifiers,
        initializer: None,
    }
}
pub struct VueVBindShorthandDirectiveBuilder {
    arg: VueDirectiveArgument,
    modifiers: VueModifierList,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl VueVBindShorthandDirectiveBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> VueVBindShorthandDirective {
        VueVBindShorthandDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::VUE_V_BIND_SHORTHAND_DIRECTIVE,
            [
                Some(SyntaxElement::Node(self.arg.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn vue_v_on_shorthand_directive(
    at_token: SyntaxToken,
    arg: AnyVueDirectiveArgument,
    modifiers: VueModifierList,
) -> VueVOnShorthandDirectiveBuilder {
    VueVOnShorthandDirectiveBuilder {
        at_token,
        arg,
        modifiers,
        initializer: None,
    }
}
pub struct VueVOnShorthandDirectiveBuilder {
    at_token: SyntaxToken,
    arg: AnyVueDirectiveArgument,
    modifiers: VueModifierList,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl VueVOnShorthandDirectiveBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> VueVOnShorthandDirective {
        VueVOnShorthandDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::VUE_V_ON_SHORTHAND_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.at_token)),
                Some(SyntaxElement::Node(self.arg.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn vue_v_slot_shorthand_directive(
    hash_token: SyntaxToken,
    arg: AnyVueDirectiveArgument,
    modifiers: VueModifierList,
) -> VueVSlotShorthandDirectiveBuilder {
    VueVSlotShorthandDirectiveBuilder {
        hash_token,
        arg,
        modifiers,
        initializer: None,
    }
}
pub struct VueVSlotShorthandDirectiveBuilder {
    hash_token: SyntaxToken,
    arg: AnyVueDirectiveArgument,
    modifiers: VueModifierList,
    initializer: Option<HtmlAttributeInitializerClause>,
}
impl VueVSlotShorthandDirectiveBuilder {
    pub fn with_initializer(mut self, initializer: HtmlAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> VueVSlotShorthandDirective {
        VueVSlotShorthandDirective::unwrap_cast(SyntaxNode::new_detached(
            HtmlSyntaxKind::VUE_V_SLOT_SHORTHAND_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.arg.into_syntax())),
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn html_attribute_list<I>(items: I) -> HtmlAttributeList
where
    I: IntoIterator<Item = AnyHtmlAttribute>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlAttributeList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn html_element_list<I>(items: I) -> HtmlElementList
where
    I: IntoIterator<Item = AnyHtmlElement>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlElementList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn svelte_await_clauses_list<I>(items: I) -> SvelteAwaitClausesList
where
    I: IntoIterator<Item = AnySvelteAwaitClauses>,
    I::IntoIter: ExactSizeIterator,
{
    SvelteAwaitClausesList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_AWAIT_CLAUSES_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn svelte_binding_assignment_binding_list<I, S>(
    items: I,
    separators: S,
) -> SvelteBindingAssignmentBindingList
where
    I: IntoIterator<Item = AnySvelteBindingAssignmentBinding>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = HtmlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SvelteBindingAssignmentBindingList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_BINDING_ASSIGNMENT_BINDING_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn svelte_binding_list<I, S>(items: I, separators: S) -> SvelteBindingList
where
    I: IntoIterator<Item = SvelteName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = HtmlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SvelteBindingList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_BINDING_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn svelte_directive_modifier_list<I>(items: I) -> SvelteDirectiveModifierList
where
    I: IntoIterator<Item = SvelteDirectiveModifier>,
    I::IntoIter: ExactSizeIterator,
{
    SvelteDirectiveModifierList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_DIRECTIVE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn svelte_else_if_clause_list<I>(items: I) -> SvelteElseIfClauseList
where
    I: IntoIterator<Item = SvelteElseIfClause>,
    I::IntoIter: ExactSizeIterator,
{
    SvelteElseIfClauseList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn vue_modifier_list<I>(items: I) -> VueModifierList
where
    I: IntoIterator<Item = VueModifier>,
    I::IntoIter: ExactSizeIterator,
{
    VueModifierList::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn astro_bogus_frontmatter<I>(slots: I) -> AstroBogusFrontmatter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    AstroBogusFrontmatter::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::ASTRO_BOGUS_FRONTMATTER,
        slots,
    ))
}
pub fn html_bogus<I>(slots: I) -> HtmlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogus::unwrap_cast(SyntaxNode::new_detached(HtmlSyntaxKind::HTML_BOGUS, slots))
}
pub fn html_bogus_attribute<I>(slots: I) -> HtmlBogusAttribute
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogusAttribute::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_BOGUS_ATTRIBUTE,
        slots,
    ))
}
pub fn html_bogus_element<I>(slots: I) -> HtmlBogusElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogusElement::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_BOGUS_ELEMENT,
        slots,
    ))
}
pub fn html_bogus_text_expression<I>(slots: I) -> HtmlBogusTextExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    HtmlBogusTextExpression::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::HTML_BOGUS_TEXT_EXPRESSION,
        slots,
    ))
}
pub fn svelte_bogus_block<I>(slots: I) -> SvelteBogusBlock
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SvelteBogusBlock::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::SVELTE_BOGUS_BLOCK,
        slots,
    ))
}
pub fn vue_bogus_directive<I>(slots: I) -> VueBogusDirective
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    VueBogusDirective::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_BOGUS_DIRECTIVE,
        slots,
    ))
}
pub fn vue_bogus_directive_argument<I>(slots: I) -> VueBogusDirectiveArgument
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    VueBogusDirectiveArgument::unwrap_cast(SyntaxNode::new_detached(
        HtmlSyntaxKind::VUE_BOGUS_DIRECTIVE_ARGUMENT,
        slots,
    ))
}
