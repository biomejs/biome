const highMin = "\u{d800}";
const highMid = "\u{d8ff}";
const highMax = "\u{DBFF}";

const lowMin = "\u{dc00}";
const lowMid = "\u{DdEf}";
const lowMax = "\u{DFFF}";

const adjacentHighLow = "\u{d83d}\u{de0a}";
const adjacentLowHigh = "\u{de0a}\u{d83d}";
const repeatedHigh = "\u{d800}\u{d801}\u{d802}";
const repeatedLow = "\u{dc00}\u{dc01}\u{dc02}";

const wrapped = "start:\u{dabc}:end";
const mixedCase = "\u{DaFf}\u{dFfF}";
const interleaved = "A\u{d912}B\u{dd34}C";
const escapedQuote = "\u{d834}\"\u{dd1e}";

const templateA = `\u{d800}`;
const templateB = `left \u{dbff} right`;
const templateC = `\u{dc00}\u{DFFF}`;
const templateD = `\u{d83d} smile? \u{de42}`;
