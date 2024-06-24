use std::collections::HashMap;

use crate::{prelude::*, JsForeignLanguage};
use biome_formatter::{write, CstFormatContext, FormatContext};

use crate::js::expressions::static_member_expression::member_chain_callee_needs_parens;
use crate::js::lists::template_element_list::FormatJsTemplateElementListOptions;
use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsExpression, AnyJsName, AnyJsTemplateElement, AnyJsxAttribute, AnyJsxAttributeName,
    AnyJsxElementName, JsIdentifierExpression, JsStaticMemberExpression, JsSyntaxNode,
    JsTemplateElement, JsTemplateElementList, JsTemplateExpression, JsxChildList, JsxElement,
    JsxExpressionChild, TsTemplateLiteralType,
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
                // check if the template only contains whitespaces or newlines
                let is_empty = template.elements().iter().all(|element| {
                    matches!(element, AnyJsTemplateElement::JsTemplateChunkElement(chunk) if chunk.text().trim().is_empty())
                });
                if is_css_embedded(template)?
                    && !is_empty
                    && f.context()
                        .options()
                        .embedded_language_formatting()
                        .is_auto()
                    && !template
                        .elements()
                        .iter()
                        .map(|element| f.context().comments().is_suppressed(element.syntax()))
                        .any(|is_suppressed| is_suppressed)
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

/// A template literal contains `JsTemplateChunkElement` and `JsTemplateElement` elements.
/// A `JsTemplateElement` is an expression that cannot be directly passed to a foreign language formatter.
/// This is because formatters for other languages, such as CSS, do not understand JavaScript expressions.
/// Therefore, we need to replace the JavaScript expression with a placeholder.
///
/// Consider the following example:
/// ```js
/// const css = css`
///    background: ${color};
/// `;
/// ```
/// First, we need to replace `${color}` with a placeholder.
/// After this replacement, the template literal will look like this:
/// ```js
/// const css = css`
///   background: placeholder;
/// `;
/// ```
/// This modified template can then be passed to the CSS formatter.
/// Once formatting is complete, we need to replace the placeholder with the original expression.
fn format_embedded_language(
    elements: JsTemplateElementList,
    language: JsForeignLanguage,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    // We need to track the relationship between placeholders and the original expressions.
    // There is a special scenario we need to handle:
    // If string interpolations are adjacent to each other, replacing them one by one with placeholders
    // makes it difficult to replace them back with the original expressions.
    // Therefore, we need to group adjacent interpolations together and replace them with a single placeholder.
    //
    // For example, consider the following:
    // `background: ${bg}${color}`
    // We should treat `${bg}${color}` as a group and replace it with a single placeholder:
    // `background: placeholder`
    // The placeholder is then mapped to `[bg, color]`.
    let mut placeholder_map: HashMap<String, Vec<JsTemplateElement>> = HashMap::new();
    let mut index = 0;

    let mut content = String::new();
    let mut element_iter = elements.iter().peekable();
    while let Some(template_element) = element_iter.next() {
        match template_element {
            AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                content.push_str(element.text().as_str());
            }
            AnyJsTemplateElement::JsTemplateElement(element) => {
                let mut string_interpolations = vec![element.clone()];
                while let Some(AnyJsTemplateElement::JsTemplateElement(element)) =
                    element_iter.peek()
                {
                    string_interpolations.push(element.clone());
                    element_iter.next();
                }
                let placeholder = std::format!("biome-placeholder-{}", index);
                index += 1;
                placeholder_map.insert(placeholder.clone(), string_interpolations);
                content.push_str("$");
                content.push_str(&placeholder);
            }
        }
    }

    let embedded_language_formatted = f
        .context()
        .get_foreign_language_formatter()
        .format(language, &content)?;

    fn replace_placeholder_with_template_element(
        element: FormatElement,
        placeholder_map: &mut HashMap<String, Vec<JsTemplateElement>>,
        f: &mut JsFormatter,
    ) -> FormatResult<FormatElement> {
        match element.clone() {
            FormatElement::LocatedTokenText { slice, .. } => {
                let text = slice.to_string();
                if let Some(template_elements) = placeholder_map.remove(&text) {
                    let interned = f.intern(&format_with(|f| {
                        for template_element in &template_elements {
                            write!(f, [template_element.format()])?;
                        }
                        Ok(())
                    }))?;
                    if let Some(interned_template_element) = interned {
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

                // SAFETY: the best fitting element is derived from a valid best fitting element
                unsafe {
                    Ok(FormatElement::BestFitting(
                        BestFittingElement::from_vec_unchecked(variants),
                    ))
                }
            }
            element => Ok(element),
        }
    }

    let mut iter = embedded_language_formatted.iter().peekable();
    let mut formatted: Vec<FormatElement> = Vec::new();
    while let Some(element) = iter.next() {
        if let FormatElement::LocatedTokenText { slice, .. } = element {
            let text = slice.to_string();
            if text == "$" {
                if let Some(FormatElement::LocatedTokenText { slice, .. }) = iter.peek() {
                    let next_text = slice.to_string();
                    if placeholder_map.contains_key(&next_text) {
                        continue;
                    }
                }
            }
        }
        formatted.push(replace_placeholder_with_template_element(
            element.clone(),
            &mut placeholder_map,
            f,
        )?);
    }
    // replace placeholders with the original string interpolations
    // let formatted = embedded_language_formatted
    //     .deref()
    //     .iter()
    //     .map(|element| {
    //         replace_placeholder_with_template_element(element.clone(), &mut placeholder_map, f)
    //     })
    //     .collect::<Result<Vec<FormatElement>, _>>()?;

    // if there are any placeholders left, we treat it is a error and format the template as normal
    if !placeholder_map.is_empty() {
        return Err(FormatError::SyntaxError);
    }

    // template chunks are formatted by the embedded language formatter, so we need to tell the formatter to ignore them
    for element in elements.iter() {
        if let AnyJsTemplateElement::JsTemplateChunkElement(element) = element {
            let token = element.template_chunk_token()?;
            write!(f, [&format_removed(&token)])?;
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
    let is_styled_jsx = is_styled_jsx(template)?;
    let is_styled_component = is_styled_component(template)?;
    Ok(is_styled_jsx || is_styled_component)
}

fn is_styled_jsx(template: &JsTemplateExpression) -> SyntaxResult<bool> {
    let tag = template.tag();
    if let Some(tag) = tag {
        // css``
        // css.global``
        // css.resolve``
        match tag {
            AnyJsExpression::JsIdentifierExpression(ident_expr) => {
                let name = ident_expr.name()?;
                return Ok(name.has_name("css"));
            }
            AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                let object = member_expr.object()?;
                let member = member_expr.member()?;
                if let AnyJsExpression::JsIdentifierExpression(object) = object {
                    if let AnyJsName::JsName(member) = member {
                        return Ok(object.name()?.has_name("css")
                            && member.value_token()?.text_trimmed() == "global");
                    }
                }
            }
            _ => {}
        }
    }

    // <style jsx>{`div{color:red}`}</style>
    let parent = template.parent::<JsxExpressionChild>();
    if let Some(jsx_expr_child) = parent {
        let parent = jsx_expr_child.parent::<JsxChildList>();
        if let Some(jsx_child_list) = parent {
            let parent = jsx_child_list.parent::<JsxElement>();
            if let Some(jsx_element) = parent {
                let opening_element = jsx_element.opening_element()?;
                let name = opening_element.name()?;
                if let AnyJsxElementName::JsxName(name) = name {
                    if name.value_token()?.text_trimmed() == "style" {
                        let attributes = opening_element.attributes();
                        // only have one attribute and it is jsx attribute
                        let count = attributes.len();
                        if count == 1 {
                            if let Some(AnyJsxAttribute::JsxAttribute(attr)) = attributes.first() {
                                let name = attr.name()?;
                                if let AnyJsxAttributeName::JsxName(name) = name {
                                    if name.value_token()?.text_trimmed() == "jsx" {
                                        return Ok(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(false)
}

// styled.foo
fn is_styled_identifier(template: &JsIdentifierExpression) -> SyntaxResult<bool> {
    let name = template.name()?;
    Ok(name.has_name("styled"))
}

// Component.extend
fn is_styled_extend(template: &JsStaticMemberExpression) -> SyntaxResult<bool> {
    let object = template.object()?;
    let member = template.member()?;
    if let AnyJsExpression::JsIdentifierExpression(ident_expr) = object {
        let name = ident_expr.name()?.text();
        // name startwith capital letter
        let is_first_char_uppercase = name.chars().next().map_or(false, |c| c.is_uppercase());
        if is_first_char_uppercase {
            if let AnyJsName::JsName(member) = member {
                return Ok(member.value_token()?.text_trimmed() == "extend");
            }
        }
    }
    Ok(false)
}

fn is_styled_component(template: &JsTemplateExpression) -> SyntaxResult<bool> {
    let tag = match template.tag() {
        Some(AnyJsExpression::JsParenthesizedExpression(paren_expr)) => paren_expr.expression()?,
        Some(expr) => expr,
        _ => return Ok(false),
    };

    match tag {
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            let is_styled_extend = is_styled_extend(&member_expr)?;
            let is_styled_identifier = {
                let object = member_expr.object()?;
                if let AnyJsExpression::JsIdentifierExpression(ident_expr) = object {
                    is_styled_identifier(&ident_expr)?
                } else {
                    false
                }
            };
            return Ok(is_styled_extend || is_styled_identifier);
        }
        AnyJsExpression::JsCallExpression(call_expr) => {
            let callee = call_expr.callee()?;
            match callee {
                AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                    // styled.foo.attrs({})``
                    if let AnyJsExpression::JsStaticMemberExpression(member_expr) =
                        member_expr.object()?
                    {
                        if let AnyJsExpression::JsIdentifierExpression(ident_expr) =
                            member_expr.object()?
                        {
                            return is_styled_identifier(&ident_expr);
                        }
                    }
                    // styled(Component).attrs({})``
                    if let AnyJsExpression::JsIdentifierExpression(ident_expr) =
                        member_expr.object()?
                    {
                        return is_styled_identifier(&ident_expr);
                    }
                    // Component.extend.attrs({})``
                    return is_styled_extend(&member_expr);
                }
                AnyJsExpression::JsIdentifierExpression(ident_expr) => {
                    return is_styled_identifier(&ident_expr);
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(false)
}
