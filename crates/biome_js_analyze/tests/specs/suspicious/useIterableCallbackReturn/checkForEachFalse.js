// When checkForEach is false (default), forEach callbacks that return values should NOT be reported
// This tests the default behavior where checkForEach = false

// Valid: forEach with return value (not reported when checkForEach is false)
[1, 2, 3].forEach(function(el) {
    return el * 2;
});

// Valid: forEach with arrow function returning value (not reported when checkForEach is false)
[1, 2, 3].forEach(el => el * 2);

// Invalid: other array methods still require proper returns
[1, 2, 3].map(function(el) {
    console.log(el);
});

[1, 2, 3].filter(function(el) {
    console.log(el);
});

[1, 2, 3].every(function(el) {
    console.log(el);
});