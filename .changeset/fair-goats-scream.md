---
"@biomejs/biome": patch
---

Added a new nursery rule [`noReactNativeDeepImports`](https://biomejs.dev/linter/rules/no-react-native-deep-imports/) that disallows deep imports from the `react-native` package. Internal paths like `react-native/Libraries/...` are not part of the public API and may change between versions.

For example, the following code triggers the rule:

```js
import View from "react-native/Libraries/Components/View/View";
```
