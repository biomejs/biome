---
title: Roadmap 2024
description: Roadmap 2024, v1.5.0 and new logo
summary: Roadmap 2024, v1.5.0 and new logo
authors:
  - ema
  - team
pubDate: 2024-01-06
coverImage:
  lightSrc: ../../assets/blog/roadmap-2024/banner-light.png
  darkSrc: ../../assets/blog/roadmap-2024/banner-dark.png
  alt: The Prettier challenge banner, with the Biome logo over it
socialImage: ../../assets/old-social-logo.png
---
## Roadmap 2024

We are thrilled to share what the Core Contributors and Maintainers would like to focus on in 2024.

We want to remind that Biome is a community-driven project, so we can't promise that all the ideas outlined below are going to be shipped.

However, if you're excited about some aspects of the project, and you want to see some of them developed faster than others, you can help us in many ways:
- [**Be involved in the project and the community**](https://github.com/biomejs). Help us to build those features.
- [**Sponsor us**](https://opencollective.com/biome). Ask your company to sponsor us. Biome is so fast that can cut down CI times in your company, and save money. Performance is part of our mission. Plus, sponsorship is a good medium of _advertisement_ for your company.
- [**Improve our documentation with ideas, recipes, or guides**](/guides/getting-started). Translate our documentation, help us to make Biome available to people who aren't proficient in English.


### Preface

The project is young and still can't compete against other giants such as Prettier, ESLint, Webpack, Vite, ESBuild, etc., although the recent events (sponsors, bounty challenge, Biome being a fork of Rome) showed that the users **have** interest in the project, and we showed those users that we have the tools for fulfilling a need.

Moving small projects from ESLint/Prettier is easy, but moving **big** code bases is challenging and time-consuming. This is a big point of friction in Biome.

Users have different needs though, so it will be impossible to satisfy all of them. We want to make sure that all features and contributions to our project [embrace our philosophy](/internals/philosophy/) and provide the best experience by default.

### Main area of focus

1. Help users to move to Biome
2. Expand Biome's language support, so Biome tools can span more of the web ecosystem
3. Deepen Biome's existing capabilities to offer more functionalities
4. Plugins
5. Transformations
6. Community and content

### Help users to move to Biome

- Offer guides on our website to users who want to migrate from Prettier (CLI commands and configuration)
- Offer guides on our website to users who want to migrate from ESlint (CLI commands and configuration)
- Offer a section on our website that shows a mapping of the ESLint rules to our rules
- Offer commands to ease the transition
  - A command called `biome migrate prettier` that will read `.prettierrc` and `.prettierignore`, will update the `biome.json` file (or create it) with the configuration coming from the Prettier files.
  - A command called `biome migrate eslint` that will read the JSON configuration of Eslint and the ignore file. There will be expectations and limitations.


### Expand Biome's language support

CSS is our next language of focus and is making good progress. HTML and Markdown will follow. Follow our [up-to-date page](/internals/language-support) to keep up with the progress of our work.

The CSS language will enable a lot of work and experimentation: CSS formatting and linting, and we will port some of the lint rules from `styelelint`. A new area of experimentation is cross-linting.

The idea of cross-linting can be explained with an example: compute the CSS styles/classes defined in a project, and warn a user when said styles aren't used inside JSX/HTML files.

Plus, we unlock another area of experimentation, which is embedded formatting.

HTML and Markdown will be our next languages of focus. HTML will enable us to parse other variants of HTML that are popular in the frontend ecosystem: [Vue](https://vuejs.org/), [Svelte](https://svelte.dev/) and [Astro](https://astro.build/). This would require some exploration on how to represent super languages of HTML.

### Deepen Biome's existing capabilities to offer more functionalities.

- Project analysis and dependency resolution
- Type system
- CLI

#### Project analysis and dependency resolution

We will provide lint rules that can read the manifest and do operations over it, such as invalid licenses.

With project resolution, we will be able to provide more lint rules, some of which will be able to detect unused modules.

With dependency resolution, we will be able to provide to - for example - detect dependencies that aren't used inside a project.

With this infrastructure, our LSP is going to be more powerful and provide more features, for example:
- rename variables across a project;
- auto-complete for imports;
- in-line types

#### Type system

Building a full-fledged type system such as TypeScript is a massive effort, that's why we decided to take a different direction and start by building a subset of the type system that would require stricter typing. This approach would allow us to build some important lint rules that users have been asking for.

This will come with a downside: relying on a stricter code, and minimal type inference from the compiler.

Once we have something we can rely on, we can slowly widen the type-checking functionality.

#### CLI

More features for the command line tool, such as:
- Add the `explain` command for offline documentation;
- Allow to export the output in different formats (JSON, etc.)
- Auto-completion for other shells such as `zsh`;
- Implement the `--modified` argument, which allows to format - for example - only the modified lines of a document;
- Expose metrics for Biome's operations, and being able to track down possible performance bottlenecks;

### Plugins

We will explore plugins, and come up with a design that fits Biome.
Biome is different from other tools because Biome is a toolchain that has multiple tools in it, so we have to think out of the box and propose a design that might differ from the tools people are used to.

We don't know yet what a Biome's plugin will look like, although we think a plugin should be able to tap all the tools that Biome offers.

Some ideas that we will consider:
- DSL
- WASM
- A Runtime

### Transformations

Transformations and code generation are going to be our first steps towards our compiler.

We will provide the ability to transform TypeScript and JSX files into JavaScript files.

### Community and content

Biome has a growing ecosystem, with an official VSCode extension, an official IntelliJ extension, and a Discord bot. We want to grow the features provided by these tools and welcome any user who wants to help us.

Our community is slowly growing, and we want to reward any person who sticks around and contributes to Biome. At Biome, **we value any type of contribution**, so you don't need to be proficient in Rust in order to help us. Even participating to discussions and help us to shape our features, or helping other people are considered *contributions*. If you'd like to continue contributing to our ecosystem, we also encourage you to [nominate yourself as a maintainer of the project](https://github.com/biomejs/biome/blob/main/GOVERNANCE.md#maintainer-nomination).

Recently Biome started its own [YouTube Channel](https://www.youtube.com/channel/UC6ssscaFgCSlbv1Pb6krGVw). We will use this channel to share learning content with the community.

## New logo and homepage

With this blog post, we also want to officially announce our new logo, homepage and rebranding of the website.

With this new logo, we want to give a different meaning to the project. Biome **isn't** a fork of Rome anymore, but a self-sufficient project ready to bloom.

The triangle of the logo represents the mountains - **soil** -, and the curly shape on the bottom left represents a wave of the ocean - **water**. Two elements important to create a self-sufficient ecosystem, so it can thrive and grow.


## Version 1.5.0

With the Roadmap 2024, we also publish a new version. This version has few features around the CLI and **many** fixes in our formatter. Our TypeScript, JSX and JavaScript formatting has surpassed the **97% compatibility rate** with Prettier.

### New features

- Process only the files that were changed.
- The command `biome ci` now prints diagnostics in GitHub PRs.
- A new command `biome explain`.
- The command `biome migrate` updates the `$schema`.
- New lint rules.

#### Process only the files that were changed

If you enable the integration with VCS, you can tell Biome to process only the files that were changed. As for now, this feature computes the files that were changed by using a VCS, so Biome doesn't know what's changed.

This feature practically makes some utilities such as `lint-staged` obsolete.

In order to take advantage of this feature, you have to tell Biome what's the default branch in the configuration file, then you'll have to pass the option `--changed` via CLI:

```json title="biome.json" ins={5}
{
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "defaultBranch": "main"
  }
}
```

Once you modified some files, use the new option to the command you need, for example the `format` command:

```shell ins="--changed"
biome format --changed --write
```

#### The command `biome ci` now prints diagnostics in GitHub PRs

For quite some time, users were confused by the difference of the commands `check` and `ci`, because until now their behaviours are very similar. From this version, the command `ci` is able to detect the GitHub CI environment, and print annotation in the PRs.

![Screenshot of a GitHub annotation printed by Biome](../../assets/blog/roadmap-2024/github-annotation.png)

It's possible that you would need to change in your permissions of your workflow files, in case you don't see the annotations:

```yaml title=".github/workflows/action.yml"
permissions:
  pull-requests: write
```

#### A new command `biome explain`

This command will serve as "offline" documentation tool. In this release, the command supports the explanation of all the lint rules, for example you can request documentation for `noAccumulatingSpread`:

```shell
biome explain noAccumulatingSpread
```

Which will print the following markdown:

``````markdown
# noAccumulatingSpread

No fix available.

This rule is recommended.

# Description
Disallow the use of spread (`...`) syntax on accumulators.

Spread syntax allows an iterable to be expanded into its individual elements.

Spread syntax should be avoided on accumulators (like those in `.reduce`)
because it causes a time complexity of `O(n^2)` instead of `O(n)`.

Source: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/

## Examples

### Invalid

```js,expect_diagnostic
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => [...acc, val], []);
```

```js,expect_diagnostic
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {return [...acc, val];}, []);
```

```js,expect_diagnostic
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => ({...acc, [val]: val}), {});
```

## Valid

```js
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {acc.push(val); return acc}, []);
```

``````

We plan to make this output more readable for terminals, as well as provide autocompletion for this command.

#### The command `biome migrate` updates the `$schema`

The command `biome migrate` now updates the `$schema` value inside the configuration file `biome.json` if you avail of the online schema. Run this command as soon as you update to Biome `v1.5.0`:

```json title="biome.json" ins="1.5.0" del={2} ins={3}
{
  "$schema": "https://biomejs.dev/schemas/1.4.1/schema.json"
  "$schema": "https://biomejs.dev/schemas/1.5.0/schema.json"
}
```

#### New rules

#####  [useExportType](https://biomejs.dev/linter/rules/use-export-type)

```ts
interface I {}
export { I };
```

<pre class="language-text"><code class="language-text">nursery/useExportType.js:2:8 <a href="https://biomejs.dev/linter/rules/use-export-type">lint/nursery/useExportType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">All exports are only types and should thus use </span><span style="color: Tomato;"><strong>export type</strong></span><span style="color: Tomato;">.</span>

    <strong>1 │ </strong>interface I {}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>export { I };
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;"> allows transpilers to safely drop exports of types without looking for their definition.</span>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a grouped </span><span style="color: lightgreen;"><strong>export type</strong></span><span style="color: lightgreen;">.</span>

<strong>  </strong><strong>  2 │ </strong>export<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>{<span style="opacity: 0.8;">·</span>I<span style="opacity: 0.8;">·</span>};
<strong>  </strong><strong>    │ </strong>       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>
</code></pre>

#####  [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention)

Enforces naming conventions for JavaScript and TypeScript filenames.

##### [useNodeImportProtocol](https://biomejs.dev/linter/rules/use-node-import-protocol)

```jsx
import fs from 'fs';
```


<pre class="language-text"><code class="language-text">nursery/useNodeImportProtocol.js:1:16 <a href="https://biomejs.dev/linter/rules/use-node-import-protocol">lint/nursery/useNodeImportProtocol</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Import from Node.js builtin module &quot;</span><span style="color: Orange;"><strong>fs</strong></span><span style="color: Orange;">&quot; should use the &quot;</span><span style="color: Orange;"><strong>node:</strong></span><span style="color: Orange;">&quot; protocol.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import fs from 'fs';
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using the </span><span style="color: lightgreen;"><strong>node:</strong></span><span style="color: lightgreen;"> protocol is more explicit and signals that the imported module belongs to Node.js.</span>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Change to &quot;node:fs&quot;.</span>

    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">s</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>'</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>:</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>

</code></pre>

##### [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules)

```jsx
import fs from "fs";
import path from "node:path";
```

<pre class="language-text"><code class="language-text">nursery/noNodejsModules.js:1:16 <a href="https://biomejs.dev/linter/rules/no-nodejs-modules">lint/nursery/noNodejsModules</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Using Node.js modules are forbidden.</span>

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import fs from &quot;fs&quot;;
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>import path from &quot;node:path&quot;;
    <strong>3 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Can be useful for client-side web projects that do not have access to those modules.</span>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the import module.</span>

</code></pre>

##### [noInvalidUseBeforeDeclaration](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration)

```jsx
function f() {
    console.log(x);
    const x;
}
```

<pre class="language-text"><code class="language-text">nursery/noInvalidUseBeforeDeclaration.js:3:11 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Const declarations must have an initialized value.</span>

    <strong>1 │ </strong>function f() {
    <strong>2 │ </strong>    console.log(x);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    const x;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>

<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This variable needs to be initialized.</span>

</code></pre>
