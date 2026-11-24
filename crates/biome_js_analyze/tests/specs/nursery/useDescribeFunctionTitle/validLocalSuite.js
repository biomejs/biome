/* should not generate diagnostics */
import { myFunction } from "./myFunction";

function describe(title, callback) {}

describe("myFunction", () => {});

describe(myFunction.name, () => {});
