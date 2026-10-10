use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth, TrailingNewline};
use biome_markdown_formatter::context::ProseWrap;
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Options applied to Markdown files
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownConfiguration {
    /// Parsing options
    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_parser_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<MarkdownParserConfiguration>,

    /// Formatter options
    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_formatter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<MarkdownFormatterConfiguration>,

    /// Linter options
    #[cfg_attr(
        feature = "cli",
        bpaf(external(markdown_linter_configuration), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<MarkdownLinterConfiguration>,

    /// Runs the linter and assist actions on the code inside fenced code blocks, frontmatter, and
    /// HTML blocks, with the rules and actions of the code's language. Defaults to `false`.
    ///
    /// Fenced code blocks often hold partial examples that aren't meant to be valid on their own,
    /// so Biome doesn't analyze them unless you enable this option. When enabled, Biome reports
    /// syntax errors, rule violations, and assist actions in every block whose language it
    /// supports, and the `--write` option applies their fixes. Biome reads the language of a
    /// fenced code block from the first word after the opening fence, such as `js` or `css`, and
    /// reads frontmatter as YAML. Code blocks inside lists and quotes aren't analyzed.
    ///
    /// To ignore a rule inside a block, add a suppression comment to the block's code.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-analyze-embeds"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_embeds: Option<MarkdownAnalyzeEmbeds>,
}

pub type MarkdownFormatterEnabled = Bool<true>;
pub type MarkdownLinterEnabled = Bool<true>;
pub type MarkdownAssistEnabled = Bool<true>;
pub type MarkdownParseFrontmatter = Bool<false>;
pub type MarkdownParseGfm = Bool<true>;
pub type MarkdownParseCjkFriendlyEmphasis = Bool<false>;
pub type MarkdownFormatEmbeds = Bool<true>;
pub type MarkdownAnalyzeEmbeds = Bool<false>;

/// Options that change how the Markdown parser behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownParserConfiguration {
    /// Enables parsing frontmatter at the start of the file. Defaults to `false`.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-parse-frontmatter"), argument("true|false"))
    )]
    pub frontmatter: Option<MarkdownParseFrontmatter>,

    /// Enables GitHub Flavored Markdown extensions. Defaults to `true`.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gfm: Option<MarkdownParseGfm>,

    /// Recognizes bold, italic, and strikethrough markers placed directly next to Chinese, Japanese, or Korean text. Defaults to `false`.
    ///
    /// Standard Markdown doesn't treat `**テスト。**テスト` as bold text, because the closing `**`
    /// is between a punctuation mark and a letter with no space around it. Languages that don't
    /// put spaces between words run into this often. When enabled, Biome follows the
    /// [CommonMark CJK-friendly amendments](https://github.com/tats-u/markdown-cjk-friendly/blob/main/specification.md)
    /// and treats such text as bold or italic. The option applies to `*`, `_`, and `~~` markers.
    /// After a variation selector (an invisible character that picks a glyph style for the
    /// character before it), Biome classifies a marker by the character before the selector.
    ///
    /// The option can also change markers that aren't next to Chinese, Japanese, or Korean text,
    /// because a marker that can now open or close formatting can take another marker's partner
    /// in the same paragraph, heading, or table cell. For example, standard Markdown shows
    /// `**テスト。**テスト a**` with `テスト a` in bold. With the option enabled, `テスト。` is bold
    /// and the last `**` is shown as written.
    ///
    /// The formatter and the linter both use this reading of the document. Enable the option only
    /// when the tool that renders your Markdown also supports these amendments. Otherwise, Biome
    /// treats as bold some text that your renderer shows with literal `**`, and formatting can
    /// change what your renderer shows. For example, the option makes `__` after a variation
    /// selector a bold marker, and the formatter rewrites it as `**`, which standard Markdown
    /// shows as bold even where it showed the original `__` as written.
    #[cfg_attr(all(feature = "cli", feature = "lang_md"), bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cjk_friendly_emphasis: Option<MarkdownParseCjkFriendlyEmphasis>,
}

/// Options that change how the Markdown formatter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownFormatterConfiguration {
    /// Control the formatter for Markdown (and its super languages) files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownFormatterEnabled>,

    /// The indent style applied to Markdown files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-indent-style"), argument("tab|space"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to Markdown files. Defaults to 2.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-indent-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// What's the max width of a line applied to Markdown files. Defaults to 80.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-line-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - https://thoughtbot.com/blog/no-newline-at-end-of-file
    /// - https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
    /// - https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,

    /// The type of line ending applied to Markdown (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-line-ending"), argument("lf|crlf|cr|auto"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// Controls whether Biome keeps, adds, or removes line breaks in Markdown paragraphs.
    ///
    /// Manual line breaks are always kept. In Markdown, a manual line break is created by ending a
    /// line with two spaces or a backslash.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-prose-wrap"), argument("preserve|always|never"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose_wrap: Option<ProseWrap>,

    /// Formats the code inside fenced code blocks, frontmatter, and HTML blocks with the formatter
    /// of the code's language. Defaults to `true`.
    ///
    /// Biome reads the language of a fenced code block from the first word after the opening
    /// fence, such as `js` or `css`, and reads frontmatter as YAML. Biome formats a block only when
    /// it supports the block's language and the formatter for that language is enabled. It uses
    /// that language's formatter options, except for the indentation, line width, and line ending,
    /// which come from the Markdown formatter options. Other blocks stay as written. For example,
    /// HTML blocks stay as written unless `html.formatter.enabled` is `true`. Blocks inside lists
    /// and quotes, and indented HTML blocks, also stay as written.
    ///
    /// A block with a syntax error stays as written, and Biome still formats the rest of the file.
    /// The linter reports such errors when `markdown.analyzeEmbeds` is `true`.
    ///
    /// When this option is `false`, Biome keeps every block as written.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-formatter-format-embeds"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_embeds: Option<MarkdownFormatEmbeds>,
}

/// Options that change how the Markdown linter behaves
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct MarkdownLinterConfiguration {
    /// Control the linter for Markdown files.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("md-linter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<MarkdownLinterEnabled>,
}
