// When checkForEach is true, forEach callbacks that return values should be reported

// Should report: forEach with explicit return
[1, 2, 3].forEach(function(el) {
    return el * 2;
});

// Should report: forEach with arrow function implicit return
[1, 2, 3].forEach((el) => el * 2);

// Should report: forEach with conditional return
[1, 2, 3].forEach(function(el) {
    if (el > 1) {
        return el;
    }
});

// Should NOT report: forEach without return
[1, 2, 3].forEach(function(el) {
    console.log(el);
});

// Should NOT report: forEach with void return
[1, 2, 3].forEach((el) => {
    console.log(el);
});

// Should NOT report: forEach with empty return
[1, 2, 3].forEach(function(el) {
    console.log(el);
    return;
});