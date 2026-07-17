const keyName =
	obj[val] !== undefined
		? // TS reverse mapped enum
			obj[val]
		: // Normal enum / `const object`
			// TODO: Figure out a way to cache the names of commonly-used enum numbers for performance if needed
			testEnumValueToKey(obj, val);

call(
	!condition
		? consequent /*consequent-trailing*/
		: alternate
);
