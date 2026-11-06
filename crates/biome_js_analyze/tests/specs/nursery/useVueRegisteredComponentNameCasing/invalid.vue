<script>
/* should generate diagnostics */
export default {
  components: {
    fooBar,
    todoItem: TodoItem,
    "todo-list": TodoList,
    'side-bar': SideBar,
    nav_bar: NavBar,
    $modal: Modal,
    ["foo-baz"]: FooBaz,
    _: Underscore,
    renderItem() {
      return h("div");
    },
  },
};
</script>
