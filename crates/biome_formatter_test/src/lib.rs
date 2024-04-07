use biome_formatter::{CstFormatContext, FormatLanguage, FormatResult, Formatted, Printed};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNode, TextRange};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::settings::ServiceLanguage;
use biome_service::settings::WorkspaceSettings;

pub mod check_reformat;
pub mod diff_report;
pub mod snapshot_builder;
pub mod spec;
pub mod test_prettier_snapshot;
pub mod utils;

pub trait TestFormatLanguage {
    type ServiceLanguage: ServiceLanguage + 'static;
    type Context: CstFormatContext<
        Options = <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
    >;
    type FormatLanguage: FormatLanguage<Context = Self::Context, SyntaxLanguage = Self::ServiceLanguage>
        + 'static;

    fn parse(&self, text: &str) -> AnyParse;

    fn to_language_settings<'a>(
        &self,
        settings: &'a WorkspaceSettings,
    ) -> &'a <Self::ServiceLanguage as ServiceLanguage>::FormatterSettings;

    fn format_node(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
    ) -> FormatResult<Formatted<Self::Context>>;

    fn format_range(
        &self,
        options: <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
        node: &SyntaxNode<Self::ServiceLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed>;

    fn default_options(&self) -> <Self::ServiceLanguage as ServiceLanguage>::FormatOptions;

    fn to_options(
        &self,
        settings: &WorkspaceSettings,
        file_source: &DocumentFileSource,
    ) -> <Self::ServiceLanguage as ServiceLanguage>::FormatOptions {
        let language_settings = self.to_language_settings(settings);
        Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &BiomePath::new(""),
            file_source,
        )
    }
}
