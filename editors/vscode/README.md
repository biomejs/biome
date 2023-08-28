# Biome VS Code Extension

[Biome](https://biomejs.dev/) unifies your development stack by combining the functionality of separate tools. It uses a single configuration file, has fantastic performance, and works with any stack. This extension brings Biome to your editor so that you can:

- Format files *on save* or when issuing the *Format Document* command
- See lints while you type and apply code fixes
- Perform refactors

## Installation

You can install the code extension by heading to the extension's [Visual Studio Code Market Place page](https://marketplace.visualstudio.com/items?itemName=biomejs.biome) or from within VS Code by either:

- Open the *extensions* tab (_View_ → _Extensions)_ and search for Biome.
- Open the _Quick Open Overlay_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd>P</kbd> or _Go -> Go to File_), enter `ext install biomejs.biome`, and hit enter.

## Getting Started

### Default Formatter

Configure Biome as the default formatter for supported files to ensure that VS Code uses Biome over other formatters that you may have installed. You can do so by opening a JavaScript or TypeScript and then:

- Open the Command Palette (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd> or View → Command Palette)
- Select _Format Document With…_
- Select _Configure Default Formatter…_
- Select Biome

You can also enable Biome for specific languages only:

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

This configuration sets Biome as the default formatter for JavaScript files. All other files will be formatted using `<other formatter>`

## Configuration Resolution

The extension automatically loads the `Biome.json` file from the workspace’s root directory.

## Biome Resolution

The extension tries to use Biome from your project's local dependencies (`node_modules/Biome`). We recommend adding Biome as a project dependency to ensure that NPM scripts and the extension use the same Biome version.

You can also explicitly specify the `Biome` binary the extension should use by configuring the `biome.lspBin` setting in your editor options.

If the project has no dependency on Biome and no explicit path is configured, the extension uses the Biome version included in its bundle.

## Usage

### Format document

To format an entire document, open the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) and select _Format Document_.

To format a text range, select the text you want to format, open the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>), and select _Format Selection_.

### Format on save

Biome respects VS Code's _Format on Save_ setting. To enable format on save, open the settings (_File_ -> _Preferences_ -> _Settings_), search for `editor.formatOnSave`, and enable the option.

### Fix on save

Biome respects VS Code's _Code Actions On Save_ setting. To enable fix on save, add


```json
{
  "editor.codeActionsOnSave": {
    "quickfix.biome": true
  }
}
```

in vscode `settings.json`.

### Imports Sorting [Experimental]

The Biome VS Code extension supports imports sorting through the "Organize Imports" code action. By default this action can be run using the <kbd title="Shift">⇧</kbd>+<kbd>Alt</kbd>+<kbd>O</kbd> keyboard shortcut, or is accessible through the _Command Palette_ (<kbd>Ctrl</kbd>/<kbd title="Cmd">⌘</kbd>+<kbd title="Shift">⇧</kbd>+<kbd>P</kbd>) by selecting _Organize Imports_.

You can add the following to your editor configuration if you want the action to run automatically on save instead of calling it manually:

```json
{
	"editor.codeActionsOnSave":{
		"source.organizeImports.biome": true
	}
}
```

## Extension Settings

### `biome.lspBin`

The `biome.lspBin` option overrides the Biome binary used by the extension.
The workspace folder is used as the base path if the path is relative.

### `biome.rename`

Enables Biome to handle renames in the workspace (experimental).

### `biome.requireConfiguration`

Disables formatting, linting, and syntax errors for projects without a `biome.json` file.
Enabled by default.

## Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.


## Troubleshooting

> I installed `@biomejs/biome`, but the extension shows a warning saying that it could not resolve library.

The library `@biomejs/biome` specifies some optional dependencies that are installed based on your OS and architecture.

It's possible though, that the extension can't resolve the binary when loading the extension. This is caused - probably - by your package manager.

**To resolve the issue**, try to install the binary manually. The warning should show you the binary that belongs to your machine.

**If you work in a team that use different OSs/architectures**, it's advised to install all the binaries:

- `@biomejs/cli-darwin-arm64`
- `@biomejs/cli-darwin-x64`
- `@biomejs/cli-linux-arm64`
- `@biomejs/cli-linux-x64`
- `@biomejs/cli-win32-arm64`
- `@biomejs/cli-win32-x64`
