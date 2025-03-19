/* should not generate diagnostics */

// Importing a package-private symbol within the same package is allowed.
import { fooPackageVariable } from "./foo.js";

// Importing a public symbol is always allowed.
import { fooPublicVariable } from "./foo.js";
import { fooPublicVariable as sub } from "./sub/foo.js";

// Importing a symbol without any visibility is allowed when the default visibility is public.
import { fooDefaultVariable } from "./foo.js";
