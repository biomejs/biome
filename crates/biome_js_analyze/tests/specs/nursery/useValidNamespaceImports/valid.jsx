/* should not generate diagnostics */
import * as Names from "./validNamedExports.js";

const element = <Names.a />;
const nested = <Names.deep.e />;
const withChildren = <Names.ExportedClass>text</Names.ExportedClass>;
