/* should generate diagnostics */

declare const text: string;
declare const needle: string;

text.slice(0, 3) === "bar";
text.slice(0, needle.length) === needle;
text.slice(-3) === "bar";
text.slice(-needle.length) === needle;
text.slice(-needle.length, text.length) === needle;
text.slice(text.length - needle.length) === needle;
text.slice(text.length - needle.length, text.length) === needle;
text.substring(0, 3) === "bar";
text.substring(text.length - 3, text.length) === "bar";
