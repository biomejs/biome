/* should not generate diagnostics */

// Valid: using props parameter directly
export default {
  setup(props) {
    return () => h('div', props.count)
  }
}

// Valid: destructuring in nested function (not root scope)
{
  const Component = {
    setup(props) {
      const handler = () => {
        const { count } = props;
        console.log(count);
      };
      return { handler };
    }
  };
}

// Valid: destructuring in nested arrow function
{
  const Component = {
    setup(props) {
      return () => {
        const { count } = props;
        return count;
      };
    }
  };
}
