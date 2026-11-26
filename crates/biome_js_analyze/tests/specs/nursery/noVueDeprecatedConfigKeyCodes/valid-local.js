/* should not generate diagnostics */
import V from "not-vue";

V.config.keyCodes = {};

function f(app) {
  app.config.keyCodes = {};
}
