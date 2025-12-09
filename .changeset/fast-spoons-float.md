---
"@biomejs/biome": patch
---

Updated the documentation & diagnostic message for `lint/nursery/noProto`, mentioning the reasons for its longstanding deprecation and why more modern alternatives are preferred.

Notably, the rule clearly states that using `__proto__` inside object literal definitions is still allowed, being a standard way to set the prototype of a newly created object.
