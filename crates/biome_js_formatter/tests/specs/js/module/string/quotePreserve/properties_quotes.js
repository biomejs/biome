class Y {
    "other" = 4;
    "method"() {}
    "another_method"() {}
    "camelMethod"() {}
    "stay-like-this"() {}
    get "getter"() {}
    set "setterr"(value) {}
    "constructor"() {}
}

let value = { "a": "test", "quotes-required": "test" };

({ "a": test } = value);

let { "a": test } = value;

// you guys stay like this

({ "$$_": test } = value);

({ "%{}": test } = value);

({ "[]": test } = value);

let { "fff--fff": test } = value;

let { "with_underscore": test } = value;

let a = { "": 10, "c_d": 30 }

let b = { "'": 10, "c_d": 30 }

let { "_$_ff$_morning_not_quotes": test, "_$_ff$_morning_yes_quotes_@": test } = value;

let { "_$_$_%": test } = value;

let { "0197": test, "3n": test, "3p": test, "p9": test } = value;

const x = {
    '¾¾¾¾': 'test1',
    '①': 'test2',
};
