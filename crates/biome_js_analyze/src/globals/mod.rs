//! This module tracks all globals variables

pub mod javascript;
pub use javascript::is_global as is_js_global;
pub use javascript::is_language_global as is_js_language_global;
pub use javascript::is_node_global as is_js_node_global;
pub use javascript::is_web_global as is_js_web_global;

pub mod typescript;
pub use typescript::is_global as is_ts_global;
pub use typescript::is_language_global as is_ts_language_global;
pub use typescript::is_node_global as is_ts_node_global;
pub use typescript::is_web_global as is_ts_web_global;

pub mod module;
pub use module::is_node_builtin_module;
