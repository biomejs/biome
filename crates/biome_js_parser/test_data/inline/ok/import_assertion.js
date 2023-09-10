import "x" assert { type: "json" }
import "foo" assert { "type": "json" };
import foo from "foo.json" assert { type: "json" };
import {test} from "foo.json" assert { for: "for" }
import foo_json from "foo.json" assert { type: "json", hasOwnProperty: "true" };
import "x" assert
{ type: "json" }
