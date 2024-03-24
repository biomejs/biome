// language
pub mod language;
// runtimes
pub mod node;
pub mod web;

pub use language::is_global as is_language_global;
pub use node::is_global as is_node_global;
pub use web::is_global as is_web_global;

/// Returns `true` if `name` is a JavaScript global
///
/// ```
/// use biome_js_analyze::globals::javascript::is_global;
///
/// assert!(is_global(&"Math"));
/// ```
pub fn is_global(name: &str) -> bool {
    is_language_global(name) || is_web_global(name) || is_node_global(name)
}
