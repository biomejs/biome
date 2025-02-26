let Component = (props) => <ol>{props.data.map(d => <li>{d.text}</li>)}</ol>;

let Component = (props) => <>{props.data.map(d => <li>{d.text}</li>)}</>;

let Component = (props) => <ol>{props.data.map(d => <li key={d.id}>{d.text}</li>)}</ol>;

function Component(props) {
  return <ol>{props.data.map(d => <li>{d.text}</li>)}</ol>;
}

function Component(props) {
  return <ol>{props.data?.map(d => <li>{d.text}</li>)}</ol>;
}

let Component = (props) => <ol>{props.data.map(() => <li />)}</ol>;

let Component = (props) => <ol>{props.data.map((...args) => <li>{args[0].text}</li>)}</ol>;