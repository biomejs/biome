/* should generate diagnostics */
const map = new Map();
setup()
delete (map)["foo"];
setup();
delete (map)["foo"];
