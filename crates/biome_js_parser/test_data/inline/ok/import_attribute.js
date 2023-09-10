import "x" with { type: "json" }
import "foo" with { "type": "json" };
import foo from "foo.json" with { type: "json" };
import {test} from "foo.json" with { for: "for" }
import foo_json from "foo.json" with { type: "json", hasOwnProperty: "true" };
import "x" with
{ type: "json" }
