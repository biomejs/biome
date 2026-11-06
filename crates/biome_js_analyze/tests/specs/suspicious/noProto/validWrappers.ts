// should not generate diagnostics
obj[(property as string)];
obj[(property satisfies string)] = value;
obj[(tag`__proto__` as string)];
obj[(tag`__proto__` as string)] = value;
obj[("'__proto__'" as string)];
obj[(`"__proto__"` satisfies string)] = value;
