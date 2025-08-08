/* should not generate diagnostics */
export default async function ServerComponent() {
  return <div>Hello</div>;
}

const ServerComponent2 = async () => {
  return <div>Hello</div>;
};

let ServerComponent3;
ServerComponent3 = async function() {
  return <div>Hello</div>;
};

const serverComponents = {
  async ServerComponent() {
    return <div>Hello</div>;
  }
};

const serverComponents2 = {
  ServerComponent: async () => {
    return <div>Hello</div>;
  }
};

class ServerComponentClass {
  async ServerComponent() {
    return <div>Hello</div>;
  }
}
