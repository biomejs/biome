use crate::*;
use biome_astro_syntax::{
    AstroSyntaxKind::{self, *},
    AstroSyntaxNode, AstroSyntaxToken,
};
use biome_rowan::{NodeOrToken, SyntaxNode, SyntaxToken};

/// Helper macro for creating syntax nodes
macro_rules! make_node {
    ($kind:expr, $($child:expr),* $(,)?) => {{
        let mut children = Vec::new();
        $(
            children.push($child.into());
        )*
        AstroSyntaxNode::new_detached($kind, children)
    }};
}

/// Creates an Astro root node
pub fn astro_root(
    bom: Option<AstroSyntaxToken>,
    frontmatter: Option<AstroSyntaxNode>,
    body: AstroSyntaxNode,
    eof: AstroSyntaxToken,
) -> AstroSyntaxNode {
    let mut children = Vec::new();
    
    if let Some(bom) = bom {
        children.push(NodeOrToken::Token(bom));
    }
    
    if let Some(frontmatter) = frontmatter {
        children.push(NodeOrToken::Node(frontmatter));
    }
    
    children.push(NodeOrToken::Node(body));
    children.push(NodeOrToken::Token(eof));
    
    AstroSyntaxNode::new_detached(ASTRO_ROOT, children)
}

/// Creates an Astro frontmatter block
pub fn astro_frontmatter(
    l_fence: AstroSyntaxToken,
    content: Option<AstroSyntaxNode>,
    r_fence: AstroSyntaxToken,
) -> AstroSyntaxNode {
    let mut children = vec![NodeOrToken::Token(l_fence)];
    
    if let Some(content) = content {
        children.push(NodeOrToken::Node(content));
    }
    
    children.push(NodeOrToken::Token(r_fence));
    
    AstroSyntaxNode::new_detached(ASTRO_FRONTMATTER, children)
}

/// Creates an Astro frontmatter content node
pub fn astro_frontmatter_content(content: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_FRONTMATTER_CONTENT, NodeOrToken::Token(content))
}

/// Creates an Astro element
pub fn astro_element(
    opening: AstroSyntaxNode,
    children: AstroSyntaxNode,
    closing: AstroSyntaxNode,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_ELEMENT,
        NodeOrToken::Node(opening),
        NodeOrToken::Node(children),
        NodeOrToken::Node(closing)
    )
}

/// Creates an Astro self-closing element
pub fn astro_self_closing_element(
    l_angle: AstroSyntaxToken,
    name: AstroSyntaxNode,
    attributes: AstroSyntaxNode,
    slash: Option<AstroSyntaxToken>,
    r_angle: AstroSyntaxToken,
) -> AstroSyntaxNode {
    let mut children = vec![
        NodeOrToken::Token(l_angle),
        NodeOrToken::Node(name),
        NodeOrToken::Node(attributes),
    ];
    
    if let Some(slash) = slash {
        children.push(NodeOrToken::Token(slash));
    }
    
    children.push(NodeOrToken::Token(r_angle));
    
    AstroSyntaxNode::new_detached(ASTRO_SELF_CLOSING_ELEMENT, children)
}

/// Creates an Astro opening element
pub fn astro_opening_element(
    l_angle: AstroSyntaxToken,
    name: AstroSyntaxNode,
    attributes: AstroSyntaxNode,
    r_angle: AstroSyntaxToken,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_OPENING_ELEMENT,
        NodeOrToken::Token(l_angle),
        NodeOrToken::Node(name),
        NodeOrToken::Node(attributes),
        NodeOrToken::Token(r_angle)
    )
}

/// Creates an Astro closing element
pub fn astro_closing_element(
    l_angle: AstroSyntaxToken,
    slash: AstroSyntaxToken,
    name: AstroSyntaxNode,
    r_angle: AstroSyntaxToken,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_CLOSING_ELEMENT,
        NodeOrToken::Token(l_angle),
        NodeOrToken::Token(slash),
        NodeOrToken::Node(name),
        NodeOrToken::Token(r_angle)
    )
}

/// Creates an Astro expression
pub fn astro_expression(
    l_curly: AstroSyntaxToken,
    content: Option<AstroSyntaxNode>,
    r_curly: AstroSyntaxToken,
) -> AstroSyntaxNode {
    let mut children = vec![NodeOrToken::Token(l_curly)];
    
    if let Some(content) = content {
        children.push(NodeOrToken::Node(content));
    }
    
    children.push(NodeOrToken::Token(r_curly));
    
    AstroSyntaxNode::new_detached(ASTRO_EXPRESSION, children)
}

/// Creates an Astro expression content node
pub fn astro_expression_content(content: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_EXPRESSION_CONTENT, NodeOrToken::Token(content))
}

/// Creates an Astro text node
pub fn astro_text(text: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_TEXT, NodeOrToken::Token(text))
}

/// Creates an Astro comment
pub fn astro_comment(content: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_COMMENT, NodeOrToken::Token(content))
}

/// Creates an Astro element name
pub fn astro_element_name(name: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_ELEMENT_NAME, NodeOrToken::Token(name))
}

/// Creates an Astro component name
pub fn astro_component_name(name: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_COMPONENT_NAME, NodeOrToken::Token(name))
}

/// Creates an Astro attribute name
pub fn astro_attribute_name(name: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_ATTRIBUTE_NAME, NodeOrToken::Token(name))
}

/// Creates an Astro attribute
pub fn astro_attribute(
    name: AstroSyntaxNode,
    initializer: Option<AstroSyntaxNode>,
) -> AstroSyntaxNode {
    let mut children = vec![NodeOrToken::Node(name)];
    
    if let Some(initializer) = initializer {
        children.push(NodeOrToken::Node(initializer));
    }
    
    AstroSyntaxNode::new_detached(ASTRO_ATTRIBUTE, children)
}

/// Creates an Astro attribute initializer clause
pub fn astro_attribute_initializer_clause(
    equals: AstroSyntaxToken,
    value: AstroSyntaxNode,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_ATTRIBUTE_INITIALIZER_CLAUSE,
        NodeOrToken::Token(equals),
        NodeOrToken::Node(value)
    )
}

/// Creates an Astro attribute value
pub fn astro_attribute_value(value: AstroSyntaxToken) -> AstroSyntaxNode {
    make_node!(ASTRO_ATTRIBUTE_VALUE, NodeOrToken::Token(value))
}

/// Creates an Astro shorthand attribute
pub fn astro_shorthand_attribute(
    l_curly: AstroSyntaxToken,
    name: AstroSyntaxNode,
    r_curly: AstroSyntaxToken,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_SHORTHAND_ATTRIBUTE,
        NodeOrToken::Token(l_curly),
        NodeOrToken::Node(name),
        NodeOrToken::Token(r_curly)
    )
}

/// Creates an Astro spread attribute
pub fn astro_spread_attribute(
    l_curly: AstroSyntaxToken,
    dot3: AstroSyntaxToken,
    expression: AstroSyntaxNode,
    r_curly: AstroSyntaxToken,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_SPREAD_ATTRIBUTE,
        NodeOrToken::Token(l_curly),
        NodeOrToken::Token(dot3),
        NodeOrToken::Node(expression),
        NodeOrToken::Token(r_curly)
    )
}

/// Creates an Astro expression attribute
pub fn astro_expression_attribute(
    name: AstroSyntaxNode,
    equals: AstroSyntaxToken,
    expression: AstroSyntaxNode,
) -> AstroSyntaxNode {
    make_node!(
        ASTRO_EXPRESSION_ATTRIBUTE,
        NodeOrToken::Node(name),
        NodeOrToken::Token(equals),
        NodeOrToken::Node(expression)
    )
}

/// Creates an element list with the given elements
pub fn astro_element_list<I>(elements: I) -> AstroSyntaxNode
where
    I: IntoIterator<Item = AstroSyntaxNode>,
{
    let children: Vec<_> = elements
        .into_iter()
        .map(NodeOrToken::Node)
        .collect();
    
    AstroSyntaxNode::new_detached(ASTRO_ELEMENT_LIST, children)
}

/// Creates an attribute list with the given attributes
pub fn astro_attribute_list<I>(attributes: I) -> AstroSyntaxNode
where
    I: IntoIterator<Item = AstroSyntaxNode>,
{
    let children: Vec<_> = attributes
        .into_iter()
        .map(NodeOrToken::Node)
        .collect();
    
    AstroSyntaxNode::new_detached(ASTRO_ATTRIBUTE_LIST, children)
}