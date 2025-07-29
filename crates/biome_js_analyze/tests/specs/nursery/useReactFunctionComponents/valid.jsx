/* should not generate diagnostics */

function Foo() {
  return <div>Hello, world!</div>;
}

const Foo = () => {
  return <div>Hello, world!</div>;
};

class Foo extends React.Component {
  componentDidCatch(error, errorInfo) {}

  render() {
    return <div>This is a class component with error handling.</div>;
  }
}

export default class Bar extends React.Component {
  componentDidCatch(error, errorInfo) {}

  render() {
    return <div>This is a class component with error handling.</div>;
  }
}
