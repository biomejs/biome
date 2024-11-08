use crate::define_role;
use biome_aria_metadata::AriaRole;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

pub trait AriaRoleDefinition: Debug {
    /// It returns an iterator over the properties of the current role
    fn properties(&self) -> Iter<(&str, bool)>;

    /// It returns an iterator over the possible roles of this definition
    fn roles(&self) -> Iter<&str>;

    /// Whether the current role is interactive
    fn is_interactive(&self) -> bool {
        self.roles().any(|role| *role == "widget")
    }
}

#[derive(Debug)]
/// https://www.w3.org/TR/wai-aria-1.1/#switch
struct ButtonRole;

impl ButtonRole {
    const PROPS: &'static [(&'static str, bool)] =
        &[("aria-expanded", false), ("aria-expanded", false)];
    const ROLES: &'static [&'static str] = &["roletype", "widget", "command"];
    const CONCEPTS: &'static [(&'static str, &'static [(&'static str, &'static str)])] =
        &[("button", &[]), ("input", &[("type", "button")])];
}

impl AriaRoleDefinition for ButtonRole {
    fn properties(&self) -> Iter<(&str, bool)> {
        Self::PROPS.iter()
    }

    fn roles(&self) -> Iter<&str> {
        Self::ROLES.iter()
    }
}

impl AriaRoleDefinitionWithConcepts for ButtonRole {
    fn concepts_by_role<'a>(&self) -> ElementsAndAttributes<'a> {
        Some(Self::CONCEPTS.iter())
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    CheckboxRole {
        PROPS: [("aria-checked", true), ("aria-readonly", false)],
        ROLES: ["switch", "menuitemcheckbox", "widget"],
        CONCEPTS: &[("input", &[("type", "checkbox")])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radio
    RadioRole {
        PROPS: [("aria-checked", true), ("aria-readonly", false)],
        ROLES: ["menuitemradio", "widget"],
        CONCEPTS: &[("input", &[("type", "radio")])],
    }
}

#[derive(Debug)]
/// https://www.w3.org/TR/wai-aria-1.1/#option
struct OptionRole;

impl OptionRole {
    const PROPS: &'static [(&'static str, bool)] = &[("aria-selected", true)];
    const ROLES: &'static [&'static str] = &["treeitem", "widget"];
    const CONCEPTS: &'static [(&'static str, &'static [(&'static str, &'static str)])] =
        &[("option", &[])];
}

impl AriaRoleDefinition for OptionRole {
    fn properties(&self) -> Iter<(&str, bool)> {
        Self::PROPS.iter()
    }

    fn roles(&self) -> Iter<&str> {
        Self::ROLES.iter()
    }
}

impl AriaRoleDefinitionWithConcepts for OptionRole {
    fn concepts_by_role<'a>(&self) -> ElementsAndAttributes<'a> {
        Some(Self::CONCEPTS.iter())
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#combobox
    ComboBoxRole {
        PROPS: [("aria-controls", true), ("aria-expanded", true)],
        ROLES: ["select", "widget"],
        CONCEPTS: &[("select", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#heading
    HeadingRole {
        PROPS:  [("aria-level", true)],
        ROLES:  ["sectionhead"],
        CONCEPTS: &[("h1", &[]), ("h2", &[]), ("h3", &[]), ("h4", &[]), ("h5", &[]), ("h6", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#separator
    SeparatorRole {
        PROPS:  [
            ("aria-valuemax", false),
            ("aria-valuemin", false),
            ("aria-valuenow", false),
        ],
        ROLES: ["structure", "widget"],
        CONCEPTS: &[("hr", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#article
    ArticleRole {
        PROPS: [],
        ROLES: ["document"],
        CONCEPTS: &[("article", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#dialog
    DialogRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["window"],
        CONCEPTS: &[("dialog", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#cell
    CellRole {
        PROPS: [
            ("aria-colindex", false),
            ("aria-colspan", false),
            ("aria-rowindex", false),
            ("aria-rowspan", false),
        ],
        ROLES: ["section"],
        CONCEPTS: &[("td", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#columnheader
    ColumnHeaderRole {
        PROPS: [("aria-sort", false)],
        ROLES: ["cell", "gridcell", "sectionhead"],
        CONCEPTS: &[("th", &[("scope", "col")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#definition
    DefinitionRole {
        PROPS: [("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("dd", &[]), ("dfn", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#figure
    FigureRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("figure", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#form
    FormRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("form", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#grid
    GridRole {
        PROPS: [("aria-level", false), ("aria-multiselectable", false), ("aria-readonly", false)],
        ROLES: ["composite", "table"],
        CONCEPTS: &[("table", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#gridcell
    GridCellRole {
        PROPS: [("aria-readonly", false), ("aria-required", false), ("aria-selected", false)],
        ROLES: ["cell", "widget"],
        CONCEPTS: &[("td", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#group
    GroupRole {
        PROPS: [("aria-activedescendant", false)],
        ROLES: ["row", "select", "toolbar"],
        CONCEPTS: &[("fieldset", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#img
    ImgRole {
        PROPS: [("aria-activedescendant", false)],
        ROLES: ["section"],
        CONCEPTS: &[("img", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#link
    LinkRole {
        PROPS: [("aria-expanded", false)],
        ROLES: ["command", "widget"],
        CONCEPTS: &[("a", &[]), ("link", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#list
    ListRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("ol", &[]), ("ul", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listbox
    ListBoxRole {
        PROPS: [],
        ROLES: ["select", "widget"],
        CONCEPTS: &[("select", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listitem
    ListItemRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("li", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#main
    MainRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("main", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#navigation
    NavigationRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("nav", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#row
    RowRole {
        PROPS: [("aria-colindex", false), ("aria-level", false), ("aria-rowindex", false), ("aria-selected", false)],
        ROLES: ["group", "widget"],
        CONCEPTS: &[("tr", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowgroup
    RowGroupRole {
        PROPS: [],
        ROLES: ["structure"],
        CONCEPTS: &[("tbody", &[]), ("tfoot", &[]), ("thead", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowheader
    RowHeaderRole {
        PROPS: [("aria-sort", false)],
        ROLES: ["cell", "gridcell", "sectionhead"],
        CONCEPTS: &[("th", &[("scope", "row")])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#search
    SearchRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("search", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#searchbox
    SearchboxRole {
        PROPS: [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ],
        ROLES: ["textbox", "widget"],
        CONCEPTS: &[("input", &[("type", "search")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#table
    TableRole {
        PROPS: [("aria-colcount", false), ("aria-rowcount", false)],
        ROLES: ["section"],
        CONCEPTS: &[("table", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#term
    TermRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("dt", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#textbox
    TextboxRole {
        PROPS: [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ],
        ROLES: ["input", "widget"],
        CONCEPTS: &[("textarea", &[]), ("input", &[("type", "search")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#generic
    GenericRole {
        PROPS: [],
        ROLES: ["structure"],
        CONCEPTS: &[("div", &[]), ("span", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#complementary
    ComplementaryRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("aside", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#blockquote
    BlockQuoteRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("blockquote", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#caption
    CaptionRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("caption", &[]), ("figcaption", &[]), ("legend", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/graphics-aria/#graphics-document
    GraphicsDocumentRole {
        PROPS: [],
        ROLES: ["document"],
        CONCEPTS: &[("graphics-object", &[]), ("img", &[]), ("article", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/graphics-aria/#graphics-object
    GraphicsObjectRole {
        PROPS: [],
        ROLES: ["group"],
        CONCEPTS: &[("graphics-document", &[]), ("group", &[]), ("img", &[]), ("graphics-symbol", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#time
    TimeRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("time", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#paragraph
    ParagraphRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("p", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#status
    StatusRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("output", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#region
    RegionRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("section", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#associationlist
    AssociationListRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("dl", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#contentinfo
    ContentInfoRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("footer", &[])],
    }
}

impl<'a> AriaRoles {
    /// These are roles that will contain "concepts".
    pub(crate) const ROLE_WITH_CONCEPTS: &'a [&'a str] = &[
        "checkbox",
        "radio",
        "option",
        "combobox",
        "heading",
        "separator",
        "button",
        "article",
        "dialog",
        "cell",
        "columnheader",
        "definition",
        "figure",
        "form",
        "grid",
        "gridcell",
        "group",
        "img",
        "link",
        "list",
        "listbox",
        "listitem",
        "navigation",
        "row",
        "rowgroup",
        "rowheader",
        "search",
        "searchbox",
        "table",
        "term",
        "textbox",
        "generic",
        "caption",
        "main",
        "time",
        "p",
        "aside",
        "blockquote",
        "associationlist",
        "status",
        "contentinfo",
        "region",
        "graphics-document",
        "graphics-object",
        "graphics-symbol",
    ];

    /// Given a element and attributes, it returns the metadata of the element's implicit role.
    ///
    /// Check: https://www.w3.org/TR/html-aria/#docconformance
    pub fn get_implicit_role(
        &self,
        element: &str,
        // To generate `attributes`, you can use `biome_js_analyze::services::aria::AriaServices::extract_defined_attributes`
        attributes: &FxHashMap<String, Vec<String>>,
    ) -> Option<AriaRole> {
        Some(match element {
            "article" => AriaRole::Article,
            "aside" => AriaRole::Complementary,
            "blockquote" => AriaRole::Blockquote,
            "button" => AriaRole::Button,
            "caption" => AriaRole::Caption,
            "code" => AriaRole::Code,
            "datalist" => AriaRole::Listbox,
            "del" => AriaRole::Deletion,
            "dd" => AriaRole::Definition,
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
            "optgroup" => AriaRole::Group,
            "output" => AriaRole::Status,
            "p" => AriaRole::Paragraph,
            "progress" => AriaRole::Progressbar,
            "search" => AriaRole::Search,
            "strong" => AriaRole::Strong,
            "sub" => AriaRole::Subscript,
            "sup" => AriaRole::Superscript,
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
            "th" => AriaRole::Rowheader,
            "time" => AriaRole::Time,
            "address" | "details" | "fieldset" => AriaRole::Group,
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => AriaRole::Heading,
            "tbody" | "tfoot" | "thead" => AriaRole::Rowgroup,
            "input" => {
                let type_values = attributes.get("type")?;
                match type_values.first()?.as_str() {
                    "checkbox" => AriaRole::Checkbox,
                    "number" => AriaRole::Spinbutton,
                    "radio" => AriaRole::Radio,
                    "range" => AriaRole::Slider,
                    "button" | "image" | "reset" | "submit" => AriaRole::Button,
                    "search" => match attributes.get("list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Searchbox,
                    },
                    "email" | "tel" | "url" => match attributes.get("list") {
                        Some(_) => AriaRole::Combobox,
                        _ => AriaRole::Textbox,
                    },
                    "text" => AriaRole::Textbox,
                    _ => AriaRole::Textbox,
                }
            }
            "a" | "area" | "link" => match attributes.get("href") {
                Some(_) => AriaRole::Link,
                _ => AriaRole::Generic,
            },
            "img" => match attributes.get("alt") {
                Some(values) => {
                    if values.iter().any(|x| !x.is_empty()) {
                        AriaRole::Img
                    } else {
                        AriaRole::Presentation
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
                    return None;
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
            "b" | "bdi" | "bdo" | "body" | "data" | "div" | "hgroup" | "i" | "q" | "samp"
            | "small" | "span" | "u" | "pre" => AriaRole::Generic,
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

        // <input type="hidden"> is not interactive.
        // `type=hidden` is not represented as concept information.
        if element_name == "input"
            && attributes
                .as_ref()
                .and_then(|attributes| attributes.get("type"))
                .map_or(false, |values| values.iter().any(|x| x == "hidden"))
        {
            return true;
        }

        for element in Self::ROLE_WITH_CONCEPTS {
            let role = match *element {
                "checkbox" => &CheckboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "radio" => &RadioRole as &dyn AriaRoleDefinitionWithConcepts,
                "option" => &OptionRole as &dyn AriaRoleDefinitionWithConcepts,
                "combobox" => &ComboBoxRole as &dyn AriaRoleDefinitionWithConcepts,
                "heading" => &HeadingRole as &dyn AriaRoleDefinitionWithConcepts,
                "separator" => &SeparatorRole as &dyn AriaRoleDefinitionWithConcepts,
                "button" => &ButtonRole as &dyn AriaRoleDefinitionWithConcepts,
                "article" => &ArticleRole as &dyn AriaRoleDefinitionWithConcepts,
                "dialog" => &DialogRole as &dyn AriaRoleDefinitionWithConcepts,
                "cell" => &CellRole as &dyn AriaRoleDefinitionWithConcepts,
                "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
                "definition" => &DefinitionRole as &dyn AriaRoleDefinitionWithConcepts,
                "figure" => &FigureRole as &dyn AriaRoleDefinitionWithConcepts,
                "form" => &FormRole as &dyn AriaRoleDefinitionWithConcepts,
                "graphics-document" => &GraphicsDocumentRole as &dyn AriaRoleDefinitionWithConcepts,
                "graphics-object" => &GraphicsObjectRole as &dyn AriaRoleDefinitionWithConcepts,
                "grid" => &GridRole as &dyn AriaRoleDefinitionWithConcepts,
                "gridcell" => &GridCellRole as &dyn AriaRoleDefinitionWithConcepts,
                "group" => &GroupRole as &dyn AriaRoleDefinitionWithConcepts,
                "img" => &ImgRole as &dyn AriaRoleDefinitionWithConcepts,
                "link" => &LinkRole as &dyn AriaRoleDefinitionWithConcepts,
                "list" => &ListRole as &dyn AriaRoleDefinitionWithConcepts,
                "listbox" => &ListBoxRole as &dyn AriaRoleDefinitionWithConcepts,
                "listitem" => &ListItemRole as &dyn AriaRoleDefinitionWithConcepts,
                "navigation" => &NavigationRole as &dyn AriaRoleDefinitionWithConcepts,
                "row" => &RowRole as &dyn AriaRoleDefinitionWithConcepts,
                "rowgroup" => &RowGroupRole as &dyn AriaRoleDefinitionWithConcepts,
                "rowheader" => &RowHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
                "search" => &SearchboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "searchbox" => &SearchboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "table" => &TableRole as &dyn AriaRoleDefinitionWithConcepts,
                "term" => &TermRole as &dyn AriaRoleDefinitionWithConcepts,
                "textbox" => &TextboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "generic" => &GenericRole as &dyn AriaRoleDefinitionWithConcepts,
                "caption" => &CaptionRole as &dyn AriaRoleDefinitionWithConcepts,
                "main" => &MainRole as &dyn AriaRoleDefinitionWithConcepts,
                "time" => &TimeRole as &dyn AriaRoleDefinitionWithConcepts,
                "p" => &ParagraphRole as &dyn AriaRoleDefinitionWithConcepts,
                "aside" => &ComplementaryRole as &dyn AriaRoleDefinitionWithConcepts,
                "blockquote" => &BlockQuoteRole as &dyn AriaRoleDefinitionWithConcepts,
                "associationlist" => &AssociationListRole as &dyn AriaRoleDefinitionWithConcepts,
                "status" => &StatusRole as &dyn AriaRoleDefinitionWithConcepts,
                "contentinfo" => &ContentInfoRole as &dyn AriaRoleDefinitionWithConcepts,
                "region" => &RegionRole as &dyn AriaRoleDefinitionWithConcepts,
                _ => return false,
            };
            if let Some(mut concepts) = role.concepts_by_element_name(element_name) {
                if concepts.any(|(name, _)| *name == element_name) && !role.is_interactive() {
                    return true;
                }
            }
        }

        false
    }

    /// Given a role, it returns the corresponding elements and attributes associated to that role
    pub fn get_elements_by_role(&self, role: &str) -> ElementsAndAttributes {
        let role_candidate = match role {
            "checkbox" => &CheckboxRole as &dyn AriaRoleDefinitionWithConcepts,
            "radio" => &RadioRole as &dyn AriaRoleDefinitionWithConcepts,
            "option" => &OptionRole as &dyn AriaRoleDefinitionWithConcepts,
            "combobox" => &ComboBoxRole as &dyn AriaRoleDefinitionWithConcepts,
            "heading" => &HeadingRole as &dyn AriaRoleDefinitionWithConcepts,
            "separator" => &SeparatorRole as &dyn AriaRoleDefinitionWithConcepts,
            "button" => &ButtonRole as &dyn AriaRoleDefinitionWithConcepts,
            "article" => &ArticleRole as &dyn AriaRoleDefinitionWithConcepts,
            "dialog" => &DialogRole as &dyn AriaRoleDefinitionWithConcepts,
            "cell" => &CellRole as &dyn AriaRoleDefinitionWithConcepts,
            "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
            "definition" => &DefinitionRole as &dyn AriaRoleDefinitionWithConcepts,
            "figure" => &FigureRole as &dyn AriaRoleDefinitionWithConcepts,
            "form" => &FormRole as &dyn AriaRoleDefinitionWithConcepts,
            "graphics-document" => &GraphicsDocumentRole as &dyn AriaRoleDefinitionWithConcepts,
            "graphics-object" => &GraphicsObjectRole as &dyn AriaRoleDefinitionWithConcepts,
            "grid" => &GridRole as &dyn AriaRoleDefinitionWithConcepts,
            "gridcell" => &GridCellRole as &dyn AriaRoleDefinitionWithConcepts,
            "group" => &GroupRole as &dyn AriaRoleDefinitionWithConcepts,
            "img" => &ImgRole as &dyn AriaRoleDefinitionWithConcepts,
            "link" => &LinkRole as &dyn AriaRoleDefinitionWithConcepts,
            "list" => &ListRole as &dyn AriaRoleDefinitionWithConcepts,
            "listbox" => &ListBoxRole as &dyn AriaRoleDefinitionWithConcepts,
            "listitem" => &ListItemRole as &dyn AriaRoleDefinitionWithConcepts,
            "navigation" => &NavigationRole as &dyn AriaRoleDefinitionWithConcepts,
            "row" => &RowRole as &dyn AriaRoleDefinitionWithConcepts,
            "rowgroup" => &RowGroupRole as &dyn AriaRoleDefinitionWithConcepts,
            "rowheader" => &RowHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
            "search" => &SearchRole as &dyn AriaRoleDefinitionWithConcepts,
            "searchbox" => &SearchboxRole as &dyn AriaRoleDefinitionWithConcepts,
            "table" => &TableRole as &dyn AriaRoleDefinitionWithConcepts,
            "term" => &TermRole as &dyn AriaRoleDefinitionWithConcepts,
            "textbox" => &TextboxRole as &dyn AriaRoleDefinitionWithConcepts,
            "generic" => &GenericRole as &dyn AriaRoleDefinitionWithConcepts,
            "caption" => &CaptionRole as &dyn AriaRoleDefinitionWithConcepts,
            "main" => &MainRole as &dyn AriaRoleDefinitionWithConcepts,
            "time" => &TimeRole as &dyn AriaRoleDefinitionWithConcepts,
            "p" => &ParagraphRole as &dyn AriaRoleDefinitionWithConcepts,
            "aside" => &ComplementaryRole as &dyn AriaRoleDefinitionWithConcepts,
            "blockquote" => &BlockQuoteRole as &dyn AriaRoleDefinitionWithConcepts,
            "associationlist" => &AssociationListRole as &dyn AriaRoleDefinitionWithConcepts,
            "status" => &StatusRole as &dyn AriaRoleDefinitionWithConcepts,
            "contentinfo" => &ContentInfoRole as &dyn AriaRoleDefinitionWithConcepts,
            "region" => &RegionRole as &dyn AriaRoleDefinitionWithConcepts,
            _ => return None,
        };

        role_candidate.concepts_by_role()
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
                |r| AriaRole::from_str(r).ok(),
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
            "dl" | "label" | "legend" | "ruby" | "pre" | "figcaption" | "br" => true,
            _ => false,
        } {
            return true;
        }

        let role_name = self.get_role_by_element_name(element_name, attributes);

        match role_name {
            None | Some(AriaRole::Presentation | AriaRole::Generic) => false,
            Some(_) => true,
        }
    }
}

type ElementsAndAttributes<'a> = Option<Iter<'a, (&'a str, &'a [(&'a str, &'a str)])>>;

pub trait AriaRoleDefinitionWithConcepts: AriaRoleDefinition {
    fn concepts_by_element_name<'a>(&self, element_name: &str) -> ElementsAndAttributes<'a> {
        if let Some(iter) = self.concepts_by_role() {
            for (concept_name, _attributes) in iter {
                if *concept_name == element_name {
                    return self.concepts_by_role();
                }
            }
        }
        None
    }

    fn concepts_by_role<'a>(&self) -> ElementsAndAttributes<'a> {
        None
    }
}

/// Convenient type to retrieve metadata regarding ARIA roles
#[derive(Debug, Default)]
pub struct AriaRoles;

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
