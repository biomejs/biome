/* should not generate diagnostics */

// Using includes directly (already correct)
"foo".includes("o");
!["a", "b", "c"].includes("a");
"abc".includes("a");
arr.includes(item);

// indexOf with different comparison (not checking for inclusion)
"foo".indexOf("o") > 0;  // checking position
"foo".indexOf("o") === 1; // checking exact position
"foo".indexOf("o") > 2;
arr.indexOf(item) === 0; // checking if first

// indexOf on identifiers (we don't know if they have includes)
arr.indexOf(element) >= 0;
str.indexOf(char) >= 0;
arr.indexOf(element) < 0;
arr.indexOf(item) != -1;
arr.indexOf(item) == -1;

// indexOf on method chaining (we don't know the return type)
str.trim().indexOf("a") !== -1;
arr.filter(x => x).indexOf(item) >= 0;

// indexOf with optional chaining (uncertain receiver)
customObj.indexOf?.("a") !== -1;

// indexOf with computed member expressions on non-literals
arr["indexOf"](element) !== -1;
str["indexOf"](char) >= 0;

// indexOf on call expressions (we don't know the return type)
getStuff().indexOf("o") >= 0;

// Regex test with non-literal receivers (we don't know if they're strings)
/bar/.test(foo);
/baz/.test(str);
/foo/.test(str);
/123/.test(numberString);
/pattern/.test(variable);
/pattern/.test(obj.prop);
/pattern/.test(func());
/pattern/.test(arr[0]);

// Regex with flags (not simple)
/a/i.test("abc");
/a/g.test("abc");
/a/m.test("abc");
/a/ig.test("abc");

// Complex regex patterns (not simple)
/[ab]/.test("abc");
/a|b/.test("abc");
/a+/.test("abc");
/a*/.test("abc");
/a?/.test("abc");
/a{2}/.test("abc");
/^a/.test("abc");
/a$/.test("abc");
/a.b/.test("abc");
/\d/.test("abc");
/\w/.test("abc");
/\s/.test("abc");
/(ab)/.test("abc");

// Type-specific methods
Set.prototype.has.call(set, value);
Map.prototype.has.call(map, key);
