---
"@biomejs/biome": minor
---

Added the `delimiterSpacing` formatter option. This option inserts spaces inside delimiters (after the opening delimiter and before the closing delimiter) when the content fits on a single line. Empty delimiters are not affected, and no space is added before the opening delimiter. The specific delimiters affected depend on the language. It can be configured globally via `formatter.delimiterSpacing` or per-language via `javascript.formatter.delimiterSpacing`, `json.formatter.delimiterSpacing`, and `css.formatter.delimiterSpacing`. Defaults to `false`.

```diff
- callFn(foo)
+ callFn( foo )
```

```diff
- const arr = [1, 2, 3];
+ const arr = [ 1, 2, 3 ];
```

##### JavaScript

When enabled, Biome inserts spaces inside parentheses (e.g., `foo( a, b )`), square brackets (e.g., `[ a, b ]`), template literal interpolations (e.g., `${ expr }`), and the logical NOT operator (e.g., `! x`, but in chains only after the last one: `!! x`). Only applies when the content fits on a single line. Empty delimiters and the space before the opening delimiter are not affected.

```diff
- if (condition) {}
+ if ( condition ) {}
```

```diff
- `Hello ${name}!`
+ `Hello ${ name }!`
```

##### JSX

When enabled, Biome inserts spaces inside JSX expression braces (e.g., `attr={ value }`) and spread attributes (e.g., `{ ...props }`). Only applies when the content fits on a single line. Empty delimiters are not affected.

```diff
- <Foo bar={value} />
+ <Foo bar={ value } />
```

##### TypeScript

When enabled, Biome inserts spaces inside TypeScript angle brackets (e.g., `foo< T >()`), indexed access types (e.g., `T[ K ]`), mapped types, tuple types, type parameters, and index signatures. Only applies when the content fits on a single line. Empty delimiters are not affected.

```diff
- type Result = Map<string, number>;
+ type Result = Map< string, number >;
```

##### JSON

When enabled, Biome inserts spaces inside square brackets when the content fits on a single line. Empty brackets are not affected.

```diff
- [1, 2, 3]
+ [ 1, 2, 3 ]
```

##### CSS

When enabled, Biome inserts spaces inside parentheses and square brackets when the content fits on a single line. Empty delimiters are not affected.

```diff
- rgba(0, 0, 0, 1)
+ rgba( 0, 0, 0, 1 )
```

```diff
- [data-attr]
+ [ data-attr ]
```
