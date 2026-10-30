// should generate diagnostics
obj[("__proto__" as string)];
obj[("__proto__" as string)] = value;
obj[("__proto__" satisfies string)];
obj[("__proto__" satisfies string)] = value;
obj[("__proto__")!];
obj[("__proto__")!] = value;
obj[<string>"__proto__"];
obj[<string>"__proto__"] = value;
obj[((`__proto__` as string)!)];
