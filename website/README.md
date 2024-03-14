# [`biomejs.dev`](https://biomejs.dev/)

## Installation

First install pnpm by enabling [corepack](https://nodejs.org/api/corepack.html):

```shell
corepack enable
```

Then install the required dependencies:

```shell
pnpm i --filter @biomejs/website --frozen-lockfile
```

## Local Development

```shell
pnpm --filter @biomejs/website start
```

This command starts a local development server. Most changes are reflected live without having to restart the server.

If you want to work on the playground, additional artifacts are required and the following command must be used instead:

```shell
pnpm --filter @biomejs/website start:playground
```

## Build

```shell
pnpm --filter @biomejs/website build
```

This command generates static content into the build directory and can be served using any static contents hosting service.
