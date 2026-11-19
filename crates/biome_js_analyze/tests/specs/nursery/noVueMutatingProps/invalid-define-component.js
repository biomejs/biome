/* should generate diagnostics */
import { defineComponent } from "vue";

export const Setup = defineComponent({
  props: {
    value: String,
    user: Object,
  },
  setup(props) {
    props.value = "test";
    props.user.name = "Alice";
  },
});

export const SetupDestructure = defineComponent({
  props: {
    list: Array,
    user: Object,
  },
  setup({ list, user }) {
    list.push(1);
    user.name = "Alice";
  },
});

export const Options = defineComponent({
  props: ["count"],
  methods: {
    increment() {
      this.count++;
    },
  },
});
