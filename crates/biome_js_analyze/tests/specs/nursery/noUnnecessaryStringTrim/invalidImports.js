/* should generate diagnostics */
import { z as a } from "other-package"; a.string().trim().startsWith("-");
import * as b from "other-package"; b.string().trim().startsWith("-");
import { string as c } from "zod"; c.string().trim().startsWith("-");
import { z as d } from "zod"; d.string("argument").trim().startsWith("-");
import { z as e } from "zod"; e.number.trim().startsWith("-");
import { z as f } from "zod"; function g(f) { f.string().trim().startsWith("-"); }
z.string().trim().startsWith("-");
