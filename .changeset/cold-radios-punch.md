---
"@biomejs/biome": "patch"
---

Fixed [#4093](https://github.com/biomejs/biome/issues/4093): the [`noDelete`](https://biomejs.dev/linter/rules/no-delete/) rule no longer triggers for `delete process.env.FOO`, since `delete` is the documented way to remove environment variables in Node.js.
