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

describe.only(bar, () => {});
it.only(bar, () => {});
test.only(bar, () => {});

describe.only(foo.bar, () => {});
it.only(foo.bar, () => {});
test.only(foo.bar, () => {});

describe.only(name = name || "bar", () => {});
it.only(name = name || "bar", () => {});
test.only(name = name || "bar", () => {});
