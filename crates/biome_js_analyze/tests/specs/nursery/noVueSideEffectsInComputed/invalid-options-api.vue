<!-- should generate diagnostics -->
<script>
export default {
  computed: {
    assign() {
      this.firstName = "lorem";
      return this.firstName;
    },
    update() {
      this.count += 2;
      this.count++;
      return this.count;
    },
    reverse() {
      return this.something.reverse();
    },
    nestedPush() {
      const test = this.another.something.push("example");
      return test;
    },
    computedMember(index) {
      this.something[index] = 1;
      return this.something;
    },
    nestedSort() {
      return this.something.keys.sort();
    },
    deleteMember() {
      delete this.cache.entry;
      return this.cache;
    },
    objectAssign() {
      return Object.assign(this.settings, { theme: "dark" });
    },
    getterOnly: {
      get() {
        return this.list.splice(0, 1);
      },
      set(value) {
        this.list = value;
      },
    },
    functionValue: function () {
      return this.items.pop();
    },
    callback() {
      this.items.forEach((item) => {
        this.total += item;
      });
      return this.total;
    },
    sharedKey() {
      this.a[this.b] = 1;
      return this.a;
    },
    swap() {
      [this.a, this.b] = [this.b, this.a];
      return this.a;
    },
    loopTarget() {
      for (this.current of this.items);
      return this.current;
    },
  },
};
</script>
