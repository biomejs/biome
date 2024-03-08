let valid: Array<foo>;
let invalid1: Array<foo, Array<string>>;
let invalid2: Promise<Array<string>>;
let invalid3: Array<Foo<Bar>>;
let invalid4: Array<[number, number]>;

let readonlyInvalid1: ReadonlyArray<foo>;
let readonlyInvalid2: Promise<ReadonlyArray<string>>;
let readonlyInvalid3: ReadonlyArray<Foo<Bar>>;
let readonlyInvalid4: ReadonlyArray<[number, number]>;
let readonlyInvalid5: ReadonlyArray<ReadonlyArray<number>>;
let readonlyInvalid6: ReadonlyArray<ReadonlyArray<ReadonlyArray<number>>>;
