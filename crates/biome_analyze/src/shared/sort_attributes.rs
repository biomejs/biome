use biome_rowan::{AstNode, Language, SyntaxToken, TriviaPieceKind};
use biome_string_case::StrLikeExtension;
use std::cmp::Ordering;

pub trait SortableAttribute {
    type Language: Language;

    fn name(&self) -> Option<SyntaxToken<Self::Language>>;

    fn node(&self) -> &impl AstNode<Language = Self::Language>;

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized;

    fn ascii_nat_cmp(&self, other: &Self) -> Ordering {
        match (self.name(), other.name()) {
            (Some(self_name), Some(other_name)) => self_name
                .text_trimmed()
                .ascii_nat_cmp(other_name.text_trimmed()),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }

    fn lexicographic_cmp(&self, other: &Self) -> Ordering {
        match (self.name(), other.name()) {
            (Some(self_name), Some(other_name)) => self_name
                .text_trimmed()
                .lexicographic_cmp(other_name.text_trimmed()),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

/// Returns the rank of an attribute according to the `sort_first` list.
///
/// Attributes whose name appears in `sort_first` get the index of that name,
/// so they sort before any attribute not present in the list (which all get
/// `sort_first.len()`). When `sort_first` is empty, every attribute gets rank
/// `0`, leaving the ordering entirely to the base comparator.
pub fn sort_first_rank<T: SortableAttribute>(attr: &T, sort_first: &[Box<str>]) -> usize {
    attr.name()
        .and_then(|name| {
            sort_first
                .iter()
                .position(|first| first.as_ref() == name.text_trimmed())
        })
        .unwrap_or(sort_first.len())
}

/// Compares two attributes, giving precedence to the `sort_first` list.
///
/// Attributes listed in `sort_first` are ordered first, in the order they
/// appear in the list. Ties (including the common case where neither attribute
/// is listed) fall back to `base`.
pub fn compare_with_sort_first<T, F>(a: &T, b: &T, sort_first: &[Box<str>], base: F) -> Ordering
where
    T: SortableAttribute,
    F: Fn(&T, &T) -> Ordering,
{
    sort_first_rank(a, sort_first)
        .cmp(&sort_first_rank(b, sort_first))
        .then_with(|| base(a, b))
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

    pub fn get_sorted_attributes<F>(&self, comparator: F) -> Option<Vec<T>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut attrs = self.attrs.clone();
        attrs.sort_by(comparator);

        let mut iter = attrs.iter_mut().peekable();

        while let Some(sorted_attr) = iter.next() {
            if iter.peek().is_some() {
                // Make sure sorted_attr has trailing whitespace if it is not the last attribute in the group
                let ends_in_whitespace = sorted_attr
                    .node()
                    .syntax()
                    .last_trailing_trivia()
                    .and_then(|last_trivia| last_trivia.last())
                    .is_some_and(|last| last.is_whitespace() || last.is_newline());

                let next_starts_with_whitespace = iter
                    .peek()
                    .and_then(|next_sorted_attr| {
                        next_sorted_attr.node().syntax().first_leading_trivia()
                    })
                    .and_then(|first_trivia| first_trivia.first())
                    .is_some_and(|first| first.is_whitespace() || first.is_newline());

                if !ends_in_whitespace && !next_starts_with_whitespace {
                    let old_last_token = sorted_attr.node().syntax().last_token().unwrap();
                    let new_last_token =
                        old_last_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

                    *sorted_attr = sorted_attr
                        .clone()
                        .replace_token(old_last_token, new_last_token)?;
                }
            }
        }

        Some(attrs)
    }

    pub fn clear(&mut self) {
        self.attrs.clear();
    }
}
