/* should not generate diagnostics */

// Importing a package-private symbol within the same package is allowed.
import { fooPackageVariable } from "./foo.js";
import { fooPackageVariable as fooPackage2 } from "./sub";

// Importing a public symbol is always allowed.
import { fooPublicVariable } from "./foo.js";
import { fooPublicVariable as fooPublic2 } from "./sub/foo.js";
import { fooPublicVariable as fooPublic3 } from "./sub";

// Importing a symbol without any visibility is allowed when the default visibility is public.
import { fooDefaultVariable } from "./foo.js";
import { fooDefaultVariable as fooDefault2 } from "./sub";

// Importing a function with more extensive docs should also work.
import { fooPublicFunction } from "./foo.js";
