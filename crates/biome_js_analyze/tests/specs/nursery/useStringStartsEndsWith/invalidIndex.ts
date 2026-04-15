/* should generate diagnostics */

declare const text: string;
declare const other: string;

text[0] === "a";
"a" === text[0];
text[0] !== "a";
text[0] === other;
text[text.length - 1] === "z";
"z" === text[text.length - 1];
