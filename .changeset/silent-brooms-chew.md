---
"@biomejs/biome": minor
---

Added support for monorepos. The feature will work *out of the box* for the majority of the users. If your project
has **nested configuration** files, use the command `biome migrate` from the *root of the project*.

Monorepo support in Biome is done in a single way. Create a `biome.json` at the root of the project. This configuration
file is now called the root configuration. Then, each nested configuration file must specify the new field `"root": false`.

We also introduced a new microsyntax for *extending a nested configuration from the root configuration*, which is `"extends": "//"`. This new syntax means “this config _extends_ from the root config”. When using this microsyntax, you **may omit** the `"root": false` field as it is implied.

Note that nested configs are not required to extend from the root config, and you can still have independent nested configs, as well as nested configs that extend from other files. In those cases, `"root": false` must be specified explicitly.
