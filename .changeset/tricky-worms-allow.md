---
"@biomejs/biome": patch
---

Fixed a bug where the combination `files.includes` and `extends` might result in an incorrect list of glob patterns.
As per requirements, the list of `files.includes` must have `**` at the beginning *if there are only negated patterns*.

The bug was caused by an incorrect merging of the `files.includes` and `extends` fields. When the `extends` field was merged into the `files.includes` field, which could result in `**` not being in at the first place of the list.

After this fix, if a configuration file coming from `extends` defines `**` in the first place, and the user configuration uses its own `files.includes`, the final list will contain `**` in the first place.

The following example, the final `files.includes` list will be `["**", "!**/dist", "!components"]`

**Example**

```json5
// shared.json, some shared configuration
{
  "files": {
    "includes": ["**", "!**/dist"]
  }
}
```

```json5
// biome.json, the user configuration
{
  "files": {
    "includes": ["!components"]
  }
}
```
Plus, Biome now removes duplicates
