let inlinable = "value1";
let notInlinable = "value2";

if (inlinable) {
    notInlinable = inlinable;
}

statement(notInlinable);

let multipleDeclaratorsInlinable = "value3",
    multipleDeclaratorsNotInlinable = "value4";

if (multipleDeclaratorsInlinable) {
    multipleDeclaratorsNotInlinable.memberWrite = multipleDeclaratorsInlinable;
}

statement(multipleDeclaratorsNotInlinable);

let variable = expression();
statement(variable);

// Inline sequence expression

const sequenceExpression = (1,2);
console.log(sequenceExpression);
