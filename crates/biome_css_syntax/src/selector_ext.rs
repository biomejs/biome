use crate::{CssComplexSelector, CssCompoundSelector};

impl CssComplexSelector {
    ///
    /// Computes the nesting level
    pub fn nesting_level(&self) -> usize {
        let iterator = CssComplexSelectorIterator::new(self.clone());
        iterator.count()
    }
}

pub struct CssComplexSelectorIterator {
    next: Option<CssComplexSelector>,
}

impl CssComplexSelectorIterator {
    fn new(complex_selector: CssComplexSelector) -> Self {
        Self {
            next: Some(complex_selector),
        }
    }
}

impl Iterator for CssComplexSelectorIterator {
    type Item = CssCompoundSelector;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;

        self.next = match current.left() {
            Ok(left) => left.as_css_complex_selector().cloned(),
            Err(_) => None,
        };
        current
            .right()
            .ok()
            .and_then(|r| r.as_css_compound_selector().cloned())
    }
}

#[cfg(test)]
mod tests {
    use biome_css_factory::syntax::CssComplexSelector;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_rowan::AstNode;

    #[test]
    fn test_nesting_level() {
        let source = "a { b { & & > p {} } }";
        let parsed = parse_css(source, CssParserOptions::default());
        let complex_selector = parsed
            .syntax()
            .descendants()
            .find_map(CssComplexSelector::cast)
            .unwrap();
        let nesting_level = complex_selector.nesting_level();
        assert_eq!(nesting_level, 2);
    }
}
