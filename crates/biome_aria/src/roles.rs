use biome_aria_metadata::AriaRole;
use std::fmt::Debug;

use crate::{Attribute, Element};

/// Convenient type to retrieve metadata regarding ARIA roles
#[derive(Debug, Default)]
pub struct AriaRoles;

impl AriaRoles {
    pub fn get_implicit_role(&self, elt: &impl Element) -> Option<AriaRole> {
        // See https://www.w3.org/TR/html-aria/
        Some(match elt.name()?.as_ref() {
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
                match elt
                    .find_attribute_by_name(|n| n == "type")
                    .as_ref()
                    .and_then(|a| a.value())
                    .as_ref()
                    .map(|v| v.as_ref())
                {
                    Some("checkbox") => AriaRole::Menuitemcheckbox,
                    Some("radio") => AriaRole::Menuitemradio,
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
                match elt
                    .find_attribute_by_name(|n| n == "scope")
                    .as_ref()
                    .and_then(|a| a.value())
                    .as_ref()
                    .map(|v| v.as_ref())
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
                match elt
                    .find_attribute_by_name(|n| n == "type")
                    .as_ref()
                    .and_then(|a| a.value())
                    .as_ref()
                    .map(|v| v.as_ref())
                {
                    Some("checkbox") => AriaRole::Checkbox,
                    Some("number") => AriaRole::Spinbutton,
                    Some("radio") => AriaRole::Radio,
                    Some("range") => AriaRole::Slider,
                    Some("button" | "image" | "reset" | "submit") => AriaRole::Button,
                    Some("search") => match elt.find_attribute_by_name(|n| n == "list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Searchbox,
                    },
                    Some(
                        "color" | "date" | "datetime-local" | "file" | "hidden" | "month"
                        | "password" | "time" | "week",
                    ) => {
                        return None;
                    }
                    _ => match elt.find_attribute_by_name(|n| n == "list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Textbox,
                    },
                }
            }
            // FIXME: `link` has no corresponding roles in https://www.w3.org/TR/html-aria/
            // Should we remove it?
            "a" | "area" | "link" => match elt.find_attribute_by_name(|n| n == "href") {
                Some(_) => AriaRole::Link,
                _ => AriaRole::Generic,
            },
            "img" => match elt
                .find_attribute_by_name(|n| n == "alt")
                .as_ref()
                .and_then(|a| a.value())
                .as_ref()
                .map(|v| v.as_ref())
            {
                Some(value) => {
                    if !value.trim_ascii().is_empty() {
                        AriaRole::Img
                    } else {
                        let has_accessible_name = elt
                            .find_attribute_by_name(|n| {
                                matches!(n, "aria-labelledby" | "aria-label" | "title")
                            })
                            .is_some();
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
                let has_accessible_name = elt
                    .find_attribute_by_name(|n| {
                        matches!(n, "aria-labelledby" | "aria-label" | "title")
                    })
                    .is_some();
                if has_accessible_name {
                    AriaRole::Region
                } else {
                    AriaRole::Generic
                }
            }
            "select" => {
                let size = match elt
                    .find_attribute_by_name(|n| n == "size")
                    .as_ref()
                    .and_then(|a| a.value())
                {
                    Some(size) => size.as_ref().parse::<i32>().ok()?,
                    None => 0,
                };
                if elt.find_attribute_by_name(|n| n == "multiple").is_none() && size <= 1 {
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

    /// Given the element, the function tells whether it's interactive
    pub fn is_not_interactive_element(&self, element: &impl Element) -> bool {
        match element.name().as_ref().map(|name| name.as_ref()) {
            // <header> elements do not technically have semantics, unless the
            // element is a direct descendant of <body>, and this crate cannot
            // reliably test that.
            //
            // Check: https://www.w3.org/TR/wai-aria-practices/examples/landmarks/banner.html
            Some("header") => false,
            // FIXME: this differs from `get_implicit_role`, is it intentional?
            //
            // Always consider `a` and `area` as interactive even if `href` is not set.
            Some("a" | "area") => false,
            // SVG elements, by default, do not have interactive semantics.
            // They are primarily used for graphics and visual rendering. While they can be made interactive with additional
            // attributes and JavaScript, inherently they don't provide user interaction capabilities.
            // Hence, we classify them as non-interactive elements similar to other non-interactive
            // elements like <img> or <progress>.
            //
            // Check: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role#svg_and_roleimg
            Some("svg") => true,
            // Elements without any concept
            Some(
                "body" | "br" | "details" | "dir" | "frame" | "iframe" | "label" | "mark"
                | "marquee" | "menu" | "meter" | "optgroup" | "pre" | "progress" | "ruby",
            ) => true,
            // `<input type="hidden">` is not interactive.
            // `type=hidden` is not represented as concept information.
            //
            // Other `<input>` are considered interactive.
            Some("input") => element
                .find_attribute_by_name(|n| n == "type")
                .as_ref()
                .and_then(|attr| attr.value())
                .is_some_and(|value| value.as_ref() == "hidden"),
            _ => self
                .get_implicit_role(element)
                .is_some_and(|implicit_role| implicit_role.is_non_interactive()),
        }
    }

    /// Given an element name and attributes, it returns the role associated with that element.
    /// If no explicit role attribute is present, an implicit role is returned.
    pub fn get_role_by_element_name(&self, element: &impl Element) -> Option<AriaRole> {
        element
            .find_attribute_by_name(|name| name == "role")
            .as_ref()
            .and_then(|role| AriaRole::from_roles(role.value()?.as_ref()))
            .or_else(|| self.get_implicit_role(element))
    }

    pub fn is_not_static_element(&self, element: &impl Element) -> bool {
        match element.name().as_ref().map(|name| name.as_ref()) {
            // embedded content
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#embedded-content
            Some("canvas" | "embed" | "iframe" | "video" | "audio") => true,
            // No corresponding role
            Some("input" | "dl" | "label" | "legend" | "ruby" | "pre" | "figcaption" | "br") => {
                true
            }
            Some("s" | "hgroup") => false,
            // FIXME: should we add `link`?
            //
            // metadata content, except `link`
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#document-metadata
            Some("meta" | "base" | "title" | "basefont" | "head") => false,
            // scripting content
            // ref: https://html.spec.whatwg.org/multipage/semantics.html#scripting-content
            Some("script" | "noscript" | "template" | "style") => false,
            _ => match self.get_role_by_element_name(element) {
                None | Some(AriaRole::Presentation | AriaRole::Generic) => false,
                Some(_) => true,
            },
        }
    }
}
