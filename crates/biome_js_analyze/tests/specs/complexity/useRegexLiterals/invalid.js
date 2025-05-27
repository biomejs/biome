// https://github.com/biomejs/biome/issues/5487
new RegExp("\/pattern$");

new RegExp("\🙂pattern");

// https://github.com/biomejs/biome/issues/5693#issuecomment-2816096167
new RegExp(`a\*b`);

// The backspace escape is not supported in regexes.
new RegExp("\b");

new RegExp("a\
b");

new RegExp(`a
b`);
