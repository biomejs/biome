// When checkForEach is false, forEach callbacks that return values are valid (not reported)

// Valid: forEach with return (checkForEach is false)
[1, 2, 3].forEach(function(el) {
    return el * 2;
});

// Valid: forEach with arrow function implicit return
[1, 2, 3].forEach((el) => el * 2);

// Valid: forEach with conditional return
[1, 2, 3].forEach(function(el) {
    if (el > 1) {
        return el;
    }
});