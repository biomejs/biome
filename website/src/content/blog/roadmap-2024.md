---
title: Roadmap 2024
description: Roadmap 2024 and new logo
summary: Roadmap 2024, new logo and homepage
authors:
  - ema
  - team
pubDate: 2024-01-08
coverImage:
  lightSrc: "@/assets/blog/roadmap-2024/banner-light.png"
  darkSrc: "@/assets/blog/roadmap-2024/banner-dark.png"
  alt: The Prettier challenge banner, with the Biome logo over it
socialImage: "@/assets/social-logo.png"
---

We are thrilled to share what the Core Contributors and Maintainers would like to focus on in 2024.

We want to remind you that Biome is a community-driven project, so we can only promise that some of the ideas outlined below will be shipped.

However, if you're excited about some aspects of the project, and you want to see some of them developed faster than others, you can help us in many ways:
- [**Be involved in the project and the community**](https://github.com/biomejs). Please help us to build those features.
- [**Sponsor us**](https://opencollective.com/biome). Ask your company to sponsor us. Biome is so fast that it can reduce your company's CI times and save money. Performance is part of our mission. Plus, sponsorship is a good medium of _advertisement_ for your company.
- [**Improve our documentation with ideas, recipes, or guides**](/guides/getting-started). Translate our documentation and help us to make Biome available to people who aren't proficient in English.


## Preface

The project is young and can't compete against giants such as Prettier, ESLint, Webpack, Vite, ESBuild, etc. However, the recent events (sponsors, bounty challenge, Biome being a fork of Rome) showed that the users **have** interest in the project, and we showed those users that we have the tools to fulfil a need.

Moving small projects from ESLint/Prettier is easy, but moving **big** code bases is challenging and time-consuming; this is a big friction point in Biome.

Users have different needs, though, so it will only be possible to satisfy some of them. We want to ensure that all features and contributions to our project [embrace our philosophy](/internals/philosophy/) and provide the best experience by default.

## Main area of focus

1. Help users to move to Biome
2. Expand Biome's language support so Biome tools can span more of the web ecosystem
3. Deepen Biome's existing capabilities to offer more functionalities
4. Plugins
5. Transformations
6. Community and content

## Help users to move to Biome

- Offer guides on our website to users who want to migrate from Prettier (CLI commands and configuration)
- Offer guides on our website to users who want to migrate from ESlint (CLI commands and configuration)
- Offer a section on our website that shows a mapping of the ESLint rules to our rules
- Offer commands to ease the transition
  - A command called `biome migrate prettier` that will read `.prettierrc` and `.prettierignore` will update the `biome.json` file (or create it) with the configuration from the Prettier files.
  - A command called `biome migrate eslint` will read the JSON configuration of Eslint and the ignore file. There will be expectations and limitations.


## Expand Biome's language support

CSS is our next language of focus, and we are making good progress. HTML and Markdown will follow. Follow our [up-to-date page](/internals/language-support) to keep up with the progress of our work.

The CSS language will enable much work and experimentation: CSS formatting and linting, and we will port some of the lint rules from `stylelint`. A new area of experimentation is cross-linting.

The idea of cross-linting can be explained with an example: compute the CSS styles/classes defined in a project and warn a user when said styles aren't used inside JSX/HTML files.

Plus, we unlock another area of experimentation, which is embedded formatting.

HTML and Markdown will be our next languages of focus. HTML will enable us to parse other variants of HTML that are popular in the frontend ecosystem: [Vue](https://vuejs.org/), [Svelte](https://svelte.dev/) and [Astro](https://astro.build/), and this would require some exploration of how to represent super languages of HTML.

## Deepen Biome's existing capabilities to offer more functionalities.

- Project analysis and dependency resolution
- Type system
- CLI

### Project analysis and dependency resolution

We will provide lint rules to read the manifest and detect errors such as invalid licenses.

With project resolution, we will be able to provide more lint rules, some of which will be able to detect unused modules.

With dependency resolution, we can - for example - detect dependencies that aren't used inside a project.

With this infrastructure, our LSP is going to be more powerful and provide more features, for example:
- rename variables across a project;
- auto-complete for imports;
- in-line types

### Type system

Building a full-fledged type system such as TypeScript is a massive effort; that's why we decided to take a different direction and start by building a subset of the type system that requires stricter typing. This approach would allow us to build some important lint rules that users have been asking for.

This will come with a downside: we will have to rely on a stricter code and minimal type inference from the compiler.

Once we have something we can rely on, we can slowly widen the capabilities of our type system.

### CLI

More features for the command line tool, such as:
- Add the `explain` command for offline documentation;
- Allow the output to be exported in different formats (JSON, etc.)
- Auto-completion for other shells such as `zsh`;
- Implement the `--modified` argument, which allows to format - for example - only the modified lines of a document;
- Expose metrics for Biome's operations and being able to track down possible performance bottlenecks;

## Plugins

We will explore plugins and come up with a design that fits Biome.
Biome is different from other tools because Biome is a toolchain that has multiple tools in it, so we have to think out of the box and propose a design that might differ from the tools people are used to.

We don't know yet what a Biome's plugin will look like, although a plugin should be able to tap all the tools that Biome offers.

Some ideas that we will consider:
- DSL
- WASM
- A Runtime

## Transformations

Transformations and code generation will be our first steps towards our compiler.

We will provide the ability to transform TypeScript and JSX files into JavaScript files.

## Community and content

Biome has a growing ecosystem, with an official VSCode extension, an official IntelliJ extension, and a Discord bot. We want to grow the features these tools provide and welcome anyone who wants to help us.

Our community is slowly growing, and we want to reward everyone who sticks around and contributes to Biome. At Biome, **we value every contribution**, so you don't need to be proficient in Rust to help us. Even participating in discussions and helping us to shape our features or helping other people are considered *contributions*. If you'd like to continue contributing to our ecosystem, we also encourage you to [nominate yourself as a maintainer of the project](https://github.com/biomejs/biome/blob/main/GOVERNANCE.md#maintainer-nomination).

Recently Biome started its own [YouTube Channel](https://www.youtube.com/channel/UC6ssscaFgCSlbv1Pb6krGVw). We will use this channel to share learning content with the community.

## New logo and homepage

With this blog post, we also want to officially announce our new logo, homepage and rebranding of the website.

With this new logo, we want to give a different meaning to the project. Biome **isn't** a fork of Rome anymore, but a self-sufficient project ready to bloom.

The triangle of the logo represents the mountains - **soil** -, and the curly shape on the bottom left represents a wave of the ocean - **water**. Two elements that are important in creating a self-sufficient ecosystem, so it can thrive and grow.
