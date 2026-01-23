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
  throw new Error("Original error");
} catch ({ message }) {
  throw new Error(message);
}
