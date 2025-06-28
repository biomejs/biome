---
"@biomejs/js-api": minor
---

Biome JavaScript Bindings has now specific
[subpath exports](https://nodejs.org/api/packages.html#subpath-exports) for the
three packages:

- `import { Biome } from "@biomejs/js-api/bundler";`
- `import { Biome } from "@biomejs/js-api/nodejs";`
- `import { Biome } from "@biomejs/js-api/web";`

These new specific subpath exports load only TypeScript declarations, whereas
the default subpath export loads declarations for all three packages. This was a
problem if you checked your code with
[`tsc`](https://www.typescriptlang.org/docs/handbook/compiler-options.html).

- Old usage with default subpath export:

  ```js
  import { Biome, Distribution } from "@biomejs/js-api";

  const biome = await Biome.create({ distribution: Distribution.NODE });
  ```

- New usage with a specific subpath export:

  ```js
  import { Biome } from "@biomejs/js-api/nodejs";

  const biome = new Biome();
  ```
