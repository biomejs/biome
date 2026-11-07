---
"@biomejs/biome": patch
---

Fixed [#12956](https://github.com/biomejs/biome/issues/12956): React, React Native, and Next.js rules that rely on framework-specific JSX behavior no longer report Astro template expressions. Mixed-framework projects can enable those rule domains without receiving React- or Next-specific diagnostics for Astro markup such as `<head>`, `<img>`, component `children` props, string refs, Astro list rendering, or Polyfill.io scripts.
