/* should generate diagnostics */
const Vue = require("vue");

Vue.config.keyCodes = { f1: 112 };

export default {
  install(Vue) {
    Vue.config.keyCodes.f1 = 112;
  },
};
