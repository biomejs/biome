use crate::services::aria::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttribute, JsxAttributeList};
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

declare_rule! {
    /// Enforce that non-interactive, visible elements (such as `<div>`) that have click handlers use the role attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={()=>{})}></div>;
    /// ```
    /// ```jsx,expect_diagnostic
    /// <span onClick={()=>{})}></span>;
    /// ```
    ///
    /// When `<a>` does not have "href" attribute, that is non-interactive.
    /// ```jsx,expect_diagnostic
    /// <a onClick={()=>{})}></a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div role="button" onClick={()=>{})}></div>
    /// <span role="link" onClick={()=>{})}></span>
    /// <a href="http://example.com" onClick={()=>{})}></a>
    /// ```
    ///
    pub NoStaticElementInteractions {
        version: "next",
        name: "noStaticElementInteractions",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("no-static-element-interactions")],
        recommended: false,
    }
}

// These are interactions defined by eslint-plugin-jsx-a11y.
// ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/974275353598e9407c76bd4a50c331a953755cee/src/rules/no-static-element-interactions.js#L33-L37
// ref: https://github.com/jsx-eslint/jsx-ast-utils/blob/main/src/eventHandlers.js
lazy_static::lazy_static! {
    static ref EVENT_TO_HANDLERS: FxHashMap<&'static str, Vec<&'static str>> = {
        let mut m = FxHashMap::default();
        m.insert("clipboard", vec!["onCopy", "onCut", "onPaste"]);
        m.insert("composition", vec!["onCompositionEnd", "onCompositionStart", "onCompositionUpdate"]);
        m.insert("keyboard", vec!["onKeyDown", "onKeyPress", "onKeyUp"]);
        m.insert("focus", vec!["onFocus", "onBlur"]);
        m.insert("form", vec!["onChange", "onInput", "onSubmit"]);
        m.insert("mouse", vec![
            "onClick", "onContextMenu", "onDblClick", "onDoubleClick", "onDrag", "onDragEnd",
            "onDragEnter", "onDragExit", "onDragLeave", "onDragOver", "onDragStart", "onDrop",
            "onMouseDown", "onMouseEnter", "onMouseLeave", "onMouseMove", "onMouseOut",
            "onMouseOver", "onMouseUp"
        ]);
        m.insert("selection", vec!["onSelect"]);
        m.insert("touch", vec!["onTouchCancel", "onTouchEnd", "onTouchMove", "onTouchStart"]);
        m.insert("ui", vec!["onScroll"]);
        m.insert("wheel", vec!["onWheel"]);
        m.insert("media", vec![
            "onAbort", "onCanPlay", "onCanPlayThrough", "onDurationChange", "onEmptied",
            "onEncrypted", "onEnded", "onError", "onLoadedData", "onLoadedMetadata", "onLoadStart",
            "onPause", "onPlay", "onPlaying", "onProgress", "onRateChange", "onSeeked", "onSeeking",
            "onStalled", "onSuspend", "onTimeUpdate", "onVolumeChange", "onWaiting"
        ]);
        m.insert("image", vec!["onLoad", "onError"]);
        m.insert("animation", vec!["onAnimationStart", "onAnimationEnd", "onAnimationIteration"]);
        m.insert("transition", vec!["onTransitionEnd"]);
        m
    };
}

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let attributes = extract_attrs(&node.attributes());
        let element_name = element_name.text_trimmed();

        if let Some(attributes_ref) = attributes.as_ref() {
            if !is_interactive_handler_present(attributes_ref) {
                return None;
            }
        } else {
            return None;
        }

        let default_attributes = FxHashMap::default();
        let attributes_ref = attributes.as_ref().unwrap_or(&default_attributes);

        let attributes_option = if !attributes_ref.is_empty() {
            Some(attributes_ref.clone())
        } else {
            None
        };

        if node
            .find_attribute_by_name("aria-hidden")
            .map_or(false, |attr| {
                attr.as_static_value()
                    .map_or(true, |val| val.text() == "true")
            })
        {
            return None;
        }

        let is_valid_element = match element_name {
            "section" => ["aria-label", "aria-labelledby"].iter().any(|&attr_name| {
                node.find_attribute_by_name(attr_name)
                    .map_or(false, |attr| {
                        attr.as_static_value()
                            .map_or(false, |val| !val.text().is_empty())
                    })
            }),
            "a" => node.find_attribute_by_name("href").map_or(false, |attr| {
                attr.as_static_value()
                    .map_or(false, |val| !val.text().is_empty())
            }),
            _ => {
                (!aria_roles.is_not_interactive_element(element_name, attributes_option.clone())
                    && !is_invalid_element(element_name))
                    || is_valid_element(element_name)
            }
        };

        if is_valid_element {
            return None;
        }

        if attributes_option.is_some() {
            match node.find_attribute_by_name("role") {
                Some(attr) => {
                    let role_value = attr.as_static_value()?;
                    let role_text = role_value.text();

                    if aria_roles.is_role_interactive(role_text) {
                        return None;
                    }
                }
                None => {
                    return Some(());
                }
            };
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {{"Avoid non-native interactive elements. If using native HTML is not possible, add an appropriate role and support for tabbing, mouse, keyboard, and touch inputs to an interactive content element"}},
        ).note(
            markup! {{"If using native HTML is not possible, add an appropriate role and support for tabbing, mouse, keyboard, and touch inputs to an interactive content element"}}
        ))
    }
}

/// Check if any interactive handler is present in the attributes.
fn is_interactive_handler_present(attributes: &FxHashMap<String, Vec<String>>) -> bool {
    let categories_to_check = vec!["focus", "keyboard", "mouse"];
    for category in categories_to_check {
        if let Some(handlers) = EVENT_TO_HANDLERS.get(category) {
            for handler in handlers {
                if let Some(values) = attributes.get(*handler) {
                    if values.iter().any(|value| value != "null") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// This method is an override of AriaService::extract_attributes.
///
/// This modified version can extract attributes with dynamic values as well.
/// The original extract_attributes in AriaService skips extracting other static attributes if a dynamic value is encountered.
pub fn extract_attrs(attribute_list: &JsxAttributeList) -> Option<FxHashMap<String, Vec<String>>> {
    let mut defined_attributes: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for attribute in attribute_list {
        if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
            let name = attr.name().ok()?.syntax().text_trimmed().to_string();
            let values = if let Some(initializer) = attr.initializer() {
                let initializer = initializer.value().ok()?;
                if let Some(static_value) = initializer.as_static_value() {
                    static_value
                        .text()
                        .split(' ')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                } else {
                    vec![initializer.syntax().text().to_string()]
                }
            } else {
                vec!["true".to_string()]
            };

            defined_attributes.entry(name).or_insert(values);
        }
    }
    Some(defined_attributes)
}

/// This function disables interactive elements.
/// This is because this is an element that is disabled by eslint-plugin-jsx-a11y.
fn is_invalid_element(element_name: &str) -> bool {
    match element_name {
        // These cases are interactive with the is_not_interactive_element method,
        // but is an invalid test case element for eslint-plugin-jsx-a11y.
        "link" | "header" => true,
        "area" | "b" | "bdi" | "bdo" | "hgroup" | "i" | "u" | "q" | "small" | "data" | "samp"
        | "acronym" | "applet" | "base" | "big" | "blink" | "center" | "cite" | "col"
        | "colgroup" | "content" | "font" | "frameset" | "head" | "kbd" | "keygen" | "map"
        | "meta" | "noembed" | "noscript" | "object" | "param" | "picture" | "rp" | "rt"
        | "rtc" | "s" | "script" | "source" | "spacer" | "strike" | "style" | "summary"
        | "title" | "track" | "tt" | "var" | "wbr" | "xmp" => true,
        _ => false,
    }
}

/// This function ables non-interactive elements.
/// This is because this is an element that is abled by eslint-plugin-jsx-a11y.
fn is_valid_element(element_name: &str) -> bool {
    matches!(
        element_name,
        "input"
            | "form"
            | "iframe"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "ruby"
            | "pre"
            | "mark"
            | "aside"
            | "blockquote"
            | "address"
            | "article"
            | "caption"
            | "output"
            | "p"
            | "li"
            | "ol"
            | "ul"
            | "nav"
            | "table"
            | "thead"
            | "tbody"
            | "tfoot"
            | "time"
            | "dfn"
            | "main"
            | "br"
            | "details"
            | "dd"
            | "dir"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "img"
            | "label"
            | "legend"
            | "marquee"
            | "menu"
            | "meter"
            | "optgroup"
            | "progress"
            | "th"
            | "td"
    )
}
