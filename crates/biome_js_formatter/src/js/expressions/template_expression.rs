use std::collections::HashMap;
use std::ops::Deref;

use crate::{prelude::*, JsForeignLanguage};
use biome_formatter::{write, CstFormatContext};

use crate::js::expressions::static_member_expression::member_chain_callee_needs_parens;
use crate::js::lists::template_element_list::FormatJsTemplateElementListOptions;
use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsExpression, AnyJsTemplateElement, JsSyntaxNode, JsTemplateElement, JsTemplateElementList,
    JsTemplateExpression, TsTemplateLiteralType,
};
use biome_js_syntax::{JsSyntaxToken, TsTypeArguments};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTemplateExpression;

impl FormatNodeRule<JsTemplateExpression> for FormatJsTemplateExpression {
    fn fmt_fields(&self, node: &JsTemplateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsTemplate::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsTemplateExpression) -> bool {
        item.needs_parentheses()
    }
}

declare_node_union! {
    AnyJsTemplate = JsTemplateExpression | TsTemplateLiteralType
}

impl Format<JsFormatContext> for AnyJsTemplate {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [
                self.tag().format(),
                self.type_arguments().format(),
                line_suffix_boundary(),
                self.l_tick_token().format(),
            ]
        )?;

        self.write_elements(f)?;

        write!(f, [self.r_tick_token().format()])
    }
}

impl AnyJsTemplate {
    fn tag(&self) -> Option<AnyJsExpression> {
        match self {
            AnyJsTemplate::JsTemplateExpression(template) => template.tag(),
            AnyJsTemplate::TsTemplateLiteralType(_) => None,
        }
    }

    fn type_arguments(&self) -> Option<TsTypeArguments> {
        match self {
            AnyJsTemplate::JsTemplateExpression(template) => template.type_arguments(),
            AnyJsTemplate::TsTemplateLiteralType(_) => None,
        }
    }

    fn l_tick_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsTemplate::JsTemplateExpression(template) => template.l_tick_token(),
            AnyJsTemplate::TsTemplateLiteralType(template) => template.l_tick_token(),
        }
    }

    fn write_elements(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            AnyJsTemplate::JsTemplateExpression(template) => {
                if is_css_embedded(template)?
                    && !template
                        .elements()
                        .iter()
                        .map(|element| f.context().comments().is_suppressed(element.syntax()))
                        .any(|is_suppressed| is_suppressed == true)
                {
                    let interned = f.intern(&format_with(|f| {
                        format_embedded_language(template.elements(), JsForeignLanguage::Css, f)
                    }));

                    match interned {
                        Ok(interned) => {
                            if let Some(interned) = interned {
                                f.write_element(interned.clone())?;
                            }
                            return Ok(());
                        }
                        Err(_) => {
                            // if we failed to format the template as css, we'll fall back to the default formatting
                        }
                    }
                }
                let is_test_each_pattern = template.is_test_each_pattern();
                let options = FormatJsTemplateElementListOptions {
                    is_test_each_pattern,
                };

                write!(f, [template.elements().format().with_options(options)])
            }
            AnyJsTemplate::TsTemplateLiteralType(template) => {
                write!(f, [template.elements().format()])
            }
        }
    }

    fn r_tick_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsTemplate::JsTemplateExpression(template) => template.r_tick_token(),
            AnyJsTemplate::TsTemplateLiteralType(template) => template.r_tick_token(),
        }
    }
}

/// `TemplateLiteral`'s are `PrimaryExpression's that never need parentheses.
impl NeedsParentheses for JsTemplateExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if self.tag().is_some() {
            member_chain_callee_needs_parens(self.clone().into(), parent)
        } else {
            false
        }
    }
}

fn format_embedded_language(
    elements: JsTemplateElementList,
    language: JsForeignLanguage,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    // keep track of the placeholders we use to replace string interpolations
    let mut placeholder_map: HashMap<String, JsTemplateElement> = HashMap::new();
    let mut index = 0;
    // we need to replace string interpolations with placeholders to avoid parsing errors
    // after formatting finished, we'll replace the placeholders with the original string interpolations
    let content = elements
        .iter()
        .fold(String::new(), |acc, element| match element {
            AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                let text = element.text();
                std::format!("{}{}", acc, text)
            }
            AnyJsTemplateElement::JsTemplateElement(element) => {
                let placeholder = std::format!("biome-placeholder-{}", index);
                index += 1;
                placeholder_map.insert(placeholder.clone(), element.clone());
                std::format!("{}{}", acc, placeholder)
            }
        });
    let embedded_language_formatted = f
        .context()
        .get_foreign_language_formatter()
        .format(language, &content)?;

    fn replace_placeholder_with_template_element(
        element: FormatElement,
        placeholder_map: &HashMap<String, JsTemplateElement>,
        f: &mut JsFormatter,
    ) -> SyntaxResult<FormatElement> {
        match element.clone() {
            FormatElement::LocatedTokenText { slice, .. } => {
                let text = slice.to_string();
                if let Some(template_element) = placeholder_map.get(&text) {
                    let interned = f.intern(&template_element.format());
                    if let Ok(Some(interned_template_element)) = interned {
                        Ok(interned_template_element)
                    } else {
                        Ok(FormatElement::Interned(Interned::new(vec![])))
                    }
                } else {
                    Ok(element)
                }
            }
            FormatElement::Interned(interned) => {
                let elemets = interned
                    .iter()
                    .map(|element| {
                        replace_placeholder_with_template_element(
                            element.clone(),
                            placeholder_map,
                            f,
                        )
                    })
                    .collect::<Result<Vec<FormatElement>, _>>()?;
                Ok(FormatElement::Interned(Interned::new(elemets)))
            }
            FormatElement::BestFitting(best_fitting) => {
                let variants = best_fitting
                    .variants()
                    .iter()
                    .map(|variant| {
                        let elements = variant
                            .iter()
                            .map(|element| {
                                replace_placeholder_with_template_element(
                                    element.clone(),
                                    placeholder_map,
                                    f,
                                )
                            })
                            .collect::<Result<Vec<FormatElement>, _>>();
                        elements.map(|elements| Box::new(elements).into_boxed_slice())
                    })
                    .collect::<Result<Vec<Box<[FormatElement]>>, _>>()?;
                unsafe {
                    Ok(FormatElement::BestFitting(
                        BestFittingElement::from_vec_unchecked(variants),
                    ))
                }
            }
            element => Ok(element),
        }
    }

    // replace placeholders with the original string interpolations
    let formatted = embedded_language_formatted
        .deref()
        .iter()
        .map(|element| {
            replace_placeholder_with_template_element(element.clone(), &placeholder_map, f)
        })
        .collect::<Result<Vec<FormatElement>, _>>()?;

    // template chunks are formatted by the embedded language formatter, so we need to tell the formatter to ignore them
    for element in elements.iter() {
        match element {
            AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                let token = element.template_chunk_token()?;
                write!(f, [&format_removed(&token)])?;
            }
            _ => {}
        }
    }
    write!(
        f,
        [
            &indent(&format_with(|f| {
                write!(f, [hard_line_break()])?;
                f.write_elements(formatted.clone())
            })),
            soft_line_break()
        ]
    )
}

fn is_css_embedded(template: &JsTemplateExpression) -> SyntaxResult<bool> {
    let tag = template.tag();
    if let Some(tag) = tag {
        let ident_expr = tag.as_js_identifier_expression();
        if let Some(ident_expr) = ident_expr {
            let name = ident_expr.name()?;
            // TODO: support more css-in-js libraries
            // css.global``
            // css.resolve``
            // styled.foo``
            // Component.foo``
            // styled(Component)``
            // styled.foo.attrs({})`
            // Component.extend.attrs({})``
            // styled(Component).attrs({})``
            // JSX element with CSS prop
            if name.has_name("css") {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
