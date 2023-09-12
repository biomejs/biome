Number.isNaN(Number.NaN);

globalThis.Number.isNaN(Number.NaN);

function localIsNaN(isNaN) {
    isNaN({});
}

function localVar() {
    var isNaN;
    isNaN()
}

localIsNaN(Number.isNaN);
