# `biome_cli`

The main binary distribution, exposes the command line interface defined in this crate,
and the language server interface defined in `biome_lsp` and used by the `biome` VSCode extension

## Logs

When the server is run in daemon mode,
it will output logs to a file created in the `biome-logs` directory inside the biome cache directory.
This directory is typically `~/.cache/biome` on Linux,
`C:\Users<UserName>\AppData\Local\biomejs\biome\cache` on Windows,
`/Users/<UserName>/Library/Caches/dev.biomejs.biome` on macOS,
and the system's temporary directory on other platforms.
To obtain the precise path, execute the command `biome __print_cache_dir`.
The log file will be rotated on a hourly basis.
