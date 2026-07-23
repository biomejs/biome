---
"@biomejs/biome": patch
---

The `rdjson` reporter now populates the [severity](https://github.com/reviewdog/reviewdog/blob/master/proto/rdf/reviewdog.proto) field of each diagnostic (`ERROR`, `WARNING`, or `INFO`), so tools consuming Reviewdog Diagnostic Format output no longer need to assume a default severity.
