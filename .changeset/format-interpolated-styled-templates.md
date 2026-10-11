---
"@biomejs/biome": patch
---

Added support for formatting CSS-in-JS templates with interpolations, such as the ones of styled-components and Emotion, when `javascript.experimentalEmbeddedSnippetsEnabled` is enabled. The interpolations are formatted as JavaScript, and the text glued to them is kept as is.

```diff
 const Button = styled.button`
-  color:${ (props)=>props.color };
-  width : ${({width})=>width}px;
+  color: ${(props) => props.color};
+  width: ${({ width }) => width}px;
 `;
```
