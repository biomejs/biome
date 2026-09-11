/* should not generate diagnostics */
import type { mySymbol } from "./helpers.ts";

class MyClass {
    declare [mySymbol]: number;
}

console.log(MyClass);
