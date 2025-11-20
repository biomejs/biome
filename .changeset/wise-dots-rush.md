---
"@biomejs/biome": patch
---

Update CSS formatting of dimension units to use correct casing for Q, Hz and kHz

**Before:**

``` css
.cssUnits {
  a: 1q;
  b: 1hz;
  c: 1khz;
}
```

**After:**

``` css
.cssUnits {
  a: 1Q;
  b: 1Hz;
  c: 1kHz;
}
```
