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
auto-format = true
comment-token = "//"
file-types = ["js", "mjs", "cjs"]
injection-regex = "(js|javascript)"
language-id = "javascript"
language-servers = ["typescript-language-server", "biome"]
roots = []
scope = "source.js"
shebangs = ["node"]

[language.formatter]
command = "biome"
args = ["format", "--stdin-file-path", "test.js"]

[language.indent]
tab-width = 2
unit = "  "

[[language]]
name = "typescript"
auto-format = true
file-types = ["ts", "mts", "cts"]
injection-regex = "(ts|typescript)"
language-id = "typescript"
language-servers = ["typescript-language-server", "biome"]
roots = []
scope = "source.ts"
shebangs = []

[language.formatter]
command = "biome"
args = ["format", "--stdin-file-path", "test.ts"]

[language.indent]
tab-width = 2
unit = "  "

[[language]]
name = "tsx"
auto-format = true
file-types = ["tsx"]
injection-regex = "(tsx)"
language-id = "typescriptreact"
language-servers = ["typescript-language-server", "biome"]
roots = []
scope = "source.tsx"

[language.formatter]
command = "biome"
args = ["format", "--stdin-file-path", "test.tsx"]

[language.indent]
tab-width = 2
unit = "  "

[[language]]
name = "jsx"
auto-format = true
comment-token = "//"
file-types = ["jsx"]
grammar = "javascript"
injection-regex = "jsx"
language-id = "javascriptreact"
language-servers = ["typescript-language-server", "biome"]
roots = []
scope = "source.jsx"

[language.formatter]
command = "biome"
args = ["format", "--stdin-file-path", "test.jsx"]

[language.indent]
tab-width = 2
unit = "  "

[[language]]
name = "json"
auto-format = true
file-types = ["json", "jsonc", "arb", "ipynb", "geojson"]
injection-regex = "json"
language-servers = ["biome"]
roots = []
scope = "source.json"

[language.formatter]
command = "biome"
args = ["format", "--stdin-file-path", "test.json"]
```

# Video record

## Code Action

https://user-images.githubusercontent.com/17974631/190205045-aeb86f87-1915-4d8b-8aad-2c046443ba83.mp4

## Formatting

https://user-images.githubusercontent.com/17974631/190205065-ddfde866-5f7c-4f53-8a62-b6cbb577982f.mp4
