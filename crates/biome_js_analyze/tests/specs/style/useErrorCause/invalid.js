/* should generate diagnostics */

try {
  throw new Error("Original error");
} catch (err) {
  throw new Error("Wrapper error");
}


try {
  throw new Error("Original error");
} catch (err) {
  throw new Error(`Failed: ${err.message}`);
}

try {
  throw new Error("Original error");
} catch (err) {
  if (true) {
    throw err;
  }
}

try {
	doSomething();
} catch {
	throw new TypeError("Something went wrong");
}


try {
  throw new Error("Original error");
} catch ({ message }) {
  throw new Error(message);
}


try {
  throw new Error("Original error");
} catch (err) {
  throw new Error(`Failed to process: ${err.message}`, { cause: err.message });
}

try {
    doSomething();
} catch (error) {
    if (whatever) {
        const error = anotherError; // This declaration is the problem.
        throw new Error("Something went wrong", { cause: error });
    }
}

try {
    doSomething();
} catch (err) {
	throw new Error("", { cause: otherVar });
}

