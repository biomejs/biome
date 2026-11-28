---
"@biomejs/biome": patch
---

Fixed [#13408](https://github.com/biomejs/biome/issues/13408): the Markdown parser now keeps the content of a fenced code block in a list item inside a block quote when a tab follows the `>` marker. Previously the code block ended after its opening fence, and the formatter moved the code out of the block.

````md
>	- ```text
>	  hello
>	  ```
````
