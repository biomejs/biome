/* should not generate diagnostics */

import type { FormValues } from "./library";

declare const values: FormValues<number>;

16 * values.pounds + values.ounces;
