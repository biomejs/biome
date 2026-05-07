Built-in parsers:
- [`babylon`](https://github.com/babel/babylon/)
- [`flow`](https://github.com/facebook/flow/tree/master/src/parser)
- [`typescript`](https://github.com/eslint/typescript-eslint-parser) _Since v1.4.0_
- [`postcss`](https://github.com/postcss/postcss) _Since v1.4.0_
- [`json`](https://github.com/babel/babylon/tree/f09eb3200f57ea94d51c2a5b1facf2149fb406bf#babylonparseexpressioncode-options) _Since v1.5.0_
- [`graphql`](https://github.com/graphql/graphql-js/tree/master/src/language) _Since v1.5.0_


Valid options:

- `true` - Add a semicolon at the end of every statement.
- `false` - Only add semicolons at the beginning of lines that may introduce ASI failures.
