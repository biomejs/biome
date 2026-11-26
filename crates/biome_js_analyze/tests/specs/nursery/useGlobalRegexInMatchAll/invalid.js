/* should generate diagnostics */
"".matchAll(/a/);
"".matchAll(/a/u);
"".matchAll(/a/ius);
"".matchAll((/a/));
"".matchAll(new RegExp("a"));
"".matchAll(new RegExp("a", "i"));
"".matchAll(new RegExp(`a`, `u`));
"".matchAll(new RegExp(/a/));
"".matchAll(new RegExp(/a/g, "i"));
"".matchAll(new RegExp());
"".matchAll(RegExp("a"));
"".matchAll(globalThis.RegExp("a", "i"));
"".matchAll(new window.RegExp("a"));
""["matchAll"](/a/);
""?.matchAll(/a/);
"".matchAll(/a/, extra);
text.matchAll(/a/);

const pattern1 = /a/;
"".matchAll(pattern1);

const pattern2 = /a/i;
"".matchAll(pattern2);

const pattern3 = new RegExp("a");
"".matchAll(pattern3);

const pattern4 = /a/;
const alias4 = pattern4;
"".matchAll(alias4);
