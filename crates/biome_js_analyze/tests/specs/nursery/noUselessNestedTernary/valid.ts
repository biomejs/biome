/* should not generate diagnostics */

foo ? (bar ? baz : object?.property!) : object.property;
foo ? object.property : (bar ? (object?.property as string) : baz);
foo ? fallback! : (bar ? fallback : baz);
