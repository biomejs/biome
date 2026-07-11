use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;
use std::borrow::Cow;
use tracing::instrument;

mod db;

#[cfg(feature = "lang_css")]
pub mod css;
#[cfg(feature = "lang_graphql")]
pub mod graphql;
#[cfg(feature = "lang_grit")]
pub mod grit;
#[cfg(feature = "lang_html")]
pub mod html;
#[cfg(feature = "lang_js")]
pub mod javascript;
#[cfg(feature = "lang_json")]
pub mod json;
#[cfg(feature = "lang_md")]
pub mod md;
#[cfg(feature = "lang_yaml")]
pub mod yaml;

#[cfg(feature = "lang_css")]
pub use crate::css::CssFileSource;
#[cfg(feature = "lang_graphql")]
pub use crate::graphql::GraphqlFileSource;
#[cfg(feature = "lang_grit")]
pub use crate::grit::GritFileSource;
#[cfg(feature = "lang_html")]
pub use crate::html::HtmlFileSource;
#[cfg(feature = "lang_js")]
pub use crate::javascript::JsFileSource;
#[cfg(feature = "lang_json")]
pub use crate::json::JsonFileSource;
#[cfg(feature = "lang_md")]
pub use crate::md::MdFileSource;
#[cfg(feature = "lang_yaml")]
pub use crate::yaml::YamlFileSource;

pub use crate::db::LanguageDb;

// NOTE: when adding a new ignore file, update [DocumentFileSource::try_from_path]
pub const GIT_IGNORE_FILE_NAME: &str = ".gitignore";
pub const IGNORE_FILE_NAME: &str = ".ignore";

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum DocumentFileSource {
    #[cfg(feature = "lang_js")]
    Js(JsFileSource),
    #[cfg(feature = "lang_json")]
    Json(JsonFileSource),
    #[cfg(feature = "lang_css")]
    Css(CssFileSource),
    #[cfg(feature = "lang_graphql")]
    Graphql(GraphqlFileSource),
    #[cfg(feature = "lang_html")]
    Html(HtmlFileSource),
    #[cfg(feature = "lang_grit")]
    Grit(GritFileSource),
    #[cfg(feature = "lang_md")]
    Markdown(MdFileSource),
    #[cfg(feature = "lang_yaml")]
    Yaml(YamlFileSource),
    // Ignore files
    Ignore,
    #[default]
    Unknown,
}

#[cfg(feature = "lang_js")]
impl From<JsFileSource> for DocumentFileSource {
    fn from(value: JsFileSource) -> Self {
        Self::Js(value)
    }
}

#[cfg(feature = "lang_json")]
impl From<JsonFileSource> for DocumentFileSource {
    fn from(value: JsonFileSource) -> Self {
        Self::Json(value)
    }
}

#[cfg(feature = "lang_css")]
impl From<CssFileSource> for DocumentFileSource {
    fn from(value: CssFileSource) -> Self {
        Self::Css(value)
    }
}
#[cfg(feature = "lang_graphql")]
impl From<GraphqlFileSource> for DocumentFileSource {
    fn from(value: GraphqlFileSource) -> Self {
        Self::Graphql(value)
    }
}

#[cfg(feature = "lang_html")]
impl From<HtmlFileSource> for DocumentFileSource {
    fn from(value: HtmlFileSource) -> Self {
        Self::Html(value)
    }
}

#[cfg(feature = "lang_grit")]
impl From<GritFileSource> for DocumentFileSource {
    fn from(value: GritFileSource) -> Self {
        Self::Grit(value)
    }
}

#[cfg(feature = "lang_md")]
impl From<MdFileSource> for DocumentFileSource {
    fn from(value: MdFileSource) -> Self {
        Self::Markdown(value)
    }
}

#[cfg(feature = "lang_yaml")]
impl From<YamlFileSource> for DocumentFileSource {
    fn from(value: YamlFileSource) -> Self {
        Self::Yaml(value)
    }
}

impl From<&Utf8Path> for DocumentFileSource {
    fn from(path: &Utf8Path) -> Self {
        Self::from_path(path, false)
    }
}

impl DocumentFileSource {
    fn try_from_well_known(
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        #[cfg(feature = "lang_json")]
        if let Ok(file_source) = JsonFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        if experimental_full_html_support {
            #[cfg(feature = "lang_html")]
            if let Ok(file_source) = HtmlFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
            #[cfg(feature = "lang_js")]
            if let Ok(file_source) = JsFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
        } else {
            #[cfg(feature = "lang_js")]
            if let Ok(file_source) = JsFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
            #[cfg(feature = "lang_html")]
            if let Ok(file_source) = HtmlFileSource::try_from_well_known(path) {
                return Ok(file_source.into());
            }
        }

        #[cfg(feature = "lang_css")]
        if let Ok(file_source) = CssFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_graphql")]
        if let Ok(file_source) = GraphqlFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }

        #[cfg(feature = "lang_md")]
        if let Ok(file_source) = MdFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }

        #[cfg(feature = "lang_yaml")]
        if let Ok(file_source) = YamlFileSource::try_from_well_known(path) {
            return Ok(file_source.into());
        }
        let _ = path;
        Err(FileSourceError::UnknownFileName)
    }

    /// Returns the document file source corresponding to this file name from well-known files
    pub fn from_well_known(path: &Utf8Path, experimental_full_html_support: bool) -> Self {
        Self::try_from_well_known(path, experimental_full_html_support).unwrap_or(Self::Unknown)
    }

    fn try_from_extension(
        extension: &str,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        // Order here is important
        if experimental_full_html_support {
            #[cfg(feature = "lang_html")]
            if let Ok(file_source) = HtmlFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
            #[cfg(feature = "lang_js")]
            if let Ok(file_source) = JsFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
        } else {
            #[cfg(feature = "lang_js")]
            if let Ok(file_source) = JsFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
            #[cfg(feature = "lang_html")]
            if let Ok(file_source) = HtmlFileSource::try_from_extension(extension) {
                return Ok(file_source.into());
            }
        }

        #[cfg(feature = "lang_json")]
        if let Ok(file_source) = JsonFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_css")]
        if let Ok(file_source) = CssFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_graphql")]
        if let Ok(file_source) = GraphqlFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_grit")]
        if let Ok(file_source) = GritFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_md")]
        if let Ok(file_source) = MdFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_yaml")]
        if let Ok(file_source) = YamlFileSource::try_from_extension(extension) {
            return Ok(file_source.into());
        }
        let _ = extension;
        Err(FileSourceError::UnknownExtension)
    }

    /// Returns the document file source corresponding to this file extension
    pub fn from_extension(extension: &str, experimental_full_html_support: bool) -> Self {
        Self::try_from_extension(extension, experimental_full_html_support).unwrap_or(Self::Unknown)
    }

    #[instrument(level = "debug", fields(result))]
    fn try_from_language_id(
        language_id: &str,
        extension: Option<&str>,
    ) -> Result<Self, FileSourceError> {
        #[cfg(feature = "lang_json")]
        if let Ok(file_source) = JsonFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_js")]
        if let Ok(file_source) = JsFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_css")]
        if let Ok(file_source) = CssFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_graphql")]
        if let Ok(file_source) = GraphqlFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_html")]
        if let Ok(file_source) = HtmlFileSource::try_from_language_id(language_id, extension) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_grit")]
        if let Ok(file_source) = GritFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        #[cfg(feature = "lang_md")]
        if let Ok(file_source) = MdFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }

        #[cfg(feature = "lang_yaml")]
        if let Ok(file_source) = YamlFileSource::try_from_language_id(language_id) {
            return Ok(file_source.into());
        }
        Err(FileSourceError::UnknownLanguageId)
    }

    /// Returns the document file source corresponding to this language ID. It accepts an optional extension
    /// to be used to narrow down the expected language to enable.
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn from_language_id(language_id: &str, extension: Option<&str>) -> Self {
        Self::try_from_language_id(language_id, extension).unwrap_or(Self::Unknown)
    }

    pub(crate) fn try_from_path(
        path: &Utf8Path,
        experimental_full_html_support: bool,
    ) -> Result<Self, FileSourceError> {
        if let Ok(file_source) = Self::try_from_well_known(path, experimental_full_html_support) {
            return Ok(file_source);
        }

        let filename = path
            .file_name()
            // We assume the file extensions are case-insensitive.
            // Thus, we normalize the filrname to lowercase.
            .map(|filename| filename.to_ascii_lowercase_cow());

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        // TODO: This should be extracted to a dedicated function, maybe in biome_fs
        // because the same logic is also used in JsFileSource::try_from
        // and we may support more and more extensions with more than one dots.
        let extension = &match filename {
            // Ignore files are extensionless files, so they need to be handled in particular way
            Some(filename) if filename == GIT_IGNORE_FILE_NAME || filename == IGNORE_FILE_NAME => {
                return Ok(Self::Ignore);
            }
            Some(filename) if filename.ends_with(".d.ts") => Cow::Borrowed("d.ts"),
            Some(filename) if filename.ends_with(".d.mts") => Cow::Borrowed("d.mts"),
            Some(filename) if filename.ends_with(".d.cts") => Cow::Borrowed("d.cts"),
            _ => path
                .extension()
                // We assume the file extensions are case-insensitive.
                // Thus, we normalize the extension to lowercase.
                .map(|ext| ext.to_ascii_lowercase_cow())
                .ok_or(FileSourceError::MissingFileExtension)?,
        };

        Self::try_from_extension(extension.as_ref(), experimental_full_html_support)
    }

    /// Returns the document file source corresponding to the file path
    pub fn from_path(path: &Utf8Path, experimental_full_html_support: bool) -> Self {
        Self::try_from_path(path, experimental_full_html_support).unwrap_or(Self::Unknown)
    }

    /// Returns the document file source if it's not unknown, otherwise returns `other`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use biome_languages::JsFileSource;
    /// use biome_languages::DocumentFileSource;
    /// let x = DocumentFileSource::from(JsFileSource::js_module());
    /// let y = DocumentFileSource::Unknown;
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::Unknown;
    /// let y = DocumentFileSource::from(JsFileSource::js_module());
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::from(JsFileSource::js_module());
    /// let y = DocumentFileSource::from(JsonFileSource::json());
    /// assert_eq!(x.or(y), JsFileSource::js_module().into());
    ///
    /// let x = DocumentFileSource::Unknown;
    /// let y = DocumentFileSource::Unknown;
    /// assert_eq!(x.or(y), DocumentFileSource::Unknown);
    /// ```
    pub fn or(self, other: Self) -> Self {
        if self != Self::Unknown { self } else { other }
    }

    #[cfg(feature = "lang_js")]
    pub const fn is_javascript_like(&self) -> bool {
        matches!(self, Self::Js(_))
    }

    #[cfg(feature = "lang_json")]
    pub const fn is_json_like(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    #[cfg(feature = "lang_css")]
    pub const fn is_css_like(&self) -> bool {
        matches!(self, Self::Css(_))
    }

    #[cfg(feature = "lang_js")]
    pub fn to_js_file_source(&self) -> Option<JsFileSource> {
        match self {
            Self::Js(file_source) => Some(*file_source),
            _ => None,
        }
    }

    #[cfg(feature = "lang_json")]
    pub fn to_json_file_source(&self) -> Option<JsonFileSource> {
        match self {
            Self::Json(json) => Some(*json),
            _ => None,
        }
    }

    #[cfg(feature = "lang_graphql")]
    pub fn to_graphql_file_source(&self) -> Option<GraphqlFileSource> {
        match self {
            Self::Graphql(graphql) => Some(*graphql),
            _ => None,
        }
    }
    #[cfg(feature = "lang_grit")]
    pub fn to_grit_file_source(&self) -> Option<GritFileSource> {
        match self {
            Self::Grit(grit) => Some(*grit),
            _ => None,
        }
    }

    #[cfg(feature = "lang_css")]
    pub fn to_css_file_source(&self) -> Option<CssFileSource> {
        match self {
            Self::Css(css) => Some(*css),
            _ => None,
        }
    }

    #[cfg(feature = "lang_html")]
    pub fn to_html_file_source(&self) -> Option<HtmlFileSource> {
        match self {
            Self::Html(html) => Some(*html),
            _ => None,
        }
    }

    #[cfg(feature = "lang_md")]
    pub fn to_markdown_file_source(&self) -> Option<MdFileSource> {
        match self {
            Self::Markdown(markdown) => Some(*markdown),
            _ => None,
        }
    }

    /// The file can be parsed
    pub fn can_parse(path: &Utf8Path) -> bool {
        let file_source = Self::from(path);
        match file_source {
            #[cfg(feature = "lang_grit")]
            Self::Grit(_) => true,
            #[cfg(feature = "lang_md")]
            Self::Markdown(_) => true,
            #[cfg(feature = "lang_yaml")]
            Self::Yaml(_) => true,
            #[cfg(feature = "lang_graphql")]
            Self::Graphql(_) => true,
            #[cfg(feature = "lang_js")]
            Self::Js(_) => true,
            #[cfg(feature = "lang_css")]
            Self::Css(_) => true,
            #[cfg(feature = "lang_json")]
            Self::Json(_) => true,
            #[cfg(feature = "lang_html")]
            Self::Html(_) => true,
            Self::Ignore => false,
            Self::Unknown => false,
        }
    }

    /// The file can be read from the file system
    pub fn can_read(path: &Utf8Path) -> bool {
        let file_source = Self::from(path);
        match file_source {
            #[cfg(feature = "lang_grit")]
            Self::Grit(_) => true,
            #[cfg(feature = "lang_md")]
            Self::Markdown(_) => true,
            #[cfg(feature = "lang_yaml")]
            Self::Yaml(_) => true,
            #[cfg(feature = "lang_graphql")]
            Self::Graphql(_) => true,
            #[cfg(feature = "lang_js")]
            Self::Js(_) => true,
            #[cfg(feature = "lang_css")]
            Self::Css(_) => true,
            #[cfg(feature = "lang_json")]
            Self::Json(_) => true,
            #[cfg(feature = "lang_html")]
            Self::Html(_) => true,
            Self::Ignore => true,
            Self::Unknown => false,
        }
    }

    /// Whether this file can contain embedded nodes
    pub fn can_contain_embeds(path: &Utf8Path, experimental_full_html_support: bool) -> bool {
        let file_source = Self::from_path(path, experimental_full_html_support);
        match file_source {
            #[cfg(feature = "lang_grit")]
            Self::Grit(_) => true,
            #[cfg(feature = "lang_md")]
            Self::Markdown(_) => false, // it will, but not yet
            #[cfg(feature = "lang_yaml")]
            Self::Yaml(_) => false,
            #[cfg(feature = "lang_graphql")]
            Self::Graphql(_) => true,
            #[cfg(feature = "lang_html")]
            Self::Html(_) => true,
            #[cfg(feature = "lang_js")]
            Self::Js(_) => true,
            #[cfg(feature = "lang_css")]
            Self::Css(_) => false,
            #[cfg(feature = "lang_json")]
            Self::Json(_) => false,
            Self::Ignore | Self::Unknown => false,
        }
    }
}

impl std::fmt::Display for DocumentFileSource {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            #[cfg(feature = "lang_js")]
            Self::Js(js) => {
                let is_jsx = js.is_jsx();
                if js.is_typescript() {
                    if is_jsx {
                        write!(fmt, "TSX")
                    } else {
                        write!(fmt, "TypeScript")
                    }
                } else if is_jsx {
                    write!(fmt, "JSX")
                } else {
                    write!(fmt, "JavaScript")
                }
            }
            #[cfg(feature = "lang_json")]
            Self::Json(json) => {
                if json.allow_comments() {
                    write!(fmt, "JSONC")
                } else {
                    write!(fmt, "JSON")
                }
            }
            #[cfg(feature = "lang_css")]
            Self::Css(_) => write!(fmt, "CSS"),
            #[cfg(feature = "lang_graphql")]
            Self::Graphql(_) => write!(fmt, "GraphQL"),
            #[cfg(feature = "lang_html")]
            Self::Html(_) => write!(fmt, "HTML"),
            #[cfg(feature = "lang_grit")]
            Self::Grit(_) => write!(fmt, "Grit"),
            #[cfg(feature = "lang_md")]
            Self::Markdown(_) => write!(fmt, "Markdown"),
            #[cfg(feature = "lang_yaml")]
            Self::Yaml(_) => write!(fmt, "YAML"),
            Self::Ignore => write!(fmt, "Ignore"),
            Self::Unknown => write!(fmt, "Unknown"),
        }
    }
}

impl biome_console::fmt::Display for DocumentFileSource {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{self}"))
    }
}
