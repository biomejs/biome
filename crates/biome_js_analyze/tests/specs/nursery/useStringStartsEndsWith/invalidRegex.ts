/* should generate diagnostics */

declare const text: string;

text.match(/^bar/) !== null;
text.match(/bar$/) === null;
/^bar/.test(text);
/bar$/.test(text);
