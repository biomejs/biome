# `biome_markup`

The crate contains procedural macros to build `rome_console` markup object with a JSX-like syntax

The macro cannot be used alone as it generates code that requires supporting types declared in the
`rome_console` crate, so it's re-exported from there and should be used as `rome_console::markup`

## Local installation

```toml
biome_markup = { version = "0.0.0", path = "../biome_markup" }
```
