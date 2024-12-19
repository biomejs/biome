<p align="center">
	<img alt="Biome - Toolchain of the web" width="400" src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg"/>
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![cargo version][cargo-badge]][cargo-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://biomejs.dev/chat
[cargo-badge]: https://badgen.net/crates/v/biome_js_parser?&color=green
[cargo-url]: https://crates.io/crates/biome_js_parser/

</div>

# `biome_js_parser`

Biome's JavaScript parser implementation. Follow the [documentation](https://docs.rs/biome_js_parser/).

## Testing

To update the `.rast` snapshots, run:

```bash
UPDATE_EXPECT=1 cargo test -p biome_js_parser
```
