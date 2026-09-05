// should generate diagnostics

let changedByTypeOnlyFunction = true;
function updateTypeOnly() {
    changedByTypeOnlyFunction = false;
}
while (changedByTypeOnlyFunction) {
    type Update = typeof updateTypeOnly;
    process();
}
