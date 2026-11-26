---
"@biomejs/biome": patch
---

Fixed [#13386](https://github.com/biomejs/biome/issues/13386): the Markdown parser no longer reports a false "Expected a list item" diagnostic when a list inside a block quote ends with a fenced code block or heading and a list item outside the quote follows a blank line.

````md
- > - ```text
  >   hello
  >   ```

- > - ```text
  >   there
  >   ```
````
