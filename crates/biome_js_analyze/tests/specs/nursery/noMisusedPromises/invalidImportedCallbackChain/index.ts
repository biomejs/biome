/* should generate diagnostics */

import { wrappedLoad } from "./wrapper";

const values = [1, 2, 3];

values.forEach((value) => wrappedLoad(value));
