/* should not generate diagnostics */

import type { Box } from "./library";

declare const box: Box<string>;

box.value || null;
