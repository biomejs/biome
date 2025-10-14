// Invalid: assignment destructuring
export default {
  setup(props) {
    let count;
    ({ count } = props);
    return { count };
  }
}
