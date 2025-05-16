---
"@biomejs/biome": minor
---

It's possible to override the option `files.maxSize`. This option is helpful if you need to process specific files that exceed the default `maxSize`:

```json
{
	"overrides": [{
		"includes": ["dist/**"]
		"files": {
			"maxSize": 20000
		}
	}]
}
```
