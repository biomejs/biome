use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, Queryable,
    RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_aria::iso::{countries, is_valid_country, is_valid_language, languages};
use biome_aria::{AriaProperties, AriaRoles};
use biome_js_syntax::{AnyJsRoot, AnyJsxAttribute, JsLanguage, JsSyntaxNode, JsxAttributeList};
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AriaServices {
    pub(crate) roles: Arc<AriaRoles>,
    pub(crate) properties: Arc<AriaProperties>,
}

impl AriaServices {
    pub fn aria_roles(&self) -> &AriaRoles {
        &self.roles
    }

    pub fn aria_properties(&self) -> &AriaProperties {
        &self.properties
    }

    pub fn is_valid_iso_language(&self, language: &str) -> bool {
        is_valid_language(language)
    }

    pub fn is_valid_iso_country(&self, country: &str) -> bool {
        is_valid_country(country)
    }

    pub fn iso_country_list(&self) -> &'static [&'static str] {
        countries()
    }

    pub fn iso_language_list(&self) -> &'static [&'static str] {
        languages()
    }

    /// Parses a [JsxAttributeList] and extracts the names and values of each [JsxAttribute],
    /// returning them as a [FxHashMap]. Attributes with no specified value are given a value of "true".
    /// If an attribute has multiple values, each value is stored as a separate item in the
    /// [FxHashMap] under the same attribute name. Returns [None] if the parsing fails.
    pub fn extract_attributes(
        &self,
        attribute_list: &JsxAttributeList,
    ) -> Option<FxHashMap<String, Vec<AttributeValue>>> {
        let mut defined_attributes: FxHashMap<String, Vec<AttributeValue>> = FxHashMap::default();
        for attribute in attribute_list {
            if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
                let name = attr.name().ok()?.syntax().text_trimmed().to_string();
                let values = if let Some(initializer) = attr.initializer() {
                    let initializer = initializer.value().ok()?;
                    if let Some(static_value) = initializer.as_static_value() {
                        static_value
                            .text()
                            .split_whitespace()
                            .map(|s| AttributeValue::StaticValue(s.to_string()))
                            .collect()
                    } else {
                        vec![AttributeValue::DynamicValue(
                            initializer.syntax().text_trimmed().to_string(),
                        )]
                    }
                } else {
                    vec![AttributeValue::StaticValue("true".to_string())]
                };

                defined_attributes.entry(name).or_insert(values);
            }
        }
        Some(defined_attributes)
    }

    pub fn convert_all_attribute_values(
        &self,
        attributes: Option<FxHashMap<String, Vec<AttributeValue>>>,
    ) -> Option<FxHashMap<String, Vec<String>>> {
        attributes.map(|attr_map| {
            attr_map
                .into_iter()
                .map(|(key, values)| {
                    let string_values = self.convert_attribute_values(values);
                    (key, string_values)
                })
                .collect()
        })
    }

    pub fn convert_attribute_values(&self, values: Vec<AttributeValue>) -> Vec<String> {
        values
            .into_iter()
            .map(|value| match value {
                AttributeValue::StaticValue(s) => s,
                AttributeValue::DynamicValue(s) => s,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    StaticValue(String),
    DynamicValue(String),
}

impl FromServices for AriaServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let roles: &Arc<AriaRoles> = services
            .get_service()
            .ok_or_else(|| MissingServicesDiagnostic::new(rule_key.rule_name(), &["AriaRoles"]))?;
        let properties: &Arc<AriaProperties> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["AriaProperties"])
        })?;
        Ok(Self {
            roles: roles.clone(),
            properties: properties.clone(),
        })
    }
}

impl Phase for AriaServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Aria<N>(pub N);

impl<N> Queryable for Aria<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = AriaServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::services::aria::{AriaServices, AttributeValue};
    use biome_aria::{AriaProperties, AriaRoles};
    use biome_js_factory::make::{
        ident, jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_name,
        jsx_string, jsx_string_literal, token,
    };
    use biome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, T};
    use std::sync::Arc;

    #[test]
    fn test_extract_attributes() {
        // Assume attributes of `<div class="wrapper document" role="article"></div>`
        let attribute_list = jsx_attribute_list(vec![
            AnyJsxAttribute::JsxAttribute(
                jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(ident("class"))))
                    .with_initializer(jsx_attribute_initializer_clause(
                        token(T![=]),
                        AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal(
                            "wrapper document",
                        ))),
                    ))
                    .build(),
            ),
            AnyJsxAttribute::JsxAttribute(
                jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(ident("role"))))
                    .with_initializer(jsx_attribute_initializer_clause(
                        token(T![=]),
                        AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("article"))),
                    ))
                    .build(),
            ),
        ]);
        let services = AriaServices {
            roles: Arc::new(AriaRoles {}),
            properties: Arc::new(AriaProperties {}),
        };

        let attribute_name_to_values = services.extract_attributes(&attribute_list).unwrap();

        assert_eq!(
            attribute_name_to_values["class"],
            vec![
                AttributeValue::StaticValue("wrapper".to_string()),
                AttributeValue::StaticValue("document".to_string())
            ]
        );
        assert_eq!(
            attribute_name_to_values["role"],
            vec![AttributeValue::StaticValue("article".to_string())]
        );
    }
}
