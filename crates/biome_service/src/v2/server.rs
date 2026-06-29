//! Transitional v2 server type.
//!
//! The first owner/handle slice keeps using the existing workspace server
//! implementation. Later slices will move owner-only methods and canonical DB
//! ownership behind this module.

pub(crate) type WorkspaceServer = crate::workspace::WorkspaceServer;
