---
"@biomejs/biome": minor
---

Added support for monorepos. The feature will work *out of the box* for the majority of the users. If your project
has **nested configuration** files, use the command `biome migrate` from the *root of the project*.

Monorepo support in Biome is one single way. Create a `biome.json` at the root of the project. This configuration
file is now called root configuration. Then, each nested configuration file must specify the new filed `"root": false`.
Failing to do so will throw an error.

We also introduced a new microsyntax for *extending a nested configuration from the root configuration*, which is `"extends": "//"`.
