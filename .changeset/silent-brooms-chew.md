---
"@biomejs/biome": minor
---

Added support for monorepos. The feature will work *out of the box* for the majority of the users. If your project
has **nested configuration** files, use the command `biome migrate` from the *root of the project*.

Monorepo support in Biome is one single way. Create a `biome.json` at the root of the project. This configuration
file is now called root configuration. Then, each nested configuration file must specify the new field `"root": false`, although you may do so implicitly by specifying `"extends": "//"`.
Failing to do so will throw an error. If neither field is set, an error is thrown.

We also introduced a new microsyntax for *extending a nested configuration from the root configuration*, which is `"extends": "//"`.

`"extends": "//"` is a new microsyntax, meaning “this config _extends_ from the root config”. 

Note that nested configs are not required to extend from the root config, and you can still have independent nested configs, as well as nested configs that extend from other files. In those cases, `"root": false` must be specified explicitly.
