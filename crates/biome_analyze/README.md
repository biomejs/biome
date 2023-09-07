# Biome Analyze

## This entire crate is a WIP with a clunky and unstable API. 

The data structures and methods are poorly designed/named and will certainly change.

- `analyzers` produce diagnostics and fixes for an entire file and are relevant both for LSP and CLI use.
- `assists` produce code actions related to cursor position and are only relevant for LSP.