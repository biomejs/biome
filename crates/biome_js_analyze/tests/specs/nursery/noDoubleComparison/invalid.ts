/* should generate diagnostics */
(x as number) === y || (x as number) < y;
x! === y || x! < y;
(x satisfies number) === y || (x satisfies number) < y;
(<number>x) === y || x < y;
x === y as number || x < y;
x === "a" satisfies string || x < "a";
(f<string>) === g || (f<string>) < g;
f<string> <= g && f<string> !== g;
f<string> !== g && g <= f<string>;
f<string> === g || g < f<string>;
