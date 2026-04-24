---
"@biomejs/biome": patch
---

Added the nursery rule [`noReactNativeRawText`](https://biomejs.dev/linter/rules/no-react-native-raw-text/), which disallows raw text outside of `<Text>` components in React Native.

The rule belongs to the new `reactNative` domain.

```jsx
// Invalid
<View>some text</View>
<View>{'some text'}</View>
```

```jsx
// Valid
<View><Text>some text</Text></View>
```

Additional components can be allowlisted through the `skip` option:

```json
{
    "options": {
        "skip": ["Title"]
    }
}
```
