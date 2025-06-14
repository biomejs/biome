// Test file to verify import pattern matching fix

// Default import
import bar from "foo";

// Named import
import { baz } from "foo";

// Multiple named imports
import { qux, quux } from "foo";

// Namespace import
import * as foobar from "foo";

// Combined import
import defaultExport, { namedExport } from "foo";

// Different source (should not match)
import something from "different"; 