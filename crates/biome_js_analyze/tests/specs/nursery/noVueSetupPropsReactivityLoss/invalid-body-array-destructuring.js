// Invalid: array destructuring in body
export default {
  setup(props) {
    const [first] = props;
    return first;
  }
}
