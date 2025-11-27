// When checkForEach is true, forEach callbacks that return values should be reported

// Invalid: forEach with explicit return
[1, 2, 3].forEach(function(el) {
    return el * 2;
});

// Invalid: forEach with arrow function implicit return
[1, 2, 3].forEach((el) => el * 2);

// Invalid: forEach with conditional return
[1, 2, 3].forEach(function(el) {
    if (el > 1) {
        return el;
    }
});