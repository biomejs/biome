// Test case: Direct property assignment on a parameter
function directPropertyAssignment(param) {
	param.key = "value"; // Should trigger a diagnostic
}

// Test case: Nested property assignment on a parameter
function nestedPropertyAssignment(param) {
	param.nested.key = "value"; // Should trigger a diagnostic
}

// Test case: Property assignment inside a loop
function propertyAssignmentInLoop(param) {
	for (let i = 0; i < 5; i++) {
		param.key = i; // Should trigger a diagnostic
	}
}

// Test case: Property assignment inside a conditional block
function propertyAssignmentInConditional(param) {
	if (param.condition) {
		param.key = "value"; // Should trigger a diagnostic
	}
}

// Test case: Property assignment using a computed property
function computedPropertyAssignment(param) {
	param["key"] = "value"; // Should trigger a diagnostic
}

// Test case: Property assignment via a function call
function assignProperty(param) {
	param.key = "mutatedValue"; // Mutates the parameter by assigning a property
}

function propertyAssignmentViaFunction(param) {
	assignProperty(param); // Should trigger a diagnostic if `assignProperty` mutates `param`
}

// Test case: Property assignment with a postfix increment
function propertyPostfixIncrement(param) {
	param.count++; // Should trigger a diagnostic
}

// Test case: Property assignment with a prefix decrement
function propertyPrefixDecrement(param) {
	--param.count; // Should trigger a diagnostic
}
