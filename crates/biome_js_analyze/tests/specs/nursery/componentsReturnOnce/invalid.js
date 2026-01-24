function Component() {
  if (condition) {
    return <div />;
  }
  return <span />;
};

const Component = () => {
  if (condition) {
    return <div />;
  }
  return <span />;
};

function Component() {
  return Math.random() > 0.5 ? <div>Big!</div> : <div>Small!</div>;
};

function Component() {
  return Math.random() > 0.5 ? <div>Big!</div> : "Small!";
};

function Component() {
  return Math.random() > 0.5 ? (
    <div>Big! No, really big!</div>
  ) : (
    <div>Small!</div>
  );
};

function Component(props) {
  return props.cond1 ? (
    <div>Condition 1</div>
  ) : Boolean(props.cond2) ? (
    <div>Not condition 1, but condition 2</div>
  ) : (
    <div>Neither condition 1 or 2</div>
  );
};

function Component(props) {
  return !!props.cond && <div>Conditional</div>;
};

function Component(props) {
  return props.primary || <div>{props.secondaryText}</div>;
};

HOC(() => {
  if (condition) {
    return <div />;
  }
  return <div />;
});