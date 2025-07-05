use crate::prelude::*;
use biome_astro_syntax::{
    AnyAstroAttribute, AnyAstroElement, AstroLanguage, AstroSyntaxKind, AstroSyntaxNode,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatRule, FormatRuleWithOptions};

#[derive(Debug, Clone, Default)]
pub struct FormatAstroSyntaxNode;

impl FormatRule<AstroSyntaxNode> for FormatAstroSyntaxNode {
    type Options = ();

    fn fmt(&self, node: &AstroSyntaxNode, f: &mut AstroFormatter) -> FormatResult<()> {
        match node.kind() {
            AstroSyntaxKind::ASTRO_ROOT => FormatNodeRule::fmt(&FormatAstroRoot, node, f),
            AstroSyntaxKind::ASTRO_FRONTMATTER => {
                FormatNodeRule::fmt(&FormatAstroFrontmatter, node, f)
            }
            AstroSyntaxKind::ASTRO_ELEMENT => FormatNodeRule::fmt(&FormatAstroElement, node, f),
            AstroSyntaxKind::ASTRO_SELF_CLOSING_ELEMENT => {
                FormatNodeRule::fmt(&FormatAstroSelfClosingElement, node, f)
            }
            AstroSyntaxKind::ASTRO_EXPRESSION => {
                FormatNodeRule::fmt(&FormatAstroExpression, node, f)
            }
            AstroSyntaxKind::ASTRO_TEXT => FormatNodeRule::fmt(&FormatAstroText, node, f),
            AstroSyntaxKind::ASTRO_COMMENT => FormatNodeRule::fmt(&FormatAstroComment, node, f),
            AstroSyntaxKind::ASTRO_ATTRIBUTE => {
                FormatNodeRule::fmt(&FormatAstroAttribute, node, f)
            }
            AstroSyntaxKind::ASTRO_SHORTHAND_ATTRIBUTE => {
                FormatNodeRule::fmt(&FormatAstroShorthandAttribute, node, f)
            }
            AstroSyntaxKind::ASTRO_SPREAD_ATTRIBUTE => {
                FormatNodeRule::fmt(&FormatAstroSpreadAttribute, node, f)
            }
            AstroSyntaxKind::ASTRO_EXPRESSION_ATTRIBUTE => {
                FormatNodeRule::fmt(&FormatAstroExpressionAttribute, node, f)
            }
            AstroSyntaxKind::ASTRO_ELEMENT_LIST => {
                FormatNodeRule::fmt(&FormatAstroElementList, node, f)
            }
            AstroSyntaxKind::ASTRO_ATTRIBUTE_LIST => {
                FormatNodeRule::fmt(&FormatAstroAttributeList, node, f)
            }
            _ => {
                // Format unknown nodes as verbatim
                format_verbatim_node(node.clone()).fmt(f)
            }
        }
    }
}

// Individual format rules for each node type
#[derive(Debug, Clone, Default)]
pub struct FormatAstroRoot;

impl FormatNodeRule<biome_astro_syntax::AstroRoot> for FormatAstroRoot {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroRoot,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // Format BOM if present
        if let Ok(bom) = node.bom() {
            write!(f, [bom.format()])?;
        }

        // Format frontmatter if present
        if let Ok(frontmatter) = node.frontmatter() {
            write!(f, [frontmatter.format(), hard_line_break()])?;
        }

        // Format body elements
        if let Ok(body) = node.body() {
            write!(f, [body.format()])?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroFrontmatter;

impl FormatNodeRule<biome_astro_syntax::AstroFrontmatter> for FormatAstroFrontmatter {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroFrontmatter,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // Format opening fence
        write!(f, [text("---"), hard_line_break()])?;

        // Format content (JavaScript/TypeScript)
        if let Ok(content) = node.content() {
            // For now, format as verbatim
            // In a full implementation, we'd integrate with the JS formatter
            write!(f, [content.format()])?;
        }

        // Format closing fence
        write!(f, [hard_line_break(), text("---")])?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroElement;

impl FormatNodeRule<biome_astro_syntax::AstroElement> for FormatAstroElement {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroElement,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // Format opening element
        if let Ok(opening) = node.opening_element() {
            write!(f, [opening.format()])?;
        }

        // Format children with indentation
        if let Ok(children) = node.children() {
            if !children.is_empty() {
                write!(f, [block_indent(&children.format())])?;
            }
        }

        // Format closing element
        if let Ok(closing) = node.closing_element() {
            write!(f, [closing.format()])?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroSelfClosingElement;

impl FormatNodeRule<biome_astro_syntax::AstroSelfClosingElement> for FormatAstroSelfClosingElement {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroSelfClosingElement,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        write!(f, [text("<")])?;

        if let Ok(name) = node.name() {
            write!(f, [name.format()])?;
        }

        if let Ok(attributes) = node.attributes() {
            if !attributes.is_empty() {
                write!(f, [space(), attributes.format()])?;
            }
        }

        write!(f, [space(), text("/"), text(">")])?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroExpression;

impl FormatNodeRule<biome_astro_syntax::AstroExpression> for FormatAstroExpression {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroExpression,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        write!(f, [text("{")])?;

        if let Ok(content) = node.expression() {
            // For now, format as verbatim
            // In a full implementation, we'd integrate with the JS formatter
            write!(f, [content.format()])?;
        }

        write!(f, [text("}")])?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroText;

impl FormatNodeRule<biome_astro_syntax::AstroText> for FormatAstroText {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroText,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        if let Ok(value) = node.value() {
            write!(f, [value.format()])?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroComment;

impl FormatNodeRule<biome_astro_syntax::AstroComment> for FormatAstroComment {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroComment,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        if let Ok(content) = node.content() {
            write!(f, [text("<!--"), content.format(), text("-->")])?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroAttribute;

impl FormatNodeRule<biome_astro_syntax::AstroAttribute> for FormatAstroAttribute {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroAttribute,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        if let Ok(name) = node.name() {
            write!(f, [name.format()])?;
        }

        if let Ok(initializer) = node.initializer() {
            write!(f, [initializer.format()])?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroShorthandAttribute;

impl FormatNodeRule<biome_astro_syntax::AstroShorthandAttribute> for FormatAstroShorthandAttribute {
    fn fmt_fields(
        &self,
        _node: &biome_astro_syntax::AstroShorthandAttribute,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // This would need proper AST access to implement
        // For now, format as verbatim
        write!(f, [text("{prop}")])?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroSpreadAttribute;

impl FormatNodeRule<biome_astro_syntax::AstroSpreadAttribute> for FormatAstroSpreadAttribute {
    fn fmt_fields(
        &self,
        _node: &biome_astro_syntax::AstroSpreadAttribute,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // This would need proper AST access to implement
        // For now, format as verbatim
        write!(f, [text("{...props}")])?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroExpressionAttribute;

impl FormatNodeRule<biome_astro_syntax::AstroExpressionAttribute> for FormatAstroExpressionAttribute {
    fn fmt_fields(
        &self,
        _node: &biome_astro_syntax::AstroExpressionAttribute,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        // This would need proper AST access to implement
        // For now, format as verbatim
        write!(f, [text("name={expr}")])?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroElementList;

impl FormatNodeRule<biome_astro_syntax::AstroElementList> for FormatAstroElementList {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroElementList,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        let mut elements = node.iter();
        if let Some(first) = elements.next() {
            write!(f, [first.format()])?;

            for element in elements {
                write!(f, [hard_line_break(), element.format()])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormatAstroAttributeList;

impl FormatNodeRule<biome_astro_syntax::AstroAttributeList> for FormatAstroAttributeList {
    fn fmt_fields(
        &self,
        node: &biome_astro_syntax::AstroAttributeList,
        f: &mut AstroFormatter,
    ) -> FormatResult<()> {
        let mut attributes = node.iter();
        if let Some(first) = attributes.next() {
            write!(f, [first.format()])?;

            for attr in attributes {
                write!(f, [space(), attr.format()])?;
            }
        }
        Ok(())
    }
}