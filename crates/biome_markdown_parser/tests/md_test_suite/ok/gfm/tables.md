| Left | Right |
| :--- | ---: |
| one  | two   |
| three |
| four | five | ignored |
plain cell

Code `a\|b` | escaped \| pipe
--- | ---
one | two

Escaped | Inline
--- | ---
f\|oo | b `\|` az
b **\|** im | plain

Reference | Value
--- | ---
[link][target] | resolved

[target]: /url

Unescaped `a|b` | is not a table
--- | ---

Center | Default
:---: | ---
center | default

No body | here
--- | ---

- Nested table

  | Left | Right |
  | --- | --- |
  | one | two |

Stop | here
--- | ---
> quote

not | a table
--- |

> Quoted | table
> --- | ---
> one | two

- List table

  A | B
  --- | ---
  one | two
outside | row

Definition | Value
--- | ---
body | row
[target]: /url

[target]
