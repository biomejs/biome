#[cfg(test)]
mod tests {
    use biome_glimmer_parser::{GlimmerParseOptions, parse_glimmer};

    #[test]
    fn test_parse_empty() {
        let result = parse_glimmer("", GlimmerParseOptions::default());
        assert!(!result.has_errors());
    }

    #[test]
    fn test_parse_text() {
        let result = parse_glimmer("Hello World", GlimmerParseOptions::default());
        // For now, we just check it doesn't panic
        let _ = result.tree();
    }

    #[test]
    fn test_parse_mustache() {
        let result = parse_glimmer("{{value}}", GlimmerParseOptions::default());
        let _ = result.tree();
    }
}
