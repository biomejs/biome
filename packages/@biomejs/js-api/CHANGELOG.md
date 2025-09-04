# @biomejs/js-api

## 3.0.0

### Patch Changes

- Updated dependencies [[`527db7f`](https://github.com/biomejs/biome/commit/527db7f7c142f8c95c6d4513603530220a4cc95c)]:
  - @biomejs/wasm-bundler@2.2.0
  - @biomejs/wasm-nodejs@2.2.0
  - @biomejs/wasm-web@2.2.0

## 2.0.3

### Patch Changes

- [#6785](https://github.com/biomejs/biome/pull/6785) [`085e3c7`](https://github.com/biomejs/biome/commit/085e3c756344c92adbf69d5270b93402ff9713e8) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6722](https://github.com/biomejs/biome/issues/6772): Missing `dist/` files are now included in the `@biomejs/js-api` package. The previous release haven't fixed the issue properly.

## 2.0.2

### Patch Changes

- [#6780](https://github.com/biomejs/biome/pull/6780) [`563f3d5`](https://github.com/biomejs/biome/commit/563f3d5fbcfb220bf5c2a386be385a42d2a7069c) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6722](https://github.com/biomejs/biome/issues/6772): Missing `dist/` files are now included in the `@biomejs/js-api` package. The previous release haven't fixed the issue properly.

- Updated dependencies []:
  - @biomejs/wasm-web@2.1.1
  - @biomejs/wasm-bundler@2.1.1
  - @biomejs/wasm-nodejs@2.1.1

## 2.0.1

### Patch Changes

- [#6776](https://github.com/biomejs/biome/pull/6776) [`08652d0`](https://github.com/biomejs/biome/commit/08652d0dfd34f84759597dc7f613cc260e362ee9) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6722](https://github.com/biomejs/biome/issues/6772): Missing `dist/` files are now included in the `@biomejs/js-api` package.

## 2.0.0

### Minor Changes

- [#6535](https://github.com/biomejs/biome/pull/6535) [`d8c08e1`](https://github.com/biomejs/biome/commit/d8c08e1691a1b64cf48e86bd490bfe1485df3fa1) Thanks [@regseb](https://github.com/regseb)! - Biome's JavaScript Bindings now have specific [subpath exports](https://nodejs.org/api/packages.html#subpath-exports) for the three packages:
  - `import { Biome } from "@biomejs/js-api/bundler";`
  - `import { Biome } from "@biomejs/js-api/nodejs";`
  - `import { Biome } from "@biomejs/js-api/web";`

  These new subpath exports load only TypeScript declarations, whereas the default export loads declarations for all three packages. This was a problem if you checked your code with [`tsc`](https://www.typescriptlang.org/docs/handbook/compiler-options.html).
  - Old usage with default export (no subpath):

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
