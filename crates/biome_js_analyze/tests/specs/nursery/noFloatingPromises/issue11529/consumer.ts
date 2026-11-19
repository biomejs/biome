/* should generate diagnostics */

import { load } from "./loader";

export interface Consumer {}

load().then(() => {});
