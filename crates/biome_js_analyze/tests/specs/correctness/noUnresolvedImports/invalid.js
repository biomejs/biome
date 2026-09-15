/* should generate diagnostics */
import foo from "./foo.js";
import { bar } from "./bar";
import { other } from "./nonExisting";

// Malformed `jsr:` specifiers are not valid JSR packages.
import { assert as missingScope } from "jsr:assert";
import { assert as missingName } from "jsr:@std";
import { assert as badVersion } from "jsr:@std/assert@notaversion";
