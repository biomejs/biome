use std::collections::HashSet;

use crate::{
    inner_string_text, static_value::StaticValue, AnyJsxAttribute, AnyJsxAttributeName,
    AnyJsxAttributeValue, AnyJsxChild, AnyJsxElementName, AnyJsxTag, JsSyntaxToken, JsxAttribute,
    JsxAttributeList, JsxElement, JsxOpeningElement, JsxSelfClosingElement, JsxString,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxResult, TokenText};

impl JsxString {
    /// Returns the inner text of a string not including the quotes.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    ///let string = make::jsx_string(make::jsx_string_literal("button")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "button");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl AnyJsxAttributeName {
    pub fn namespace(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsxName(_) => None,
            Self::JsxNamespaceName(name) => name.namespace().ok()?.value_token().ok(),
        }
    }

    pub fn name(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsxName(name) => name.value_token(),
            Self::JsxNamespaceName(name) => name.name()?.value_token(),
        }
    }
}

impl AnyJsxTag {
    pub fn name(&self) -> Option<AnyJsxElementName> {
        match self {
            Self::JsxElement(element) => element.opening_element().ok()?.name().ok(),
            Self::JsxFragment(_) => None,
            Self::JsxSelfClosingElement(element) => element.name().ok(),
        }
    }

    pub fn attributes(&self) -> Option<JsxAttributeList> {
        match self {
            Self::JsxElement(element) => Some(element.opening_element().ok()?.attributes()),
            Self::JsxFragment(_) => None,
            Self::JsxSelfClosingElement(element) => Some(element.attributes()),
        }
    }
}

impl JsxOpeningElement {
    /// Find and return the [JsxAttribute] that matches the given name.
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use biome_js_factory::make;
    /// use biome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list};
    /// use biome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
    ///
    /// let div = AnyJsxAttribute::JsxAttribute(jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let img = AnyJsxAttribute::JsxAttribute(jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("img"))
    ///     )
    /// ).build());
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     img
    /// ]);
    ///
    /// let opening_element = jsx_opening_element(
    ///     token(T![<]),
    ///     AnyJsxElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![>]),
    /// ).build();
    ///
    /// assert!(opening_element.find_attribute_by_name("div").is_some());
    /// assert!(opening_element.find_attribute_by_name("img").is_some());
    /// assert!(!opening_element.find_attribute_by_name("p").is_some());
    /// ```
    ///
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<JsxAttribute> {
        self.attributes().find_by_name(name_to_lookup)
    }

    /// Returns `true` if the current attribute has a trailing spread props.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use biome_js_syntax::{AnyJsExpression, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
    ///
    /// let div = jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build();
    ///
    /// let spread = AnyJsxAttribute::from(jsx_spread_attribute(
    ///     token(T!['{']),
    ///     token(T![...]),
    ///     AnyJsExpression::JsIdentifierExpression(js_identifier_expression(
    ///         js_reference_identifier(ident("spread"))
    ///     )),
    ///     token(T!['}']),
    /// ));
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     AnyJsxAttribute::from(div.clone()),
    ///     spread
    /// ]);
    ///
    /// let opening_element = jsx_opening_element(
    ///     token(T![<]),
    ///     AnyJsxElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![>]),
    /// ).build();
    ///
    /// let div = opening_element.find_attribute_by_name("div").unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(&div));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
    }

    /// Returns `true` if jsx element has a child that is accessible.
    pub fn has_accessible_child(&self) -> bool {
        self.parent::<JsxElement>().is_some_and(|parent| {
            parent
                .children()
                .into_iter()
                .any(|child| child.is_accessible_node().unwrap_or(true))
        })
    }
}

impl JsxSelfClosingElement {
    /// Returns the [JsxAttribute] that matches `name_to_lookup`.
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use biome_js_factory::make;
    /// use biome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element};
    /// use biome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
    ///
    /// let div = AnyJsxAttribute::JsxAttribute(jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let img = AnyJsxAttribute::JsxAttribute(jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("img"))
    ///     )
    /// ).build());
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     img
    /// ]);
    ///
    /// let opening_element = jsx_self_closing_element(
    ///     token(T![<]),
    ///     AnyJsxElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![/]),
    ///     token(T![>]),
    /// ).build();
    ///
    /// assert!(opening_element.find_attribute_by_name("div").is_some());
    /// assert!(opening_element.find_attribute_by_name("img").is_some());
    /// assert!(!opening_element.find_attribute_by_name("p").is_some());
    /// ```
    ///
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<JsxAttribute> {
        self.attributes().find_by_name(name_to_lookup)
    }

    /// Returns `true` if the current attribute has a trailing spread props.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use biome_js_syntax::{AnyJsExpression, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
    ///
    /// let div = AnyJsxAttribute::JsxAttribute(jsx_attribute(
    ///     AnyJsxAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let spread = AnyJsxAttribute::JsxSpreadAttribute(jsx_spread_attribute(
    ///     token(T!['{']),
    ///     token(T![...]),
    ///     AnyJsExpression::JsIdentifierExpression(js_identifier_expression(
    ///         js_reference_identifier(ident("spread"))
    ///     )),
    ///     token(T!['}']),
    /// ));
    ///
    ///
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     spread
    /// ]);
    ///
    /// let opening_element = jsx_self_closing_element(
    ///     token(T![<]),
    ///     AnyJsxElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![/]),
    ///     token(T![>]),
    /// ).build();
    ///
    /// let div = opening_element.find_attribute_by_name("div").unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(&div));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
    }
}

impl JsxAttributeList {
    /// Returns the [JsxAttribute] that match the given `names_to_lookup`.
    /// Only attributes with name as [AnyJsxAttributeName::JsxName] can be returned.
    ///
    /// Each name of `names_to_lookup` must be unique.
    ///
    /// Supports a maximum of 16 names to avoid stack overflow. Each attribute will consume:
    ///
    /// - 8 bytes for the `Option<JsxAttribute>` result;
    /// - plus 16 bytes for the [&str] argument.
    pub fn find_by_names<const N: usize>(
        &self,
        names_to_lookup: [&str; N],
    ) -> [Option<JsxAttribute>; N] {
        // assert there are no duplicates
        debug_assert!(HashSet::<_>::from_iter(names_to_lookup).len() == N);
        debug_assert!(N <= 16);

        const INIT: Option<JsxAttribute> = None;
        let mut results = [INIT; N];
        let mut missing = N;

        for att in self {
            if let AnyJsxAttribute::JsxAttribute(attribute) = att {
                if let Some(name) = attribute
                    .name()
                    .ok()
                    .and_then(|x| x.as_jsx_name()?.value_token().ok())
                {
                    let name = name.text_trimmed();
                    for i in 0..N {
                        if results[i].is_none() && names_to_lookup[i] == name {
                            results[i] = Some(attribute);
                            if missing == 1 {
                                return results;
                            } else {
                                missing -= 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        results
    }

    pub fn find_by_name(&self, name_to_lookup: &str) -> Option<JsxAttribute> {
        self.iter().find_map(|attribute| {
            if let AnyJsxAttribute::JsxAttribute(attribute) = attribute {
                if let Ok(AnyJsxAttributeName::JsxName(name)) = attribute.name() {
                    if name.value_token().ok()?.text_trimmed() == name_to_lookup {
                        return Some(attribute);
                    }
                }
            }
            None
        })
    }

    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        let mut current_attribute_found = false;
        for attribute in self {
            if let Some(attribute) = attribute.as_jsx_attribute() {
                if attribute == current_attribute {
                    current_attribute_found = true;
                    continue;
                }
            }
            if current_attribute_found && attribute.as_jsx_spread_attribute().is_some() {
                return true;
            }
        }
        false
    }
}

impl AnyJsxElementName {
    pub fn name_value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsxElementName::JsxMemberName(member) => member.member()?.value_token(),
            AnyJsxElementName::JsxName(name) => name.value_token(),
            AnyJsxElementName::JsxNamespaceName(name) => name.name()?.value_token(),
            AnyJsxElementName::JsxReferenceIdentifier(name) => name.value_token(),
        }
    }
}

declare_node_union! {
    pub AnyJsxElement = JsxOpeningElement | JsxSelfClosingElement
}

impl AnyJsxElement {
    pub fn name(&self) -> SyntaxResult<AnyJsxElementName> {
        match self {
            AnyJsxElement::JsxOpeningElement(element) => element.name(),
            AnyJsxElement::JsxSelfClosingElement(element) => element.name(),
        }
    }

    pub fn attributes(&self) -> JsxAttributeList {
        match self {
            AnyJsxElement::JsxOpeningElement(element) => element.attributes(),
            AnyJsxElement::JsxSelfClosingElement(element) => element.attributes(),
        }
    }

    pub fn name_value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        self.name()?.name_value_token()
    }

    /// Returns `true` if the current element is actually a component.
    ///
    /// - `<Span />` is a component and it would return `true`
    /// - `<span ></span>` is **not** component and it returns `false`
    pub fn is_custom_component(&self) -> bool {
        self.name().is_ok_and(|it| it.as_jsx_name().is_none())
    }

    /// Returns `true` if the current element is an HTML element.
    ///
    /// - `<Span />` is a component and it would return `false`
    /// - `<span ></span>` is **not** component and it returns `true`
    pub fn is_element(&self) -> bool {
        self.name().is_ok_and(|it| it.as_jsx_name().is_some())
    }

    pub fn has_spread_prop(&self) -> bool {
        self.attributes()
            .into_iter()
            .any(|attribute| matches!(attribute, AnyJsxAttribute::JsxSpreadAttribute(_)))
    }

    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        match self {
            AnyJsxElement::JsxSelfClosingElement(element) => {
                element.has_trailing_spread_prop(current_attribute)
            }
            AnyJsxElement::JsxOpeningElement(element) => {
                element.has_trailing_spread_prop(current_attribute)
            }
        }
    }

    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<JsxAttribute> {
        match self {
            AnyJsxElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name(name_to_lookup)
            }
            AnyJsxElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name(name_to_lookup)
            }
        }
    }

    /// Returns the attribute value of [JsxString] attributes.
    ///
    /// ```
    /// use biome_js_syntax::jsx_ext::AnyJsxElement;
    /// use biome_js_factory::make::{ident, js_boolean_literal_expression, jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_expression_attribute_value, jsx_name, jsx_self_closing_element, jsx_string, jsx_string_literal, token};
    /// use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, AnyJsxElementName, T};
    ///
    /// let string_attr = AnyJsxAttribute::JsxAttribute(
    ///     jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(ident("type"))))
    ///         .with_initializer(jsx_attribute_initializer_clause(
    ///             token(T![=]),
    ///             AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("button"))),
    ///         ))
    ///         .build()
    /// );
    ///
    /// let boolean_attr = AnyJsxAttribute::JsxAttribute(
    ///     jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(ident("disabled"))))
    ///         .with_initializer(jsx_attribute_initializer_clause(
    ///             token(T![=]),
    ///             AnyJsxAttributeValue::JsxExpressionAttributeValue(
    ///                 jsx_expression_attribute_value(
    ///                     token(T!['{']),
    ///                     AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsBooleanLiteralExpression(js_boolean_literal_expression(token(T![true])))),
    ///                     token(T!['}']),
    ///                 )
    ///             ))
    ///         )
    ///         .build()
    /// );
    ///
    /// let attributes = jsx_attribute_list(vec![boolean_attr, string_attr]);
    ///
    /// let jsx_element = AnyJsxElement::JsxSelfClosingElement(
    ///   jsx_self_closing_element(
    ///       token(T![<]),
    ///       AnyJsxElementName::JsxName(
    ///           jsx_name(ident("button"))
    ///       ),
    ///       attributes,
    ///       token(T![/]),
    ///       token(T![>]),
    ///   ).build()
    /// );
    ///
    /// assert!(jsx_element.get_attribute_inner_string_text("unknown").is_none());
    /// assert!(jsx_element.get_attribute_inner_string_text("disabled").is_none());
    /// assert_eq!(jsx_element.get_attribute_inner_string_text("type").unwrap(), "button");
    ///```
    ///
    pub fn get_attribute_inner_string_text(&self, name_to_lookup: &str) -> Option<TokenText> {
        if let Some(attr) = self.find_attribute_by_name(name_to_lookup) {
            let initializer = attr.initializer()?.value().ok()?;
            initializer.as_jsx_string()?.inner_string_text().ok()
        } else {
            None
        }
    }

    pub fn has_truthy_attribute(&self, name_to_lookup: &str) -> bool {
        self.find_attribute_by_name(name_to_lookup)
            .is_some_and(|attribute| {
                attribute
                    .as_static_value()
                    .map_or(true, |value| !(value.is_falsy() || value.text() == "false"))
                    && !self.has_trailing_spread_prop(&attribute)
            })
    }
}
impl biome_aria::Element for AnyJsxElement {
    fn name(&self) -> Option<impl AsRef<str>> {
        Self::name(self).ok().and_then(|name| {
            if let AnyJsxElementName::JsxName(name) = name {
                Some(name.value_token().ok()?.token_text_trimmed())
            } else {
                None
            }
        })
    }

    fn attributes(&self) -> impl Iterator<Item = impl biome_aria::Attribute> {
        Self::attributes(self).into_iter().filter_map(|attribute| {
            let AnyJsxAttribute::JsxAttribute(attribute) = attribute else {
                return None;
            };
            Some(attribute)
        })
    }
}

impl JsxAttribute {
    pub fn is_value_null_or_undefined(&self) -> bool {
        self.as_static_value()
            .is_some_and(|it| it.is_null_or_undefined())
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        self.initializer()?.value().ok()?.as_static_value()
    }

    pub fn name_value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self.name()? {
            AnyJsxAttributeName::JsxName(name) => name.value_token(),
            AnyJsxAttributeName::JsxNamespaceName(name) => name.name()?.value_token(),
        }
    }
}
impl biome_aria::Attribute for JsxAttribute {
    fn name(&self) -> Option<impl AsRef<str>> {
        Self::name(self).ok().and_then(|name| {
            if let AnyJsxAttributeName::JsxName(name) = name {
                Some(name.value_token().ok()?.token_text_trimmed())
            } else {
                None
            }
        })
    }

    fn value(&self) -> Option<impl AsRef<str>> {
        self.initializer()?.value().ok()?.as_static_value()
    }
}

impl AnyJsxAttributeName {
    pub fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsxName(jsx_name) => jsx_name.value_token(),
            Self::JsxNamespaceName(jsx_namespace_name) => jsx_namespace_name
                .name()
                .and_then(|jsx_name| jsx_name.value_token()),
        }
    }

    pub fn namespace_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsxName(_) => None,
            Self::JsxNamespaceName(jsx_namespace_name) => jsx_namespace_name
                .namespace()
                .and_then(|namespace| namespace.value_token())
                .ok(),
        }
    }
}

impl AnyJsxAttributeValue {
    pub fn is_value_null_or_undefined(&self) -> bool {
        self.as_static_value()
            .is_some_and(|it| it.is_null_or_undefined())
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            AnyJsxAttributeValue::AnyJsxTag(_) => None,
            AnyJsxAttributeValue::JsxExpressionAttributeValue(expression) => {
                expression.expression().ok()?.as_static_value()
            }
            AnyJsxAttributeValue::JsxString(string) => {
                Some(StaticValue::String(string.value_token().ok()?))
            }
        }
    }
}

impl AnyJsxChild {
    /// Check if jsx child node is accessible for screen readers
    pub fn is_accessible_node(&self) -> Option<bool> {
        Some(match self {
            AnyJsxChild::JsxText(text) => {
                let value_token = text.value_token().ok()?;
                value_token.text_trimmed().trim() != ""
            }
            AnyJsxChild::JsxExpressionChild(expression) => {
                let expression = expression.expression()?;
                expression
                    .as_static_value()
                    .map_or(true, |value| !value.is_falsy())
            }
            AnyJsxChild::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let jsx_element = AnyJsxElement::cast(opening_element.into_syntax())?;

                // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
                // since we don't have enough information about how the property is used.
                jsx_element.is_custom_component()
                    || !jsx_element.has_truthy_attribute("aria-hidden")
            }
            AnyJsxChild::JsxSelfClosingElement(element) => {
                let jsx_element = AnyJsxElement::unwrap_cast(element.syntax().clone());
                jsx_element.is_custom_component()
                    || !jsx_element.has_truthy_attribute("aria-hidden")
            }
            AnyJsxChild::JsxFragment(fragment) => fragment
                .children()
                .into_iter()
                .any(|child| child.is_accessible_node().unwrap_or(true)),
            _ => true,
        })
    }
}
