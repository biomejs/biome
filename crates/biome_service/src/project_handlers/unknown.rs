use crate::project_handlers::ProjectHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct UnknownProjectHandler {}

impl ProjectHandler for UnknownProjectHandler {}
