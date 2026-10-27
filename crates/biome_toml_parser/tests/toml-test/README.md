The files in this directory are the TOML 1.1.0 cases of
[toml-test](https://github.com/toml-lang/toml-test) v2.2.0, listed in
`files-toml-1.1.0`. They are distributed under the MIT license in `LICENSE`.

Files under `valid/` must parse without diagnostics, and their decoded values
must match the sibling `.json` file. Files under `invalid/` must produce at least
one diagnostic.
