---
"@biomejs/biome": patch
---

Fix [#4751](https://github.com/biomejs/biome/issues/4751) by checking fragments inside `JSXElement` and conditional expressions. For example:

The Case:

```jsx
<section>
  <>
    <div />
    <div />
  </>
</section>;
```

And:

```jsx
showFullName ? <>{fullName}</> : <>{firstName}</>;
```

It will report.
