/* should generate diagnostics */

import * as CB from "callbag-basics";
import { forEach } from "strict-callbag-basics";

CB.forEach((x) => console.log(x));

forEach.forEach((x) => console.log(x));
