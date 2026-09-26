# Extreme query string parser

Based on [Type Challenge 151](https://github.com/type-challenges/type-challenges/tree/0b0b0b18bcb7ac42dc22ce26ffb438231d4754b1/questions/00151-extreme-query-string-parser).
Recursive parsing and deduplication of up to 216 parameters, plus 18 route variants.
Times cold whole-module inference; parsed results currently infer as `Unknown`.

```sh
cargo bench -p biome_module_graph --bench type_inference -- type_challenges
```
