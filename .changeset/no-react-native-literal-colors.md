---
"@biomejs/biome": patch
---

Added the nursery rule [`noReactNativeLiteralColors`](https://biomejs.dev/linter/rules/no-react-native-literal-colors/), which disallows color literals inside React Native styles.

The rule belongs to the `reactNative` domain. It reports properties whose name contains `color` and whose value is a string literal when they appear inside a `StyleSheet.create(...)` call or inside a JSX attribute whose name contains `style`.

```jsx
// Invalid
const Hello = () => <Text style={{ backgroundColor: '#FFFFFF' }}>hi</Text>;

const styles = StyleSheet.create({
    text: { color: 'red' }
});
```

```jsx
// Valid
const red = '#f00';
const styles = StyleSheet.create({
    text: { color: red }
});
```
