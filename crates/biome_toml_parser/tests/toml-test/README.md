The files in this directory are the TOML 1.1.0 cases of
[toml-test](https://github.com/toml-lang/toml-test) v2.2.0, listed in
`files-toml-1.1.0`, without the `.json` files that list the decoded values of the
valid documents. They are distributed under the MIT license in `LICENSE`.

Files under `valid/` must parse without diagnostics. Files under `invalid/` must
produce at least one diagnostic.
