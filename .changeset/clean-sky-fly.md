---
"@biomejs/biome": patch
---

Fixed [#4528](https://github.com/biomejs/biome/issues/4528): `biome migrate eslint` now correctly handles shared ESLint configuration that don't follow the ESLint naming convention.

ESLint recommends that a package that exports a shared configuration be prefixed with `eslint-config-` or simply named `eslint-config`.
This is only a recommendation.
Packages that export shared configurations can have arbitrary names.
Biome is now able to load any package.
