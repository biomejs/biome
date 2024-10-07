const string1 = number.toFixed();
const string2 = number.toFixed(/* comment */);
Number(1).toFixed();

// False positive cases
const bigNumber = new BigNumber(1); const string3 = bigNumber.toFixed();
