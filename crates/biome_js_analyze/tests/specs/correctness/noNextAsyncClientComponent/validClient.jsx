/* should not generate diagnostics */
"use client";
export default function ClientComponent() {
  return <div>Hello</div>;
}

function ClientComponent2() {
  return <div>Hello</div>;
}

const ClientComponent3 = () => {
  return <div>Hello</div>;
};

const ClientComponent4 = function() {
  return <div>Hello</div>;
};

let ClientComponent5;
ClientComponent5 = () => {
  return <div>Hello</div>;
};

let ClientComponent6;
ClientComponent6 = function() {
  return <div>Hello</div>;
};

const components = {
  MyComponent() {
    return <div>Hello</div>;
  }
};

const components2 = {
  MyComponent: () => {
    return <div>Hello</div>;
  }
};

const components3 = {
  MyComponent: function() {
    return <div>Hello</div>;
  }
};

class ComponentClass {
  MyComponent() {
    return <div>Hello</div>;
  }
}
