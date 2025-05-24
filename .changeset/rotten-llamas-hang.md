---
"@biomejs/biome": patch
---

Fixed [#5985](https://github.com/biomejs/biome/issues/5985), which caused the import organizer to fail the merging of a default import with a named import.
The following code is now correctly organized:

```diff
- import moment from 'moment';
- import { Moment } from 'moment';
+ import moment, { Moment } from 'moment';
```
