// Importing a symbol without any visibility from sub package is NOT allowed when the default visibility is package.
import { fooDefaultVariable } from "./sub/foo.js";

// Looser visibility takes precedence over the default visibility, so these imports are allowed.
import { fooPackageVariable } from "./foo.js";
import { fooPublicVariable } from "./foo.js";
import { fooPublicVariable as subPublic } from "./sub/foo.js";

// Tighter visibility also takes precedence, so these imports are NOT allowed.
import { fooPackageVariable as subPackage } from "./sub/foo.js";
import { fooPrivateVariable as subPrivate } from "./sub/foo.js";
