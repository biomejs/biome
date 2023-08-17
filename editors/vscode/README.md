# Rome VS Code Extension

[Rome](https://biome.tools/) unifies your development stack by combining the functionality of separate tools. It uses a single configuration file, has fantastic performance, and works with any stack. This extension brings Rome to your editor so that you can:

- Format files *on save* or when issuing the *Format Document* command
- See lints while you type and apply code fixes
- Perform refactors

## Installation

You can install the code extension by heading to the extension's [Visual Studio Code Market Place page](https://marketplace.visualstudio.com/items?itemName=biomejs.biome) or from within VS Code by either:

- Open the *extensions* tab (_View_ → _Extensions)_ and search for Rome.
- Open the _Quick Open Overlay_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd>P</kbd> or _Go -> Go to File_), enter `ext install biomejs.biome`, and hit enter.

## Getting Started

### Default Formatter

Configure Rome as the default formatter for supported files to ensure that VS Code uses Rome over other formatters that you may have installed. You can do so by opening a JavaScript or TypeScript and then:

- Open the Command Palette (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd> or View → Command Palette)
- Select _Format Document With…_
- Select _Configure Default Formatter…_
- Select Rome

You can also enable Rome for specific languages only:

- [Open the `settings.json`](https://code.visualstudio.com/docs/getstarted/settings#_settingsjson): open the _Command Palette_(<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) and select _Preferences: Open User Settings (JSON)_
- And set the `editor.defaultFormatter` to `biomejs.biome` for the desired language

```json
{
	"editor.defaultFormatter": "<other formatter>",
	"[javascript]": {
		"editor.defaultFormatter": "biomejs.biome"
	}
}
```

This configuration sets Rome as the default formatter for JavaScript files. All other files will be formatted using `<other formatter>`

## Configuration Resolution

The extension automatically loads the `rome.json` file from the workspace’s root directory.

## Rome Resolution

The extension tries to use Rome from your project's local dependencies (`node_modules/rome`). We recommend adding Rome as a project dependency to ensure that NPM scripts and the extension use the same Rome version.

You can also explicitly specify the `rome` binary the extension should use by configuring the `biome.lspBin` setting in your editor options.

If the project has no dependency on Rome and no explicit path is configured, the extension uses the Rome version included in its bundle.

## Usage

### Format document

To format an entire document, open the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) and select _Format Document_.

To format a text range, select the text you want to format, open the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>), and select _Format Selection_.

### Format on save

Rome respects VS Code's _Format on Save_ setting. To enable format on save, open the settings (_File_ -> _Preferences_ -> _Settings_), search for `editor.formatOnSave`, and enable the option.

### Fix on save

Rome respects VS Code's _Code Actions On Save_ setting. To enable fix on save, add `"editor.codeActionsOnSave": { "quickfix.rome": true }` in vscode settings.json.

### Imports Sorting [Experimental]

The Rome VS Code extension has experimental support for imports sorting through the "Organize Imports" code action. By default this action can be run using the <kbd title="Shift">⇧</kbd>+<kbd>Alt</kbd>+<kbd>O</kbd> keyboard shortcut, or is accessible through the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) by selecting _Organize Imports_.

You can add the following to your editor configuration if you want the action to run automatically on save instead of calling it manually:

```json
{
	"editor.codeActionsOnSave":{
		"source.organizeImports.rome": true
	}
}
```

## Extension Settings

### `biome.lspBin`

The `biome.lspBin` option overrides the Rome binary used by the extension. The workspace folder is used as the base path if the path is relative.

### `biome.rename`

Enables Rome to handle renames in the workspace (experimental).

### `biome.requireConfiguration`

Disables formatting, linting, and syntax errors for projects without a `rome.json` file. Requires Rome 12 or newer.
Enabled by default.

## Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.
