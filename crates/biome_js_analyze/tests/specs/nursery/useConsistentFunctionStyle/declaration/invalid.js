// should generate diagnostics
const foo = function() {};
let arrow = () => {};
var named = function inner() {};
const parenthesized = (((function() {})));
const parenthesizedArrow = (() => {});
const asyncArrow = async () => {};
const generator = function* () {};
export const exported = function() {}, exportedArrow = () => {};
for (let loop = () => {}; condition;) {}
const nestedFunction = () => { function inner() { this; } };
const nestedArrow = () => () => this;
const nestedMethod = () => ({ method() { return this; } });
const nestedGetter = () => ({ get value() { return this; } });
const { destructured } = function() {};

const nestedSetter = () => ({ set value(value) { this.value = value; } });
const nestedClassMethod = () => class { method() { return this; } };
const nestedConstructor = () => class { constructor() { this.value = 0; } };
const nestedClassGetter = () => class { get value() { return this; } };
const nestedClassSetter = () => class { set value(value) { this.value = value; } };
