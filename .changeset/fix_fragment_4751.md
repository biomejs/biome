---
"@biomejs/biome": patch
---

Fixed [#4751](https://github.com/biomejs/biome/issues/4751) by checking fragments inside `JSXElement` and conditional expressions. 

For example, the following two cases will now be reported:

```jsx
<section>
  <>
    <div />
    <div />
  </>
</section>;
```

```jsx
showFullName ? <>{fullName}</> : <>{firstName}</>;
```
