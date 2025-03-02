// With ignoreRestSiblings: false, unused variables should be reported
const car = { brand: "Tesla", year: 2019, countryCode: "US" };
const { brand, year, ...other } = car;
console.log(other);

// Renamed properties
const data = { foo: 1, bar: 2, baz: 3 };
const { foo: renamedFoo, bar: renamedBar, ...remaining } = data;
console.log(remaining);
