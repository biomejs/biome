/* should generate diagnostics */

declare const value: "a" | "b";
declare const dynamicCase: "a" | "b";

switch (value) {
    case dynamicCase:
        break;
}
