---
"@biomejs/biome": patch
---
Our JavaScript parser can now gracefully handle situations where we detect the
parser to have stalled, such as in
[#4622](https://github.com/biomejs/biome/issues/4622). This means we don't fail
with an assertion anymore, but invalid code can trigger a graceful diagnostic
in such cases.
