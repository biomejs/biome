Number.isFinite(Number.NaN);

globalThis.Number.isFinite(Number.NaN);

function localIsFinite(isFinite) {
    isFinite({});
}

function localVar() {
    var isFinite;
    isFinite()
}

localIsFinite(Number.isFinite);
