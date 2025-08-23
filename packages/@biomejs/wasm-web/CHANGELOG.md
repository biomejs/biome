# @biomejs/wasm-web

## 2.2.1

### Patch Changes

- [#7281](https://github.com/biomejs/biome/pull/7281) [`6436180`](https://github.com/biomejs/biome/commit/6436180f4a3b257e2de018bac45c99a76eff58be) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the function `scanProject` wouldn't work as expected.

## 2.2.0

### Minor Changes

- [#6896](https://github.com/biomejs/biome/pull/6896) [`527db7f`](https://github.com/biomejs/biome/commit/527db7f7c142f8c95c6d4513603530220a4cc95c) Thanks [@ematipico](https://github.com/ematipico)! - Added new functions to the `@biomejs/wasm-*` packages:
  - `fileExists`: returns whether the input file exists in the workspace.
  - `isPathIgnored`: returns whether the input path is ignored.
  - `updateModuleGraph`: updates the internal module graph of the input path.
  - `getModuleGraph`: it returns a serialized version of the internal module graph.
  - `scanProject`: scans the files and directories in the project to build the internal module graph.

## 2.1.4

## 2.1.3

## 2.1.2

## 2.1.1

## 2.1.0

## 2.0.6

## 2.0.5

## 2.0.4

## 2.0.3

## 2.0.2

## 2.0.1

## 2.0.0

### Patch Changes

- [`9d5d95f`](https://github.com/biomejs/biome/commit/9d5d95fffd5734522c8911db18c6d16ee6a96756) Thanks [@arendjr](https://github.com/arendjr)! - Fixed the type definition of `IFileFeaturesResult.featuresSupported`
