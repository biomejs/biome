// Test file for import pattern matching
// This should demonstrate that our fix allows the pattern `import $imports from "foo"`
// to match all types of imports from "foo"

// Default import - should match
import bar from "foo";

// Named import - should now match (previously didn't)
import { baz } from "foo";

// Multiple named imports - should now match (previously didn't)
import { qux, quux } from "foo";

// Namespace import - should now match (previously didn't)
import * as foobar from "foo";

// Combined import - should now match (previously didn't)
import defaultExport, { namedExport } from "foo";

// Bare import - should now match (previously didn't)
import "foo";

// Different source - should NOT match our pattern
import something from "different";
import { other } from "another"; 