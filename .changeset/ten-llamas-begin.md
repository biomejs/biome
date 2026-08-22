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
Very tiny line with, so
the paragraph is more
compact.
```

##### Markdown linter

The linter ships with a few rules inspired by [markdownlint](https://github.com/markdownlint/markdownlint). We plan to ship more rules in the upcoming releases.

Some rules you can start using already:
- [`useTopLevelHeading`](https://biomejs.dev/linter/rules/use-top-level-heading/)
- [`useConsistentHeadingLevel`](https://biomejs.dev/linter/rules/use-consistent-heading-level/)

##### Markdown snippets

Thanks to Biome capabilities, elements such as frontmatter, inline HTML and fenced code blocks are recognized as embedded
languages, which means they are analyzed and formatted using your configuration.

Given the following Markdown document, when you run `biome lint`, Biome will emit a parsing diagnostic for
the JavaScript snippet, and a lint diagnostic for the CSS snippet:

````md
# Embeds

```js
function () {}
```

```css
a {
  color: red;
  color: blue;
}
```
````

```text
file.md:4:10 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × expected a name for the function in a function declaration, but found none

     3 │ ```js
   > 4 │ function () {}
       │          ^
     5 │ ```
     6 │
```

```text
file.md:10:3 lint/suspicious/noDuplicateProperties ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally.

     8 │ a {
     9 │   color: red;
  > 10 │   color: blue;
       │   ^^^^^
    11 │ }
    12 │ ```

  i color is already defined here.

     7 │ ```css
     8 │ a {
   > 9 │   color: red;
       │   ^^^^^
    10 │   color: blue;
    11 │ }

  i Remove or rename the duplicate property to ensure consistent styling.
```
