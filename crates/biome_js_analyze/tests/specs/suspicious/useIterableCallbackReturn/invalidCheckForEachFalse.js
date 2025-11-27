// When checkForEach is false, other array methods should still be checked

// Invalid: map without return
[1, 2, 3].map(function(el) {
    console.log(el);
});

// Invalid: filter without return
[1, 2, 3].filter(function(el) {
    console.log(el);
});

// Invalid: every without return
[1, 2, 3].every(function(el) {
    console.log(el);
});