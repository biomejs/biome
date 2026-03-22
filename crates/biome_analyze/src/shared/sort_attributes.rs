use biome_rowan::{Language, SyntaxResult, SyntaxToken};
use biome_string_case::StrLikeExtension;
use std::cmp::Ordering;

pub trait SortableAttribute {
    type Language: Language;

    fn name(&self) -> SyntaxResult<SyntaxToken<Self::Language>>;

    fn ascii_nat_cmp(&self, other: &Self) -> Ordering {
        let (Ok(self_name), Ok(other_name)) = (self.name(), other.name()) else {
            return Ordering::Equal;
        };

        self_name
            .text_trimmed()
            .ascii_nat_cmp(other_name.text_trimmed())
    }

    fn lexicographic_cmp(&self, other: &Self) -> Ordering {
        let (Ok(self_name), Ok(other_name)) = (self.name(), other.name()) else {
            return Ordering::Equal;
        };

        self_name
            .text_trimmed()
            .lexicographic_cmp(other_name.text_trimmed())
    }
}

#[derive(Clone)]
pub struct AttributeGroup<T: SortableAttribute + Clone> {
    pub attrs: Vec<T>,
}

impl<T: SortableAttribute + Clone> Default for AttributeGroup<T> {
    fn default() -> Self {
        Self { attrs: Vec::new() }
    }
}

impl<T: SortableAttribute + Clone> AttributeGroup<T> {
    pub fn is_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    pub fn is_sorted<F>(&self, comparator: F) -> bool
    where
        F: Fn(&T, &T) -> bool,
    {
        self.attrs.is_sorted_by(comparator)
    }

    pub fn get_sorted_attributes<F>(&self, comparator: F) -> Vec<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut new_attrs = self.attrs.clone();
        new_attrs.sort_by(comparator);
        new_attrs
    }

    pub fn clear(&mut self) {
        self.attrs.clear();
    }
}
