"use client";

export default async function MyComponent() {
  return <div>Hello</div>;
}

async function MyComponent2() {
  return <div>Hello</div>;
}

const MyComponent3 = async () => {
  return <div>Hello</div>;
};

const MyComponent4 = async function() {
  return <div>Hello</div>;
};

let MyComponent5;
MyComponent5 = async () => {
  return <div>Hello</div>;
};

let MyComponent6;
MyComponent6 = async function() {
  return <div>Hello</div>;
};

const components = {
  async MyComponent() {
    return <div>Hello</div>;
  }
};

const components2 = {
  MyComponent: async () => {
    return <div>Hello</div>;
  }
};

const components3 = {
  MyComponent: async function() {
    return <div>Hello</div>;
  }
};

class ComponentClass {
  async MyComponent() {
    return <div>Hello</div>;
  }
}
