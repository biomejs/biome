use crate::{define_role, is_aria_property_valid};
use biome_aria_metadata::AriaPropertiesEnum;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

pub trait AriaRoleDefinition: Debug {
    /// It returns an iterator over the properties of the current role
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    /// let checkbox_role = roles.get_role("checkbox").unwrap();
    ///
    /// let properties = checkbox_role.properties();
    /// assert_eq!(properties.len(), 2);
    /// ```
    fn properties(&self) -> Iter<(&str, bool)>;

    /// It returns an iterator over the possible roles of this definition
    fn roles(&self) -> Iter<&str>;

    /// Given a [aria property](ARIA_PROPERTIES) as input, it checks if it's required
    /// for the current role.
    ///
    /// If the property doesn't exist for the current role, [false] is returned.
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use biome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    /// let checkbox_role = roles.get_role("checkbox").unwrap();
    ///
    /// assert_eq!(checkbox_role.is_property_required("aria-readonly"), false);
    /// assert_eq!(checkbox_role.is_property_required("aria-checked"), true);
    ///
    /// ```
    fn is_property_required(&self, property_to_check: &str) -> bool {
        if is_aria_property_valid(property_to_check) {
            let property_to_check = AriaPropertiesEnum::from_str(property_to_check);
            if let Ok(property_to_check) = property_to_check {
                for (property, required) in self.properties() {
                    let property = AriaPropertiesEnum::from_str(property).unwrap();
                    if property == property_to_check {
                        return *required;
                    }
                }
            }
        }
        false
    }

    /// Whether the current role is interactive
    fn is_interactive(&self) -> bool {
        self.roles().any(|role| *role == "widget")
    }

    /// Returns a concrete type name.
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#button
    ButtonRole {
        PROPS: [("aria-expanded", false), ("aria-expanded", false)],
        ROLES: ["roletype", "widget", "command"],
        CONCEPTS: &[("button", &[]), ("input", &[("type", "button")])],
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
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#switch
    SwitchRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["checkbox", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#option
    OptionRole {
        PROPS: [("aria-selected", true)],
        ROLES: ["treeitem", "widget"],
        CONCEPTS: &[("option", &[])],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#spinbutton
    SpinButtonRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["composite", "input", "range", "widget"],
        CONCEPTS: &[("hr", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    SliderRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["input", "range", "widget"],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#separator
    SeparatorRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["structure", "widget"],
        CONCEPTS: &[("hr", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#scrollbar
    ScollbarRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
            ("aria-orientation", true),
            ("aria-controls", true),
        ],
        ROLES: ["range", "widget"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#alert
    AlertRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("alert", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#alertdialog
    AlertDialogRole {
        PROPS: [],
        ROLES: ["structure"],
        CONCEPTS: &[("alert", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#application
    ApplicationRole {
        PROPS: [],
        ROLES: ["alert", "dialog"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#banner
    BannerRole {
        PROPS: [],
        ROLES: ["landmark"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#feed
    FeedRole {
        PROPS: [("aria-labelledby", false), ("aria-setsize", false)],
        ROLES: ["section"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#log
    LogRole {
        PROPS: [],
        ROLES: ["section"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#menubar
    MenubarRole {
        PROPS: [],
        ROLES: ["toolbar"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menu
    MenuRole {
        PROPS: [("aria-posinset", false), ("aria-setsize", false)],
        ROLES: ["select"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitem
    MenuItemRole {
        PROPS: [("aria-posinset", false), ("aria-setsize", false)],
        ROLES: ["command", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemcheckbox
    MenuItemCheckboxRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["checkbox", "menuitem", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemradio
    MenuItemRadioRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["radio", "menuitemcheckbox", "widget"],
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
    /// https://www.w3.org/TR/wai-aria-1.2/#note
    NoteRole {
        PROPS: [],
        ROLES: ["section"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#progressbar
    ProgressBarRole {
        PROPS: [("aria-valuenow", true), ("aria-valuemin", true), ("aria-valuemax", true)],
        ROLES: ["range", "widget"],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radiogroup
    RadiogroupRole {
        PROPS: [("aria-readonly", false), ("aria-required", false)],
        ROLES: ["range"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#tab
    TabRole {
        PROPS: [("aria-posinset", false), ("aria-selected", false), ("aria-setsize", false)],
        ROLES: ["sectionhead", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#tablist
    TabListRole {
        PROPS: [("aria-multiselectable", false), ("aria-orientation", false)],
        ROLES: ["composite"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#tabpanel
    TabPanelRole {
        PROPS: [],
        ROLES: ["section"],
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
    /// https://www.w3.org/TR/wai-aria-1.1/#toolbar
    ToolbarRole {
        PROPS: [("aria-orientation", false)],
        ROLES: ["group"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#tooltip
    TooltipRole {
        PROPS: [],
        ROLES: ["section"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tree
    TreeRole {
        PROPS: [("aria-multiselectable", false), ("aria-required", false)],
        ROLES: ["select"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#treegrid
    TreeGridRole {
        PROPS: [],
        ROLES: ["grid", "tree"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#treeitem
    TreeItemRole {
        PROPS: [("aria-expanded", false), ("aria-haspopup", false)],
        ROLES: ["listitem", "option"],
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
    /// https://www.w3.org/TR/wai-aria-1.2/#code
    CodeRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("code", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#deletion
    DeletionRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("del", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#directory
    DirectoryRole {
        PROPS: [],
        ROLES: ["list"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#document
    DocumentRole {
        PROPS: [],
        ROLES: ["structure"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#emphasis
    EmphasisRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("em", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#insertion
    InsertionRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("ins", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#math
    MathRole {
        PROPS: [],
        ROLES: ["section"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#strong
    StrongRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("strong", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#subscript
    SubScriptRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("sub", &[]), ("sup", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#superscript
    SuperScriptRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("sub", &[]), ("sup", &[])],
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
    /// https://www.w3.org/TR/wai-aria-1.2/#time
    TimeRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("time", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#timer
    TimerRole {
        PROPS: [],
        ROLES: ["status"],
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
    /// https://www.w3.org/TR/wai-aria-1.2/#meter
    MeterRole {
        PROPS: [],
        ROLES: ["range"],
        CONCEPTS: &[("meter", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.2/#presentation
    PresentationRole {
        PROPS: [],
        ROLES: ["structure"],
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
    /// https://w3c.github.io/aria/#mark
    MarkRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("mark", &[])],
    }
}

define_role! {
    /// https://w3c.github.io/aria/#marquee
    MarqueeRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("marquee", &[])],
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
        "alert",
        "alertdialog",
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
    ];

    /// It returns the metadata of a role, if it exits.
    /// These roles don't include abstract roles.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    ///
    /// let button_role = roles.get_role("button");
    /// let made_up_role = roles.get_role("made-up");
    ///
    /// assert!(button_role.is_some());
    /// assert!(made_up_role.is_none());
    /// ```
    pub fn get_role(&self, role: &str) -> Option<&'static dyn AriaRoleDefinition> {
        let result = match role {
            "alert" => &AlertRole as &dyn AriaRoleDefinition,
            "alertdialog" => &AlertDialogRole as &dyn AriaRoleDefinition,
            "application" => &ApplicationRole as &dyn AriaRoleDefinition,
            "article" => &ArticleRole as &dyn AriaRoleDefinition,
            "banner" => &BannerRole as &dyn AriaRoleDefinition,
            "blockquote" => &BlockQuoteRole as &dyn AriaRoleDefinition,
            "button" => &ButtonRole as &dyn AriaRoleDefinition,
            "caption" => &CaptionRole as &dyn AriaRoleDefinition,
            "cell" => &CellRole as &dyn AriaRoleDefinition,
            "checkbox" => &CheckboxRole as &dyn AriaRoleDefinition,
            "code" => &CodeRole as &dyn AriaRoleDefinition,
            "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinition,
            "combobox" => &ComboBoxRole as &dyn AriaRoleDefinition,
            "complementary" => &ComplementaryRole as &dyn AriaRoleDefinition,
            "contentinfo" => &ContentInfoRole as &dyn AriaRoleDefinition,
            "definition" => &DefinitionRole as &dyn AriaRoleDefinition,
            "delete" => &DeletionRole as &dyn AriaRoleDefinition,
            "dialog" => &DialogRole as &dyn AriaRoleDefinition,
            "directory" => &DirectoryRole as &dyn AriaRoleDefinition,
            "document" => &DocumentRole as &dyn AriaRoleDefinition,
            "emphasis" => &EmphasisRole as &dyn AriaRoleDefinition,
            "feed" => &FeedRole as &dyn AriaRoleDefinition,
            "figure" => &FigureRole as &dyn AriaRoleDefinition,
            "form" => &FormRole as &dyn AriaRoleDefinition,
            "generic" => &GenericRole as &dyn AriaRoleDefinition,
            "grid" => &GridRole as &dyn AriaRoleDefinition,
            "gridcell" => &GridCellRole as &dyn AriaRoleDefinition,
            "group" => &GroupRole as &dyn AriaRoleDefinition,
            "heading" => &HeadingRole as &dyn AriaRoleDefinition,
            "img" => &ImgRole as &dyn AriaRoleDefinition,
            "insertion" => &InsertionRole as &dyn AriaRoleDefinition,
            "link" => &LinkRole as &dyn AriaRoleDefinition,
            "list" => &ListRole as &dyn AriaRoleDefinition,
            "listbox" => &ListBoxRole as &dyn AriaRoleDefinition,
            "listitem" => &ListItemRole as &dyn AriaRoleDefinition,
            "log" => &LogRole as &dyn AriaRoleDefinition,
            "main" => &MainRole as &dyn AriaRoleDefinition,
            "marquee" => &MarqueeRole as &dyn AriaRoleDefinition,
            "math" => &MathRole as &dyn AriaRoleDefinition,
            "menu" => &MenuRole as &dyn AriaRoleDefinition,
            "menubar" => &MenubarRole as &dyn AriaRoleDefinition,
            "menuitem" => &MenuItemRole as &dyn AriaRoleDefinition,
            "menuitemcheckbox" => &MenuItemCheckboxRole as &dyn AriaRoleDefinition,
            "menuitemradio" => &MenuItemRadioRole as &dyn AriaRoleDefinition,
            "meter" => &MeterRole as &dyn AriaRoleDefinition,
            "navigation" => &NavigationRole as &dyn AriaRoleDefinition,
            "note" => &NoteRole as &dyn AriaRoleDefinition,
            "option" => &OptionRole as &dyn AriaRoleDefinition,
            "paragraph" => &ParagraphRole as &dyn AriaRoleDefinition,
            "presentation" => &PresentationRole as &dyn AriaRoleDefinition,
            "progressbar" => &ProgressBarRole as &dyn AriaRoleDefinition,
            "radio" => &RadioRole as &dyn AriaRoleDefinition,
            "radiogroup" => &RadiogroupRole as &dyn AriaRoleDefinition,
            "region" => &RegionRole as &dyn AriaRoleDefinition,
            "row" => &RowRole as &dyn AriaRoleDefinition,
            "rowgroup" => &RowGroupRole as &dyn AriaRoleDefinition,
            "rowheader" => &RowHeaderRole as &dyn AriaRoleDefinition,
            "search" => &SearchRole as &dyn AriaRoleDefinition,
            "searchbox" => &SearchboxRole as &dyn AriaRoleDefinition,
            "separator" => &SeparatorRole as &dyn AriaRoleDefinition,
            "slider" => &SliderRole as &dyn AriaRoleDefinition,
            "spinbutton" => &SpinButtonRole as &dyn AriaRoleDefinition,
            "scrollbar" => &ScollbarRole as &dyn AriaRoleDefinition,
            "status" => &StatusRole as &dyn AriaRoleDefinition,
            "strong" => &StrongRole as &dyn AriaRoleDefinition,
            "subscript" => &SubScriptRole as &dyn AriaRoleDefinition,
            "superscript" => &SuperScriptRole as &dyn AriaRoleDefinition,
            "switch" => &SwitchRole as &dyn AriaRoleDefinition,
            "tab" => &TabRole as &dyn AriaRoleDefinition,
            "table" => &TableRole as &dyn AriaRoleDefinition,
            "tablist" => &TabListRole as &dyn AriaRoleDefinition,
            "tabpanel" => &TabPanelRole as &dyn AriaRoleDefinition,
            "term" => &TermRole as &dyn AriaRoleDefinition,
            "textbox" => &TextboxRole as &dyn AriaRoleDefinition,
            "time" => &TimeRole as &dyn AriaRoleDefinition,
            "timer" => &TimerRole as &dyn AriaRoleDefinition,
            "toolbar" => &ToolbarRole as &dyn AriaRoleDefinition,
            "tooltip" => &TooltipRole as &dyn AriaRoleDefinition,
            "tree" => &TreeRole as &dyn AriaRoleDefinition,
            "treegrid" => &TreeGridRole as &dyn AriaRoleDefinition,
            "treeitem" => &TreeItemRole as &dyn AriaRoleDefinition,
            _ => return None,
        };
        Some(result)
    }

    /// Given a element and attributes, it returns the metadata of the element's implicit role.
    ///
    /// Check: https://www.w3.org/TR/html-aria/#docconformance
    pub fn get_implicit_role(
        &self,
        element: &str,
        // To generate `attributes`, you can use `biome_js_analyze::aria_services::AriaServices::extract_defined_attributes`
        attributes: &FxHashMap<String, Vec<String>>,
    ) -> Option<&'static dyn AriaRoleDefinition> {
        let result = match element {
            "article" => &ArticleRole as &dyn AriaRoleDefinition,
            "aside" => &ComplementaryRole as &dyn AriaRoleDefinition,
            "blockquote" => &BlockQuoteRole as &dyn AriaRoleDefinition,
            "button" => &ButtonRole as &dyn AriaRoleDefinition,
            "caption" => &CaptionRole as &dyn AriaRoleDefinition,
            "code" => &CodeRole as &dyn AriaRoleDefinition,
            "datalist" => &ListBoxRole as &dyn AriaRoleDefinition,
            "del" => &DeletionRole as &dyn AriaRoleDefinition,
            "dfn" => &TermRole as &dyn AriaRoleDefinition,
            "dialog" => &DialogRole as &dyn AriaRoleDefinition,
            "em" => &EmphasisRole as &dyn AriaRoleDefinition,
            "figure" => &FigureRole as &dyn AriaRoleDefinition,
            "form" => &FormRole as &dyn AriaRoleDefinition,
            "hr" => &SeparatorRole as &dyn AriaRoleDefinition,
            "html" => &DocumentRole as &dyn AriaRoleDefinition,
            "ins" => &InsertionRole as &dyn AriaRoleDefinition,
            "main" => &MainRole as &dyn AriaRoleDefinition,
            "marquee" => &MarqueeRole as &dyn AriaRoleDefinition,
            "math" => &MathRole as &dyn AriaRoleDefinition,
            "menu" => &ListRole as &dyn AriaRoleDefinition,
            "meter" => &MeterRole as &dyn AriaRoleDefinition,
            "nav" => &NavigationRole as &dyn AriaRoleDefinition,
            "ul" | "ol" => &ListRole as &dyn AriaRoleDefinition,
            "li" => &ListItemRole as &dyn AriaRoleDefinition,
            "optgroup" => &GroupRole as &dyn AriaRoleDefinition,
            "output" => &StatusRole as &dyn AriaRoleDefinition,
            "p" => &ParagraphRole as &dyn AriaRoleDefinition,
            "progress" => &ProgressBarRole as &dyn AriaRoleDefinition,
            "search" => &SearchRole as &dyn AriaRoleDefinition,
            "strong" => &StrongRole as &dyn AriaRoleDefinition,
            "sub" => &SubScriptRole as &dyn AriaRoleDefinition,
            "sup" => &SuperScriptRole as &dyn AriaRoleDefinition,
            "svg" => &GraphicsDocumentRole as &dyn AriaRoleDefinition,
            "table" => &TableRole as &dyn AriaRoleDefinition,
            "textarea" => &TextboxRole as &dyn AriaRoleDefinition,
            "tr" => &RowRole as &dyn AriaRoleDefinition,
            "time" => &TimeRole as &dyn AriaRoleDefinition,
            "address" | "details" | "fieldset" => &GroupRole as &dyn AriaRoleDefinition,
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => &HeadingRole as &dyn AriaRoleDefinition,
            "tbody" | "tfoot" | "thead" => &RowGroupRole as &dyn AriaRoleDefinition,
            "input" => {
                let type_values = attributes.get("type")?;
                match type_values.first()?.as_str() {
                    "checkbox" => &CheckboxRole as &dyn AriaRoleDefinition,
                    "number" => &SpinButtonRole as &dyn AriaRoleDefinition,
                    "radio" => &RadioRole as &dyn AriaRoleDefinition,
                    "range" => &SliderRole as &dyn AriaRoleDefinition,
                    "button" | "image" | "reset" | "submit" => {
                        &ButtonRole as &dyn AriaRoleDefinition
                    }
                    "search" => match attributes.get("list") {
                        Some(_) => &ComboBoxRole as &dyn AriaRoleDefinition,
                        _ => &SearchboxRole as &dyn AriaRoleDefinition,
                    },
                    "email" | "tel" | "url" => match attributes.get("list") {
                        Some(_) => &ComboBoxRole as &dyn AriaRoleDefinition,
                        _ => &TextboxRole as &dyn AriaRoleDefinition,
                    },
                    "text" => &TextboxRole as &dyn AriaRoleDefinition,
                    _ => &TextboxRole as &dyn AriaRoleDefinition,
                }
            }
            "a" | "area" => match attributes.get("href") {
                Some(_) => &LinkRole as &dyn AriaRoleDefinition,
                _ => &GenericRole as &dyn AriaRoleDefinition,
            },
            "img" => match attributes.get("alt") {
                Some(values) => {
                    if values.iter().any(|x| !x.is_empty()) {
                        &ImgRole as &dyn AriaRoleDefinition
                    } else {
                        &PresentationRole as &dyn AriaRoleDefinition
                    }
                }
                None => &ImgRole as &dyn AriaRoleDefinition,
            },
            "section" => {
                let has_accessible_name = attributes.get("aria-labelledby").is_some()
                    || attributes.get("aria-label").is_some()
                    || attributes.get("title").is_some();
                if has_accessible_name {
                    &RegionRole as &dyn AriaRoleDefinition
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
                    &ComboBoxRole as &dyn AriaRoleDefinition
                } else {
                    &ListBoxRole as &dyn AriaRoleDefinition
                }
            }
            "b" | "bdi" | "bdo" | "body" | "data" | "div" | "hgroup" | "i" | "q" | "samp"
            | "small" | "span" | "u" => &GenericRole as &dyn AriaRoleDefinition,
            "header" | "footer" => {
                // This crate does not support checking a descendant of an element.
                // header (maybe BannerRole): https://www.w3.org/WAI/ARIA/apg/patterns/landmarks/examples/banner.html
                // footer (maybe ContentInfoRole): https://www.w3.org/WAI/ARIA/apg/patterns/landmarks/examples/contentinfo.html
                &GenericRole as &dyn AriaRoleDefinition
            }
            _ => return None,
        };
        Some(result)
    }

    /// Given a role, it returns whether this role is interactive
    pub fn is_role_interactive(&self, role: &str) -> bool {
        let role = self.get_role(role);
        if let Some(role) = role {
            role.is_interactive()
        } else {
            false
        }
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
                "alert" => &AlertRole as &dyn AriaRoleDefinitionWithConcepts,
                "alertdialog" => &AlertDialogRole as &dyn AriaRoleDefinitionWithConcepts,
                "cell" => &CellRole as &dyn AriaRoleDefinitionWithConcepts,
                "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
                "definition" => &DefinitionRole as &dyn AriaRoleDefinitionWithConcepts,
                "figure" => &FigureRole as &dyn AriaRoleDefinitionWithConcepts,
                "form" => &FormRole as &dyn AriaRoleDefinitionWithConcepts,
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
}

type ElementsAndAttributes<'a> = Option<Iter<'a, (&'a str, &'a [(&'a str, &'a str)])>>;

pub trait AriaRoleDefinitionWithConcepts: AriaRoleDefinition {
    fn concepts_by_element_name<'a>(&self, _element_name: &str) -> ElementsAndAttributes<'a> {
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
        assert_eq!(implicit_role.type_name(), "biome_aria::roles::ButtonRole");

        // <input type="search">
        let mut attributes = FxHashMap::default();
        attributes.insert("type".to_string(), vec!["search".to_string()]);
        let implicit_role = aria_roles.get_implicit_role("input", &attributes).unwrap();
        assert_eq!(
            implicit_role.type_name(),
            "biome_aria::roles::SearchboxRole"
        );

        // <select name="animals" multiple size="4">
        let mut attributes = FxHashMap::default();
        attributes.insert("name".to_string(), vec!["animals".to_string()]);
        attributes.insert("multiple".to_string(), vec![String::new()]);
        attributes.insert("size".to_string(), vec!["4".to_string()]);
        let implicit_role = aria_roles.get_implicit_role("select", &attributes).unwrap();
        assert_eq!(implicit_role.type_name(), "biome_aria::roles::ListBoxRole");
    }
}
