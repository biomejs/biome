import "foo" with { type, "json" };
import { foo } with { type: "json" };
import foo2 from "foo.json" with { "type": "json", type: "html", "type": "js" };
import "x" with;
import ipsum from "ipsum.json" with { type: "json", lazy: true, startAtLine: 1 };
import { a } from "a.json" with
