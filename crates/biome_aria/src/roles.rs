use biome_aria_metadata::AriaRole;
use rustc_hash::FxHashMap;
use std::fmt::Debug;

/// Convenient type to retrieve metadata regarding ARIA roles
#[derive(Debug, Default)]
pub struct AriaRoles;

impl AriaRoles {
    /// Given a element and attributes, it returns the metadata of the element's implicit role.
    ///
    /// Check: https://www.w3.org/TR/html-aria/#docconformance
    pub fn get_implicit_role(
        &self,
        element: &str,
        // To generate `attributes`, you can use `biome_js_analyze::services::aria::AriaServices::extract_defined_attributes`
        attributes: &FxHashMap<String, Vec<String>>,
    ) -> Option<AriaRole> {
        // See https://www.w3.org/TR/html-aria/
        Some(match element {
            "article" => AriaRole::Article,
            "aside" => AriaRole::Complementary,
            "blockquote" => AriaRole::Blockquote,
            "button" => AriaRole::Button,
            // FIXME: `figcaption` has no corresponding roles in https://www.w3.org/TR/html-aria/
            // Should we remove it?
            "caption" | "figcaption" | "legend" => AriaRole::Caption,
            "code" => AriaRole::Code,
            "datalist" => AriaRole::Listbox,
            "del" | "s" => AriaRole::Deletion,
            // FIXME: `dd` has no corresponding roles in https://www.w3.org/TR/html-aria/
            // Should we remove it?
            "dd" => AriaRole::Definition,
            // FIXME: `dt` has no corresponding roles in https://www.w3.org/TR/html-aria/
            // Should we remove it?
            "dt" => AriaRole::Term,
            "dfn" => AriaRole::Term,
            "mark" => AriaRole::Mark,
            "dialog" => AriaRole::Dialog,
            "em" => AriaRole::Emphasis,
            "figure" => AriaRole::Figure,
            "form" => AriaRole::Form,
            "hr" => AriaRole::Separator,
            "html" => AriaRole::Document,
            "ins" => AriaRole::Insertion,
            "main" => AriaRole::Main,
            "marquee" => AriaRole::Marquee,
            "math" => AriaRole::Math,
            "menu" => AriaRole::List,
            "menuitem" => {
                let type_values = attributes.get("type")?;
                match type_values.first()?.as_str() {
                    "checkbox" => AriaRole::Menuitemcheckbox,
                    "radio" => AriaRole::Menuitemradio,
                    _ => AriaRole::Menuitem,
                }
            }
            "meter" => AriaRole::Meter,
            "nav" => AriaRole::Navigation,
            "ul" | "ol" => AriaRole::List,
            "li" => AriaRole::Listitem,
            "option" => AriaRole::Option,
            "hgroup" | "optgroup" => AriaRole::Group,
            "output" => AriaRole::Status,
            "p" => AriaRole::Paragraph,
            "progress" => AriaRole::Progressbar,
            "search" => AriaRole::Search,
            "strong" => AriaRole::Strong,
            "sub" => AriaRole::Subscript,
            "sup" => AriaRole::Superscript,
            "svg" => AriaRole::GraphicsDocument,
            "table" => AriaRole::Table,
            "textarea" => AriaRole::Textbox,
            "tr" => AriaRole::Row,
            // cell if a descendant of a <table> element,
            // but this crate does not support checking a descendant of an element.
            //
            // ref: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td
            "td" => AriaRole::Cell,
            // <th> element is able to be a rowheader, columnheader,
            // but this crate does not support checking a descendant of an element.
            //
            // ref: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th
            "th" => {
                match attributes
                    .get("scope")
                    .and_then(|xs| xs.first())
                    .map(|x| x.as_ref())
                {
                    Some("col") => AriaRole::Columnheader,
                    Some("row") => AriaRole::Rowheader,
                    _ => AriaRole::Rowheader,
                }
            }
            "time" => AriaRole::Time,
            "address" | "details" | "fieldset" => AriaRole::Group,
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => AriaRole::Heading,
            "tbody" | "tfoot" | "thead" => AriaRole::Rowgroup,
            "input" => {
                match attributes
                    .get("type")
                    .and_then(|xs| xs.first())
                    .map(|x| x.as_ref())
                {
                    Some("checkbox") => AriaRole::Checkbox,
                    Some("number") => AriaRole::Spinbutton,
                    Some("radio") => AriaRole::Radio,
                    Some("range") => AriaRole::Slider,
                    Some("button" | "image" | "reset" | "submit") => AriaRole::Button,
                    Some("search") => match attributes.get("list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Searchbox,
                    },
                    Some(
                        "color" | "date" | "datetime-local" | "file" | "hidden" | "month"
                        | "password" | "time" | "week",
                    ) => {
                        return None;
                    }
                    _ => match attributes.get("list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Textbox,
                    },
                }
            }
            // FIXME: `link` has no corresponding roles in https://www.w3.org/TR/html-aria/
            // Should we remove it?
            "a" | "area" | "link" => match attributes.get("href") {
                Some(_) => AriaRole::Link,
                _ => AriaRole::Generic,
            },
            "img" => match attributes.get("alt") {
                Some(values) => {
                    if values.iter().any(|x| !x.is_empty()) {
                        AriaRole::Img
                    } else {
                        let has_accessible_name = attributes.get("aria-labelledby").is_some()
                            || attributes.get("aria-label").is_some()
                            || attributes.get("title").is_some();
                        if has_accessible_name {
                            AriaRole::Img
                        } else {
                            AriaRole::Presentation
                        }
                    }
                }
                None => AriaRole::Img,
            },
            "section" => {
                let has_accessible_name = attributes.get("aria-labelledby").is_some()
                    || attributes.get("aria-label").is_some()
                    || attributes.get("title").is_some();
                if has_accessible_name {
                    AriaRole::Region
                } else {
                    AriaRole::Generic
                }
            }
            "select" => {
                let size = match attributes.get("size") {
                    Some(size) => size
                        .first()
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>()
                        .ok()?,
                    None => 0,
                };
                let multiple = attributes.get("multiple");
                if multiple.is_none() && size <= 1 {
                    AriaRole::Combobox
                } else {
                    AriaRole::Listbox
                }
            }
            "b" | "bdi" | "bdo" | "body" | "data" | "div" | "i" | "q" | "samp" | "small"
            | "span" | "u" | "pre" => AriaRole::Generic,
            "header" | "footer" => {
                // This crate does not support checking a descendant of an element.
                // header (maybe BannerRole): https://www.w3.org/WAI/ARIA/apg/patterns/landmarks/examples/banner.html
                // footer (maybe ContentInfoRole): https://www.w3.org/WAI/ARIA/apg/patterns/landmarks/examples/contentinfo.html
                AriaRole::Generic
            }
            _ => return None,
        })
    }

    /// Given the name of element, the function tells whether it's interactive
    pub fn is_not_interactive_element(
        &self,
        element_name: &str,
        attributes: Option<FxHashMap<String, Vec<String>>>,
    ) -> bool {
        // <header> elements do not technically have semantics, unless the
        // element is a direct descendant of <body>, and this crate cannot
        // reliably test that.
        //
        // Check: https://www.w3.org/TR/wai-aria-practices/examples/landmarks/banner.html
        if element_name == "header" {
            return false;
        }

        // FIXME: this differs from `get_implicit_role`, is it intentional?
        //
        // Always consider `a` and `area` as interactive even if `href` is not set.
        if matches!(element_name, "a" | "area") {
            return false;
        }

        // SVG elements, by default, do not have interactive semantics.
        // They are primarily used for graphics and visual rendering. While they can be made interactive with additional
        // attributes and JavaScript, inherently they don't provide user interaction capabilities.
        // Hence, we classify them as non-interactive elements similar to other non-interactive
        // elements like <img> or <progress>.
        //
        // Check: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role#svg_and_roleimg
        if element_name == "svg" {
            return true;
        }

        let elements_no_concept_info = [
            "body", "br", "details", "dir", "frame", "iframe", "label", "mark", "marquee", "menu",
            "meter", "optgroup", "pre", "progress", "ruby",
        ];
        if elements_no_concept_info.contains(&element_name) {
            return true;
        }

        // `<input type="hidden">` is not interactive.
        // `type=hidden` is not represented as concept information.
        //
        // Other `<input>` are considered interactive.
        if element_name == "input" {
            return attributes
                .as_ref()
                .and_then(|attributes| attributes.get("type"))
                .map_or(false, |values| values.iter().any(|x| x == "hidden"));
        }

        if let Some(implicit_role) = self.get_implicit_role(
            element_name,
            attributes.as_ref().unwrap_or(&FxHashMap::default()),
        ) {
            if implicit_role.is_non_interactive() {
                return true;
            }
        }

        false
    }

    /// Given an element name and attributes, it returns the role associated with that element.
    /// If no explicit role attribute is present, an implicit role is returned.
    pub fn get_role_by_element_name(
        &self,
        element_name: &str,
        attributes: &FxHashMap<String, Vec<String>>,
    ) -> Option<AriaRole> {
        attributes
            .get("role")
            .and_then(|role| role.first())
            .map_or_else(
                || self.get_implicit_role(element_name, attributes),
                |r| AriaRole::from_roles(r),
            )
    }

    pub fn is_not_static_element(
        &self,
        element_name: &str,
        attributes: &FxHashMap<String, Vec<String>>,
    ) -> bool {
        if match element_name {
            // embedded content
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#embedded-content
            "canvas" | "embed" | "iframe" | "video" | "audio" => true,
            // metadata content
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#document-metadata
            "meta" | "link" | "base" | "title" | "basefont" | "head" => false,
            // scripting content
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#scripting-content
            "script" | "noscript" | "template" | "style" => false,
            // No corresponding role
            "input" | "dl" | "label" | "legend" | "ruby" | "pre" | "figcaption" | "br" => true,
            _ => false,
        } {
            return true;
        }

        if matches!(element_name, "s" | "hgroup") {
            return false;
        }

        let role = self.get_role_by_element_name(element_name, attributes);

        match role {
            None | Some(AriaRole::Presentation | AriaRole::Generic) => false,
            Some(_) => true,
        }
    }
}

#[cfg(test)]
mod test {
    use rustc_hash::FxHashMap;

    use crate::AriaRoles;
    use biome_aria_metadata::AriaRole;

    #[test]
    fn should_be_interactive() {
        let aria_roles = AriaRoles {};
        assert!(!aria_roles.is_not_interactive_element("header", None));
        assert!(!aria_roles.is_not_interactive_element("input", {
            let mut attributes = FxHashMap::default();
            attributes.insert("type".to_string(), vec!["search".to_string()]);
            Some(attributes)
        }));
    }

    #[test]
    fn should_not_be_interactive() {
        let aria_roles = AriaRoles {};
        assert!(aria_roles.is_not_interactive_element("h1", None));
        assert!(aria_roles.is_not_interactive_element("h2", None));
        assert!(aria_roles.is_not_interactive_element("h3", None));
        assert!(aria_roles.is_not_interactive_element("h4", None));
        assert!(aria_roles.is_not_interactive_element("h5", None));
        assert!(aria_roles.is_not_interactive_element("h6", None));
        assert!(aria_roles.is_not_interactive_element("body", None));
        assert!(aria_roles.is_not_interactive_element("input", {
            let mut attributes = FxHashMap::default();
            attributes.insert("type".to_string(), vec!["hidden".to_string()]);
            Some(attributes)
        }));
    }

    #[test]
    fn test_get_implicit_role() {
        let aria_roles = AriaRoles {};

        // No attributes
        let implicit_role = aria_roles
            .get_implicit_role("button", &FxHashMap::default())
            .unwrap();
        assert_eq!(implicit_role, AriaRole::Button);

        // <input type="search">
        let mut attributes = FxHashMap::default();
        attributes.insert("type".to_string(), vec!["search".to_string()]);
        let implicit_role = aria_roles.get_implicit_role("input", &attributes).unwrap();
        assert_eq!(implicit_role, AriaRole::Searchbox);

        // <select name="animals" multiple size="4">
        let mut attributes = FxHashMap::default();
        attributes.insert("name".to_string(), vec!["animals".to_string()]);
        attributes.insert("multiple".to_string(), vec![String::new()]);
        attributes.insert("size".to_string(), vec!["4".to_string()]);
        let implicit_role = aria_roles.get_implicit_role("select", &attributes).unwrap();
        assert_eq!(implicit_role, AriaRole::Listbox);
    }
}
