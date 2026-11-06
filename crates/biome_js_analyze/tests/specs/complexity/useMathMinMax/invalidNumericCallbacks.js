/* should generate diagnostics */

const numbers = [1, 2];
const largestNumber = numbers.reduce((a, b) => (a > b ? a : b));
const smallestNumber = numbers.reduceRight((a, b) => (a < b ? a : b));

const wordsWithNumericAccumulator = ["foo", "bar"];
const numericAccumulator = wordsWithNumericAccumulator.reduce(
    (size, word) => (size > 10 ? size : 10),
    0,
);

const wordsWithNumericIndex = ["foo", "bar"];
const numericIndex = wordsWithNumericIndex.reduce(
    (size, word, index) => (index > 2 ? index : 2),
    0,
);

const mutatedValues = ["foo", 1];
mutatedValues[0] = 2;
const mutatedMaximum = mutatedValues.reduce((a, b) => (a > b ? a : b));

const unknownMaximum = unknownArray.reduce((a, b) => (a > b ? a : b));

const aliasedValues = ["foo", 1];
const valuesAlias = aliasedValues;
valuesAlias[0] = 2;
const aliasedMaximum = aliasedValues.reduce((a, b) => (a > b ? a : b));

const numericStrings = ["1", "2"];
const convertedMaximum = numericStrings.reduce((a, b) => {
    b = Number(b);
    return a > b ? a : b;
}, 0);

const numberFromTemplate = (parts) => Number(parts[0]);
const taggedNumbers = [numberFromTemplate`1`, numberFromTemplate`2`];
const taggedMaximum = taggedNumbers.reduce((a, b) => (a > b ? a : b));

const wrappedNumbers = [1, 2];
const wrappedMaximum = ((wrappedNumbers).reduce)(((a, b) => (a > b ? a : b)));

let changingElement = "1";
changingElement = 1;
const valuesWithChangedElement = [changingElement, 2];
const changedElementMaximum = valuesWithChangedElement.reduce((a, b) => (a > b ? a : b));
