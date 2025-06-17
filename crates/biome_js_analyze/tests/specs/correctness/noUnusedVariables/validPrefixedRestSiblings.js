/* should not generate diagnostics */
// Prefixed variables should be ignored even with ignoreRestSiblings: false
const car = { brand: "Tesla", year: 2019, countryCode: "US" };
const { brand: _brand, year: _year, ...other } = car;
console.log(other);
