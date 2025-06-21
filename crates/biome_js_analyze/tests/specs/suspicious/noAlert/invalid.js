// Direct function calls (should trigger the rule)
alert("here!");

confirm("Are you sure?");

prompt("What's your name?", "John Doe");

// Window object calls (should trigger the rule)
window.alert("hello");

window.confirm("proceed?");

window.prompt("enter name");

// Bracket notation calls (should trigger the rule)
window["alert"]("bracket notation");

// Expression calls (should trigger the rule)  
(alert)("wrapped in parens");

// Nested in other expressions
if (confirm("really?")) {
    console.log("yes");
}

const result = prompt("input:");

// Multiple calls
alert("first");
alert("second");
confirm("third");