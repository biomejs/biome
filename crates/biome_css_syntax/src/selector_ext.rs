use crate::{CssComplexSelector, CssCompoundSelector};

impl CssComplexSelector {
    ///
    /// Computes the nesting level
    ///
    /// ## Example
    /// ```
    /// use biome_css_parser::CssParserOptions;
    /// use biome_css_syntax::CssComplexSelector;
    /// use biome_rowan::AstNode;
    /// use biome_rowan::SyntaxResult;
    ///
    /// # fn example() -> Option<()> {
    /// let source = ".a { .b { & & > p } }";
    /// let parsed = biome_css_parser::parse_css(source, CssParserOptions::default());
    /// let complex_selector = parsed.syntax().descendants().find_map(CssComplexSelector::cast)?;
    /// let nesting_level = complex_selector.nesting_level();
    /// assert_eq!(nesting_level, 2);
    /// # Some(())
    /// # }
    /// ```
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

        self.next = current.left().ok()?.as_css_complex_selector().cloned();
        current.right().ok()?.as_css_compound_selector().cloned()
    }
}
