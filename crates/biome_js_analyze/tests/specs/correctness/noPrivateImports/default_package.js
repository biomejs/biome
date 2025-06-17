import { unknown } from "external-package";

// Importing a symbol without any visibility from sub package is NOT allowed when the default visibility is package.
import { fooDefaultVariable } from "./sub/foo.js";

// Re-exporting widens the allowed import scope for package private, so this is allowed:
import { fooDefaultVariable as fooDefault2 } from "./sub";

// Looser visibility takes precedence over the default visibility, so these imports are allowed.
import { fooPackageVariable } from "./foo.js";
import { fooPublicVariable } from "./foo.js";
import { fooPublicVariable as subPublic } from "./sub/foo.js";

// Tighter visibility also takes precedence, so these imports are NOT allowed.
import { fooPackageVariable as subPackage } from "./sub/foo.js";
import { fooPrivateVariable as subPrivate } from "./sub/foo.js";
