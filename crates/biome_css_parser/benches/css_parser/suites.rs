use biome_languages::CssFileSource;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParserBenchmarkSuite {
    pub(crate) group_name: &'static str,
    pub(crate) libraries: &'static str,
    pub(crate) source_type: CssFileSource,
    pub(crate) require_all_libraries: bool,
    pub(crate) require_clean_parse: bool,
}

impl ParserBenchmarkSuite {
    pub(crate) fn handle_load_error(self, library: &str, error: &str) {
        if self.require_all_libraries {
            panic!(
                "failed to load {} benchmark fixture {library}: {error}",
                self.group_name
            );
        }
        println!("{error:?}");
    }
}

pub(crate) fn parser_benchmark_suites() -> [ParserBenchmarkSuite; 2] {
    [
        ParserBenchmarkSuite {
            group_name: "css_parser",
            libraries: include_str!("../libs-css.txt"),
            source_type: CssFileSource::css(),
            require_all_libraries: false,
            require_clean_parse: false,
        },
        ParserBenchmarkSuite {
            group_name: "scss_parser",
            libraries: include_str!("../libs-scss.txt"),
            source_type: CssFileSource::scss(),
            require_all_libraries: true,
            require_clean_parse: true,
        },
    ]
}
