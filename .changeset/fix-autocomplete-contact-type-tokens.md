---
"@biomejs/biome": patch
---

Fixed [#10743](https://github.com/biomejs/biome/issues/10743): The [`useValidAutocomplete`](https://biomejs.dev/linter/rules/use-valid-autocomplete/) rule now accepts a contact-type qualifier (`home`, `work`, `mobile`, `fax`, `pager`) before a telephone, email, or messaging field, matching the WHATWG autofill grammar.

```html
<!-- No longer reported -->
<input type="tel" autocomplete="work tel" />
<input type="email" autocomplete="home email" />
```

The qualifier still only applies to those fields, so `autocomplete="home url"` stays invalid.
