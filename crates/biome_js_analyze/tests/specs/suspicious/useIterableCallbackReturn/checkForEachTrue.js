// should generate diagnostics

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

// Valid: forEach with no return value
[1, 2, 3].forEach(function(el) {
    console.log(el);
});

// Valid: forEach with empty return
[1, 2, 3].forEach(function(el) {
    if (el > 10) {
        return;
    }
    console.log(el);
});
