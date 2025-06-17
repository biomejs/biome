---
"@biomejs/biome": patch
---
Fixed [#4553](https://github.com/biomejs/biome/issues/4553): `noUselessFragments` will now correctly fix JSX attributes:

```jsx
<Suspense fallback={<><span>Loading...</span></>}>
  {children}
</Suspense>;
```

becomes:

```jsx
<Suspense fallback={<span>Loading...</span>}>
  {children}
</Suspense>;
```
