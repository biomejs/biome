// should generate diagnostics

let node = getNode();
while (node) {
    process(node);
}
node = getOtherNode();

for (let index = 0; index < 5;) {
    process(index);
}

let ready = true;
do {
    process();
} while (ready);

let left = 0;
let right = 1;
while (left !== right) {
    process(left, right);
}

let select = true;
let whenTrue = 1;
let whenFalse = 2;
while (select ? whenTrue : whenFalse) {
    process();
}

let first = true;
let second = true;
while (first && second) {
    first = false;
}

let nestedLeft = 0;
let nestedRight = 0;
let sharedLimit = 10;
while (nestedLeft < sharedLimit && nestedRight < sharedLimit) {
    nestedLeft++;
}

let interpolation = "";
while (`${interpolation}`) {
    process(interpolation);
}

let specifier = "./module.js";
while (import(specifier)) {
    process();
}

let initializedOnly = 0;
for (initializedOnly = 1; initializedOnly < 5;) {
    process(initializedOnly);
}

for (var initializedVarOnly = 0; initializedVarOnly < 5;) {
    process(initializedVarOnly);
}

let shadowed = true;
while (shadowed) {
    let shadowed = false;
    process(shadowed);
}

let changedByUnusedFunction = true;
function updateUnused() {
    changedByUnusedFunction = false;
}
while (changedByUnusedFunction) {
    process();
}

let changedByInitializerFunction = true;
function updateInInitializer() {
    changedByInitializerFunction = false;
}
for (updateInInitializer(); changedByInitializerFunction;) {
    process();
}

let objectMethodGroup = 0;
while (objectMethodGroup < { method() { check(); } }) {
    process();
}

let computedMethodName = "method";
while ({ [computedMethodName]() {} }) {
    process();
}
