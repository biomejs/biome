# @biomejs/js-api

## 2.0.0

### Minor Changes

- [#6535](https://github.com/biomejs/biome/pull/6535) [`d8c08e1`](https://github.com/biomejs/biome/commit/d8c08e1691a1b64cf48e86bd490bfe1485df3fa1) Thanks [@regseb](https://github.com/regseb)! - Biome JavaScript Bindings has now specific
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

### Patch Changes

- Updated dependencies []:
  - @biomejs/wasm-web@2.1.0
  - @biomejs/wasm-bundler@2.1.0
  - @biomejs/wasm-nodejs@2.1.0

## 1.0.0

### Minor Changes

- [`9d5d95f`](https://github.com/biomejs/biome/commit/9d5d95fffd5734522c8911db18c6d16ee6a96756) Thanks [@arendjr](https://github.com/arendjr)! - The package now requires `v2` of the WebAssembly packages. The internal APIs of Workspace are now `camelCase`.

### Patch Changes

- [`9d5d95f`](https://github.com/biomejs/biome/commit/9d5d95fffd5734522c8911db18c6d16ee6a96756) Thanks [@arendjr](https://github.com/arendjr)! - Removed wrong `openProject()` definition, and added JSDoc documentation.

- [`9d5d95f`](https://github.com/biomejs/biome/commit/9d5d95fffd5734522c8911db18c6d16ee6a96756) Thanks [@arendjr](https://github.com/arendjr)! - Fixed the type definition of `IFileFeaturesResult.featuresSupported`

- Updated dependencies [[`9d5d95f`](https://github.com/biomejs/biome/commit/9d5d95fffd5734522c8911db18c6d16ee6a96756)]:
  - @biomejs/wasm-web@2.0.0
  - @biomejs/wasm-nodejs@2.0.0
  - @biomejs/wasm-bundler@2.0.0
