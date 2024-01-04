# biome_ungrammar

Biome's fork of rust-analyzer's [Ungrammar](https://github.com/rust-analyzer/ungrammar).

This fork adds support for new syntax features, including:

- double-pipe (`||`) combinator for specifying "one or more of the alternatives, in any order"
- double-amp (`&&`) combinator for specifying "all of the alternatives, in any order"
- parsing documentation comments to attach to nodes and rules.