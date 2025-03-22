// Attempt to import a package-private symbol from outside the package.
import { fooPackageVariable as subPackage } from "./sub/foo.js";

// Attempt to import a private symbol from outside the package.
import { fooPrivateVariable as subPrivate } from "./sub/foo.js";
import { fooPrivateVariable as subPrivate2 } from "./sub";

// Attempt to import a private symbol from the same package.
import { fooPrivateVariable } from "./foo.js";

// Default and combined imports of private symbols are still not allowed
import privateFunction from "./foo.js";
import privateFunction2, { fooPrivateVariable as subPrivate2 } from "./sub/foo.js";
