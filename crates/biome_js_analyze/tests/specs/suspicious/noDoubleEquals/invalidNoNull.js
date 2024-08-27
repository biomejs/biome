const foo1 = `
text
${a == b}
`;

const foo2 = `
text
${a == null}
`;

const foo3 = `
text
${null == a}
`;

// existing comment
a == b;

// existing comment
null == a;

// existing comment
a == null;


if (a == b) {
    false;
}

if (a == null) {
	false;
}

if (null == b) {
	false;
}

if (/** some weird comment
    **/ a == b) {

    }

let a1 = `Output of "biome rage":
  formatter enabled: ${formatter == true}
  linter: ${linter}
`;

let a2 = `Output of "biome rage":
  formatter enabled: ${formatter == null}
  linter: ${linter}
`;

let a3 = `Output of "biome rage":
  formatter enabled: ${null == formatter}
  linter: ${linter}
`;
