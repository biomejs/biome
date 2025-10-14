// Invalid: arrow function with body destructuring
export default {
  setup: (props) => {
    const { count } = props;
    return () => count;
  }
}
