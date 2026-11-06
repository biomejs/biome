<script>
/* should not generate diagnostics */
export default {
  components: {
    FooBar,
    TodoItem: todoItem,
    "TodoList": List,
    QR,
    V8Engine,
    漢字: Kanji,
    ...otherComponents,
  },
};
</script>
