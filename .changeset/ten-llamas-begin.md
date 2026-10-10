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

The parser also comes with an opt-in option called `cjkFriendlyEmphasis`. When enabled, the parser follows the [CommonMark CJK-friendly amendments](https://github.com/tats-u/markdown-cjk-friendly/blob/main/specification.md) and recognizes emphasis markers, such as `**`, placed directly next to Chinese, Japanese, or Korean text. Enable it only when the tool that renders your Markdown supports the same amendments, because the option changes which text Biome formats and lints as emphasis:

```json5
// biome.json
{
  "markdown": {
    "parser": {
      "cjkFriendlyEmphasis": true
    }
  }
}
```

With the option enabled, the following text is parsed as bold, instead of as literal `**`:

```md
**このアスタリスクは強調記号として認識されず、そのまま表示されます。**この文のせいで。
```

##### Markdown formatter

The formatter has great compatibility with Prettier formatting, more than 90% detected by our infrastructure. The formatter ships with
a new option called `proseWrap`, that allows controlling how the Biome formatter should wrap paragraphs.

For example, when `proseWrap` is set to `always`, the formatter will wrap the paragraph to match the configured `lineWidth`:

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

##### Markdown snippets

Biome recognizes the code inside fenced code blocks, frontmatter, and HTML blocks as embedded languages, and formats it with
the formatter of its language and your configuration:

````diff
 ```js
-const   value   =   {a:1,b:2}
+const value = { a: 1, b: 2 };
 ```
````

HTML blocks are formatted only when the HTML formatter is enabled. A block with a syntax error stays as written, and Biome
still formats the rest of the file.

The option `markdown.formatter.formatEmbeds` controls this behavior, and it's enabled by default. Set it to `false` to keep
the code of these blocks as written:

```json5
// biome.json
{
  "markdown": {
    "formatter": {
      "formatEmbeds": false
    }
  }
}
```

The option `markdown.analyzeEmbeds` runs the linter and assist actions on the same blocks, with the rules and actions of
their language, and reports their syntax errors. It's disabled by default, because fenced code blocks usually contain
examples or partial code:

```json5
// biome.json
{
  "markdown": {
    "analyzeEmbeds": true
  }
}
```
