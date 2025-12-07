---
"@biomejs/biome": patch
---

Clarify diagnostic message for `lint/style/useUnifiedTypeSignatures`

The rule's diagnostic message now clearly states that multiple _similar_ overload signatures are hard to read & maintain, as opposed to overload signatures in general.
