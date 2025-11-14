/* Regression test: ensure regular JS files still work correctly */
import A from "mod";
import B from "mod";

// Only A is used, B should warn
export { A };
