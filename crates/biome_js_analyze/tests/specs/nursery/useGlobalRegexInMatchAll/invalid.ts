/* should generate diagnostics */
"".matchAll(/a/ as RegExp);
"".matchAll(<RegExp>/a/);

const pattern1: RegExp = /a/;
"".matchAll(pattern1);

const pattern2 = /a/ satisfies RegExp;
"".matchAll(pattern2);
