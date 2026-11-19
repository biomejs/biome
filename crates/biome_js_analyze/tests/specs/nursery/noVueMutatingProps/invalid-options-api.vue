<!-- should generate diagnostics -->
<script>
export default {
  props: {
    value: String,
    list: Array,
    user: Object,
    count: Number,
  },
  methods: {
    assign() {
      this.value = "test";
    },
    push() {
      this.list.push(0);
    },
    sort() {
      this.list.sort();
    },
    setMember() {
      this.user.id = 1;
    },
    setDeepMember() {
      this.user.address.street = "Main Street";
    },
    increment() {
      this.count++;
    },
    compoundAssign() {
      this.count += 1;
    },
    remove() {
      delete this.user.name;
    },
    destructuringAssign() {
      [this.value] = ["test"];
    },
    setItem(index) {
      this.list[index].name = "test";
    },
    assignObject() {
      Object.assign(this.user, { id: 1 });
    },
    viaProps() {
      this.$props.value = "test";
    },
  },
};
</script>
