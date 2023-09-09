# `biome_diagnostics_categories`

This crate contains a static registry of all the diagnostic categories used
throughout the Biome codebase

## Code Generation

The list of categories is defined in `src/categories.rs` using the
`define_dategories!` macro, but instead of relying on conventional Rust macro
expansion this crate instead uses a build script (in `build.rs`) to control how
the code resulting from the macro is generated.

Specifically this lets us generate new identifiers, which is something plain
Rust macros cannot do, without having to use full-blown procedural macros,
which would require creating and building yet another crate.
