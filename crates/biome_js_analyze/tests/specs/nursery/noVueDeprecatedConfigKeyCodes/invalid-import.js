/* should generate diagnostics */
import Vue from "vue";
import * as VueNamespace from "vue";
import { config } from "vue";

Vue.config.keyCodes = { f1: 112 };
VueNamespace.config.keyCodes = { f1: 112 };
config.keyCodes = { f1: 112 };
