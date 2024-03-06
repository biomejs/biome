let valid: Foo[];
let invalid1: Array<foo, string[]>;
let invalid2: Promise<string[]>;
let invalid3: Foo<Bar>[];
let invalid4: [number, number][];

let readonlyInvalid1: readonly foo[];
let readonlyInvalid2: Promise<readonly string[]>;
let readonlyInvalid3: readonly Foo<Bar>[];
let readonlyInvalid4: readonly [number, number][];
let readonlyInvalid5: readonly (readonly number[])[];
let readonlyInvalid6: readonly (readonly (readonly number[])[])[];
