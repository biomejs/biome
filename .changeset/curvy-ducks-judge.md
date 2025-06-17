---
"@biomejs/biome": minor
---

Added a new `propertyAssignment` option to the `noParameterAssign` rule.
This option allows to configure whether property assignments on function parameters are permitted.
By default, `propertyAssignment` is set to `allow`.
Setting it to `deny` enforces stricter immutability by disallowing property mutations on function parameters.
