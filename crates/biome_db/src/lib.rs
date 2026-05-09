// Salsa's `#[salsa::input]` macro generates `use<...>` capture syntax that
// clippy flags as redundant. We cannot suppress it on the struct itself because
// the lint fires inside the macro expansion.
#![allow(impl_trait_redundant_captures)]

use camino::Utf8PathBuf;

#[salsa::db]
pub trait Db: salsa::Database {}

/// The very primitive of a file in the database.
#[salsa::input]
#[derive(Debug)]
pub struct SourceFile {
    #[returns(ref)]
    pub path: Utf8PathBuf,

    #[returns(ref)]
    pub content: String,
}
