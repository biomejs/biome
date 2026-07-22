---
"@biomejs/biome": patch
---

Fixed [#11014](https://github.com/biomejs/biome/issues/11014): [`noDelete`](https://biomejs.dev/linter/rules/no-delete/) no longer reports `delete process.env["FOO"]`. The rule already allowed the static form `delete process.env.FOO`, since Node.js documents `delete` as the way to remove environment variables; the computed form is now treated the same.
