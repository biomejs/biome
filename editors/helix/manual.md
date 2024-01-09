# Configuration

Currently, biome supports the following file extensions: `js`, `jsx`, `ts`, `tsx`, `d.ts`, `json` and `jsonc`.

Biome has an `lsp-proxy` command that acts as a server for the Language Server Protocol over stdin/stdout.

## Helix 23.10

Helix 23.10 has [support for multiple language servers](https://github.com/helix-editor/helix/pull/2507). Now you can use biome alongside `typescript-language-server`.

```toml
[language-server]
biome = { command = "biome", args = ["lsp-proxy"] }

[[language]]
name = "javascript"
language-servers = [ { name = "typescript-language-server", except-features = [ "format" ] }, "biome" ]
auto-format = true

[[language]]
name = "typescript"
language-servers = [ { name = "typescript-language-server", except-features = [ "format" ] }, "biome" ]
auto-format = true

[[language]]
name = "tsx"
auto-format = true
language-servers = [ { name = "typescript-language-server", except-features = [ "format" ] }, "biome" ]

[[language]]
name = "jsx"
auto-format = true
language-servers = [ { name = "typescript-language-server", except-features = [ "format" ] }, "biome" ]

[[language]]
name = "json"
language-servers = [ { name = "vscode-json-language-server", except-features = [ "format" ] }, "biome" ]
```

# Video record

## Code Action

https://user-images.githubusercontent.com/17974631/190205045-aeb86f87-1915-4d8b-8aad-2c046443ba83.mp4

## Formatting

https://user-images.githubusercontent.com/17974631/190205065-ddfde866-5f7c-4f53-8a62-b6cbb577982f.mp4
