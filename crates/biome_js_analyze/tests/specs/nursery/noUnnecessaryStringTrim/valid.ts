/* should not generate diagnostics */
import { z } from "zod";

(z.string() as unknown).trim().startsWith("-");
(z.string() satisfies unknown).trim().startsWith("-");
z.string()!.trim().startsWith("-");
(<unknown>z.string()).trim().startsWith("-");
value.trim().startsWith("foo " as const);
value.trim().startsWith((cond ? "x" : "x ") as string);
