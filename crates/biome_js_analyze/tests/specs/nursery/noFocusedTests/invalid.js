describe.only("bar", function () {});
it.only("bar", function () {});
test.only("bar", function () {});

describe.only("bar", () => {});
it.only("bar", () => {});
test.only("bar", () => {});

describe["only"]("bar", function () {});
it["only"]("bar", function () {});
test["only"]("bar", function () {});

fdescribe("foo", () => {});
fit("foo", () => {});
