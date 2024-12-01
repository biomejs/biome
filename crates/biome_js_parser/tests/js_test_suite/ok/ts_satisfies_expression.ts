interface A {
   a: string
};
let x = { a: 'test' } satisfies A;
let y = { a: 'test', b: 'test' } satisfies A;
const z = undefined satisfies 1;
let not_a_satisfies_expression = undefined
satisfies;
let precedence = "hello" satisfies string + 3 satisfies number satisfies number;
