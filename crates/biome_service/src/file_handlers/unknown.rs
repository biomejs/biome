use super::ExtensionHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct UnknownFileHandler {}

impl ExtensionHandler for UnknownFileHandler {
    fn mime(&self) -> super::Mime {
        super::Mime::Text
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}
