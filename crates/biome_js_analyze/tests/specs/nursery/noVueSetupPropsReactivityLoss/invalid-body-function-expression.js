// Invalid: function expression with body destructuring
export default {
  setup: function(props) {
    const { count } = props;
    return { count };
  }
}
