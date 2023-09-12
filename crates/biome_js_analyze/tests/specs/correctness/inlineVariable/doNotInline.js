
// Do not inline arrow function expressions

const arrowFunctionExpression = (x) => x;
arrowFunctionExpression(1);

// Do not inline function expressions

const functionExpression = function(x) {};
functionExpression(1);

// Do not inline class expressions

const classExpression = class A {};
new classExpression();

// Do not inline assignment expressions

const assignmentExpressionA = 1;
assignmentExpressionA = 2;
const assignmentExpressionB = assignmentExpressionA = 2;
console.log(assignmentExpressionB);
