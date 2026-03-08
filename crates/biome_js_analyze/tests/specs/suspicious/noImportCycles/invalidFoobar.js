import { baz } from "./invalidBaz.js";

export function foo() {
    baz();
}

export function bar() {
    console.log("foobar");
}
