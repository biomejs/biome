/// Determines the kind of script that is contained in a `<script>` tag.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScriptType {
    /// Used for any `<script>` tag that either has no `type` attribute, or that
    /// has a `type` attribute with a supported JavaScript MIME type.
    #[default]
    Classic,

    /// `<script type="module">`
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules#applying_the_module_to_your_html
    Module,

    /// `<script type="importmap">`
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script/type/importmap
    ImportMap,

    /// `<script type="application/json">`
    JSON,

    /// The `<script>` tag has a `type` attribute, but the value is not
    /// supported by Biome.
    Unsupported,
}

impl ScriptType {
    /// Returns the script type based on the value of a `<script type="...">`
    /// attribute.
    pub fn from_type_value(type_value: &str) -> Self {
        if type_value.eq_ignore_ascii_case("module") {
            Self::Module
        } else if type_value.eq_ignore_ascii_case("text/javascript")
            || type_value.eq_ignore_ascii_case("application/javascript")
            || type_value.eq_ignore_ascii_case("application/ecmascript")
        {
            Self::Classic
        } else if type_value.eq_ignore_ascii_case("importmap") {
            Self::ImportMap
        } else if type_value.eq_ignore_ascii_case("application/json") {
            Self::JSON
        } else {
            Self::Unsupported
        }
    }

    /// Returns whether the script type indicates JavaScript content.
    ///
    /// Returns `true` for both classic scripts as well as ECMAScript modules.
    pub fn is_javascript(self) -> bool {
        matches!(self, Self::Classic | Self::Module)
    }

    /// Returns whether the script type indicates a JavaScript module.
    pub fn is_javascript_module(self) -> bool {
        matches!(self, Self::Module)
    }

    /// Returns whether the script type indicates JSON content.
    ///
    /// Returns `true` for JSON content, including import maps.
    pub fn is_json(self) -> bool {
        matches!(self, Self::JSON | Self::ImportMap)
    }

    /// Returns `true` for any script type that can be handled by Biome.
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }
}
