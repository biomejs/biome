---
"@biomejs/biome": minor
---

Added support for formatting and linting embedded CSS snippets in JavaScript.

For example, the following snippets are now formatted and linted:

```js
import styled from "styled-components";

const Foo = styled.div`
  display: flex;
  color: red;
`;
```

```js
import styled from "@emotion/styled";

const Foo = styled(Box)`
  display: flex;
  color: red;
`;
```

```js
import { css } from "@emotion/react";

const style = css`
  display: flex;
  color: red;
`;
```

However, snippets with interpolations are not supported yet. For example, the following snippet **isn't** formatted:

```ts
import styled from "@emotion/styled";

const Foo = styled.div<{color: string}>`
  display: flex;
  color: ${(props) => props.color};
`;
```

This feature is experimental and must be enabled explicitly in the configuration:

```json
{
  "javascript": {
    "experimentalEmbeddedSnippetsEnabled": true
  }
}
```
