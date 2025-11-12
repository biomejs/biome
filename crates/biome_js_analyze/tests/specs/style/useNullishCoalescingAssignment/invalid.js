// Basic assignment
x ||= 'default';

// Object property
obj.prop ||= getValue();

// Array element
arr[0] ||= fallback;

// Complex expressions
user.settings.theme ||= 'light';

// In function
function init(config) {
  config.timeout ||= 5000;
}

// Multiple assignments
foo ||= 1;
bar ||= 2;
baz ||= 3;

// Nested object access
a.b.c ||= defaultValue;

// Computed property
obj[key] ||= computedDefault();
