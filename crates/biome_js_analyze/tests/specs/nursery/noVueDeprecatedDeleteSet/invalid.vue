<!-- should generate diagnostics -->
<script>
import Vue, { set, del, set as setProperty, del as deleteProperty } from "vue";

export default {
  mounted() {
    this.$set(obj, key, value);
    this.$delete(obj, key);
    this?.$set(obj, key, value);
    (this).$delete(obj, key);
    this["$set"](obj, key, value);
  },
  methods: {
    update() {
      const vm = this;
      vm.$set(obj, key, value);
      vm.$delete(obj, key);
    },
    withArrow() {
      [1, 2].forEach(() => this.$set(obj, key, value));
    },
    fromVue() {
      Vue.set(obj, key, value);
      Vue.delete(obj, key);
      set(obj, key, value);
      del(obj, key);
      setProperty(obj, key, value);
      deleteProperty(obj, key);
    },
  },
};
</script>
