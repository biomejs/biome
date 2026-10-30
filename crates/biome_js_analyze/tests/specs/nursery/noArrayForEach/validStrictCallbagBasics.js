/* should not generate diagnostics */

import * as CB from "strict-callbag-basics";
import * as callbagBasics from "strict-callbag-basics";

CB.pipe(source, CB.forEach((x) => console.log(x)));

callbagBasics.forEach((value) => console.log(value));
