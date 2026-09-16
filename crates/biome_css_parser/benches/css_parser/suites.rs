use biome_languages::CssFileSource;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParserBenchmarkSuite {
    pub(crate) group_name: &'static str,
    pub(crate) libraries: &'static str,
    pub(crate) source_type: CssFileSource,
    pub(crate) require_clean_parse: bool,
}

pub(crate) fn parser_benchmark_suites() -> [ParserBenchmarkSuite; 2] {
    [
        ParserBenchmarkSuite {
            group_name: "css_parser",
            libraries: include_str!("../libs-css.txt"),
            source_type: CssFileSource::css(),
            require_clean_parse: false,
        },
        ParserBenchmarkSuite {
            group_name: "scss_parser",
            libraries: include_str!("../libs-scss.txt"),
            source_type: CssFileSource::scss(),
            require_clean_parse: true,
        },
    ]
}
