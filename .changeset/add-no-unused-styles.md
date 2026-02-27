---
"@biomejs/biome": minor
---

Added new nursery lint rule [`noUnusedStyles`](https://biomejs.dev/linter/rules/no-unused-styles/) for CSS. The rule detects CSS class selectors that are never referenced in any HTML or JSX file that imports the stylesheet. This is a project-domain rule that requires the module graph.

```css
/* styles.css — .ghost is never used in any importing file */
.button { color: blue; }
.ghost { color: red; }
```

```jsx
/* App.jsx */
import "./styles.css";
export default () => <div className="button" />;
```
