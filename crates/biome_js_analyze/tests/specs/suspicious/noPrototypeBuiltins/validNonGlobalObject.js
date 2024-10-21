(Object) => Object.hasOwnProperty.call(obj, prop); // not global Object
(Object) => ({}).hasOwnProperty.call(obj, prop); // Object is shadowed, so Object.hasOwn cannot be used here
(Object) => Object.prototype.hasOwnProperty.call(obj, prop); // not global Object