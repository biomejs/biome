---
"@biomejs/biome": minor
---

Added the `noAccessKey` lint rule for HTML. The rule enforces that the `accesskey` attribute is not used on any HTML element, as it can conflict with keyboard commands used by screen readers and keyboard-only users.

Invalid:

```html
<input type="submit" accesskey="s" value="Submit" />
<a href="https://webaim.org/" accesskey="w">WebAIM.org</a>
<button accesskey="n">Next</button>
```

Valid:
```html
<input type="submit" value="Submit" />
<a href="https://webaim.org/">WebAIM.org</a>
<button>Next</button>
```
