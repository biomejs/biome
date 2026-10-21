---
"@biomejs/biome": patch
---

Fixed [#11446](https://github.com/biomejs/biome/issues/11446): type inference ignored a call's explicit type arguments when the callee was an overload set, so [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) reported conditions on results like `garrison<Trooper>(null)` as always truthy. The type arguments now instantiate every overload before one is selected.

Overload selection was also corrected for calls that supply no type arguments. A union argument is now accepted by a union parameter when every one of its members is accepted, so `string | number` matches a `string | number | boolean` parameter. A callback parameter whose return type accepts any result, such as `() => unknown`, `() => any`, or an unconstrained generic return, now accepts an `async` callback, while `() => string | void` and a constrained generic return still do not.
