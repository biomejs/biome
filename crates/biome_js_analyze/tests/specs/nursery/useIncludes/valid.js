/* should not generate diagnostics */

// Using includes directly
str.includes("foo");
!arr.includes(item);

// Using indexOf for position
const idx = str.indexOf("foo");

// indexOf used with different comparisons that don't map to includes
str.indexOf("foo") > 0;
str.indexOf("foo") >= 1;
str.indexOf("foo") === 0;
str.indexOf("foo") !== 0;

// Different method name
str.lastIndexOf("foo") !== -1;
str.search("foo") !== -1;

// Not a method call
indexOf("foo") !== -1;
