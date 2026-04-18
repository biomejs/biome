---
"@biomejs/biome": patch
---

Added the nursery rule [`useReactNativePlatformComponents`](https://biomejs.dev/linter/rules/use-react-native-platform-components/) that ensures platform-specific React Native components (e.g. `ProgressBarAndroid`, `ActivityIndicatorIOS`) are only imported in files with a matching platform suffix. It also reports when Android and iOS components are mixed in the same file.

The following code triggers the rule when the file does not have an `.android.js` suffix:

```js
// file.js
import { ProgressBarAndroid } from "react-native";
```
