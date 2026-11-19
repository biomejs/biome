# ARIA data

This package provides a script that extracts data from an ARIA specification.
This is a best effort approach because the ARIA specifications are in a semi-structured representation.

Generated data consumed by `biome_aria_metadata` is stored in that crate's `aria-data` directory.
Run these commands from this package to update the files:

```shell
node generate-aria-data.js wai-aria-1.3 >| ../../crates/biome_aria_metadata/aria-data/wai-aria-1-3.json
node generate-aria-data.js graphics-aria-1.0 >| ../../crates/biome_aria_metadata/aria-data/graphics-aria-1-0.json
node generate-aria-data.js dpub-aria-1.1 >| ../../crates/biome_aria_metadata/aria-data/dpub-aria-1-1.json
```
