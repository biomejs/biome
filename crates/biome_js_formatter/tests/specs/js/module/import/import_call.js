import(x)
import('x')
import(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, {assert: {type:'json'}})
import.defer("foo");
import.defer("x", { with: { attr: "val" } });
import.source("foo");
import.source("x", { with: { attr: "val" } });
import.source("foo", { type: "bar" });
