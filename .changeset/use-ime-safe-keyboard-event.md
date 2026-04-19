---
"@biomejs/biome": patch
---

Added the `noComposingEnterKey` nursery rule. It warns when `Enter` is handled in keyboard event callbacks without guarding IME composition, which helps prevent accidental submits while users are composing text.
