# Challenge report

Challenge: https://console.algora.io/challenges/prettier

We provide two compatibility reports for the challenge:

- [report-es215](./report-es2015.md) that takes only ES2015 syntaxes into account;
- [report-es2024+](./report-es2024+.md) that takes ES2024 syntaxes into account and widely supported experimental syntaxes (decorators, import assertions, import attributes, explicit resource management).

You can test Biome directly on the [playground](https://biomejs.dev/playground),
or using our [latest nightly release](https://github.com/biomejs/biome/releases/tag/cli%2Fv1.3.3-nightly.ced82da).

```sh
npm install -D @biomejs/biome@1.3.3-nightly.ced82da
```

Please find more details in our [Getting Started guide](https://biomejs.dev/guides/getting-started/).


## Test case notes

### Ignored test cases

[report-es215](./report-es2015.md) and  [report-es2024+](./report-es2024+.md) ignore the following test cases:

- JSX
  - `js/binary-expressions/inline-jsx.js`
  - `js/binary-expressions/jsx_parent.js`
  - `js/call/first-argument-expansion/jsx.js`
  - `js/comments-closure-typecast/styled-components.js`
  - `js/comments/html-like/`
  - `js/comments/jsx.js`
  - `js/comments/return-statement.js`
  - `js/last-argument-expansion/jsx.js`
  - `js/trailing-comma/jsx.js`
  - `js/throw_statement/jsx.js`
  - `js/unicode/nbsp-jsx.js`
  - `js/yield/jsx-without-parenthesis.js`
  - `js/yield/jsx.js`

- Embedded language formatting inside template literals
  - `js/multiparser-comments/`
  - `js/multiparser-css/`
  - `js/multiparser-graphql/`
  - `js/multiparser-html/`
  - `js/multiparser-invalid/`
  - `js/multiparser-markdown/`
  - `js/multiparser-text/`
  - `js/template-literals/css-prop.js`
  - `js/template-literals/styled-components-with-expressions.js`
  - `js/template-literals/styled-jsx-with-expressions.js`
  - `js/template-literals/styled-jsx.js`
  - `js/range/issue-7082.js`
  - `js/last-argument-expansion/embed.js`
  - `js/last-argument-expansion/embed.js`

- Non-standard and experimental syntaxes
  - `js/v8_intrinsic`
  - `js/babel-plugins/`
  - `js/async-do-expressions/`
  - `js/do/`
  - `export X from "mod"`
    - `js/export-default/export-default-from/`
    - `js/export-default/escaped/default-escaped.js`
    - `module <id> {}`
      - `js/module-blocks`
      - `js/explicit-resource-management/valid-module-block-top-level-await-using-binding.js`
      - `js/explicit-resource-management/valid-module-block-top-level-using-binding.js`
    - `#[]` and `#{}`
      - `js/tuple`
      - `js/record`
      - `js/arrays/tuple-and-record.js`
      - `js/arrows/tuple-and-record.js`
      - `js/binary-expressions/tuple-and-record.js`
      - `js/class-extends/tuple-and-record.js`
      - `js/comments-closure-typecast/tuple-and-record.js`
      - `js/comments/tuple-and-record.js`
      - `js/function-single-destructuring/tuple-and-record.js`
      - `js/method-chain/tuple-and-record.js`
    - pipeline operator `|>`
      - `js/comments-pipeline-own-line`
      - `js/partial-application`
      - `js/pipeline-operator`
    - bind operator `::`
      - `js/arrows-bind/`
      - `js/bind-expressions/`
      - `js/objects/expression.js`
      - `js/no-semi-babylon-extensions/no-semi.js`
    - `js/destructuring-private-fields/`
      - `js/deferred-import-evaluation/`
      - `js/source-phase-imports/`
      - `js/import-reflection/`

In addition to these ignored tests cases, [report-es215](./report-es2015.md) also ignores the following test cases:

- Widespread experimental syntaxes
  - Decorators
    - `js/decorators`
    - `js/decorator-auto-accessors/`
    - `js/decorators-export/`
    - `js/ignore/class-expression-decorator.js`
    - `js/ignore/decorator.js`
  - `js/import-assertions/`
  - `js/import-attributes/`
  - `js/explicit-resource-management`

- Standard ES2016+ syntaxes
  - exponentiation operator `**`
    - `js/async/exponentiation.js`
    - `js/binary-expressions/exp.js`
  - `async function` and `async () =>` and `await`
    - `js/async/`
    - `js/arrows/newline-before-arrow/newline-before-arrow.js`
    - `js/assignment/discussion-15196.js`
    - `js/assignment/issue-5610.js`
    - `js/assignment/issue-7091.js`
    - `js/assignment/issue-10218.js`
    - `js/assignment/lone-arg.js`
    - `js/ignore/issue-14404.js`
  - Trailing comma in function call
    - `js/trailing-comma/function-calls.js`
    - `js/arrows/arrow-chain-with-trailing-comments.js`
  - Object spread and rest `{ ...x }`
    - `js/spread`
    - `js/destructuring/`
    - `js/destructuring-ignore/`
    - `js/assignment/destructuring-heuristic.js`
    - `js/assignment/destructuring.js`
    - `js/function-single-destructuring/object.js`
    - `js/last-argument-expansion/assignment-pattern.js`
    - `js/last-argument-expansion/issue-10708.js`
    - `js/last-argument-expansion/issue-7518.js`
  - async iterator `for await`
    - `js/for-await/`
  - Private class field `#field`
    - `js/classes-private-fields`
    - `js/no-semi/private-field.js`
  - `try {} catch {}`
    - `js/optional-catch-binding`
  - Nullish coalescing `a ?? b`
    - `js/nullish-coalescing`
    - `js/arrows/chain-in-logical-expression.js`
  - Optional chaining `prop?.`
    - `js/optional-chaining/`
    - `js/optional-chaining-assignment/`
    - `js/chain-expression/test.js`
  - Bigint
    - `js/big-int/`
    - `js/objects/bigint-key.js`
  - Numeric separator `1_000`
    - `js/literal-numeric-separator/`
    - `js/quote-props/numeric-separator.js`
  - Logical assignment`??=`, `&&=`, ...
    - `js/logical-assignment/`
  - Private brand check `#field in`
    - `js/private-in`
  - Private methods
    - `js/classes/keyword-property/private.js`
    - `js/decorator-auto-accessors/private.js`
    - `js/decorator-auto-accessors/static-private.js`
  - class instance fields
    - `js/classes/class-fields-features.js`
  - Static class blocks `static {}`
    - `js/class-static-block/`
  - Top-level `await`
    - `js/top-level-await/`
  - regex `d` flag `/regex/d`
    - js/regex/d-flag.js",
  - Shebang `#!/usr/bin/node`
    - `js/shebang/`
  - regex `v` flag `/regex/v`
    - `js/regex/v-flag.js`

### Non-strict test cases

The following test cases are handled in non-strict JavaScript mode,
also known as sloppy mode or script mode.

- `js/with/`
- `js/sloppy-mode/`
- `js/identifier/`

### Unstable test cases

When formatted twice using Prettier, some tests cases are formatted differently.
The Biome testing infrastructure catches this kind of issues.

We decided to match the stabilized formatted version (i.e. the one obtained after multiple run of Prettier on the input - actually a second run is enough to stabilize the output).

The following test cases are affected by this formatting issue:

- `js/sequence-expression/parenthesized.js`
- `js/comments/tagged-template-literal.js`
- `js/comments/return-statement.js`
- `js/last-argument-expansion/embed.js`
- `js/for/continue-and-break-comment-without-blocks.js`
- `js/class-comment/misc.js`
- `js/range/boundary.js`
- `js/range/class-declaration.js`
- `js/range/multiple-statements2.js`

### Deliberate formatting divergences

Some divergences are deliberate, because of the strictness of Biome parsing phase or because we think it is more readable to keep the way it is.
You can find a detailed description of these test cases in the [dedicated issue](https://github.com/biomejs/biome/issues/739).


## Option support

We implemented all JavaScript options provided by Prettier.

See the documentation of [the global configuration for formatter](https://biomejs.dev/reference/configuration/#formatter) and [the specific configuration for the JavaScript formatter](https://biomejs.dev/reference/configuration/#formatter).

In contrast to Prettier, Biome only provides `as-needed` and `preserve` values for `quoteProps`. It doesn't provide the `consistent` value.
This is a deliberate choice.
