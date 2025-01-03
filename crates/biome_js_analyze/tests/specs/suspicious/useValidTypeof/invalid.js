// Invalid string literals
typeof foo === "strnig";
typeof foo == `String`;
typeof foo !== "undefimed";
typeof foo != "undefimed";
"nunber" === typeof foo
"nunber" == typeof foo;
"fucntion" !== typeof foo
"fucntion" != typeof foo;

// Invalid literals
typeof foo != undefined;
typeof foo != null;
typeof foo != 5;
typeof foo != -5;
typeof foo != true;

typeof foo != void 0;
typeof foo != 0 + 1;
typeof foo != (false || true);
