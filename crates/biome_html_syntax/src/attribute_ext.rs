use crate::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnySvelteTemplateElement, AnyVueDirective,
    HtmlAttribute, HtmlAttributeList, HtmlAttributeName, static_value::StaticValue,
};
use biome_aria::Attribute;
use biome_rowan::{AstNodeList, TokenText};
use biome_string_case::StrOnlyExtension;

/// Extracts a static value from a Vue directive's binding value.
///
/// For Vue bindings like `:attr="'value'"` (HTML string containing a JS string literal),
/// returns a static value. Returns `None` for dynamic expressions (plain identifiers).
fn vue_binding_static_value(value: AnyHtmlAttributeInitializer) -> Option<StaticValue> {
    match value {
        AnyHtmlAttributeInitializer::HtmlString(ref string) => {
            let token = string.value_token().ok()?;
            let text = token.text_trimmed();
            // Strip the HTML attribute outer quotes to get the JS expression
            if text.len() < 2 {
                return None;
            }
            let inner = &text[1..text.len() - 1];
            // Only return a static value if the inner content is a JS string literal
            // (starts and ends with the same quote character). Plain identifiers like
            // `roleValue` are dynamic references and should not be treated as static.
            if inner.len() >= 2
                && ((inner.starts_with('"') && inner.ends_with('"'))
                    || (inner.starts_with('\'') && inner.ends_with('\'')))
            {
                Some(StaticValue::String(token))
            } else {
                None
            }
        }
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) => {
            expression.expression().ok()?.as_static_value()
        }
        _ => None,
    }
}

impl AnyHtmlAttributeInitializer {
    /// Returns the string value of the attribute, if available, without quotes.
    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            Self::HtmlString(string) => {
                let token = string.value_token().ok()?;
                Some(StaticValue::String(token))
            }
            Self::HtmlAttributeSingleTextExpression(expression) => {
                expression.expression().ok()?.as_static_value()
            }
            // When all elements are plain text chunks (no {expression} interpolations),
            // the static text can be recovered by concatenating the chunk tokens.
            Self::SvelteTemplateAttributeValue(value) => {
                let elements = value.elements();
                let mut tokens = Vec::new();
                for element in elements.iter() {
                    match element {
                        AnySvelteTemplateElement::SvelteTemplateChunkElement(chunk) => {
                            tokens.push(chunk.html_template_chunk_token().ok()?);
                        }
                        AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(_) => {
                            // Dynamic interpolation — no static value available.
                            return None;
                        }
                    }
                }
                tokens
                    .first()
                    .map(|token| StaticValue::String(token.clone()))
            }
            _ => None,
        }
    }
}

impl HtmlAttribute {
    /// Extracts the value from an attribute's initializer.
    ///
    /// Returns `None` if the attribute has no initializer or the value cannot be extracted.
    pub fn as_static_value(&self) -> Option<StaticValue> {
        self.initializer()?.value().ok()?.as_static_value()
    }
}

impl HtmlAttributeName {
    /// Returns the token text of the attribute name.
    pub fn token_text(&self) -> Option<TokenText> {
        self.value_token().ok().map(|token| token.token_text())
    }

    /// Returns the trimmed token text of the attribute name.
    pub fn token_text_trimmed(&self) -> Option<TokenText> {
        self.value_token()
            .ok()
            .map(|token| token.token_text_trimmed())
    }
}

impl Attribute for AnyHtmlAttribute {
    fn name(&self) -> Option<impl AsRef<str>> {
        self.name()
    }

    fn value(&self) -> Option<impl AsRef<str>> {
        self.as_static_value()
    }
}

impl AnyHtmlAttribute {
    pub fn name(&self) -> Option<TokenText> {
        match self {
            Self::HtmlAttribute(attr) => attr.name().ok()?.token_text_trimmed(),
            Self::AnyVueDirective(vue) => match vue {
                // :attr="..." — shorthand Vue binding
                AnyVueDirective::VueVBindShorthandDirective(d) => d
                    .arg()
                    .ok()
                    .and_then(|arg| arg.arg().ok())
                    .and_then(|arg| arg.as_vue_static_argument().cloned())
                    .and_then(|s| s.name_token().ok())
                    .map(|t| t.token_text_trimmed()),
                // v-bind:attr="..." — full Vue binding
                AnyVueDirective::VueDirective(d) if d.is_binding() => d
                    .arg()
                    .and_then(|arg| arg.arg().ok())
                    .and_then(|arg| arg.as_vue_static_argument().cloned())
                    .and_then(|s| s.name_token().ok())
                    .map(|t| t.token_text_trimmed()),
                _ => None,
            },
            Self::AnySvelteDirective(_)
            | Self::HtmlAttributeDoubleTextExpression(_)
            | Self::HtmlAttributeSingleTextExpression(_)
            | Self::HtmlBogusAttribute(_)
            | Self::HtmlSpreadAttribute(_)
            | Self::AnyAstroDirective(_)
            | Self::SvelteAttachAttribute(_) => None,
        }
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            Self::HtmlAttribute(attr) => attr.as_static_value(),
            Self::AnyVueDirective(vue) => match vue {
                AnyVueDirective::VueVBindShorthandDirective(d) => {
                    vue_binding_static_value(d.initializer()?.value().ok()?)
                }
                AnyVueDirective::VueDirective(d) if d.is_binding() => {
                    vue_binding_static_value(d.initializer()?.value().ok()?)
                }
                _ => None,
            },
            Self::AnySvelteDirective(_)
            | Self::HtmlAttributeDoubleTextExpression(_)
            | Self::HtmlAttributeSingleTextExpression(_)
            | Self::HtmlBogusAttribute(_)
            | Self::HtmlSpreadAttribute(_)
            | Self::AnyAstroDirective(_)
            | Self::SvelteAttachAttribute(_) => None,
        }
    }

    pub fn is_attribute_or_vue_binding(&self, name_to_lookup: &str) -> bool {
        match self {
            Self::HtmlAttribute(a) => a
                .name()
                .ok()
                .and_then(|n| n.value_token().ok())
                .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup)),

            Self::HtmlAttributeSingleTextExpression(single_text_expr) => {
                let Some(expression) = single_text_expr.expression().ok() else {
                    return false;
                };
                let Some(name) = expression.string_value() else {
                    return false;
                };
                name.eq_ignore_ascii_case(name_to_lookup)
            }

            Self::AnyVueDirective(vue) => match vue {
                // :name="..."
                AnyVueDirective::VueVBindShorthandDirective(d) => d
                    .arg()
                    .ok()
                    .and_then(|arg| arg.arg().ok())
                    .and_then(|arg| arg.as_vue_static_argument().cloned())
                    .and_then(|s| s.name_token().ok())
                    .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup)),

                // v-bind:name="..."
                AnyVueDirective::VueDirective(d) => {
                    d.is_binding()
                        && d.arg()
                            .and_then(|arg| arg.arg().ok())
                            .and_then(|arg| arg.as_vue_static_argument().cloned())
                            .and_then(|s| s.name_token().ok())
                            .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup))
                }

                _ => false,
            },

            _ => false,
        }
    }

    pub fn is_attribute_or_vue_event_binding(&self, name_to_lookup: &str) -> bool {
        match self {
            Self::HtmlAttribute(a) => a
                .name()
                .ok()
                .and_then(|n| n.value_token().ok())
                .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup)),

            Self::HtmlAttributeSingleTextExpression(single_text_expr) => {
                let Some(expression) = single_text_expr.expression().ok() else {
                    return false;
                };
                let Some(name) = expression.string_value() else {
                    return false;
                };
                name.eq_ignore_ascii_case(name_to_lookup)
            }

            Self::AnyVueDirective(vue) => {
                let stripped_name_to_lookup =
                    name_to_lookup.strip_prefix("on").unwrap_or(name_to_lookup);
                match vue {
                    // @name="..."
                    AnyVueDirective::VueVOnShorthandDirective(d) => d
                        .arg()
                        .ok()
                        .and_then(|arg| arg.as_vue_static_argument().cloned())
                        .and_then(|s| s.name_token().ok())
                        .is_some_and(|t| {
                            t.text_trimmed()
                                .eq_ignore_ascii_case(stripped_name_to_lookup)
                        }),

                    // v-on:name="..."
                    AnyVueDirective::VueDirective(d) => {
                        d.is_event_listener()
                            && d.arg()
                                .and_then(|arg| arg.arg().ok())
                                .and_then(|arg| arg.as_vue_static_argument().cloned())
                                .and_then(|s| s.name_token().ok())
                                .is_some_and(|t| {
                                    t.text_trimmed()
                                        .eq_ignore_ascii_case(stripped_name_to_lookup)
                                })
                    }

                    _ => false,
                }
            }

            _ => false,
        }
    }
}

impl biome_aria::Attribute for HtmlAttribute {
    fn name(&self) -> Option<impl AsRef<str>> {
        // HTML attribute names are case-insensitive; lowercase for matching
        Some(
            self.name()
                .ok()?
                .value_token()
                .ok()?
                .text_trimmed()
                .to_lowercase_cow()
                .into_owned(),
        )
    }

    fn value(&self) -> Option<impl AsRef<str>> {
        self.initializer()?.value().ok()?.as_static_value()
    }
}

impl HtmlAttributeList {
    /// Finds an attribute by name (case-insensitive) within this list of attributes.
    ///
    /// This will not detect attributes in Svelte attribute shorthand like `<div {foo}>`, or vue bindings like `<div :foo="...">` or `<div v-bind:foo="...">`.
    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<AnyHtmlAttribute> {
        self.iter().find_map(|attr| match attr.clone() {
            AnyHtmlAttribute::HtmlAttribute(html_attr) => {
                let name = html_attr.name().ok()?;
                let name_token = name.value_token().ok()?;
                if name_token
                    .text_trimmed()
                    .eq_ignore_ascii_case(name_to_lookup)
                {
                    Some(attr.clone())
                } else {
                    None
                }
            }
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(single_text_expr) => {
                let expression = single_text_expr.expression().ok()?;
                let name = expression.string_value()?;
                if name.eq_ignore_ascii_case(name_to_lookup) {
                    Some(attr.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Finds multiple attributes by name (case-insensitive) within this list of attributes.
    ///
    /// Returns an array of `Option<HtmlAttribute>`, where each attribute corresponds to the name at the same index in `names_to_lookup`.
    pub fn find_multiple_attributes_by_name<const N: usize>(
        &self,
        names_to_lookup: &[&str; N],
    ) -> [Option<HtmlAttribute>; N] {
        const INIT: Option<HtmlAttribute> = None;
        let mut result: [Option<HtmlAttribute>; N] = [INIT; N];
        let mut remaining = N;

        for attr in self.iter() {
            if remaining == 0 {
                break;
            }

            let Some(attribute) = attr.as_html_attribute() else {
                continue;
            };
            let Ok(name) = attribute.name() else {
                continue;
            };
            let Ok(name_token) = name.value_token() else {
                continue;
            };
            let attribute_name = name_token.text_trimmed();

            for (index, name_to_lookup) in names_to_lookup.iter().enumerate() {
                if result[index].is_none() && attribute_name.eq_ignore_ascii_case(name_to_lookup) {
                    result[index] = Some(attribute.clone());
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
        }

        result
    }

    /// Check if the list has a given HTML attribute or a Vue v-bind binding
    /// targeting the same attribute name.
    ///
    /// Handles:
    /// - `name="..."` — standard HTML attribute
    /// - `:name="..."` — Vue v-bind shorthand (`VueVBindShorthandDirective`)
    /// - `v-bind:name="..."` — explicit Vue v-bind (`VueDirective`)
    pub fn find_attribute_or_vue_binding(&self, name_to_lookup: &str) -> Option<AnyHtmlAttribute> {
        self.iter().find_map(|attr| {
            let matches = attr.is_attribute_or_vue_binding(name_to_lookup);
            if matches { Some(attr) } else { None }
        })
    }

    /// Check if the list has a given HTML attribute or a Vue v-on binding
    /// targeting the same attribute name.
    ///
    /// Handles:
    /// - `name="..."` — standard HTML attribute
    /// - `@name="..."` — Vue v-on shorthand (`VueVOnShorthandDirective`)
    /// - `v-on:name="..."` — explicit Vue v-on (`VueDirective`)
    pub fn find_attribute_or_vue_event_binding(
        &self,
        name_to_lookup: &str,
    ) -> Option<AnyHtmlAttribute> {
        self.iter().find_map(|attr| {
            let matches = attr.is_attribute_or_vue_event_binding(name_to_lookup);
            if matches { Some(attr) } else { None }
        })
    }

    /// Find a Vue event binding (either `@name` or `v-on:name`) by the target attribute name. This intentionally excludes standard HTML attributes, so it won't match `name="..."` even if it exists on the same element.
    ///
    /// Vue bindings are case sensitive, so this will only match if the case of `name_to_lookup` exactly matches the case used in the binding. For example, `@fooBar="..."` will only be matched by `find_vue_event_binding("fooBar")`, not `find_vue_event_binding("foobar")`.
    ///
    /// See also: [Self::find_attribute_or_vue_event_binding] which checks for both HTML attributes and Vue event bindings.
    pub fn find_vue_event_binding(&self, name_to_lookup: &str) -> Option<AnyVueDirective> {
        self.iter().find_map(|attr| {
            let directive = attr.as_any_vue_directive()?;
            let matches = match directive {
                // @name="..."
                AnyVueDirective::VueVOnShorthandDirective(d) => d
                    .arg()
                    .ok()
                    .and_then(|arg| arg.as_vue_static_argument().cloned())
                    .and_then(|s| s.name_token().ok())
                    .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup)),

                // v-on:name="..."
                AnyVueDirective::VueDirective(d) => {
                    d.is_event_listener()
                        && d.arg()
                            .and_then(|arg| arg.arg().ok())
                            .and_then(|arg| arg.as_vue_static_argument().cloned())
                            .and_then(|s| s.name_token().ok())
                            .is_some_and(|t| t.text_trimmed().eq_ignore_ascii_case(name_to_lookup))
                }

                _ => false,
            };
            if matches {
                Some(directive.clone())
            } else {
                None
            }
        })
    }

    /// Find a Vue binding (either `:name` or `v-bind:name`) by the target attribute name. This intentionally excludes standard HTML attributes, so it won't match `name="..."` even if it exists on the same element.
    ///
    /// Vue bindings are case sensitive, so this will only match if the case of `name_to_lookup` exactly matches the case used in the binding. For example, `:fooBar="..."` will only be matched by `find_vue_binding("fooBar")`, not `find_vue_binding("foobar")`.
    ///
    /// See also: [Self::find_attribute_or_vue_binding] which checks for both HTML attributes and Vue bindings.
    pub fn find_vue_binding(&self, name_to_lookup: &str) -> Option<AnyVueDirective> {
        self.iter().find_map(|attr| {
            let directive = attr.as_any_vue_directive()?;
            let matches = match directive {
                // :name="..."
                AnyVueDirective::VueVBindShorthandDirective(d) => d
                    .arg()
                    .ok()
                    .and_then(|arg| arg.arg().ok())
                    .and_then(|arg| arg.as_vue_static_argument().cloned())
                    .and_then(|s| s.name_token().ok())
                    .is_some_and(|t| t.text_trimmed() == name_to_lookup),

                // v-bind:name="..."
                AnyVueDirective::VueDirective(d) => {
                    d.is_binding()
                        && d.arg()
                            .and_then(|arg| arg.arg().ok())
                            .and_then(|arg| arg.as_vue_static_argument().cloned())
                            .and_then(|s| s.name_token().ok())
                            .is_some_and(|t| t.text_trimmed() == name_to_lookup)
                }

                _ => false,
            };
            if matches {
                Some(directive.clone())
            } else {
                None
            }
        })
    }
}
