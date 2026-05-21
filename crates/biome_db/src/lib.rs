// Salsa's `#[salsa::input]` macro generates `use<...>` capture syntax that
// clippy flags as redundant. We cannot suppress it on the struct itself because
// the lint fires inside the macro expansion.
#![allow(impl_trait_redundant_captures)]

#[salsa::db]
pub trait Db: salsa::Database {}
