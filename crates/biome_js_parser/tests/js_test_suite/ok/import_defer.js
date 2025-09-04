import defer * as feature from "some-module";
import defer * as ns from "x" with { attr: "val" };
import.defer("foo");
import.defer("x", { with: { attr: "val" } });