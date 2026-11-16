/* should generate diagnostics */

import type { Box } from "./library";

declare const box: Box<string | null>;

export const r = box.value || "x";
