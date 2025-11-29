---
"@biomejs/biome": patch
---

The HTML formatter has been updated to match Prettier 3.7's behavior for handling `<iframe>`'s `allow` attribute.

```diff
- <iframe allow="layout-animations 'none'; unoptimized-images 'none'; oversized-images 'none'; sync-script 'none'; sync-xhr 'none'; unsized-media 'none';"></iframe>
+ <iframe
+ 	allow="
+ 		layout-animations 'none';
+ 		unoptimized-images 'none';
+ 		oversized-images 'none';
+ 		sync-script 'none';
+ 		sync-xhr 'none';
+ 		unsized-media 'none';
+ 	"
+ ></iframe>
```
