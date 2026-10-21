//! The Salsa database shared by the Biome crates.
//!
//! A file enters the database as a [Source] input, which holds the text
//! of the file and the file source it's parsed as. Parsing is a capability of
//! the databases built on top of this one, see `biome_parser::db`.

pub mod testing;

use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

#[salsa::db]
pub trait Db: salsa::Database {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<Source>;
}

/// One content of the file at `path`, such as its content on disk or the
/// content of an editor. A path can have several of them.
#[salsa::input]
#[derive(Debug)]
pub struct Source {
    pub path: Utf8PathBuf,

    #[returns(ref)]
    pub text: Arc<str>,

    /// The index of the file source that the text is parsed as, in the table
    /// of file sources of the database.
    #[returns(copy)]
    pub file_source_index: usize,

    /// The index of the configuration that the client that published the
    /// text applies on top of the settings of the file, in the table of
    /// configurations of the database, if there is one.
    #[default]
    #[returns(copy)]
    pub configuration_index: Option<usize>,
}
