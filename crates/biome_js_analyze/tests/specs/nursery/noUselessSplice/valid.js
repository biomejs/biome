/* should not generate diagnostics */
array.shift();
array.unshift(element);
array.pop();
array.push(element);
array.length = 0;

array.splice(index, 1);
array.splice(index, 2);
array.splice(index, 1, element);
array.splice(index, count);
array.splice(1, 0, element);
array.splice(0, 2);
array.splice(0, 1, element);
array.splice(- -1, 1);
array.splice(array.length - 2, 1);
array.splice(array.length - 1, 0, element);
array.splice(otherArray.length - 1, 1);
array.splice(otherArray.length, 0, element);
array.splice(1, array.length);
getArray().splice(getArray().length, 0, element);
array.splice(...args);
array.splice(0, ...rest);
array.toSpliced(0, 1);
splice(0, 1);

// Only dot notation is checked.
array["splice"](0, 1);
array[splice](0, 1);

// Optional chaining
array?.splice(0, 1);
array.splice?.(0, 1);
object?.array.splice(0, 1);
getObject?.().array.splice(0, 1);
(object?.array).splice(0, 1);

// The removed elements are used.
const removed = array.splice(0);
const allRemoved = array.splice(0, array.length);
for (const item of array.splice(0)) { item; }
for (const item of array.splice(0, array.length)) { item; }
consume((f(), array.splice(0)));
