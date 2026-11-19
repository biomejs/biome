/* should not generate diagnostics */
import { createContext, useId, useRef } from "some-other-library";

// The hooks are not imported from React, so the convention does not apply.
const theme = createContext("");
const randomString = useId();
const node = useRef(null);
