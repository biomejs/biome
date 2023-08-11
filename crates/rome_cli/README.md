# `rome_cli`

Rome's main binary distribution, exposes the command line interface defined in
this crate, and the language server interface defined in `rome_lsp` and used by
the `rome` VSCode extension

# Logs

When the server is run in daemon mode, it will output logs to a file created in
a `biome-logs` directory inside the system temporary directory. The log file
will be rotated on a hourly basis.
