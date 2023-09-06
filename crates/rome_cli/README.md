# `rome_cli`

The main binary distribution, exposes the command line interface defined in this crate,
and the language server interface defined in `biome_lsp` and used by the `biome` VSCode extension

# Logs

When the server is run in daemon mode,
it will output logs to a file created in the `biome-logs` directory inside the system temporary directory.
The log file will be rotated on a hourly basis.
