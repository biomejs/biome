/* should generate diagnostics */

import { getPayload } from "./api.ts";

// Assigning return value of imported function returning `any`
const payload = getPayload();

// Assigning to a variable with non-any annotation
const typed: string = getPayload();

// Let declaration
let mutable = getPayload();
