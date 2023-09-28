
[Biome](https://biomejs.dev/) is a powerful tool designed to enhance your development experience. This plugin integrates seamlessly with many [JetBrains IDE's](#Supported IDEs) to provide some capabilities:

-  See lints while you type
-  Apply code fixes (from mouse-over, <kbd title="Option">⌥</kbd>+<kbd  title="Enter">⏎</kbd> or <kbd title="Alt">Alt</kbd>+<kbd title="Enter">Enter</kbd>)
-  Reformat your code with <kbd>⌥⇧</kbd>+<kbd title="Cmd">⌘</kbd>+<kbd  title="L">L</kbd> or <kbd title="Ctrl">Ctrl</kbd>+<kbd title="Alt">Alt</kbd>+<kbd  title="L">L</kbd> (You can also format your [code on save](https://www.jetbrains.com/help/webstorm/reformat-and-rearrange-code.html#reformat-on-save))

However, please note the following limitations:

- Automatically applying code fixes on save

## Installation

To install the Biome IntelliJ Plugin, Head over to [official plugin page](https://plugins.jetbrains.com/plugin/22761-biome) or follow these steps:

### From JetBrains IDEs

1. Open IntelliJ IDEA.
2. Go to **Settings/Preferences**.
3. Select **Plugins** from the left-hand menu.
4. Click on the **Marketplace** tab.
5. Search for "Biome" and click **Install**.
6. Restart the IDE to activate the plugin.

### From disk

1. Download the plugin .zip from releases tab.
2. Press `⌘Сmd,` to open the IDE settings and then select Plugins.
3. On the Plugins page, click The Settings button and then click Install Plugin from Disk….

## Biome Resolution

The Plugin tries to use Biome from your project’s local dependencies (`node_modules/.bin/biome`). We recommend adding Biome as a project dependency to ensure that NPM scripts and the extension use the same Biome version.

You can also explicitly specify the `Biome` binary the extension should use by configuring the `Biome CLI Path` in `Settings`->`Language & Frameworks`->`Biome Settings`.

## Plugin settings

### `Biome CLI Path`

This setting overrides the Biome binary used by the plugin.

## Supported IDEs

This plugin is currently supported in the following IDEs:

- IntelliJ IDEA Ultimate >2023.2.2
- WebStorm >2023.2.2
- AppCode >2023.2.2
- PhpStorm >2023.2.2
- RubyMine >2023.2.2

## Contributing

We welcome contributions to the Biome IntelliJ Plugin. If you encounter any issues or have suggestions for improvements, please open an issue on our [GitHub repository](https://github.com/biomejs/biome/issues/new/choose). We also have a [Discord community](https://discord.gg/BypW39g6Yc) where you can discuss the plugin, ask questions, and connect with other Biome's developers.
