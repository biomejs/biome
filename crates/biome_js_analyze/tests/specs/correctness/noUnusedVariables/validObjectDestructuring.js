// https://github.com/biomejs/biome/issues/1910
const foo = { a: 1, b: 2 };

const { a, b: b1, ...other } = foo;

console.log(other)