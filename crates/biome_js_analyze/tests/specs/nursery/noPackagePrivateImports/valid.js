/* should not generate diagnostics */

// Imports within the same module are always allowed.
import { fooPackageVariable } from "./foo.js";

// Resources (anything other than JS/TS files) are exempt.
import { barResource } from "../aunt/bar.png";

// A parent index file is accessible like other modules.
import { internal } from "../../index.js";

// If the `sub` module is accessible, so is its index file.
import { subPackageVariable } from "./sub/index.js";

// Library imports are exempt.
import useAsync from "react-use/lib/useAsync";

// Including library imports with an extension.
import map from "lodash/map.js";

// Scoped packages work too.
import netlify from "@astrojs/netlify/functions.js";
