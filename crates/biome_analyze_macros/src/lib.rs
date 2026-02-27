use proc_macro::TokenStream;

mod group_macro;

/// Declares an analyzer group by reading rule files from the filesystem.
///
/// This macro scans the filesystem at compile time to discover all rule files
/// in a directory and generates the necessary module declarations and group
/// registration code.
///
/// # Example
///
/// ```ignore
/// use biome_analyze_macros::declare_group_from_fs;
///
/// declare_group_from_fs! {
///     category: "lint",
///     group: "nursery"
/// }
/// ```
///
/// This will:
/// 1. Scan the nursery/ subdirectory for .rs files
/// 2. Generate pub mod declarations for each rule
/// 3. Generate a declare_lint_group! invocation with all discovered rules
#[proc_macro]
pub fn declare_group_from_fs(input: TokenStream) -> TokenStream {
    group_macro::declare_group_from_fs_impl(input)
}
