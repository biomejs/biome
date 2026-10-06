/* should not generate diagnostics */
import { nextTick } from "vue";
import * as Vue from "vue";

await nextTick();

nextTick().then(() => {
    updateDom();
});

Vue.nextTick().then(() => {
    updateDom();
});

export default {
    async mounted() {
        await this.$nextTick();
    },
};

const localNextTick = (callback) => callback();
localNextTick(() => {
    updateDom();
});

nextTick("not a callback");
