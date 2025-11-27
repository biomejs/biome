// When checkForEach is false, forEach callbacks that return values should NOT be reported
// But other array methods should still be checked

// Should NOT report: forEach with return (checkForEach is false)
[1, 2, 3].forEach(function(el) {
    return el * 2;
});

// Should NOT report: forEach with arrow function implicit return
[1, 2, 3].forEach((el) => el * 2);

// Should NOT report: forEach with conditional return
[1, 2, 3].forEach(function(el) {
    if (el > 1) {
        return el;
    }
});

// Should still report: map without return
[1, 2, 3].map(function(el) {
    console.log(el);
});

// Should still report: filter without return
[1, 2, 3].filter(function(el) {
    console.log(el);
});

// Should still report: every without return
[1, 2, 3].every(function(el) {
    console.log(el);
});