/* should not generate diagnostics */

// Valid: named export should not trigger the rule
export const myComponent = {
  setup(props) {
    const { count } = props;
    return { count };
  }
};

// Only export default is checked
export default {
  setup(props) {
    return () => props.count;
  }
}
