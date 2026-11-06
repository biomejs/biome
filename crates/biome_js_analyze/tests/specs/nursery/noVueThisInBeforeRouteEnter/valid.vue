<script>
/* should not generate diagnostics */
export default {
  beforeRouteEnter(to, from, next) {
    const variable = 42;
    someFunction(variable);
    next((vm) => {
      vm.load(to.params.id);
    });
    const helper = function () {
      return this.value;
    };
    function declared() {
      return this.value;
    }
    const object = {
      method() {
        return this.value;
      },
      get getter() {
        return this.value;
      },
      set setter(value) {
        this.value = value;
      },
    };
    class Local {
      field = this;
      static {
        this.count = 0;
      }
      constructor(value = this.defaultValue) {
        this.value = value;
      }
      method() {
        return this.value;
      }
    }
  },
  beforeRouteUpdate(to) {
    this.load(to.params.id);
  },
  beforeRouteLeave() {
    this.save();
  },
  methods: {
    beforeRouteEnter() {
      this.value = 1;
    },
  },
};
</script>
