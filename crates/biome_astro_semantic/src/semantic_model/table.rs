//! Rows of the semantic model addressed by typed ids.
//!
//! Each table accepts only its own id type, so a `BlockId` cannot read a scope
//! row by accident.

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// An id that addresses one row of a [`Table`].
pub(crate) trait TableId: Copy + fmt::Debug {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;
}

/// The rows of one kind of model value.
///
/// An id returned by [`Table::push`] addresses its row until the rows are
/// reordered through the slice API.
///
/// # Panics
///
/// Indexing panics if the id does not address a row of this table, such as an
/// id issued by another model. Debug builds report the id in the message.
#[derive(Eq, PartialEq)]
pub(crate) struct Table<I, T> {
    rows: Vec<T>,
    id: PhantomData<I>,
}

impl<I: TableId, T> Table<I, T> {
    /// Appends `row` and returns the id that addresses it.
    pub(crate) fn push(&mut self, row: T) -> I {
        let id = I::from_index(self.rows.len());
        self.rows.push(row);
        id
    }
}

impl<I, T> Default for Table<I, T> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            id: PhantomData,
        }
    }
}

impl<I, T: fmt::Debug> fmt::Debug for Table<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rows.fmt(f)
    }
}

impl<I, T> Deref for Table<I, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.rows
    }
}

impl<I, T> DerefMut for Table<I, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.rows
    }
}

impl<I: TableId, T> Index<I> for Table<I, T> {
    type Output = T;

    fn index(&self, id: I) -> &T {
        debug_assert!(
            id.to_index() < self.rows.len(),
            "{id:?} does not address a row of a table with {} rows",
            self.rows.len()
        );
        &self.rows[id.to_index()]
    }
}

impl<I: TableId, T> IndexMut<I> for Table<I, T> {
    fn index_mut(&mut self, id: I) -> &mut T {
        debug_assert!(
            id.to_index() < self.rows.len(),
            "{id:?} does not address a row of a table with {} rows",
            self.rows.len()
        );
        &mut self.rows[id.to_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::{Table, TableId};

    #[derive(Clone, Copy, Debug)]
    struct RowId(usize);

    impl TableId for RowId {
        fn from_index(index: usize) -> Self {
            Self(index)
        }

        fn to_index(self) -> usize {
            self.0
        }
    }

    #[test]
    fn push_returns_the_id_of_the_new_row() {
        let mut table = Table::<RowId, &str>::default();
        let first = table.push("first");
        let second = table.push("second");

        table[second] = "updated";

        assert_eq!(table[first], "first");
        assert_eq!(table[second], "updated");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "RowId(1) does not address a row of a table with 1 rows")]
    fn foreign_id_panics_with_the_id_in_debug_builds() {
        let mut table = Table::<RowId, &str>::default();
        table.push("only");

        let _ = table[RowId(1)];
    }
}
