// When checkForEach is true, these forEach patterns are still valid

// Valid: forEach without return
[1, 2, 3].forEach(function(el) {
    console.log(el);
});

// Valid: forEach with void arrow function
[1, 2, 3].forEach((el) => {
    console.log(el);
});

// Valid: forEach with empty return
[1, 2, 3].forEach(function(el) {
    console.log(el);
    return;
});