use std::collections::HashSet;

use crate::{
    inner_string_text, static_value::StaticValue, AnyJsxAttribute, AnyJsxAttributeName,
    AnyJsxAttributeValue, AnyJsxChild, AnyJsxElementName, JsSyntaxToken, JsxAttribute,
    JsxAttributeList, JsxElement, JsxName, JsxOpeningElement, JsxSelfClosingElement, JsxString,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxResult, TokenText};

impl JsxString {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make;
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

impl JsxOpeningElement {
    /// Find and return the `JsxAttribute` that matches the given name
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list};
    /// use rome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
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
    /// assert_eq!(opening_element.find_attribute_by_name("div").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("img").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("p").unwrap().is_some(), false);
    /// ```
    ///
    pub fn find_attribute_by_name(
        &self,
        name_to_lookup: &str,
    ) -> SyntaxResult<Option<JsxAttribute>> {
        self.attributes().find_by_name(name_to_lookup)
    }

    /// It checks if current attribute has a trailing spread props
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use rome_js_syntax::{AnyJsExpression, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
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
    /// let div = opening_element.find_attribute_by_name("div").unwrap().unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(&div));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
    }

    /// Check if jsx element has a child that is accessible
    pub fn has_accessible_child(&self) -> bool {
        self.parent::<JsxElement>().map_or(false, |parent| {
            parent
                .children()
                .into_iter()
                .any(|child| child.is_accessible_node().unwrap_or(true))
        })
    }
}

impl JsxSelfClosingElement {
    /// Find and return the `JsxAttribute` that matches the given name
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element};
    /// use rome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
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
    /// assert_eq!(opening_element.find_attribute_by_name("div").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("img").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("p").unwrap().is_some(), false);
    /// ```
    ///
    pub fn find_attribute_by_name(
        &self,
        name_to_lookup: &str,
    ) -> SyntaxResult<Option<JsxAttribute>> {
        self.attributes().find_by_name(name_to_lookup)
    }

    /// It checks if current attribute has a trailing spread props
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use rome_js_syntax::{AnyJsExpression, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxElementName, T};
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
    /// let div = opening_element.find_attribute_by_name("div").unwrap().unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(&div));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
    }
}

impl JsxAttributeList {
    /// Finds and returns attributes `JsxAttribute` that matches the given names like [Self::find_by_name].
    /// Only attributes with name as [JsxName] can be returned.
    ///
    /// Each name of "names_to_lookup" should be unique.
    ///
    /// Supports maximum of 16 names to avoid stack overflow. Each attribute will consume:
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

        'attributes: for att in self {
            if let Some(attribute) = att.as_jsx_attribute() {
                if let Some(name) = attribute
                    .name()
                    .ok()
                    .and_then(|x| x.as_jsx_name()?.value_token().ok())
                {
                    let name = name.text_trimmed();
                    for i in 0..N {
                        if results[i].is_none() && names_to_lookup[i] == name {
                            results[i] = Some(attribute.clone());
                            if missing == 1 {
                                break 'attributes;
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

    pub fn find_by_name(&self, name_to_lookup: &str) -> SyntaxResult<Option<JsxAttribute>> {
        let attribute = self.iter().find_map(|attribute| {
            let attribute = JsxAttribute::cast_ref(attribute.syntax())?;
            let name = attribute.name().ok()?;
            let name = JsxName::cast_ref(name.syntax())?;
            if name.value_token().ok()?.text_trimmed() == name_to_lookup {
                Some(attribute)
            } else {
                None
            }
        });

        Ok(attribute)
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
    pub fn name_value_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsxElementName::JsxMemberName(member) => member.member().ok()?.value_token().ok(),
            AnyJsxElementName::JsxName(name) => name.value_token().ok(),
            AnyJsxElementName::JsxNamespaceName(name) => name.name().ok()?.value_token().ok(),
            AnyJsxElementName::JsxReferenceIdentifier(name) => name.value_token().ok(),
        }
    }
}

declare_node_union! {
    pub AnyJsxElement = JsxOpeningElement | JsxSelfClosingElement
}

impl AnyJsxElement {
    pub fn attributes(&self) -> JsxAttributeList {
        match self {
            AnyJsxElement::JsxOpeningElement(element) => element.attributes(),
            AnyJsxElement::JsxSelfClosingElement(element) => element.attributes(),
        }
    }

    pub fn name(&self) -> SyntaxResult<AnyJsxElementName> {
        match self {
            AnyJsxElement::JsxOpeningElement(element) => element.name(),
            AnyJsxElement::JsxSelfClosingElement(element) => element.name(),
        }
    }

    pub fn name_value_token(&self) -> Option<JsSyntaxToken> {
        self.name().ok()?.name_value_token()
    }

    /// Return true if the current element is actually a component
    ///
    /// - `<Span />` is a component and it would return `true`
    /// - `<span ></span>` is **not** component and it returns `false`
    pub fn is_custom_component(&self) -> bool {
        self.name().map_or(false, |it| it.as_jsx_name().is_none())
    }

    /// Return true if the current element is an HTML element
    ///
    /// - `<Span />` is a component and it would return `false`
    /// - `<span ></span>` is **not** component and it returns `true`
    pub fn is_element(&self) -> bool {
        self.name().map_or(false, |it| it.as_jsx_name().is_some())
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
                element.find_attribute_by_name(name_to_lookup).ok()?
            }
            AnyJsxElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name(name_to_lookup).ok()?
            }
        }
    }

    pub fn has_truthy_attribute(&self, name_to_lookup: &str) -> bool {
        self.find_attribute_by_name(name_to_lookup)
            .map_or(false, |attribute| {
                attribute
                    .as_static_value()
                    .map_or(true, |value| !(value.is_falsy() || value.text() == "false"))
                    && !self.has_trailing_spread_prop(&attribute)
            })
    }
}

impl JsxAttribute {
    pub fn is_value_null_or_undefined(&self) -> bool {
        self.as_static_value()
            .map_or(false, |it| it.is_null_or_undefined())
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        self.initializer()?.value().ok()?.as_static_value()
    }

    pub fn name_value_token(&self) -> Option<JsSyntaxToken> {
        match self.name().ok()? {
            AnyJsxAttributeName::JsxName(name) => name.value_token().ok(),
            AnyJsxAttributeName::JsxNamespaceName(name) => name.name().ok()?.value_token().ok(),
        }
    }
}

impl AnyJsxAttributeValue {
    pub fn is_value_null_or_undefined(&self) -> bool {
        self.as_static_value()
            .map_or(false, |it| it.is_null_or_undefined())
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
                let jsx_element = AnyJsxElement::cast(opening_element.syntax().clone())?;

                // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
                // since we don't have enough information about how the property is used.
                jsx_element.is_custom_component()
                    || !jsx_element.has_truthy_attribute("aria-hidden")
            }
            AnyJsxChild::JsxSelfClosingElement(element) => {
                let jsx_element = AnyJsxElement::cast(element.syntax().clone())?;
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
