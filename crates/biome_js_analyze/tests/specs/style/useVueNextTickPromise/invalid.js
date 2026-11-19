/* should generate diagnostics */
import { nextTick } from "vue";
import * as Vue from "vue";

nextTick(() => {
    updateDom();
});

nextTick(function () {
    updateDom();
});

Vue.nextTick(() => {
    updateDom();
});

export default {
    mounted() {
        this.$nextTick(() => {
            updateDom();
        });
    },
};
