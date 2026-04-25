/* should generate diagnostics */

declare const text: string;
declare const needle: string;

text.indexOf(needle) === 0;
0 === text.indexOf(needle);
text.indexOf(needle) !== 0;
text.lastIndexOf(needle) === text.length - needle.length;
text.length - needle.length === text.lastIndexOf(needle);
text.lastIndexOf("bar") !== text.length - 3;
