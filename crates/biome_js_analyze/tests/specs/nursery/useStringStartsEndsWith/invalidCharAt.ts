/* should generate diagnostics */

declare const text: string;
declare const other: string;

text.charAt(0) === "a";
"a" === text.charAt(0);
text.charAt(0) !== "a";
text.charAt(0) === other;
text.charAt(text.length - 1) === "z";
"z" === text.charAt(text.length - 1);
