use super::ExtensionHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct UnknownFileHandler {}

impl ExtensionHandler for UnknownFileHandler {}
