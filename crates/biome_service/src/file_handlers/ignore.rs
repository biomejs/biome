use crate::file_handlers::ExtensionHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct IgnoreFileHandler;

impl ExtensionHandler for IgnoreFileHandler {}
