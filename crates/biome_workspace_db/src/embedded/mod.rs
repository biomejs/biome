pub mod bindings;
pub mod references;
pub(crate) mod visitor;

#[salsa::db]
pub trait EmbeddedDb: biome_db::Db + biome_languages::LanguageDb {}
