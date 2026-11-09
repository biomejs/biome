/* should generate diagnostics */
import * as Names from "./validNamedExports.js";

const element = <Names.e />;
const nested = <Names.deep.missing />;
const withChildren = <Names.missing>text</Names.missing>;
