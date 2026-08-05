/* should generate diagnostics */

// With `compilationMode: "all"`, every function is analyzed, including
// utility functions that don't follow React naming conventions.

let counter = 0;

export function increment() {
    counter = counter + 1;
    return counter;
}
