/* should generate diagnostics */
import * as names from "./validNamedExports.js";
import * as barrel from "./validBarrel.js";
import * as empty from "./validEmpty.js";
import * as starBarrel from "./validStarBarrel.js";
import defaultNamespace from "./validDefaultNamespace.js";
import first, * as combined from "./validNamedExports.js";
import { deep } from "./validNamedExports.js";
import { "deep" as quotedDeep } from "./validNamedExports.js";
export * as reexportedEmpty from "./validEmpty.js";

names.c;
names?.c;
(names).c;
names.default;
names.deep.missing;
names.deep.deeper.missing;
(names.deep).missing;

const { c } = names;
const { c: renamed } = names;
const { c: { nested } } = names;
const { "c": quoted } = names;
const { deep: { missing } } = names;
const { deep: { deeper: { missing: alsoMissing } } } = names;

function inner() {
	const { c } = names;
	return names.c;
}

names.a = 1;
names.c = 1;
names["a"] = 1;
names.deep.e = 1;
names.a += 1;
names.a++;
[names.a] = [1];
({ value: names.a } = { value: 1 });

barrel.c;
barrel.importedDeep.missing;
defaultNamespace.missing;
combined.c;
deep.missing;
deep.deeper.missing;
quotedDeep.missing;
empty.anything;
// `export * from` doesn't forward the default export.
starBarrel.default;
