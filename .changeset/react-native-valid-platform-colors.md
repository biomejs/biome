---
"@biomejs/biome": patch
---

Added the nursery rule [`useReactNativeValidPlatformColors`](https://biomejs.dev/linter/rules/use-react-native-valid-platform-colors/), which requires `PlatformColor()` and `DynamicColorIOS()` calls to use values written directly in the call, so React Native can optimize them.

```js
import { PlatformColor } from "react-native";

const labelColor = "labelColor";
const color = PlatformColor(labelColor);
```
