---
"@biomejs/biome": minor
---

#### Markdown is now supported

Added support for linting and formatting Markdown files. The first version of Biome Markdown parser has 100% compatibility
against CommonMark.

Other variants of Markdown, such as GitHub Flavored Markdown (GFM) aren't currently supported; however, we plan to support them soon.

Formatter and Linter are automatically enabled by default.

##### Markdown parser

The parser comes with an opt-in option to enable frontmatter parsing. When enabled, the first thematic break `---` is considered the opening fence of the frontmatter:

```json5
// biome.json
{
  "markdown": {
    "parser": {
      "frontmatter": true
    }
  }
}
```

````md
---
title: Lorem ipsum
---
````

##### Markdown formatter

The formatter has great compatibility with Prettier formatting, more than 90% detected by our infrastructure. The formatter ships with
a new option called `proseWrap`, that allows controlling how the Biome formatter should wrap paragraphs.

For example, when `proseWrap` is set to `always`, the formatter will wrap the paragraph to match the conown::markdown_formatter_configurfigured `lineWidth`:

```json5
// biome.json
{
  "markdown": {
    "formatter": {
      "lineWidth": 20,
      "proseWrap": "always"
    }
  }
}
```

```md
Very tiny line width, so
the paragraph is more
compact.
```

##### Markdown linter

The linter ships with a few rules inspired by [markdownlint](https://github.com/markdownlint/markdownlint). We plan to ship more rules in the upcoming releases.

Some rules you can start using already:
- [`useTopLevelHeading`](https://biomejs.dev/linter/rules/use-top-level-heading/)
- [`useConsistentHeadingLevel`](https://biomejs.dev/linter/rules/use-consistent-heading-level/)
