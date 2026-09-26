/* should generate diagnostics */
import { log, value } from "./helpers";
[1].forEach(x => log(x));
[1].map(x => log(x));
[1].forEach(() => value());
