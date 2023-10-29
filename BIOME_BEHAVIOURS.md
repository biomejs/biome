# Configuration 

The configuration file is considered **optional**, Biome has good defaults. Use the configuration 
file to change those defaults. 

The Biome configuration file is named `biome.json` and should be placed in the root directory of your project. The root 
directory is usually the directory containing your project's `package.json`. 

Here's an example: 

<CodeBlockHeader filename="biome.json" />

## Note:
The project configuration files like `package.json`, `package-lock.json`, `tsconfig.json`, `jsconfig.json` are currently ignored by Biome. This means that no diagnostics will be ever emitted by Biome for those files.

# Known Files
Here is a list of some known files:
- `typescript.json`
<!-- - `tsconfig.json`
- `jsconfig.json` -->
- `tslint.json`
- `babel.config.json`
- `.babelrc.json`
- `.ember-cli`
- `typedoc.json`
- `.eslintrc`
- `.eslintrc.json`
- `.jsfmtrc`
- `.jshintrc`
- `.swcrc`
- `.hintrc`
- `.babelrc`

It is to note that when Biome analyses these files, they are parsed as `JSON files` with `json.parser.allowComments` and `json.parser.allowTrailingCommas` set to `true`. This is because editor tools like VSCode treat them like this.