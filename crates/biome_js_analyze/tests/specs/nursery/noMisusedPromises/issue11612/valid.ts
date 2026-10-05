/* should not generate diagnostics */

import { schema } from "./library";

schema.refine((data) => data.password === data.confirmPassword);
